#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::Sapiens;
use crate::acam::{ForeColor, InsetRect, NewPtr, SectRect, SetRect};
use crate::cbridge::{blackColor, whiteColor};
use crate::port;
use crate::sapiens::sapwbrt::{CopyOffscreen, DoFrameRect, SapiensCreeOffscreen};
use crate::sapiens::sapwclav::GetKeyMap;
use crate::sapiens::{FormeId, Mont};
use crate::shared::{CGrafPtr, Rect};
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

/* Etat mutable de SAPWCART.C, ex-`static mut`, regroupe dans cette
structure, champ de `Sapiens`. */
pub(crate) struct Sapwcart {
    /* ex-`EcrCarte` */
    ecr_carte: CGrafPtr,
    /* ex-`TabEltCar` */
    tab_elt_car: *mut EltCart,
    /* ex-`AireCarte` */
    aire_carte: Rect,
}

impl Default for Sapwcart {
    fn default() -> Self {
        Self {
            ecr_carte: null_mut(),
            tab_elt_car: null_mut(),
            aire_carte: Rect::default(),
        }
    }
}

impl Sapiens {
    /*;-_-_-_-_-_-_-_-_  Routine 2: transfere carte  sur ecran -_-_-_-_-_-_-_-
     */
    unsafe fn tfrcar(&mut self) {
        let mut rdest = Rect::default();
        SetRect(
            &mut rdest,
            (160 * ECHELLE) as c_short,
            (117 * ECHELLE) as c_short,
            ((160 + TAILLE_X_CARTE) * ECHELLE) as c_short,
            ((117 + TAILLE_Y_CARTE) * ECHELLE) as c_short,
        );
        CopyOffscreen(
            self.sapwcart.ecr_carte,
            self.sapwind.debecr,
            addr_of_mut!(self.sapwcart.aire_carte),
            &mut rdest,
        );
        CopyOffscreen(
            self.sapwcart.ecr_carte,
            self.sapwind.mainWindow,
            addr_of_mut!(self.sapwcart.aire_carte),
            &mut rdest,
        );
    }

    /*
    ;-_-_-_-_-_-_-_-_  Routine 1: calcule positions sur ecran -_-_-_-_-_-_-_-
    ; => Pointeur sur la structure montagne ,Zoom_
    ; <= Xc,Yc (Yc à 0 si montagne invisible)
    */
    unsafe fn calc_cart(&self, mont: &Mont, eltCart: *mut EltCart) -> u8 {
        let Some(forme) = mont.forme.map(FormeId::forme) else {
            return 0;
        };
        /* Position en ligne */
        let x = ((self.sapglob.yj as i64 - mont.posy as i64) / (self.sapglob.zoom as i64))
            + TAILLE_X_CARTE;
        let y = ((self.sapglob.xj as i64 - mont.posx as i64) / (self.sapglob.zoom as i64))
            + TAILLE_Y_CARTE;
        let r = forme.rayon as i64 / (self.sapglob.zoom as i64 / 2);
        if forme.num_mob == 8 {
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
        if SectRect(
            &(*eltCart).aire,
            addr_of!(self.sapwcart.aire_carte),
            &mut rb,
        ) {
            (*eltCart).mob = self.sapwind.mob3D + forme.numMob3d;
            return 1; /* VRAI */
        }
        0 /* FAUX */
    }
}

/*************************************************************************
                Affichage des montagnes sur la carte
=> Nombre d'objets a afficher
***************************************************************************/
impl Sapiens {
    unsafe fn aff_mont(&self, nb: c_short) {
        loop {
            let mut minyc;
            let mut jmin: c_short = 0;
            minyc = 32767;
            for j in 0..nb {
                if (*self.sapwcart.tab_elt_car.add(j as usize)).aire.bottom < minyc {
                    jmin = j;
                    minyc = (*self.sapwcart.tab_elt_car.add(j as usize)).aire.bottom;
                }
            }
            if minyc == 32767 {
                return;
            }
            self.Affmob3d(
                (*self.sapwcart.tab_elt_car.add(jmin as usize)).mob,
                &mut (*self.sapwcart.tab_elt_car.add(jmin as usize)).aire,
                null_mut(),
            );
            (*self.sapwcart.tab_elt_car.add(jmin as usize)).aire.bottom = 32767;
        }
    }
}

impl Sapiens {
    /***********************************************************************
                             EffCarte
    Efface le fond de la carte
    ***********************************************************************/
    unsafe fn EffCarte(&self) {
        let mut r = Rect::default();
        SetRect(
            &mut r,
            0,
            0,
            (TAILLE_X_CARTE * ECHELLE) as c_short,
            (TAILLE_Y_CARTE * ECHELLE) as c_short,
        );
        self.BoxFill(self.sapwcart.ecr_carte, &mut r, 0x050); /* palhaute[3]*/
    }

    /***********************************************************************
                             CARTE:
    ***********************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn carte(&mut self) {
        let mut j: c_long;
        let angle: c_short;
        let mut r = Rect::default();

        /* deconnecte sous menu */
        if self.sapwcart.ecr_carte.is_null() {
            self.sapwcart.ecr_carte = SapiensCreeOffscreen(
                (TAILLE_X_CARTE * ECHELLE) as c_short,
                (TAILLE_Y_CARTE * ECHELLE) as c_short,
            );
            if self.sapwcart.ecr_carte.is_null() {
                return;
            }
            self.sapwcart.tab_elt_car =
                NewPtr(NB_OBJETS_MAX * size_of::<EltCart>() as c_long) as *mut EltCart;
            if self.sapwcart.tab_elt_car.is_null() {
                return;
            }
            SetRect(
                addr_of_mut!(self.sapwcart.aire_carte),
                0,
                0,
                (TAILLE_X_CARTE * ECHELLE) as c_short,
                (TAILLE_Y_CARTE * ECHELLE) as c_short,
            );
        }
        let _g = port::on(self.sapwcart.ecr_carte);
        (*addr_of_mut!(self.sapglob.smenu))[1] = 111;
        /* efface carte */
        self.EffCarte();
        /* trie les objets */
        j = 0;
        for mont in &self.sap3d.aigoual {
            if j >= NB_OBJETS_MAX {
                break;
            }
            if mont.forme.is_some_and(|id| id.forme().num_mob >= 0) {
                if self.calc_cart(mont, &mut *self.sapwcart.tab_elt_car.add(j as usize)) != 0 {
                    j += 1;
                }
            }
        }
        /* affichage */
        self.aff_mont(j as c_short);
        /* on affiche la fleche */
        angle = ((self.sapglob.dzeta + 32) / 64) & 15;
        self.Affmobbrtomb(
            self.sapwind.mobFleches + 1 + angle,
            (TAILLE_X_CARTE / 2 - 8) as c_short,
            (TAILLE_Y_CARTE / 2 - 8) as c_short,
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
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn ChangeZoomCarte(&mut self, delta: c_short) {
        let mut z = self.sapglob.zoom as c_short + delta;
        if z > 160 {
            z = 160;
        }
        if z < 16 {
            z = 16;
        }
        self.sapglob.zoom = z as c_uchar;
        self.carte();
    }

    /***********************************************************************
                             ZOOCAR:Gere le zoom
    ***********************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn zoocar(&mut self) {
        let mut bitmap: c_short = 0;
        let mut r = Rect::default();

        let _g = port::on(self.sapwind.mainWindow);

        loop {
            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
            if self.sapglob.bouton == 0 {
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
        self.carte();
        loop {
            self.GereInterface();
            if GetKeyMap(&mut bitmap) != 0 {
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
                let s = *addr_of!(self.sapglob.sposy);
                if s != *addr_of!(self.sapglob.dsposy) {
                    self.sapglob.zoom = (s / 2 + 30) as c_uchar;
                    if self.sapglob.zoom < 16 {
                        self.sapglob.zoom = 16;
                    }
                    if self.sapglob.zoom > 160 {
                        self.sapglob.zoom = 160;
                    }
                    *addr_of_mut!(self.sapglob.dsposy) = s;
                    self.carte();
                }
            }
            if self.sapglob.bouton != 0 {
                break;
            }
        }
        DoFrameRect(&mut r);
    }
}
