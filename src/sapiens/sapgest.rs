#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/***********************************************************************
                        Module  # 5 gestion et autres
                           D G le 30 Janvier 1987
***********************************************************************/
use crate::Sapiens;
use crate::acam::{FAUX, TickCount, VRAI};
use crate::cbridge::blackColor;
use crate::exchange::{request_open_file, request_save_file};
use crate::port::PortId;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapglob::{ListeId, SapId, soma, tenerv};
use crate::sapiens::saptxt::Saptxt;
use crate::sapiens::sapwind::{ValideUpdate, bande};
use crate::sapiens::sapwmus::{attends_bruit, quel_synthe};
use crate::sapiens::{
    ECHELLE, FormeVue, NB_ARBRES_MAX, NB_MAX_HERBE_AV_PLAN, NB_NUAGES_MAX, NB_OISEAUX_MAX,
    NB_ROCHERS_MAX, PRECISION_SILEX, Vue,
};
use crate::shared::{Boolean, Point, RGBColor, Rect};
use crate::stubs::CarboniseInvalRect;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::Ordering;

const NB_LIGNE_SILEX: i16 = 200;
const NB_COL_SILEX: i16 = 300;
const COULEUR_FOND_NUCLEUS: i16 = 0x000;
static forsy1: [u8; 100] = [
    0, 0, 0, 0, 1, 1, 2, 3, 4, 4, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
    15, 15, 16, 16, 16, 16, 17, 17, 18, 18, 18, 18, 19, 19, 20, 20, 20, 20, 21, 21, 22, 22, 22, 22,
    22, 22, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24, 24, 24, 24, 24, 23, 23, 23, 23, 23, 23, 22, 22,
    22, 22, 21, 21, 20, 19, 18, 17, 16, 15, 14, 12, 10, 8, 6, 5, 3, 1, 0, 0, 0, 0, 0, 0,
];
static forsy2: [u8; 100] = [
    0, 0, 0, 2, 4, 5, 6, 7, 8, 8, 8, 9, 10, 10, 11, 11, 12, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18,
    18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 28, 28, 29,
    29, 29, 29, 30, 30, 30, 30, 30, 30, 30, 30, 31, 31, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 31, 31, 30, 29, 28, 26, 24, 22, 20, 16, 12, 6, 0, 0, 0, 0, 0, 0,
];

static formonde: [u8; 100] = [
    50, 50, 50, 50, 49, 49, 49, 49, 48, 48, 47, 47, 47, 46, 46, 46, 46, 45, 45, 44, 43, 43, 42, 42,
    41, 40, 39, 39, 38, 38, 37, 37, 36, 35, 34, 34, 33, 33, 32, 32, 31, 30, 29, 28, 27, 26, 25, 25,
    24, 24, 23, 23, 22, 21, 20, 20, 19, 19, 18, 18, 17, 17, 16, 16, 15, 15, 15, 15, 14, 14, 13, 13,
    12, 12, 11, 11, 10, 10, 9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0, 0, 0,
];

pub(crate) struct Sapgest {
    forsyl: &'static [u8],
    palSilex: [u16; 8],
    /* Dernier fichier d'aventure charge ou sauve (nom par defaut du dialogue) */
    dernier_fichier: Option<PathBuf>,
}

impl Default for Sapgest {
    fn default() -> Self {
        Self {
            forsyl: &forsy1,
            palSilex: [0; 8],
            dernier_fichier: None,
        }
    }
}

const NB_COUL_SILEX: i16 = 8;

/* Correspondance entre epaisseur et couleur */
static epaiss: [u16; 75] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5,
    5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 7,
];

/* Mobs du silex et curseurs (macros de SAPIENS.H, basees sur des globaux) */
const MOB_DESIGNE_BASE: i16 = 17; /* mobFleches+17 */
const MOB_HERBE_BASE: i16 = 1; /* mobAvplan+1 */
const MOB_SILEX_SAGAIE_BASE: i16 = 13; /* mobSilex+13 */
const MOB_SILEX_HACHE_BASE: i16 = 14; /* mobSilex+14 */
const MOB_SILEX_CASSE_BASE: i16 = 15; /* mobSilex+15 */
const MOB_DEHORS_BASE: i16 = 20; /* mobBambou+20 */
const MOB_FRAPPE_BASE: i16 = 21; /* mobBambou+21 */

const MOB_COURANT_BASE: i16 = 22; /* mobBambou+22 */
/* Traps de saisie (mots de passe de la version initiale) */

static TRAPS: [[u8; 7]; 7] = [
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
    pub(crate) fn plot_ecr(&mut self, y: i16, x: i16, couleur: i16) {
        let rgb =
            RGBColor::CvtCouleurStEnRgb(self.sapglob.palhaute.borrow().as_ref()[couleur as usize]);
        self.sapwind.ports.RGBForeColor(&rgb);
        self.sapwind
            .ports
            .MoveTo(x * ECHELLE as i16, y * ECHELLE as i16);
        self.sapwind.ports.Line(ECHELLE as i16, 0);
        self.sapwind.ports.ForeColor(blackColor as i16);
    }
}

impl Sapiens {
    /***********************************************************************
                        lit_sil
    Lit un point coloré dans le silex
    => ligne,colonne
    <= couleur 0-NB_COUL_SILEX
    ***********************************************************************/
    fn lit_sil(&mut self, y: i16, x: i16) -> i16 {
        let index = self
            .sapwind
            .ports
            .port(PortId::Ecran1)
            .GetIndexPixBrt(x + 106, y);
        let mut i: i16 = 0;
        while i < NB_COUL_SILEX {
            if self.sapgest.palSilex[i as usize] as i16 == index {
                return i;
            }
            i += 1;
        }
        self.Collapse("[Sapgest.lit_sil.1] Color not found");
        0
    }

    /**********************************************************************
                        ecrit_sil
    Ecrit un point coloré dans le silex
    => ligne (0-400) ,colonne (0-640),couleur 0-NB_COUL_SILEX
    ***********************************************************************/
    fn ecrit_sil(&mut self, y: i16, x: i16, couleur: i16) {
        self.sapwind.ports.port_mut(PortId::Ecran1).PlotIndexPixBrt(
            x + 106,
            y,
            self.sapgest.palSilex[couleur as usize] as i16,
        );
    }
}

/***********************************************************************
                        CAT:Affiche le catalogue courant
***********************************************************************/
pub(crate) fn cat() {}

impl Sapiens {
    /***********************************************************************
                    TRAPSURE:Acces reserve aux initiés...
    ***********************************************************************/
    pub(crate) fn trapsure(&mut self, quoi: i16) {
        self.sapglob.flefba = 0;
        if quoi > 0 {
            self.comment(114 + quoi as u16);
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
                self.sapglob.ajoutl(ListeId::Liste1, 22);
                self.sapglob.ajoutl(ListeId::Liste1, 25);
                self.sapglob.ajoutl(ListeId::Liste1, 46);
            }
            5 => self.sceveu(12),
            6 => cat(),
            _ => {}
        }
    }

    /***********************************************************************
                            TABDRT: Etablit la position droite
    ***********************************************************************/
    fn tabdrt(&mut self, pos: i16) -> i16 {
        let mut i: i16 = NB_COL_SILEX + 1;
        loop {
            i -= 1;
            if self.lit_sil(pos, i) != 0 || i <= NB_COL_SILEX / 2 {
                return i;
            }
        }
    }

    /***********************************************************************
                            TABGAU: Etablit la position gauche
    ***********************************************************************/
    fn tabgau(&mut self, pos: i16) -> i16 {
        let mut i: i16 = 0;
        loop {
            let courant = i;
            i += 1;
            if self.lit_sil(pos, courant) != 0 || i >= NB_COL_SILEX / 2 {
                return i;
            }
        }
    }

    /***********************************************************************
                            VALSYL:
    Evalue l'erreur entre l'etat du nucleus et la forme theorique attendue
    ***********************************************************************/
    fn valsyl(&mut self) -> u16 {
        let mut i: i16 = 0;
        let mut ecart: u32 = 0;
        while i < NB_LIGNE_SILEX {
            let f = self.sapgest.forsyl[(i / 2) as usize] as i16;
            let mut trav1 = (NB_COL_SILEX / 2) - 2 * f - self.tabgau(i); /*ecart par rapport à ma gauche*/
            if trav1 < 0 {
                trav1 = -trav1;
            }
            let mut trav2 = (NB_COL_SILEX / 2) + 2 * f - self.tabdrt(i); /*ecart à la droite*/
            if trav2 < 0 {
                trav2 = -trav2;
            }
            ecart = ecart.wrapping_add((trav1 + trav2) as u32);
            i += 2;
        }
        (ecart / 3) as u16
    }

    /***********************************************************************
                            AfficheSuite
    Affiche le mob suite
    ***********************************************************************/
    fn AfficheSuite(&mut self) {
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
const fn MouseDansSuite(posx: i16, posy: i16) -> bool {
    AIRE_SUITE.pt_in_rect(Point::new(posx, posy))
}

impl Sapiens {
    /***********************************************************************
                            SILEX: Affiche un silex
    Affiche le nucleus et la forme ideale du silex par dessus
    ***********************************************************************/
    fn TraceSilex(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        /* on efface la souris */
        self.eff_sour();
        /*dessin du noyau*/
        self.tfrvirt();
        /*dessin de la forme ideale a obtenir*/
        let mut i: i16 = 0;
        while i < NB_LIGNE_SILEX {
            let f = self.sapgest.forsyl[(i / 2) as usize] as i16;
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
        let mut x: i16;
        let mut pos1: i16;
        let mut pos2: i16;
        let mut couleur: i16;
        let mut posTheo: i16;

        /* Palette du silex */
        self.effima(COULEUR_FOND_NUCLEUS);
        let mut delta1: i16 = 60;
        /* TIRAGE DU COEFF KK DE LA PIERRE DE 3 A 15 */
        let kk = self.random.myRandom(3, 16) as i16;
        /* Pour l'ensemble des lignes du silex */
        let mut y = 0;
        while y < NB_LIGNE_SILEX {
            delta1 = delta1.wrapping_add(self.random.myRandom(-1, 2) as i16);
            if delta1 < 0 {
                delta1 = 0;
            }
            let index = y as usize / 2;
            // bug dans le Code C original avec dépassement de tableau
            let next = (index + 1).min(self.sapgest.forsyl.len() - 1);
            posTheo = (self.sapgest.forsyl[index] as i16 + self.sapgest.forsyl[next] as i16) / 2;
            pos1 = (NB_COL_SILEX / 2) - posTheo - delta1; /* position gauche du nucleus */
            delta1 = delta1.wrapping_add(self.random.myRandom(-1, 2) as i16);
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
                self.ecrit_sil(y, x, couleur);
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
                self.ecrit_sil(y, x, couleur);
                x -= 1;
            }
            y += 1;
        }
    }

    /***********************************************************************
                           AppliqueNlleEpaiss
    Applique une nouvelle epaisseur si la nouvelle est inferieure a l'ancienne
    ***********************************************************************/
    fn AppliqueNlleEpaiss(&mut self, y: i16, x: i16, epaiss_loc: i16) {
        let ancEpaiss = self.lit_sil(y, x);
        if ancEpaiss < epaiss_loc || epaiss_loc == 0 {
            self.ecrit_sil(y, x, epaiss_loc);
        }
    }

    /***********************************************************************
                           AppliqueImpact

    => Position x et y du coup, force du coup
    <= Bruit a jouer
    ***********************************************************************/
    fn AppliqueImpact(&mut self, mut posxf_l: i16, mut posyf_l: i16, forcef: i16) -> i16 {
        let posimp: i16;
        let mut onde: u16;
        let mut yact: u16;
        let mut calc: u16;
        let mut tfr: u16;
        let mut cass: u16;
        let mut okbruit: u16 = 0;
        let mut trav1: u16;
        let paslec: i16;
        let mut distact: i16;

        cass = 0;
        posxf_l *= ECHELLE as i16;
        posyf_l *= ECHELLE as i16;
        /* Le point d'impact est-il sur le nucleus si non on sort*/
        if (posxf_l > self.tabdrt(posyf_l)) || (posxf_l < self.tabgau(posyf_l)) {
            /* Dessin du silex */
            self.TraceSilex();
            return okbruit as i16;
        }
        /* Sens de l'ondex : coup a gauche ou a droite */
        let senson = (posxf_l < (NB_COL_SILEX / 2)) as u16; /*vrai si gauche faux si droite*/
        /* Tfr varie de 0 à 15*/
        tfr = (forcef / 16) as u16;
        /* Penet varie de 0 à 50 */
        let penet = (forcef * 50) / 256;
        let distcb = if senson != 0 {
            /*gauche*/
            tfr = tfr.wrapping_add(posxf_l as u16); /*le coup porte vers l'interieur*/
            if tfr < penet as u16 {
                tfr = penet as u16;
            }
            /*si on depasse le milieu ca casse*/
            if tfr > (NB_COL_SILEX / 2) as u16 {
                cass = cass.wrapping_add(1);
            }
            posimp = tfr as i16 - penet;
            /* Distance maximale */
            penet as u16 + (posimp - self.tabgau(posyf_l)) as u16
        } else {
            /*droite*/
            tfr = posxf_l as u16 - tfr;
            if tfr < (NB_COL_SILEX / 2) as u16 {
                cass = cass.wrapping_add(1); /*depasse le milieu casse*/
            }
            posimp = tfr as i16 + penet;
            penet as u16 - (posimp - self.tabdrt(posyf_l)) as u16
        };
        if (((forcef - 1) * 48 / 256) as u16) > distcb {
            /*force suffisante*/
            if cass != 0 {
                /*c'est cassé...*/
                okbruit = 3;
                let mut a: i16 = 74;
                if posyf_l < NB_LIGNE_SILEX / 2 {
                    /* On casse le haut du silex */
                    let mut i: i16 = posyf_l + 4;
                    while (0..NB_LIGNE_SILEX).contains(&i) {
                        if a < 0 {
                            a = 0;
                        }
                        let mut j: i16 = 0;
                        while j < NB_COL_SILEX {
                            self.AppliqueNlleEpaiss(i, j, 0);
                            j += 1;
                        }
                        i -= 1;
                        a = a.wrapping_sub(8);
                    }
                } else {
                    let mut i: i16 = posyf_l - 5;
                    while (0..NB_LIGNE_SILEX).contains(&i) {
                        if a < 0 {
                            a = 0;
                        }
                        let mut j: i16 = 0;
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
                yact = posyf_l as u16;
                calc = posyf_l as u16 + 1;
                onde = 0;
                trav1 = 0;
                while onde < 100 {
                    /*on applique l'onde*/
                    if yact < NB_LIGNE_SILEX as u16 {
                        let mut a: i16 = if senson != 0 {
                            /*gauche*/
                            (formonde[onde as usize] as i16 * forcef) / 256 + posimp
                        } else {
                            /*DROITE*/
                            posimp - (formonde[onde as usize] as i16 * forcef) / 256
                        };
                        if senson != 0 {
                            distact = a - self.tabgau(yact as i16);
                        } else {
                            distact = self.tabdrt(yact as i16) - a;
                        }
                        if distact > 0 {
                            let mut pas: i16 = if senson != 0 {
                                7400 / (distact + 1)
                            } else {
                                7400 / distact
                            };
                            if pas == 0 {
                                pas = 1;
                            }
                            let mut i: i16 = 7400;
                            while i >= 0 {
                                self.AppliqueNlleEpaiss(
                                    yact as i16,
                                    a,
                                    epaiss[(i / 100) as usize] as i16,
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
                    if calc < NB_LIGNE_SILEX as u16 {
                        let mut a: i16 = if senson != 0 {
                            /* gauche */
                            (formonde[onde as usize] as i16 * forcef) / 256 + posimp
                        } else {
                            /* droite */
                            posimp - (formonde[onde as usize] as i16 * forcef) / 256
                        };
                        if senson != 0 {
                            distact = a - self.tabgau(calc as i16);
                        } else {
                            distact = self.tabdrt(calc as i16) - a;
                        }
                        if distact > 0 {
                            let mut pas: i16 = if senson != 0 {
                                7400 / (distact + 1)
                            } else {
                                7400 / distact
                            };
                            if pas == 0 {
                                pas = 1;
                            }
                            let mut i: i16 = 7400;
                            while i >= 0 {
                                self.AppliqueNlleEpaiss(
                                    calc as i16,
                                    a,
                                    epaiss[(i / 100) as usize] as i16,
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
                    trav1 = trav1.wrapping_add((paslec & 255) as u16);
                    /*plus retenue*/
                    onde = onde
                        .wrapping_add((trav1 > 255) as u16)
                        .wrapping_add((paslec / 256) as u16);
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
        okbruit as i16
    }

    /***********************************************************************
                            CURSIL:Taille le silex
    => Silex a tailler
    <= Collapse sur le silex
    ***********************************************************************/
    fn cursil(&mut self, regb: i16) -> i16 {
        let mut i: i16;

        /* On calcule les index de couleurs */
        {
            let _g = self.sapwind.ports.on(PortId::Debecr);
            i = 0;
            while i < NB_COUL_SILEX {
                self.sapgest.palSilex[i as usize] =
                    SapColor2Index(&TAB_COUL_SILEX_RGB[i as usize]) as u16;
                i += 1;
            }
        }
        /* On fixe l'ecart stochastique gauche et droite */
        self.sapgest.forsyl = if regb == 0 { &forsy1 } else { &forsy2 };

        /* Effacement du centre : on remplit avec une pierre noire */
        self.FabriqueNucleus();
        self.sapglob.flefba = 0;

        /* on l'affiche tel quel */
        /* Mise en place des bords */
        bande(1);
        self.sapglob.flefba = 0;
        self.TraceSilex();
        let r = Rect::new(
            16 * ECHELLE as i16,
            0,
            (320 - 16) * ECHELLE as i16,
            200 * ECHELLE as i16,
        );
        self.FixeAireNoMouse(r);

        loop {
            loop {
                (self.sapglob.bouton, self.sapglob.posxf, self.sapglob.posyf) = self.vq_mouse();
                if self.sapglob.bouton == 0 {
                    break;
                }
            }
            let okbruit: u8;
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
                return self.valsyl() as i16;
            }
            /* Point d'application */
            self.plot_ecr(self.sapglob.posyf, self.sapglob.posxf, 0x700);
            if (self.sapglob.bouton & 1) != 0 {
                /*on frappe*/
                let t: u64 = TickCount() as u64;
                /* On determine la force du coup */
                let mut forcef: i16;
                loop {
                    (self.sapglob.bouton, _, _) = self.vq_mouse();
                    forcef = ((TickCount() as u64).wrapping_sub(t) * 4) as i16;
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
                        as u8;
                self.aff_sour(
                    self.sapwind.mobBambou + MOB_COURANT_BASE,
                    self.sapglob.posyf - 25,
                    self.sapglob.posxf,
                );
                /* On joue le bruit */
                if okbruit != 0 {
                    self.bruit(okbruit as i16);
                }
            } /*on frappe*/
        }
    }

    /***********************************************************************
                            GETNOM:Saisi un nom de la taille donnée
    ***********************************************************************/
    fn getnom(&mut self, mess: i16, position: *mut i8, taille: i16) -> i16 {
        let mut pos: i16 = 0;
        let mut car: i16;

        while self.litclav() != 0 {
            //Purge
        }
        let mut i = 0;
        while i < taille {
            self.sapglob.zoneu.elements[i as usize] = 32; /*on vide la zone*/
            i += 1;
        }
        unsafe {
            loop {
                /* On affiche le commentaire */
                self.efbadi();
                self.comment(mess as u16);
                let mut i: i16 = 0;
                while i < taille && self.sapglob.zoneu.elements[i as usize] != b' ' {
                    self.affcarmess(self.sapglob.zoneu.elements[i as usize] as i16);
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
                    for (t, trap_array) in TRAPS.iter().enumerate() {
                        let mut oktrap: i16 = 1;
                        for (j, trap) in trap_array.iter().enumerate() {
                            if (trap & 223) != (self.sapglob.zoneu.elements[j] & 223) {
                                oktrap = 0;
                            }
                        }
                        if oktrap != 0 {
                            self.trapsure(t as i16);
                            pos = 0;
                        }
                    }
                    if pos != 0 {
                        /*au moins un caractere*/
                        let mut p = position;
                        let mut i: i16 = 0;
                        while i < taille {
                            *p = self.sapglob.zoneu.elements[i as usize] as i8;
                            p = p.add(1);
                            i += 1;
                        }
                    }
                    self.efbadass();
                    self.TfrBadAss();
                    return pos;
                }
                if car > 32 && car <= b'z' as i16 && !(car >= b'0' as i16 && car <= b'9' as i16) {
                    if pos < taille {
                        self.sapglob.zoneu.elements[pos as usize] = car as u8;
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
    Initialise decor 3d, et affiche le copyright
    ***********************************************************************/
    pub(crate) fn DebutAventure(&mut self, isFirst: Boolean) {
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
        self.affich(self.sapglob.dzeta as u16); /* Affiche la vue trois d sur ecran 1*/
        self.tfrvirt();
        self.sapglob.flefba = 1;
        self.InitGenerique();
        self.generique(FAUX);
        self.TfrEcr(None);
        self.Frame3d((FAUX) as i16);
        self.efbadi();
        self.comment(67); /*d et o guillion*/
        for mut i in 0..60 {
            let posTimer = self.sapwclav.sapiens_timer.load(Ordering::Relaxed);
            self.chrono(1);
            self.chrono(0);
            self.chrono(0);
            self.affich(self.sapglob.dzeta as u16);
            self.sapwind.ports.tfrima();
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
        self.sapglob.liste1.elements[..9].copy_from_slice(&self.sapglob.listdep);
        if self.IsEnreg() {
            self.sapglob.liste1.elements[0] = 22; /*Massue a la place de hache */
        }
        self.sapglob.enetoi.copy_from_slice(&tenerv);
        self.sapglob.tagress.copy_from_slice(&self.sapglob.agrdep);
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
    fn AireJauge(&self, i: i16) -> Rect {
        Rect::new(
            114,
            6 + 14 * i,
            (114 + self.sapglob.maxparam[self.sapglob.sexe as usize][i as usize] / 2 + 2) as i16,
            16 + 14 * i,
        )
    }

    /***********************************************************************
                            AireValJauge
    Calcule l'aire ecran de la valeur interne d'une jauge
    ***********************************************************************/
    fn AireValJauge(&self, i: i16) -> Rect {
        Rect::new(
            115,
            7 + 14 * i,
            (115 + self.sapglob.param[i as u8 as usize] / 2) as i16,
            15 + 14 * i,
        )
    }

    /***********************************************************************
                            AfficheCadreJauge
    ***********************************************************************/
    pub(crate) fn AfficheCadreJauge(&mut self, i: i16) {
        let mut r = self.AireJauge(i);
        self.sapwind.ports.ForeColor(blackColor as i16);
        r.left *= ECHELLE as i16;
        r.right *= ECHELLE as i16;
        r.top *= ECHELLE as i16;
        r.bottom *= ECHELLE as i16;
        self.sapwind.ports.DoFrameRect(&r);
    }

    /***********************************************************************
                            JAUGE:
    Trace une jauge dans Debecr
    => numero de la jauge, indeicateur de transfert sur le reel
    ***********************************************************************/
    pub(crate) fn jauge(&mut self, i: i16, tfr: Boolean) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        let rval = self.AireValJauge(i);
        self.AfficheCadreJauge(i);
        self.boxf(rval.top, rval.left, rval.bottom, rval.right, 2);
        /* Effacement de la partie droite */
        let mut r = self.AireJauge(i);
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
            self.TfrEcr(Some(r));
        }
    }

    /***********************************************************************
                            PRIMJAUGE:primaire pour les jauges
    ***********************************************************************/
    pub(crate) fn primjauge(&mut self, jaugeCredit: i16) {
        self.sapglob.dervue = Vue::AUCUNE;
        self.effima(0x050); /* Efface ecran sur 100 ligne */
        self.tfrvirt();
        /* Affichage sur Ecran */
        self.Gotoxy(6, 1);
        self.motlst2(0);
        self.Gotoxy(6, 3);
        self.motlst2(1);
        self.Gotoxy(6, 5);
        self.motlst2(2);
        self.Gotoxy(6, 7);
        self.motlst2(3);
        self.Gotoxy(6, 9);
        self.motlst2(4);
        if jaugeCredit != 0 {
            self.Gotoxy(6, 11);
            self.motlst2(5);
        }

        /*trace des cadres jauges*/
        let mut i: i16 = 0;
        while i < 5 + jaugeCredit {
            self.jauge(i, false);
            i += 1;
        }
        self.TfrEcr(None);
    }

    /***********************************************************************
                            DEFSAP: Defini un sapiens
    ***********************************************************************/
    pub(crate) fn defsap(&mut self) {
        self.sapglob.sapvit = 0;
        let nomjoueur_ptr = &raw mut self.saptxt.nomjoueur as *mut i8;
        let taille = self.getnom(88, nomjoueur_ptr, 8);
        if taille == 0 {
            return;
        }
        self.sapglob.flefba = 0;
        self.saptxt.nomjoueur[taille as usize] = 0;
        /* definitions des quotas personnage*/
        for i in 0..5usize {
            self.sapglob.param[i] = 130;
        }
        self.sapglob.param[5] = 20;
        self.generique(VRAI);
        self.comment(89); /*   XX ou XY ???*/
        let mut car: u8 = 0;
        self.sapwind.reset_clav();
        self.sapglob.init_and_check_texts();
        let homme = self.sapglob.homme;
        let femme = self.sapglob.femme;
        while car != homme && car != femme {
            car = (self.litclav() & 223) as u8;
        }
        self.affcarmess(car as i16);
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
        self.sapglob.pal_chron = Rc::clone(&self.sapglob.palhaute);
        self.sapglob.palhaute = Rc::clone(&self.saplist.pal2);
        self.primjauge(1);
        self.AfficheSuite();
        let mut cont = 1;
        let mut pos;
        self.sapglob.sposx = 160;
        self.sapglob.dsposx = 160;
        self.sapglob.sposy = 50;
        self.sapglob.dsposy = 50;
        let mut r = Rect::new(
            16 * ECHELLE as i16,
            0,
            (320 - 16) * ECHELLE as i16,
            200 * ECHELLE as i16,
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
                let pt = Point::new(self.sapglob.sposx, self.sapglob.sposy);
                let mut i: i16 = 0;
                while i < 5 {
                    r = self.AireJauge(i);
                    if r.pt_in_rect(pt) {
                        pos = i as i8;
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
                            let rval = self.AireValJauge(pos as i16);
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
                            self.jauge(pos as i16, true);
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
        self.sapwind.ports.tfrima();
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
            self.sapwind.ports.tfrdemi();
            self.tfrvirt();
            self.AfficheSuite();

            self.affomb(
                self.sapwind.mobFleches + MOB_DESIGNE_BASE,
                (pos as i16 * 2 + 6) * 7 + 1,
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
                pos = (self.sapglob.sposy > 53) as i8;
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
        self.sapglob.palhaute = Rc::clone(&self.sapglob.pal_chron);
        self.DebutAventure(false);
        self.FixeAireNoMouseAireBasse();
    }
}

/***********************************************************************
                        AIREJAUGE3d:
Trace une jauge dans Debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
const CAR_UTILE: i16 = 180;

const fn AIREJAUGE3d(i: i16) -> Rect {
    Rect::new(75, 7 + 14 * i, 75 + CAR_UTILE, 15 + 14 * i)
}

/***********************************************************************
                        JAUGE3d:
Trace une jauge dans Debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
const NB_JAUGES_3D: i16 = 5;

impl Sapiens {
    fn JAUGE3d(&mut self, i: i16, val: f32) {
        let r = AIREJAUGE3d(i);
        self.boxf(r.top - 1, r.left - 1, r.bottom + 1, r.right + 1, 16);
        self.boxf(
            r.top,
            r.left,
            r.bottom,
            r.left + (CAR_UTILE as f32 * val) as i16,
            14,
        );
        self.boxf(
            r.top,
            r.left + (CAR_UTILE as f32 * val) as i16,
            r.bottom,
            r.right,
            4,
        );
    }

    /***********************************************************************
                            JAUGES3D
    Trace une jauge dans Debecr
    => numero de la jauge, indeicateur de transfert sur le reel
    ***********************************************************************/
    fn JAUGES3D(&mut self) {
        let mut i = 0;
        let mut r;
        let _g = self.sapwind.ports.on(PortId::Debecr);
        self.boxf(0, 32, 101, 287, 5);

        /* La jauge est tracee dans Debecr (hors-ecran, echelle 1) : la mise a
        l'echelle se fait plus tard, au recopie vers l'ecran. */
        macro_rules! aire_zoom {
            ($i:expr) => {{
                r = AIREJAUGE3d($i);
                r = self.sapwind.ports.current().zoomed(&r);
            }};
        }

        aire_zoom!(i);
        i += 1;
        r.left = 80;
        r.right = r.left + 64;
        self.Affmob3d(self.sapwind.mob3D + 7, r, None); //Nuage

        aire_zoom!(i);
        i += 1;
        r.left = 120;
        r.right = r.left + 16;
        self.Affmob3d(self.sapwind.mob3D + 28, r, None); //Oiseau

        aire_zoom!(i);
        i += 1;
        r.InsetRect(0, -2);
        r.left = 110;
        r.right = r.left + 26;
        self.Affmob3d(self.sapwind.mob3D + 34, r, None); //Pierre

        aire_zoom!(i);
        i += 1;
        r.InsetRect(0, -4);
        r.left = 110;
        r.right = r.left + 32;
        self.Affmob3d(self.sapwind.mob3D + 10, r, None); //Arbre

        aire_zoom!(i);
        i += 1;
        r.InsetRect(0, -4);
        r.left = 110;
        r.right = r.left + 32;
        self.Affmob3d(self.sapwind.mobAvplan + MOB_HERBE_BASE, r, None); //Herbe

        self.AfficheSuite();

        let mut j: i16 = 0;
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
        let r = Rect::new(0, 0, 600, 400);
        self.TfrEcr(Some(r));
        let r = Rect::new(0, 0, 320 * ECHELLE as i16, 200 * ECHELLE as i16);
        CarboniseInvalRect(&raw const r);
        ValideUpdate();
    }

    /***********************************************************************
                            DEF3D: Defini la 3d
    ***********************************************************************/
    pub(crate) fn DEF3D(&mut self) {
        self.sapglob.pal_chron = Rc::clone(&self.sapglob.palhaute);
        self.sapglob.palhaute = Rc::clone(&self.saplist.pal2);
        /* Affichage sur Ecran */
        let mut cont: i8 = 1;
        let mut pos: i8;
        self.sapglob.sposx = 160;
        self.sapglob.dsposx = 160;
        self.sapglob.sposy = 50;
        self.sapglob.dsposy = 50;
        let mut r = Rect::new(
            16 * ECHELLE as i16,
            0,
            (320 - 16) * ECHELLE as i16,
            200 * ECHELLE as i16,
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
                let pt = Point::new(self.sapglob.sposx, self.sapglob.sposy);
                let mut i: i16 = 0;
                while i < NB_JAUGES_3D {
                    r = AIREJAUGE3d(i);
                    if pt.v >= r.top && pt.v <= r.bottom {
                        pos = i as i8;
                        loop {
                            let val =
                                (self.sapglob.sposx - r.left) as f32 / (r.right - r.left) as f32;

                            match pos {
                                0 => self.sap3d.nbNuages = (val * NB_NUAGES_MAX as f32) as i64,
                                1 => self.sap3d.nbOiseaux = (val * NB_OISEAUX_MAX as f32) as i64,
                                2 => self.sap3d.nbRochers = (val * NB_ROCHERS_MAX as f32) as i64,
                                3 => self.sap3d.nbArbres = (val * NB_ARBRES_MAX as f32) as i64,
                                4 => {
                                    self.sapcomb.nbHerbes =
                                        (val * NB_MAX_HERBE_AV_PLAN as f32) as i64
                                }
                                _ => {}
                            }
                            if self.sap3d.nbNuages > NB_NUAGES_MAX as i64 {
                                self.sap3d.nbNuages = NB_NUAGES_MAX as i64;
                            }
                            if self.sap3d.nbOiseaux > NB_OISEAUX_MAX as i64 {
                                self.sap3d.nbOiseaux = NB_OISEAUX_MAX as i64;
                            }
                            if self.sap3d.nbRochers > NB_ROCHERS_MAX as i64 {
                                self.sap3d.nbRochers = NB_ROCHERS_MAX as i64;
                            }
                            if self.sap3d.nbArbres > NB_ARBRES_MAX as i64 {
                                self.sap3d.nbArbres = NB_ARBRES_MAX as i64;
                            }
                            if self.sapcomb.nbHerbes > NB_MAX_HERBE_AV_PLAN as i64 {
                                self.sapcomb.nbHerbes = NB_MAX_HERBE_AV_PLAN as i64;
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

fn nom_joueur(saptxt: &Saptxt) -> String {
    let bytes: Vec<u8> = saptxt
        .nomjoueur
        .iter()
        .copied()
        .take_while(|b| *b != 0)
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn nom_defaut_sauvegarde(sapiens: &Sapiens) -> String {
    sapiens
        .sapgest
        .dernier_fichier
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|n| n.to_string_lossy().into_owned())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| nom_joueur(&sapiens.saptxt))
}

impl Sapiens {
    /***********************************************************************
                                    SAUVE:
    ***********************************************************************/
    pub(crate) fn sauve(&mut self) {
        self.sapglob.flefba = 0;
        if self.sapglob.jour_act == self.sapglob.jour_dsk {
            self.comment(113); /*Une seule aventure par jour*/
            return;
        }

        let nom = nom_defaut_sauvegarde(self);
        let Some(mut chemin) = request_save_file("sap", &nom) else {
            return;
        };
        chemin.set_extension("sap");
        self.sapgest.dernier_fichier = Some(chemin.clone());
        let Ok(mut fichier) = File::create(&chemin) else {
            self.sapglob.flefba = 0;
            self.comment(92); /*erreur*/
            return;
        };
        let mut ecriture_ok = fichier.write_all(b"GOD").is_ok();
        self.random.initrnd(b'S' as i16, b'A' as i16, b'P' as i16);
        let mut checksum: i16 = 111;
        for cellule in &soma {
            for i in 0..cellule.taille as usize {
                let car = (self.soma_mut(cellule.adresse)[i] as i16) ^ self.random.rnd0();
                checksum = checksum.wrapping_add(car);
                ecriture_ok &= fichier.write_u8(car as u8).is_ok();
            }
        }
        checksum &= 255;
        ecriture_ok &= fichier.write_u8(checksum as u8).is_ok();
        ecriture_ok &= self.sap3d.SauveEltsDecorsMobiles(&mut fichier).is_ok();
        ecriture_ok &= fichier.flush().is_ok();
        drop(fichier);

        let Ok(mut fichier) = File::open(&chemin) else {
            self.sapglob.flefba = 0;
            self.comment(92); /*erreur*/
            return;
        };
        if !ecriture_ok || fichier.read_u8().unwrap_or(0) != b'G' {
            self.sapglob.flefba = 0;
            self.comment(92); /*erreur*/
            return;
        }
        self.sapglob.jour_dsk = self.sapglob.jour_act;
    }

    /***********************************************************************
                                    CHARGE:
    ***********************************************************************/
    pub(crate) fn charge(&mut self) {
        self.sapglob.flefba = 0;

        let Some(mut chemin) = request_open_file("sap", "") else {
            return;
        };
        chemin.set_extension("sap");
        let path = PathBuf::from(&chemin);
        self.sapgest.dernier_fichier = Some(chemin);
        self.sapglob.flefba = 0;
        let Ok(mut fichier) = File::open(path) else {
            self.comment(92);
            return;
        };
        self.primsap();
        self.random.initrnd(b'S' as i16, b'A' as i16, b'P' as i16);
        let mut checksum: i16 = 111;
        if fichier.read_u8().unwrap_or(0) == b'G'
            && fichier.read_u8().unwrap_or(0) == b'O'
            && fichier.read_u8().unwrap_or(0) == b'D'
        {
            for cellule in &soma {
                for i in 0..cellule.taille as usize {
                    let mut car = fichier.read_u8().map(|c| c as i16).unwrap_or(-1);
                    if car < 0 {
                        self.sapglob.flefba = 0;
                        self.comment(92); /*erreur*/
                        return;
                    }
                    checksum = checksum.wrapping_add(car);
                    car ^= self.random.rnd0();
                    self.soma_mut(cellule.adresse)[i] = car as u8;
                }
            }
            checksum &= 255;
            if checksum != fichier.read_u8().map(|c| c as i16).unwrap_or(-1) {
                self.sapglob.flefba = 0;
                self.comment(92); /*erreur*/
                self.primsap(); /*Nyak nyak !!!*/
                self.sapglob.mort = 0;
            }
        } else {
            self.comment(92); /*erreur*/
        }
        self.CreeDecor3d(true);
        let _ = self.sap3d.ChargeEltsDecorsMobiles(&mut fichier);
        self.sapglob.minute = 0;
        self.sapglob.zj = 0;
        self.chrono(0); /*pour couleur*/
        self.sapglob.jour_dsk = self.sapglob.jour_act;
        self.affich(self.sapglob.dzeta as u16);
        self.sapwind.ports.tfrima();
        self.generique(VRAI);
        if self.sapglob.scenar == 6 {
            self.sceveu(6); /* Ouaaais c'est lui !!!!!*/
        }
    }

    #[allow(dead_code)]
    fn ajuste(&mut self) {
        let mut car = 0;
        self.sceveu(10);
        self.sapwind.reset_clav();
        while car == 0 {
            car = (self.litclav() & 223) as u8;
        }
        if car as char >= 'A' && car as char <= 'C' {
            quel_synthe((car - b'A') as i16);
        }
    }

    /***********************************************************************
                          BRAVO:Affiche resultat de la taille
    ***********************************************************************/
    fn bravo(&mut self, quoi: u16) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);
        self.eff_sour();
        /* Le fond est blanc */
        self.effima(COULEUR_FOND_NUCLEUS);
        let mut coeff: f32 = 0.0;
        while coeff < 1.0 {
            self.GereInterface();
            let tab_index_palette = self.CalculePaletteAttenuee(quoi as i16, coeff, 999);
            self.Affmobbrt(quoi as i16, 96, 1, 0, Some(&tab_index_palette));
            self.sapwind.ports.tfrima();
            coeff += 0.05;
        }
    }

    /***********************************************************************
                                    TRFAB:Fabrique silex
    ***********************************************************************/
    pub(crate) fn trfab(&mut self, regb: u16) {
        self.FixeMenusReduit();
        self.sapglob.flefba = 0;
        self.clrbadi();
        /*recherche si silex */
        let mut i = self.sapglob.liste1.taille as i8 - 1;
        while i >= 0 && self.sapglob.liste1.elements[i as usize] != 19 {
            i -= 1;
        }
        if i < 0 {
            /*pas trouve ds la liste*/
            self.comment(87); /*tu n'as pas de silex*/
            return;
        }
        self.sapglob.dervue = Vue::AUCUNE;
        let tfr = self.sapglob.ciel_degr.clone();
        /* on taille le silex */
        self.bruit(10);
        let trace_syl = self.cursil(regb as i16);
        self.sapglob.palhaute = Rc::clone(&self.saplist.pal2);
        self.sapglob.flefba = 0;
        if (trace_syl as u16) > PRECISION_SILEX as u16 {
            /* trace et incertitude du dit*/
            self.bruit(7); /* HELAS */
            /*affichage du silex casse*/
            self.bravo((self.sapwind.mobSilex + MOB_SILEX_CASSE_BASE) as u16);
            self.sapglob.liste1.elements[i as usize] = 20; /*un caillou !!!*/
            self.comment(86);
        } else {
            self.bruit(6); /* VICTOIRE*/
            if (trace_syl as u16) < 150 {
                self.comment(110);
            }
            if regb != 1 {
                self.sapglob.liste1.elements[i as usize] = 23; /*sagaie*/
                self.bravo((self.sapwind.mobSilex + MOB_SILEX_SAGAIE_BASE) as u16);
            } else {
                self.sapglob.liste1.elements[i as usize] = 21; /*hache*/
                self.bravo((self.sapwind.mobSilex + MOB_SILEX_HACHE_BASE) as u16);
            }
            self.comment(85); /*tu possèdes une nouvelle arme*/
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
    pub(crate) fn trgest(&mut self, regb: u16) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchange::TEST_FILE;

    /// `Sapiens` est volumineux : on l'exécute dans un thread à grosse pile.
    fn avec_sapiens(f: impl FnOnce(&mut Sapiens) + Send + 'static) {
        std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let mut sapiens = Sapiens::default();
                sapiens.init_glob();
                sapiens.CalculeToutDegrades();
                let h = sapiens
                    .sapglob
                    .tabheur
                    .iter()
                    .find(|e| e.q != 0)
                    .unwrap()
                    .clone();
                sapiens.sapw3d.FixeDegradesFonctionHeure(h);
                f(&mut sapiens);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// Impose le fichier « choisi » dans la boîte de dialogue simulée.
    fn choisit_fichier(nom: &str) -> PathBuf {
        let chemin =
            std::env::temp_dir().join(format!("sapiens_test_{}_{nom}.sap", std::process::id()));
        TEST_FILE.with(|p| *p.borrow_mut() = Some(chemin.clone()));
        chemin
    }

    /// Aventure de référence (octets produits par la version d'origine du format).
    /// Régénérable avec `UPDATE_GOLDEN=1 cargo test sauve_produit`.
    const AVENTURE_REFERENCE: &[u8] = include_bytes!("testdata/aventure_reference.sap");

    /// Place l'état de jeu de référence (valeurs plausibles et distinctes).
    fn etat_reference(sapiens: &mut Sapiens) {
        let g = &mut sapiens.sapglob;
        g.soif = 200;
        g.fatig = 190;
        g.faim = 180;
        g.mort = 4;
        g.bless = 170;
        g.zoom = 120;
        g.agilite = 7;
        g.force = 9;
        g.resistance = 11;
        g.charisme = 13;
        g.vie = 15;
        g.sexe = 1;
        g.scenar = 2;
        g.scenpr = 20;
        g.ciel = 3;
        g.terre = 5;
        g.xj = 0x5800;
        g.yj = 0x5400;
        g.dzeta = 768;
        g.heure = 42;
        g.jour_act = 3;
        g.jour_dsk = 0;
    }

    #[test]
    fn sauve_produit_le_fichier_de_reference() {
        avec_sapiens(|sapiens| {
            let chemin = choisit_fichier("golden_sauve");
            etat_reference(sapiens);
            sapiens.sauve();
            let octets = std::fs::read(&chemin).unwrap();
            let _ = std::fs::remove_file(&chemin);
            if std::env::var_os("UPDATE_GOLDEN").is_some() {
                std::fs::write(
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/src/sapiens/testdata/aventure_reference.sap"
                    ),
                    &octets,
                )
                .unwrap();
                return;
            }
            assert_eq!(octets, AVENTURE_REFERENCE);
        });
    }

    #[test]
    fn charge_lit_le_fichier_de_reference() {
        avec_sapiens(|sapiens| {
            let chemin = choisit_fichier("golden_charge");
            std::fs::write(&chemin, AVENTURE_REFERENCE).unwrap();
            sapiens.charge();
            let _ = std::fs::remove_file(&chemin);

            let g = &sapiens.sapglob;
            assert_eq!(g.mort, 4);
            assert_eq!(g.zoom, 120);
            assert_eq!(g.agilite, 7);
            assert_eq!(g.force, 9);
            assert_eq!(g.resistance, 11);
            assert_eq!(g.charisme, 13);
            assert_eq!(g.vie, 15);
            assert_eq!(g.sexe, 1);
            assert_eq!(g.scenar, 2);
            assert_eq!(g.scenpr, 20);
            assert_eq!(g.ciel, 3);
            assert_eq!(g.terre, 5);
            assert_eq!(g.xj, 0x5800);
            assert_eq!(g.yj, 0x5400);
            assert_eq!(g.jour_act, 3);
            // charge() appelle chrono(0), qui peut faire baisser légèrement ces quotas
            assert!((190..=200).contains(&g.soif), "soif = {}", g.soif);
            assert!((180..=190).contains(&g.fatig), "fatig = {}", g.fatig);
            assert!((170..=180).contains(&g.faim), "faim = {}", g.faim);
            assert!((160..=170).contains(&g.bless), "bless = {}", g.bless);
        });
    }

    #[test]
    fn sauve_refuse_une_deuxieme_aventure_le_meme_jour() {
        avec_sapiens(|sapiens| {
            let chemin = choisit_fichier("memejour");
            sapiens.sapglob.jour_act = 1;
            sapiens.sapglob.jour_dsk = 1;
            sapiens.sauve();
            assert!(!chemin.exists());
        });
    }

    #[test]
    fn charge_fichier_corrompu_reinitialise_la_partie() {
        avec_sapiens(|sapiens| {
            let chemin = choisit_fichier("checksum");
            let mut octets = AVENTURE_REFERENCE.to_vec();
            let taille: usize = soma.iter().map(|c| c.taille as usize).sum();
            octets[3 + taille] ^= 0xFF; // octet de checksum
            std::fs::write(&chemin, octets).unwrap();

            sapiens.charge();
            let _ = std::fs::remove_file(chemin);
            // checksum faux => primsap() : valeurs de départ, mort remis à 0
            // (chrono(0) a pu baisser légèrement les quotas ensuite)
            assert_eq!(sapiens.sapglob.mort, 0);
            assert_eq!(sapiens.sapglob.scenar, 1);
            assert!(sapiens.sapglob.soif >= 240);
        });
    }

    #[test]
    fn charge_fichier_sans_en_tete_god_ne_charge_rien() {
        avec_sapiens(|sapiens| {
            let chemin = choisit_fichier("entete");
            let mut octets = AVENTURE_REFERENCE.to_vec();
            octets[0] = b'X';
            std::fs::write(&chemin, octets).unwrap();

            sapiens.charge();
            let _ = std::fs::remove_file(chemin);
            // les variables du fichier ne sont pas lues
            assert_ne!(sapiens.sapglob.agilite, 7);
            assert_ne!(sapiens.sapglob.force, 9);
        });
    }
}
