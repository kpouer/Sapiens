/****************************Version 2. 02*******************************
                        VARIABLES GLOBALES
                Mercredi 10 Décembre 1986
***********************************************************************/
use crate::sapiens::sap3d::{
    DistMinFoudroie, FreqOrage, VitesseDeplacementNuages, nbArbres, nbHommes, nbNuages, nbOiseaux,
    nbRochers,
};
use crate::sapiens::sapcomb::nbHerbes;
use crate::sapiens::saptxt::{nomcopain, nomjoueur};
use crate::sapiens::{
    TypeMuse, TypeMuse_MUSIQUE_HARMONY_QUALITE, TypeReference_REFER0, TypeReference_REFER1,
    TypeReference_REFER2, TypeReference_REFER3, cellule, liste, sap,
};
use crate::sapiens::{evt, gemcadre, gemnbrecad, h_chron};
use std::os::raw::{c_char, c_short, c_uchar, c_ushort};
use std::ptr::{addr_of_mut, null_mut};
/*définitions des variables globales*/
pub static mut lpos: c_uchar = 1;
pub static mut stock1: c_uchar = 0;
pub static mut stock2: c_uchar = 0;
pub static mut criter: c_uchar = 0;
pub static mut lastmen: c_uchar = 255;
pub static mut rega: c_uchar = 0;

pub static mut who: c_char = 0;

pub static mut dzeta: c_short = 0;
pub static mut ldzeta: c_short = 0;
pub static mut colaff: c_short = 1;
pub static mut linaff: c_short = 1;
pub static mut type_synthe: c_short = 0;
pub static mut profond: [c_uchar; 5] = [10, 8, 6, 4, 2]; /*objet prefere ?*/
pub static mut tabprf: [c_uchar; 4] = [25, 33, 34, 35]; /*objet prefere*/
pub static mut tobje: [[c_uchar; 4]; 4] = [
    /*objets de la tribu*/
    [23, 26, 28, 38],
    [21, 25, 29, 39],
    [36, 27, 3, 23],
    [25, 24, 26, 23],
];

pub static mut tenerv: [c_ushort; 4] = [256, 4864, 25600, 40960]; /*enervement tribu -etranger*/
pub static mut enetoi: [c_ushort; 4] = [0, 0, 0, 0]; /*enerv contre toi*/

pub static mut okmuse: TypeMuse = TypeMuse_MUSIQUE_HARMONY_QUALITE;

pub static mut HOMME: c_uchar = b'H';
pub static mut FEMME: c_uchar = b'F';
pub static mut OUI: c_uchar = b'O';
pub static mut listdep: [c_uchar; 9] = [21, 23, 23, 27, 37, 36, 19, 19, 25];
pub static mut maxparam: [[c_uchar; 6]; 2] = [
    [240, 180, 254, 200, 130, 254],
    [200, 254, 130, 200, 200, 254],
];
pub static mut agrdep: [c_uchar; 4] = [110, 150, 200, 201];
pub static mut tagress: [c_uchar; 4] = [110, 150, 200, 201]; /*agressivite*/

/*donnees du menu en cours*/
/**************************/
pub static mut menu: c_ushort = 0; /*menu en cours*/
pub static mut clin: c_ushort = 1; /*debut du cadre en ligne64*/
pub static mut ccol: c_ushort = 1; /*debut du cadre en colonne 64*/
pub static mut colcdr: c_ushort = 10; /*nbre de colonnes40 du cadre*/
pub static mut cnlin: c_ushort = 10; /*nbre de lignes du cadre*/
pub static mut cncol: c_ushort = 10; /*nbre de colonnes du cadre*/
/*tprm est fixe � 2*/
pub static mut coulcur: c_ushort = 1; /*couleur d'affichage du curseur*/

pub static mut poscur: c_short = 0; /*position curseur*/
/**/
pub static mut stdlist: *mut liste = null_mut(); /*liste std pour ce cadre*/
/**/
pub static mut curpos: c_ushort = 0; /*position en memoire sur ecran du curseur*/
pub static mut pcadre: c_ushort = 0; /*position en memoire sur ecran du cadre*/
pub static mut coulfd: c_ushort = 0; /*couleur du fond du cadre*/
/*------- definition pour les gestions de cadre ------*/
pub static mut gemzoom: [gemcadre; gemnbrecad as usize] = [gemcadre {
    ligne: 0,
    colon: 0,
    haut: 0,
    larg: 0,
}; gemnbrecad as usize];

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

pub static mut liste0: liste = liste {
    typeRef: TypeReference_REFER0,
    taille: 11,
    maxtaille: 20,
    elements: elems([11, 10, 5, 4, 3, 6, 1, 2, 7, 8, 9]),
};
pub static mut liste1: liste = liste {
    typeRef: TypeReference_REFER1,
    taille: 16,
    maxtaille: 30,
    elements: elems([
        21, 23, 23, 27, 19, 19, 19, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ]),
};
pub static mut liste2: liste = liste {
    typeRef: TypeReference_REFER2,
    taille: 28,
    maxtaille: 0,
    elements: elems([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28,
    ]),
};
pub static mut liste3: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 4,
    maxtaille: 0,
    elements: elems([1, 2, 3, 4]),
};
pub static mut liste4: liste = liste {
    typeRef: TypeReference_REFER1,
    taille: 2,
    maxtaille: 30,
    elements: elems([2, 3, 4]),
};
pub static mut liste5: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 3,
    maxtaille: 0,
    elements: elems([5, 6, 7]),
};
pub static mut liste6: liste = liste {
    typeRef: TypeReference_REFER1,
    taille: 2,
    maxtaille: 0,
    elements: elems([23, 21]),
};
pub static mut liste7: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 5,
    maxtaille: 0,
    elements: elems([8, 9, 10, 11, 12]),
};
pub static mut liste8: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 5,
    maxtaille: 0,
    elements: elems([19, 13, 14, 15, 26]), /* Informations, Charge, Sauve, personnage, Pr�f�rences */
};
pub static mut list10: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 0,
    maxtaille: 20,
    elements: [0; 30],
};
pub static mut list11: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 0,
    maxtaille: 20,
    elements: [0; 30],
};
pub static mut list12: liste = liste {
    typeRef: TypeReference_REFER3,
    taille: 0,
    maxtaille: 20,
    elements: [0; 30],
};
pub static mut listbidon: liste = liste {
    typeRef: TypeReference_REFER0,
    taille: 0,
    maxtaille: 11,
    elements: [0; 30],
};
pub static mut zoneu: liste = liste {
    typeRef: TypeReference_REFER1,
    taille: 0,
    maxtaille: 30,
    elements: [0; 30],
}; /*liste de travail pour critere*/
pub static mut cross: liste = liste {
    typeRef: TypeReference_REFER1,
    taille: 0,
    maxtaille: 30,
    elements: [0; 30],
};

/***=======================================================================*/
/*              Inits des tableaux des menus                               */
pub static mut tclin: [c_char; 9] = [18, 19, 18, 18, 19, 18, 19, 20, 18];
pub static mut tccol: [c_char; 9] = [4, 22, 23, 28, 26, 31, 33, 23, 26];
pub static mut tcolcdr: [c_char; 9] = [11, 17, 17, 18, 12, 12, 12, 17, 13];
pub static mut tcnlin: [c_char; 9] = [7, 7, 8, 6, 6, 3, 2, 5, 6];
pub static mut tcncol: [c_char; 9] = [13, 16, 28, 12, 16, 16, 9, 26, 17];
pub static mut tlpos: [c_char; 9] = [1, 1, 1, 1, 1, 1, 1, 1, 1];
pub static mut tcoulcur: [c_char; 9] = [9, 9, 9, 9, 9, 9, 9, 9, 9];
pub static mut tposcur: [c_short; 9] = [6082, 5787, 6121, 5795, 6432, 6118, 6441, 6445, 5792];

pub static mut tstdlist: [*mut liste; 9] = [null_mut(); 9]; /* &liste0..&liste8, initialise par init_glob */

/*------ decouper ici----------*/
/*Variables Globales*/
pub static mut distsp: c_short = 0;
pub static mut presence: c_short = 0;
pub static mut fin: c_uchar = 0;
pub static mut sensmob: c_uchar = 0;
pub static mut nummob: c_uchar = 0;
pub static mut ptranim: *mut c_char = null_mut();

/*tableaux*/
pub static mut hautlin: [c_char; 4] = [10, 15, 25, 30];
pub static mut tabinp: [c_char; 4] = [3, 5, 20, 30];
pub static mut hautou: [c_char; 4] = [10, 20, 27, 35];
pub static mut tribforce: [c_char; 4] = [15, 15, 25, 20];
pub static mut tribvie: [c_char; 4] = [
    160u8 as c_char,
    150u8 as c_char,
    200u8 as c_char,
    180u8 as c_char,
];
pub static mut bestlin: [c_char; 4] = [2, 0xDDu8 as c_char, 2, 2];
pub static mut bestvie: [c_char; 4] = [70, 200u8 as c_char, 1, 1];
pub static mut bestagress: [c_char; 4] = [
    200u8 as c_char,
    180u8 as c_char,
    200u8 as c_char,
    180u8 as c_char,
];
pub static mut besthaut: [c_char; 4] = [2, 1, 3, 3];
pub static mut present: [c_char; 4] = [0, 0, 0, 0];
pub static mut qtviande: [c_char; 4] = [1, 2, 4, 8];

pub static mut anjm1: [c_char; 48] = [
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
];
pub static mut anjm2: [c_char; 6] = [0x80u8 as c_char, 0, 0, 26, 0, 0xFFu8 as c_char];
pub static mut anjm3: [c_char; 4] = [4, 26, 0, 0xFFu8 as c_char];
pub static mut anacr1: [c_char; 6] = [0x80u8 as c_char, 2, 15, 27, 1, 0xFFu8 as c_char];
pub static mut anacr2: [c_char; 4] = [16, 27, 2, 0xFFu8 as c_char];
pub static mut angen1: [c_char; 6] = [0x80u8 as c_char, 13, 17, 19, 0, 0xFFu8 as c_char];
pub static mut angen2: [c_char; 4] = [18, 24, 0, 0xFFu8 as c_char];
pub static mut anaga1: [c_char; 26] = [
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
];
pub static mut anaga2: [c_char; 14] = [
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
];
pub static mut anrga1: [c_char; 26] = [
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
];
pub static mut anrga2: [c_char; 14] = [
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
];
pub static mut anbras: [c_char; 31] = [
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
];
pub static mut anbra1: [c_char; 4] = [34, 12, 0, 0xFFu8 as c_char];
pub static mut anbra2: [c_char; 4] = [33, 12, 0, 0xFFu8 as c_char];
pub static mut anhac1: [c_char; 4] = [38, 12, 0, 0xFFu8 as c_char];
pub static mut ansag1: [c_char; 4] = [36, 9, 2, 0xFFu8 as c_char];
pub static mut anpoin: [c_char; 14] = [
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
];
pub static mut anhach: [c_char; 18] = [
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
];
pub static mut ansag: [c_char; 10] = [
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
];
pub static mut anbust: [c_char; 4] = [120, 12, 1, 0xFFu8 as c_char];
pub static mut anbst1: [c_char; 16] = [
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
];
pub static mut antete: [c_char; 13] = [53, 0, 2, 53, 0, 2, 54, 0, 2, 54, 0, 2, 0xFFu8 as c_char];
pub static mut antet1: [c_char; 7] = [53, 0, 2, 54, 0, 2, 0xFFu8 as c_char];
pub static mut antet2: [c_char; 4] = [53, 0, 2, 0xFFu8 as c_char];
pub static mut anarm1: [c_char; 20] = [
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
];
pub static mut anarm3: [c_char; 4] = [45, 7, -2, 0xFFu8 as c_char];
pub static mut anarm4: [c_char; 4] = [43, 11, -1, 0xFFu8 as c_char];
pub static mut aneutr: [c_char; 2] = [0x83u8 as c_char, 0xFFu8 as c_char];
pub static mut anlou1: [c_char; 4] = [96, 26, 0, 0xFFu8 as c_char];
pub static mut anlou2: [c_char; 19] = [
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
];
pub static mut anlap1: [c_char; 7] = [87, 35, 0, 87, 35, 0, 0xFFu8 as c_char];
pub static mut anlap2: [c_char; 12] = [
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
];
pub static mut anlap3: [c_char; 13] =
    [90, 28, 2, 90, 28, 2, 90, 28, 2, 90, 28, 2, 0xFFu8 as c_char];

/*blocks animation -- initialise par init_glob*/
pub static mut debban: [[*mut c_char; 14]; 13] = [[null_mut(); 14]; 13];

pub static mut sap1: sap = sap {
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
    adversaire: null_mut(), /* &sap1, remis dans init_glob */
    possession: null_mut(), /* &liste1, remis dans init_glob */
};

pub static mut sap2: sap = sap {
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
    adversaire: null_mut(), /* &sap2, remis dans init_glob */
    possession: null_mut(), /* &list10, remis dans init_glob */
};

pub static mut sap3: sap = sap {
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
    adversaire: null_mut(), /* &sap2, remis dans init_glob */
    possession: null_mut(), /* &list11, remis dans init_glob */
};

pub static mut sap4: sap = sap {
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
    adversaire: null_mut(), /* &sap2, remis dans init_glob */
    possession: null_mut(), /* &list12, remis dans init_glob */
};

/*--------------------------- EVENEMENTS ----------------------------*/
pub static mut destine: [evt; 40] = [
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
];

pub static mut posxf: c_short = 0;
pub static mut posyf: c_short = 0;
pub static mut ralent: c_short = 0;

pub static mut soif: c_uchar = 0;
pub static mut fatig: c_uchar = 0;
pub static mut faim: c_uchar = 0;
pub static mut mort: c_uchar = 0;
pub static mut bless: c_uchar = 0;
pub static mut echan: c_uchar = 0;
pub static mut vlchan: c_uchar = 0;
pub static mut flefba: c_uchar = 0;
pub static mut predil: c_uchar = 0;
pub static mut fdialo: c_uchar = 0;
pub static mut fpress: c_uchar = 0;
pub static mut chef: c_uchar = 0;
pub static mut tribu: c_uchar = 0;
pub static mut scenar: c_uchar = 0;
pub static mut flcomb: c_uchar = 0;
pub static mut agress: c_uchar = 0;
pub static mut entvie: c_uchar = 0;
pub static mut sexe: c_uchar = 1;
pub static mut dquest: c_uchar = 0;
pub static mut dervue: c_uchar = 0;
pub static mut valsoi: c_uchar = 0;
pub static mut source: c_uchar = 0;
pub static mut minute: c_uchar = 0;
pub static mut sapvit: c_uchar = 0;
pub static mut viecopain: c_uchar = 0;
pub static mut sap3clav: c_uchar = 0;
pub static mut cptsoi: c_uchar = 0;
pub static mut cptagil: c_uchar = 0;
pub static mut finjou: c_uchar = 0;
pub static mut agilite: c_uchar = 128;
pub static mut force: c_uchar = 128;
pub static mut resistance: c_uchar = 128;
pub static mut charisme: c_uchar = 128;
pub static mut vie: c_uchar = 128;
pub static mut flagbadi: c_uchar = 0;
pub static mut ciel: c_uchar = 221;
pub static mut terre: c_uchar = 136;
pub static mut freq: c_uchar = 10;
pub static mut flgcharism: c_uchar = 0;
pub static mut scenpr: c_char = 0;

pub static mut enerv: c_ushort = 0;
pub static mut xj: c_ushort = 0;
pub static mut yj: c_ushort = 0;
pub static mut zj: c_ushort = 0;
pub static mut derxj: c_ushort = 0;
pub static mut deryj: c_ushort = 0;
pub static mut bidon: c_ushort = 0;
pub static mut kk: c_ushort = 0;
pub static mut dismin: c_ushort = 1;
pub static mut dist: c_ushort = 1;
pub static mut heure: c_ushort = 1;
pub static mut metpos: c_ushort = 70;
pub static mut angsol: c_ushort = 0;
pub static mut jour_act: c_ushort = 0;
pub static mut jour_dsk: c_ushort = 0;

pub static mut zoom: c_uchar = 64;
pub static mut tabclav1: [c_uchar; 7] = [72, 80, 75, 77, 28, 57, 1];
pub static mut tabclav2: [c_uchar; 7] = [18, 45, 31, 32, 34, 33, 1];
pub static mut param: [c_uchar; 6] = [128, 128, 128, 128, 128, 0];

pub static mut peuple: [*mut sap; 4] = [null_mut(); 4]; /* &sap1..&sap4, remis dans init_glob */

pub static mut soma: [cellule; 54] = [cellule {
    adresse: null_mut(),
    taille: 0,
}; 54]; /* rempli dans init_glob */

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

const fn hc(h: c_short, q: c_short, pal: [c_short; 32]) -> h_chron {
    h_chron {
        h,
        q,
        pal,
        TabCouleursCiel: null_mut(),
        TabPaletteDistance: null_mut(),
    }
}

pub static mut tabheur: [h_chron; 37] = [
    hc(
        0,
        0,
        pal_s([
            0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641, 0x444, 0x030,
            0x040, 0x730, 0x230, 0x005,
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
            0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x730, 0x340, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x730, 0x340, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x740, 0x440, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x750, 0x440, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x760, 0x450, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x770, 0x440, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x760, 0x440, 0x005,
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
            0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x750, 0x340, 0x005,
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
            0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641, 0x444, 0x040,
            0x050, 0x740, 0x340, 0x005,
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
            0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641, 0x444, 0x020,
            0x030, 0x666, 0x230, 0x005,
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
            0x777, 0x000, 0x700, 0x221, 0x332, 0x443, 0x554, 0x420, 0x532, 0x641, 0x444, 0x030,
            0x040, 0x222, 0x230, 0x005,
        ]),
    ),
];

pub static mut palhaute: *mut c_short = null_mut();
pub static mut palbasse: *mut c_short = null_mut();
pub static mut pal_chron: *mut c_short = null_mut();
pub static mut ciel_degr: *mut c_short = null_mut();
pub static mut pos_evt: c_short = 0;

pub static mut sposx: c_short = 0;
pub static mut sposy: c_short = 0;
pub static mut dsposx: c_short = 0;
pub static mut dsposy: c_short = 0;
pub static mut dicone: c_short = 0;
pub static mut s_menu: c_short = 0;
pub static mut icone: c_short = 0;
pub static mut bouton: c_short = 0;
pub static mut handle: c_short = 0;
pub static mut s_action: c_short = 0;
pub static mut smenu: [c_char; 2] = [0, -1];

/********************** END OF DATAS AND VARIABLES **************/

/// Pointeurs reliant les tables entre elles (debban, tstdlist, peuple,
/// sapanim, adversaire, possession, soma). Equivalent des initialisateurs
/// statiques auto-referents du C. A appeler avant sapiens_main.
pub unsafe fn init_glob() {
    unsafe {
        // macro prête à l'emploi
        macro_rules! ccp {
            ($x:expr) => {
                ($x) as *mut c_char
            };
        }

        tstdlist = [
            addr_of_mut!(liste0) as *mut liste,
            addr_of_mut!(liste1) as *mut liste,
            addr_of_mut!(liste2) as *mut liste,
            addr_of_mut!(liste3) as *mut liste,
            addr_of_mut!(liste4) as *mut liste,
            addr_of_mut!(liste5) as *mut liste,
            addr_of_mut!(liste6) as *mut liste,
            addr_of_mut!(liste7) as *mut liste,
            addr_of_mut!(liste8) as *mut liste,
        ];

        debban = [
            /*marche*/
            [
                ccp!(addr_of_mut!(anjm1)),
                ccp!(addr_of_mut!(anjm1)),
                ccp!(addr_of_mut!(anbras)),
                ccp!(addr_of_mut!(anbras)),
                ccp!(addr_of_mut!(anjm1).add(27)),
                ccp!(addr_of_mut!(anjm1)),
                ccp!(addr_of_mut!(anbust)),
                ccp!(addr_of_mut!(anbust)),
                ccp!(addr_of_mut!(antete)),
                ccp!(addr_of_mut!(antete)),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(anbras).add(15)),
                ccp!(addr_of_mut!(anbras)),
            ],
            /*arret*/
            [
                ccp!(addr_of_mut!(anjm2)),
                ccp!(addr_of_mut!(anjm2)),
                ccp!(addr_of_mut!(anbra1)),
                ccp!(addr_of_mut!(anbra1)),
                ccp!(addr_of_mut!(anjm3)),
                ccp!(addr_of_mut!(anjm3)),
                ccp!(addr_of_mut!(anbust)),
                ccp!(addr_of_mut!(anbust)),
                ccp!(addr_of_mut!(antet2)),
                ccp!(addr_of_mut!(antet2)),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(anbra2)),
                ccp!(addr_of_mut!(anbra2)),
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
                ccp!(addr_of_mut!(aneutr)),
                null_mut(),
                ccp!(addr_of_mut!(anpoin)),
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
                ccp!(addr_of_mut!(anarm1)),
                null_mut(),
                ccp!(addr_of_mut!(anhach)),
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
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(ansag)),
                ccp!(addr_of_mut!(anbra2)),
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
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
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
                ccp!(addr_of_mut!(anarm3)),
                ccp!(addr_of_mut!(anarm3)),
                ccp!(addr_of_mut!(anhac1)),
                ccp!(addr_of_mut!(anhac1)),
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
                ccp!(addr_of_mut!(anarm4)),
                ccp!(addr_of_mut!(anarm4)),
                ccp!(addr_of_mut!(ansag1)),
                ccp!(addr_of_mut!(ansag1)),
            ],
            /*accroupi*/
            [
                ccp!(addr_of_mut!(anacr1)),
                ccp!(addr_of_mut!(anacr1)),
                ccp!(addr_of_mut!(anbra1)),
                ccp!(addr_of_mut!(anbra1)),
                ccp!(addr_of_mut!(anacr2)),
                ccp!(addr_of_mut!(anacr2)),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(antet2)),
                ccp!(addr_of_mut!(antet2)),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(anbra2)),
                ccp!(addr_of_mut!(anbra2)),
            ],
            /*a genoux*/
            [
                ccp!(addr_of_mut!(angen1)),
                ccp!(addr_of_mut!(angen1)),
                ccp!(addr_of_mut!(anbra1)),
                ccp!(addr_of_mut!(anbra1)),
                ccp!(addr_of_mut!(angen2)),
                ccp!(addr_of_mut!(angen2)),
                ccp!(addr_of_mut!(anbust)),
                ccp!(addr_of_mut!(anbust)),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            ],
            /*plat ventre*/
            [
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(aneutr)),
                ccp!(addr_of_mut!(anbst1)),
                ccp!(addr_of_mut!(anbust)),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            ],
            /*en garde avance*/
            [
                ccp!(addr_of_mut!(anaga1)),
                null_mut(),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(anaga2)),
                null_mut(),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(antet1)),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            ],
            /*en garde recule*/
            [
                ccp!(addr_of_mut!(anrga1)),
                null_mut(),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(anrga2)),
                null_mut(),
                null_mut(),
                null_mut(),
                ccp!(addr_of_mut!(antet1)),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            ],
        ];

        peuple = [
            addr_of_mut!(sap1) as *mut sap,
            addr_of_mut!(sap2) as *mut sap,
            addr_of_mut!(sap3) as *mut sap,
            addr_of_mut!(sap4) as *mut sap,
        ];

        for s in [
            addr_of_mut!(sap1),
            addr_of_mut!(sap2),
            addr_of_mut!(sap3),
            addr_of_mut!(sap4),
        ] {
            for i in 0..14 {
                (*s).sapanim[i] = ccp!(addr_of_mut!(aneutr));
            }
            (*s).sapanim[14] = null_mut();
        }

        sap1.adversaire = addr_of_mut!(sap1) as *mut sap;
        sap2.adversaire = addr_of_mut!(sap2) as *mut sap;
        sap3.adversaire = addr_of_mut!(sap2) as *mut sap;
        sap4.adversaire = addr_of_mut!(sap2) as *mut sap;
        sap1.possession = addr_of_mut!(liste1) as *mut liste;
        sap2.possession = addr_of_mut!(list10) as *mut liste;
        sap3.possession = addr_of_mut!(list11) as *mut liste;
        sap4.possession = addr_of_mut!(list12) as *mut liste;

        soma = [
            cellule {
                adresse: addr_of_mut!(soif) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(fatig) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(faim) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(mort) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(bless) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(zoom) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(agilite) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(force) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(resistance) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(charisme) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(vie) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(sexe) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(viecopain) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(sap1.cross) as *mut _ as *mut std::os::raw::c_void,
                taille: 8,
            },
            cellule {
                adresse: addr_of_mut!(sap3.cross) as *mut _ as *mut std::os::raw::c_void,
                taille: 8,
            },
            cellule {
                adresse: addr_of_mut!(scenar) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(scenpr) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(ciel) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(terre) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(liste1.taille) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(liste1.elements) as *mut _ as *mut std::os::raw::c_void,
                taille: 30,
            },
            cellule {
                adresse: addr_of_mut!(tagress[0]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(enetoi[0]) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(tagress[1]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(enetoi[1]) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(tagress[2]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(enetoi[2]) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(tagress[3]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(enetoi[3]) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(xj) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(yj) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(dzeta) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(heure) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(sap3.vie) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(nomjoueur) as *mut _ as *mut std::os::raw::c_void,
                taille: 8,
            },
            cellule {
                adresse: addr_of_mut!(nomcopain) as *mut _ as *mut std::os::raw::c_void,
                taille: 6,
            },
            cellule {
                adresse: addr_of_mut!(present[2]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(jour_act) as *mut _ as *mut std::os::raw::c_void,
                taille: 2,
            },
            cellule {
                adresse: addr_of_mut!(param) as *mut _ as *mut std::os::raw::c_void,
                taille: 6,
            },
            cellule {
                adresse: addr_of_mut!(tabclav1) as *mut _ as *mut std::os::raw::c_void,
                taille: 7,
            },
            cellule {
                adresse: addr_of_mut!(tabclav2) as *mut _ as *mut std::os::raw::c_void,
                taille: 7,
            },
            cellule {
                adresse: addr_of_mut!(liste8.elements[4]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(liste8.elements[5]) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(bidon) as *mut _ as *mut std::os::raw::c_void,
                taille: 1,
            },
            cellule {
                adresse: addr_of_mut!(FreqOrage) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(DistMinFoudroie) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(nbNuages) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(nbOiseaux) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(nbRochers) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(nbHommes) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(nbArbres) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(nbHerbes) as *mut _ as *mut std::os::raw::c_void,
                taille: 4,
            },
            cellule {
                adresse: addr_of_mut!(VitesseDeplacementNuages) as *mut _
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
