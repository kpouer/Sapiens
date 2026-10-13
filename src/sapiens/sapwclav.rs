#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/*************************************************************************
                    FixeAireNoMouse
Fixe l'aire dans laquelle il ne faut plus afficher la souris systeme
**************************************************************************/
use crate::Sapiens;
use crate::acam::{DiffRgn, DisposeRgn, FAUX, GetClip, NewRgn, RectRgn, SetClip, SetRect, VRAI};
use crate::cbridge::{QElemPtr, TMTask};
use crate::exchange::{Cursor, set_cursor};
use crate::port;
use crate::sapiens::ECHELLE;
use crate::sapiens::sapwbrt::CopyOffscreen;
use crate::sapiens::sapwind::SapWind;
use crate::shared::{Boolean, KeyMap, Rect};
use crate::stubs::{Button, CarboniseSetCursorArrow, GetKeys, GetMouse, InsXTime, PrimeTime};
use std::os::raw::{c_short, c_ulong, c_ushort};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

/* Globals de SAPWCLAV.C passes en champs (ex-`static mut`). */
#[derive(Default)]
pub(crate) struct Sapwclav {
    /* Compteur 11 ms (ex-`sapiensTimer`), partage avec le thread timer (main.rs). */
    pub(crate) sapiens_timer: Arc<AtomicU64>,
    /* Aire courante de la souris graphique (ex-`rsouris`) */
    rsouris: Rect,
    /* Aire ou la souris systeme est masquee (ex-`rNoMouse`) */
    r_no_mouse: Rect,
    /* Derniere position tracee de la souris (ex-`lastposx/lastposy`) */
    last_posx: c_short,
    last_posy: c_short,
    /* Tache du timer 11 ms (ex-`timerTask`) */
    timer_task: TMTask,
    /* ex-`isScrollMouse` */
    #[allow(dead_code)]
    is_scroll_mouse: Boolean,
}

impl Sapwclav {
    pub(crate) fn with_sapiens_timer(sapiens_timer: Arc<AtomicU64>) -> Self {
        Sapwclav {
            sapiens_timer,
            ..Default::default()
        }
    }
}

/*************************************************************************
                    InstalleTimeTask
Installe le timer de comptage 11ms pour le ralentissement
**************************************************************************/

impl Sapiens {
    pub(crate) fn InstalleTimeTask(&mut self) {
        self.sapwclav.timer_task.qLink = core::ptr::null_mut();
        self.sapwclav.timer_task.qType = 0;
        self.sapwclav.timer_task.tmAddr = core::ptr::null_mut();
        self.sapwclav.timer_task.tmCount = 0;
        self.sapwclav.timer_task.tmWakeUp = 0;
        self.sapwclav.timer_task.tmReserved = 0;
        InsXTime(&raw mut self.sapwclav.timer_task as QElemPtr);
        PrimeTime(&raw mut self.sapwclav.timer_task as QElemPtr, 11); // 11 milliseconds = 90 fps
    }

    /*************************************************************************
                        litclav
    **************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn litclav(&mut self) -> c_short {
        self.GereInterface();
        let n = self.sapwind.toucheClav;
        self.sapwind.toucheClav = 0;
        n
    }
}

/*************************************************************************
            Eff_sour
Efface la souris graphique menus
Pour effacer le souris on recopie debecr sur l'aire de la souris
**************************************************************************/
impl Sapiens {
    pub(crate) fn eff_sour(&mut self) {
        CopyOffscreen(
            self.sapwind.debecr,
            self.sapwind.mainWindow,
            &raw mut self.sapwclav.rsouris,
            &raw mut self.sapwclav.rsouris,
        );
        SetRect(&raw mut self.sapwclav.rsouris, 0, 0, 0, 0);
    }

    /**************************************************************************
    ;--     Aff_sour_: Affiche la souris graphique menus
    ; => N. mob,Ligne,Colonne
    **************************************************************************/
    pub(crate) fn aff_sour(&mut self, mob: c_short, ligne: c_short, colonne: c_short) {
        let mut rsource: Rect = Rect::default();
        let mut r: Rect = Rect::default();
        let mut oldRect: Rect = self.sapwclav.rsouris;
        /* Effacement a l'ancienne position */
        /* Aire de la souris sur l'ecran */
        let p_rsouris = &raw mut self.sapwclav.rsouris;
        self.CalculeAireMob(mob, colonne, ligne, p_rsouris);
        self.CalculeAireMob(mob, 0, 0, &raw mut rsource);
        /* On gere l'ombre */
        self.sapwclav.rsouris.right += 8;
        self.sapwclav.rsouris.bottom += 8;
        rsource.right += 8;
        rsource.bottom += 8;
        /* Le mob s'affiche sur l'ecran virtuel : on saisie ce qu'il y a
        sur l'ecran */
        CopyOffscreen(
            self.sapwind.debecr,
            self.sapwind.BuffMouse,
            &raw mut self.sapwclav.rsouris,
            &raw mut rsource,
        );
        /* On copie egalement l'ecran physique sur le buffer de la souris */
        /* On affiche la souris */
        let _g = port::on(self.sapwind.BuffMouse);
        self.Affmobbrtomb(mob, 0, 0, 4, 4, &raw mut r);
        /* On le recopie sur le reel */
        CopyOffscreen(
            self.sapwind.BuffMouse,
            self.sapwind.mainWindow,
            &raw mut rsource,
            &raw mut self.sapwclav.rsouris,
        );
        /* On efface la difference (les deux rectangles sont sur l'ecran : ils
        sont a l'echelle de la fenetre, meme si on dessine sur BuffMouse) */
        let rgn1 = NewRgn();
        let rgn2 = NewRgn();
        let rgnClip = NewRgn();
        let r1 = port::zoomed(port::window(), &oldRect);
        let r2 = port::zoomed(port::window(), &self.sapwclav.rsouris);
        RectRgn(rgn1, &r1);
        RectRgn(rgn2, &r2);
        DiffRgn(rgn1, rgn2, rgn1);
        GetClip(rgnClip);
        SetClip(rgn1);
        CopyOffscreen(
            self.sapwind.debecr,
            self.sapwind.mainWindow,
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
pub(crate) fn KeyPressed(km: *mut KeyMap, ascii: c_short) -> Boolean {
    let p = km as *mut c_ulong;
    unsafe { ((*p.add((ascii >> 5) as usize) >> (((ascii ^ 24) & 31) as u32)) & 1) != 0 }
}

/*************************************************************
                    GetKeyMap
***************************************************************/
pub(crate) fn GetKeyMap(bitmap: *mut i16) -> i16 {
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
pub(crate) extern "C" fn IsKey(ascii: c_short) -> Boolean {
    let mut theKeys = KeyMap::default();

    GetKeys(theKeys);
    KeyPressed(&raw mut theKeys, ascii)
}

/*************************************************************
                    GetKeyMapJoueur2
***************************************************************/
fn GetKeyMapJoueur2() -> i16 {
    let mut theKeys = KeyMap::default();

    let mut bitmap = 0;
    GetKeys(theKeys);
    if KeyPressed(&raw mut theKeys, 0x5b) {
        bitmap |= 1;
    }
    if KeyPressed(&raw mut theKeys, 0x54) {
        bitmap |= 2;
    }
    if KeyPressed(&raw mut theKeys, 0x56) {
        bitmap |= 4;
    }
    if KeyPressed(&raw mut theKeys, 0x58) {
        bitmap |= 8;
    }
    if KeyPressed(&raw mut theKeys, 0x53) {
        bitmap |= 32; /* Espace */
    }
    if KeyPressed(&raw mut theKeys, 0x55) {
        bitmap |= 16; /* RC */
    }
    if KeyPressed(&raw mut theKeys, 0x42) {
        bitmap |= 64; /* ESC*/
    }
    bitmap
}

impl Sapiens {
    /*************************************************************
                        GETKEY3D
    Donne un deplacement / a la position 160,100 de la souris
    ***************************************************************/
    pub(crate) fn getkey3d(&mut self) -> c_short {
        let mut bitmap: c_short = 0;

        if GetKeyMap(&raw mut bitmap) == 0 {
            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
            let mut delt = self.sapglob.sposx - self.sapglob.dsposx;
            if delt > 0 && delt > 16 {
                bitmap |= 8;
            }
            if delt < 0 && -delt > 16 {
                bitmap |= 4;
            }
            delt = self.sapglob.sposy - self.sapglob.dsposy;
            if delt > 0 && delt > 8 {
                bitmap |= 2;
            }
            if delt < 0 && -delt > 8 {
                bitmap |= 1;
            }
            if (self.sapglob.bouton & 1) != 0 {
                bitmap |= 64;
            }
            if (self.sapglob.bouton & 2) != 0 {
                bitmap |= 64;
            }
        } else {
            self.reset_mouse();
        }
        bitmap
    }

    /*************************************************************
                        GETKEY
    **************************************************************
    => 0 joeur principal, 1 secondaire
    */
    pub(crate) fn getkey(&mut self, quoi: c_ushort) -> c_short {
        let mut bitmap = 0;

        if quoi != 0 {
            bitmap = GetKeyMapJoueur2();
        } else {
            /* joueur primaire */
            if GetKeyMap(&raw mut bitmap) == 0 {
                (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
                let delt = self.sapglob.sposx - self.sapglob.dsposx;
                if delt > 0 && delt > 10 {
                    bitmap |= 8;
                }
                if delt < 0 && -delt > 10 {
                    bitmap |= 4;
                }
                let delt = self.sapglob.sposy - self.sapglob.dsposy;
                if delt > 0 && delt > 5 {
                    bitmap |= 2;
                }
                if delt < 0 && -delt > 5 {
                    bitmap |= 1;
                }
                if (self.sapglob.bouton & 1) != 0 {
                    bitmap |= 32;
                }
                if (self.sapglob.bouton & 2) != 0 {
                    bitmap |= 16;
                }
            } else {
                self.reset_mouse();
            }
            self.sapglob.dsposx = self.sapglob.sposx;
            self.sapglob.dsposy = self.sapglob.sposy;
        }
        bitmap
    }

    /*************************************************************
                        BOUGE_SOURIS
    => globales :sposx,sposy,bouton,icone
    **************************************************************/
    pub(crate) fn bouge_souris(&mut self) {
        self.GereInterface();
        (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
        /* Attends et prends coord souris */

        /* sortie basse ??? */
        if self.sapglob.sposy > 184 {
            self.sapglob.sposy = 184;
        }
        /* sortie droite */
        if self.sapglob.sposx > 303 {
            self.sapglob.sposx = 303;
        }
        /* si bouge alors retrace */
        if ((self.sapglob.sposx != self.sapwclav.last_posx)
            || (self.sapglob.sposy != self.sapwclav.last_posy)
            || (self.sapglob.icone != self.sapglob.dicone))
            && self.sapglob.icone != 0
        {
            self.sapglob.dicone = self.sapglob.icone;
            self.sapwclav.last_posx = self.sapglob.sposx;
            self.sapwclav.last_posy = self.sapglob.sposy;
            let (icone, sposy, sposx) =
                (self.sapglob.icone, self.sapglob.sposy, self.sapglob.sposx);
            self.aff_sour(icone, sposy, sposx);
        }
    }

    /**************************************************************************
           AttendsFinBouton
    **************************************************************************/
    pub(crate) fn AttendsFinBouton(&mut self) {
        while self.sapglob.bouton != 0 {
            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
        }
    }
}

/*************************************************************************
                    vq_mouse
**************************************************************************/
static rBasse: Rect = Rect {
    top: (116 * ECHELLE) as i16,
    left: 0,
    bottom: (200 * ECHELLE) as i16,
    right: (320 * ECHELLE) as i16,
};

impl Sapiens {
    /**************************************************************************
                      resetmouse
    Fixe le zéro de la souris
    *************************************************************************/
    pub(crate) fn reset_mouse(&mut self) {
        (_, self.sapglob.dsposx, self.sapglob.dsposy) = self.vq_mouse();
    }
}

impl SapWind {
    /***************************************************************************
                            reset_clav
    ****************************************************************************/
    pub(crate) fn reset_clav(&mut self) {
        self.toucheClav = 0;
    }
}

#[allow(non_snake_case)]
pub(crate) fn HideCursor() {
    set_cursor(Cursor::Hidden);
}

#[allow(non_snake_case)]
pub(crate) fn ShowCursor() {
    set_cursor(Cursor::Default);
}

#[derive(Debug, Default)]
pub(crate) struct Souris {
    hidden: bool,
}

impl Souris {
    /*************************************************************************
                        CacheSouris
    **************************************************************************/
    pub(crate) fn CacheSouris(&mut self) {
        if !self.hidden {
            HideCursor();
            self.hidden = VRAI;
        }
    }

    /*************************************************************************
                        MontreSouris
    **************************************************************************/
    pub(crate) fn MontreSouris(&mut self) {
        if self.hidden {
            ShowCursor();
            CarboniseSetCursorArrow();
            self.hidden = FAUX;
        }
    }
}

impl Sapiens {
    pub(crate) fn vq_mouse(&mut self) -> (i16, i16, i16) {
        let mut theKeys = KeyMap::default();

        port::set(self.sapwind.mainWindow);
        self.GereInterface();
        let pt = GetMouse().zoom(port::window_zoom());
        if self.sapwclav.r_no_mouse.pt_in_rect(pt) {
            self.souris.CacheSouris();
        } else {
            self.souris.MontreSouris();
        }
        let x = (pt.h as f32 / ECHELLE as f32) as i16;
        let y = (pt.v as f32 / ECHELLE as f32) as i16;
        let mut bouton_ptr = Button() as i16;
        if bouton_ptr > 0 {
            GetKeys(theKeys);
            if KeyPressed(&raw mut theKeys, 0x38) {
                bouton_ptr |= 2;
            }
        }
        (bouton_ptr, x, y)
    }

    pub(crate) const fn FixeAireNoMouse(&mut self, r: Rect) {
        self.sapwclav.r_no_mouse = r;
    }

    /*************************************************************************
                        FixeAireNoMouseAireBasse
    **************************************************************************/
    pub(crate) const fn FixeAireNoMouseAireBasse(&mut self) {
        self.FixeAireNoMouse(rBasse);
    }
}
