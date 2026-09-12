//! Les 148 prototypes de l'API "Acam" (declarations dans
//! `Sources/Headers/acam.h`, seul vestige de la glue C) sont fournis *ici* en
//! Rust : ils sont exposes en `#[no_mangle] extern "C"` et servent de
//! squelette au portage. Chaque erreur de lien du `cargo build` correspond a un
//! prototype d'`acam.h` a stuber (l'inventaire exact vient du linker).
//!
//! Aucun code C n'est compile : le port est complet, `Sources/Headers/acam.h`
//! n'est plus lu a la compilation.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::shared::{Ptr, RGBColor};
use std::os::raw::c_short;

pub mod muse;
pub mod sap3d;
pub mod sapbonc;
pub mod sapcolorcvt;
pub mod sapcomb;
pub mod sapenreg;
pub mod sapgest;
pub mod sapglob;
pub mod sapglue2017;
pub mod saplist;
pub mod saprel;
pub mod sapscrollbar;
pub mod saptxt;
pub mod saptxt_es;
pub mod saptxt_fr;
pub mod saptxt_gb;
pub mod sapw3d;
pub mod sapwarb;
pub mod sapwbrt;
pub mod sapwcar;
pub mod sapwcart;
pub mod sapwclav;
pub mod sapwcvt;
pub mod sapwind;
pub mod sapwmess;
pub mod sapwmus;
pub mod sapwpref;
pub mod sapwsyl;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct liste {
    pub typeRef: TypeReference,
    pub taille: ::std::os::raw::c_uchar,
    pub maxtaille: ::std::os::raw::c_uchar,
    pub elements: [::std::os::raw::c_uchar; 30usize],
}
impl Default for liste {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PalCoul {
    pub nbEntrees: ::std::os::raw::c_short,
    pub tabIndex: *mut ::std::os::raw::c_short,
    pub tabRgb: *mut RGBColor,
}
impl Default for PalCoul {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mob {
    pub indexPalette: ::std::os::raw::c_short,
    pub nbcol: ::std::os::raw::c_short,
    pub nbligne: ::std::os::raw::c_short,
    pub vide: ::std::os::raw::c_short,
    pub datas: Ptr,
}
impl Default for Mob {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

pub type TypeReference = ::std::os::raw::c_uint;
pub const TypeReference_REFER0: TypeReference = 0;
pub const TypeReference_REFER1: TypeReference = 1;
pub const TypeReference_REFER2: TypeReference = 2;
pub const TypeReference_REFER3: TypeReference = 3;
pub const TypeMuse_NO_MUSIQUE: TypeMuse = 0;
pub const TypeMuse_MUSIQUE_HARMONY_QUALITE: TypeMuse = 1;
pub const TypeMuse_MUSIQUE_HARMONY_VITESSE: TypeMuse = 2;
pub const TypeMuse_MUSIQUE_MIDI: TypeMuse = 3;
pub const TypeMuse_MUSIQUE_QUICKTIME: TypeMuse = 4;
pub const TypeMuse_NB_TYPE_MUSIQUE: TypeMuse = 5;
pub type TypeMuse = ::std::os::raw::c_uint;

pub const NOM_PREF: &[u8; 12] = b"Sapiens.cfg\0";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cellule {
    pub adresse: *mut ::std::os::raw::c_void,
    pub taille: ::std::os::raw::c_uchar,
}
impl Default for cellule {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sap {
    pub colonne: ::std::os::raw::c_short,
    pub ligne: ::std::os::raw::c_uchar,
    pub sens: ::std::os::raw::c_uchar,
    pub vitesse: ::std::os::raw::c_uchar,
    pub objet: ::std::os::raw::c_uchar,
    pub hauteur: ::std::os::raw::c_uchar,
    pub action: ::std::os::raw::c_uchar,
    pub vie: ::std::os::raw::c_uchar,
    pub force: ::std::os::raw::c_uchar,
    pub agressivite: ::std::os::raw::c_uchar,
    pub souffle: ::std::os::raw::c_uchar,
    pub deltah: ::std::os::raw::c_uchar,
    pub blockimpact: ::std::os::raw::c_uchar,
    pub ligneimpact: ::std::os::raw::c_uchar,
    pub colimpact: ::std::os::raw::c_uchar,
    pub forcehache: ::std::os::raw::c_uchar,
    pub coeur: ::std::os::raw::c_short,
    pub cptrsouffle: ::std::os::raw::c_short,
    pub nbresag: ::std::os::raw::c_short,
    pub animal: ::std::os::raw::c_short,
    pub linsag: ::std::os::raw::c_short,
    pub colsag: ::std::os::raw::c_short,
    pub snssag: ::std::os::raw::c_short,
    pub fuite: ::std::os::raw::c_short,
    pub flg_mort: ::std::os::raw::c_short,
    pub poslcoeur: ::std::os::raw::c_short,
    pub posccoeur: ::std::os::raw::c_short,
    pub sapanim: [*mut ::std::os::raw::c_char; 15usize],
    pub cross: [::std::os::raw::c_uchar; 8usize],
    pub adversaire: *mut sap,
    pub possession: *mut liste,
}
impl Default for sap {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

pub const DLOG_COORDONNEES: u32 = 1000;
pub const DLOG_NUM_SERIE: u32 = 1001;
pub const DIALOG_PREF: u32 = 1002;
pub const ALRT_OK_ENREG: u32 = 200;
pub const ALRT_NO_FONCTION: u32 = 201;
pub const ALRT_ENREG: u32 = 202;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct brin {
    pub type_: ::std::os::raw::c_ushort,
    pub lin: ::std::os::raw::c_ushort,
    pub col: ::std::os::raw::c_ushort,
    pub sens: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct h_chron {
    pub h: ::std::os::raw::c_short,
    pub q: ::std::os::raw::c_short,
    pub pal: [::std::os::raw::c_short; 32usize],
    pub TabCouleursCiel: *mut IndexColor,
    pub TabPaletteDistance: *mut *mut ::std::os::raw::c_short,
}
impl Default for h_chron {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct evt {
    pub type_: ::std::os::raw::c_uchar,
    pub posx: ::std::os::raw::c_uchar,
    pub posy: ::std::os::raw::c_uchar,
    pub distmax: ::std::os::raw::c_uchar,
    pub rnd: ::std::os::raw::c_uchar,
    pub evt: ::std::os::raw::c_uchar,
    pub message: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct gemcadre {
    pub ligne: ::std::os::raw::c_short,
    pub colon: ::std::os::raw::c_short,
    pub haut: ::std::os::raw::c_short,
    pub larg: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IndexColor {
    pub index: ::std::os::raw::c_short,
    pub rgb: RGBColor,
}
pub type IndexColorPtr = *mut IndexColor;
pub type IndexColorHandle = *mut *mut IndexColor;
pub const VUE_AUCUNE: _bindgen_ty_3 = 0;
pub const VUE_PAYSAGE: _bindgen_ty_3 = 1;
pub const VUE_LATERALE: _bindgen_ty_3 = 2;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const FORME_BUSTE: _bindgen_ty_4 = 3;
pub const FORME_TETE: _bindgen_ty_4 = 4;
pub const FORME_ARME: _bindgen_ty_4 = 5;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct reference {
    pub nomref: [::std::os::raw::c_char; 64usize],
    pub param1: ::std::os::raw::c_uchar,
    pub param2: ::std::os::raw::c_uchar,
}
impl Default for reference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

#[macro_export]
macro_rules! MOB_FLECHE {
    () => {
        crate::sapiens::sapwind::mobSapmob + 49
    };
}

#[macro_export]
macro_rules! MOB_IMPACT {
    () => {
        crate::sapiens::sapwind::mobSapmob + 32
    };
}

#[macro_export]
macro_rules! MOB_SANG_A_TERRE {
    () => {
        crate::sapiens::sapwind::mobSapmob + 87
    };
}

#[macro_export]
macro_rules! MOB_EMPREINTE {
    () => {
        crate::sapiens::sapwind::mobSapmob + 20
    };
}

#[macro_export]
macro_rules! MOB_CORPS {
    () => {
        crate::sapiens::sapwind::mobSapmob + 6
    };
}

#[macro_export]
macro_rules! MOB_CHEMIN {
    () => {
        crate::sapiens::sapwind::mobSapmob + 83
    };
}

#[macro_export]
macro_rules! MOB_ANIMAL_MORT {
    () => {
        crate::sapiens::sapwind::mobSapmob + 109
    };
}

#[macro_export]
macro_rules! MOB_PICT {
    () => {
        crate::sapiens::sapwind::mobSilex + 1
    };
}

#[macro_export]
macro_rules! MOB_GUEPARD {
    () => {
        crate::sapiens::sapwind::mobSilex + 12
    };
}

#[macro_export]
macro_rules! MOB_SILEX_SAGAIE {
    () => {
        crate::sapiens::sapwind::mobSilex + 13
    };
}

#[macro_export]
macro_rules! MOB_SILEX_HACHE {
    () => {
        crate::sapiens::sapwind::mobSilex + 14
    };
}

#[macro_export]
macro_rules! MOB_SILEX_CASSE {
    () => {
        crate::sapiens::sapwind::mobSilex + 15
    };
}

#[macro_export]
macro_rules! MOB_MORT {
    () => {
        crate::sapiens::sapwind::mobSapmob + 114
    };
}

#[macro_export]
macro_rules! MOB_SOLEIL {
    () => {
        crate::sapiens::sapwind::mobFace + 28
    };
}

#[macro_export]
macro_rules! MOB_FEUILLE {
    () => {
        crate::sapiens::sapwind::mobFeuille + 1
    };
}

#[macro_export]
macro_rules! MOB_HERBE {
    () => {
        crate::sapiens::sapwind::mobAvplan + 1
    };
}

#[macro_export]
macro_rules! MOB_CADRE {
    () => {
        crate::sapiens::sapwind::mobBambou + 1
    };
}

#[macro_export]
macro_rules! UP {
    () => {
        crate::sapiens::sapwind::mobBambou + 18
    };
}

#[macro_export]
macro_rules! DOWN {
    () => {
        crate::sapiens::sapwind::mobBambou + 19
    };
}

#[macro_export]
macro_rules! INCONNU {
    () => {
        crate::sapiens::sapwind::mobBambou + 20
    };
}

#[macro_export]
macro_rules! SELECT {
    () => {
        crate::sapiens::sapwind::mobBambou + 21
    };
}

#[macro_export]
macro_rules! DEBFLECH {
    () => {
        crate::sapiens::sapwind::mobFleches + 1
    };
}

#[macro_export]
macro_rules! DEBCRANE {
    () => {
        crate::sapiens::sapwind::mobFace + 13
    };
}

#[macro_export]
macro_rules! DEBTETE {
    () => {
        crate::sapiens::sapwind::mobFace + 1
    };
}

#[macro_export]
macro_rules! DEBBEST {
    () => {
        crate::sapiens::sapwind::mobFace + 24
    };
}

#[macro_export]
macro_rules! MOB_DESIGNE {
    () => {
        crate::sapiens::sapwind::mobFleches + 17
    };
}

#[macro_export]
macro_rules! MOB_AIDE {
    () => {
        crate::sapiens::sapwind::mobBambou + 27
    };
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Forme {
    pub nbrdisk: ::std::os::raw::c_uchar,
    pub relief: ::std::os::raw::c_uchar,
    pub hauteur: ::std::os::raw::c_ushort,
    pub rayon: ::std::os::raw::c_ushort,
    pub dskrayon: [::std::os::raw::c_ushort; 16usize],
    pub dskdelta: [::std::os::raw::c_char; 32usize],
    pub num_mob: ::std::os::raw::c_short,
    pub numMob3d: ::std::os::raw::c_short,
}
pub type FormePtr = *mut Forme;
pub type FormeHandle = *mut *mut Forme;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mont {
    pub chaine: *mut Mont,
    pub distance: ::std::os::raw::c_ushort,
    pub posx: ::std::os::raw::c_ushort,
    pub posy: ::std::os::raw::c_ushort,
    pub ign: ::std::os::raw::c_ushort,
    pub ptrforme: FormePtr,
}

impl Default for Mont {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type MontPtr = *mut Mont;
pub type MontHandle = *mut *mut Mont;

pub const FREQ_ORAGE: u32 = 9990;
pub const DIST_MIN_FOUDROIE: u32 = 250;
pub const NB_NUAGES: u32 = 100;
pub const NB_OISEAUX: u32 = 20;
pub const NB_ROCHERS: u32 = 300;
pub const NB_HOMMES: u32 = 0;
pub const NB_ARBRES: u32 = 800;

pub const PRECISION_SILEX: u32 = 300;
pub const NB_ETOILES: u32 = 10000;
pub const NB_NUAGES_MAX: u32 = 2000;
pub const NB_OISEAUX_MAX: u32 = 400;
pub const NB_ROCHERS_MAX: u32 = 50000;
pub const NB_ARBRES_MAX: u32 = 100000;
pub const NB_ARBRES_LATERAL: u32 = 3;
pub const NB_HERBE_AV_PLAN: u32 = 20;
pub const NB_HERBE_ARR_PLAN: u32 = 15;
pub const NB_ARBRES_LATERAL_MAX: u32 = 20;
pub const NB_MAX_HERBE_AV_PLAN: u32 = 100;
pub const NB_MAX_HERBE_AR_PLAN: u32 = 100;
pub const ECHELLE: u32 = 2;
pub const TAILLE_STR_MESS_FORMATE: u32 = 512;
pub const NB_MAX_MOB: u32 = 640;
pub const POLICE_LISSAGE: u32 = 2;
pub const STYLE_LISSAGE: u32 = 1;
pub const DELTA_LANGUE: u32 = 1000;
pub const STR_VRAC: u32 = 0;
pub const ALRT_VERSION: u32 = 133;
pub const SIZELISTE: u32 = 8;
pub const SAGAIE: u32 = 23;
pub const dstsag: u32 = 10;
pub const dstfra: u32 = 4;
pub const TAILLE_MAX_NOM_USER: u32 = 80;
pub const TAILLE_MAX_NOM: u32 = 32;
pub const NB_MOTS_BKMOTS: u32 = 46;
pub const NBREVTHEURE: u32 = 37;
pub const NB_LIGNES_CIEL: u32 = 128;
pub const NB_LIGNES_TERRE: u32 = 72;
pub const NB_DEGRADE_DISTANCE: u32 = 72;
pub const NBREVTDESTINE: u32 = 40;
pub const gemnbrecad: u32 = 4;
pub const geminterv: u32 = 4;
pub const BASEMOB: c_short = 0;
pub const iPrAbort: u32 = 128;
