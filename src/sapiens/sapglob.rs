/****************************Version 2. 02*******************************
                        VARIABLES GLOBALES
                Mercredi 10 Décembre 1986
***********************************************************************/
#![allow(non_upper_case_globals)]

use crate::sapiens::Sapiens;
use crate::sapiens::{
    CelluleId, IndexColor, ListeCle, NB_LIGNES_CIEL, SapCle, TypeMuse, TypeReference, Vue, cellule,
    liste, sap,
};
use crate::sapiens::{evt, gemcadre, gemnbrecad, h_chron};
use bytemuck::bytes_of_mut;
use std::cell::RefCell;
use std::rc::Rc;
use strum::EnumIter;

pub(crate) static tabprf: [u8; 4] = [25, 33, 34, 35]; /*objet prefere*/
pub(crate) static tobje: [[u8; 4]; 4] = [
    /*objets de la tribu*/
    [23, 26, 28, 38],
    [21, 25, 29, 39],
    [36, 27, 3, 23],
    [25, 24, 26, 23],
];

pub(crate) static tenerv: [u16; 4] = [256, 4864, 25600, 40960]; /*enervement tribu -etranger*/

/* HOMME/FEMME/OUI sont des champs de Sapglob (voir plus bas). */

/*liste initialisee par init_glob*/
const fn elems<const N: usize>(v: [u8; N]) -> [u8; 30] {
    let mut out = [0; 30];
    let mut i = 0;
    while i < N {
        out[i] = v[i];
        i += 1;
    }
    out
}

/***=======================================================================*/
/*              Inits des tableaux des menus                               */
pub(crate) static tclin: [i8; 9] = [18, 19, 18, 18, 19, 18, 19, 20, 18];
pub(crate) static tccol: [i8; 9] = [4, 22, 23, 28, 26, 31, 33, 23, 26];
pub(crate) static tcolcdr: [i8; 9] = [11, 17, 17, 18, 12, 12, 12, 17, 13];
pub(crate) static tcnlin: [i8; 9] = [7, 7, 8, 6, 6, 3, 2, 5, 6];
pub(crate) static tcncol: [i8; 9] = [13, 16, 28, 12, 16, 16, 9, 26, 17];
pub(crate) static tposcur: [i16; 9] = [6082, 5787, 6121, 5795, 6432, 6118, 6441, 6445, 5792];

/*tableaux*/
pub(crate) static hautlin: [i8; 4] = [10, 15, 25, 30];
pub(crate) static tabinp: [i8; 4] = [3, 5, 20, 30];
pub(crate) static hautou: [i8; 4] = [10, 20, 27, 35];
pub(crate) static tribforce: [i8; 4] = [15, 15, 25, 20];
pub(crate) static tribvie: [i8; 4] = [160u8 as i8, 150u8 as i8, 200u8 as i8, 180u8 as i8];
pub(crate) static bestlin: [i8; 4] = [2, 0xDDu8 as i8, 2, 2];
pub(crate) static bestvie: [i8; 4] = [70, 200u8 as i8, 1, 1];
pub(crate) static bestagress: [i8; 4] = [200u8 as i8, 180u8 as i8, 200u8 as i8, 180u8 as i8];
pub(crate) static besthaut: [i8; 4] = [2, 1, 3, 3];
pub(crate) static qtviande: [i8; 4] = [1, 2, 4, 8];

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub(crate) enum ListeId {
    #[default]
    Liste0,
    Liste1,
    Liste2,
    Liste3,
    Liste4,
    Liste5,
    Liste6,
    Liste7,
    Liste8,
    Liste10,
    Liste11,
    Liste12,
    Listbidon,
    Zoneu,
}

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub(crate) enum SapId {
    #[default]
    Sap1,
    Sap2,
    Sap3,
    Sap4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AnimTable {
    Anjm1,
    Anjm2,
    Anjm3,
    Anacr1,
    Anacr2,
    Angen1,
    Angen2,
    Anaga1,
    Anaga2,
    Anrga1,
    Anrga2,
    Anbras,
    Anbra1,
    Anbra2,
    Anhac1,
    Ansag1,
    Anpoin,
    Anhach,
    Ansag,
    Anbust,
    Anbst1,
    Antete,
    Antet1,
    Antet2,
    Anarm1,
    Anarm3,
    Anarm4,
    Aneutr,
    Anlou1,
    Anlou2,
    Anlap1,
    Anlap2,
    Anlap3,
}

/// Curseur dans une table d'animation (remplace l'ancien `*mut i8`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct AnimPtr {
    table: AnimTable,
    off: u16,
}

impl AnimPtr {
    pub(crate) const fn new(table: AnimTable) -> Self {
        Self { table, off: 0 }
    }

    const fn at(table: AnimTable, offset: u16) -> Self {
        Self { table, off: offset }
    }
}

pub(crate) static soma: [cellule; 53] = [
    cellule {
        adresse: CelluleId::Soif,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Fatig,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Faim,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Mort,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Bless,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Zoom,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Agilite,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Force,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Resistance,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Charisme,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Vie,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Sexe,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Viecopain,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Sap(SapId::Sap1, SapCle::Cross),
        taille: 8,
    },
    cellule {
        adresse: CelluleId::Sap(SapId::Sap3, SapCle::Cross),
        taille: 8,
    },
    cellule {
        adresse: CelluleId::Scenar,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Scenpr,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Ciel,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Terre,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Liste(ListeId::Liste1, ListeCle::Taille),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Liste(ListeId::Liste1, ListeCle::Elements),
        taille: 30,
    },
    cellule {
        adresse: CelluleId::Tagress(0),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Enetoi(0),
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Tagress(1),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Enetoi(1),
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Tagress(2),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Enetoi(2),
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Tagress(3),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Enetoi(3),
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Xj,
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Yj,
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Dzeta,
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Heure,
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Sap(SapId::Sap3, SapCle::Vie),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::SapTxtNomjoueur,
        taille: 8,
    },
    cellule {
        adresse: CelluleId::SapTxtNomcopain,
        taille: 6,
    },
    cellule {
        adresse: CelluleId::Present(2),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Jour_act,
        taille: 2,
    },
    cellule {
        adresse: CelluleId::Param,
        taille: 6,
    },
    cellule {
        adresse: CelluleId::Tabclav1,
        taille: 7,
    },
    cellule {
        adresse: CelluleId::Tabclav2,
        taille: 7,
    },
    cellule {
        adresse: CelluleId::Liste(ListeId::Liste8, ListeCle::Element(4)),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Liste(ListeId::Liste8, ListeCle::Element(5)),
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Bidon,
        taille: 1,
    },
    cellule {
        adresse: CelluleId::Sap3dFreqOrage,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dDistMinFoudroie,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dNbNuages,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dNbOiseaux,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dNbRochers,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dNbHommes,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dNbArbres,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::SapcombNbHerbes,
        taille: 4,
    },
    cellule {
        adresse: CelluleId::Sap3dVitesseDeplacementNuages,
        taille: 4,
    },
];

pub(crate) struct Sapglob {
    /*définitions des variables globales*/
    pub(crate) lpos: u8,
    pub(crate) stock1: u8,
    pub(crate) stock2: u8,
    pub(crate) criter: u8,
    pub(crate) lastmen: u8,
    pub(crate) rega: u8,

    pub(crate) who: i8,

    /* touches « homme », « femme », « oui » selon la langue (InitAndCheckTexts) */
    pub(crate) homme: u8,
    pub(crate) femme: u8,
    pub(crate) oui: u8,

    pub(crate) dzeta: i16,
    pub(crate) ldzeta: i16,
    pub(crate) colaff: i16,
    pub(crate) linaff: i16,
    pub(crate) profond: [u8; 5], /*objet prefere ?*/
    pub(crate) enetoi: [u16; 4], /*enerv contre toi*/
    pub(crate) okmuse: TypeMuse,

    pub(crate) listdep: [u8; 9],
    pub(crate) maxparam: [[u8; 6]; 2],
    pub(crate) agrdep: [u8; 4],
    pub(crate) tagress: [u8; 4], /*agressivite*/

    /*donnees du menu en cours*/
    /**************************/
    pub(crate) menu: u16,   /*menu en cours*/
    pub(crate) clin: u16,   /*debut du cadre en ligne64*/
    pub(crate) ccol: u16,   /*debut du cadre en colonne 64*/
    pub(crate) colcdr: u16, /*nbre de colonnes40 du cadre*/
    pub(crate) cnlin: u16,  /*nbre de lignes du cadre*/
    pub(crate) cncol: u16,  /*nbre de colonnes du cadre*/

    pub(crate) poscur: i16,      /*position curseur*/
    pub(crate) stdlist: ListeId, /*liste std pour ce cadre*/
    /*------- definition pour les gestions de cadre ------*/
    pub(crate) gemzoom: [gemcadre; gemnbrecad as usize],
    pub(crate) liste0: liste,
    pub(crate) liste1: liste,
    pub(crate) liste2: liste,
    liste3: liste,
    pub(crate) liste4: liste,
    liste5: liste,
    liste6: liste,
    liste7: liste,
    liste8: liste,
    pub(crate) list10: liste,
    list11: liste,
    list12: liste,
    pub(crate) listbidon: liste,
    pub(crate) zoneu: liste, /*liste de travail pour critere*/
    pub(crate) cross: liste,
    pub(crate) tlpos: [i8; 9],
    pub(crate) tstdlist: [ListeId; 9], /* &liste0..&liste8, initialise par init_glob */
    /*Variables Globales*/
    pub(crate) distsp: i16,
    pub(crate) presence: i16, // bool ?
    pub(crate) fin: u8,
    pub(crate) sensmob: u8,
    pub(crate) ptranim: Option<AnimPtr>,
    pub(crate) present: [i8; 4],
    anjm1: [i8; 48],
    anjm2: [i8; 6],
    anjm3: [i8; 4],
    anacr1: [i8; 6],
    anacr2: [i8; 4],
    angen1: [i8; 6],
    angen2: [i8; 4],
    anaga1: [i8; 26],
    anaga2: [i8; 14],
    anrga1: [i8; 26],
    anrga2: [i8; 14],
    anbras: [i8; 31],
    anbra1: [i8; 4],
    anbra2: [i8; 4],
    anhac1: [i8; 4],
    ansag1: [i8; 4],
    anpoin: [i8; 14],
    anhach: [i8; 18],
    ansag: [i8; 10],
    anbust: [i8; 4],
    anbst1: [i8; 16],
    antete: [i8; 13],
    antet1: [i8; 7],
    antet2: [i8; 4],
    anarm1: [i8; 20],
    anarm3: [i8; 4],
    anarm4: [i8; 4],
    pub(crate) aneutr: [i8; 2],
    pub(crate) anlou1: [i8; 4],
    pub(crate) anlou2: [i8; 19],
    pub(crate) anlap1: [i8; 7],
    pub(crate) anlap2: [i8; 12],
    pub(crate) anlap3: [i8; 13],
    /*blocks animation -- initialise par init_glob*/
    pub(crate) debban: [[Option<AnimPtr>; 14]; 13],
    pub(crate) sap1: sap,
    pub(crate) sap2: sap,
    pub(crate) sap3: sap,
    pub(crate) sap4: sap,
    /*--------------------------- EVENEMENTS ----------------------------*/
    pub(crate) destine: [evt; 40],
    pub(crate) posxf: i16,
    pub(crate) posyf: i16,

    pub(crate) soif: u8,
    pub(crate) fatig: u8,
    pub(crate) faim: u8,
    pub(crate) mort: u8,
    pub(crate) bless: u8,
    pub(crate) echan: u8,
    pub(crate) vlchan: u8,
    pub(crate) flefba: u8,
    pub(crate) predil: u8,
    pub(crate) fdialo: u8,
    pub(crate) fpress: u8,
    pub(crate) chef: u8,
    pub(crate) tribu: u8,
    pub(crate) scenar: u8,
    pub(crate) flcomb: u8,
    pub(crate) agress: u8,
    pub(crate) entvie: u8,
    pub(crate) sexe: u8,
    pub(crate) dquest: u8,
    pub(crate) dervue: Vue,
    pub(crate) source: u8,
    pub(crate) minute: u8,
    pub(crate) sapvit: u8,
    pub(crate) viecopain: u8,
    pub(crate) sap3clav: u8,
    pub(crate) cptagil: u8,
    pub(crate) finjou: u8,
    pub(crate) agilite: u8,
    pub(crate) force: u8,
    pub(crate) resistance: u8,
    pub(crate) charisme: u8,
    pub(crate) vie: u8,
    pub(crate) flagbadi: u8,
    pub(crate) ciel: u8,
    pub(crate) terre: u8,
    pub(crate) flgcharism: u8,
    pub(crate) scenpr: i8,

    pub(crate) enerv: u16,
    pub(crate) xj: u16,
    pub(crate) yj: u16,
    pub(crate) zj: u16,
    pub(crate) derxj: u16,
    pub(crate) deryj: u16,
    bidon: u16,
    pub(crate) dismin: u16,
    pub(crate) heure: u16,
    pub(crate) metpos: u16,
    pub(crate) angsol: u16,
    pub(crate) jour_act: u16,
    pub(crate) jour_dsk: u16,

    pub(crate) zoom: u8,
    tabclav1: [u8; 7],
    tabclav2: [u8; 7],
    pub(crate) param: [u8; 6],

    pub(crate) palhaute: Rc<RefCell<[i16; 32]>>,
    pub(crate) palbasse: Rc<RefCell<[i16; 32]>>,
    pub(crate) pal_chron: Rc<RefCell<[i16; 32]>>,
    pub(crate) ciel_degr: Rc<RefCell<[i16; 32]>>,
    pub(crate) pos_evt: i16,

    pub(crate) tabheur: [h_chron; 37],
    pub(crate) sposx: i16,
    pub(crate) sposy: i16,
    pub(crate) dsposx: i16,
    pub(crate) dsposy: i16,
    pub(crate) dicone: i16,
    pub(crate) s_menu: i16,
    pub(crate) icone: i16,
    pub(crate) bouton: i16,
    pub(crate) s_action: i16,
    pub(crate) smenu: [i8; 2],
}

impl Sapiens {
    /// Octets de la variable désignée par une cellule de `soma` (équivalent du
    /// pointeur `adresse` du C), utilisés pour la sauvegarde / le chargement.
    pub(crate) fn soma_mut(&mut self, cellule: CelluleId) -> &mut [u8] {
        let g = &mut self.sapglob;
        match cellule {
            CelluleId::Soif => bytes_of_mut(&mut g.soif),
            CelluleId::Fatig => bytes_of_mut(&mut g.fatig),
            CelluleId::Faim => bytes_of_mut(&mut g.faim),
            CelluleId::Mort => bytes_of_mut(&mut g.mort),
            CelluleId::Bless => bytes_of_mut(&mut g.bless),
            CelluleId::Zoom => bytes_of_mut(&mut g.zoom),
            CelluleId::Agilite => bytes_of_mut(&mut g.agilite),
            CelluleId::Force => bytes_of_mut(&mut g.force),
            CelluleId::Resistance => bytes_of_mut(&mut g.resistance),
            CelluleId::Charisme => bytes_of_mut(&mut g.charisme),
            CelluleId::Vie => bytes_of_mut(&mut g.vie),
            CelluleId::Sexe => bytes_of_mut(&mut g.sexe),
            CelluleId::Viecopain => bytes_of_mut(&mut g.viecopain),
            CelluleId::Scenar => bytes_of_mut(&mut g.scenar),
            CelluleId::Scenpr => bytes_of_mut(&mut g.scenpr),
            CelluleId::Ciel => bytes_of_mut(&mut g.ciel),
            CelluleId::Xj => bytes_of_mut(&mut g.xj),
            CelluleId::Yj => bytes_of_mut(&mut g.yj),
            CelluleId::Dzeta => bytes_of_mut(&mut g.dzeta),
            CelluleId::Heure => bytes_of_mut(&mut g.heure),
            CelluleId::Jour_act => bytes_of_mut(&mut g.jour_act),
            CelluleId::Param => bytes_of_mut(&mut g.param),
            CelluleId::Tabclav1 => bytes_of_mut(&mut g.tabclav1),
            CelluleId::Tabclav2 => bytes_of_mut(&mut g.tabclav2),
            CelluleId::Bidon => bytes_of_mut(&mut g.bidon),
            CelluleId::Terre => bytes_of_mut(&mut g.terre),
            CelluleId::Tagress(i) => bytes_of_mut(&mut g.tagress[i]),
            CelluleId::Enetoi(i) => bytes_of_mut(&mut g.enetoi[i]),
            CelluleId::Present(i) => bytes_of_mut(&mut g.present[i]),
            CelluleId::Sap(id, SapCle::Cross) => bytes_of_mut(&mut g.sap_mut(id).cross),
            CelluleId::Sap(id, SapCle::Vie) => bytes_of_mut(&mut g.sap_mut(id).vie),
            CelluleId::Liste(id, ListeCle::Taille) => bytes_of_mut(&mut g.liste_mut(id).taille),
            CelluleId::Liste(id, ListeCle::Elements) => bytes_of_mut(&mut g.liste_mut(id).elements),
            CelluleId::Liste(id, ListeCle::Element(i)) => {
                bytes_of_mut(&mut g.liste_mut(id).elements[i])
            }
            CelluleId::SapTxtNomjoueur => bytes_of_mut(&mut self.saptxt.nomjoueur),
            CelluleId::SapTxtNomcopain => bytes_of_mut(&mut self.saptxt.nomcopain),
            CelluleId::Sap3dFreqOrage => bytes_of_mut(&mut self.sap3d.FreqOrage),
            CelluleId::Sap3dDistMinFoudroie => bytes_of_mut(&mut self.sap3d.DistMinFoudroie),
            CelluleId::Sap3dNbNuages => bytes_of_mut(&mut self.sap3d.nbNuages),
            CelluleId::Sap3dNbOiseaux => bytes_of_mut(&mut self.sap3d.nbOiseaux),
            CelluleId::Sap3dNbRochers => bytes_of_mut(&mut self.sap3d.nbRochers),
            CelluleId::Sap3dNbHommes => bytes_of_mut(&mut self.sap3d.nbHommes),
            CelluleId::Sap3dNbArbres => bytes_of_mut(&mut self.sap3d.nbArbres),
            CelluleId::SapcombNbHerbes => bytes_of_mut(&mut self.sapcomb.nbHerbes),
            CelluleId::Sap3dVitesseDeplacementNuages => {
                bytes_of_mut(&mut self.sap3d.VitesseDeplacementNuages)
            }
        }
    }
}

impl Default for Sapglob {
    fn default() -> Self {
        Self {
            lpos: 1,
            stock1: 0,
            stock2: 0,
            criter: 0,
            lastmen: 255,
            rega: 0,

            who: 0,

            homme: b'H',
            femme: b'F',
            oui: b'O',

            dzeta: 0,
            ldzeta: 0,
            colaff: 1,
            linaff: 1,
            profond: [10, 8, 6, 4, 2], /*objet prefere ?*/
            enetoi: [0, 0, 0, 0],      /*enerv contre toi*/
            okmuse: TypeMuse::MUSIQUE_HARMONY_QUALITE,
            listdep: [21, 23, 23, 27, 37, 36, 19, 19, 25],
            maxparam: [
                [240, 180, 254, 200, 130, 254],
                [200, 254, 130, 200, 200, 254],
            ],
            agrdep: [110, 150, 200, 201],
            tagress: [110, 150, 200, 201], /*agressivite*/

            /*donnees du menu en cours*/
            /**************************/
            menu: 0,    /*menu en cours*/
            clin: 1,    /*debut du cadre en ligne64*/
            ccol: 1,    /*debut du cadre en colonne 64*/
            colcdr: 10, /*nbre de colonnes40 du cadre*/
            cnlin: 10,  /*nbre de lignes du cadre*/
            cncol: 10,  /*nbre de colonnes du cadre*/

            poscur: 0,                /*position curseur*/
            stdlist: ListeId::Liste0, /*liste std pour ce cadre*/
            /*------- definition pour les gestions de cadre ------*/
            gemzoom: [gemcadre {
                ligne: 0,
                colon: 0,
                haut: 0,
                larg: 0,
            }; gemnbrecad as usize],
            liste0: liste {
                typeRef: TypeReference::REFER0,
                taille: 11,
                maxtaille: 20,
                elements: elems([11, 10, 5, 4, 3, 6, 1, 2, 7, 8, 9]),
            },
            liste1: liste {
                typeRef: TypeReference::REFER1,
                taille: 16,
                maxtaille: 30,
                elements: elems([
                    21, 23, 23, 27, 19, 19, 19, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ]),
            },
            liste2: liste {
                typeRef: TypeReference::REFER2,
                taille: 28,
                maxtaille: 0,
                elements: elems([
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28,
                ]),
            },
            liste3: liste {
                typeRef: TypeReference::REFER3,
                taille: 4,
                maxtaille: 0,
                elements: elems([1, 2, 3, 4]),
            },
            liste4: liste {
                typeRef: TypeReference::REFER1,
                taille: 2,
                maxtaille: 30,
                elements: elems([2, 3, 4]),
            },
            liste5: liste {
                typeRef: TypeReference::REFER3,
                taille: 3,
                maxtaille: 0,
                elements: elems([5, 6, 7]),
            },
            liste6: liste {
                typeRef: TypeReference::REFER1,
                taille: 2,
                maxtaille: 0,
                elements: elems([23, 21]),
            },
            liste7: liste {
                typeRef: TypeReference::REFER3,
                taille: 5,
                maxtaille: 0,
                elements: elems([8, 9, 10, 11, 12]),
            },
            liste8: liste {
                typeRef: TypeReference::REFER3,
                taille: 5,
                maxtaille: 0,
                elements: elems([19, 13, 14, 15, 26]), /* Informations, Charge, Sauve, personnage, Pr�f�rences */
            },
            list10: liste {
                typeRef: TypeReference::REFER3,
                taille: 0,
                maxtaille: 20,
                elements: [0; 30],
            },
            list11: liste {
                typeRef: TypeReference::REFER3,
                taille: 0,
                maxtaille: 20,
                elements: [0; 30],
            },
            list12: liste {
                typeRef: TypeReference::REFER3,
                taille: 0,
                maxtaille: 20,
                elements: [0; 30],
            },
            listbidon: liste {
                typeRef: TypeReference::REFER0,
                taille: 0,
                maxtaille: 11,
                elements: [0; 30],
            },
            zoneu: liste {
                typeRef: TypeReference::REFER1,
                taille: 0,
                maxtaille: 30,
                elements: [0; 30],
            }, /*liste de travail pour critere*/
            cross: liste {
                typeRef: TypeReference::REFER1,
                taille: 0,
                maxtaille: 30,
                elements: [0; 30],
            },
            tlpos: [1; 9],
            tstdlist: [
                ListeId::Liste0,
                ListeId::Liste1,
                ListeId::Liste2,
                ListeId::Liste3,
                ListeId::Liste4,
                ListeId::Liste5,
                ListeId::Liste6,
                ListeId::Liste7,
                ListeId::Liste8,
            ], /* &liste0..&liste8, initialise par init_glob */
            /*Variables Globales*/
            distsp: 0,
            presence: 0,
            fin: 0,
            sensmob: 0,
            ptranim: None,
            present: [0; 4],
            anjm1: [
                0x82u8 as i8,
                0x81u8 as i8,
                0,
                26,
                0,
                0x80u8 as i8,
                -2,
                0x81u8 as i8,
                1,
                26,
                0,
                0x81u8 as i8,
                2,
                26,
                0,
                0x80u8 as i8,
                -1,
                0x81u8 as i8,
                3,
                26,
                0,
                0x81u8 as i8,
                4,
                26,
                0,
                0x80u8 as i8,
                0,
                /*anjm1b =anjm1+27*/
                0x81u8 as i8,
                5,
                26,
                0,
                0x81u8 as i8,
                6,
                26,
                0,
                0x81u8 as i8,
                7,
                26,
                0,
                0x81u8 as i8,
                8,
                26,
                0,
                0x81u8 as i8,
                9,
                26,
                0,
                0xFFu8 as i8,
            ],
            anjm2: [0x80u8 as i8, 0, 0, 26, 0, 0xFFu8 as i8],
            anjm3: [4, 26, 0, 0xFFu8 as i8],
            anacr1: [0x80u8 as i8, 2, 15, 27, 1, 0xFFu8 as i8],
            anacr2: [16, 27, 2, 0xFFu8 as i8],
            angen1: [0x80u8 as i8, 13, 17, 19, 0, 0xFFu8 as i8],
            angen2: [18, 24, 0, 0xFFu8 as i8],
            anaga1: [
                15,
                27,
                1,
                0x80u8 as i8,
                -1,
                8,
                26,
                0,
                0x81u8 as i8,
                0x80u8 as i8,
                -2,
                9,
                26,
                0,
                0x81u8 as i8,
                0x80u8 as i8,
                -1,
                0,
                26,
                0,
                0x80u8 as i8,
                0,
                0x82u8 as i8,
                0x86u8 as i8,
                127,
                0xFFu8 as i8,
            ],
            anaga2: [
                16,
                27,
                2,
                5,
                26,
                0,
                6,
                26,
                0,
                6,
                26,
                0,
                0x82u8 as i8,
                0xFFu8 as i8,
            ],
            anrga1: [
                15,
                27,
                1,
                0x80u8 as i8,
                -1,
                0,
                26,
                0,
                0x85u8 as i8,
                0x80u8 as i8,
                -2,
                9,
                26,
                0,
                0x80u8 as i8,
                -1,
                0x85u8 as i8,
                8,
                26,
                0,
                0x80u8 as i8,
                0,
                0x82u8 as i8,
                0x86u8 as i8,
                127,
                0xFFu8 as i8,
            ],
            anrga2: [
                16,
                27,
                2,
                6,
                26,
                0,
                6,
                26,
                0,
                5,
                26,
                0,
                0x82u8 as i8,
                0xFFu8 as i8,
            ],
            anbras: [
                31,
                12,
                0,
                32,
                12,
                0,
                33,
                12,
                0,
                34,
                12,
                0,
                35,
                12,
                0,
                /*anbrad = anbras+15*/
                35,
                12,
                0,
                34,
                12,
                0,
                33,
                12,
                0,
                32,
                12,
                0,
                31,
                12,
                0,
                0xFFu8 as i8,
            ],
            anbra1: [34, 12, 0, 0xFFu8 as i8],
            anbra2: [33, 12, 0, 0xFFu8 as i8],
            anhac1: [38, 12, 0, 0xFFu8 as i8],
            ansag1: [36, 9, 2, 0xFFu8 as i8],
            anpoin: [
                34,
                12,
                0,
                107,
                12,
                -1,
                0x87u8 as i8,
                3,
                37,
                12,
                1,
                0x86u8 as i8,
                252u8 as i8,
                0xFFu8 as i8,
            ],
            anhach: [
                38,
                12,
                0,
                39,
                12,
                -1,
                40,
                12,
                -1,
                41,
                14,
                0,
                39,
                12,
                -1,
                0x86u8 as i8,
                252u8 as i8,
                0xFFu8 as i8,
            ],
            ansag: [
                36,
                9,
                2,
                37,
                9,
                1,
                0x86u8 as i8,
                252u8 as i8,
                0x88u8 as i8,
                0xFFu8 as i8,
            ],
            anbust: [120, 12, 1, 0xFFu8 as i8],
            anbst1: [
                0x80u8 as i8,
                18,
                121,
                12,
                1,
                121,
                12,
                1,
                121,
                12,
                1,
                121,
                12,
                1,
                0x84u8 as i8,
                0xFFu8 as i8,
            ],
            antete: [53, 0, 2, 53, 0, 2, 54, 0, 2, 54, 0, 2, 0xFFu8 as i8],
            antet1: [53, 0, 2, 54, 0, 2, 0xFFu8 as i8],
            antet2: [53, 0, 2, 0xFFu8 as i8],
            anarm1: [
                45,
                7,
                -2,
                47,
                8,
                -3,
                0x87u8 as i8,
                3,
                49,
                6,
                -4,
                51,
                10,
                -1,
                45,
                8,
                -2,
                0x86u8 as i8,
                252u8 as i8,
                0xFFu8 as i8,
            ],
            anarm3: [45, 7, -2, 0xFFu8 as i8],
            anarm4: [43, 11, -1, 0xFFu8 as i8],
            aneutr: [0x83u8 as i8, 0xFFu8 as i8],
            anlou1: [96, 26, 0, 0xFFu8 as i8],
            anlou2: [
                95,
                26,
                0,
                0x81u8 as i8,
                96,
                26,
                0,
                0x81u8 as i8,
                97,
                26,
                0,
                0x81u8 as i8,
                98,
                26,
                0,
                0x81u8 as i8,
                0x87u8 as i8,
                1,
                0xFFu8 as i8,
            ],
            anlap1: [87, 35, 0, 87, 35, 0, 0xFFu8 as i8],
            anlap2: [
                87,
                35,
                0,
                0x81u8 as i8,
                88,
                35,
                0,
                89,
                35,
                0,
                0x81u8 as i8,
                0xFFu8 as i8,
            ],
            anlap3: [90, 28, 2, 90, 28, 2, 90, 28, 2, 90, 28, 2, 0xFFu8 as i8],
            debban: [[None; 14]; 13],
            sap1: sap {
                colonne: 2,
                ligne: 32,
                sens: 0,
                vitesse: 2,
                objet: 0,
                hauteur: 0,
                action: 0,
                vie: 240,
                force: 0,
                agressivite: 128,
                souffle: 255,
                deltah: 0,
                blockimpact: 0,
                ligneimpact: 0,
                colimpact: 0,
                forcehache: 10,
                coeur: 71,
                cptrsouffle: 1,
                nbresag: 5,
                animal: 37,
                linsag: 0,
                colsag: 0,
                snssag: 255,
                fuite: 0,
                flg_mort: 0,
                poslcoeur: 8,
                posccoeur: 5, /* ligne colonne position du coeur */
                sapanim: [None; 15],
                cross: [0; 8],
                adversaire: SapId::Sap1,     /* &sap1, remis dans init_glob */
                possession: ListeId::Liste1, /* &liste1, remis dans init_glob */
            },
            sap2: sap {
                colonne: 2,
                ligne: 54,
                sens: 1,
                vitesse: 2,
                objet: 0,
                hauteur: 0,
                action: 0,
                vie: 240,
                force: 0,
                agressivite: 128,
                souffle: 255,
                deltah: 0,
                blockimpact: 0,
                ligneimpact: 0,
                colimpact: 0,
                forcehache: 10,
                coeur: 71,
                cptrsouffle: 1,
                nbresag: 5,
                animal: 37,
                linsag: 0,
                colsag: 0,
                snssag: 255,
                fuite: 0,
                flg_mort: 0,
                poslcoeur: 8,
                posccoeur: 296, /* ligne colonne position du coeur */
                sapanim: [None; 15],
                cross: [0; 8],
                adversaire: SapId::Sap2, /* &sap2, remis dans init_glob */
                possession: ListeId::Liste10, /* &list10, remis dans init_glob */
            },
            sap3: sap {
                colonne: 2,
                ligne: 54,
                sens: 1,
                vitesse: 2,
                objet: 0,
                hauteur: 0,
                action: 0,
                vie: 240,
                force: 0,
                agressivite: 128,
                souffle: 255,
                deltah: 0,
                blockimpact: 0,
                ligneimpact: 0,
                colimpact: 0,
                forcehache: 10,
                coeur: 71,
                cptrsouffle: 1,
                nbresag: 5,
                animal: 37,
                linsag: 0,
                colsag: 0,
                snssag: 255,
                fuite: 0,
                flg_mort: 0,
                poslcoeur: 56,
                posccoeur: 5, /* ligne colonne position du coeur */
                sapanim: [None; 15],
                cross: [0; 8],
                adversaire: SapId::Sap2, /* &sap2, remis dans init_glob */
                possession: ListeId::Liste11, /* &list11, remis dans init_glob */
            },
            sap4: sap {
                colonne: 2,
                ligne: 54,
                sens: 1,
                vitesse: 2,
                objet: 0,
                hauteur: 0,
                action: 0,
                vie: 240,
                force: 0,
                agressivite: 128,
                souffle: 255,
                deltah: 0,
                blockimpact: 0,
                ligneimpact: 0,
                colimpact: 0,
                forcehache: 10,
                coeur: 71,
                cptrsouffle: 1,
                nbresag: 5,
                animal: 37,
                linsag: 0,
                colsag: 0,
                snssag: 255,
                fuite: 0,
                flg_mort: 0,
                poslcoeur: 56,
                posccoeur: 296, /* ligne colonne position du coeur */
                sapanim: [None; 15],
                cross: [0; 8],
                adversaire: SapId::Sap2, /* &sap2, remis dans init_glob */
                possession: ListeId::Liste12, /* &list12, remis dans init_glob */
            },
            destine: [
                /*chefs -- Type, posx et y, distmax, Rnd, evt, message */
                evt {
                    type_: 1,
                    posx: 0x58,
                    posy: 0x54,
                    distmax: 1,
                    rnd: 245,
                    evt: 0x81,
                    message: 80,
                },
                evt {
                    type_: 1,
                    posx: 155,
                    posy: 56,
                    distmax: 1,
                    rnd: 245,
                    evt: 0x82,
                    message: 80,
                },
                evt {
                    type_: 1,
                    posx: 47,
                    posy: 207,
                    distmax: 1,
                    rnd: 245,
                    evt: 0x83,
                    message: 80,
                },
                evt {
                    type_: 1,
                    posx: 195,
                    posy: 170,
                    distmax: 1,
                    rnd: 245,
                    evt: 0x84,
                    message: 80,
                },
                /*hommes*/
                evt {
                    type_: 0,
                    posx: 88,
                    posy: 81,
                    distmax: 20,
                    rnd: 10,
                    evt: 1,
                    message: 79,
                },
                evt {
                    type_: 0,
                    posx: 155,
                    posy: 56,
                    distmax: 30,
                    rnd: 10,
                    evt: 2,
                    message: 79,
                },
                evt {
                    type_: 0,
                    posx: 47,
                    posy: 207,
                    distmax: 60,
                    rnd: 20,
                    evt: 3,
                    message: 79,
                },
                evt {
                    type_: 0,
                    posx: 195,
                    posy: 170,
                    distmax: 40,
                    rnd: 25,
                    evt: 4,
                    message: 79,
                },
                /*sources*/
                evt {
                    type_: 0,
                    posx: 82,
                    posy: 78,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 48,
                    posy: 111,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 187,
                    posy: 164,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 8,
                    posy: 8,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 152,
                    posy: 3,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 63,
                    posy: 164,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 125,
                    posy: 63,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 176,
                    posy: 99,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 24,
                    posy: 176,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 195,
                    posy: 196,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                },
                evt {
                    type_: 0,
                    posx: 95,
                    posy: 74,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                }, /*Nlles sources */
                evt {
                    type_: 0,
                    posx: 128,
                    posy: 68,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                }, /*Nlles sources */
                evt {
                    type_: 0,
                    posx: 157,
                    posy: 56,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                }, /*Nlles sources */
                evt {
                    type_: 0,
                    posx: 171,
                    posy: 66,
                    distmax: 1,
                    rnd: 255,
                    evt: 0,
                    message: 76,
                }, /*Nlles sources */
                /*lapins*/
                evt {
                    type_: 0,
                    posx: 9,
                    posy: 20,
                    distmax: 40,
                    rnd: 20,
                    evt: 7,
                    message: 77,
                },
                evt {
                    type_: 0,
                    posx: 20,
                    posy: 194,
                    distmax: 20,
                    rnd: 10,
                    evt: 8,
                    message: 77,
                },
                evt {
                    type_: 0,
                    posx: 80,
                    posy: 84,
                    distmax: 10,
                    rnd: 10,
                    evt: 7,
                    message: 77,
                },
                evt {
                    type_: 1,
                    posx: 110,
                    posy: 181,
                    distmax: 10,
                    rnd: 128,
                    evt: 7,
                    message: 77,
                },
                evt {
                    type_: 1,
                    posx: 110,
                    posy: 33,
                    distmax: 10,
                    rnd: 128,
                    evt: 8,
                    message: 77,
                },
                evt {
                    type_: 1,
                    posx: 179,
                    posy: 234,
                    distmax: 10,
                    rnd: 128,
                    evt: 7,
                    message: 77,
                },
                evt {
                    type_: 1,
                    posx: 195,
                    posy: 147,
                    distmax: 10,
                    rnd: 128,
                    evt: 8,
                    message: 77,
                },
                evt {
                    type_: 1,
                    posx: 202,
                    posy: 23,
                    distmax: 5,
                    rnd: 128,
                    evt: 8,
                    message: 77,
                },
                evt {
                    type_: 1,
                    posx: 236,
                    posy: 154,
                    distmax: 5,
                    rnd: 128,
                    evt: 8,
                    message: 77,
                },
                /*loups*/
                evt {
                    type_: 0,
                    posx: 50,
                    posy: 134,
                    distmax: 20,
                    rnd: 30,
                    evt: 5,
                    message: 74,
                },
                evt {
                    type_: 0,
                    posx: 76,
                    posy: 214,
                    distmax: 20,
                    rnd: 30,
                    evt: 5,
                    message: 74,
                },
                evt {
                    type_: 0,
                    posx: 135,
                    posy: 60,
                    distmax: 20,
                    rnd: 50,
                    evt: 5,
                    message: 74,
                },
                evt {
                    type_: 0,
                    posx: 236,
                    posy: 134,
                    distmax: 20,
                    rnd: 30,
                    evt: 5,
                    message: 74,
                },
                evt {
                    type_: 0,
                    posx: 195,
                    posy: 170,
                    distmax: 10,
                    rnd: 40,
                    evt: 5,
                    message: 74,
                },
                /*ours*/
                evt {
                    type_: 0,
                    posx: 30,
                    posy: 54,
                    distmax: 10,
                    rnd: 10,
                    evt: 6,
                    message: 75,
                },
                evt {
                    type_: 0,
                    posx: 97,
                    posy: 140,
                    distmax: 10,
                    rnd: 10,
                    evt: 6,
                    message: 75,
                },
                evt {
                    type_: 0,
                    posx: 139,
                    posy: 221,
                    distmax: 10,
                    rnd: 15,
                    evt: 6,
                    message: 75,
                },
                evt {
                    type_: 0,
                    posx: 185,
                    posy: 87,
                    distmax: 5,
                    rnd: 20,
                    evt: 6,
                    message: 75,
                },
            ],
            posxf: 0,
            posyf: 0,

            soif: 0,
            fatig: 0,
            faim: 0,
            mort: 0,
            bless: 0,
            echan: 0,
            vlchan: 0,
            flefba: 0,
            predil: 0,
            fdialo: 0,
            fpress: 0,
            chef: 0,
            tribu: 0,
            scenar: 0,
            flcomb: 0,
            agress: 0,
            entvie: 0,
            sexe: 1,
            dquest: 0,
            dervue: Vue::AUCUNE,
            source: 0,
            minute: 0,
            sapvit: 0,
            viecopain: 0,
            sap3clav: 0,
            cptagil: 0,
            finjou: 0,
            agilite: 128,
            force: 128,
            resistance: 128,
            charisme: 128,
            vie: 128,
            flagbadi: 0,
            ciel: 221,
            terre: 136,
            flgcharism: 0,
            scenpr: 0,

            enerv: 0,
            xj: 0,
            yj: 0,
            zj: 0,
            derxj: 0,
            deryj: 0,
            bidon: 0,
            dismin: 1,
            heure: 1,
            metpos: 70,
            angsol: 0,
            jour_act: 0,
            jour_dsk: 0,

            zoom: 64,
            tabclav1: [72, 80, 75, 77, 28, 57, 1],
            tabclav2: [18, 45, 31, 32, 34, 33, 1],
            param: [128, 128, 128, 128, 128, 0],
            palhaute: Rc::new(RefCell::new([0; 32])),
            palbasse: Rc::new(RefCell::new([0; 32])),
            pal_chron: Rc::new(RefCell::new([0; 32])),
            ciel_degr: Rc::new(RefCell::new([0; 32])),
            pos_evt: 0,

            tabheur: [
                hc(
                    0,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641,
                        0x444, 0x030, 0x040, 0x730, 0x230, 0x005,
                    ]),
                ),
                hc(
                    1,
                    1,
                    pal_s([0x003, 0x003, 0x003, 0x003, 0x003, 0x004, 0x004, 0x005]),
                ),
                hc(
                    3,
                    1,
                    pal_s([0x003, 0x003, 0x003, 0x003, 0x103, 0x103, 0x203, 0x203]),
                ),
                hc(
                    5,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x730, 0x340, 0x005,
                    ]),
                ),
                hc(
                    5,
                    1,
                    pal_s([0x013, 0x113, 0x213, 0x212, 0x212, 0x211, 0x301, 0x301]),
                ),
                hc(
                    8,
                    1,
                    pal_s([0x013, 0x113, 0x213, 0x212, 0x211, 0x301, 0x312, 0x401]),
                ),
                hc(
                    10,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x730, 0x340, 0x005,
                    ]),
                ),
                hc(
                    12,
                    1,
                    pal_s([0x014, 0x114, 0x214, 0x213, 0x212, 0x301, 0x401, 0x501]),
                ),
                hc(
                    14,
                    1,
                    pal_s([0x024, 0x114, 0x223, 0x323, 0x313, 0x402, 0x501, 0x600]),
                ),
                hc(
                    15,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x740, 0x440, 0x005,
                    ]),
                ),
                hc(
                    17,
                    1,
                    pal_s([0x024, 0x124, 0x223, 0x322, 0x322, 0x422, 0x522, 0x622]),
                ),
                hc(
                    18,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x750, 0x440, 0x005,
                    ]),
                ),
                hc(
                    19,
                    1,
                    pal_s([0x025, 0x124, 0x224, 0x324, 0x424, 0x434, 0x533, 0x633]),
                ),
                hc(
                    20,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x760, 0x450, 0x005,
                    ]),
                ),
                hc(
                    21,
                    1,
                    pal_s([0x025, 0x035, 0x235, 0x335, 0x435, 0x445, 0x545, 0x645]),
                ),
                hc(
                    24,
                    1,
                    pal_s([0x035, 0x135, 0x235, 0x335, 0x435, 0x435, 0x535, 0x535]),
                ),
                hc(
                    25,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x770, 0x440, 0x005,
                    ]),
                ),
                hc(
                    27,
                    1,
                    pal_s([0x035, 0x035, 0x135, 0x235, 0x335, 0x335, 0x435, 0x435]),
                ),
                hc(
                    31,
                    1,
                    pal_s([0x036, 0x036, 0x136, 0x236, 0x336, 0x336, 0x436, 0x436]),
                ),
                hc(
                    35,
                    1,
                    pal_s([0x036, 0x036, 0x136, 0x236, 0x246, 0x346, 0x446, 0x446]),
                ),
                hc(
                    39,
                    1,
                    pal_s([0x046, 0x046, 0x046, 0x046, 0x056, 0x056, 0x356, 0x356]),
                ),
                hc(
                    45,
                    1,
                    pal_s([0x046, 0x046, 0x046, 0x046, 0x047, 0x047, 0x057, 0x057]),
                ),
                hc(
                    50,
                    1,
                    pal_s([0x047, 0x047, 0x047, 0x047, 0x057, 0x057, 0x067, 0x067]),
                ),
                hc(
                    468,
                    1,
                    pal_s([0x014, 0x025, 0x025, 0x533, 0x434, 0x750, 0x753, 0x754]),
                ),
                hc(
                    470,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x760, 0x440, 0x005,
                    ]),
                ),
                hc(
                    474,
                    1,
                    pal_s([0x014, 0x014, 0x025, 0x536, 0x533, 0x740, 0x750, 0x753]),
                ),
                hc(
                    480,
                    1,
                    pal_s([0x014, 0x014, 0x014, 0x523, 0x630, 0x730, 0x740, 0x750]),
                ),
                hc(
                    481,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x750, 0x340, 0x005,
                    ]),
                ),
                hc(
                    486,
                    1,
                    pal_s([0x014, 0x014, 0x014, 0x304, 0x523, 0x630, 0x730, 0x740]),
                ),
                hc(
                    490,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641,
                        0x444, 0x040, 0x050, 0x740, 0x340, 0x005,
                    ]),
                ),
                hc(
                    492,
                    1,
                    pal_s([0x014, 0x014, 0x014, 0x205, 0x304, 0x523, 0x630, 0x730]),
                ),
                hc(
                    498,
                    1,
                    pal_s([0x003, 0x014, 0x014, 0x005, 0x205, 0x304, 0x523, 0x630]),
                ),
                hc(
                    500,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641,
                        0x444, 0x020, 0x030, 0x666, 0x230, 0x005,
                    ]),
                ),
                hc(
                    504,
                    1,
                    pal_s([0x003, 0x003, 0x003, 0x004, 0x005, 0x205, 0x304, 0x523]),
                ),
                hc(
                    510,
                    1,
                    pal_s([0x003, 0x3, 0x3, 0x3, 0x4, 0x5, 0x205, 0x304]),
                ),
                hc(516, 1, pal_s([0x1, 0x1, 0x1, 0x2, 0x3, 0x4, 0x5, 0x205])),
                hc(
                    1020,
                    0,
                    pal_s([
                        0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641,
                        0x444, 0x030, 0x040, 0x222, 0x230, 0x005,
                    ]),
                ),
            ],
            sposx: 0,
            sposy: 0,
            dsposx: 0,
            dsposy: 0,
            dicone: 0,
            s_menu: 0,
            icone: 0,
            bouton: 0,
            s_action: 0,
            smenu: [0, -1],
        }
    }
}

impl Sapglob {
    const fn anim_table(&self, table: AnimTable) -> &[i8] {
        match table {
            AnimTable::Anjm1 => &self.anjm1,
            AnimTable::Anjm2 => &self.anjm2,
            AnimTable::Anjm3 => &self.anjm3,
            AnimTable::Anacr1 => &self.anacr1,
            AnimTable::Anacr2 => &self.anacr2,
            AnimTable::Angen1 => &self.angen1,
            AnimTable::Angen2 => &self.angen2,
            AnimTable::Anaga1 => &self.anaga1,
            AnimTable::Anaga2 => &self.anaga2,
            AnimTable::Anrga1 => &self.anrga1,
            AnimTable::Anrga2 => &self.anrga2,
            AnimTable::Anbras => &self.anbras,
            AnimTable::Anbra1 => &self.anbra1,
            AnimTable::Anbra2 => &self.anbra2,
            AnimTable::Anhac1 => &self.anhac1,
            AnimTable::Ansag1 => &self.ansag1,
            AnimTable::Anpoin => &self.anpoin,
            AnimTable::Anhach => &self.anhach,
            AnimTable::Ansag => &self.ansag,
            AnimTable::Anbust => &self.anbust,
            AnimTable::Anbst1 => &self.anbst1,
            AnimTable::Antete => &self.antete,
            AnimTable::Antet1 => &self.antet1,
            AnimTable::Antet2 => &self.antet2,
            AnimTable::Anarm1 => &self.anarm1,
            AnimTable::Anarm3 => &self.anarm3,
            AnimTable::Anarm4 => &self.anarm4,
            AnimTable::Aneutr => &self.aneutr,
            AnimTable::Anlou1 => &self.anlou1,
            AnimTable::Anlou2 => &self.anlou2,
            AnimTable::Anlap1 => &self.anlap1,
            AnimTable::Anlap2 => &self.anlap2,
            AnimTable::Anlap3 => &self.anlap3,
        }
    }

    /// Octet courant de `ptranim`, sans avancer.
    pub(crate) const fn anim_peek(&self) -> i8 {
        let p = self.ptranim.expect("ptranim null");
        self.anim_table(p.table)[p.off as usize]
    }

    /// Octet courant de `ptranim`, puis avance d'un octet.
    pub(crate) const fn anim_read(&mut self) -> i8 {
        let v = self.anim_peek();
        self.anim_skip();
        v
    }

    /// Avance `ptranim` d'un octet.
    pub(crate) const fn anim_skip(&mut self) {
        self.ptranim.as_mut().expect("ptranim null").off += 1;
    }

    pub(crate) const fn sap(&self, sap_id: SapId) -> &sap {
        match sap_id {
            SapId::Sap1 => &self.sap1,
            SapId::Sap2 => &self.sap2,
            SapId::Sap3 => &self.sap3,
            SapId::Sap4 => &self.sap4,
        }
    }

    pub(crate) const fn sap_mut(&mut self, sap_id: SapId) -> &mut sap {
        match sap_id {
            SapId::Sap1 => &mut self.sap1,
            SapId::Sap2 => &mut self.sap2,
            SapId::Sap3 => &mut self.sap3,
            SapId::Sap4 => &mut self.sap4,
        }
    }

    pub(crate) const fn liste(&self, liste_id: ListeId) -> &liste {
        match liste_id {
            ListeId::Liste0 => &self.liste0,
            ListeId::Liste1 => &self.liste1,
            ListeId::Liste2 => &self.liste2,
            ListeId::Liste3 => &self.liste3,
            ListeId::Liste4 => &self.liste4,
            ListeId::Liste5 => &self.liste5,
            ListeId::Liste6 => &self.liste6,
            ListeId::Liste7 => &self.liste7,
            ListeId::Liste8 => &self.liste8,
            ListeId::Liste10 => &self.list10,
            ListeId::Liste11 => &self.list11,
            ListeId::Liste12 => &self.list12,
            ListeId::Listbidon => &self.listbidon,
            ListeId::Zoneu => &self.zoneu,
        }
    }

    pub(crate) const fn liste_mut(&mut self, liste_id: ListeId) -> &mut liste {
        match liste_id {
            ListeId::Liste0 => &mut self.liste0,
            ListeId::Liste1 => &mut self.liste1,
            ListeId::Liste2 => &mut self.liste2,
            ListeId::Liste3 => &mut self.liste3,
            ListeId::Liste4 => &mut self.liste4,
            ListeId::Liste5 => &mut self.liste5,
            ListeId::Liste6 => &mut self.liste6,
            ListeId::Liste7 => &mut self.liste7,
            ListeId::Liste8 => &mut self.liste8,
            ListeId::Liste10 => &mut self.list10,
            ListeId::Liste11 => &mut self.list11,
            ListeId::Liste12 => &mut self.list12,
            ListeId::Listbidon => &mut self.listbidon,
            ListeId::Zoneu => &mut self.zoneu,
        }
    }

    pub(crate) const fn peuple(&self, sap_id: SapId) -> &sap {
        self.sap(sap_id)
    }

    pub(crate) const fn peuple_mut(&mut self, sap_id: SapId) -> &mut sap {
        self.sap_mut(sap_id)
    }

    pub(crate) fn peuple_iter(&self) -> PeupleIter<'_> {
        PeupleIter {
            sap: [&self.sap1, &self.sap2, &self.sap3, &self.sap4],
            index: 0,
        }
    }
}

pub(crate) struct PeupleIter<'a> {
    sap: [&'a sap; 4],
    index: usize,
}

impl<'a> Iterator for PeupleIter<'a> {
    type Item = &'a sap;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.sap.get(self.index).copied();
        self.index += usize::from(ret.is_some());
        ret
    }
}

/* Heure,ciel,terre */
const fn pal_s<const N: usize>(v: [i16; N]) -> [i16; 32] {
    let mut out = [0; 32];
    let mut i = 0;
    while i < N {
        out[i] = v[i];
        i += 1;
    }
    out
}

fn hc(h: i16, q: i16, pal: [i16; 32]) -> h_chron {
    h_chron {
        h,
        q,
        pal: Rc::new(RefCell::new(pal)),
        TabCouleursCiel: [IndexColor::default(); NB_LIGNES_CIEL],
        TabPaletteDistance: Vec::new(),
    }
}

/********************** END OF DATAS AND VARIABLES **************/

impl Sapiens {
    /// Pointeurs reliant les tables entre elles (debban, tstdlist, peuple,
    /// sapanim, adversaire, possession). Equivalent des initialisateurs
    /// statiques auto-referents du C. A appeler avant sapiens_main.
    pub(crate) fn init_glob(&mut self) {
        self.sapglob.debban = [
            /*marche*/
            [
                Some(AnimPtr::new(AnimTable::Anjm1)),
                Some(AnimPtr::new(AnimTable::Anjm1)),
                Some(AnimPtr::new(AnimTable::Anbras)),
                Some(AnimPtr::new(AnimTable::Anbras)),
                Some(AnimPtr::at(AnimTable::Anjm1, 27)),
                Some(AnimPtr::new(AnimTable::Anjm1)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                Some(AnimPtr::new(AnimTable::Antete)),
                Some(AnimPtr::new(AnimTable::Antete)),
                None,
                None,
                Some(AnimPtr::at(AnimTable::Anbras, 15)),
                Some(AnimPtr::new(AnimTable::Anbras)),
            ],
            /*arret*/
            [
                Some(AnimPtr::new(AnimTable::Anjm2)),
                Some(AnimPtr::new(AnimTable::Anjm2)),
                Some(AnimPtr::new(AnimTable::Anbra1)),
                Some(AnimPtr::new(AnimTable::Anbra1)),
                Some(AnimPtr::new(AnimTable::Anjm3)),
                Some(AnimPtr::new(AnimTable::Anjm3)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                Some(AnimPtr::new(AnimTable::Antet2)),
                Some(AnimPtr::new(AnimTable::Antet2)),
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anbra2)),
                Some(AnimPtr::new(AnimTable::Anbra2)),
            ],
            /*frappe du poing*/
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Aneutr)),
                None,
                Some(AnimPtr::new(AnimTable::Anpoin)),
                None,
            ],
            /*frappe avec hache*/
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anarm1)),
                None,
                Some(AnimPtr::new(AnimTable::Anhach)),
                None,
            ],
            /*frappe avec sagaie*/
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Ansag)),
                Some(AnimPtr::new(AnimTable::Anbra2)),
            ],
            /*rien en main*/
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                None,
                None,
            ],
            /*hache en main*/
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anarm3)),
                Some(AnimPtr::new(AnimTable::Anarm3)),
                Some(AnimPtr::new(AnimTable::Anhac1)),
                Some(AnimPtr::new(AnimTable::Anhac1)),
            ],
            /*sagaie en main*/
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anarm4)),
                Some(AnimPtr::new(AnimTable::Anarm4)),
                Some(AnimPtr::new(AnimTable::Ansag1)),
                Some(AnimPtr::new(AnimTable::Ansag1)),
            ],
            /*accroupi*/
            [
                Some(AnimPtr::new(AnimTable::Anacr1)),
                Some(AnimPtr::new(AnimTable::Anacr1)),
                Some(AnimPtr::new(AnimTable::Anbra1)),
                Some(AnimPtr::new(AnimTable::Anbra1)),
                Some(AnimPtr::new(AnimTable::Anacr2)),
                Some(AnimPtr::new(AnimTable::Anacr2)),
                None,
                None,
                Some(AnimPtr::new(AnimTable::Antet2)),
                Some(AnimPtr::new(AnimTable::Antet2)),
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anbra2)),
                Some(AnimPtr::new(AnimTable::Anbra2)),
            ],
            /*a genoux*/
            [
                Some(AnimPtr::new(AnimTable::Angen1)),
                Some(AnimPtr::new(AnimTable::Angen1)),
                Some(AnimPtr::new(AnimTable::Anbra1)),
                Some(AnimPtr::new(AnimTable::Anbra1)),
                Some(AnimPtr::new(AnimTable::Angen2)),
                Some(AnimPtr::new(AnimTable::Angen2)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            /*plat ventre*/
            [
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Aneutr)),
                Some(AnimPtr::new(AnimTable::Anbst1)),
                Some(AnimPtr::new(AnimTable::Anbust)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            /*en garde avance*/
            [
                Some(AnimPtr::new(AnimTable::Anaga1)),
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anaga2)),
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Antet1)),
                None,
                None,
                None,
                None,
                None,
            ],
            /*en garde recule*/
            [
                Some(AnimPtr::new(AnimTable::Anrga1)),
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Anrga2)),
                None,
                None,
                None,
                Some(AnimPtr::new(AnimTable::Antet1)),
                None,
                None,
                None,
                None,
                None,
            ],
        ];

        for sap_id in [SapId::Sap1, SapId::Sap2, SapId::Sap3, SapId::Sap4] {
            let s = self.sapglob.sap_mut(sap_id);
            for i in 0..14 {
                s.sapanim[i] = Some(AnimPtr::new(AnimTable::Aneutr));
            }
            s.sapanim[14] = None;
        }
    }
}
