#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/***********************************************************************
                        Module  # 5 gestion et autres
                           D G le 30 Janvier 1987
***********************************************************************/
use crate::Sapiens;
use crate::acam::{
    CtoPstr, DisposPtr, FAUX, ForeColor, Line, MoveTo, PtoCstr, RGBForeColor, SetPt, SetRect,
    TickCount, VRAI,
};
use crate::cbridge::{
    CInfoPBRec, blackColor, fclose, fopen, getc, putc, remove, sprintf, strcpy, stricmp, strlen,
};
use crate::langue::Langue::{ES, FR};
use crate::port;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapglob::{ListeId, SapId, tenerv};
use crate::sapiens::saptxt::langue;
use crate::sapiens::sapwbrt::{DoFrameRect, GetIndexPixBrt, PlotIndexPixBrt};
use crate::sapiens::sapwcvt::Sapwcvt;
use crate::sapiens::sapwind::{ValideUpdate, bande};
use crate::sapiens::sapwmus::{attends_bruit, quel_synthe};
use crate::sapiens::{
    ECHELLE, FormeVue, NB_ARBRES_MAX, NB_MAX_HERBE_AV_PLAN, NB_NUAGES_MAX, NB_OISEAUX_MAX,
    NB_ROCHERS_MAX, PRECISION_SILEX, Vue,
};
use crate::shared::{Boolean, Point, RGBColor, Rect, SFReply, StringPtr};
use crate::stubs::{
    CarboniseInvalRect, DisposeRoutineDescriptor, PreservePath, RestorePath, SFGetFile, SFPutFile,
    SaisitChemin, SetPathApplication, SetVol, qd,
};
use std::ffi::c_ushort;
use std::os::raw::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_void};
use std::ptr::null_mut;
use std::sync::atomic::Ordering;

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

pub(crate) struct Sapgest {
    forsyl: *const c_uchar,
    palSilex: [c_ushort; 8],
    /* Nom du dernier fichier d'aventure propose a la sauvegarde */
    nom_sauve: [c_char; 255],
    /* Extension courante du filtre de selection de fichiers (ex-`ExtensionCmp`) */
    extension_cmp: [c_char; 10],
    /* Volume et repertoire de l'application (ex-globals SAPIENS.H) */
    pub(crate) PathApplication: c_long,
    pub(crate) volumeApplication: c_short,
}

impl Default for Sapgest {
    fn default() -> Self {
        Self {
            forsyl: forsy1.as_ptr(),
            palSilex: [0; 8],
            nom_sauve: [0; 255],
            extension_cmp: [0; 10],
            PathApplication: 0,
            volumeApplication: 0,
        }
    }
}

const NB_COUL_SILEX: c_short = 8;

/* Correspondance entre epaisseur et couleur */
static epaiss: [c_ushort; 75] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5,
    5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 7,
];

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

static AIRE_SUITE: Rect = Rect {
    top: 100 - 20,
    left: 320 - 16 - 32 - 8,
    bottom: 100 - 20 + 32,
    right: 320 - 16 - 8,
};

static TAB_COUL_SILEX_RGB: [RGBColor; 8] = [
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

impl Sapiens {
    /**********************************************************************
                        plot_ecr
    Ecrit un point coloré directement sur l'ecran
    => ligne,colonne ,couleur ST
    ***********************************************************************/
    pub(crate) fn plot_ecr(&self, y: c_short, x: c_short, couleur: c_short) {
        let mut rgb = RGBColor::default();
        Sapwcvt::CvtCouleurEnRgb(self.sapglob.palhaute, couleur, &mut rgb);
        RGBForeColor(&rgb);
        MoveTo(x * ECHELLE as c_short, y * ECHELLE as c_short);
        Line(ECHELLE as c_short, 0);
        ForeColor(blackColor as c_short);
    }
}

/***********************************************************************
                    lit_sil
Lit un point coloré dans le silex
=> ligne,colonne
<= couleur 0-NB_COUL_SILEX
***********************************************************************/
fn lit_sil(sapiens: &Sapiens, y: c_short, x: c_short) -> c_short {
    let index = GetIndexPixBrt(sapiens.sapwind.Ecran1, x + 106, y);
    let mut i: c_short = 0;
    while i < NB_COUL_SILEX {
        if sapiens.sapgest.palSilex[i as usize] as c_short == index {
            return i;
        }
        i += 1;
    }
    sapiens.Collapse("[Sapgest.lit_sil.1] Color not found");
    0
}

/**********************************************************************
                    ecrit_sil
Ecrit un point coloré dans le silex
=> ligne (0-400) ,colonne (0-640),couleur 0-NB_COUL_SILEX
***********************************************************************/
fn ecrit_sil(sapiens: &Sapiens, y: c_short, x: c_short, couleur: c_short) {
    PlotIndexPixBrt(
        sapiens.sapwind.Ecran1,
        x + 106,
        y,
        sapiens.sapgest.palSilex[couleur as usize] as c_short,
    );
}

/***********************************************************************
                        CAT:Affiche le catalogue courant
***********************************************************************/
pub(crate) fn cat() {}

impl Sapiens {
    /***********************************************************************
                    TRAPSURE:Acces reserve aux initiés...
    ***********************************************************************/
    pub(crate) fn trapsure(&mut self, quoi: c_short) {
        self.sapglob.flefba = 0;
        if quoi > 0 {
            self.comment(114 + quoi as c_ushort);
        }
        match quoi {
            0 => self.DEF3D(),
            /* Le bloc #ifdef OBSOLETE qui suivait n'est pas compile */
            1 => {
                self.sapglob.xj = 22528;
                self.sapglob.yj = 21504;
            }
            2 => self.sceveu(11),
            3 => self.sceveu(13),
            4 => {
                self.ajoutl(ListeId::Liste1, 22);
                self.ajoutl(ListeId::Liste1, 25);
                self.ajoutl(ListeId::Liste1, 46);
            }
            5 => self.sceveu(12),
            6 => cat(),
            _ => {}
        }
    }

    /***********************************************************************
                            TABDRT: Etablit la position droite
    ***********************************************************************/
    fn tabdrt(&self, pos: c_short) -> c_short {
        let mut i: c_short = NB_COL_SILEX + 1;
        loop {
            i -= 1;
            if lit_sil(self, pos, i) != 0 || i <= NB_COL_SILEX / 2 {
                return i;
            }
        }
    }

    /***********************************************************************
                            TABGAU: Etablit la position gauche
    ***********************************************************************/
    fn tabgau(&self, pos: c_short) -> c_short {
        let mut i: c_short = 0;
        loop {
            let courant = i;
            i += 1;
            if lit_sil(self, pos, courant) != 0 || i >= NB_COL_SILEX / 2 {
                return i;
            }
        }
    }

    /***********************************************************************
                            VALSYL:
    Evalue l'erreur entre l'etat du nucleus et la forme theorique attendue
    ***********************************************************************/
    fn valsyl(&self) -> c_ushort {
        let mut i: c_short = 0;
        let mut ecart: c_uint = 0;
        while i < NB_LIGNE_SILEX {
            let f = unsafe { *self.sapgest.forsyl.add((i / 2) as usize) } as c_short;
            let mut trav1 = (NB_COL_SILEX / 2) - 2 * f - self.tabgau(i); /*ecart par rapport à ma gauche*/
            if trav1 < 0 {
                trav1 = -trav1;
            }
            let mut trav2 = (NB_COL_SILEX / 2) + 2 * f - self.tabdrt(i); /*ecart à la droite*/
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
    fn AfficheSuite(&self) {
        self.affomb(
            self.sapwind.mobFleches + 19,
            AIRE_SUITE.top,
            AIRE_SUITE.left,
            2,
            2,
            true,
        );
    }
}

/***********************************************************************
                        MouseDansSuite
***********************************************************************/
const fn MouseDansSuite(posx: c_short, posy: c_short) -> bool {
    AIRE_SUITE.pt_in_rect(Point::new(posx, posy))
}

impl Sapiens {
    /***********************************************************************
                            SILEX: Affiche un silex
    Affiche le nucleus et la forme ideale du silex par dessus
    ***********************************************************************/
    fn TraceSilex(&mut self) {
        let _g = port::on(self.sapwind.debecr);
        /* on efface la souris */
        self.eff_sour();
        /*dessin du noyau*/
        self.tfrvirt();
        /*dessin de la forme ideale a obtenir*/
        let mut i: c_short = 0;
        while i < NB_LIGNE_SILEX {
            let f = unsafe { *self.sapgest.forsyl.add((i / 2) as usize) } as c_short;
            if f != 0 {
                self.plot_ecr(i / 2 + 1, 160 + f, 0x007);
                self.plot_ecr(i / 2 + 1, 160 - f, 0x007);
            }
            i += 4;
        }
        /* La jauge de force (AireJaugeForce) n'est plus tracee dans l'original */
        self.AfficheSuite();
        /* On transfere sur l'ecran */
        self.TfrEcr(None);
    }

    /***********************************************************************
                           FabriqueNucleus
    Dessine un nucleus brut sur l'ecran 1
    ***********************************************************************/
    fn FabriqueNucleus(&mut self) {
        let mut y: c_short;
        let mut x: c_short;
        let mut delta1: c_short;
        let mut pos1: c_short;
        let mut pos2: c_short;
        let kk: c_short;
        let mut couleur: c_short;
        let mut posTheo: c_short;

        /* Palette du silex */
        self.effima(COULEUR_FOND_NUCLEUS);
        delta1 = 60;
        /* TIRAGE DU COEFF KK DE LA PIERRE DE 3 A 15 */
        kk = self.random.myRandom(3, 16) as c_short;
        /* Pour l'ensemble des lignes du silex */
        y = 0;
        while y < NB_LIGNE_SILEX {
            delta1 = delta1.wrapping_add(self.random.myRandom(-1, 2) as c_short);
            if delta1 < 0 {
                delta1 = 0;
            }
            posTheo = (unsafe { *self.sapgest.forsyl.add((y / 2) as usize) } as c_short
                + unsafe { *self.sapgest.forsyl.add((y / 2 + 1) as usize) } as c_short)
                / 2;
            pos1 = (NB_COL_SILEX / 2) - posTheo - delta1; /* position gauche du nucleus */
            delta1 = delta1.wrapping_add(self.random.myRandom(-1, 2) as c_short);
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
                couleur = 1 + ((x - pos1) * yc + self.random.Rnd() * kk) / 1000;
                if couleur >= NB_COUL_SILEX {
                    couleur = NB_COUL_SILEX - 1;
                }
                ecrit_sil(self, y, x, couleur);
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
                couleur = 1 + ((pos2 - x) * yc + self.random.Rnd() * kk) / 1000;
                if couleur >= NB_COUL_SILEX {
                    couleur = NB_COUL_SILEX - 1;
                }
                ecrit_sil(self, y, x, couleur);
                x -= 1;
            }
            y += 1;
        }
    }

    /***********************************************************************
                           AppliqueNlleEpaiss
    Applique une nouvelle epaisseur si la nouvelle est inferieure a l'ancienne
    ***********************************************************************/
    fn AppliqueNlleEpaiss(&mut self, y: c_short, x: c_short, epaiss_loc: c_short) {
        let ancEpaiss = lit_sil(self, y, x);
        if ancEpaiss < epaiss_loc || epaiss_loc == 0 {
            ecrit_sil(self, y, x, epaiss_loc);
        }
    }

    /***********************************************************************
                           AppliqueImpact

    => Position x et y du coup, force du coup
    <= Bruit a jouer
    ***********************************************************************/
    fn AppliqueImpact(
        &mut self,
        mut posxf_l: c_short,
        mut posyf_l: c_short,
        forcef: c_short,
    ) -> c_short {
        let penet: c_short;
        let posimp: c_short;
        let mut onde: c_ushort;
        let mut yact: c_ushort;
        let mut calc: c_ushort;
        let distcb: c_ushort;
        let mut tfr: c_ushort;
        let mut cass: c_ushort;
        let senson: c_ushort;
        let mut okbruit: c_ushort = 0;
        let mut trav1: c_ushort;
        let paslec: c_short;
        let mut distact: c_short;

        cass = 0;
        posxf_l *= ECHELLE as c_short;
        posyf_l *= ECHELLE as c_short;
        /* Le point d'impact est-il sur le nucleus si non on sort*/
        if (posxf_l > self.tabdrt(posyf_l)) || (posxf_l < self.tabgau(posyf_l)) {
            /* Dessin du silex */
            self.TraceSilex();
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
            distcb = penet as c_ushort + (posimp - self.tabgau(posyf_l)) as c_ushort;
        } else {
            /*droite*/
            tfr = posxf_l as c_ushort - tfr;
            if tfr < (NB_COL_SILEX / 2) as c_ushort {
                cass = cass.wrapping_add(1); /*depasse le milieu casse*/
            }
            posimp = tfr as c_short + penet;
            distcb = penet as c_ushort - (posimp - self.tabdrt(posyf_l)) as c_ushort;
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
                            self.AppliqueNlleEpaiss(i, j, 0);
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
                            self.AppliqueNlleEpaiss(i, j, 0);
                            j += 1;
                        }
                        i += 1;
                        a = a.wrapping_sub(10);
                    }
                }
            } else {
                /* Le silex n'est pas cassé , on taille un eclat*/
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
                            distact = a - self.tabgau(yact as c_short) as c_short;
                        } else {
                            distact = self.tabdrt(yact as c_short) as c_short - a;
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
                                self.AppliqueNlleEpaiss(
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
                            distact = a - self.tabgau(calc as c_short) as c_short;
                        } else {
                            distact = self.tabdrt(calc as c_short) as c_short - a;
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
                                self.AppliqueNlleEpaiss(
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
        self.TraceSilex();
        okbruit as c_short
    }

    /***********************************************************************
                            CURSIL:Taille le silex
    => Silex a tailler
    <= Collapse sur le silex
    ***********************************************************************/
    fn cursil(&mut self, regb: c_short) -> c_short {
        let mut i: c_short;

        /* On calcule les index de couleurs */
        let _g = port::on(self.sapwind.debecr);
        i = 0;
        while i < NB_COUL_SILEX {
            self.sapgest.palSilex[i as usize] =
                SapColor2Index(&TAB_COUL_SILEX_RGB[i as usize]) as c_ushort;
            i += 1;
        }
        /* On fixe l'ecart stochastique gauche et droite */
        self.sapgest.forsyl = if regb == 0 {
            forsy1.as_ptr()
        } else {
            forsy2.as_ptr()
        };

        /* Effacement du centre : on remplit avec une pierre noire */
        self.FabriqueNucleus();
        self.sapglob.flefba = 0;

        /* on l'affiche tel quel */
        /* Mise en place des bords */
        bande(1);
        self.sapglob.flefba = 0;
        self.TraceSilex();
        let r = Rect::new(
            16 * ECHELLE as c_short,
            0,
            (320 - 16) * ECHELLE as c_short,
            200 * ECHELLE as c_short,
        );
        self.FixeAireNoMouse(r);

        loop {
            loop {
                (self.sapglob.bouton, self.sapglob.posxf, self.sapglob.posyf) = self.vq_mouse();
                if self.sapglob.bouton == 0 {
                    break;
                }
            }
            let okbruit: c_uchar;
            loop {
                (self.sapglob.bouton, self.sapglob.posxf, self.sapglob.posyf) = self.vq_mouse();
                if (self.sapglob.posxf < 32)
                    || (self.sapglob.posxf > 320)
                    || (self.sapglob.posyf > 100)
                    || (self.sapglob.posyf < 2)
                {
                    if self.sapglob.posyf > 100 {
                        self.sapglob.posyf = 100;
                    }
                    if self.sapglob.posxf > 320 - 16 {
                        self.sapglob.posxf = 320 - 16;
                    }
                    self.aff_sour(
                        self.sapwind.mobBambou + MOB_DEHORS_BASE,
                        self.sapglob.posyf,
                        self.sapglob.posxf,
                    );
                    self.sapglob.bouton &= 2;
                } else if MouseDansSuite(self.sapglob.posxf, self.sapglob.posyf) {
                    self.aff_sour(
                        self.sapwind.mobBambou + MOB_FRAPPE_BASE,
                        self.sapglob.posyf,
                        self.sapglob.posxf,
                    );
                } else {
                    self.aff_sour(
                        self.sapwind.mobBambou + MOB_COURANT_BASE,
                        self.sapglob.posyf - 25,
                        self.sapglob.posxf,
                    );
                }
                if self.sapglob.bouton == 0 {
                    break;
                }
            }
            /* On valide le silex */
            if MouseDansSuite(self.sapglob.posxf, self.sapglob.posyf) {
                self.affomb(
                    self.sapwind.mobFleches + 20,
                    AIRE_SUITE.top,
                    AIRE_SUITE.left,
                    2,
                    2,
                    true,
                );
                self.FixeAireNoMouseAireBasse();
                return self.valsyl() as c_short;
            }
            /* Point d'application */
            self.plot_ecr(self.sapglob.posyf, self.sapglob.posxf, 0x700);
            if (self.sapglob.bouton & 1) != 0 {
                /*on frappe*/
                let t: u64 = TickCount() as u64;
                /* On determine la force du coup */
                let mut forcef: c_short;
                loop {
                    (self.sapglob.bouton, _, _) = self.vq_mouse();
                    forcef = ((TickCount() as u64).wrapping_sub(t) * 4) as c_short;
                    self.aff_sour(
                        self.sapwind.mobBambou + MOB_COURANT_BASE,
                        self.sapglob.posyf - 25 - forcef / 8 - 2,
                        self.sapglob.posxf + forcef / 8 + 2,
                    );
                    if self.sapglob.bouton == 0 || forcef >= 254 {
                        break;
                    }
                }
                if forcef > 254 {
                    forcef = 254;
                }
                i = forcef;
                while i >= 0 {
                    self.aff_sour(
                        self.sapwind.mobBambou + MOB_COURANT_BASE,
                        self.sapglob.posyf - 25 - i / 8,
                        self.sapglob.posxf + i / 8,
                    );
                    i -= 32;
                }
                /* On applique l'impact : conversion de coordonnees */
                okbruit =
                    self.AppliqueImpact(self.sapglob.posxf - 85, self.sapglob.posyf - 1, forcef)
                        as c_uchar;
                self.aff_sour(
                    self.sapwind.mobBambou + MOB_COURANT_BASE,
                    self.sapglob.posyf - 25,
                    self.sapglob.posxf,
                );
                /* On joue le bruit */
                if okbruit != 0 {
                    self.bruit(okbruit as c_short);
                }
            } /*on frappe*/
        }
    }

    /***********************************************************************
                            GETNOM:Saisi un nom de la taille donnée
    ***********************************************************************/
    fn getnom(&mut self, mess: c_short, position: *mut c_char, taille: c_short) -> c_short {
        let mut pos: c_short = 0;
        let mut car: c_short;

        unsafe {
            while self.litclav() != 0 {
                //Purge
            }
            let mut i: c_short = 0;
            while i < taille {
                self.sapglob.zoneu.elements[i as usize] = 32; /*on vide la zone*/
                i += 1;
            }
            loop {
                /* On affiche le commentaire */
                self.efbadi();
                self.comment(mess as c_ushort);
                let mut i: c_short = 0;
                while i < taille && self.sapglob.zoneu.elements[i as usize] != b' ' {
                    self.affcarmess(self.sapglob.zoneu.elements[i as usize] as c_short);
                    i += 1;
                }
                self.affcarmess(2); /* Curseur */
                self.TfrBadAss();
                loop {
                    car = self.litclav();
                    if car != 0 {
                        break;
                    }
                }
                if car == 13 {
                    /* Validation : test des trapsures */
                    for t in 0..7usize {
                        let mut oktrap: c_short = 1;
                        for j in 0..6usize {
                            if (TRAPS[t][j] & 223) != (self.sapglob.zoneu.elements[j] & 223) {
                                oktrap = 0;
                            }
                        }
                        if oktrap != 0 {
                            self.trapsure(t as c_short);
                            pos = 0;
                        }
                    }
                    if pos != 0 {
                        /*au moins un caractere*/
                        let mut p = position;
                        let mut i: c_short = 0;
                        while i < taille {
                            *p = self.sapglob.zoneu.elements[i as usize] as c_char;
                            p = p.add(1);
                            i += 1;
                        }
                    }
                    self.efbadass();
                    self.TfrBadAss();
                    return pos;
                }
                if car > 32
                    && car <= b'z' as c_short
                    && !(car >= b'0' as c_short && car <= b'9' as c_short)
                {
                    if pos < taille {
                        self.sapglob.zoneu.elements[pos as usize] = car as c_uchar;
                        pos += 1;
                    }
                } else if car == 8 && pos != 0 {
                    pos -= 1;
                    self.sapglob.zoneu.elements[pos as usize] = 32;
                }
            }
        }
    }

    /***********************************************************************
                            DebutAventure
    Initialise decor 3d, et afiche le copyright
    ***********************************************************************/
    pub(crate) extern "C" fn DebutAventure(&mut self, isFirst: Boolean) {
        self.FixeMenusReduit();
        self.primsap();
        self.generique(FAUX);
        if isFirst {
            self.DefautDecor3d();
        } else {
            self.InitDecor3d();
        }
        self.CreeDecor3d(false);
        self.effvirt(0x000);
        self.effima(0x000);
        /*Presentation*/
        self.sapglob.heure = 10;
        self.chrono(0);
        self.affich(self.sapglob.dzeta as c_ushort); /* Affiche la vue trois d sur ecran 1*/
        self.tfrvirt();
        self.sapglob.flefba = 1;
        self.InitGenerique();
        self.generique(FAUX);
        self.TfrEcr(None);
        self.Frame3d((FAUX) as c_short);
        self.efbadi();
        self.comment(67); /*d et o guillion*/
        for mut i in 0..60 {
            let posTimer = self.sapwclav.sapiens_timer.load(Ordering::Relaxed);
            self.chrono(1);
            self.chrono(0);
            self.chrono(0);
            self.affich(self.sapglob.dzeta as c_ushort);
            self.tfrima();
            if (self.getkey(0) & (32 + 16)) > 0 {
                i = 111;
            }
            self.sapglob.flefba = 0;
            if i == 25 {
                self.comment(68);
            }
            if i == 50 {
                self.comment(69);
            }
            /* Synchronisation de la trame */
            while posTimer + 30 >= self.sapwclav.sapiens_timer.load(Ordering::Relaxed) {}
        }
        self.comment(70);
        self.FixeMenusComplet();
        self.MenuFenetres(if VRAI { 1 } else { 0 });
        self.efbadi();
        self.comment(132);
    }

    /***********************************************************************
                            PRIMSAP:definition primaires des saps
    ***********************************************************************/
    pub(crate) fn primsap(&mut self) {
        self.sapglob.scenar = 1;
        self.sapglob.jour_act = 1;
        self.sapglob.scenpr = 20;
        self.sapglob.fdialo = 0;
        self.sapglob.pos_evt = 0;
        self.sapglob.presence = 0;
        self.sapglob.viecopain = 0;
        self.sapglob.jour_dsk = 0;
        self.sapglob.dervue = Vue::AUCUNE;
        self.prm(SapId::Sap2, 0);
        self.prm(SapId::Sap3, 0);
        self.prm(SapId::Sap4, 0);
        /*quotas et positions*/
        self.sapglob.mort = 4;
        self.sapglob.faim = 255;
        self.sapglob.soif = 255;
        self.sapglob.fatig = 255;
        self.sapglob.bless = 255;
        self.sapglob.xj = 0x5800;
        self.sapglob.yj = 0x5400;
        self.sapglob.dzeta = 768;
        self.sapglob.zoom = 120;
        self.sapglob.heure = 1;
        self.sapglob.zj = 40;
        /*objets portes*/
        self.sapglob.liste1.taille = 9;
        for i in 0..9 {
            self.sapglob.liste1.elements[i] = self.sapglob.listdep[i];
        }
        if self.IsEnreg() {
            self.sapglob.liste1.elements[0] = 22; /*Massue a la place de hache */
        }
        for i in 0..4 {
            self.sapglob.enetoi[i] = tenerv[i];
            self.sapglob.tagress[i] = self.sapglob.agrdep[i];
        }
        self.sapglob.destine[0].type_ = 1; /*regenere les 4 chefs*/
        self.sapglob.destine[1].type_ = 1;
        self.sapglob.destine[2].type_ = 1;
        self.sapglob.destine[3].type_ = 1;
        self.sapglob.liste4.taille = 0;
    }

    /***********************************************************************
                            AireJauge
    Calcule l'aire ecran totale d'une jauge
    ***********************************************************************/
    pub(crate) fn AireJauge(&self, i: c_short, r: *mut Rect) {
        SetRect(
            r,
            114,
            6 + 14 * i,
            (114 + self.sapglob.maxparam[self.sapglob.sexe as usize][i as usize] / 2 + 2) as i16,
            16 + 14 * i,
        );
    }

    /***********************************************************************
                            AireValJauge
    Calcule l'aire ecran de la valeur interne d'une jauge
    ***********************************************************************/
    pub(crate) extern "C" fn AireValJauge(&self, i: c_short, r: *mut Rect) {
        SetRect(
            r,
            115,
            7 + 14 * i,
            (115 + self.sapglob.param[i as u8 as usize] / 2) as i16,
            15 + 14 * i,
        );
    }

    /***********************************************************************
                            AfficheCadreJauge
    ***********************************************************************/
    pub(crate) extern "C" fn AfficheCadreJauge(&self, i: c_short) {
        let mut r = Rect::default();

        self.AireJauge(i, &mut r);
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
    pub(crate) extern "C" fn jauge(&self, i: c_short, tfr: Boolean) {
        let mut rval = Rect::default();
        let mut r = Rect::default();

        let _g = port::on(self.sapwind.debecr);
        self.AireValJauge(i, &mut rval);
        self.AfficheCadreJauge(i);
        self.boxf(rval.top, rval.left, rval.bottom, rval.right, 2);
        /* Effacement de la partie droite */
        self.AireJauge(i, &mut r);
        if self.sapglob.param[i as usize] / 2
            < self.sapglob.maxparam[self.sapglob.sexe as usize][i as usize] / 2
        {
            self.boxf(r.top + 1, rval.right, r.bottom - 1, r.right - 1, 14);
            if (self.sapglob.param[i as usize]) > 0 {
                self.boxf(r.bottom - 2, r.left + 1, r.bottom - 1, rval.right - 1, 9);
                self.boxf(r.top + 1, rval.right - 1, r.bottom - 1, rval.right, 9);
            }
        }
        if tfr {
            r.left *= ECHELLE as i16;
            r.right *= ECHELLE as i16;
            r.top *= ECHELLE as i16;
            r.bottom *= ECHELLE as i16;
            self.TfrEcr(Some(&r));
        }
    }

    /***********************************************************************
                            PRIMJAUGE:primaire pour les jauges
    ***********************************************************************/
    pub(crate) extern "C" fn primjauge(&mut self, jaugeCredit: c_short) {
        unsafe {
            let app = self.GetApp();
            self.sapglob.dervue = Vue::AUCUNE;
            self.effima(0x050); /* Efface ecran sur 100 ligne */
            self.tfrvirt();
            /* Affichage sur Ecran */
            self.Gotoxy(6, 1);
            self.motlst2(*app.add(0));
            self.Gotoxy(6, 3);
            self.motlst2(*app.add(1));
            self.Gotoxy(6, 5);
            self.motlst2(*app.add(2));
            self.Gotoxy(6, 7);
            self.motlst2(*app.add(3));
            self.Gotoxy(6, 9);
            self.motlst2(*app.add(4));
            if jaugeCredit != 0 {
                self.Gotoxy(6, 11);
                self.motlst2(*app.add(5));
            }

            /*trace des cadres jauges*/
            let mut i: c_short = 0;
            while i < 5 + jaugeCredit {
                self.jauge(i, false);
                i += 1;
            }
            self.TfrEcr(None);
        }
    }

    /***********************************************************************
                            DEFSAP: Defini un sapiens
    ***********************************************************************/
    pub(crate) extern "C" fn defsap(&mut self) {
        unsafe {
            let mut rval = Rect::default();
            let mut pt = Point::default();

            self.sapglob.sapvit = 0;
            let nomjoueur_ptr = &raw mut self.saptxt.nomjoueur as *mut c_char;
            let taille = self.getnom(88, nomjoueur_ptr, 8);
            if taille == 0 {
                return;
            }
            self.sapglob.flefba = 0;
            *(&raw mut self.saptxt.nomjoueur as *mut c_char).add(taille as usize) = 0;
            /* definitions des quotas personnage*/
            for i in 0..5usize {
                self.sapglob.param[i] = 130;
            }
            self.sapglob.param[5] = 20;
            self.generique(VRAI);
            self.comment(89); /*   XX ou XY ???*/
            let mut car: c_uchar = 0;
            self.sapwind.reset_clav();
            self.sapglob.init_and_check_texts();
            let homme = self.sapglob.homme;
            let femme = self.sapglob.femme;
            while car != homme && car != femme {
                car = (self.litclav() & 223) as c_uchar;
            }
            self.affcarmess(car as c_short);
            self.TfrBadAss();
            if car == homme {
                self.sapglob.sexe = 1;
                let sap1 = &mut self.sapglob.sap1;
                sap1.cross[FormeVue::BUSTE as usize] = 0;
                sap1.cross[FormeVue::TETE as usize] = 0;
            } else {
                self.sapglob.sexe = 0;
                let sap1 = &mut self.sapglob.sap1;
                sap1.cross[FormeVue::BUSTE as usize] = 8;
                sap1.cross[FormeVue::TETE as usize] = 12;
            }
            self.sapglob.pal_chron = self.sapglob.palhaute;
            self.sapglob.palhaute = &raw const self.saplist.pal2 as *mut c_short;
            self.primjauge(1);
            self.AfficheSuite();
            let mut cont: c_char = 1;
            let mut pos: c_char;
            self.sapglob.sposx = 160;
            self.sapglob.dsposx = 160;
            self.sapglob.sposy = 50;
            self.sapglob.dsposy = 50;
            let mut r = Rect::new(
                16 * ECHELLE as c_short,
                0,
                (320 - 16) * ECHELLE as c_short,
                200 * ECHELLE as c_short,
            );
            self.FixeAireNoMouse(r);

            /* Selection des Kotas de force */
            while cont != 0 {
                loop {
                    (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
                    self.aff_sour(
                        self.sapwind.mobBambou + 21,
                        self.sapglob.sposy,
                        self.sapglob.sposx,
                    );
                    if self.sapglob.bouton == 0 {
                        break;
                    }
                }

                if MouseDansSuite(self.sapglob.sposx, self.sapglob.sposy) {
                    self.affomb(
                        self.sapwind.mobFleches + 20,
                        AIRE_SUITE.top,
                        AIRE_SUITE.left,
                        2,
                        2,
                        true,
                    );
                    cont = 0; /*esc*/
                    self.sapglob.agilite = if self.sapglob.param[0] < 223 {
                        self.sapglob.param[0] + 32
                    } else {
                        255
                    };
                    self.sapglob.force = self.sapglob.param[1];
                    self.sapglob.charisme = self.sapglob.param[2];
                    self.sapglob.vie = self.sapglob.param[3];
                    self.sapglob.resistance = self.sapglob.param[4];
                    while self.getkey(0) != 0 { /* Epuise le clavier */ }
                } else {
                    SetPt(&mut pt, self.sapglob.sposx, self.sapglob.sposy);
                    let mut i: c_short = 0;
                    while i < 5 {
                        self.AireJauge(i, &mut r);
                        if r.pt_in_rect(pt) {
                            pos = i as c_char;
                            /* Fleche */
                            self.affomb(
                                self.sapwind.mobFleches + MOB_DESIGNE_BASE,
                                r.top - 4,
                                86,
                                0,
                                0,
                                true,
                            );
                            loop {
                                self.AireValJauge(pos as c_short, &mut rval);
                                /* On augmente la jauge */
                                if self.sapglob.sposx > rval.right
                                    && self.sapglob.param[5] != 0
                                    && self.sapglob.param[pos as usize]
                                        < self.sapglob.maxparam[self.sapglob.sexe as usize]
                                            [pos as usize]
                                {
                                    self.sapglob.param[5] -= 1;
                                    self.sapglob.param[pos as usize] += 1;
                                }
                                if self.sapglob.sposx < rval.right
                                    && self.sapglob.param[5] < 254
                                    && self.sapglob.param[pos as usize] != 0
                                {
                                    self.sapglob.param[5] += 1;
                                    self.sapglob.param[pos as usize] -= 1;
                                }
                                (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) =
                                    self.vq_mouse();
                                self.jauge(pos as c_short, true);
                                /* Quantite restante */
                                self.jauge(5, true);
                                self.aff_sour(
                                    self.sapwind.mobBambou + MOB_FRAPPE_BASE,
                                    self.sapglob.sposy,
                                    self.sapglob.sposx,
                                );
                                if self.sapglob.bouton == 0 {
                                    break;
                                }
                            } /* while(bouton) */
                            self.primjauge(1);
                            self.AfficheSuite();
                            i = 255;
                        }
                        i += 1;
                    }
                }
            }
            /*selection de l'apparence*/
            self.eff_sour();
            cont = 1;
            pos = 0;
            self.prm(SapId::Sap1, -1);
            self.effima(0x050);
            self.tfrvirt();
            self.tfrima();
            self.sapglob.flefba = 0;
            self.eff_sour();
            while cont != 0 {
                {
                    let sap1 = &mut self.sapglob.sap1;
                    sap1.colonne = 28;
                    sap1.sens = 1;
                }
                self.effima(0x050); //??
                self.mvtsap(SapId::Sap1);
                self.tfrdemi();
                self.tfrvirt();
                self.AfficheSuite();

                self.affomb(
                    self.sapwind.mobFleches + MOB_DESIGNE_BASE,
                    (pos as c_short * 2 + 6) * 7 + 1,
                    100,
                    4,
                    4,
                    VRAI,
                );
                loop {
                    (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
                    self.aff_sour(
                        self.sapwind.mobBambou + 21,
                        self.sapglob.sposy,
                        self.sapglob.sposx,
                    );
                    if self.sapglob.bouton == 0 {
                        break;
                    }
                }

                if MouseDansSuite(self.sapglob.sposx, self.sapglob.sposy) {
                    self.affomb(
                        self.sapwind.mobFleches + 20,
                        AIRE_SUITE.top,
                        AIRE_SUITE.left,
                        2,
                        2,
                        true,
                    );
                    cont = 0; /*esc*/
                } else {
                    pos = (self.sapglob.sposy > 53) as c_char;
                    self.sapglob.dsposy = self.sapglob.sposy;
                    self.sapglob.dsposx = self.sapglob.sposx;
                    if (self.sapglob.bouton & 1) != 0 {
                        /*gauche*/
                        if pos != 0 {
                            let sap1 = &mut self.sapglob.sap1;
                            sap1.cross[FormeVue::BUSTE as usize] =
                                sap1.cross[FormeVue::BUSTE as usize].wrapping_add(2); /*Buste*/
                            if self.sapglob.sexe != 0 {
                                if sap1.cross[FormeVue::BUSTE as usize] >= 8 {
                                    sap1.cross[FormeVue::BUSTE as usize] = 0;
                                }
                            } else if sap1.cross[FormeVue::BUSTE as usize] >= 16 {
                                sap1.cross[FormeVue::BUSTE as usize] = 8;
                            }
                        } else {
                            let sap1 = &mut self.sapglob.sap1;
                            sap1.cross[FormeVue::TETE as usize] =
                                sap1.cross[FormeVue::TETE as usize].wrapping_add(2); /*tete*/
                            if self.sapglob.sexe != 0 {
                                if sap1.cross[FormeVue::TETE as usize] > 6 {
                                    sap1.cross[FormeVue::TETE as usize] = 0;
                                }
                            } else if sap1.cross[FormeVue::TETE as usize] > 22 {
                                sap1.cross[FormeVue::TETE as usize] = 12;
                            }
                        }
                        loop {
                            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) =
                                self.vq_mouse();
                            if (self.sapglob.bouton & 1) == 0 {
                                break;
                            }
                        }
                    }
                }
            }
            self.eff_sour();
            /* On est enregistre le personnage a une massue */
            self.sapglob.palhaute = self.sapglob.pal_chron;
            self.DebutAventure(false);
            self.FixeAireNoMouseAireBasse();
        }
    }
}

/***********************************************************************
                        AIREJAUGE3d:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
const CAR_UTILE: c_short = 180;

fn AIREJAUGE3d(i: c_short, r: *mut Rect) {
    SetRect(r, 75, 7 + 14 * i, 75 + CAR_UTILE, 15 + 14 * i);
}

/***********************************************************************
                        JAUGE3d:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
const NB_JAUGES_3D: c_short = 5;

impl Sapiens {
    fn JAUGE3d(&self, i: c_short, val: f32) {
        let mut r = Rect::default();

        unsafe {
            AIREJAUGE3d(i, &mut r);
            self.boxf(r.top - 1, r.left - 1, r.bottom + 1, r.right + 1, 16);
            self.boxf(
                r.top,
                r.left,
                r.bottom,
                r.left + (CAR_UTILE as f32 * val) as c_short,
                14,
            );
            self.boxf(
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
    fn JAUGES3D(&self) {
        let mut i: c_short = 0;
        let mut r = Rect::default();
        let _g = port::on(self.sapwind.debecr);
        unsafe {
            self.boxf(0, 32, 101, 287, 5);

            /* La jauge est tracee dans debecr (hors-ecran, echelle 1) : la mise a
            l'echelle se fait plus tard, au recopie vers l'ecran. */
            macro_rules! aire_zoom {
                ($i:expr) => {{
                    AIREJAUGE3d($i, &mut r);
                    r = port::zoomed(port::current(), &r);
                }};
            }

            aire_zoom!(i);
            i += 1;
            r.left = 80;
            r.right = r.left + 64;
            self.Affmob3d(self.sapwind.mob3D + 7, &mut r, null_mut()); //Nuage

            aire_zoom!(i);
            i += 1;
            r.left = 120;
            r.right = r.left + 16;
            self.Affmob3d(self.sapwind.mob3D + 28, &mut r, null_mut()); //Oiseau

            aire_zoom!(i);
            i += 1;
            r.InsetRect(0, -2);
            r.left = 110;
            r.right = r.left + 26;
            self.Affmob3d(self.sapwind.mob3D + 34, &mut r, null_mut()); //Pierre

            aire_zoom!(i);
            i += 1;
            r.InsetRect(0, -4);
            r.left = 110;
            r.right = r.left + 32;
            self.Affmob3d(self.sapwind.mob3D + 10, &mut r, null_mut()); //Arbre

            aire_zoom!(i);
            i += 1;
            r.InsetRect(0, -4);
            r.left = 110;
            r.right = r.left + 32;
            self.Affmob3d(self.sapwind.mobAvplan + MOB_HERBE_BASE, &mut r, null_mut()); //Herbe

            self.AfficheSuite();

            let mut j: c_short = 0;
            self.JAUGE3d(j, self.sap3d.nbNuages as f32 / (NB_NUAGES_MAX as f32));
            j += 1;
            self.JAUGE3d(j, self.sap3d.nbOiseaux as f32 / (NB_OISEAUX_MAX as f32));
            j += 1;
            self.JAUGE3d(j, self.sap3d.nbRochers as f32 / (NB_ROCHERS_MAX as f32));
            j += 1;
            self.JAUGE3d(j, self.sap3d.nbArbres as f32 / (NB_ARBRES_MAX as f32));
            j += 1;
            self.JAUGE3d(
                j,
                self.sapcomb.nbHerbes as f32 / (NB_MAX_HERBE_AV_PLAN as f32),
            );
            SetRect(&mut r, 0, 0, 600, 400);
            self.TfrEcr(Some(&r));
            SetRect(
                &mut r,
                0,
                0,
                320 * ECHELLE as c_short,
                200 * ECHELLE as c_short,
            );
            CarboniseInvalRect(&raw const r);
            ValideUpdate();
        }
    }

    /***********************************************************************
                            DEF3D: Defini la 3d
    ***********************************************************************/
    pub(crate) fn DEF3D(&mut self) {
        let mut r = Rect::default();
        let mut pt = Point::default();

        self.sapglob.pal_chron = self.sapglob.palhaute;
        self.sapglob.palhaute = &raw const self.saplist.pal2 as *mut c_short;
        /* Affichage sur Ecran */
        let mut cont: c_char = 1;
        let mut pos: c_char;
        self.sapglob.sposx = 160;
        self.sapglob.dsposx = 160;
        self.sapglob.sposy = 50;
        self.sapglob.dsposy = 50;
        SetRect(
            &mut r,
            16 * ECHELLE as c_short,
            0,
            (320 - 16) * ECHELLE as c_short,
            200 * ECHELLE as c_short,
        );
        self.FixeAireNoMouse(r);
        while cont != 0 {
            self.JAUGES3D();
            loop {
                (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
                self.aff_sour(
                    self.sapwind.mobBambou + 21,
                    self.sapglob.sposy,
                    self.sapglob.sposx,
                );
                if self.sapglob.bouton == 0 {
                    break;
                }
            }
            if MouseDansSuite(self.sapglob.sposx, self.sapglob.sposy) {
                cont = 0;
            } else {
                SetPt(&mut pt, self.sapglob.sposx, self.sapglob.sposy);
                let mut i: c_short = 0;
                while i < NB_JAUGES_3D as c_short {
                    AIREJAUGE3d(i, &mut r);
                    if pt.v >= r.top && pt.v <= r.bottom {
                        pos = i as c_char;
                        loop {
                            let val =
                                (self.sapglob.sposx - r.left) as f32 / (r.right - r.left) as f32;

                            match pos {
                                0 => self.sap3d.nbNuages = (val * NB_NUAGES_MAX as f32) as c_long,
                                1 => self.sap3d.nbOiseaux = (val * NB_OISEAUX_MAX as f32) as c_long,
                                2 => self.sap3d.nbRochers = (val * NB_ROCHERS_MAX as f32) as c_long,
                                3 => self.sap3d.nbArbres = (val * NB_ARBRES_MAX as f32) as c_long,
                                4 => {
                                    self.sapcomb.nbHerbes =
                                        (val * NB_MAX_HERBE_AV_PLAN as f32) as c_long
                                }
                                _ => {}
                            }
                            if self.sap3d.nbNuages > NB_NUAGES_MAX as c_long {
                                self.sap3d.nbNuages = NB_NUAGES_MAX as c_long;
                            }
                            if self.sap3d.nbOiseaux > NB_OISEAUX_MAX as c_long {
                                self.sap3d.nbOiseaux = NB_OISEAUX_MAX as c_long;
                            }
                            if self.sap3d.nbRochers > NB_ROCHERS_MAX as c_long {
                                self.sap3d.nbRochers = NB_ROCHERS_MAX as c_long;
                            }
                            if self.sap3d.nbArbres > NB_ARBRES_MAX as c_long {
                                self.sap3d.nbArbres = NB_ARBRES_MAX as c_long;
                            }
                            if self.sapcomb.nbHerbes > NB_MAX_HERBE_AV_PLAN as c_long {
                                self.sapcomb.nbHerbes = NB_MAX_HERBE_AV_PLAN as c_long;
                            }
                            if self.sap3d.nbNuages < 0 {
                                self.sap3d.nbNuages = 0;
                            }
                            if self.sap3d.nbOiseaux < 0 {
                                self.sap3d.nbOiseaux = 0;
                            }
                            if self.sap3d.nbRochers < 0 {
                                self.sap3d.nbRochers = 0;
                            }
                            if self.sap3d.nbArbres < 0 {
                                self.sap3d.nbArbres = 0;
                            }
                            if self.sapcomb.nbHerbes < 0 {
                                self.sapcomb.nbHerbes = 0;
                            }
                            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) =
                                self.vq_mouse();
                            self.JAUGES3D();
                            self.aff_sour(
                                self.sapwind.mobBambou + 21,
                                self.sapglob.sposy,
                                self.sapglob.sposx,
                            );
                            if self.sapglob.bouton == 0 {
                                break;
                            }
                        }
                        i = 255;
                    }
                    i += 1;
                }
            }
        }
        self.CreeDecor3d(true);
        self.sapwind.FixeTailleMainWindow();
    }
}

/*********************************************************************
                           SaisieNomFichierLecture
Saisi un nom de fichier pour une lecture
=> Adresse ou rnager le nom,extension avec laquelle filtrer
<= VRAI si ok
********************************************************************/
/*---------------------------------------------------------------
                Filtre pour la selection des fichiers selon leur extension
---------------------------------------------------------------*/
extern "C" fn vect_filtre_selon_extension_sapiens(
    info: *mut CInfoPBRec,
    extension: *const c_char,
) -> Boolean {
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
                if stricmp(nom.as_mut_ptr().add(i), extension) == 0 {
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

impl Sapiens {
    /******************************************************************
                               InitCheminsSapiens
    Initialise les chemins de l'application
    ********************************************************************/
    pub(crate) fn InitCheminsSapiens(&mut self) {
        SaisitChemin(
            &raw mut self.sapgest.volumeApplication,
            null_mut(),
            &raw mut self.sapgest.PathApplication,
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

/*********************************************************************
                         SaisieNomFichierLecture
Saisi un nom de fichier pour une lecture
=> Adresse ou ranger le nom,extension avec laquelle filtrer
<= VRAI si ok
***********************************************************************/
impl Sapiens {
    fn SaisieNomFichierLecture(&mut self, nom: *mut c_char, extension: *const c_char) -> Boolean {
        let mut reply = SFReply {
            good: false,
            vRefNum: 0,
            fType: 0,
            fName: [0; 64],
        };
        let mut chaine = [0 as c_char; 255];

        let point = Point::new(
            ((qd.screenBits.bounds.right - 348) / 2) as c_short,
            ((qd.screenBits.bounds.bottom - 200) / 2) as c_short,
        );
        unsafe {
            if strlen(extension) != 0 {
                strcpy(self.sapgest.extension_cmp.as_mut_ptr(), extension);
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
        if langue() == FR as c_short {
            strcpy(titre.as_mut_ptr(), c"Fichier".as_ptr());
        } else if langue() == ES as c_short {
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

impl Sapiens {
    /***********************************************************************
                                    SAUVE:
    ***********************************************************************/
    pub(crate) extern "C" fn sauve(&mut self) {
        unsafe {
            let mut checksum: c_short = 111;
            self.sapglob.flefba = 0;
            if self.sapglob.jour_act == self.sapglob.jour_dsk {
                self.comment(113); /*Une seule aventure par jour*/
                return;
            }

            PreservePath();
            if strlen(&raw const self.sapgest.nom_sauve as *const c_char) == 0 {
                strcpy(
                    &raw mut self.sapgest.nom_sauve as *mut c_char,
                    &raw const self.saptxt.nomjoueur as *const c_char,
                );
            }
            if !SaisieNomFichierAventureEcriture(&raw mut self.sapgest.nom_sauve as *mut c_char) {
                RestorePath();
                return;
            }
            ajoute_extension(
                &raw mut self.sapgest.nom_sauve as *mut c_char,
                c"sap".as_ptr(),
            );
            remove(&raw const self.sapgest.nom_sauve as *const c_char);
            let mut fichier = fopen(
                &raw const self.sapgest.nom_sauve as *const c_char,
                c"wb".as_ptr(),
            );
            if fichier.is_null() {
                self.sapglob.flefba = 0;
                self.comment(92); /*erreur*/
                RestorePath();
                return;
            }
            let mut j: usize = 0;
            let mut ptr = self.sapglob.soma[j].adresse as *mut c_uchar;
            putc(b'G' as c_int, fichier);
            putc(b'O' as c_int, fichier);
            putc(b'D' as c_int, fichier);
            self.random
                .initrnd(b'S' as c_short, b'A' as c_short, b'P' as c_short);
            while !ptr.is_null() {
                let mut i: c_ushort = 0;
                while i < self.sapglob.soma[j].taille as c_ushort {
                    let car = ((*ptr) as c_short) ^ self.random.rnd0();
                    checksum = checksum.wrapping_add(car);
                    putc(car as c_int, fichier);
                    ptr = ptr.add(1);
                    i += 1;
                }
                j += 1;
                ptr = self.sapglob.soma[j].adresse as *mut c_uchar;
            }
            checksum &= 255;
            putc(checksum as c_int, fichier);
            self.sap3d.SauveEltsDecorsMobiles(fichier);
            fclose(fichier);
            fichier = fopen(
                &raw const self.sapgest.nom_sauve as *const c_char,
                c"rb".as_ptr(),
            );
            if getc(fichier) != b'G' as c_int {
                self.sapglob.flefba = 0;
                self.comment(92); /*erreur*/
                RestorePath();
                return;
            } else {
                self.sapglob.jour_dsk = self.sapglob.jour_act;
            }
            fclose(fichier);
            RestorePath();
        }
    }

    /***********************************************************************
                                    CHARGE:
    ***********************************************************************/
    pub(crate) fn charge(&mut self) {
        unsafe {
            let mut nom = [0 as c_char; 255];
            let mut ptr: *mut c_uchar;
            let mut j: usize;
            let mut checksum: c_short = 111;

            self.sapglob.flefba = 0;

            PreservePath();
            if !self.SaisieNomFichierLecture(nom.as_mut_ptr(), c"sap".as_ptr()) {
                RestorePath();
                return;
            }
            ajoute_extension(nom.as_mut_ptr(), c"sap".as_ptr());
            let fichier = fopen(nom.as_ptr(), c"rb".as_ptr());
            self.sapglob.flefba = 0;
            if fichier.is_null() {
                self.comment(92);
                RestorePath();
                return;
            }
            self.primsap();
            j = 0;
            ptr = self.sapglob.soma[j].adresse as *mut c_uchar;
            self.random
                .initrnd(b'S' as c_short, b'A' as c_short, b'P' as c_short);
            if getc(fichier) == b'G' as c_int
                && getc(fichier) == b'O' as c_int
                && getc(fichier) == b'D' as c_int
            {
                while !ptr.is_null() {
                    let mut i: c_ushort = 0;
                    while i < self.sapglob.soma[j].taille as c_ushort {
                        let mut car = getc(fichier) as c_short;
                        if car < 0 {
                            self.sapglob.flefba = 0;
                            self.comment(92); /*erreur*/
                            fclose(fichier);
                            RestorePath();
                            return;
                        }
                        checksum = checksum.wrapping_add(car as c_short);
                        car = car ^ self.random.rnd0();
                        *ptr = car as c_uchar;
                        ptr = ptr.add(1);
                        i += 1;
                    }
                    j += 1;
                    ptr = self.sapglob.soma[j].adresse as *mut c_uchar;
                }
                checksum &= 255;
                if checksum != getc(fichier) as c_short {
                    self.sapglob.flefba = 0;
                    self.comment(92); /*erreur*/
                    self.primsap(); /*Nyak nyak !!!*/
                    self.sapglob.mort = 0;
                }
            } else {
                self.comment(92); /*erreur*/
            }
            self.CreeDecor3d(true);
            self.sap3d.ChargeEltsDecorsMobiles(fichier);
            fclose(fichier);
            self.sapglob.minute = 0;
            self.sapglob.zj = 0;
            self.chrono(0); /*pour couleur*/
            self.sapglob.jour_dsk = self.sapglob.jour_act;
            self.affich(self.sapglob.dzeta as c_ushort);
            self.tfrima();
            self.generique(VRAI);
            if self.sapglob.scenar == 6 {
                self.sceveu(6); /* Ouaaais c'est lui !!!!!*/
            }
            RestorePath();
        }
    }

    fn ajuste(&mut self) {
        let mut car = 0;
        self.sceveu(10);
        self.sapwind.reset_clav();
        while car == 0 {
            car = (self.litclav() & 223) as u8;
        }
        if car as char >= 'A' && car as char <= 'C' {
            quel_synthe((car - ('A' as u8)) as c_short);
        }
    }

    /***********************************************************************
                          BRAVO:Affiche resultat de la taille
    ***********************************************************************/
    fn bravo(&mut self, quoi: c_ushort) {
        let _g = port::on(self.sapwind.Ecran1);
        self.eff_sour();
        /* Le fond est blanc */
        self.effima(COULEUR_FOND_NUCLEUS);
        let mut coeff: f32 = 0.0;
        while coeff < 1.0 {
            self.GereInterface();
            let tab_index_palette = self.CalculePaletteAttenuee(quoi as c_short, coeff, 999);
            self.Affmobbrt(quoi as c_short, 96, 1, 0, tab_index_palette);
            if !tab_index_palette.is_null() {
                DisposPtr(tab_index_palette as *mut u8);
            }
            self.tfrima();
            coeff += 0.05;
        }
    }

    /***********************************************************************
                                    TRFAB:Fabrique silex
    ***********************************************************************/
    pub(crate) fn trfab(&mut self, regb: c_ushort) {
        self.FixeMenusReduit();
        self.sapglob.flefba = 0;
        self.clrbadi();
        /*recherche si silex */
        let mut i: c_char = self.sapglob.liste1.taille as c_char - 1;
        while i >= 0 && self.sapglob.liste1.elements[i as usize] != 19 {
            i -= 1;
        }
        if i < 0 {
            /*pas trouve ds la liste*/
            self.comment(87); /*tu n'as pas de silex*/
            return;
        }
        self.sapglob.dervue = Vue::AUCUNE;
        let tfr = self.sapglob.ciel_degr;
        /* on taille le silex */
        self.bruit(10);
        let trace_syl = self.cursil(regb as c_short);
        self.sapglob.palhaute = &raw const self.saplist.pal2 as *mut c_short;
        self.sapglob.flefba = 0;
        if (trace_syl as c_ushort) > PRECISION_SILEX as c_ushort {
            /* trace et incertitude du dit*/
            self.bruit(7); /* HELAS */
            /*affichage du silex casse*/
            self.bravo((self.sapwind.mobSilex + MOB_SILEX_CASSE_BASE) as c_ushort);
            self.sapglob.liste1.elements[i as usize] = 20; /*un caillou !!!*/
            self.comment(86);
        } else {
            self.bruit(6); /* VICTOIRE*/
            if (trace_syl as c_ushort) < 150 {
                self.comment(110);
            }
            if regb != 1 {
                self.sapglob.liste1.elements[i as usize] = 23; /*sagaie*/
                self.bravo((self.sapwind.mobSilex + MOB_SILEX_SAGAIE_BASE) as c_ushort);
            } else {
                self.sapglob.liste1.elements[i as usize] = 21; /*hache*/
                self.bravo((self.sapwind.mobSilex + MOB_SILEX_HACHE_BASE) as c_ushort);
            }
            self.comment(85); /*tu pssede une nll arme*/
        }
        attends_bruit();
        bande(0);
        self.generique(VRAI);
        self.sapglob.ciel_degr = tfr;
        self.FixeMenusComplet();
    }

    /***********************************************************************
                                    TRGEST:gestion
    ***********************************************************************/
    pub(crate) fn trgest(&mut self, regb: c_ushort) {
        self.souris.MontreSouris();
        match regb {
            /* Informations */
            0 =>
            //DoInfo();
            {
                self.DoAide()
            }
            //sceveu(8);
            /* Charge aventure */
            1 => {
                if self.ConfirmePerteAventure() {
                    self.charge();
                    self.inimen(8);
                }
            }
            /* Sauve aventure */
            2 => self.sauve(),
            /* Personnage : nouvlle aventure */
            3 => {
                self.FixeMenusReduit();
                if self.ConfirmePerteAventure() {
                    self.defsap();
                }
                self.FixeMenusComplet();
            }
            4 => self.DoPreferences(),
            _ => {}
        }
    }
}
