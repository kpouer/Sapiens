//! Les prototypes de l'API "Acam" historiques (référence :
//! `Sources/Headers/acam.h`) sont réimplémentés en Rust pur.
//! Aucun code C n'est compilé et aucune frontière `extern "C"` n'est
//! nécessaire pour le jeu : les fonctions s'appellent comme du Rust ordinaire.

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::model::RGBColor;
use crate::sapiens::sapglob::{AnimPtr, ListeId, SapId};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) mod c_str;
pub(crate) mod muse;
pub(crate) mod sap3d;
pub(crate) mod sapcolorcvt;
pub(crate) mod sapcomb;
pub(crate) mod sapenreg;
pub(crate) mod sapgest;
pub(crate) mod sapglob;
pub(crate) mod saplist;
pub(crate) mod saprel;
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

use crate::acam::Random;
use crate::pict::Pict;
use crate::sapiens::sap3d::Sap3d;
use crate::sapiens::sapcomb::Sapcomb;
use crate::sapiens::sapenreg::Sapenreg;
use crate::sapiens::sapgest::Sapgest;
use crate::sapiens::sapglob::Sapglob;
use crate::sapiens::saplist::Saplist;
use crate::sapiens::saprel::Saprel;
use crate::sapiens::saptxt::{SapLang, Saptxt};
use crate::sapiens::sapw3d::Sapw3d;
use crate::sapiens::sapwarb::Sapwarb;
use crate::sapiens::sapwbrt::Sapwbrt;
use crate::sapiens::sapwcar::Sapwcar;
use crate::sapiens::sapwclav::{Sapwclav, Souris};
use crate::sapiens::sapwcvt::Sapwcvt;
use crate::sapiens::sapwind::SapWind;
use crate::sapiens::sapwmess::Sapwmess;
use crate::sapiens::sapwmus::Sapwmus;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

/// État complet du jeu : l'équivalent des variables globales des fichiers C,
/// regroupées par fichier d'origine.
#[derive(Default)]
pub(crate) struct Sapiens {
    souris: Souris,
    sapwclav: Sapwclav,
    sapwind: SapWind,
    sapcomb: Sapcomb,
    sapglob: Sapglob,
    sap3d: Sap3d,
    sapw3d: Sapw3d,
    sapwbrt: Sapwbrt,
    random: Random,
    saplist: Saplist,
    sapwmus: Sapwmus,
    sapwarb: Sapwarb,
    sapwcar: Sapwcar,
    saprel: Saprel,
    sapwmess: Sapwmess,
    sapgest: Sapgest,
    sapenreg: Sapenreg,
    sapwcvt: Sapwcvt,
    saptxt: Saptxt,
    sap_lang: SapLang,
    pict: Pict,
}

/// Point d'entrée du thread de jeu : construit l'état avec le zoom initial
/// choisi et lance la boucle infinie du jeu original (`sapiens_main`).
pub(crate) fn run(sapiens_timer: Arc<AtomicU64>, initial_zoom: f32) {
    let mut sapwind = SapWind::default();
    sapwind.set_zoom(initial_zoom);
    // Pointeurs auto-référents de SAPGLOB.C (debban, peuple...).
    let mut sapiens = Sapiens {
        sapwclav: Sapwclav::with_sapiens_timer(sapiens_timer),
        sapwind,
        ..Default::default()
    };
    sapiens.init_glob();
    sapiens.sapiens_main();
}

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct liste {
    pub(crate) typeRef: TypeReference,
    pub(crate) taille: u8,
    pub(crate) maxtaille: u8,
    pub(crate) elements: [u8; 30],
}

#[derive(Default, Debug, Clone)]
pub(crate) struct PalCoul {
    pub(crate) nbEntrees: i16,
    pub(crate) tabIndex: Vec<i16>,
    pub(crate) tabRgb: Vec<RGBColor>,
}

const DEFAULT_PALCOUL: PalCoul = PalCoul {
    nbEntrees: 0,
    tabIndex: Vec::new(),
    tabRgb: Vec::new(),
};

#[derive(Default, Debug, Clone)]
pub(crate) struct Mob {
    pub(crate) indexPalette: i16,
    pub(crate) nbcol: i16,
    pub(crate) nbligne: i16,
    pub(crate) vide: i16,
    pub(crate) datas: Vec<u8>,
}

const DEFAULT_MOB: Mob = Mob {
    indexPalette: 0,
    nbcol: 0,
    nbligne: 0,
    vide: 0,
    datas: Vec::new(),
};

impl Mob {
    /// Données du mob (`nbcol * nbligne` index de couleur), vide si non chargé.
    pub(crate) fn data(&self) -> &[u8] {
        &self.datas
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub(crate) enum TypeReference {
    #[default]
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

impl From<i16> for TypeMuse {
    fn from(value: i16) -> Self {
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

#[derive(Debug, Copy, Clone)]
pub(crate) enum CelluleId {
    Soif,
    Fatig,
    Faim,
    Mort,
    Bless,
    Zoom,
    Agilite,
    Force,
    Resistance,
    Charisme,
    Vie,
    Sexe,
    Viecopain,
    Scenar,
    Scenpr,
    Ciel,
    Xj,
    Yj,
    Dzeta,
    Heure,
    Jour_act,
    Param,
    Tabclav1,
    Tabclav2,
    Bidon,
    Terre,
    Tagress(usize),
    Enetoi(usize),
    Present(usize),
    Sap(SapId, SapCle),
    Liste(ListeId, ListeCle),
    SapTxtNomjoueur,
    SapTxtNomcopain,
    Sap3dFreqOrage,
    Sap3dDistMinFoudroie,
    Sap3dNbNuages,
    Sap3dNbOiseaux,
    Sap3dNbRochers,
    Sap3dNbHommes,
    Sap3dNbArbres,
    SapcombNbHerbes,
    Sap3dVitesseDeplacementNuages,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum SapCle {
    Cross,
    Vie,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ListeCle {
    Taille,
    Elements,
    Element(usize),
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct cellule {
    pub(crate) adresse: CelluleId,
    pub(crate) taille: u8,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct sap {
    pub(crate) colonne: i16,
    pub(crate) ligne: u8,
    pub(crate) sens: u8,
    pub(crate) vitesse: u8,
    pub(crate) objet: u8,
    pub(crate) hauteur: u8,
    pub(crate) action: u8,
    pub(crate) vie: u8,
    pub(crate) agressivite: u8,
    pub(crate) souffle: u8,
    pub(crate) deltah: u8,
    pub(crate) blockimpact: u8,
    pub(crate) ligneimpact: u8,
    pub(crate) colimpact: u8,
    pub(crate) forcehache: u8,
    pub(crate) coeur: i16,
    pub(crate) cptrsouffle: i16,
    pub(crate) nbresag: i16,
    pub(crate) animal: i16,
    pub(crate) linsag: i16,
    pub(crate) colsag: i16,
    pub(crate) snssag: i16,
    pub(crate) fuite: i16,
    pub(crate) flg_mort: i16,
    pub(crate) poslcoeur: i16,
    pub(crate) posccoeur: i16,
    pub(crate) sapanim: [Option<AnimPtr>; 15],
    pub(crate) cross: [u8; 8],
    pub(crate) adversaire: SapId,
    pub(crate) possession: ListeId,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct brin {
    pub(crate) type_: u16,
    pub(crate) lin: u16,
    pub(crate) col: u16,
    pub(crate) sens: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct h_chron {
    pub(crate) h: i16,
    pub(crate) q: i16,
    pub(crate) pal: Rc<RefCell<[i16; 32]>>,
    pub(crate) TabCouleursCiel: [IndexColor; NB_LIGNES_CIEL],
    pub(crate) TabPaletteDistance: Vec<Vec<i16>>,
}

impl Default for h_chron {
    fn default() -> Self {
        Self {
            h: 0,
            q: 0,
            pal: Rc::new(RefCell::new([0; 32])),
            TabCouleursCiel: [IndexColor::default(); NB_LIGNES_CIEL],
            TabPaletteDistance: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct evt {
    pub(crate) type_: u8,
    pub(crate) posx: u8,
    pub(crate) posy: u8,
    pub(crate) distmax: u8,
    pub(crate) rnd: u8,
    pub(crate) evt: u8,
    pub(crate) message: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct gemcadre {
    pub(crate) ligne: i16,
    pub(crate) colon: i16,
    pub(crate) haut: i16,
    pub(crate) larg: i16,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct IndexColor {
    pub(crate) index: i16,
    pub(crate) rgb: RGBColor,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Vue {
    AUCUNE = 0,
    PAYSAGE = 1,
    LATERALE = 2,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum FormeVue {
    BUSTE = 3,
    TETE = 4,
    ARME = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct reference {
    pub(crate) nomref: [u8; 64],
    pub(crate) param1: u8,
    pub(crate) param2: u8,
}

impl Default for reference {
    fn default() -> Self {
        Self {
            nomref: [0; 64],
            param1: 0,
            param2: 0,
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
#[derive(Debug, Default, Clone)]
pub(crate) struct Forme {
    pub(crate) nbrdisk: u8,
    pub(crate) relief: u8,
    pub(crate) hauteur: u16,
    pub(crate) rayon: u16,
    pub(crate) dskrayon: [u16; 16usize],
    pub(crate) dskdelta: [i8; 32usize],
    pub(crate) num_mob: i16,
    pub(crate) numMob3d: i16,
}

/// Identifie une des formes statiques du décor 3d (voir `sap3d.rs`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(crate) enum FormeId {
    #[default]
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
    pub(crate) distance: u16,
    pub(crate) posx: u16,
    pub(crate) posy: u16,
    pub(crate) ign: u16,
    pub(crate) forme: FormeId,
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
pub(crate) const NB_ETOILES: usize = 10000;
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
pub(crate) const NB_MAX_MOB: u32 = 640;
pub(crate) const POLICE_LISSAGE: u32 = 2;
pub(crate) const STYLE_LISSAGE: u32 = 1;
pub(crate) const STR_VRAC: u32 = 0;
pub(crate) const SAGAIE: u32 = 23;
pub(crate) const NB_MOTS_BKMOTS: u32 = 46;
pub(crate) const NBREVTHEURE: usize = 37;
pub(crate) const NB_LIGNES_CIEL: usize = 128;
pub(crate) const NB_LIGNES_TERRE: u32 = 72;
pub(crate) const NB_DEGRADE_DISTANCE: u32 = 72;
pub(crate) const NBREVTDESTINE: u32 = 40;
pub(crate) const gemnbrecad: u32 = 4;
pub(crate) const geminterv: u32 = 4;
pub(crate) const BASEMOB: i16 = 0;
