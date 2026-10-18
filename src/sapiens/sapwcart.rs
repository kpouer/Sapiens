#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::Sapiens;
use crate::acam::{ForeColor, SectRect};
use crate::cbridge::{blackColor, whiteColor};
use crate::port;
use crate::sapiens::sapwbrt::{CopyOffscreen, DoFrameRect, SapiensCreeOffscreen};
use crate::sapiens::sapwclav::GetKeyMap;
use crate::sapiens::{FormeId, Mont};
use crate::shared::{CGrafPtr, Rect};
use log::error;
use std::ptr::{addr_of, addr_of_mut, null_mut};

const TAILLE_X_CARTE: i64 = 144;
const TAILLE_Y_CARTE: i64 = 80;
const NB_OBJETS_MAX: usize = 200;
const ECHELLE: i64 = 2;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct EltCart {
    aire: Rect,
    mob: i16,
}

/* Etat mutable de SAPWCART.C, ex-`static mut`, regroupe dans cette
structure, champ de `Sapiens`. */
pub(crate) struct Sapwcart {
    /* ex-`EcrCarte` */
    ecr_carte: CGrafPtr,
    /* ex-`TabEltCar` */
    tab_elt_car: Option<[EltCart; NB_OBJETS_MAX]>,
    /* ex-`AireCarte` */
    aire_carte: Rect,
}

impl Default for Sapwcart {
    fn default() -> Self {
        Self {
            ecr_carte: null_mut(),
            tab_elt_car: None,
            aire_carte: Rect::default(),
        }
    }
}

impl Sapiens {
    /*;-_-_-_-_-_-_-_-_  Routine 2: transfere carte  sur ecran -_-_-_-_-_-_-_-
     */
    fn tfrcar(&mut self) {
        let rdest = Rect::new(
            (160 * ECHELLE) as i16,
            (117 * ECHELLE) as i16,
            ((160 + TAILLE_X_CARTE) * ECHELLE) as i16,
            ((117 + TAILLE_Y_CARTE) * ECHELLE) as i16,
        );
        CopyOffscreen(
            self.sapwcart.ecr_carte,
            self.sapwind.debecr,
            addr_of_mut!(self.sapwcart.aire_carte),
            &rdest,
        );
        CopyOffscreen(
            self.sapwcart.ecr_carte,
            self.sapwind.mainWindow,
            addr_of_mut!(self.sapwcart.aire_carte),
            &rdest,
        );
    }

    /*
    ;-_-_-_-_-_-_-_-_  Routine 1: calcule positions sur ecran -_-_-_-_-_-_-_-
    ; => Pointeur sur la structure montagne ,Zoom_
    ; <= Xc,Yc (Yc à 0 si montagne invisible)
    */
    fn calc_cart(&self, mont: &Mont, eltCart_index: usize) -> bool {
        let Some(forme) = mont.forme.map(FormeId::forme) else {
            return false;
        };
        /* Position en ligne */
        let x = ((self.sapglob.yj as i64 - mont.posy as i64) / (self.sapglob.zoom as i64))
            + TAILLE_X_CARTE;
        let y = ((self.sapglob.xj as i64 - mont.posx as i64) / (self.sapglob.zoom as i64))
            + TAILLE_Y_CARTE;
        let r = forme.rayon as i64 / (self.sapglob.zoom as i64 / 2);
        let Some(eltCart) = &mut self.sapwcart.tab_elt_car.map(|t| t[eltCart_index]) else {
            return false;
        };
        if forme.num_mob == 8 {
            /* (num_mob==BASEMOB+8) Fumee -> BASEMOB=0 */
            eltCart.aire = Rect::new(
                (x - 8) as i16,
                (y - 32) as i16,
                (x + 8) as i16,
                (y + 32) as i16,
            );
        } else if r < 16 {
            eltCart.aire = Rect::new(
                (x - 8) as i16,
                (y - 8) as i16,
                (x + 8) as i16,
                (y + 8) as i16,
            );
        } else {
            eltCart.aire = Rect::new(
                (x - r / 2) as i16,
                (y - r / 8) as i16,
                (x + r / 2) as i16,
                (y + r / 8) as i16,
            );
        }
        let mut rb = Rect::default();
        if SectRect(&eltCart.aire, addr_of!(self.sapwcart.aire_carte), &mut rb) {
            eltCart.mob = self.sapwind.mob3D + forme.numMob3d;
            return true; /* VRAI */
        }
        false /* FAUX */
    }
}

/*************************************************************************
                Affichage des montagnes sur la carte
=> Nombre d'objets a afficher
***************************************************************************/
impl Sapiens {
    fn aff_mont(&mut self, nb: i16) {
        loop {
            let mut jmin = 0;
            let mut minyc = 32767;
            if let Some(tab_elt_car) = &self.sapwcart.tab_elt_car {
                for j in 0..nb as usize {
                    if tab_elt_car[j].aire.bottom < minyc {
                        jmin = j;
                        minyc = tab_elt_car[j].aire.bottom;
                    }
                }
                if minyc == 32767 {
                    return;
                }
                self.Affmob3d(tab_elt_car[jmin].mob, tab_elt_car[jmin].aire, null_mut());
            }
            if let Some(tab_elt_car) = &mut self.sapwcart.tab_elt_car {
                tab_elt_car[jmin].aire.bottom = 32767;
            } else {
                error!("self.sapwcart.tab_elt_car should be something");
            }
        }
    }
}

impl Sapiens {
    /***********************************************************************
                             EffCarte
    Efface le fond de la carte
    ***********************************************************************/
    fn EffCarte(&self) {
        let mut r = Rect::new(
            0,
            0,
            (TAILLE_X_CARTE * ECHELLE) as i16,
            (TAILLE_Y_CARTE * ECHELLE) as i16,
        );
        self.BoxFill(self.sapwcart.ecr_carte, &mut r, 0x050); /* palhaute[3]*/
    }

    /***********************************************************************
                             CARTE:
    ***********************************************************************/
    pub(crate) fn carte(&mut self) {
        let mut r = Rect::default();

        /* deconnecte sous menu */
        if self.sapwcart.ecr_carte.is_null() {
            self.sapwcart.ecr_carte = SapiensCreeOffscreen(
                (TAILLE_X_CARTE * ECHELLE) as i16,
                (TAILLE_Y_CARTE * ECHELLE) as i16,
            );
            if self.sapwcart.ecr_carte.is_null() {
                return;
            }
            self.sapwcart.tab_elt_car = Some([EltCart::default(); NB_OBJETS_MAX]);
            self.sapwcart.aire_carte = Rect::new(
                0,
                0,
                (TAILLE_X_CARTE * ECHELLE) as i16,
                (TAILLE_Y_CARTE * ECHELLE) as i16,
            );
        }
        let _g = port::on(self.sapwcart.ecr_carte);
        self.sapglob.smenu[1] = 111;
        /* efface carte */
        self.EffCarte();
        /* trie les objets */
        let mut j = 0;
        for mont in &self.sap3d.aigoual {
            if j >= NB_OBJETS_MAX {
                break;
            }
            if mont.forme.is_some_and(|id| id.forme().num_mob >= 0) {
                if self.calc_cart(mont, j) {
                    j += 1;
                }
            }
        }
        /* affichage */
        self.aff_mont(j as i16);
        /* on affiche la fleche */
        let angle = ((self.sapglob.dzeta + 32) / 64) & 15;
        self.Affmobbrtomb(
            self.sapwind.mobFleches + 1 + angle,
            (TAILLE_X_CARTE / 2 - 8) as i16,
            (TAILLE_Y_CARTE / 2 - 8) as i16,
            5,
            5,
            &mut r,
        );
        /* et on transfere */
        self.tfrcar();
    }

    /***********************************************************************
                           ChangeZoomCarte
    ***********************************************************************/
    pub(crate) fn ChangeZoomCarte(&mut self, delta: i16) {
        let mut z = self.sapglob.zoom as i16 + delta;
        if z > 160 {
            z = 160;
        }
        if z < 16 {
            z = 16;
        }
        self.sapglob.zoom = z as u8;
        self.carte();
    }

    /***********************************************************************
                             ZOOCAR:Gere le zoom
    ***********************************************************************/
    pub(crate) fn zoocar(&mut self) {
        let _g = port::on(self.sapwind.mainWindow);

        loop {
            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
            if self.sapglob.bouton == 0 {
                break;
            }
        }
        let mut r = Rect::new(
            (160 * ECHELLE) as i16,
            (117 * ECHELLE) as i16,
            ((160 + TAILLE_X_CARTE) * ECHELLE) as i16,
            ((117 + TAILLE_Y_CARTE) * ECHELLE) as i16,
        );
        r.InsetRect(-1, -1);
        ForeColor(whiteColor as i16);
        DoFrameRect(&r);
        ForeColor(blackColor as i16);
        self.carte();
        loop {
            self.GereInterface();
            let bitmap = GetKeyMap();
            if bitmap != 0 {
                if bitmap & 1 != 0 {
                    self.ChangeZoomCarte(-2);
                }
                if bitmap & 2 != 0 {
                    self.ChangeZoomCarte(2);
                }
                if bitmap & 64 != 0 {
                    self.sapglob.bouton = 1;
                }
            } else {
                (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
                let s = self.sapglob.sposy;
                if s != self.sapglob.dsposy {
                    self.sapglob.zoom = (s / 2 + 30) as u8;
                    if self.sapglob.zoom < 16 {
                        self.sapglob.zoom = 16;
                    }
                    if self.sapglob.zoom > 160 {
                        self.sapglob.zoom = 160;
                    }
                    self.sapglob.dsposy = s;
                    self.carte();
                }
            }
            if self.sapglob.bouton != 0 {
                break;
            }
        }
        DoFrameRect(&r);
    }
}
