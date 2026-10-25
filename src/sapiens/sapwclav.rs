#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/*************************************************************************
                    FixeAireNoMouse
Fixe l'aire dans laquelle il ne faut plus afficher la souris systeme
**************************************************************************/
use crate::Sapiens;
use crate::acam::{CarboniseSetCursorArrow, FAUX, GetClip, SetClip, VRAI};
use crate::exchange::set_cursor_hidden;
use crate::port;
use crate::sapiens::ECHELLE;
use crate::sapiens::sapwbrt::CopyOffscreen;
use crate::sapiens::sapwind::SapWind;
use crate::shared::{Boolean, KeyMap, Rect};
use crate::stubs::{Button, GetKeys, GetMouse};
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
    last_posx: i16,
    last_posy: i16,
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
    /*************************************************************************
                        litclav
    **************************************************************************/
    pub(crate) fn litclav(&mut self) -> i16 {
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
            &self.sapwclav.rsouris,
            &self.sapwclav.rsouris,
        );
        self.sapwclav.rsouris = Rect::default();
    }

    /**************************************************************************
    ;--     Aff_sour_: Affiche la souris graphique menus
    ; => N. mob,Ligne,Colonne
    **************************************************************************/
    pub(crate) fn aff_sour(&mut self, mob: i16, ligne: i16, colonne: i16) {
        let oldRect = self.sapwclav.rsouris;
        /* Effacement a l'ancienne position */
        /* Aire de la souris sur l'ecran */
        self.sapwclav.rsouris = self.CalculeAireMob(mob, colonne, ligne);
        let mut rsource = self.CalculeAireMob(mob, 0, 0);
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
            &self.sapwclav.rsouris,
            &rsource,
        );
        /* On copie egalement l'ecran physique sur le buffer de la souris */
        /* On affiche la souris */
        {
            let _g = port::on(self.sapwind.BuffMouse);
            let mut r = Rect::default();
            self.Affmobbrtomb(mob, 0, 0, 4, 4, &mut r);
        }
        /* On le recopie sur le reel */
        CopyOffscreen(
            self.sapwind.BuffMouse,
            self.sapwind.mainWindow,
            &rsource,
            &self.sapwclav.rsouris,
        );
        /* On efface la difference (les deux rectangles sont sur l'ecran : ils
        sont a l'echelle de la fenetre, meme si on dessine sur BuffMouse).
        On decompose oldRect - rsouris en rectangles disjoints : une region
        rectangulaire ne peut pas representer le L obtenu quand les deux se
        chevauchent, et effacer le rectangle entier recouvrirait le curseur
        fraichement dessine (clignotement). */
        {
            let _g = port::on(self.sapwind.mainWindow);
            let rgnClip = GetClip().expect("clip");
            for band in difference_rects(&oldRect, &self.sapwclav.rsouris) {
                if band.is_empty() {
                    continue;
                }
                let rgn = port::zoomed(port::window(), &band);
                SetClip(&rgn);
                CopyOffscreen(self.sapwind.debecr, self.sapwind.mainWindow, &band, &band);
            }
            SetClip(&rgnClip);
        }
    }
}

/*************************************************************************
            difference_rects
Decoupe `a - b` en rectangles disjoints (au plus 4).
**************************************************************************/
fn difference_rects(a: &Rect, b: &Rect) -> [Rect; 4] {
    let mut out = [Rect::default(); 4];
    if a.is_empty() {
        return out;
    }
    if b.is_empty() {
        out[0] = *a;
        return out;
    }
    let mut n = 0;
    /* Bande du haut, pleine largeur. */
    let bas = a.bottom.min(b.top);
    if a.top < bas {
        out[n] = Rect::new(a.left, a.top, a.right, bas);
        n += 1;
    }
    /* Bande du bas, pleine largeur. */
    let haut = a.top.max(b.bottom);
    if haut < a.bottom {
        out[n] = Rect::new(a.left, haut, a.right, a.bottom);
        n += 1;
    }
    /* Bande verticale restante, de part et d'autre de b. */
    let vt = a.top.max(b.top);
    let vb = a.bottom.min(b.bottom);
    if vt < vb {
        if a.left < b.left {
            let r = Rect::new(a.left, vt, a.right.min(b.left), vb);
            if !r.is_empty() {
                out[n] = r;
                n += 1;
            }
        }
        if b.right < a.right {
            let r = Rect::new(a.left.max(b.right), vt, a.right, vb);
            if !r.is_empty() {
                out[n] = r;
            }
        }
    }
    out
}

/*********************************************************************
                        KeyPressed
Dit si une touche d'ascii donnee est appuyee sur le keymap en entree
********************************************************************/
pub(crate) fn KeyPressed(km: &KeyMap, ascii: i16) -> Boolean {
    ((km[(ascii >> 5) as usize] >> (((ascii ^ 24) & 31) as u32)) & 1) != 0
}

/*************************************************************
                    GetKeyMap
***************************************************************/
pub(crate) fn GetKeyMap() -> i16 {
    let mut bitmap = 0;
    let theKeys = GetKeys();
    if KeyPressed(&theKeys, 0x7E) {
        bitmap |= 1;
    }
    if KeyPressed(&theKeys, 0x7D) {
        bitmap |= 2;
    }
    if KeyPressed(&theKeys, 0x7B) {
        bitmap |= 4;
    }
    if KeyPressed(&theKeys, 0x7C) {
        bitmap |= 8;
    }
    if KeyPressed(&theKeys, 0x31) {
        bitmap |= 32; /* Espace */
    }
    if KeyPressed(&theKeys, 0x24) {
        bitmap |= 16; /* RC */
    }
    if KeyPressed(&theKeys, 0x35) {
        bitmap |= 64; /* ESC*/
    }
    if KeyPressed(&theKeys, 0x08) {
        bitmap |= 32; /* Carte*/
    }
    if KeyPressed(&theKeys, 0x2e) {
        bitmap |= 32; /* Carte*/
    }
    bitmap
}

/*************************************************************
                    GetKeyMapJoueur2
***************************************************************/
fn GetKeyMapJoueur2() -> i16 {
    let mut bitmap = 0;
    let theKeys = GetKeys();
    if KeyPressed(&theKeys, 0x5b) {
        bitmap |= 1;
    }
    if KeyPressed(&theKeys, 0x54) {
        bitmap |= 2;
    }
    if KeyPressed(&theKeys, 0x56) {
        bitmap |= 4;
    }
    if KeyPressed(&theKeys, 0x58) {
        bitmap |= 8;
    }
    if KeyPressed(&theKeys, 0x53) {
        bitmap |= 32; /* Espace */
    }
    if KeyPressed(&theKeys, 0x55) {
        bitmap |= 16; /* RC */
    }
    if KeyPressed(&theKeys, 0x42) {
        bitmap |= 64; /* ESC*/
    }
    bitmap
}

impl Sapiens {
    /*************************************************************
                        GETKEY3D
    Donne un deplacement / a la position 160,100 de la souris
    ***************************************************************/
    pub(crate) fn getkey3d(&mut self) -> i16 {
        let mut bitmap = GetKeyMap();
        if bitmap == 0 {
            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
            let mut delt = self.sapglob.sposx - self.sapglob.dsposx;
            if delt > 16 {
                bitmap |= 8;
            }
            if delt < 0 && -delt > 16 {
                bitmap |= 4;
            }
            delt = self.sapglob.sposy - self.sapglob.dsposy;
            if delt > 8 {
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
    pub(crate) fn getkey(&mut self, quoi: u16) -> i16 {
        let mut bitmap;

        if quoi != 0 {
            bitmap = GetKeyMapJoueur2();
        } else {
            /* joueur primaire */
            bitmap = GetKeyMap();
            if bitmap == 0 {
                (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
                let delt = self.sapglob.sposx - self.sapglob.dsposx;
                if delt > 10 {
                    bitmap |= 8;
                }
                if delt < 0 && -delt > 10 {
                    bitmap |= 4;
                }
                let delt = self.sapglob.sposy - self.sapglob.dsposy;
                if delt > 5 {
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
    set_cursor_hidden(true);
}

#[allow(non_snake_case)]
pub(crate) fn ShowCursor() {
    set_cursor_hidden(false);
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
            let theKeys = GetKeys();
            if KeyPressed(&theKeys, 0x38) {
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

#[cfg(test)]
mod tests {
    use super::difference_rects;
    use crate::shared::Rect;

    fn area(r: &Rect) -> i32 {
        if r.is_empty() {
            0
        } else {
            (r.right as i32 - r.left as i32) * (r.bottom as i32 - r.top as i32)
        }
    }

    fn aire_diff(a: &Rect, b: &Rect) -> i32 {
        difference_rects(a, b).iter().map(area).sum()
    }

    #[test]
    fn chevauchement_diagonal() {
        let a = Rect::new(10, 10, 20, 20);
        let b = Rect::new(14, 14, 24, 24);
        let parts = difference_rects(&a, &b);
        assert_eq!(aire_diff(&a, &b), 100 - 36);
        for p in &parts {
            assert!(p.intersect(&b).is_empty());
        }
    }

    #[test]
    fn b_inclus_dans_a() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(2, 2, 8, 8);
        let parts = difference_rects(&a, &b);
        assert_eq!(aire_diff(&a, &b), 100 - 36);
        for p in &parts {
            assert!(p.intersect(&b).is_empty());
        }
    }

    #[test]
    fn sans_chevauchement_ou_a_inclus() {
        let a = Rect::new(0, 0, 10, 10);
        assert_eq!(aire_diff(&a, &Rect::new(20, 20, 30, 30)), 100);
        assert_eq!(aire_diff(&a, &Rect::new(-5, -5, 15, 15)), 0);
    }
}
