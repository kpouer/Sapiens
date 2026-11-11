#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::acam::SectRect;
use crate::cbridge::{blackColor, whiteColor};
use crate::model::Rect;
use crate::port::PortId;
use crate::sapiens::Sapiens;
use crate::sapiens::sapwbrt::SapiensCreeOffscreen;
use crate::sapiens::sapwclav::GetKeyMap;
use crate::sapiens::{FormeId, Mont};
use crate::shared::SapPort;

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
    pub(crate) ecr_carte: SapPort,
    /* ex-`TabEltCar` */
    tab_elt_car: [EltCart; NB_OBJETS_MAX],
    /* ex-`AireCarte` */
    aire_carte: Rect,
}

impl Default for Sapwcart {
    fn default() -> Self {
        Self {
            ecr_carte: SapiensCreeOffscreen(
                (TAILLE_X_CARTE * ECHELLE) as i16,
                (TAILLE_Y_CARTE * ECHELLE) as i16,
            ),
            tab_elt_car: [EltCart::default(); NB_OBJETS_MAX],
            aire_carte: Rect::new(
                0,
                0,
                (TAILLE_X_CARTE * ECHELLE) as i16,
                (TAILLE_Y_CARTE * ECHELLE) as i16,
            ),
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
        self.sapwind.ports.CopyOffscreen(
            PortId::EcrCarte,
            PortId::Debecr,
            self.sapwind.ports.sapwcart.aire_carte,
            &rdest,
        );
        self.sapwind.ports.CopyOffscreen(
            PortId::EcrCarte,
            PortId::MainWindow,
            self.sapwind.ports.sapwcart.aire_carte,
            &rdest,
        );
    }

    /*
    ;-_-_-_-_-_-_-_-_  Routine 1: calcule positions sur ecran -_-_-_-_-_-_-_-
    ; => Pointeur sur la structure montagne ,Zoom_
    ; <= Xc,Yc (Yc à 0 si montagne invisible)
    */
    fn calc_cart(&mut self, mont: &Mont, eltCart_index: usize) -> bool {
        let Some(forme) = mont.forme.map(FormeId::forme) else {
            return false;
        };
        /* Position en ligne */
        let x = ((self.sapglob.yj as i64 - mont.posy as i64) / (self.sapglob.zoom as i64))
            + TAILLE_X_CARTE;
        let y = ((self.sapglob.xj as i64 - mont.posx as i64) / (self.sapglob.zoom as i64))
            + TAILLE_Y_CARTE;
        let r = forme.rayon as i64 / (self.sapglob.zoom as i64 / 2);
        let eltCart = &mut self.sapwind.ports.sapwcart.tab_elt_car[eltCart_index];
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
        if SectRect(&eltCart.aire, &self.sapwind.ports.sapwcart.aire_carte).is_some() {
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
    fn aff_mont(&mut self, _nb: i16) {
        loop {
            let mut jmin = 0;
            let mut minyc = 32767;
            for (j, elt) in self.sapwind.ports.sapwcart.tab_elt_car.iter().enumerate() {
                if elt.aire.bottom < minyc {
                    jmin = j;
                    minyc = elt.aire.bottom;
                }
            }
            if minyc == 32767 {
                return;
            }
            self.Affmob3d(
                self.sapwind.ports.sapwcart.tab_elt_car[jmin].mob,
                self.sapwind.ports.sapwcart.tab_elt_car[jmin].aire,
                None,
            );
            self.sapwind.ports.sapwcart.tab_elt_car[jmin].aire.bottom = 32767;
        }
    }

    /***********************************************************************
                             EffCarte
    Efface le fond de la carte
    ***********************************************************************/
    fn EffCarte(&mut self) {
        let mut r = Rect::new(
            0,
            0,
            (TAILLE_X_CARTE * ECHELLE) as i16,
            (TAILLE_Y_CARTE * ECHELLE) as i16,
        );
        self.BoxFill(PortId::EcrCarte, &mut r, 0x050); /* palhaute[3]*/
    }

    /***********************************************************************
                             CARTE:
    ***********************************************************************/
    pub(crate) fn carte(&mut self) {
        let mut r = Rect::default();
        let _g = self.sapwind.ports.on(PortId::EcrCarte);
        self.sapglob.smenu[1] = 111;
        /* efface carte */
        self.EffCarte();
        /* trie les objets */
        let mut j = 0;
        let aigoual = std::mem::take(&mut self.sap3d.aigoual);
        for mont in &aigoual {
            if j >= NB_OBJETS_MAX {
                break;
            }
            let visible = mont.forme.is_some_and(|id| id.forme().num_mob >= 0);
            if visible && self.calc_cart(mont, j) {
                j += 1;
            }
        }
        self.sap3d.aigoual = aigoual;
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
        let z = (self.sapglob.zoom as i16 + delta).clamp(16, 160);
        self.sapglob.zoom = z as u8;
        self.carte();
    }

    /***********************************************************************
                             ZOOCAR:Gere le zoom
    ***********************************************************************/
    pub(crate) fn zoocar(&mut self) {
        let _g = self.sapwind.ports.on(PortId::MainWindow);

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
        self.sapwind.ports.ForeColor(whiteColor as i16);
        self.sapwind.ports.DoFrameRect(&r);
        self.sapwind.ports.ForeColor(blackColor as i16);
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
                    self.sapglob.zoom = self.sapglob.zoom.clamp(16, 160);
                    self.sapglob.dsposy = s;
                    self.carte();
                }
            }
            if self.sapglob.bouton != 0 {
                break;
            }
        }
        self.sapwind.ports.DoFrameRect(&r);
    }
}
