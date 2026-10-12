//! Les 148 prototypes de l'API "Acam" (declarations dans
//! `Sources/Headers/acam.h`, seul vestige de la glue C) sont fournis *ici* en
//! Rust : ils sont exposes en `#[no_mangle] extern "C"` et servent de
//! squelette au portage. Chaque erreur de lien du `cargo build` correspond a un
//! prototype d'`acam.h` a stuber (l'inventaire exact vient du linker).
//!
//! Aucun code C n'est compile : le port est complet, `Sources/Headers/acam.h`
//! n'est plus lu a la compilation.

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::sapiens::sapglob::SapId;
use crate::shared::{Ptr, RGBColor};
use std::os::raw::c_short;

pub(crate) mod muse;
pub(crate) mod sap3d;
pub(crate) mod sapcolorcvt;
pub(crate) mod sapcomb;
pub(crate) mod sapenreg;
pub(crate) mod sapgest;
pub(crate) mod sapglob;
pub(crate) mod saplist;
pub(crate) mod saprel;
pub(crate) mod sapscrollbar;
pub(crate) mod saptxt;
pub(crate) mod saptxt_es;
pub(crate) mod saptxt_fr;
pub(crate) mod saptxt_gb;
pub(crate) mod sapw3d;
pub(crate) mod sapwarb;
pub(crate) mod sapwbrt;
pub(crate) mod sapwcar;
pub(crate) mod sapwcart;
pub(crate) mod sapwclav;
pub(crate) mod sapwcvt;
pub(crate) mod sapwind;
pub(crate) mod sapwmess;
pub(crate) mod sapwmus;
pub(crate) mod sapwpref;
pub(crate) mod sapwsyl;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct liste {
    pub(crate) typeRef: TypeReference,
    pub(crate) taille: ::std::os::raw::c_uchar,
    pub(crate) maxtaille: ::std::os::raw::c_uchar,
    pub(crate) elements: [::std::os::raw::c_uchar; 30usize],
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
pub(crate) struct PalCoul {
    pub(crate) nbEntrees: ::std::os::raw::c_short,
    pub(crate) tabIndex: *mut ::std::os::raw::c_short,
    pub(crate) tabRgb: *mut RGBColor,
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
pub(crate) struct Mob {
    pub(crate) indexPalette: ::std::os::raw::c_short,
    pub(crate) nbcol: ::std::os::raw::c_short,
    pub(crate) nbligne: ::std::os::raw::c_short,
    pub(crate) vide: ::std::os::raw::c_short,
    pub(crate) datas: Ptr,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum TypeReference {
    REFER0,
    REFER1,
    REFER2,
    REFER3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TypeMuse {
    NO_MUSIQUE,
    MUSIQUE_HARMONY_QUALITE,
    MUSIQUE_HARMONY_VITESSE,
    MUSIQUE_MIDI,
    MUSIQUE_QUICKTIME,
    NB_TYPE_MUSIQUE,
}

impl From<c_short> for TypeMuse {
    fn from(value: c_short) -> Self {
        match value {
            0 => TypeMuse::NO_MUSIQUE,
            1 => TypeMuse::MUSIQUE_HARMONY_QUALITE,
            2 => TypeMuse::MUSIQUE_HARMONY_VITESSE,
            3 => TypeMuse::MUSIQUE_MIDI,
            4 => TypeMuse::MUSIQUE_QUICKTIME,
            5 => TypeMuse::NB_TYPE_MUSIQUE,
            _ => {
                eprint!("Invalid music id {value}");
                TypeMuse::NO_MUSIQUE
            }
        }
    }
}

pub(crate) const NOM_PREF: &[u8; 12] = b"Sapiens.cfg\0";

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct cellule {
    pub(crate) adresse: *mut ::std::os::raw::c_void,
    pub(crate) taille: ::std::os::raw::c_uchar,
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
#[derive(Debug, Clone)]
pub(crate) struct sap {
    pub(crate) colonne: ::std::os::raw::c_short,
    pub(crate) ligne: ::std::os::raw::c_uchar,
    pub(crate) sens: ::std::os::raw::c_uchar,
    pub(crate) vitesse: ::std::os::raw::c_uchar,
    pub(crate) objet: ::std::os::raw::c_uchar,
    pub(crate) hauteur: ::std::os::raw::c_uchar,
    pub(crate) action: ::std::os::raw::c_uchar,
    pub(crate) vie: ::std::os::raw::c_uchar,
    pub(crate) force: ::std::os::raw::c_uchar,
    pub(crate) agressivite: ::std::os::raw::c_uchar,
    pub(crate) souffle: ::std::os::raw::c_uchar,
    pub(crate) deltah: ::std::os::raw::c_uchar,
    pub(crate) blockimpact: ::std::os::raw::c_uchar,
    pub(crate) ligneimpact: ::std::os::raw::c_uchar,
    pub(crate) colimpact: ::std::os::raw::c_uchar,
    pub(crate) forcehache: ::std::os::raw::c_uchar,
    pub(crate) coeur: ::std::os::raw::c_short,
    pub(crate) cptrsouffle: ::std::os::raw::c_short,
    pub(crate) nbresag: ::std::os::raw::c_short,
    pub(crate) animal: ::std::os::raw::c_short,
    pub(crate) linsag: ::std::os::raw::c_short,
    pub(crate) colsag: ::std::os::raw::c_short,
    pub(crate) snssag: ::std::os::raw::c_short,
    pub(crate) fuite: ::std::os::raw::c_short,
    pub(crate) flg_mort: ::std::os::raw::c_short,
    pub(crate) poslcoeur: ::std::os::raw::c_short,
    pub(crate) posccoeur: ::std::os::raw::c_short,
    pub(crate) sapanim: [*mut ::std::os::raw::c_char; 15usize],
    pub(crate) cross: [::std::os::raw::c_uchar; 8usize],
    pub(crate) adversaire: Option<SapId>,
    pub(crate) possession: *mut liste,
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

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct brin {
    pub(crate) type_: ::std::os::raw::c_ushort,
    pub(crate) lin: ::std::os::raw::c_ushort,
    pub(crate) col: ::std::os::raw::c_ushort,
    pub(crate) sens: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct h_chron {
    pub(crate) h: ::std::os::raw::c_short,
    pub(crate) q: ::std::os::raw::c_short,
    pub(crate) pal: [::std::os::raw::c_short; 32usize],
    pub(crate) TabCouleursCiel: *mut IndexColor,
    pub(crate) TabPaletteDistance: *mut *mut ::std::os::raw::c_short,
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
pub(crate) struct evt {
    pub(crate) type_: ::std::os::raw::c_uchar,
    pub(crate) posx: ::std::os::raw::c_uchar,
    pub(crate) posy: ::std::os::raw::c_uchar,
    pub(crate) distmax: ::std::os::raw::c_uchar,
    pub(crate) rnd: ::std::os::raw::c_uchar,
    pub(crate) evt: ::std::os::raw::c_uchar,
    pub(crate) message: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct gemcadre {
    pub(crate) ligne: ::std::os::raw::c_short,
    pub(crate) colon: ::std::os::raw::c_short,
    pub(crate) haut: ::std::os::raw::c_short,
    pub(crate) larg: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct IndexColor {
    pub(crate) index: ::std::os::raw::c_short,
    pub(crate) rgb: RGBColor,
}
pub(crate) type IndexColorPtr = *mut IndexColor;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Vue {
    AUCUNE = 0,
    PAYSAGE = 1,
    LATERALE = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum FormeVue {
    BUSTE = 3,
    TETE = 4,
    ARME = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct reference {
    pub(crate) nomref: [::std::os::raw::c_char; 64usize],
    pub(crate) param1: ::std::os::raw::c_uchar,
    pub(crate) param2: ::std::os::raw::c_uchar,
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
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 49
    };
}

#[macro_export]
macro_rules! MOB_IMPACT {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 32
    };
}

#[macro_export]
macro_rules! MOB_SANG_A_TERRE {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 87
    };
}

#[macro_export]
macro_rules! MOB_EMPREINTE {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 20
    };
}

#[macro_export]
macro_rules! MOB_CORPS {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 6
    };
}

#[macro_export]
macro_rules! MOB_CHEMIN {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 83
    };
}

#[macro_export]
macro_rules! MOB_ANIMAL_MORT {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 109
    };
}

#[macro_export]
macro_rules! MOB_PICT {
    ($sapwind:expr) => {
        $sapwind.mobSilex + 1
    };
}

#[macro_export]
macro_rules! MOB_GUEPARD {
    ($sapwind:expr) => {
        $sapwind.mobSilex + 12
    };
}

#[macro_export]
macro_rules! MOB_SILEX_SAGAIE {
    ($sapwind:expr) => {
        $sapwind.mobSilex + 13
    };
}

#[macro_export]
macro_rules! MOB_SILEX_HACHE {
    ($sapwind:expr) => {
        $sapwind.mobSilex + 14
    };
}

#[macro_export]
macro_rules! MOB_SILEX_CASSE {
    () => {
        $sapwind.mobSilex + 15
    };
}

#[macro_export]
macro_rules! MOB_MORT {
    ($sapwind:expr) => {
        $sapwind.mobSapmob + 114
    };
}

#[macro_export]
macro_rules! MOB_SOLEIL {
    ($sapwind:expr) => {
        $sapwind.mobFace + 28
    };
}

#[macro_export]
macro_rules! MOB_FEUILLE {
    ($sapwind:expr) => {
        $sapwind.mobFeuille + 1
    };
}

#[macro_export]
macro_rules! MOB_HERBE {
    ($sapwind:expr) => {
        $sapwind.mobAvplan + 1
    };
}

#[macro_export]
macro_rules! MOB_CADRE {
    ($sapwind:expr) => {
        $sapwind.mobBambou + 1
    };
}

#[macro_export]
macro_rules! UP {
    ($sapwind:expr) => {
        $sapwind.mobBambou + 18
    };
}

#[macro_export]
macro_rules! DOWN {
    ($sapwind:expr) => {
        $sapwind.mobBambou + 19
    };
}

#[macro_export]
macro_rules! INCONNU {
    ($sapwind:expr) => {
        $sapwind.mobBambou + 20
    };
}

#[macro_export]
macro_rules! SELECT {
    ($sapwind:expr) => {
        $sapwind.mobBambou + 21
    };
}

#[macro_export]
macro_rules! DEBFLECH {
    ($sapwind:expr) => {
        $sapwind.mobFleches + 1
    };
}

#[macro_export]
macro_rules! DEBCRANE {
    ($sapwind:expr) => {
        $sapwind.mobFace + 13
    };
}

#[macro_export]
macro_rules! DEBTETE {
    ($sapwind:expr) => {
        $sapwind.mobFace + 1
    };
}

#[macro_export]
macro_rules! DEBBEST {
    ($sapwind:expr) => {
        $sapwind.mobFace + 24
    };
}

#[macro_export]
macro_rules! MOB_DESIGNE {
    ($sapwind:expr) => {
        $sapwind.mobFleches + 17
    };
}

#[macro_export]
macro_rules! MOB_AIDE {
    ($sapwind:expr) => {
        $sapwind.mobBambou + 27
    };
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Forme {
    pub(crate) nbrdisk: ::std::os::raw::c_uchar,
    pub(crate) relief: ::std::os::raw::c_uchar,
    pub(crate) hauteur: ::std::os::raw::c_ushort,
    pub(crate) rayon: ::std::os::raw::c_ushort,
    pub(crate) dskrayon: [::std::os::raw::c_ushort; 16usize],
    pub(crate) dskdelta: [::std::os::raw::c_char; 32usize],
    pub(crate) num_mob: ::std::os::raw::c_short,
    pub(crate) numMob3d: ::std::os::raw::c_short,
}
/// Identifie une des formes statiques du décor 3d (voir `sap3d.rs`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum FormeId {
    Forme1,
    Forme2,
    Forme4,
    Forme6,
    Forme7,
    Forme8,
    Nuage1,
    Nuage2,
    Nuage3,
    Eclair,
    ToucheEclair,
    Foudroye,
    Peupli,
    Chene,
    Noiset,
    Fontaine,
    Totem,
    Hutte,
    Oiseau,
    Genevr,
    Hetre,
    Roc1,
    Roc2,
    Roc3,
    Fumee,
    Foyer,
    Building,
    Rocher,
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct Mont {
    /// Lien vers l'objet suivant de la chaîne de rendu (tri par distance).
    /// Index dans le bloc contigu `Sap3d::aigoual`, `None` = fin de chaîne.
    pub(crate) chaine: Option<MontId>,
    pub(crate) distance: ::std::os::raw::c_ushort,
    pub(crate) posx: ::std::os::raw::c_ushort,
    pub(crate) posy: ::std::os::raw::c_ushort,
    pub(crate) ign: ::std::os::raw::c_ushort,
    pub(crate) forme: Option<FormeId>,
}
pub(crate) type MontId = u32;

pub(crate) const FREQ_ORAGE: u32 = 9990;
pub(crate) const DIST_MIN_FOUDROIE: u32 = 250;
pub(crate) const NB_NUAGES: u32 = 100;
pub(crate) const NB_OISEAUX: u32 = 20;
pub(crate) const NB_ROCHERS: u32 = 300;
pub(crate) const NB_HOMMES: u32 = 0;
pub(crate) const NB_ARBRES: u32 = 800;

pub(crate) const PRECISION_SILEX: u32 = 300;
pub(crate) const NB_ETOILES: u32 = 10000;
pub(crate) const NB_NUAGES_MAX: u32 = 2000;
pub(crate) const NB_OISEAUX_MAX: u32 = 400;
pub(crate) const NB_ROCHERS_MAX: u32 = 50000;
pub(crate) const NB_ARBRES_MAX: u32 = 100000;
pub(crate) const NB_ARBRES_LATERAL: u32 = 3;
pub(crate) const NB_HERBE_AV_PLAN: u32 = 20;
pub(crate) const NB_HERBE_ARR_PLAN: u32 = 15;
pub(crate) const NB_ARBRES_LATERAL_MAX: u32 = 20;
pub(crate) const NB_MAX_HERBE_AV_PLAN: u32 = 100;
pub(crate) const NB_MAX_HERBE_AR_PLAN: u32 = 100;
pub(crate) const ECHELLE: u32 = 2;
pub(crate) const TAILLE_STR_MESS_FORMATE: u32 = 512;
pub(crate) const NB_MAX_MOB: u32 = 640;
pub(crate) const POLICE_LISSAGE: u32 = 2;
pub(crate) const STYLE_LISSAGE: u32 = 1;
pub(crate) const STR_VRAC: u32 = 0;
pub(crate) const SAGAIE: u32 = 23;
pub(crate) const NB_MOTS_BKMOTS: u32 = 46;
pub(crate) const NBREVTHEURE: u32 = 37;
pub(crate) const NB_LIGNES_CIEL: u32 = 128;
pub(crate) const NB_LIGNES_TERRE: u32 = 72;
pub(crate) const NB_DEGRADE_DISTANCE: u32 = 72;
pub(crate) const NBREVTDESTINE: u32 = 40;
pub(crate) const gemnbrecad: u32 = 4;
pub(crate) const geminterv: u32 = 4;
pub(crate) const BASEMOB: c_short = 0;
