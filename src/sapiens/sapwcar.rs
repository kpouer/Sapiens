#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, ClipRect, DisposeRgn, GetClip, MoveTo, NewRgn,
    RGBForeColor, SetClip, SetPt, SetRect,
};
use crate::cbridge::Collapse;
use crate::langue::Langue::ES;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapglob::{colaff, linaff};
use crate::sapiens::saptxt::{
    CvtCar, CvtCarLissage, GetBkMot, langue, nomadversaire, nomcopain, nomjoueur, nomsecondaire,
    nomtribu,
};
use crate::sapiens::sapwbrt::{AffmobCar, DoEraseRect, DoPaintRect, TabMob};
use crate::sapiens::sapwind::{Ecran1, Fonte10, Fonte14, debecr};
use crate::sapiens::sapwmess::rMess;
use crate::sapiens::{NB_MOTS_BKMOTS, POLICE_LISSAGE, STYLE_LISSAGE};
use crate::shared::{Boolean, CGrafPtr, FontInfo, Point, RGBColor, Rect, RgnHandle};
use crate::stubs::CarboniseSetPatBlack;
use crate::text::{CharWidth, DrawChar, GetFontInfo, TextFace, TextFont, TextSize};
use std::os::raw::{c_char, c_int, c_short};
use std::ptr::{addr_of, addr_of_mut, null_mut};

const ECHELLE: i16 = 2;

#[unsafe(no_mangle)]
pub static mut LissagePolice: Boolean = true;

static rgbWhite: RGBColor = RGBColor {
    red: 0xFFFF,
    green: 0xFFFF,
    blue: 0xFFFF,
};
static rgbBlack: RGBColor = RGBColor {
    red: 0,
    green: 0,
    blue: 0,
};
static rgbBrown: RGBColor = RGBColor {
    red: 34368,
    green: 27840,
    blue: 7008,
};
static rgbGray: RGBColor = RGBColor {
    red: 50000,
    green: 50000,
    blue: 50000,
};
static mut indexWhite: c_short = 0;
static mut indexBlack: c_short = 0;
static mut indexBrown: c_short = 0;
static mut indexGray: c_short = 0;

#[unsafe(no_mangle)]
static mut ptEcrCar: Point = Point { v: 0, h: 0 };

/*************************************************************************
                    InitCouleurFontes
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitCouleurFontes() {
    let mut gbak: CGrafPtr = null_mut();
    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);
    *addr_of_mut!(indexWhite) = SapColor2Index(addr_of!(rgbWhite) as *mut _) as c_short;
    *addr_of_mut!(indexBlack) = SapColor2Index(addr_of!(rgbBlack) as *mut _) as c_short;
    *addr_of_mut!(indexBrown) = SapColor2Index(addr_of!(rgbBrown) as *mut _) as c_short;
    *addr_of_mut!(indexGray) = SapColor2Index(addr_of!(rgbGray) as *mut _) as c_short;
    AcamSetPortOffscreen(gbak);
}

/*************************************************************************
                    Gotoxy
Positionne l'ecriture des caracteres en y et x
=> position en colonne et ligne caracteres
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Gotoxy(x: c_short, y: c_short) {
    SetPt(
        addr_of_mut!(ptEcrCar),
        (x * ECHELLE) * 6,
        (y * ECHELLE) * 7 - 1,
    );
    *addr_of_mut!(colaff) = x;
    *addr_of_mut!(linaff) = y;
}

/*************************************************************************
                    GotoxyPixel
Positionne l'ecriture des caracteres en y et x pixel
=> position en colonne et ligne caracteres
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GotoxyPixel(x: c_short, y: c_short) {
    SetPt(addr_of_mut!(ptEcrCar), x, y);
}

/*************************************************************************
                    TailleCar
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TailleCar(car: c_short, taille: c_short) -> c_short {
    if LissagePolice || langue == ES as c_short {
        /* ES Pour les caracteres speciaux */
        let car = CvtCarLissage(car);
        TextFont(POLICE_LISSAGE as i16);
        TextSize(taille);
        TextFace(STYLE_LISSAGE as i16);
        return CharWidth(car as i8);
    } else {
        let car = CvtCar(car);
        if taille == 10 {
            let num = Fonte10 + car - 31;
            return (*addr_of!(TabMob))[num as usize].nbcol + 2;
        } else {
            let num = Fonte14 + car - 31;
            return (*addr_of!(TabMob))[num as usize].nbcol + 3;
        }
    }
}

/*************************************************************************
                    SetColorFont
**************************************************************************/
unsafe fn SetColorFont(id: c_short) {
    if id == indexWhite {
        RGBForeColor(addr_of!(rgbWhite));
    } else if id == indexBlack {
        RGBForeColor(addr_of!(rgbBlack));
    } else if id == indexBrown {
        RGBForeColor(addr_of!(rgbBrown));
    } else if id == indexGray {
        RGBForeColor(addr_of!(rgbGray));
    } else {
        Collapse(
            c"SetColorFont: %d unknown".as_ptr() as *mut c_char,
            id as c_int,
        );
    }
}

/*************************************************************************
                    AffCarBrt
Affiche un caractere brut à la position courante
=> Caractere à afficher
**************************************************************************/
unsafe fn AffCarBrt(car: c_short, couleur: c_short, couleurOmbre: c_short, taille: c_short) {
    if LissagePolice || langue == ES as c_short {
        /* ES Pour les caracteres speciaux */
        let mut info = FontInfo::default();
        let mut dx: c_short = 0;
        let dy: c_short = 0;
        let mut car = CvtCarLissage(car);
        if car == '<' as c_short {
            car = 0xC9;
            dx -= 1;
        }
        if car == '>' as c_short {
            car = 0xC9;
            dx -= 1;
        }
        TextFont(POLICE_LISSAGE as i16);
        TextSize(taille);
        TextFace(STYLE_LISSAGE as i16);
        GetFontInfo(&mut info);
        SetColorFont(couleurOmbre);
        MoveTo(ptEcrCar.h + dx, ptEcrCar.v + 1 + info.ascent + dy + 1);
        DrawChar(car as i8);
        SetColorFont(couleur);
        MoveTo(ptEcrCar.h + dx, ptEcrCar.v + info.ascent + dy);
        DrawChar(car as i8);
        ptEcrCar.h += CharWidth(car as i8);
    } else {
        let car = CvtCar(car);
        let num: c_short;
        if taille == 10 {
            num = Fonte10 + car - 31;
            AffmobCar(num, ptEcrCar.h + 1, ptEcrCar.v + 1, couleurOmbre);
            AffmobCar(num, ptEcrCar.h, ptEcrCar.v, couleur);
            ptEcrCar.h += (*addr_of!(TabMob))[num as usize].nbcol + 2;
        } else {
            num = Fonte14 + car - 31;
            AffmobCar(num, ptEcrCar.h + 2, ptEcrCar.v + 1, couleurOmbre);
            AffmobCar(num, ptEcrCar.h + 1, ptEcrCar.v, couleur);
            AffmobCar(num, ptEcrCar.h, ptEcrCar.v, couleur);
            ptEcrCar.h += (*addr_of!(TabMob))[num as usize].nbcol + 3;
        }
    }
}

/*************************************************************************
                    GetStrBkMot
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStrBkMot(car: c_short) -> *mut c_char {
    let vide: *mut c_char = c"".as_ptr() as *mut c_char;
    let bkmots = GetBkMot();
    let mut p: *mut c_char = vide;
    let car = car - 129;
    if car < NB_MOTS_BKMOTS as c_short {
        p = *bkmots.add(car as usize);
        if *p == b'$' as c_char {
            /* Substitution */
            let c = *p.add(1);
            if c == b'0' as c_char {
                p = addr_of!(nomsecondaire) as *mut c_char;
            } else if c == b'1' as c_char {
                p = addr_of!(nomjoueur) as *mut c_char;
            } else if c == b'2' as c_char {
                p = addr_of!(nomadversaire) as *mut c_char;
            } else if c == b'3' as c_char {
                p = addr_of!(nomtribu) as *mut c_char;
            } else if c == b'4' as c_char {
                p = addr_of!(nomcopain) as *mut c_char;
            }
        }
    } else {
        eprintln!("GetStrBkMot error");
    }
    p
}

/*************************************************************************
                    AffCarBank
Affiche un caractere sur le port en entrée
**************************************************************************/
unsafe fn AffCarBank(
    port: CGrafPtr,
    car: c_short,
    couleur1: c_short,
    couleur2: c_short,
    taille: c_short,
) {
    let mut gbak: CGrafPtr = null_mut();
    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(port);
    if car > 128 {
        let mut p = GetStrBkMot(car);
        while *p != 0 {
            AffCarBank(port, *p as u8 as c_short, couleur1, couleur2, taille);
            p = p.add(1);
        }
    } else {
        AffCarBrt(car, couleur1, couleur2, taille);
        *addr_of_mut!(colaff) += 1;
    }
    AcamSetPortOffscreen(gbak);
}

/*************************************************************************
                    affcar
Affiche le caractere demandé à la position courante
Les caracteres >128 sont des macros substitions
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn affcar(car: c_short) {
    AffCarBank(debecr, car, indexBlack, indexBrown, 14);
}

/*************************************************************************
                    affcar10
Affiche le caractere demandé à la position courante
Les caracteres >128 sont des macros substitions
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn affcar10(car: c_short) {
    AffCarBank(debecr, car, indexBlack, indexGray, 10);
}

/*************************************************************************
                    Car2
Affiche un caractere blanc sur une ombre noire
Le caractere est affiché sur debecr et n'est pas transféré
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn car2(car: c_short) {
    AffCarBank(debecr, car, indexWhite, indexBlack, 14);
}

/*************************************************************************
                    affcarmess
Affiche le caractere demandé à la position courante
Les caracteres >128 sont des macros substitions
Le caractere est affiche sur debecr
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn affcarmess(car: c_short) {
    let mut r = Rect::default();
    let mut gbak: CGrafPtr = null_mut();
    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(debecr);
    let rgn: RgnHandle = NewRgn();
    GetClip(rgn);
    ClipRect(addr_of!(rMess));
    /* Effacement et affichage curseur */
    if car < 32 {
        SetRect(
            &mut r,
            ptEcrCar.h,
            ptEcrCar.v,
            ptEcrCar.h + 8,
            ptEcrCar.v + 11,
        );
        if car == 2 {
            CarboniseSetPatBlack();
            DoPaintRect(&mut r);
        }
        if car == 1 {
            DoEraseRect(&mut r);
        }
    } else {
        AffCarBank(debecr, car, indexBlack, indexGray, 14);
    }
    SetClip(rgn);
    DisposeRgn(rgn);
    AcamSetPortOffscreen(gbak);
}

/***********************************************************************
                        MOTLST2:Affiche Pour jauges
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn motlst2(debut: *mut c_char) {
    let mut p = debut as *mut u8;
    while *p != 0 {
        car2(*p as c_short);
        p = p.add(1);
    }
}
