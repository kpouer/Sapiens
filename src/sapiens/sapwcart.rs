#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, CarboniseGetWindowPort, CarboniseSetPortWindow,
    ForeColor, InsetRect, NewPtr, SectRect, SetRect,
};
use crate::cbridge::{blackColor, whiteColor};
use crate::sapiens::MontPtr;
use crate::sapiens::sap3d::{aigoual, nbreobj};
use crate::sapiens::sapglob::{bouton, dsposy, dzeta, smenu, sposx, sposy, xj, yj, zoom};
use crate::sapiens::sapwbrt::{
    Affmob3d, Affmobbrtomb, BoxFill, CopyOffscreen, DoFrameRect, SapiensCreeOffscreen,
};
use crate::sapiens::sapwclav::{GetKeyMap, vq_mouse};
use crate::sapiens::sapwind::{GereInterface, debecr, mainWindow, mob3D, mobFleches};
use crate::shared::{CGrafPtr, Handle, Rect};
use crate::stubs::{HLock, HUnlock};
use std::os::raw::{c_long, c_short, c_uchar};
use std::ptr::{addr_of, addr_of_mut, null_mut};

const TAILLE_X_CARTE: c_long = 144;
const TAILLE_Y_CARTE: c_long = 80;
const NB_OBJETS_MAX: c_long = 200;
const ECHELLE: c_long = 2;

#[repr(C)]
#[derive(Clone, Copy)]
struct EltCart {
    aire: Rect,
    mob: c_short,
}

static mut EcrCarte: CGrafPtr = null_mut();
static mut TabEltCar: *mut EltCart = null_mut();
static mut AireCarte: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
};

/*;-_-_-_-_-_-_-_-_  Routine 2: transfere carte  sur ecran -_-_-_-_-_-_-_-
*/
unsafe fn tfrcar() {
    let mut rdest = Rect::default();
    SetRect(
        &mut rdest,
        (160 * ECHELLE) as c_short,
        (117 * ECHELLE) as c_short,
        ((160 + TAILLE_X_CARTE) * ECHELLE) as c_short,
        ((117 + TAILLE_Y_CARTE) * ECHELLE) as c_short,
    );
    CopyOffscreen(EcrCarte, debecr, addr_of_mut!(AireCarte), &mut rdest);
    CopyOffscreen(
        EcrCarte,
        CarboniseGetWindowPort(mainWindow),
        addr_of_mut!(AireCarte),
        &mut rdest,
    );
}

/*
;-_-_-_-_-_-_-_-_  Routine 1: calcule positions sur ecran -_-_-_-_-_-_-_-
; => Pointeur sur la structure montagne ,Zoom_
; <= Xc,Yc (Yc à 0 si montagne invisible)
*/
unsafe fn calc_cart(mont: MontPtr, eltCart: *mut EltCart) -> u8 {
    let forme = (*mont).ptrforme;
    /* Position en ligne */
    let x = ((yj as i64 - (*mont).posy as i64) / (zoom as i64)) + TAILLE_X_CARTE;
    let y = ((xj as i64 - (*mont).posx as i64) / (zoom as i64)) + TAILLE_Y_CARTE;
    let r = (*forme).rayon as i64 / (zoom as i64 / 2);
    if (*forme).num_mob == 8 {
        /* (num_mob==BASEMOB+8) Fumee -> BASEMOB=0 */
        SetRect(
            &mut (*eltCart).aire,
            (x - 8) as c_short,
            (y - 32) as c_short,
            (x + 8) as c_short,
            (y + 32) as c_short,
        );
    } else if r < 16 {
        SetRect(
            &mut (*eltCart).aire,
            (x - 8) as c_short,
            (y - 8) as c_short,
            (x + 8) as c_short,
            (y + 8) as c_short,
        );
    } else {
        SetRect(
            &mut (*eltCart).aire,
            (x - r / 2) as c_short,
            (y - r / 8) as c_short,
            (x + r / 2) as c_short,
            (y + r / 8) as c_short,
        );
    }
    let mut rb = Rect::default();
    if SectRect(&(*eltCart).aire, addr_of!(AireCarte), &mut rb) {
        (*eltCart).mob = mob3D + (*forme).numMob3d;
        return 1; /* VRAI */
    }
    0 /* FAUX */
}

/*************************************************************************
                Affichage des montagnes sur la carte
=> Nombre d'objets a afficher
***************************************************************************/
unsafe fn aff_mont(nb: c_short) {
    loop {
        let mut minyc;
        let mut jmin: c_short = 0;
        minyc = 32767;
        for j in 0..nb {
            if (*TabEltCar.add(j as usize)).aire.bottom < minyc {
                jmin = j;
                minyc = (*TabEltCar.add(j as usize)).aire.bottom;
            }
        }
        if minyc == 32767 {
            return;
        }
        Affmob3d(
            (*TabEltCar.add(jmin as usize)).mob,
            &mut (*TabEltCar.add(jmin as usize)).aire,
            null_mut(),
        );
        (*TabEltCar.add(jmin as usize)).aire.bottom = 32767;
    }
}

/***********************************************************************
                         EffCarte
Efface le fond de la carte
***********************************************************************/
unsafe fn EffCarte() {
    let mut r = Rect::default();
    SetRect(
        &mut r,
        0,
        0,
        (TAILLE_X_CARTE * ECHELLE) as c_short,
        (TAILLE_Y_CARTE * ECHELLE) as c_short,
    );
    BoxFill(EcrCarte, &mut r, 0x050); /* palhaute[3]*/
}

/***********************************************************************
                         CARTE:
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn carte() {
    let mut i: c_long;
    let mut j: c_long;
    let angle: c_short;
    let mut mont: MontPtr;
    let mut gbak: CGrafPtr = null_mut();
    let mut r = Rect::default();

    /* deconnecte sous menu */
    if EcrCarte.is_null() {
        EcrCarte = SapiensCreeOffscreen(
            (TAILLE_X_CARTE * ECHELLE) as c_short,
            (TAILLE_Y_CARTE * ECHELLE) as c_short,
        );
        if EcrCarte.is_null() {
            return;
        }
        TabEltCar = NewPtr(NB_OBJETS_MAX * size_of::<EltCart>() as c_long) as *mut EltCart;
        if TabEltCar.is_null() {
            return;
        }
        SetRect(
            addr_of_mut!(AireCarte),
            0,
            0,
            (TAILLE_X_CARTE * ECHELLE) as c_short,
            (TAILLE_Y_CARTE * ECHELLE) as c_short,
        );
    }
    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(EcrCarte);
    (*addr_of_mut!(smenu))[1] = 111;
    /* efface carte */
    EffCarte();
    /* trie les objets */
    HLock(aigoual as Handle);
    mont = *aigoual;
    i = nbreobj;
    j = 0;
    while i != 0 && j < NB_OBJETS_MAX {
        i -= 1;
        if (*mont).ptrforme.is_null() == false && (*(*mont).ptrforme).num_mob >= 0 {
            if calc_cart(mont, &mut *TabEltCar.add(j as usize)) != 0 {
                j += 1;
            }
        }
        mont = mont.add(1);
    }
    HUnlock(aigoual as Handle);
    /* affichage */
    aff_mont(j as c_short);
    /* on affiche la fleche */
    angle = ((dzeta + 32) / 64) & 15;
    Affmobbrtomb(
        mobFleches + 1 + angle,
        (TAILLE_X_CARTE / 2 - 8) as c_short,
        (TAILLE_Y_CARTE / 2 - 8) as c_short,
        5,
        5,
        &mut r,
    );
    /* et on transfere */
    tfrcar();
    AcamSetPortOffscreen(gbak);
}

/***********************************************************************
                       ChangeZoomCarte
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeZoomCarte(delta: c_short) {
    let mut z = zoom as c_short + delta;
    if z > 160 {
        z = 160;
    }
    if z < 16 {
        z = 16;
    }
    zoom = z as c_uchar;
    carte();
}

/***********************************************************************
                         ZOOCAR:Gere le zoom
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zoocar() {
    let mut bitmap: c_short = 0;
    let mut r = Rect::default();
    let mut gbak: CGrafPtr = null_mut();

    AcamGetPortOffscreen(&mut gbak);
    CarboniseSetPortWindow(mainWindow);

    loop {
        vq_mouse(
            addr_of_mut!(bouton),
            addr_of_mut!(sposx),
            addr_of_mut!(sposy),
        );
        if bouton == 0 {
            break;
        }
    }
    SetRect(
        &mut r,
        (160 * ECHELLE) as c_short,
        (117 * ECHELLE) as c_short,
        ((160 + TAILLE_X_CARTE) * ECHELLE) as c_short,
        ((117 + TAILLE_Y_CARTE) * ECHELLE) as c_short,
    );
    InsetRect(&mut r, -1, -1);
    ForeColor(whiteColor as i16);
    DoFrameRect(&mut r);
    ForeColor(blackColor as i16);
    carte();
    loop {
        GereInterface();
        if GetKeyMap(&mut bitmap) != 0 {
            if bitmap & 1 != 0 {
                ChangeZoomCarte(-2);
            }
            if bitmap & 2 != 0 {
                ChangeZoomCarte(2);
            }
            if bitmap & 64 != 0 {
                bouton = 1;
            }
        } else {
            vq_mouse(
                addr_of_mut!(bouton),
                addr_of_mut!(sposx),
                addr_of_mut!(sposy),
            );
            let s = *addr_of!(sposy);
            if s != *addr_of!(dsposy) {
                zoom = (s / 2 + 30) as c_uchar;
                if zoom < 16 {
                    zoom = 16;
                }
                if zoom > 160 {
                    zoom = 160;
                }
                *addr_of_mut!(dsposy) = s;
                carte();
            }
        }
        if bouton != 0 {
            break;
        }
    }
    DoFrameRect(&mut r);
    AcamSetPortOffscreen(gbak);
}
