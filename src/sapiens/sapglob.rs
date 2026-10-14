/****************************Version 2. 02*******************************
                        VARIABLES GLOBALES
                Mercredi 10 Décembre 1986
***********************************************************************/
#![allow(non_upper_case_globals)]

use crate::Sapiens;
use crate::sapiens::{
    IndexColor, NB_LIGNES_CIEL, TypeMuse, TypeReference, Vue, cellule, liste, sap,
};
use crate::sapiens::{evt, gemcadre, gemnbrecad, h_chron};
use std::os::raw::{c_char, c_short, c_uchar, c_ushort};
use std::ptr::{addr_of_mut, null_mut};
use strum::EnumIter;

pub(crate) static tabprf: [c_uchar; 4] = [25, 33, 34, 35]; /*objet prefere*/
pub(crate) static tobje: [[c_uchar; 4]; 4] = [
    /*objets de la tribu*/
    [23, 26, 28, 38],
    [21, 25, 29, 39],
    [36, 27, 3, 23],
    [25, 24, 26, 23],
];

pub(crate) static tenerv: [c_ushort; 4] = [256, 4864, 25600, 40960]; /*enervement tribu -etranger*/

/* HOMME/FEMME/OUI sont des champs de Sapglob (voir plus bas). */

/*liste initialisee par init_glob*/
const fn elems<const N: usize>(v: [c_uchar; N]) -> [c_uchar; 30] {
    let mut out = [0 as c_uchar; 30];
    let mut i = 0;
    while i < N {
        out[i] = v[i];
        i += 1;
    }
    out
}

/***=======================================================================*/
/*              Inits des tableaux des menus                               */
pub(crate) static tclin: [c_char; 9] = [18, 19, 18, 18, 19, 18, 19, 20, 18];
pub(crate) static tccol: [c_char; 9] = [4, 22, 23, 28, 26, 31, 33, 23, 26];
pub(crate) static tcolcdr: [c_char; 9] = [11, 17, 17, 18, 12, 12, 12, 17, 13];
pub(crate) static tcnlin: [c_char; 9] = [7, 7, 8, 6, 6, 3, 2, 5, 6];
pub(crate) static tcncol: [c_char; 9] = [13, 16, 28, 12, 16, 16, 9, 26, 17];
pub(crate) static tposcur: [c_short; 9] = [6082, 5787, 6121, 5795, 6432, 6118, 6441, 6445, 5792];

/*tableaux*/
pub(crate) static hautlin: [c_char; 4] = [10, 15, 25, 30];
pub(crate) static tabinp: [c_char; 4] = [3, 5, 20, 30];
pub(crate) static hautou: [c_char; 4] = [10, 20, 27, 35];
pub(crate) static tribforce: [c_char; 4] = [15, 15, 25, 20];
pub(crate) static tribvie: [c_char; 4] = [
    160u8 as c_char,
    150u8 as c_char,
    200u8 as c_char,
    180u8 as c_char,
];
pub(crate) static bestlin: [c_char; 4] = [2, 0xDDu8 as c_char, 2, 2];
pub(crate) static bestvie: [c_char; 4] = [70, 200u8 as c_char, 1, 1];
pub(crate) static bestagress: [c_char; 4] = [
    200u8 as c_char,
    180u8 as c_char,
    200u8 as c_char,
    180u8 as c_char,
];
pub(crate) static besthaut: [c_char; 4] = [2, 1, 3, 3];
pub(crate) static qtviande: [c_char; 4] = [1, 2, 4, 8];

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub(crate) enum ListeId {
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
    Listbidon,
    Zoneu,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub(crate) enum SapId {
    Sap1,
    Sap2,
    Sap3,
    Sap4,
}

pub(crate) struct Sapglob {
    /*définitions des variables globales*/
    pub(crate) lpos: c_uchar,
    pub(crate) stock1: c_uchar,
    pub(crate) stock2: c_uchar,
    pub(crate) criter: c_uchar,
    pub(crate) lastmen: c_uchar,
    pub(crate) rega: c_uchar,

    pub(crate) who: c_char,

    /* touches « homme », « femme », « oui » selon la langue (InitAndCheckTexts) */
    pub(crate) homme: c_uchar,
    pub(crate) femme: c_uchar,
    pub(crate) oui: c_uchar,

    pub(crate) dzeta: c_short,
    pub(crate) ldzeta: c_short,
    pub(crate) colaff: c_short,
    pub(crate) linaff: c_short,
    pub(crate) profond: [c_uchar; 5], /*objet prefere ?*/
    pub(crate) enetoi: [c_ushort; 4], /*enerv contre toi*/
    pub(crate) okmuse: TypeMuse,

    pub(crate) listdep: [c_uchar; 9],
    pub(crate) maxparam: [[c_uchar; 6]; 2],
    pub(crate) agrdep: [c_uchar; 4],
    pub(crate) tagress: [c_uchar; 4], /*agressivite*/

    /*donnees du menu en cours*/
    /**************************/
    pub(crate) menu: c_ushort,   /*menu en cours*/
    pub(crate) clin: c_ushort,   /*debut du cadre en ligne64*/
    pub(crate) ccol: c_ushort,   /*debut du cadre en colonne 64*/
    pub(crate) colcdr: c_ushort, /*nbre de colonnes40 du cadre*/
    pub(crate) cnlin: c_ushort,  /*nbre de lignes du cadre*/
    pub(crate) cncol: c_ushort,  /*nbre de colonnes du cadre*/

    pub(crate) poscur: c_short,  /*position curseur*/
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
    pub(crate) tlpos: [c_char; 9],
    pub(crate) tstdlist: [ListeId; 9], /* &liste0..&liste8, initialise par init_glob */
    /*Variables Globales*/
    pub(crate) distsp: c_short,
    pub(crate) presence: c_short,
    pub(crate) fin: c_uchar,
    pub(crate) sensmob: c_uchar,
    pub(crate) ptranim: *mut c_char,
    pub(crate) present: [c_char; 4],
    anjm1: [c_char; 48],
    anjm2: [c_char; 6],
    anjm3: [c_char; 4],
    anacr1: [c_char; 6],
    anacr2: [c_char; 4],
    angen1: [c_char; 6],
    angen2: [c_char; 4],
    anaga1: [c_char; 26],
    anaga2: [c_char; 14],
    anrga1: [c_char; 26],
    anrga2: [c_char; 14],
    anbras: [c_char; 31],
    anbra1: [c_char; 4],
    anbra2: [c_char; 4],
    anhac1: [c_char; 4],
    ansag1: [c_char; 4],
    anpoin: [c_char; 14],
    anhach: [c_char; 18],
    ansag: [c_char; 10],
    anbust: [c_char; 4],
    anbst1: [c_char; 16],
    antete: [c_char; 13],
    antet1: [c_char; 7],
    antet2: [c_char; 4],
    anarm1: [c_char; 20],
    anarm3: [c_char; 4],
    anarm4: [c_char; 4],
    pub(crate) aneutr: [c_char; 2],
    pub(crate) anlou1: [c_char; 4],
    pub(crate) anlou2: [c_char; 19],
    pub(crate) anlap1: [c_char; 7],
    pub(crate) anlap2: [c_char; 12],
    pub(crate) anlap3: [c_char; 13],
    /*blocks animation -- initialise par init_glob*/
    pub(crate) debban: [[*mut c_char; 14]; 13],
    pub(crate) sap1: sap,
    pub(crate) sap2: sap,
    pub(crate) sap3: sap,
    pub(crate) sap4: sap,
    /*--------------------------- EVENEMENTS ----------------------------*/
    pub(crate) destine: [evt; 40],
    pub(crate) posxf: c_short,
    pub(crate) posyf: c_short,

    pub(crate) soif: c_uchar,
    pub(crate) fatig: c_uchar,
    pub(crate) faim: c_uchar,
    pub(crate) mort: c_uchar,
    pub(crate) bless: c_uchar,
    pub(crate) echan: c_uchar,
    pub(crate) vlchan: c_uchar,
    pub(crate) flefba: c_uchar,
    pub(crate) predil: c_uchar,
    pub(crate) fdialo: c_uchar,
    pub(crate) fpress: c_uchar,
    pub(crate) chef: c_uchar,
    pub(crate) tribu: c_uchar,
    pub(crate) scenar: c_uchar,
    pub(crate) flcomb: c_uchar,
    pub(crate) agress: c_uchar,
    pub(crate) entvie: c_uchar,
    pub(crate) sexe: c_uchar,
    pub(crate) dquest: c_uchar,
    pub(crate) dervue: Vue,
    pub(crate) source: c_uchar,
    pub(crate) minute: c_uchar,
    pub(crate) sapvit: c_uchar,
    pub(crate) viecopain: c_uchar,
    pub(crate) sap3clav: c_uchar,
    pub(crate) cptagil: c_uchar,
    pub(crate) finjou: c_uchar,
    pub(crate) agilite: c_uchar,
    pub(crate) force: c_uchar,
    pub(crate) resistance: c_uchar,
    pub(crate) charisme: c_uchar,
    pub(crate) vie: c_uchar,
    pub(crate) flagbadi: c_uchar,
    pub(crate) ciel: c_uchar,
    pub(crate) terre: c_uchar,
    pub(crate) flgcharism: c_uchar,
    pub(crate) scenpr: c_char,

    pub(crate) enerv: c_ushort,
    pub(crate) xj: c_ushort,
    pub(crate) yj: c_ushort,
    pub(crate) zj: c_ushort,
    pub(crate) derxj: c_ushort,
    pub(crate) deryj: c_ushort,
    bidon: c_ushort,
    pub(crate) dismin: c_ushort,
    pub(crate) heure: c_ushort,
    pub(crate) metpos: c_ushort,
    pub(crate) angsol: c_ushort,
    pub(crate) jour_act: c_ushort,
    pub(crate) jour_dsk: c_ushort,

    pub(crate) zoom: c_uchar,
    tabclav1: [c_uchar; 7],
    tabclav2: [c_uchar; 7],
    pub(crate) param: [c_uchar; 6],

    pub(crate) soma: [cellule; 54], /* rempli dans init_glob */
    pub(crate) palhaute: *mut c_short,
    pub(crate) palbasse: *mut c_short,
    pub(crate) pal_chron: *mut c_short,
    pub(crate) ciel_degr: *mut c_short,
    pub(crate) pos_evt: c_short,

    pub(crate) tabheur: [h_chron; 37],
    pub(crate) sposx: c_short,
    pub(crate) sposy: c_short,
    pub(crate) dsposx: c_short,
    pub(crate) dsposy: c_short,
    pub(crate) dicone: c_short,
    pub(crate) s_menu: c_short,
    pub(crate) icone: c_short,
    pub(crate) bouton: c_short,
    pub(crate) s_action: c_short,
    pub(crate) smenu: [c_char; 2],
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
            ptranim: null_mut(),
            present: [0; 4],
            anjm1: [
                0x82u8 as c_char,
                0x81u8 as c_char,
                0,
                26,
                0,
                0x80u8 as c_char,
                -2,
                0x81u8 as c_char,
                1,
                26,
                0,
                0x81u8 as c_char,
                2,
                26,
                0,
                0x80u8 as c_char,
                -1,
                0x81u8 as c_char,
                3,
                26,
                0,
                0x81u8 as c_char,
                4,
                26,
                0,
                0x80u8 as c_char,
                0,
                /*anjm1b =anjm1+27*/
                0x81u8 as c_char,
                5,
                26,
                0,
                0x81u8 as c_char,
                6,
                26,
                0,
                0x81u8 as c_char,
                7,
                26,
                0,
                0x81u8 as c_char,
                8,
                26,
                0,
                0x81u8 as c_char,
                9,
                26,
                0,
                0xFFu8 as c_char,
            ],
            anjm2: [0x80u8 as c_char, 0, 0, 26, 0, 0xFFu8 as c_char],
            anjm3: [4, 26, 0, 0xFFu8 as c_char],
            anacr1: [0x80u8 as c_char, 2, 15, 27, 1, 0xFFu8 as c_char],
            anacr2: [16, 27, 2, 0xFFu8 as c_char],
            angen1: [0x80u8 as c_char, 13, 17, 19, 0, 0xFFu8 as c_char],
            angen2: [18, 24, 0, 0xFFu8 as c_char],
            anaga1: [
                15,
                27,
                1,
                0x80u8 as c_char,
                -1,
                8,
                26,
                0,
                0x81u8 as c_char,
                0x80u8 as c_char,
                -2,
                9,
                26,
                0,
                0x81u8 as c_char,
                0x80u8 as c_char,
                -1,
                0,
                26,
                0,
                0x80u8 as c_char,
                0,
                0x82u8 as c_char,
                0x86u8 as c_char,
                127,
                0xFFu8 as c_char,
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
                0x82u8 as c_char,
                0xFFu8 as c_char,
            ],
            anrga1: [
                15,
                27,
                1,
                0x80u8 as c_char,
                -1,
                0,
                26,
                0,
                0x85u8 as c_char,
                0x80u8 as c_char,
                -2,
                9,
                26,
                0,
                0x80u8 as c_char,
                -1,
                0x85u8 as c_char,
                8,
                26,
                0,
                0x80u8 as c_char,
                0,
                0x82u8 as c_char,
                0x86u8 as c_char,
                127,
                0xFFu8 as c_char,
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
                0x82u8 as c_char,
                0xFFu8 as c_char,
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
                0xFFu8 as c_char,
            ],
            anbra1: [34, 12, 0, 0xFFu8 as c_char],
            anbra2: [33, 12, 0, 0xFFu8 as c_char],
            anhac1: [38, 12, 0, 0xFFu8 as c_char],
            ansag1: [36, 9, 2, 0xFFu8 as c_char],
            anpoin: [
                34,
                12,
                0,
                107,
                12,
                -1,
                0x87u8 as c_char,
                3,
                37,
                12,
                1,
                0x86u8 as c_char,
                252u8 as c_char,
                0xFFu8 as c_char,
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
                0x86u8 as c_char,
                252u8 as c_char,
                0xFFu8 as c_char,
            ],
            ansag: [
                36,
                9,
                2,
                37,
                9,
                1,
                0x86u8 as c_char,
                252u8 as c_char,
                0x88u8 as c_char,
                0xFFu8 as c_char,
            ],
            anbust: [120, 12, 1, 0xFFu8 as c_char],
            anbst1: [
                0x80u8 as c_char,
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
                0x84u8 as c_char,
                0xFFu8 as c_char,
            ],
            antete: [53, 0, 2, 53, 0, 2, 54, 0, 2, 54, 0, 2, 0xFFu8 as c_char],
            antet1: [53, 0, 2, 54, 0, 2, 0xFFu8 as c_char],
            antet2: [53, 0, 2, 0xFFu8 as c_char],
            anarm1: [
                45,
                7,
                -2,
                47,
                8,
                -3,
                0x87u8 as c_char,
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
                0x86u8 as c_char,
                252u8 as c_char,
                0xFFu8 as c_char,
            ],
            anarm3: [45, 7, -2, 0xFFu8 as c_char],
            anarm4: [43, 11, -1, 0xFFu8 as c_char],
            aneutr: [0x83u8 as c_char, 0xFFu8 as c_char],
            anlou1: [96, 26, 0, 0xFFu8 as c_char],
            anlou2: [
                95,
                26,
                0,
                0x81u8 as c_char,
                96,
                26,
                0,
                0x81u8 as c_char,
                97,
                26,
                0,
                0x81u8 as c_char,
                98,
                26,
                0,
                0x81u8 as c_char,
                0x87u8 as c_char,
                1,
                0xFFu8 as c_char,
            ],
            anlap1: [87, 35, 0, 87, 35, 0, 0xFFu8 as c_char],
            anlap2: [
                87,
                35,
                0,
                0x81u8 as c_char,
                88,
                35,
                0,
                89,
                35,
                0,
                0x81u8 as c_char,
                0xFFu8 as c_char,
            ],
            anlap3: [90, 28, 2, 90, 28, 2, 90, 28, 2, 90, 28, 2, 0xFFu8 as c_char],
            debban: [[null_mut(); 14]; 13],
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
                sapanim: [null_mut(); 15],
                cross: [0; 8],
                adversaire: None,       /* &sap1, remis dans init_glob */
                possession: null_mut(), /* &liste1, remis dans init_glob */
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
                sapanim: [null_mut(); 15],
                cross: [0; 8],
                adversaire: None,       /* &sap2, remis dans init_glob */
                possession: null_mut(), /* &list10, remis dans init_glob */
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
                sapanim: [null_mut(); 15],
                cross: [0; 8],
                adversaire: None,       /* &sap2, remis dans init_glob */
                possession: null_mut(), /* &list11, remis dans init_glob */
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
                sapanim: [null_mut(); 15],
                cross: [0; 8],
                adversaire: None,       /* &sap2, remis dans init_glob */
                possession: null_mut(), /* &list12, remis dans init_glob */
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

            soma: [cellule {
                adresse: null_mut(),
                taille: 0,
            }; 54], /* rempli dans init_glob */
            palhaute: null_mut(),
            palbasse: null_mut(),
            pal_chron: null_mut(),
            ciel_degr: null_mut(),
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
const fn pal_s<const N: usize>(v: [c_short; N]) -> [c_short; 32] {
    let mut out = [0 as c_short; 32];
    let mut i = 0;
    while i < N {
        out[i] = v[i];
        i += 1;
    }
    out
}

fn hc(h: c_short, q: c_short, pal: [c_short; 32]) -> h_chron {
    h_chron {
        h,
        q,
        pal,
        TabCouleursCiel: [IndexColor::default(); NB_LIGNES_CIEL],
        TabPaletteDistance: null_mut(),
    }
}

/********************** END OF DATAS AND VARIABLES **************/

impl Sapiens {
    /// Pointeurs reliant les tables entre elles (debban, tstdlist, peuple,
    /// sapanim, adversaire, possession, soma). Equivalent des initialisateurs
    /// statiques auto-referents du C. A appeler avant sapiens_main.
    pub(crate) unsafe fn init_glob(&mut self) {
        unsafe {
            // macro prête à l'emploi
            macro_rules! ccp {
                ($x:expr) => {
                    ($x) as *mut c_char
                };
            }

            self.sapglob.debban = [
                /*marche*/
                [
                    ccp!(addr_of_mut!(self.sapglob.anjm1)),
                    ccp!(addr_of_mut!(self.sapglob.anjm1)),
                    ccp!(addr_of_mut!(self.sapglob.anbras)),
                    ccp!(addr_of_mut!(self.sapglob.anbras)),
                    ccp!(addr_of_mut!(self.sapglob.anjm1).add(27)),
                    ccp!(addr_of_mut!(self.sapglob.anjm1)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    ccp!(addr_of_mut!(self.sapglob.antete)),
                    ccp!(addr_of_mut!(self.sapglob.antete)),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anbras).add(15)),
                    ccp!(addr_of_mut!(self.sapglob.anbras)),
                ],
                /*arret*/
                [
                    ccp!(addr_of_mut!(self.sapglob.anjm2)),
                    ccp!(addr_of_mut!(self.sapglob.anjm2)),
                    ccp!(addr_of_mut!(self.sapglob.anbra1)),
                    ccp!(addr_of_mut!(self.sapglob.anbra1)),
                    ccp!(addr_of_mut!(self.sapglob.anjm3)),
                    ccp!(addr_of_mut!(self.sapglob.anjm3)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    ccp!(addr_of_mut!(self.sapglob.antet2)),
                    ccp!(addr_of_mut!(self.sapglob.antet2)),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anbra2)),
                    ccp!(addr_of_mut!(self.sapglob.anbra2)),
                ],
                /*frappe du poing*/
                [
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anpoin)),
                    null_mut(),
                ],
                /*frappe avec hache*/
                [
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anarm1)),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anhach)),
                    null_mut(),
                ],
                /*frappe avec sagaie*/
                [
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.ansag)),
                    ccp!(addr_of_mut!(self.sapglob.anbra2)),
                ],
                /*rien en main*/
                [
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    null_mut(),
                    null_mut(),
                ],
                /*hache en main*/
                [
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anarm3)),
                    ccp!(addr_of_mut!(self.sapglob.anarm3)),
                    ccp!(addr_of_mut!(self.sapglob.anhac1)),
                    ccp!(addr_of_mut!(self.sapglob.anhac1)),
                ],
                /*sagaie en main*/
                [
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anarm4)),
                    ccp!(addr_of_mut!(self.sapglob.anarm4)),
                    ccp!(addr_of_mut!(self.sapglob.ansag1)),
                    ccp!(addr_of_mut!(self.sapglob.ansag1)),
                ],
                /*accroupi*/
                [
                    ccp!(addr_of_mut!(self.sapglob.anacr1)),
                    ccp!(addr_of_mut!(self.sapglob.anacr1)),
                    ccp!(addr_of_mut!(self.sapglob.anbra1)),
                    ccp!(addr_of_mut!(self.sapglob.anbra1)),
                    ccp!(addr_of_mut!(self.sapglob.anacr2)),
                    ccp!(addr_of_mut!(self.sapglob.anacr2)),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.antet2)),
                    ccp!(addr_of_mut!(self.sapglob.antet2)),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anbra2)),
                    ccp!(addr_of_mut!(self.sapglob.anbra2)),
                ],
                /*a genoux*/
                [
                    ccp!(addr_of_mut!(self.sapglob.angen1)),
                    ccp!(addr_of_mut!(self.sapglob.angen1)),
                    ccp!(addr_of_mut!(self.sapglob.anbra1)),
                    ccp!(addr_of_mut!(self.sapglob.anbra1)),
                    ccp!(addr_of_mut!(self.sapglob.angen2)),
                    ccp!(addr_of_mut!(self.sapglob.angen2)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                ],
                /*plat ventre*/
                [
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.aneutr)),
                    ccp!(addr_of_mut!(self.sapglob.anbst1)),
                    ccp!(addr_of_mut!(self.sapglob.anbust)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                ],
                /*en garde avance*/
                [
                    ccp!(addr_of_mut!(self.sapglob.anaga1)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anaga2)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.antet1)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                ],
                /*en garde recule*/
                [
                    ccp!(addr_of_mut!(self.sapglob.anrga1)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.anrga2)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    ccp!(addr_of_mut!(self.sapglob.antet1)),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                ],
            ];

            for sap_id in [SapId::Sap1, SapId::Sap2, SapId::Sap3, SapId::Sap4] {
                for i in 0..14 {
                    let x = addr_of_mut!(self.sapglob.aneutr);
                    let s = self.sapglob.sap_mut(sap_id);
                    s.sapanim[i] = ccp!(x);
                }
                let s = self.sapglob.sap_mut(sap_id);
                s.sapanim[14] = null_mut();
            }

            self.sapglob.sap1.adversaire = Some(SapId::Sap1);
            self.sapglob.sap2.adversaire = Some(SapId::Sap2);
            self.sapglob.sap3.adversaire = Some(SapId::Sap2);
            self.sapglob.sap4.adversaire = Some(SapId::Sap2);
            self.sapglob.sap1.possession = addr_of_mut!(self.sapglob.liste1) as *mut liste;
            self.sapglob.sap2.possession = addr_of_mut!(self.sapglob.list10) as *mut liste;
            self.sapglob.sap3.possession = addr_of_mut!(self.sapglob.list11) as *mut liste;
            self.sapglob.sap4.possession = addr_of_mut!(self.sapglob.list12) as *mut liste;

            self.sapglob.soma = [
                cellule {
                    adresse: addr_of_mut!(self.sapglob.soif) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.fatig) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.faim) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.mort) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.bless) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.zoom) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.agilite) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.force) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.resistance) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.charisme) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.vie) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.sexe) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.viecopain) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.sap1.cross) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 8,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.sap3.cross) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 8,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.scenar) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.scenpr) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.ciel) as *mut _ as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.terre) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.liste1.taille) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.liste1.elements) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 30,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.tagress[0]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.enetoi[0]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.tagress[1]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.enetoi[1]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.tagress[2]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.enetoi[2]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.tagress[3]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.enetoi[3]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.xj) as *mut _ as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.yj) as *mut _ as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.dzeta) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.heure) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.sap3.vie) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.saptxt.nomjoueur) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 8,
                },
                cellule {
                    adresse: addr_of_mut!(self.saptxt.nomcopain) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 6,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.present[2]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.jour_act) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 2,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.param) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 6,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.tabclav1) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 7,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.tabclav2) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 7,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.liste8.elements[4]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.liste8.elements[5]) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapglob.bidon) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 1,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.FreqOrage) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.DistMinFoudroie) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.nbNuages) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.nbOiseaux) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.nbRochers) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.nbHommes) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.nbArbres) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sapcomb.nbHerbes) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: addr_of_mut!(self.sap3d.VitesseDeplacementNuages) as *mut _
                        as *mut std::os::raw::c_void,
                    taille: 4,
                },
                cellule {
                    adresse: null_mut(),
                    taille: 0,
                },
            ];
        }
    }
}
