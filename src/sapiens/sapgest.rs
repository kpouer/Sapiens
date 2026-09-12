#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/***********************************************************************
                        Module  # 5 gestion et autres
                           D G le 30 Janvier 1987
***********************************************************************/
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, CtoPstr, DisposPtr, FAUX, ForeColor, InsetRect,
    Line, MoveTo, PtInRect, PtoCstr, RGBForeColor, SetPt, SetRect, TickCount, VRAI,
};
use crate::cbridge::{
    CInfoPBRec, blackColor, fclose, fopen, getc, putc, remove, sprintf, strcpy, stricmp, strlen,
};
use crate::langue::Langue::{ES, FR};
use crate::sapiens::sap3d::{
    ChargeEltsDecorsMobiles, CreeDecor3d, DefautDecor3d, InitDecor3d, SauveEltsDecorsMobiles,
    affich, nbArbres, nbNuages, nbOiseaux, nbRochers,
};
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapcomb::{InitGenerique, generique, mvtsap, nbHerbes, prm};
use crate::sapiens::sapenreg::IsEnreg;
use crate::sapiens::sapglob::{
    FEMME, HOMME, agilite, agrdep, bless, bouton, charisme, ciel_degr, dervue, destine, dsposx,
    dsposy, dzeta, enetoi, faim, fatig, fdialo, flefba, force, heure, jour_act, jour_dsk, listdep,
    liste1, liste4, maxparam, minute, mort, pal_chron, palhaute, param, pos_evt, posxf, posyf,
    presence, resistance, sap1, sap2, sap3, sap4, sapvit, scenar, scenpr, sexe, soif, soma, sposx,
    sposy, tagress, tenerv, vie, viecopain, xj, yj, zj, zoneu, zoom,
};
use crate::sapiens::saplist::{DoAide, ajoutl, inimen, pal2};
use crate::sapiens::saprel::{chrono, clrbadi, comment, efbadi, sceveu};
use crate::sapiens::saptxt::{GetApp, langue, nomjoueur};
use crate::sapiens::sapwbrt::{
    Affmob3d, Affmobbrt, CalculePaletteAttenuee, DoFrameRect, GetIndexPixBrt, PlotIndexPixBrt,
    TfrEcr, affomb, effima, effvirt, tfrdemi, tfrima, tfrvirt,
};
use crate::sapiens::sapwcar::{Gotoxy, affcarmess, motlst2};
use crate::sapiens::sapwclav::{
    FixeAireNoMouse, FixeAireNoMouseAireBasse, MontreSouris, aff_sour, eff_sour, getkey, litclav,
    reset_clav, sapiensTimer, vq_mouse,
};
use crate::sapiens::sapwcvt::CvtCouleurEnRgb;
use crate::sapiens::sapwind::{
    Collapse, ConfirmePerteAventure, Ecran1, FixeMenusComplet, FixeMenusReduit,
    FixeTailleMainWindow, Frame3d, GereInterface, MenuFenetres, Rnd, SapZoom, ValideUpdate, bande,
    boxf, debecr, initrnd, mob3D, mobAvplan, mobBambou, mobFleches, mobSilex, myRandom, rnd0,
};
use crate::sapiens::sapwmess::{TfrBadAss, efbadass};
use crate::sapiens::sapwmus::{attends_bruit, bruit, quel_synthe};
use crate::sapiens::sapwpref::DoPreferences;
use crate::sapiens::{
    ECHELLE, FORME_BUSTE, FORME_TETE, NB_ARBRES_MAX, NB_MAX_HERBE_AV_PLAN, NB_NUAGES_MAX,
    NB_OISEAUX_MAX, NB_ROCHERS_MAX, PRECISION_SILEX, VUE_AUCUNE,
};
use crate::shared::{Boolean, CGrafPtr, Point, RGBColor, Rect, SFReply, StringPtr};
use crate::stubs::{
    CarboniseInvalRect, DisposeRoutineDescriptor, PathApplication, PreservePath, RestorePath,
    SFGetFile, SFPutFile, SaisitChemin, SetPathApplication, SetVol, qd, volumeApplication,
};
use std::ffi::c_ushort;
use std::os::raw::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_void};
use std::ptr::{addr_of_mut, null_mut};

const NB_LIGNE_SILEX: c_short = 200;
const NB_COL_SILEX: c_short = 300;
const COULEUR_FOND_NUCLEUS: c_short = 0x000;
static forsy1: [c_uchar; 100] = [
    0, 0, 0, 0, 1, 1, 2, 3, 4, 4, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
    15, 15, 16, 16, 16, 16, 17, 17, 18, 18, 18, 18, 19, 19, 20, 20, 20, 20, 21, 21, 22, 22, 22, 22,
    22, 22, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 23, 23, 23, 23, 23, 23, 22, 22,
    22, 22, 21, 21, 20, 19, 18, 17, 16, 15, 14, 12, 10, 8, 6, 5, 3, 1, 0, 0, 0, 0, 0, 0,
];
static forsy2: [c_uchar; 100] = [
    0, 0, 0, 2, 4, 5, 6, 7, 8, 8, 8, 9, 10, 10, 11, 11, 12, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18,
    18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 28, 28, 29,
    29, 29, 29, 30, 30, 30, 30, 30, 30, 30, 30, 31, 31, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 31, 31, 30, 29, 28, 26, 24, 22, 20, 16, 12, 6, 0, 0, 0, 0, 0, 0,
];

static formonde: [c_uchar; 100] = [
    50, 50, 50, 50, 49, 49, 49, 49, 48, 48, 47, 47, 47, 46, 46, 46, 46, 45, 45, 44, 43, 43, 42, 42,
    41, 40, 39, 39, 38, 38, 37, 37, 36, 35, 34, 34, 33, 33, 32, 32, 31, 30, 29, 28, 27, 26, 25, 25,
    24, 24, 23, 23, 22, 21, 20, 20, 19, 19, 18, 18, 17, 17, 16, 16, 15, 15, 15, 15, 14, 14, 13, 13,
    12, 12, 11, 11, 10, 10, 9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0, 0, 0,
];

static mut forsyl: *const c_uchar = forsy1.as_ptr();

static mut AireJaugeForce: Rect = Rect {
    top: ((4 + 87 - 254 / 3) * ECHELLE) as i16,
    left: (40 * ECHELLE) as i16,
    bottom: ((4 + 87) * ECHELLE) as i16,
    right: (48 * ECHELLE) as i16,
};
const NB_COUL_SILEX: c_short = 8;

/* Correspondance entre epaisseur et couleur */
static epaiss: [c_ushort; 75] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5,
    5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 7,
];
static mut palSilex: [c_ushort; 8] = [0; 8];

/* Mobs du silex et curseurs (macros de SAPIENS.H, basees sur des globaux) */
const MOB_DESIGNE_BASE: c_short = 17; /* mobFleches+17 */
const MOB_HERBE_BASE: c_short = 1; /* mobAvplan+1 */
const MOB_SILEX_SAGAIE_BASE: c_short = 13; /* mobSilex+13 */
const MOB_SILEX_HACHE_BASE: c_short = 14; /* mobSilex+14 */
const MOB_SILEX_CASSE_BASE: c_short = 15; /* mobSilex+15 */
const MOB_DEHORS_BASE: c_short = 20; /* mobBambou+20 */
const MOB_FRAPPE_BASE: c_short = 21; /* mobBambou+21 */

const MOB_COURANT_BASE: c_short = 22; /* mobBambou+22 */
/* Traps de saisie (mots de passe de la version initiale) */

static TRAPS: [[c_uchar; 7]; 7] = [
    [b's', b'a', b'l', b'a', b'u', b'd', 0],
    [b'c', b'o', b'm', b'm', b'a', b'n', 0],
    [b'n', b'i', b'e', b'u', b'l', 0, 0],
    [b'c', b'l', b'a', b'i', b'r', 0, 0],
    [b'c', b'h', b'e', b'f', 0, 0, 0],
    [b'o', b'n', b'i', b'm', b'p', b'r', 0],
    [b'g', b'r', b'a', b'n', b'v', b'p', 0],
];

static mut AIRE_SUITE: Rect = Rect {
    top: 100 - 20,
    left: 320 - 16 - 32 - 8,
    bottom: 100 - 20 + 32,
    right: 320 - 16 - 8,
};

static mut TAB_COUL_SILEX_RGB: [RGBColor; 8] = [
    RGBColor {
        red: 0x0000,
        green: 0x0000,
        blue: 0x0000,
    },
    RGBColor {
        red: 0x3b00,
        green: 0x1b00,
        blue: 0x1200,
    },
    RGBColor {
        red: 0x6d00,
        green: 0x4900,
        blue: 0x0000,
    },
    RGBColor {
        red: 0x9200,
        green: 0x6d00,
        blue: 0x0000,
    },
    RGBColor {
        red: 0xB600,
        green: 0x9200,
        blue: 0x0000,
    },
    RGBColor {
        red: 0xDB00,
        green: 0x9200,
        blue: 0x2400,
    },
    RGBColor {
        red: 0xDB00,
        green: 0xB600,
        blue: 0x0000,
    },
    RGBColor {
        red: 0xFD00,
        green: 0xE700,
        blue: 0x0000,
    },
];

/**********************************************************************
                    plot_ecr
Ecrit un point coloré directement sur l'ecran
=> ligne,colonne ,couleur ST
***********************************************************************/
pub extern "C" fn plot_ecr(y: c_short, x: c_short, couleur: c_short) {
    let mut rgb = RGBColor::default();
    unsafe {
        CvtCouleurEnRgb(palhaute, couleur, &mut rgb);
    }
    RGBForeColor(&rgb);
    MoveTo(x * ECHELLE as c_short, y * ECHELLE as c_short);
    Line(ECHELLE as c_short, 0);
    ForeColor(blackColor as c_short);
}

/***********************************************************************
                    lit_sil
Lit un point coloré dans le silex
=> ligne,colonne
<= couleur 0-NB_COUL_SILEX
***********************************************************************/
fn lit_sil(y: c_short, x: c_short) -> c_short {
    unsafe {
        let index = GetIndexPixBrt(Ecran1, x + 106, y);
        let mut i: c_short = 0;
        while i < NB_COUL_SILEX {
            if palSilex[i as usize] as c_short == index {
                return i;
            }
            i += 1;
        }
        Collapse(c"[Sapgest.lit_sil.1] Color not found".as_ptr() as *mut c_char);
        0
    }
}

/**********************************************************************
                    ecrit_sil
Ecrit un point coloré dans le silex
=> ligne (0-400) ,colonne (0-640),couleur 0-NB_COUL_SILEX
***********************************************************************/
fn ecrit_sil(y: c_short, x: c_short, couleur: c_short) {
    unsafe {
        PlotIndexPixBrt(Ecran1, x + 106, y, palSilex[couleur as usize] as c_short);
    }
}

/***********************************************************************
                        CAT:Affiche le catalogue courant
***********************************************************************/
pub extern "C" fn cat() {}

/***********************************************************************
                TRAPSURE:Acces reserve aux initiés...
***********************************************************************/
pub extern "C" fn trapsure(quoi: c_short) {
    unsafe {
        flefba = 0;
        if quoi > 0 {
            comment(114 + quoi as c_ushort);
        }
        match quoi {
            0 => DEF3D(),
            /* Le bloc #ifdef OBSOLETE qui suivait n'est pas compile */
            1 => {
                xj = 22528;
                yj = 21504;
            }
            2 => sceveu(11),
            3 => sceveu(13),
            4 => {
                ajoutl(&raw mut liste1, 22);
                ajoutl(&raw mut liste1, 25);
                ajoutl(&raw mut liste1, 46);
            }
            5 => sceveu(12),
            6 => cat(),
            _ => {}
        }
    }
}

/***********************************************************************
                        TABDRT: Etablit la position droite
***********************************************************************/
fn tabdrt(pos: c_short) -> c_short {
    let mut i: c_short = NB_COL_SILEX + 1;
    loop {
        i -= 1;
        if lit_sil(pos, i) != 0 || i <= NB_COL_SILEX / 2 {
            return i;
        }
    }
}

/***********************************************************************
                        TABGAU: Etablit la position gauche
***********************************************************************/
fn tabgau(pos: c_short) -> c_short {
    let mut i: c_short = 0;
    loop {
        let courant = i;
        i += 1;
        if lit_sil(pos, courant) != 0 || i >= NB_COL_SILEX / 2 {
            return i;
        }
    }
}

/***********************************************************************
                        VALSYL:
Evalue l'erreur entre l'etat du nucleus et la forme theorique attendue
***********************************************************************/
fn valsyl() -> c_ushort {
    let mut i: c_short = 0;
    let mut ecart: c_uint = 0;
    while i < NB_LIGNE_SILEX {
        let f = unsafe { *forsyl.add((i / 2) as usize) } as c_short;
        let mut trav1 = (NB_COL_SILEX / 2) - 2 * f - tabgau(i); /*ecart par rapport à ma gauche*/
        if trav1 < 0 {
            trav1 = -trav1;
        }
        let mut trav2 = (NB_COL_SILEX / 2) + 2 * f - tabdrt(i); /*ecart à la droite*/
        if trav2 < 0 {
            trav2 = -trav2;
        }
        ecart = ecart.wrapping_add((trav1 + trav2) as c_uint);
        i += 2;
    }
    (ecart / 3) as c_ushort
}

/***********************************************************************
                        AfficheSuite
Affiche le mob suite
***********************************************************************/
fn AfficheSuite() {
    unsafe {
        affomb(mobFleches + 19, AIRE_SUITE.top, AIRE_SUITE.left, 2, 2, true);
    }
}

/***********************************************************************
                        MouseDansSuite
***********************************************************************/
fn MouseDansSuite(posx: c_short, posy: c_short) -> Boolean {
    let mut pt = Point::default();
    unsafe {
        SetPt(&mut pt, posx, posy);
        if PtInRect(pt, &raw const AIRE_SUITE) {
            return true;
        }
    }
    false
}

/***********************************************************************
                        SILEX: Affiche un silex
Affiche le nucleus et la forme ideale du silex par dessus
***********************************************************************/
fn TraceSilex() {
    let mut gbak = CGrafPtr::default();
    unsafe {
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
    }
    /* on efface la souris */
    eff_sour();
    /*dessin du noyau*/
    tfrvirt();
    /*dessin de la forme ideale a obtenir*/
    let mut i: c_short = 0;
    while i < NB_LIGNE_SILEX {
        let f = unsafe { *forsyl.add((i / 2) as usize) } as c_short;
        if f != 0 {
            plot_ecr(i / 2 + 1, 160 + f, 0x007);
            plot_ecr(i / 2 + 1, 160 - f, 0x007);
        }
        i += 4;
    }
    /* La jauge de force (AireJaugeForce) n'est plus tracee dans l'original */
    AfficheSuite();
    /* On transfere sur l'ecran */
    unsafe {
        TfrEcr(null_mut());
        AcamSetPortOffscreen(gbak);
    }
}

/***********************************************************************
                       FabriqueNucleus
Dessine un nucleus brut sur l'ecran 1
***********************************************************************/
fn FabriqueNucleus() {
    let mut y: c_short;
    let mut x: c_short;
    let mut delta1: c_short;
    let mut pos1: c_short;
    let mut pos2: c_short;
    let kk: c_short;
    let mut couleur: c_short;
    let mut posTheo: c_short;

    /* Palette du silex */
    unsafe { effima(COULEUR_FOND_NUCLEUS) };
    delta1 = 60;
    /* TIRAGE DU COEFF KK DE LA PIERRE DE 3 A 15 */
    kk = unsafe { myRandom(3, 16) } as c_short;
    /* Pour l'ensemble des lignes du silex */
    y = 0;
    while y < NB_LIGNE_SILEX {
        delta1 = delta1.wrapping_add(unsafe { myRandom(-1, 2) } as c_short);
        if delta1 < 0 {
            delta1 = 0;
        }
        posTheo = (unsafe { *forsyl.add((y / 2) as usize) } as c_short
            + unsafe { *forsyl.add((y / 2 + 1) as usize) } as c_short)
            / 2;
        pos1 = (NB_COL_SILEX / 2) - posTheo - delta1; /* position gauche du nucleus */
        delta1 = delta1.wrapping_add(unsafe { myRandom(-1, 2) } as c_short);
        if delta1 < 0 {
            delta1 = 0;
        }
        pos2 = (NB_COL_SILEX / 2) + posTheo + delta1; /* position droite du nucleus */
        /* securite : sorties */
        if pos1 < 0 {
            pos1 = 0;
        }
        if pos2 > NB_COL_SILEX {
            pos2 = NB_COL_SILEX;
        }
        /* pas n'est pas relu ensuite dans l'original : on ne le calcule
        que pour reproduire la sequence d'appels */
        let _pas = if pos1 == (NB_COL_SILEX / 2) {
            1
        } else {
            750 / ((NB_COL_SILEX / 2) - pos1)
        };
        x = pos1;
        while x <= (NB_COL_SILEX / 2) {
            let yc = if y < (NB_LIGNE_SILEX / 2) {
                y
            } else {
                NB_LIGNE_SILEX - y
            };
            couleur = 1 + ((x - pos1) * yc + unsafe { Rnd() } * kk) / 1000;
            if couleur >= NB_COUL_SILEX {
                couleur = NB_COUL_SILEX - 1;
            }
            ecrit_sil(y, x, couleur);
            x += 1;
        }
        let _pas = if pos2 == (NB_COL_SILEX / 2) {
            1
        } else {
            750 / (pos2 - (NB_COL_SILEX / 2))
        };
        x = pos2;
        while x > (NB_COL_SILEX / 2) {
            let yc = if y < (NB_LIGNE_SILEX / 2) {
                y
            } else {
                NB_LIGNE_SILEX - y
            };
            couleur = 1 + ((pos2 - x) * yc + unsafe { Rnd() } * kk) / 1000;
            if couleur >= NB_COUL_SILEX {
                couleur = NB_COUL_SILEX - 1;
            }
            ecrit_sil(y, x, couleur);
            x -= 1;
        }
        y += 1;
    }
}

/***********************************************************************
                       AppliqueNlleEpaiss
Applique une nouvelle epaisseur si la nouvelle est inferieure a l'ancienne
***********************************************************************/
fn AppliqueNlleEpaiss(y: c_short, x: c_short, epaiss_loc: c_short) {
    let ancEpaiss = lit_sil(y, x);
    if ancEpaiss < epaiss_loc || epaiss_loc == 0 {
        ecrit_sil(y, x, epaiss_loc);
    }
}

/***********************************************************************
                       AppliqueImpact

=> Position x et y du coup, force du coup
<= Bruit a jouer
***********************************************************************/
fn AppliqueImpact(mut posxf_l: c_short, mut posyf_l: c_short, forcef: c_short) -> c_short {
    let penet: c_short;
    let posimp: c_short;
    let mut onde: c_ushort;
    let mut yact: c_ushort;
    let mut calc: c_ushort;
    let distcb: c_ushort;
    let mut tfr: c_ushort;
    let mut cass: c_ushort = 0;
    let senson: c_ushort;
    let mut okbruit: c_ushort = 0;
    let mut trav1: c_ushort;
    let paslec: c_short;
    let mut distact: c_short;

    cass = 0;
    posxf_l *= ECHELLE as c_short;
    posyf_l *= ECHELLE as c_short;
    /* Le point d'impact est-il sur le nucleus si non on sort*/
    if (posxf_l > tabdrt(posyf_l)) || (posxf_l < tabgau(posyf_l)) {
        /* Dessin du silex */
        TraceSilex();
        return okbruit as c_short;
    }
    /* Sens de l'ondex : coup a gauche ou a droite */
    senson = (posxf_l < (NB_COL_SILEX / 2)) as c_ushort; /*vrai si gauche faux si droite*/
    /* Tfr varie de 0 à 15*/
    tfr = (forcef / 16) as c_ushort;
    /* Penet varie de 0 à 50 */
    penet = ((forcef * 50) / 256) as c_short;
    if senson != 0 {
        /*gauche*/
        tfr = tfr.wrapping_add(posxf_l as c_ushort); /*le coup porte vers l'interieur*/
        if tfr < penet as c_ushort {
            tfr = penet as c_ushort;
        }
        /*si on depasse le milieu ca casse*/
        if tfr > (NB_COL_SILEX / 2) as c_ushort {
            cass = cass.wrapping_add(1);
        }
        posimp = tfr as c_short - penet;
        /* Distance maximale */
        distcb = penet as c_ushort + (posimp - tabgau(posyf_l)) as c_ushort;
    } else {
        /*droite*/
        tfr = posxf_l as c_ushort - tfr;
        if tfr < (NB_COL_SILEX / 2) as c_ushort {
            cass = cass.wrapping_add(1); /*depasse le milieu casse*/
        }
        posimp = tfr as c_short + penet;
        distcb = penet as c_ushort - (posimp - tabdrt(posyf_l)) as c_ushort;
    }
    if (((forcef - 1) * 48 / 256) as c_ushort) > distcb {
        /*force suffisante*/
        if cass != 0 {
            /*c'est cassé...*/
            okbruit = 3;
            let mut a: c_short = 74;
            if posyf_l < NB_LIGNE_SILEX / 2 {
                /* On casse le haut du silex */
                let mut i: c_short = posyf_l + 4;
                while i >= 0 && i < NB_LIGNE_SILEX {
                    if a < 0 {
                        a = 0;
                    }
                    let mut j: c_short = 0;
                    while j < NB_COL_SILEX {
                        AppliqueNlleEpaiss(i, j, 0);
                        j += 1;
                    }
                    i -= 1;
                    a = a.wrapping_sub(8);
                }
            } else {
                let mut i: c_short = posyf_l - 5;
                while i >= 0 && i < NB_LIGNE_SILEX {
                    if a < 0 {
                        a = 0;
                    }
                    let mut j: c_short = 0;
                    while j < NB_COL_SILEX {
                        AppliqueNlleEpaiss(i, j, 0);
                        j += 1;
                    }
                    i += 1;
                    a = a.wrapping_sub(10);
                }
            }
        }
        /* Le silex n'est pas cassé , on taille un eclat*/
        else {
            paslec = (forcef ^ 255) * 8 + 128;
            yact = posyf_l as c_ushort;
            calc = posyf_l as c_ushort + 1;
            tfr = 0;
            onde = 0;
            trav1 = 0;
            while onde < 100 {
                /*on applique l'onde*/
                if yact < NB_LIGNE_SILEX as c_ushort {
                    let mut a: c_short = if senson != 0 {
                        /*gauche*/
                        (formonde[onde as usize] as c_short * forcef) / 256 + posimp
                    } else {
                        /*DROITE*/
                        posimp - (formonde[onde as usize] as c_short * forcef) / 256
                    };
                    if senson != 0 {
                        distact = a - tabgau(yact as c_short) as c_short;
                    } else {
                        distact = tabdrt(yact as c_short) as c_short - a;
                    }
                    if distact > 0 {
                        let mut pas: c_short = if senson != 0 {
                            7400 / (distact + 1)
                        } else {
                            7400 / distact
                        };
                        if pas == 0 {
                            pas = 1;
                        }
                        let mut i: c_short = 7400;
                        while i >= 0 {
                            AppliqueNlleEpaiss(
                                yact as c_short,
                                a,
                                epaiss[(i / 100) as usize] as c_short,
                            );
                            i -= pas;
                            if senson != 0 {
                                a -= 1;
                            } else {
                                a += 1;
                            }
                        }
                    }
                } /* si yact <100 */
                /* on fait le symetrique bas */
                if calc < NB_LIGNE_SILEX as c_ushort {
                    let mut a: c_short = if senson != 0 {
                        /* gauche */
                        (formonde[onde as usize] as c_short * forcef) / 256 + posimp
                    } else {
                        /* droite */
                        posimp - (formonde[onde as usize] as c_short * forcef) / 256
                    };
                    if senson != 0 {
                        distact = a - tabgau(calc as c_short) as c_short;
                    } else {
                        distact = tabdrt(calc as c_short) as c_short - a;
                    }
                    if distact > 0 {
                        let mut pas: c_short = if senson != 0 {
                            7400 / (distact + 1)
                        } else {
                            7400 / distact
                        };
                        if pas == 0 {
                            pas = 1;
                        }
                        let mut i: c_short = 7400;
                        while i >= 0 {
                            AppliqueNlleEpaiss(
                                calc as c_short,
                                a,
                                epaiss[(i / 100) as usize] as c_short,
                            );
                            i -= pas;
                            if senson != 0 {
                                a -= 1;
                            } else {
                                a += 1;
                            }
                        }
                    }
                } /* fin de calc <100*/
                /* on passe a la ligne suivante */
                calc = calc.wrapping_add(1);
                yact = yact.wrapping_sub(1); /*ligne suiv haute*/
                trav1 = trav1.wrapping_add((paslec & 255) as c_ushort);
                /*plus retenue*/
                onde = onde
                    .wrapping_add((trav1 > 255) as c_ushort)
                    .wrapping_add((paslec / 256) as c_ushort);
                trav1 &= 255; /*on garde que le PL*/
            } /*balaye le silex*/
            okbruit = 4; /*aigu*/
        } /*si pas casse*/
    }
    /*si force suffisante*/
    else {
        /* La force est insuffisante pour attaquer le nucleus */
        okbruit = 5; /*bruit sourd*/
    }
    /* Dessin du silex */
    TraceSilex();
    okbruit as c_short
}

/***********************************************************************
                        CURSIL:Taille le silex
=> Silex a tailler
<= Collapse sur le silex
***********************************************************************/
fn cursil(regb: c_short) -> c_short {
    let mut i: c_short;
    let mut r = Rect::default();
    let mut gbak = CGrafPtr::default();

    unsafe {
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
    }
    /* On calcule les index de couleurs */
    unsafe {
        i = 0;
        while i < NB_COUL_SILEX {
            palSilex[i as usize] =
                SapColor2Index(&raw mut TAB_COUL_SILEX_RGB[i as usize]) as c_ushort;
            i += 1;
        }
    }
    unsafe { AcamSetPortOffscreen(gbak) };
    /* On fixe l'ecart stochastique gauche et droite */
    unsafe {
        forsyl = if regb == 0 {
            forsy1.as_ptr()
        } else {
            forsy2.as_ptr()
        };
    }

    /* Effacement du centre : on remplit avec une pierre noire */
    FabriqueNucleus();
    unsafe { flefba = 0 };

    /* on l'affiche tel quel */
    /* Mise en place des bords */
    unsafe {
        bande(1);
        flefba = 0;
    }
    TraceSilex();
    unsafe {
        SetRect(
            &mut r,
            16 * ECHELLE as c_short,
            0 * ECHELLE as c_short,
            (320 - 16) * ECHELLE as c_short,
            200 * ECHELLE as c_short,
        );
        FixeAireNoMouse(&mut r);
    }

    loop {
        unsafe {
            loop {
                vq_mouse(&raw mut bouton, &raw mut posxf, &raw mut posyf);
                if bouton == 0 {
                    break;
                }
            }
        }
        let mut okbruit: c_uchar = 0;
        unsafe {
            loop {
                vq_mouse(&raw mut bouton, &raw mut posxf, &raw mut posyf);
                if (posxf < 32) || (posxf > 320) || (posyf > 100) || (posyf < 2) {
                    if posyf > 100 {
                        posyf = 100;
                    }
                    if posxf > 320 - 16 {
                        posxf = 320 - 16;
                    }
                    aff_sour(mobBambou + MOB_DEHORS_BASE, posyf, posxf);
                    bouton &= 2;
                } else if MouseDansSuite(posxf, posyf) {
                    aff_sour(mobBambou + MOB_FRAPPE_BASE, posyf, posxf);
                } else {
                    aff_sour(mobBambou + MOB_COURANT_BASE, posyf - 25, posxf);
                }
                if bouton == 0 {
                    break;
                }
            }
            /* On valide le silex */
            if MouseDansSuite(posxf, posyf) {
                affomb(mobFleches + 20, AIRE_SUITE.top, AIRE_SUITE.left, 2, 2, true);
                FixeAireNoMouseAireBasse();
                return valsyl() as c_short;
            }
            /* Point d'application */
            plot_ecr(posyf, posxf, 0x700);
            if (bouton & 1) != 0 {
                /*on frappe*/
                let t: u64 = TickCount() as u64;
                /* On determine la force du coup */
                let mut forcef: c_short = 0;
                loop {
                    let mut bidon: c_short = 0;
                    vq_mouse(&raw mut bouton, &raw mut bidon, &raw mut bidon);
                    forcef = ((TickCount() as u64).wrapping_sub(t) * 4) as c_short;
                    aff_sour(
                        mobBambou + MOB_COURANT_BASE,
                        posyf - 25 - forcef / 8 - 2,
                        posxf + forcef / 8 + 2,
                    );
                    if bouton == 0 || forcef >= 254 {
                        break;
                    }
                }
                if forcef > 254 {
                    forcef = 254;
                }
                i = forcef;
                while i >= 0 {
                    aff_sour(
                        mobBambou + MOB_COURANT_BASE,
                        posyf - 25 - i / 8,
                        posxf + i / 8,
                    );
                    i -= 32;
                }
                /* On applique l'impact : conversion de coordonnees */
                okbruit = AppliqueImpact(posxf - 85, posyf - 1, forcef) as c_uchar;
                aff_sour(mobBambou + MOB_COURANT_BASE, posyf - 25, posxf);
                /* On joue le bruit */
                if okbruit != 0 {
                    bruit(okbruit as c_short);
                }
            } /*on frappe*/
        }
    }
}

/***********************************************************************
                        GETNOM:Saisi un nom de la taille donnée
***********************************************************************/
fn getnom(mess: c_short, position: *mut c_char, taille: c_short) -> c_short {
    let mut pos: c_short = 0;
    let mut car: c_short;

    unsafe {
        while litclav() != 0 {
            //Purge
        }
        let mut i: c_short = 0;
        while i < taille {
            zoneu.elements[i as usize] = 32; /*on vide la zone*/
            i += 1;
        }
        loop {
            /* On affiche le commentaire */
            efbadi();
            comment(mess as c_ushort);
            let mut i: c_short = 0;
            while i < taille && zoneu.elements[i as usize] != b' ' {
                affcarmess(zoneu.elements[i as usize] as c_short);
                i += 1;
            }
            affcarmess(2); /* Curseur */
            TfrBadAss();
            loop {
                car = litclav();
                if car != 0 {
                    break;
                }
            }
            if car == 13 {
                /* Validation : test des trapsures */
                for t in 0..7usize {
                    let mut oktrap: c_short = 1;
                    for j in 0..6usize {
                        if (TRAPS[t][j] & 223) != (zoneu.elements[j] & 223) {
                            oktrap = 0;
                        }
                    }
                    if oktrap != 0 {
                        trapsure(t as c_short);
                        pos = 0;
                    }
                }
                if pos != 0 {
                    /*au moins un caractere*/
                    let mut p = position;
                    let mut i: c_short = 0;
                    while i < taille {
                        *p = zoneu.elements[i as usize] as c_char;
                        p = p.add(1);
                        i += 1;
                    }
                }
                efbadass();
                TfrBadAss();
                return pos;
            }
            if car > 32
                && car <= b'z' as c_short
                && !(car >= b'0' as c_short && car <= b'9' as c_short)
            {
                if pos < taille {
                    zoneu.elements[pos as usize] = car as c_uchar;
                    pos += 1;
                }
            } else if car == 8 && pos != 0 {
                pos -= 1;
                zoneu.elements[pos as usize] = 32;
            }
        }
    }
}

/***********************************************************************
                        DebutAventure
Initialise decor 3d, et afiche le copyright
***********************************************************************/
pub extern "C" fn DebutAventure(isFirst: Boolean) {
    unsafe {
        FixeMenusReduit();
        primsap();
        generique(FAUX);
        if isFirst {
            DefautDecor3d();
        } else {
            InitDecor3d();
        }
        CreeDecor3d(false);
        effvirt(0x000);
        effima(0x000);
        /*Presentation*/
        heure = 10;
        chrono(0);
        affich(dzeta as c_ushort); /* Affiche la vue trois d sur ecran 1*/
        tfrvirt();
        flefba = 1;
        InitGenerique();
        generique(FAUX);
        TfrEcr(std::ptr::null_mut());
        Frame3d((FAUX) as c_short);
        efbadi();
        comment(67); /*d et o guillion*/
        for mut i in 0..60 {
            let posTimer = sapiensTimer;
            chrono(1);
            chrono(0);
            chrono(0);
            affich(dzeta as c_ushort);
            tfrima();
            if (getkey(0) & (32 + 16)) > 0 {
                i = 111;
            }
            flefba = 0;
            if i == 25 {
                comment(68);
            }
            if i == 50 {
                comment(69);
            }
            /* Synchronisation de la trame */
            while posTimer + 30 >= sapiensTimer {}
        }
        comment(70);
        FixeMenusComplet();
        MenuFenetres(if VRAI { 1 } else { 0 });
        efbadi();
        comment(132);
    }
}

/***********************************************************************
                        PRIMSAP:definition primaires des saps
***********************************************************************/
pub fn primsap() {
    unsafe {
        scenar = 1;
        jour_act = 1;
        scenpr = 20;
        fdialo = 0;
        pos_evt = 0;
        presence = 0;
        viecopain = 0;
        jour_dsk = 0;
        dervue = VUE_AUCUNE as c_uchar;
        prm(addr_of_mut!(sap2), 0);
        prm(addr_of_mut!(sap3), 0);
        prm(addr_of_mut!(sap4), 0);
        /*quotas et positions*/
        mort = 4;
        faim = 255;
        soif = 255;
        fatig = 255;
        bless = 255;
        xj = 0x5800;
        yj = 0x5400;
        dzeta = 768;
        zoom = 120;
        heure = 1;
        zj = 40;
        /*objets portes*/
        liste1.taille = 9;
        for i in 0..9 {
            liste1.elements[i] = listdep[i];
        }
        if IsEnreg() {
            liste1.elements[0] = 22; /*Massue a la place de hache */
        }
        for i in 0..4 {
            enetoi[i] = tenerv[i];
            tagress[i] = agrdep[i];
        }
        destine[0].type_ = 1; /*regenere les 4 chefs*/
        destine[1].type_ = 1;
        destine[2].type_ = 1;
        destine[3].type_ = 1;
        liste4.taille = 0;
    }
}

/***********************************************************************
                        AireJauge
Calcule l'aire ecran totale d'une jauge
***********************************************************************/
pub extern "C" fn AireJauge(i: c_short, r: *mut Rect) {
    unsafe {
        SetRect(
            r,
            114,
            6 + 14 * i,
            (114 + maxparam[sexe as u8 as usize][i as usize] / 2 + 2) as i16,
            16 + 14 * i,
        );
    }
}

/***********************************************************************
                        AireValJauge
Calcule l'aire ecran de la valeur interne d'une jauge
***********************************************************************/
pub extern "C" fn AireValJauge(i: c_short, r: *mut Rect) {
    unsafe {
        SetRect(
            r,
            115,
            7 + 14 * i,
            (115 + param[i as u8 as usize] / 2) as i16,
            15 + 14 * i,
        );
    }
}

/***********************************************************************
                        AfficheCadreJauge
***********************************************************************/
pub extern "C" fn AfficheCadreJauge(i: c_short) {
    let mut r = Rect::default();

    AireJauge(i, &mut r);
    ForeColor(blackColor as i16);
    r.left *= ECHELLE as i16;
    r.right *= ECHELLE as i16;
    r.top *= ECHELLE as i16;
    r.bottom *= ECHELLE as i16;
    DoFrameRect(&mut r);
}

/***********************************************************************
                        JAUGE:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
pub extern "C" fn jauge(i: c_short, tfr: Boolean) {
    let mut rval = Rect::default();
    let mut r = Rect::default();
    let mut gbak = CGrafPtr::default();

    AcamGetPortOffscreen(&mut gbak);
    unsafe {
        AcamSetPortOffscreen(debecr);
        AireValJauge(i, &mut rval);
        AfficheCadreJauge(i);
        boxf(rval.top, rval.left, rval.bottom, rval.right, 2);
        /* Effacement de la partie droite */
        AireJauge(i, &mut r);
        if param[i as usize] / 2 < maxparam[sexe as u8 as usize][i as usize] / 2 {
            boxf(r.top + 1, rval.right, r.bottom - 1, r.right - 1, 14);
            if (param[i as usize]) > 0 {
                boxf(r.bottom - 2, r.left + 1, r.bottom - 1, rval.right - 1, 9);
                boxf(r.top + 1, rval.right - 1, r.bottom - 1, rval.right, 9);
            }
        }
        AcamSetPortOffscreen(gbak);
        if tfr {
            r.left *= ECHELLE as i16;
            r.right *= ECHELLE as i16;
            r.top *= ECHELLE as i16;
            r.bottom *= ECHELLE as i16;
            TfrEcr(&mut r);
        }
    }
}

/***********************************************************************
                        PRIMJAUGE:primaire pour les jauges
***********************************************************************/
pub extern "C" fn primjauge(jaugeCredit: c_short) {
    unsafe {
        let app = GetApp();
        dervue = VUE_AUCUNE as c_uchar;
        effima(0x050); /* Efface ecran sur 100 ligne */
        tfrvirt();
        /* Affichage sur Ecran */
        Gotoxy(6, 1);
        motlst2(*app.add(0));
        Gotoxy(6, 3);
        motlst2(*app.add(1));
        Gotoxy(6, 5);
        motlst2(*app.add(2));
        Gotoxy(6, 7);
        motlst2(*app.add(3));
        Gotoxy(6, 9);
        motlst2(*app.add(4));
        if jaugeCredit != 0 {
            Gotoxy(6, 11);
            motlst2(*app.add(5));
        }

        /*trace des cadres jauges*/
        let mut i: c_short = 0;
        while i < 5 + jaugeCredit {
            jauge(i, false);
            i += 1;
        }
        TfrEcr(null_mut());
    }
}

/***********************************************************************
                        DEFSAP: Defini un sapiens
***********************************************************************/
pub extern "C" fn defsap() {
    unsafe {
        let mut r = Rect::default();
        let mut rval = Rect::default();
        let mut pt = Point::default();

        sapvit = 0;
        let taille = getnom(88, &raw mut nomjoueur as *mut c_char, 8);
        if taille == 0 {
            return;
        }
        flefba = 0;
        *(&raw mut nomjoueur as *mut c_char).add(taille as usize) = 0;
        /* definitions des quotas personnage*/
        for i in 0..5usize {
            param[i] = 130;
        }
        param[5] = 20;
        generique(VRAI);
        comment(89); /*   XX ou XY ???*/
        let mut car: c_uchar = 0;
        reset_clav();
        while car != HOMME && car != FEMME {
            car = (litclav() & 223) as c_uchar;
        }
        affcarmess(car as c_short);
        TfrBadAss();
        if car == HOMME {
            sexe = 1;
            sap1.cross[FORME_BUSTE as usize] = 0;
            sap1.cross[FORME_TETE as usize] = 0;
        } else {
            sexe = 0;
            sap1.cross[FORME_BUSTE as usize] = 8;
            sap1.cross[FORME_TETE as usize] = 12;
        }
        pal_chron = palhaute;
        palhaute = &raw const pal2 as *mut c_short;
        primjauge(1);
        AfficheSuite();
        let mut cont: c_char = 1;
        let mut pos: c_char = 0;
        sposx = 160;
        dsposx = 160;
        sposy = 50;
        dsposy = 50;
        SetRect(
            &mut r,
            16 * ECHELLE as c_short,
            0 * ECHELLE as c_short,
            (320 - 16) * ECHELLE as c_short,
            200 * ECHELLE as c_short,
        );
        FixeAireNoMouse(&mut r);

        /* Selection des Kotas de force */
        while cont != 0 {
            loop {
                vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                aff_sour(mobBambou + 21, sposy, sposx);
                if bouton == 0 {
                    break;
                }
            }

            if MouseDansSuite(sposx, sposy) {
                affomb(mobFleches + 20, AIRE_SUITE.top, AIRE_SUITE.left, 2, 2, true);
                cont = 0; /*esc*/
                if param[0] < 223 {
                    agilite = param[0] + 32;
                } else {
                    agilite = 255;
                }
                force = param[1];
                charisme = param[2];
                vie = param[3];
                resistance = param[4];
                while getkey(0) != 0 { /* Epuise le clavier */ }
            } else {
                SetPt(&mut pt, sposx, sposy);
                let mut i: c_short = 0;
                while i < 5 {
                    AireJauge(i, &mut r);
                    if PtInRect(pt, &raw const r) {
                        pos = i as c_char;
                        /* Fleche */
                        affomb(mobFleches + MOB_DESIGNE_BASE, r.top - 4, 86, 0, 0, true);
                        loop {
                            AireValJauge(pos as c_short, &mut rval);
                            /* On augmente la jauge */
                            if sposx > rval.right
                                && param[5] != 0
                                && param[pos as usize] < maxparam[sexe as usize][pos as usize]
                            {
                                param[5] -= 1;
                                param[pos as usize] += 1;
                            }
                            if sposx < rval.right && param[5] < 254 && param[pos as usize] != 0 {
                                param[5] += 1;
                                param[pos as usize] -= 1;
                            }
                            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                            jauge(pos as c_short, true);
                            /* Quantite restante */
                            jauge(5, true);
                            aff_sour(mobBambou + MOB_FRAPPE_BASE, sposy, sposx);
                            if bouton == 0 {
                                break;
                            }
                        } /* while(bouton) */
                        primjauge(1);
                        AfficheSuite();
                        i = 255;
                    }
                    i += 1;
                }
            }
        }
        /*selection de l'apparence*/
        eff_sour();
        cont = 1;
        pos = 0;
        prm(&raw mut sap1, -1);
        effima(0x050);
        tfrvirt();
        tfrima();
        flefba = 0;
        eff_sour();
        while cont != 0 {
            sap1.colonne = 28;
            sap1.sens = 1;
            effima(0x050); //??
            mvtsap(&raw mut sap1);
            tfrdemi();
            tfrvirt();
            AfficheSuite();

            affomb(
                mobFleches + MOB_DESIGNE_BASE,
                (pos as c_short * 2 + 6) * 7 + 1,
                100,
                4,
                4,
                VRAI,
            );
            loop {
                vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                aff_sour(mobBambou + 21, sposy, sposx);
                if bouton == 0 {
                    break;
                }
            }

            if MouseDansSuite(sposx, sposy) {
                affomb(mobFleches + 20, AIRE_SUITE.top, AIRE_SUITE.left, 2, 2, true);
                cont = 0; /*esc*/
            } else {
                pos = (sposy > 53) as c_char;
                dsposy = sposy;
                dsposx = sposx;
                if (bouton & 1) != 0 {
                    /*gauche*/
                    if pos != 0 {
                        sap1.cross[FORME_BUSTE as usize] =
                            sap1.cross[FORME_BUSTE as usize].wrapping_add(2); /*Buste*/
                        if sexe != 0 {
                            if sap1.cross[FORME_BUSTE as usize] >= 8 {
                                sap1.cross[FORME_BUSTE as usize] = 0;
                            }
                        } else if sap1.cross[FORME_BUSTE as usize] >= 16 {
                            sap1.cross[FORME_BUSTE as usize] = 8;
                        }
                    } else {
                        sap1.cross[FORME_TETE as usize] =
                            sap1.cross[FORME_TETE as usize].wrapping_add(2); /*tete*/
                        if sexe != 0 {
                            if sap1.cross[FORME_TETE as usize] > 6 {
                                sap1.cross[FORME_TETE as usize] = 0;
                            }
                        } else if sap1.cross[FORME_TETE as usize] > 22 {
                            sap1.cross[FORME_TETE as usize] = 12;
                        }
                    }
                    loop {
                        vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                        if (bouton & 1) == 0 {
                            break;
                        }
                    }
                }
            }
        }
        eff_sour();
        /* On est enregistre le personnage a une massue */
        palhaute = pal_chron;
        DebutAventure(false);
        FixeAireNoMouseAireBasse();
    }
}

/***********************************************************************
                        AIREJAUGE3d:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
const CAR_UTILE: c_short = 180;

fn AIREJAUGE3d(i: c_short, r: *mut Rect) {
    unsafe {
        SetRect(r, 75, 7 + 14 * i, 75 + CAR_UTILE, 15 + 14 * i);
    }
}

/***********************************************************************
                        JAUGE3d:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
const NB_JAUGES_3D: c_short = 5;

fn JAUGE3d(i: c_short, val: f32) {
    let mut r = Rect::default();

    unsafe {
        AIREJAUGE3d(i, &mut r);
        boxf(r.top - 1, r.left - 1, r.bottom + 1, r.right + 1, 16);
        boxf(
            r.top,
            r.left,
            r.bottom,
            r.left + (CAR_UTILE as f32 * val) as c_short,
            14,
        );
        boxf(
            r.top,
            r.left + (CAR_UTILE as f32 * val) as c_short,
            r.bottom,
            r.right,
            4,
        );
    }
}

/***********************************************************************
                        JAUGES3D
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
fn JAUGES3D() {
    unsafe {
        let mut i: c_short = 0;
        let mut gbak = CGrafPtr::default();
        let mut r = Rect::default();

        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
        boxf(0, 32, 101, 287, 5);

        macro_rules! aire_zoom {
            ($i:expr) => {{
                AIREJAUGE3d($i, &mut r);
                let (l, t, ri, b) = (r.left, r.top, r.right, r.bottom);
                SetRect(
                    &mut r,
                    (SapZoom * l as f32) as c_short,
                    (SapZoom * t as f32) as c_short,
                    (SapZoom * ri as f32) as c_short,
                    (SapZoom * b as f32) as c_short,
                );
            }};
        }

        aire_zoom!(i);
        i += 1;
        r.left = 80;
        r.right = r.left + 64;
        Affmob3d(mob3D + 7, &mut r, null_mut()); //Nuage

        aire_zoom!(i);
        i += 1;
        r.left = 120;
        r.right = r.left + 16;
        Affmob3d(mob3D + 28, &mut r, null_mut()); //Oiseau

        aire_zoom!(i);
        i += 1;
        InsetRect(&mut r, 0, -2);
        r.left = 110;
        r.right = r.left + 26;
        Affmob3d(mob3D + 34, &mut r, null_mut()); //Pierre

        aire_zoom!(i);
        i += 1;
        InsetRect(&mut r, 0, -4);
        r.left = 110;
        r.right = r.left + 32;
        Affmob3d(mob3D + 10, &mut r, null_mut()); //Arbre

        aire_zoom!(i);
        i += 1;
        InsetRect(&mut r, 0, -4);
        r.left = 110;
        r.right = r.left + 32;
        Affmob3d(mobAvplan + MOB_HERBE_BASE, &mut r, null_mut()); //Herbe

        AfficheSuite();

        let mut j: c_short = 0;
        JAUGE3d(j, nbNuages as f32 / (NB_NUAGES_MAX as f32));
        j += 1;
        JAUGE3d(j, nbOiseaux as f32 / (NB_OISEAUX_MAX as f32));
        j += 1;
        JAUGE3d(j, nbRochers as f32 / (NB_ROCHERS_MAX as f32));
        j += 1;
        JAUGE3d(j, nbArbres as f32 / (NB_ARBRES_MAX as f32));
        j += 1;
        JAUGE3d(j, nbHerbes as f32 / (NB_MAX_HERBE_AV_PLAN as f32));
        SetRect(&mut r, 0, 0, 600, 400);
        TfrEcr(&mut r);
        SetRect(
            &mut r,
            0,
            0,
            320 * ECHELLE as c_short,
            200 * ECHELLE as c_short,
        );
        CarboniseInvalRect(&raw const r);
        ValideUpdate();
        AcamSetPortOffscreen(gbak);
    }
}

/***********************************************************************
                        DEF3D: Defini la 3d
***********************************************************************/
pub extern "C" fn DEF3D() {
    unsafe {
        let mut r = Rect::default();
        let mut pt = Point::default();

        pal_chron = palhaute;
        palhaute = &raw const pal2 as *mut c_short;
        /* Affichage sur Ecran */
        let mut cont: c_char = 1;
        let mut pos: c_char = 0;
        sposx = 160;
        dsposx = 160;
        sposy = 50;
        dsposy = 50;
        SetRect(
            &mut r,
            16 * ECHELLE as c_short,
            0 * ECHELLE as c_short,
            (320 - 16) * ECHELLE as c_short,
            200 * ECHELLE as c_short,
        );
        FixeAireNoMouse(&mut r);
        while cont != 0 {
            JAUGES3D();
            loop {
                vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                aff_sour(mobBambou + 21, sposy, sposx);
                if bouton == 0 {
                    break;
                }
            }
            if MouseDansSuite(sposx, sposy) {
                cont = 0;
            } else {
                SetPt(&mut pt, sposx, sposy);
                let mut i: c_short = 0;
                while i < NB_JAUGES_3D as c_short {
                    AIREJAUGE3d(i, &mut r);
                    if pt.v >= r.top && pt.v <= r.bottom {
                        pos = i as c_char;
                        loop {
                            let val = (sposx - r.left) as f32 / (r.right - r.left) as f32;

                            match pos {
                                0 => nbNuages = (val * NB_NUAGES_MAX as f32) as c_long,
                                1 => nbOiseaux = (val * NB_OISEAUX_MAX as f32) as c_long,
                                2 => nbRochers = (val * NB_ROCHERS_MAX as f32) as c_long,
                                3 => nbArbres = (val * NB_ARBRES_MAX as f32) as c_long,
                                4 => nbHerbes = (val * NB_MAX_HERBE_AV_PLAN as f32) as c_long,
                                _ => {}
                            }
                            if nbNuages > NB_NUAGES_MAX as c_long {
                                nbNuages = NB_NUAGES_MAX as c_long;
                            }
                            if nbOiseaux > NB_OISEAUX_MAX as c_long {
                                nbOiseaux = NB_OISEAUX_MAX as c_long;
                            }
                            if nbRochers > NB_ROCHERS_MAX as c_long {
                                nbRochers = NB_ROCHERS_MAX as c_long;
                            }
                            if nbArbres > NB_ARBRES_MAX as c_long {
                                nbArbres = NB_ARBRES_MAX as c_long;
                            }
                            if nbHerbes > NB_MAX_HERBE_AV_PLAN as c_long {
                                nbHerbes = NB_MAX_HERBE_AV_PLAN as c_long;
                            }
                            if nbNuages < 0 {
                                nbNuages = 0;
                            }
                            if nbOiseaux < 0 {
                                nbOiseaux = 0;
                            }
                            if nbRochers < 0 {
                                nbRochers = 0;
                            }
                            if nbArbres < 0 {
                                nbArbres = 0;
                            }
                            if nbHerbes < 0 {
                                nbHerbes = 0;
                            }
                            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                            JAUGES3D();
                            aff_sour(mobBambou + 21, sposy, sposx);
                            if bouton == 0 {
                                break;
                            }
                        }
                        i = 255;
                    }
                    i += 1;
                }
            }
        }
        CreeDecor3d(true);
        FixeTailleMainWindow();
    }
}

/*********************************************************************
                           SaisieNomFichierLecture
Saisi un nom de fichier pour une lecture
=> Adresse ou rnager le nom,extension avec laquelle filtrer
<= VRAI si ok
********************************************************************/
static mut ExtensionCmp: [c_char; 10] = [0; 10];
/*---------------------------------------------------------------
                Filtre pour la selection des fichiers selon leur extension
---------------------------------------------------------------*/
extern "C" fn vect_filtre_selon_extension_sapiens(info: *mut CInfoPBRec) -> Boolean {
    let mut nom = [0 as c_char; 80];
    let mut i: usize = 0;

    unsafe {
        /* BlockMove((char *) info->hFileInfo.ioNamePtr, nom, 79) */
        let src = (*info).hFileInfo.ioNamePtr as *const c_char;
        let mut j: usize = 0;
        while j < 79 {
            nom[j] = *src.add(j);
            j += 1;
        }
        PtoCstr(nom.as_mut_ptr() as StringPtr);
        while nom[i] != 0 {
            if nom[i] == b'.' as c_char {
                i += 1;
                if stricmp(
                    nom.as_mut_ptr().add(i),
                    &raw const ExtensionCmp as *const c_char,
                ) == 0
                {
                    return false;
                } else {
                    return true;
                }
            }
            i += 1;
        }
    }
    true
}

/*********************************************************************
                           CarboniseCvtPath
Supprime les ':' dans le nom du path
********************************************************************/
pub extern "C" fn CarboniseCvtPath(_chaine: *mut c_char) {}

/******************************************************************
                           InitCheminsSapiens
Initialise les chemins de l'application
********************************************************************/
pub extern "C" fn InitCheminsSapiens() {
    unsafe {
        SaisitChemin(
            &raw mut volumeApplication,
            null_mut(),
            &raw mut PathApplication,
        );
        SetPathApplication();
    }
}

/*****************************************************************************
                   AJOUTE_EXTENSION
=> Nom du fichier, Extension du fichier
*****************************************************************************/
fn ajoute_extension(mut nom: *mut c_char, mut ext: *const c_char) {
    unsafe {
        while *nom != 0 && *nom != b'.' as c_char {
            nom = nom.add(1);
        }

        *nom = b'.' as c_char;
        nom = nom.add(1);
        *nom = *ext;
        ext = ext.add(1);
        nom = nom.add(1);
        *nom = *ext;
        ext = ext.add(1);
        nom = nom.add(1);
        *nom = *ext;
        nom = nom.add(1);
        *nom = 0;
    }
}

/* Nom du dernier fichier d'aventure propose a la sauvegarde */
static mut nom_sauve: [c_char; 255] = [0; 255];

/*********************************************************************
                         SaisieNomFichierLecture
Saisi un nom de fichier pour une lecture
=> Adresse ou ranger le nom,extension avec laquelle filtrer
<= VRAI si ok
***********************************************************************/
fn SaisieNomFichierLecture(nom: *mut c_char, extension: *const c_char) -> Boolean {
    let mut point = Point::default();
    let mut reply = SFReply {
        good: false,
        vRefNum: 0,
        fType: 0,
        fName: [0; 64],
    };
    let mut chaine = [0 as c_char; 255];

    unsafe {
        SetPt(
            &mut point,
            ((qd.screenBits.bounds.right - 348) / 2) as c_short,
            ((qd.screenBits.bounds.bottom - 200) / 2) as c_short,
        );
        if strlen(extension) != 0 {
            strcpy(&raw mut ExtensionCmp as *mut c_char, extension);
            sprintf(
                chaine.as_mut_ptr(),
                c"Chargement du fichier :|*.%s|*.%s".as_ptr(),
                extension,
                extension,
            );
            let vect_filtre = vect_filtre_selon_extension_sapiens as *mut c_void;
            SFGetFile(
                point,
                chaine.as_mut_ptr() as StringPtr,
                vect_filtre,
                -1,
                null_mut(),
                null_mut(),
                &mut reply,
            );
            DisposeRoutineDescriptor(vect_filtre);
        } else {
            let mut vide: u8 = 0;
            SFGetFile(
                point,
                &raw mut vide as StringPtr,
                null_mut(),
                -1,
                null_mut(),
                null_mut(),
                &mut reply,
            );
        }

        if !reply.good {
            return false;
        }
        SetVol(null_mut(), reply.vRefNum);
        PtoCstr(reply.fName.as_mut_ptr() as *mut u8);
        strcpy(nom, reply.fName.as_ptr());
        VRAI
    }
}

/*********************************************************************
                         SaisieNomFichierAventureEcriture
Saisi un nom de fichier pour une ecriture
=> Prompt,nom,format ou 0 si aucun
   si protect == NULL aucune saisie n'est faite
<= Format du fichier ou 0 si aucun
********************************************************************/
fn SaisieNomFichierAventureEcriture(nom: *mut c_char) -> Boolean {
    let mut point = Point::default();
    let mut reply = SFReply {
        good: false,
        vRefNum: 0,
        fType: 0,
        fName: [0; 64],
    };
    let mut chaine = [0 as c_char; 255];
    let mut titre = [0 as c_char; 255];

    unsafe {
        SetPt(
            &mut point,
            ((qd.screenBits.bounds.right - 304) / 2) as c_short,
            ((qd.screenBits.bounds.bottom - 184) / 2) as c_short,
        );
        strcpy(chaine.as_mut_ptr(), nom);
        CtoPstr(chaine.as_mut_ptr());
        if langue == FR as c_short {
            strcpy(titre.as_mut_ptr(), c"Fichier".as_ptr());
        } else if langue == ES as c_short {
            strcpy(titre.as_mut_ptr(), c"Archivo".as_ptr());
        } else {
            strcpy(titre.as_mut_ptr(), c"File".as_ptr());
        }
        CtoPstr(titre.as_mut_ptr());
        SFPutFile(
            point,
            titre.as_mut_ptr() as StringPtr,
            chaine.as_mut_ptr() as StringPtr,
            null_mut(),
            &mut reply,
        );
        if !reply.good {
            return false;
        }
        SetVol(null_mut(), reply.vRefNum);
        PtoCstr(reply.fName.as_mut_ptr() as *mut u8);
        strcpy(nom, reply.fName.as_ptr());
        true
    }
}

/***********************************************************************
                                SAUVE:
***********************************************************************/
pub extern "C" fn sauve() {
    unsafe {
        let mut checksum: c_short = 111;
        flefba = 0;
        if jour_act == jour_dsk {
            comment(113); /*Une seule aventure par jour*/
            return;
        }

        PreservePath();
        if strlen(&raw const nom_sauve as *const c_char) == 0 {
            strcpy(
                &raw mut nom_sauve as *mut c_char,
                &raw const nomjoueur as *const c_char,
            );
        }
        if !SaisieNomFichierAventureEcriture(&raw mut nom_sauve as *mut c_char) {
            RestorePath();
            return;
        }
        ajoute_extension(&raw mut nom_sauve as *mut c_char, c"sap".as_ptr());
        remove(&raw const nom_sauve as *const c_char);
        let mut fichier = fopen(&raw const nom_sauve as *const c_char, c"wb".as_ptr());
        if fichier.is_null() {
            flefba = 0;
            comment(92); /*erreur*/
            RestorePath();
            return;
        }
        let mut j: usize = 0;
        let mut ptr = soma[j].adresse as *mut c_uchar;
        putc(b'G' as c_int, fichier);
        putc(b'O' as c_int, fichier);
        putc(b'D' as c_int, fichier);
        initrnd(b'S' as c_short, b'A' as c_short, b'P' as c_short);
        while !ptr.is_null() {
            let mut i: c_ushort = 0;
            while i < soma[j].taille as c_ushort {
                let car = ((*ptr) as c_short) ^ rnd0();
                checksum = checksum.wrapping_add(car);
                putc(car as c_int, fichier);
                ptr = ptr.add(1);
                i += 1;
            }
            j += 1;
            ptr = soma[j].adresse as *mut c_uchar;
        }
        checksum &= 255;
        putc(checksum as c_int, fichier);
        SauveEltsDecorsMobiles(fichier);
        fclose(fichier);
        fichier = fopen(&raw const nom_sauve as *const c_char, c"rb".as_ptr());
        if getc(fichier) != b'G' as c_int {
            flefba = 0;
            comment(92); /*erreur*/
            RestorePath();
            return;
        } else {
            jour_dsk = jour_act;
        }
        fclose(fichier);
        RestorePath();
    }
}

/***********************************************************************
                                CHARGE:
***********************************************************************/
pub extern "C" fn charge() {
    unsafe {
        let mut nom = [0 as c_char; 255];
        let mut ptr: *mut c_uchar;
        let mut j: usize;
        let mut checksum: c_short = 111;

        flefba = 0;

        PreservePath();
        if !SaisieNomFichierLecture(nom.as_mut_ptr(), c"sap".as_ptr()) {
            RestorePath();
            return;
        }
        ajoute_extension(nom.as_mut_ptr(), c"sap".as_ptr());
        let fichier = fopen(nom.as_ptr(), c"rb".as_ptr());
        flefba = 0;
        if fichier.is_null() {
            comment(92);
            RestorePath();
            return;
        }
        primsap();
        j = 0;
        ptr = soma[j].adresse as *mut c_uchar;
        initrnd(b'S' as c_short, b'A' as c_short, b'P' as c_short);
        if getc(fichier) == b'G' as c_int
            && getc(fichier) == b'O' as c_int
            && getc(fichier) == b'D' as c_int
        {
            while !ptr.is_null() {
                let mut i: c_ushort = 0;
                while i < soma[j].taille as c_ushort {
                    let mut car = getc(fichier) as c_short;
                    if car < 0 {
                        flefba = 0;
                        comment(92); /*erreur*/
                        fclose(fichier);
                        RestorePath();
                        return;
                    }
                    checksum = checksum.wrapping_add(car as c_short);
                    car = car ^ rnd0();
                    *ptr = car as c_uchar;
                    ptr = ptr.add(1);
                    i += 1;
                }
                j += 1;
                ptr = soma[j].adresse as *mut c_uchar;
            }
            checksum &= 255;
            if checksum != getc(fichier) as c_short {
                flefba = 0;
                comment(92); /*erreur*/
                primsap(); /*Nyak nyak !!!*/
                mort = 0;
            }
        } else {
            comment(92); /*erreur*/
        }
        CreeDecor3d(true);
        ChargeEltsDecorsMobiles(fichier);
        fclose(fichier);
        minute = 0;
        zj = 0;
        chrono(0); /*pour couleur*/
        jour_dsk = jour_act;
        affich(dzeta as c_ushort);
        tfrima();
        generique(VRAI);
        if scenar == 6 {
            sceveu(6); /* Ouaaais c'est lui !!!!!*/
        }
        RestorePath();
    }
}

fn ajuste() {
    let mut car = 0;

    unsafe {
        sceveu(10);
        reset_clav();
        while car == 0 {
            car = (litclav() & 223) as u8;
        }
        if car as char >= 'A' && car as char <= 'C' {
            quel_synthe((car - ('A' as u8)) as c_short);
        }
    }
}

/***********************************************************************
                      BRAVO:Affiche resultat de la taille
***********************************************************************/
fn bravo(quoi: c_ushort) {
    let mut gbak = CGrafPtr::default();

    unsafe {
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(Ecran1);
        eff_sour();
        /* Le fond est blanc */
        effima(COULEUR_FOND_NUCLEUS);
        let mut coeff: f32 = 0.0;
        while coeff < 1.0 {
            GereInterface();
            let tab_index_palette = CalculePaletteAttenuee(quoi as c_short, coeff, 999);
            Affmobbrt(quoi as c_short, 96, 1, 0, tab_index_palette);
            if !tab_index_palette.is_null() {
                DisposPtr(tab_index_palette as *mut u8);
            }
            tfrima();
            coeff += 0.05;
        }
        AcamSetPortOffscreen(gbak);
    }
}

/***********************************************************************
                                TRFAB:Fabrique silex
***********************************************************************/
pub extern "C" fn trfab(regb: c_ushort) {
    unsafe {
        FixeMenusReduit();
        flefba = 0;
        clrbadi();
        /*recherche si silex */
        let mut i: c_char = liste1.taille as c_char - 1;
        while i >= 0 && liste1.elements[i as usize] != 19 {
            i -= 1;
        }
        if i < 0 {
            /*pas trouve ds la liste*/
            comment(87); /*tu n'as pas de silex*/
            return;
        }
        dervue = VUE_AUCUNE as c_uchar;
        let tfr = ciel_degr;
        /* on taille le silex */
        bruit(10);
        let trace_syl = cursil(regb as c_short);
        palhaute = &raw const pal2 as *mut c_short;
        flefba = 0;
        if (trace_syl as c_ushort) > PRECISION_SILEX as c_ushort {
            /* trace et incertitude du dit*/
            bruit(7); /* HELAS */
            /*affichage du silex casse*/
            bravo((mobSilex + MOB_SILEX_CASSE_BASE) as c_ushort);
            liste1.elements[i as usize] = 20; /*un caillou !!!*/
            comment(86);
        } else {
            bruit(6); /* VICTOIRE*/
            if (trace_syl as c_ushort) < 150 {
                comment(110);
            }
            if regb != 1 {
                liste1.elements[i as usize] = 23; /*sagaie*/
                bravo((mobSilex + MOB_SILEX_SAGAIE_BASE) as c_ushort);
            } else {
                liste1.elements[i as usize] = 21; /*hache*/
                bravo((mobSilex + MOB_SILEX_HACHE_BASE) as c_ushort);
            }
            comment(85); /*tu pssede une nll arme*/
        }
        attends_bruit();
        bande(0);
        generique(VRAI);
        ciel_degr = tfr;
        FixeMenusComplet();
    }
}

/***********************************************************************
                                TRGEST:gestion
***********************************************************************/
pub fn trgest(regb: c_ushort) {
    MontreSouris();
    unsafe {
        match regb {
            /* Informations */
            0 =>
            //DoInfo();
            {
                DoAide()
            }
            //sceveu(8);
            /* Charge aventure */
            1 => {
                if ConfirmePerteAventure() {
                    charge();
                    inimen(8);
                }
            }
            /* Sauve aventure */
            2 => sauve(),
            /* Personnage : nouvlle aventure */
            3 => {
                FixeMenusReduit();
                if ConfirmePerteAventure() {
                    defsap();
                }
                FixeMenusComplet();
            }
            4 => DoPreferences(),
            _ => {}
        }
    }
}
