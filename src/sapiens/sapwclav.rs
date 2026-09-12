#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/*************************************************************************
                    FixeAireNoMouse
Fixe l'aire dans laquelle il ne faut plus afficher la souris systeme
**************************************************************************/
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, CarboniseGetWindowPort, CarboniseSetPortWindow,
    DiffRgn, DisposeRgn, FAUX, GetClip, NewRgn, PtInRect, RectRgn, SetClip, SetRect, VRAI,
};
use crate::cbridge::{QElemPtr, TMTask, TMTaskPtr};
use crate::sapiens::ECHELLE;
use crate::sapiens::sapglob::{bouton, dicone, dsposx, dsposy, icone, sposx, sposy};
use crate::sapiens::sapwbrt::{Affmobbrtomb, CalculeAireMob, CopyOffscreen, DoZoomRect};
use crate::sapiens::sapwind::{BuffMouse, GereInterface, SapZoom, debecr, mainWindow, toucheClav};
use crate::shared::{Boolean, CGrafPtr, KeyMap, Point, Rect};
use crate::stubs::{
    Button, CarboniseSetCursorArrow, GetKeys, GetMouse, HideCursor, InsXTime, PrimeTime, ShowCursor,
};
use std::os::raw::{c_short, c_ulong, c_ushort, c_void};
/* Timer 11 millisec et souris de defilement (globals de SAPWCLAV.C) */
#[unsafe(no_mangle)]
pub static mut sapiensTimer: c_ulong = 0;
#[unsafe(no_mangle)]
pub static mut isScrollMouse: Boolean = false;

/*************************************************************************
                    InstalleTimeTask
Installe le timer de comptage 11ms pour le ralentissement
**************************************************************************/

fn TimerCallbackProc(tmTaskPtr: TMTaskPtr) {
    unsafe {
        let cur = core::ptr::read_volatile(&raw const sapiensTimer);
        core::ptr::write_volatile(&raw mut sapiensTimer, cur.wrapping_add(1));
        PrimeTime(tmTaskPtr as QElemPtr, 11); // reset the timer
    }
}

static mut timerTask: TMTask = TMTask {
    qLink: core::ptr::null_mut(),
    qType: 0,
    tmAddr: core::ptr::null_mut(),
    tmCount: 0,
    tmWakeUp: 0,
    tmReserved: 0,
};

#[unsafe(no_mangle)]
pub extern "C" fn InstalleTimeTask() {
    unsafe {
        timerTask.qLink = core::ptr::null_mut();
        timerTask.qType = 0;
        timerTask.tmAddr = TimerCallbackProc as *const () as *mut c_void;
        timerTask.tmCount = 0;
        timerTask.tmWakeUp = 0;
        timerTask.tmReserved = 0;
        InsXTime(&raw mut timerTask as QElemPtr);
        PrimeTime(&raw mut timerTask as QElemPtr, 11); // 11 milliseconds = 90 fps
    }
}

/*************************************************************************
                    litclav
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn litclav() -> c_short {
    GereInterface();
    let n: c_short;
    unsafe {
        n = toucheClav;
        toucheClav = 0;
    }
    n
}

/*************************************************************************
                    wait_mouse
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn wait_mouse(bouton_ptr: *mut c_short, x: *mut c_short, y: *mut c_short) {
    unsafe {
        let orgx = *x;
        let orgy = *y;
        let orgbouton = *bouton_ptr;
        loop {
            vq_mouse(bouton_ptr, x, y);
            if *x != orgx || *y != orgy || *bouton_ptr != orgbouton {
                break;
            }
        }
    }
}

/*************************************************************************
            Eff_sour
Efface la souris graphique menus
Pour effacer le souris on recopie debecr sur l'aire de la souris
**************************************************************************/
static mut rsouris: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
};

#[unsafe(no_mangle)]
pub extern "C" fn eff_sour() {
    unsafe {
        CopyOffscreen(
            debecr,
            CarboniseGetWindowPort(mainWindow),
            &raw mut rsouris,
            &raw mut rsouris,
        );
        SetRect(&raw mut rsouris, 0, 0, 0, 0);
    }
}

/**************************************************************************
;--     Aff_sour_: Affiche la souris graphique menus
; => N. mob,Ligne,Colonne
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn aff_sour(mob: c_short, ligne: c_short, colonne: c_short) {
    unsafe {
        let mut gbak: CGrafPtr = core::ptr::null_mut();
        let mut rsource: Rect = Rect::default();
        let mut r: Rect = Rect::default();
        let mut oldRect: Rect = rsouris;
        let mut r1: Rect = Rect::default();
        let mut r2: Rect = Rect::default();

        /* Effacement a l'ancienne position */
        /* Aire de la souris sur l'ecran */
        CalculeAireMob(mob, colonne, ligne, &raw mut rsouris);
        CalculeAireMob(mob, 0, 0, &raw mut rsource);
        /* On gere l'ombre */
        rsouris.right += 8;
        rsouris.bottom += 8;
        rsource.right += 8;
        rsource.bottom += 8;
        /* Le mob s'affiche sur l'ecran virtuel : on saisie ce qu'il y a
        sur l'ecran */
        CopyOffscreen(debecr, BuffMouse, &raw mut rsouris, &raw mut rsource);
        /* On copie egalement l'ecran physique sur le buffer de la souris */
        /* On affiche la souris */
        AcamGetPortOffscreen(&raw mut gbak);
        AcamSetPortOffscreen(BuffMouse);
        Affmobbrtomb(mob, 0, 0, 4, 4, &raw mut r);
        AcamSetPortOffscreen(gbak);
        /* On le recopie sur le reel */
        CopyOffscreen(
            BuffMouse,
            CarboniseGetWindowPort(mainWindow),
            &raw mut rsource,
            &raw mut rsouris,
        );
        /* On efface la difference */
        let rgn1 = NewRgn();
        let rgn2 = NewRgn();
        let rgnClip = NewRgn();
        DoZoomRect(&raw mut oldRect, &raw mut r1);
        DoZoomRect(&raw mut rsouris, &raw mut r2);
        RectRgn(rgn1, &r1);
        RectRgn(rgn2, &r2);
        DiffRgn(rgn1, rgn2, rgn1);
        GetClip(rgnClip);
        SetClip(rgn1);
        CopyOffscreen(
            debecr,
            CarboniseGetWindowPort(mainWindow),
            &raw mut oldRect,
            &raw mut oldRect,
        );
        SetClip(rgnClip);
        DisposeRgn(rgnClip);
        DisposeRgn(rgn1);
        DisposeRgn(rgn2);
    }
}

/*********************************************************************
                        KeyPressed
Dit si une touche d'ascii donnee est appuyee sur le keymap en entree
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn KeyPressed(km: *mut KeyMap, ascii: c_short) -> Boolean {
    unsafe {
        let p = km as *mut c_ulong;
        ((*p.add((ascii >> 5) as usize) >> (((ascii ^ 24) & 31) as u32)) & 1) != 0
    }
}

/*************************************************************
                    GetKeyMap
***************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn GetKeyMap(bitmap: *mut c_short) -> c_short {
    let mut theKeys = KeyMap::default();

    unsafe {
        *bitmap = 0;
        GetKeys(theKeys);
        if KeyPressed(&raw mut theKeys, 0x7E) {
            *bitmap |= 1;
        }
        if KeyPressed(&raw mut theKeys, 0x7D) {
            *bitmap |= 2;
        }
        if KeyPressed(&raw mut theKeys, 0x7B) {
            *bitmap |= 4;
        }
        if KeyPressed(&raw mut theKeys, 0x7C) {
            *bitmap |= 8;
        }
        if KeyPressed(&raw mut theKeys, 0x31) {
            *bitmap |= 32; /* Espace */
        }
        if KeyPressed(&raw mut theKeys, 0x24) {
            *bitmap |= 16; /* RC */
        }
        if KeyPressed(&raw mut theKeys, 0x35) {
            *bitmap |= 64; /* ESC*/
        }
        if KeyPressed(&raw mut theKeys, 0x08) {
            *bitmap |= 32; /* Carte*/
        }
        if KeyPressed(&raw mut theKeys, 0x2e) {
            *bitmap |= 32; /* Carte*/
        }
    }
    unsafe { *bitmap }
}

/*********************************************************************
                        IsKey
  Dit si une touche d'ascii donnee est appuyee sur le keymap en entree
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn IsKey(ascii: c_short) -> Boolean {
    let mut theKeys = KeyMap::default();

    GetKeys(theKeys);
    KeyPressed(&raw mut theKeys, ascii)
}

/*************************************************************
                    GetKeyMapJoueur2
***************************************************************/
fn GetKeyMapJoueur2(bitmap: *mut c_short) {
    let mut theKeys = KeyMap::default();

    unsafe {
        *bitmap = 0;
        GetKeys(theKeys);
        if KeyPressed(&raw mut theKeys, 0x5b) {
            *bitmap |= 1;
        }
        if KeyPressed(&raw mut theKeys, 0x54) {
            *bitmap |= 2;
        }
        if KeyPressed(&raw mut theKeys, 0x56) {
            *bitmap |= 4;
        }
        if KeyPressed(&raw mut theKeys, 0x58) {
            *bitmap |= 8;
        }
        if KeyPressed(&raw mut theKeys, 0x53) {
            *bitmap |= 32; /* Espace */
        }
        if KeyPressed(&raw mut theKeys, 0x55) {
            *bitmap |= 16; /* RC */
        }
        if KeyPressed(&raw mut theKeys, 0x42) {
            *bitmap |= 64; /* ESC*/
        }
    }
}

/*************************************************************
                    GETKEY3D
Donne un deplacement / a la position 160,100 de la souris
***************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn getkey3d() -> c_short {
    let mut bitmap: c_short = 0;

    unsafe {
        if GetKeyMap(&raw mut bitmap) == 0 {
            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
            let mut delt = sposx - dsposx;
            if delt > 0 && delt > 16 {
                bitmap |= 8;
            }
            if delt < 0 && -delt > 16 {
                bitmap |= 4;
            }
            delt = sposy - dsposy;
            if delt > 0 && delt > 8 {
                bitmap |= 2;
            }
            if delt < 0 && -delt > 8 {
                bitmap |= 1;
            }
            if (bouton & 1) != 0 {
                bitmap |= 64;
            }
            if (bouton & 2) != 0 {
                bitmap |= 64;
            }
        } else {
            reset_mouse();
        }
    }
    bitmap
}

/*************************************************************
                    GETKEY
**************************************************************
=> 0 joeur principal, 1 secondaire
*/
#[unsafe(no_mangle)]
pub extern "C" fn getkey(quoi: c_ushort) -> c_short {
    let mut bitmap: c_short = 0;

    unsafe {
        if quoi != 0 {
            GetKeyMapJoueur2(&raw mut bitmap);
        } else {
            /* joueur primaire */
            if GetKeyMap(&raw mut bitmap) == 0 {
                let mut delt: c_short;
                vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                delt = sposx - dsposx;
                if delt > 0 && delt > 10 {
                    bitmap |= 8;
                }
                if delt < 0 && -delt > 10 {
                    bitmap |= 4;
                }
                delt = sposy - dsposy;
                if delt > 0 && delt > 5 {
                    bitmap |= 2;
                }
                if delt < 0 && -delt > 5 {
                    bitmap |= 1;
                }
                if (bouton & 1) != 0 {
                    bitmap |= 32;
                }
                if (bouton & 2) != 0 {
                    bitmap |= 16;
                }
            } else {
                reset_mouse();
            }
            dsposx = sposx;
            dsposy = sposy;
        }
    }
    bitmap
}

/*************************************************************
                    BOUGE_SOURIS
=> globales :sposx,sposy,bouton,icone
**************************************************************/
static mut lastposx: c_short = 0;
static mut lastposy: c_short = 0;

#[unsafe(no_mangle)]
pub extern "C" fn bouge_souris() {
    unsafe {
        GereInterface();
        vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
        /* Attends et prends coord souris */

        /* sortie basse ??? */
        if sposy > 184 {
            sposy = 184;
        }
        /* sortie droite */
        if sposx > 303 {
            sposx = 303;
        }
        /* si bouge alors retrace */
        if ((sposx != lastposx) || (sposy != lastposy) || (icone != dicone)) && icone != 0 {
            dicone = icone;
            lastposx = sposx;
            lastposy = sposy;
            aff_sour(icone, sposy, sposx);
        }
    }
}

/**************************************************************************
       AttendsFinBouton
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AttendsFinBouton() {
    unsafe {
        while bouton != 0 {
            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
        }
    }
}

/*************************************************************************
                    vq_mouse
**************************************************************************/
static mut rBasse: Rect = Rect {
    top: (116 * ECHELLE) as i16,
    left: 0,
    bottom: (200 * ECHELLE) as i16,
    right: (320 * ECHELLE) as i16,
};
static mut rNoMouse: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
};

/**************************************************************************
                  resetmouse
Fixe le zéro de la souris
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn reset_mouse() {
    let mut b: c_short = 0;

    unsafe {
        vq_mouse(&raw mut b, &raw mut dsposx, &raw mut dsposy);
    }
}
/***************************************************************************
                        reset_clav
****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn reset_clav() {
    unsafe {
        toucheClav = 0;
    }
}

static mut hidden: bool = false;

/*************************************************************************
                    CacheSouris
**************************************************************************/
fn CacheSouris() {
    unsafe {
        if !hidden {
            HideCursor();
            hidden = VRAI;
        }
    }
}
/*************************************************************************
                    MontreSouris
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn MontreSouris() {
    unsafe {
        if hidden {
            ShowCursor();
            CarboniseSetCursorArrow();
            hidden = FAUX;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vq_mouse(
    bouton_ptr: *mut ::std::os::raw::c_short,
    x: *mut ::std::os::raw::c_short,
    y: *mut ::std::os::raw::c_short,
) {
    let mut pt = Point::default();
    let mut theKeys = KeyMap::default();

    unsafe {
        CarboniseSetPortWindow(mainWindow);
        GereInterface();
        GetMouse(&mut pt);
        pt.h = (pt.h as f32 / SapZoom) as i16;
        pt.v = (pt.v as f32 / SapZoom) as i16;
        if PtInRect(pt, &raw mut rNoMouse) {
            CacheSouris();
        } else {
            MontreSouris();
        }
        *x = (pt.h as f32 / ECHELLE as f32) as c_short;
        *y = (pt.v as f32 / ECHELLE as f32) as c_short;
        *bouton_ptr = Button() as c_short;
        if (*bouton_ptr) > 0 {
            GetKeys(theKeys);
            if KeyPressed(&raw mut theKeys, 0x38) {
                *bouton_ptr |= 2;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn FixeAireNoMouse(r: *mut Rect) {
    unsafe {
        rNoMouse = *r;
    }
}
/*************************************************************************
                    FixeAireNoMouseAireBasse
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn FixeAireNoMouseAireBasse() {
    unsafe {
        FixeAireNoMouse(&raw mut rBasse);
    }
}
