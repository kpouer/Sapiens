#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
use crate::acam::{ClipRect, DisposeRgn, GetClip, MoveTo, NewRgn, RGBForeColor, SetClip};
use crate::langue::Langue::ES;
use crate::langue::{Langue, langue};
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::saptxt::{CvtCar, CvtCarLissage};
use crate::sapiens::sapwbrt::{DoEraseRect, DoPaintRect};
use crate::sapiens::{NB_MOTS_BKMOTS, POLICE_LISSAGE, STYLE_LISSAGE};
use crate::shared::{Boolean, CGrafPtr, FontInfo, Point, RGBColor, Rect, RgnHandle};
use crate::stubs::CarboniseSetPatBlack;
use crate::text::{CharWidth, DrawChar, GetFontInfo, TextFace, TextFont, TextSize};
use crate::{Sapiens, port};
use std::os::raw::{c_char, c_short};
use std::ptr::addr_of;

const ECHELLE: i16 = 2;

/* Etat mutable de SAPWCAR.C, ex-`static mut`, regroupe dans cette
structure, champ de `Sapiens`. */
pub(crate) struct Sapwcar {
    /* ex-`LissagePolice` */
    pub(crate) lissage_police: Boolean,
    /* ex-`indexWhite` */
    index_white: c_short,
    /* ex-`indexBlack` */
    index_black: c_short,
    /* ex-`indexBrown` */
    index_brown: c_short,
    /* ex-`indexGray` */
    index_gray: c_short,
    /* ex-`ptEcrCar` */
    pt_ecr_car: Point,
}

impl Default for Sapwcar {
    fn default() -> Self {
        Self {
            lissage_police: true,
            index_white: 0,
            index_black: 0,
            index_brown: 0,
            index_gray: 0,
            pt_ecr_car: Point { v: 0, h: 0 },
        }
    }
}

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

impl Sapiens {
    /*************************************************************************
                        InitCouleurFontes
    **************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn InitCouleurFontes(&mut self) {
        let _g = port::on(self.sapwind.Ecran1);
        self.sapwcar.index_white = SapColor2Index(&rgbWhite) as c_short;
        self.sapwcar.index_black = SapColor2Index(&rgbBlack) as c_short;
        self.sapwcar.index_brown = SapColor2Index(&rgbBrown) as c_short;
        self.sapwcar.index_gray = SapColor2Index(&rgbGray) as c_short;
    }

    /*************************************************************************
                        Gotoxy
    Positionne l'ecriture des caracteres en y et x
    => position en colonne et ligne caracteres
    **************************************************************************/
    pub(crate) fn Gotoxy(&mut self, x: c_short, y: c_short) {
        self.sapwcar.pt_ecr_car = Point::new((x * ECHELLE) * 6, (y * ECHELLE) * 7 - 1);
        self.sapglob.colaff = x;
        self.sapglob.linaff = y;
    }
}

impl Sapwcar {
    /*************************************************************************
                        GotoxyPixel
    Positionne l'ecriture des caracteres en y et x pixel
    => position en colonne et ligne caracteres
    **************************************************************************/
    pub(crate) fn GotoxyPixel(&mut self, x: c_short, y: c_short) {
        self.pt_ecr_car = Point::new(x, y);
    }
}

impl Sapiens {
    /*************************************************************************
                        TailleCar
    **************************************************************************/
    pub(crate) fn TailleCar(&self, car: c_short, taille: c_short) -> c_short {
        if self.sapwcar.lissage_police || langue() == ES {
            /* ES Pour les caracteres speciaux */
            let car = CvtCarLissage(car);
            TextFont(POLICE_LISSAGE as i16);
            TextSize(taille);
            TextFace(STYLE_LISSAGE as i16);
            CharWidth(car as i8)
        } else {
            let car = CvtCar(car);
            if taille == 10 {
                let num = self.sapwind.Fonte10 + car - 31;
                self.sapwbrt.TabMob[num as usize].nbcol + 2
            } else {
                let num = self.sapwind.Fonte14 + car - 31;
                self.sapwbrt.TabMob[num as usize].nbcol + 3
            }
        }
    }
}

/*************************************************************************
                    SetColorFont
**************************************************************************/
fn SetColorFont(sapiens: &Sapiens, id: c_short) {
    if id == sapiens.sapwcar.index_white {
        RGBForeColor(&rgbWhite);
    } else if id == sapiens.sapwcar.index_black {
        RGBForeColor(&rgbBlack);
    } else if id == sapiens.sapwcar.index_brown {
        RGBForeColor(&rgbBrown);
    } else if id == sapiens.sapwcar.index_gray {
        RGBForeColor(&rgbGray);
    } else {
        sapiens.Collapse(&format!("SetColorFont: {id} unknown"));
    }
}

impl Sapiens {
    /*************************************************************************
                        AffCarBrt
    Affiche un caractere brut à la position courante
    => Caractere à afficher
    **************************************************************************/
    fn AffCarBrt(
        &mut self,
        car: c_short,
        couleur: c_short,
        couleurOmbre: c_short,
        taille: c_short,
    ) {
        if self.sapwcar.lissage_police || langue() == Langue::ES {
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
            SetColorFont(self, couleurOmbre);
            MoveTo(
                self.sapwcar.pt_ecr_car.h + dx,
                self.sapwcar.pt_ecr_car.v + 1 + info.ascent + dy + 1,
            );
            DrawChar(car as i8);
            SetColorFont(self, couleur);
            MoveTo(
                self.sapwcar.pt_ecr_car.h + dx,
                self.sapwcar.pt_ecr_car.v + info.ascent + dy,
            );
            DrawChar(car as i8);
            self.sapwcar.pt_ecr_car.h += CharWidth(car as i8);
        } else {
            let car = CvtCar(car);
            let num: c_short;
            if taille == 10 {
                num = self.sapwind.Fonte10 + car - 31;
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h + 1,
                    self.sapwcar.pt_ecr_car.v + 1,
                    couleurOmbre,
                );
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h,
                    self.sapwcar.pt_ecr_car.v,
                    couleur,
                );
                self.sapwcar.pt_ecr_car.h += self.sapwbrt.TabMob[num as usize].nbcol + 2;
            } else {
                num = self.sapwind.Fonte14 + car - 31;
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h + 2,
                    self.sapwcar.pt_ecr_car.v + 1,
                    couleurOmbre,
                );
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h + 1,
                    self.sapwcar.pt_ecr_car.v,
                    couleur,
                );
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h,
                    self.sapwcar.pt_ecr_car.v,
                    couleur,
                );
                self.sapwcar.pt_ecr_car.h += self.sapwbrt.TabMob[num as usize].nbcol + 3;
            }
        }
    }
}

/*************************************************************************
                    GetStrBkMot
**************************************************************************/
impl Sapiens {
    pub(crate) fn GetStrBkMot(&self, car: c_short) -> *mut c_char {
        let vide: *mut c_char = c"".as_ptr() as *mut c_char;
        let bkmots = self.sap_lang.GetBkMot();
        let mut p: *mut c_char = vide;
        let car = car - 129;
        unsafe {
            if car < NB_MOTS_BKMOTS as c_short {
                p = *bkmots.add(car as usize);
                if *p == b'$' as c_char {
                    /* Substitution */
                    let c = *p.add(1);
                    if c == b'0' as c_char {
                        p = addr_of!(self.saptxt.nomsecondaire) as *mut c_char;
                    } else if c == b'1' as c_char {
                        p = addr_of!(self.saptxt.nomjoueur) as *mut c_char;
                    } else if c == b'2' as c_char {
                        p = addr_of!(self.saptxt.nomadversaire) as *mut c_char;
                    } else if c == b'3' as c_char {
                        p = addr_of!(self.saptxt.nomtribu) as *mut c_char;
                    } else if c == b'4' as c_char {
                        p = addr_of!(self.saptxt.nomcopain) as *mut c_char;
                    }
                }
            } else {
                eprintln!("GetStrBkMot error");
            }
        }
        p
    }

    /*************************************************************************
                        AffCarBank
    Affiche un caractere sur le port en entrée
    **************************************************************************/
    fn AffCarBank(
        &mut self,
        port: CGrafPtr,
        car: c_short,
        couleur1: c_short,
        couleur2: c_short,
        taille: c_short,
    ) {
        let _g = port::on(port);
        unsafe {
            if car > 128 {
                let mut p = self.GetStrBkMot(car);
                while *p != 0 {
                    self.AffCarBank(port, *p as u8 as c_short, couleur1, couleur2, taille);
                    p = p.add(1);
                }
            } else {
                self.AffCarBrt(car, couleur1, couleur2, taille);
                self.sapglob.colaff += 1;
            }
        }
    }

    /*************************************************************************
                        affcar
    Affiche le caractere demandé à la position courante
    Les caracteres >128 sont des macros substitions
    **************************************************************************/
    pub(crate) fn affcar(&mut self, car: c_short) {
        self.AffCarBank(
            self.sapwind.debecr,
            car,
            self.sapwcar.index_black,
            self.sapwcar.index_brown,
            14,
        );
    }

    /*************************************************************************
                        affcar10
    Affiche le caractere demandé à la position courante
    Les caracteres >128 sont des macros substitions
    **************************************************************************/
    pub(crate) fn affcar10(&mut self, car: c_short) {
        self.AffCarBank(
            self.sapwind.debecr,
            car,
            self.sapwcar.index_black,
            self.sapwcar.index_gray,
            10,
        );
    }

    /*************************************************************************
                        Car2
    Affiche un caractere blanc sur une ombre noire
    Le caractere est affiché sur debecr et n'est pas transféré
    **************************************************************************/
    pub(crate) fn car2(&mut self, car: c_short) {
        self.AffCarBank(
            self.sapwind.debecr,
            car,
            self.sapwcar.index_white,
            self.sapwcar.index_black,
            14,
        );
    }

    /*************************************************************************
                        affcarmess
    Affiche le caractere demandé à la position courante
    Les caracteres >128 sont des macros substitions
    Le caractere est affiche sur debecr
    **************************************************************************/
    pub(crate) fn affcarmess(&mut self, car: c_short) {
        let _g = port::on(self.sapwind.debecr);
        let rgn: RgnHandle = NewRgn();
        GetClip(rgn);
        ClipRect(addr_of!(self.sapwmess.rMess));
        /* Effacement et affichage curseur */
        if car < 32 {
            let r = Rect::new(
                self.sapwcar.pt_ecr_car.h,
                self.sapwcar.pt_ecr_car.v,
                self.sapwcar.pt_ecr_car.h + 8,
                self.sapwcar.pt_ecr_car.v + 11,
            );
            if car == 2 {
                CarboniseSetPatBlack();
                DoPaintRect(&r);
            }
            if car == 1 {
                DoEraseRect(&r);
            }
        } else {
            self.AffCarBank(
                self.sapwind.debecr,
                car,
                self.sapwcar.index_black,
                self.sapwcar.index_gray,
                14,
            );
        }
        SetClip(rgn);
        DisposeRgn(rgn);
    }

    /***********************************************************************
                            MOTLST2:Affiche Pour jauges
    ***********************************************************************/
    pub(crate) fn motlst2(&mut self, debut_index: usize) {
        let app = self.sap_lang.GetApp();
        let debut = app[debut_index];
        let mut p = debut as *mut u8;
        unsafe {
            while *p != 0 {
                self.car2(*p as c_short);
                p = p.add(1);
            }
        }
    }
}
