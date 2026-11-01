/*****************Version 2. 02************************************************
                Module de Gestion de la Trois D v 2.0
                (C) God Le 4 Décembre 1986
                Port Rust de "Sources/SAPW3D.C"
********************************************************************************/
#![allow(non_snake_case)]
// Gestion C de la 3d (cf. Sources/SAPW3D.C, (c) Og le 19-1-1996) :
// dégradés de palette en fonction de l'heure, ciel nocturne et étoiles,
// sol en lignes successives, pluie et éclairs, visualisation des montagnes.
#![allow(unsafe_op_in_unsafe_fn)]

use crate::Sapiens;
use crate::acam::{Random, TickCount};
use crate::port::{PortId, Ports};
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::{
    ECHELLE, FormeId, IndexColor, MontId, NB_DEGRADE_DISTANCE, NB_ETOILES, NB_LIGNES_CIEL,
    NB_LIGNES_TERRE, h_chron,
};
use crate::shared::{RGBColor, Rect};

const LARGEUR_ECRAN: i32 = 512;
const HAUTEUR_ECRAN: i32 = 256;
const AIRE_X_ETOILE: i32 = 2048;
const AIRE_Y_ETOILE: i32 = 128;
const MAX_MAGNITUDE: i32 = 16;

const NB_INTERV: usize = 10;
const LARGEUR_BANDE_SOL: i32 = 80;
const LARGEUR_MOB: i64 = 256;
const HAUTEUR_MOB: i64 = 32;
const HAUTEUR_MIN_BANDE: i32 = 5;

/* Les lignes sont organisees du plus loin au plus pres. */
#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
struct LigneSol {
    y: i16,
    hauteur: i16,
    largeur: i16,
    offsetEnMob: i16, /* Position X dans le Mob qui apparait au milieu de l'écran */
    offsetEcran: i16,
    dist: i16,
}

pub(crate) struct Sapw3d {
    TAB_PALETTE_DISTANCE: Vec<Vec<i16>>,
    TAB_COULEURS_CIEL: [IndexColor; NB_LIGNES_CIEL],
    COULEUR_ORAGE_JOUR: i16,
    COULEUR_ORAGE_NUIT: i16,
    IOTA_PALETTE_STD: f32,
    TAB_ETOILES: [Etoile; NB_ETOILES],
    COUL_PULSE: i16,
    METEORE: i16,
    X_METEORE: i16,
    Y_METEORE: i16,
    LAST_DZETA: i16,
    SOL_RAND_SEED: i64,
    TAB_LIGNE_SOL: [LigneSol; NB_INTERV],
}

impl Default for Sapw3d {
    fn default() -> Self {
        Self {
            TAB_PALETTE_DISTANCE: Vec::new(),
            TAB_COULEURS_CIEL: [IndexColor::default(); NB_LIGNES_CIEL],
            COULEUR_ORAGE_JOUR: 0,
            COULEUR_ORAGE_NUIT: 0,
            IOTA_PALETTE_STD: 0.14,
            TAB_ETOILES: [Etoile::default(); NB_ETOILES],
            COUL_PULSE: 0,
            METEORE: 0,
            X_METEORE: 0,
            Y_METEORE: 0,
            LAST_DZETA: 0,
            SOL_RAND_SEED: 0x2548795A,
            TAB_LIGNE_SOL: [LigneSol::default(); NB_INTERV],
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
struct Etoile {
    x: i16, /* 0 à AIRE_X_ETOILE */
    y: i16, /* 0 à 128 */
    magnitude: i16,
    couleur: i16,
}

/**************************************************************************
                        sinshort
***************************************************************************/
pub(crate) fn sinshort(angle: i16) -> i16 {
    (255.0 * (angle as f32 as f64 * 3.1415925 / 512.0).sin()) as i16
}

/**************************************************************************
                        cosshort
***************************************************************************/
pub(crate) fn cosshort(angle: i16) -> i16 {
    (255.0 * (angle as f32 as f64 * 3.1415925 / 512.0).cos()) as i16
}

/*****************************************************************************
                                        distance
*****************************************************************************/
pub(crate) const fn distance(dx: i64, dy: i64) -> i16 {
    let mut dx = dx.abs();
    let dy = dy.abs();
    if dx > dy {
        dx += dy * 2 / 5;
    } else {
        dx = dy + dx * 2 / 5;
    }
    if dx > 32700 {
        dx = 32700;
    }
    dx as i16
}

/*******************************************
            CalcPoint3D
=> x,y,z joueur, angle visee, x,y,z point
@= x,y ecran, distance
<= VRAI si visible
********************************************/
fn CalcPoint3D(
    pxj: i64,
    pyj: i64,
    pzj: i64,
    ca: f32,
    sa: f32,
    mut xp: i64,
    mut yp: i64,
    mut zp: i64,
) -> Option<(i16, i16, i16)> {
    let mut tmp: i64;
    /* --- Rabat les positions du point en fn du joueur  --- */
    xp -= pxj;
    yp -= pyj;
    zp -= pzj;

    /* --- Recalcule les positions 3d en fonction de l'angle de vue --- */
    tmp = (xp as f32 * ca - yp as f32 * sa) as i64;
    yp = (xp as f32 * sa + yp as f32 * ca) as i64;
    xp = tmp;

    /* --- Visible ? --- */
    if xp <= 2 {
        return None;
    }
    tmp = distance(xp, yp) as i64;
    if tmp == 0 {
        return None;
    }
    let dist = tmp;
    /* --- Calcule x écran --- */
    let xl = LARGEUR_ECRAN as i64 / 2 - (LARGEUR_ECRAN as i64 / 2) * yp / xp;

    /* --- Calcule Y ecran --- */
    let yl = HAUTEUR_ECRAN as i64 / 2 - (HAUTEUR_ECRAN as i64 / 2) * zp / xp;

    let xl = xl.clamp(-32760, 32760);
    let yl = yl.clamp(-32760, 32760);
    let x = xl as i16;
    let y = yl as i16;

    Some((x, y, dist as i16))
}

impl Sapiens {
    /************************************************************************
    ;-_-_-_-_-_-_-_-_-_-_-_-_ SapDep : deplacement du joueur -_-_-_-_-_-_-_-_
    ; Entrées -C- 1.Angle d'avancée
    ;             2.Vitesse de progression
    ;
    ************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn sapdep(&mut self, angle: i16, vitesse: i16) {
        self.sapglob.xj = (self.sapglob.xj as i64
            + (((cosshort(angle) as i64) * (vitesse as i64)) >> 8))
            .clamp(0, 65535) as u16;
        self.sapglob.yj = (self.sapglob.yj as i64
            + (((sinshort(angle) as i64) * (vitesse as i64)) >> 8))
            .clamp(0, 65535) as u16;
        self.AvanceSol(vitesse);
    }
}

/****************************************************************************
                    CalculeDegradeDuCiel
Calcul des tableau de degrades du ciel
*****************************************************************************/
const TAB_LIGN: [i16; 8] = [0, 41 * 2, 48 * 2, 53 * 2, 57 * 2, 59 * 2, 61 * 2, 64 * 2];

impl Sapiens {
    fn CalculeDegradeDuCiel(&mut self, index: usize) {
        let mut rgbDep = RGBColor::default();
        let mut j = 0;
        let mut incRed = 0;
        let mut incGreen = 0;
        let mut incBlue = 0;

        let _g = self.sapwind.ports.on(PortId::Ecran1);
        for i in 0..NB_LIGNES_CIEL as i32 {
            /* Changement de ligne */
            if i == TAB_LIGN[j as usize] as i32 {
                let palette = self.sapglob.tabheur[index].pal.borrow();
                rgbDep = RGBColor::CvtCouleurStEnRgb(palette[j as usize]);
                j += 1;
                let rgbDest = RGBColor::CvtCouleurStEnRgb(palette[j as usize]);
                let dist = TAB_LIGN[j as usize] as i32 - TAB_LIGN[(j - 1) as usize] as i32;
                incRed = (rgbDest.red as i64 - rgbDep.red as i64) / dist as i64;
                incGreen = (rgbDest.green as i64 - rgbDep.green as i64) / dist as i64;
                incBlue = (rgbDest.blue as i64 - rgbDep.blue as i64) / dist as i64;
            }
            rgbDep.red = (rgbDep.red as i64 + incRed) as u16;
            rgbDep.green = (rgbDep.green as i64 + incGreen) as u16;
            rgbDep.blue = (rgbDep.blue as i64 + incBlue) as u16;
            let idx = &mut self.sapglob.tabheur[index].TabCouleursCiel[i as usize];
            idx.rgb = rgbDep;
            idx.index = SapColor2Index(&rgbDep);
        }
    }

    /****************************************************************************
                        CalculeDegradeDistance
    Calcule le degrade des montagnes en fonction de la distance au joueur
    Le premier degrade est celui de premier plan, le dernier celui de l'horizon
    *****************************************************************************/
    fn CalculeDegradeDistance(&mut self, index: usize) {
        let mut coeff: f32;
        let delta: f32;
        let h = self.sapglob.tabheur[index].h;
        if h as i32 > 512 - 50 || (h as i32) < 50 {
            /* Nuit */
            let pos = if (h as i32) < 50 {
                h as i32
            } else {
                512 - h as i32
            };
            coeff =
                (0.7 + (pos as f32) as f64 / 200.0 + self.sapw3d.IOTA_PALETTE_STD as f64) as f32;
            delta = (0.5 / NB_DEGRADE_DISTANCE as f64) as f32;
        } else {
            /* c'est le jour */
            coeff = (1.0 + self.sapw3d.IOTA_PALETTE_STD as f64) as f32;
            delta = (0.4 / NB_DEGRADE_DISTANCE as f64) as f32;
        }
        for i in 0..NB_DEGRADE_DISTANCE as i32 {
            self.sapglob.tabheur[index].TabPaletteDistance[i as usize] =
                self.CalculePaletteAttenuee(self.sapwind.mob3D + 1, coeff, 33);
            coeff -= delta;
            if coeff < 0.0 {
                coeff = 0.0;
            }
        }
    }

    /****************************************************************************
                        CalculeToutDegrades
    Calcule l'ensemble des degrades de palette en fonction de l'heure en cours
    *****************************************************************************/
    pub(crate) fn CalculeToutDegrades(&mut self) {
        let mut rgb = RGBColor::default();

        let _g = self.sapwind.ports.on(PortId::Ecran1);
        /* Pour l'ensemble des evenements on calcule le degrade */
        for i in 0..self.sapglob.tabheur.len() {
            let e = &mut self.sapglob.tabheur[i];
            if e.q != 0 {
                e.TabCouleursCiel = [IndexColor::default(); NB_LIGNES_CIEL];
                e.TabPaletteDistance = vec![Vec::new(); NB_DEGRADE_DISTANCE as usize];
                self.CalculeDegradeDuCiel(i);
                self.CalculeDegradeDistance(i);
            }
        }
        rgb.red = 20000;
        rgb.green = 20000;
        rgb.blue = 20000;
        self.sapw3d.COULEUR_ORAGE_JOUR = SapColor2Index(&rgb);
        rgb.red = 8000;
        rgb.green = 8000;
        rgb.blue = 8000;
        self.sapw3d.COULEUR_ORAGE_NUIT = SapColor2Index(&rgb);
    }
}

impl Sapw3d {
    /****************************************************************************
                        FixeDegradesFonctionHeure
    En fonctione de l'evenement de changement de l'aire injecte les tabelaux de couleurs
    *****************************************************************************/
    pub(crate) fn FixeDegradesFonctionHeure(&mut self, chron: h_chron) {
        self.TAB_COULEURS_CIEL = chron.TabCouleursCiel;
        self.TAB_PALETTE_DISTANCE = chron.TabPaletteDistance;
    }
}

impl Sapiens {
    /****************************************************************************
                        CalculeEtoiles
                Effacement de la 3d
    *****************************************************************************/
    pub(crate) fn CalculeEtoiles(&mut self) {
        let mut rgb = RGBColor::default();
        let _g = self.sapwind.ports.on(PortId::Ecran1);

        /* Couleur de l'etoile */
        self.sapw3d.TAB_ETOILES = [Etoile::default(); NB_ETOILES];
        rgb.red = 65535;
        rgb.green = 65535;
        rgb.blue = 65535;
        self.sapw3d.COUL_PULSE = SapColor2Index(&rgb);
        for e in &mut self.sapw3d.TAB_ETOILES {
            e.x = self.random.myRandom(0, AIRE_X_ETOILE as i64) as i16;
            e.y = self.random.myRandom(0, AIRE_Y_ETOILE as i64) as i16;
            e.magnitude = self.random.myRandom(0, MAX_MAGNITUDE as i64) as i16;
            rgb.red = (55000 - e.y as i64 * (32768 / AIRE_Y_ETOILE as i64)) as u16;
            rgb.green = rgb.red;
            rgb.blue = 0;
            e.couleur = SapColor2Index(&rgb);
        }
    }

    /****************************************************************************
                        AfficheEtoiles
    Affiche les etoiles sur le ciel
    *****************************************************************************/
    pub(crate) fn AfficheEtoiles(&mut self) {
        if self.sapglob.heure as i32 > 480 || (self.sapglob.heure as i32) < 20 {
            /* Nuit */
            let port = self.sapwind.ports.current_mut();
            let tailleLigne = port.row_bytes;
            let magnitude: i16 = if (self.sapglob.heure as i32) < MAX_MAGNITUDE {
                (MAX_MAGNITUDE - self.sapglob.heure as i32) as i16
            } else if (self.sapglob.heure as i32) < 480 + MAX_MAGNITUDE {
                (self.sapglob.heure as i32 - (480 + MAX_MAGNITUDE)) as i16
            } else {
                MAX_MAGNITUDE as i16
            };
            for p in &mut self.sapw3d.TAB_ETOILES {
                if p.magnitude <= magnitude {
                    let x = (((self.sapglob.dzeta as i32 * 2 - p.x as i32 - 512) & 2047) - 512
                        + 256) as i16;
                    if (0..512).contains(&x) {
                        let y = p.y as i32;
                        let off = (y as i64 * tailleLigne as i64 + x as i64) as isize;
                        let coul = if self.random.Rnd() > 127 {
                            self.sapw3d.COUL_PULSE
                        } else {
                            p.couleur
                        };
                        port.put(off, coul as u8);
                    }
                }
            }
            if self.sapw3d.METEORE != 0 {
                let x = (((self.sapglob.dzeta as i32 * 2 - self.sapw3d.X_METEORE as i32 - 512)
                    & 2047)
                    - 512
                    + 256) as i16;
                self.Affmobbrt(
                    self.sapwind.mob3D + 21,
                    x / ECHELLE as i16,
                    self.sapw3d.Y_METEORE / ECHELLE as i16,
                    0,
                    None,
                );
                self.sapw3d.METEORE = (self.sapw3d.METEORE as i32 - 1) as i16;
                self.sapw3d.X_METEORE = (self.sapw3d.X_METEORE as i32 + 5) as i16;
                self.sapw3d.Y_METEORE = (self.sapw3d.Y_METEORE as i32 + 5) as i16;
            } else if self.random.myRandom(0, 1000) > 990 {
                self.sapw3d.X_METEORE = self.random.myRandom(0, AIRE_X_ETOILE as i64) as i16;
                self.sapw3d.Y_METEORE = self.random.myRandom(0, (AIRE_Y_ETOILE / 2) as i64) as i16;
                self.sapw3d.METEORE = self.random.myRandom(3, 6) as i16;
            }
            /* Etoile polaire */
            let x = (((self.sapglob.dzeta as i32 * 2 + 512) % 2048) - 512 + 256) as i16;
            let num = (self.sapwind.mob3D as i32 + self.random.myRandom(22, 24) as i32) as i16;
            self.Affmobbrt(num, x / 2, 20, 0, None);
        }
    }

    /*****************************************************************************
                                            CalculeDimensionsLigne
                                    Calcule les nouvelles dimensions des lignes du sol
    ******************************************************************************/
    fn CalculeDimensionsLignes(&mut self) {
        for i in 1..NB_INTERV {
            self.sapw3d.TAB_LIGNE_SOL[i].hauteur = (self.sapw3d.TAB_LIGNE_SOL[i].y as i32
                - self.sapw3d.TAB_LIGNE_SOL[i - 1].y as i32)
                as i16;
            self.sapw3d.TAB_LIGNE_SOL[i].largeur =
                (LARGEUR_MOB * self.sapw3d.TAB_LIGNE_SOL[i].hauteur as i64 / HAUTEUR_MOB) as i16;
        }
        self.sapw3d.TAB_LIGNE_SOL[0].hauteur = self.sapw3d.TAB_LIGNE_SOL[0].y;
        self.sapw3d.TAB_LIGNE_SOL[0].largeur =
            (LARGEUR_MOB * self.sapw3d.TAB_LIGNE_SOL[0].hauteur as i64 / HAUTEUR_MOB) as i16;
    }

    /*******************************************************************************
                                                                    TourneSol
                                    Le personnage tourne la tete, le tableau des lignes du sol bouge
    *********************************************************************************/
    fn TourneSol(&mut self) {
        if self.sapglob.dzeta != self.sapw3d.LAST_DZETA {
            let delta = self.sapglob.dzeta as i32 - self.sapw3d.LAST_DZETA as i32;
            for i in 1..NB_INTERV {
                if self.sapw3d.TAB_LIGNE_SOL[i].hauteur != 0 {
                    self.sapw3d.TAB_LIGNE_SOL[i].offsetEnMob =
                        ((self.datum(delta, i)) % LARGEUR_MOB as i32) as i16;
                    self.sapw3d.TAB_LIGNE_SOL[i].offsetEcran =
                        (-(self.sapw3d.TAB_LIGNE_SOL[i].offsetEnMob as i64)
                            * self.sapw3d.TAB_LIGNE_SOL[i].hauteur as i64
                            / HAUTEUR_MOB) as i16;
                }
            }
            self.sapw3d.LAST_DZETA = self.sapglob.dzeta;
        }
    }

    // offsetEnMob = offsetEnMob + (long)(delta*2)*(long)HAUTEUR_MOB/(long)hauteur
    const fn datum(&self, delta: i32, i: usize) -> i32 {
        (self.sapw3d.TAB_LIGNE_SOL[i].offsetEnMob as i64
            + (delta as i64 * 2) * HAUTEUR_MOB / self.sapw3d.TAB_LIGNE_SOL[i].hauteur as i64)
            as i32
    }

    /******************************************************************************
                                                            CalcY
            Calcule le Y ecran en fonction du delta d'altitude et de la distance
    *******************************************************************************/
    const fn CalcY(&self, idx: i16) -> i16 {
        let dist = self.sapw3d.TAB_LIGNE_SOL[idx as usize].dist as i64;
        (256 * (38 + self.sapglob.zj as i64 / 20) / dist) as i16
    }

    /*******************************************************************************
                                                                    AvanceSol
                                    Le personnage avance, le tableau des lignes du sol bouge
    *********************************************************************************/
    pub(crate) fn AvanceSol(&mut self, dist: i16) {
        if dist != 0 {
            self.sapw3d.SOL_RAND_SEED =
                TickCount() + self.sapglob.xj as i64 + self.sapglob.yj as i64;
        }
        /* Est-ce l'init ? oui -> on calcule tout*/
        if self.sapw3d.TAB_LIGNE_SOL[0].dist == 0 {
            let mut d: i32 = 70;
            for i in 0..NB_INTERV {
                self.sapw3d.TAB_LIGNE_SOL[NB_INTERV - 1 - i].dist = d as i16;
                self.sapw3d.TAB_LIGNE_SOL[NB_INTERV - 1 - i].y =
                    self.CalcY((NB_INTERV - 1 - i) as i16);
                d += LARGEUR_BANDE_SOL;
            }
        } else {
            for i in 0..NB_INTERV {
                self.sapw3d.TAB_LIGNE_SOL[i].dist =
                    (self.sapw3d.TAB_LIGNE_SOL[i].dist as i32 - dist as i32) as i16;
                self.sapw3d.TAB_LIGNE_SOL[i].y = self.CalcY(i as i16);
            }
            /* La ligne la plus proche s'est-elle barrée ? */
            while self.sapw3d.TAB_LIGNE_SOL[NB_INTERV - 1].dist < 70 {
                /* On décale le tableau */
                for i in (1..NB_INTERV).rev() {
                    self.sapw3d.TAB_LIGNE_SOL[i] = self.sapw3d.TAB_LIGNE_SOL[i - 1];
                }
                /* On recalcule le dernier élément */
                self.sapw3d.TAB_LIGNE_SOL[0].dist =
                    (self.sapw3d.TAB_LIGNE_SOL[1].dist as i32 + LARGEUR_BANDE_SOL) as i16;
                self.sapw3d.TAB_LIGNE_SOL[0].y = self.CalcY(0);
            }
        }
    }

    /*******************************************************************************
                                                                    AfficheSol
                                    On affiche les lignes du sol
    *********************************************************************************/
    fn AfficheSol(&mut self) {
        let mut r;
        let mut rdep: Rect;
        let mut i: i32;
        let mut numPalette: i32;
        let numMob: i16;

        self.AvanceSol(0);
        self.CalculeDimensionsLignes();
        self.TourneSol();
        /* Trace toutes les rangées */
        /* On recherch la premiere ligne qui n'est pas au loin */
        i = 1;
        while i < NB_INTERV as i32 && self.sapw3d.TAB_LIGNE_SOL[i as usize].hauteur < 4 {
            i += 1;
        }

        /* On trace toutes les lignes au loin */
        let ymax = if i < NB_INTERV as i32 {
            self.sapw3d.TAB_LIGNE_SOL[i as usize].y as i32 + NB_LIGNES_CIEL as i32
        } else {
            NB_LIGNES_CIEL as i32 + NB_LIGNES_TERRE as i32
        };
        let largeur = HAUTEUR_MIN_BANDE * LARGEUR_MOB as i32 * 4 / HAUTEUR_MOB as i32;
        let RandSeedBak = self.random.CarboniseGetRandSeed();
        self.random.CarboniseSetRandSeed(self.sapw3d.SOL_RAND_SEED);
        let mut j = ymax;
        while j > NB_LIGNES_CIEL as i32 {
            /* des bandes de 4 */
            let offset = ((2048 - self.sapglob.dzeta as i32 * 2 + self.random.Rnd() as i32)
                % largeur) as i16;
            let top = if j < NB_LIGNES_CIEL as i32 + HAUTEUR_MIN_BANDE {
                NB_LIGNES_CIEL as i32
            } else {
                j - HAUTEUR_MIN_BANDE
            };
            r = Rect::new(
                (256 - offset as i32) as i16,
                top as i16,
                (256 + largeur - offset as i32) as i16,
                j as i16,
            );
            numPalette = if r.bottom > NB_LIGNES_CIEL as i16 {
                (NB_LIGNES_CIEL as i32 + NB_LIGNES_TERRE as i32) - r.bottom as i32
            } else {
                NB_LIGNES_CIEL as i32 - NB_LIGNES_TERRE as i32 - r.bottom as i32
            };
            if numPalette >= NB_DEGRADE_DISTANCE as i32 {
                numPalette = NB_DEGRADE_DISTANCE as i32 - 1;
            }
            if numPalette < 0 {
                numPalette = 0;
            }
            /* Trace la partie droite */
            rdep = r;
            let mut k = 256 - offset as i32;
            while k < 512 {
                self.sapwbrt.Affmob3d(
                    self.sapwind.ports.current_mut(),
                    self.sapwind.mob3D + 25,
                    r,
                    Some(&self.sapw3d.TAB_PALETTE_DISTANCE[numPalette as usize]),
                );
                r.OffsetRect(largeur as i16, 0);
                k += largeur;
            }
            r = rdep;
            k = 256 - offset as i32;
            while k > 0 {
                r.OffsetRect(-largeur as i16, 0);
                self.sapwbrt.Affmob3d(
                    self.sapwind.ports.current_mut(),
                    self.sapwind.mob3D + 25,
                    r,
                    Some(&self.sapw3d.TAB_PALETTE_DISTANCE[numPalette as usize]),
                );
                k -= largeur;
            }
            j -= HAUTEUR_MIN_BANDE;
        }

        /* On trace toutes les ligne präs */
        let _ = numMob;
        while i < NB_INTERV as i32 {
            let hauteur = self.sapw3d.TAB_LIGNE_SOL[i as usize].hauteur as i32;
            let largeur2 = self.sapw3d.TAB_LIGNE_SOL[i as usize].largeur as i32;
            if hauteur > 0 && largeur2 > 0 {
                let offset = self.sapw3d.TAB_LIGNE_SOL[i as usize].offsetEcran as i32;
                let nmob = self.sapwind.mob3D + 20;
                let top = if i != 0 {
                    self.sapw3d.TAB_LIGNE_SOL[(i - 1) as usize].y as i32 + NB_LIGNES_CIEL as i32
                } else {
                    NB_LIGNES_CIEL as i32
                };
                let bottom = self.sapw3d.TAB_LIGNE_SOL[i as usize].y as i32 + NB_LIGNES_CIEL as i32;
                r = Rect::new(
                    (256 - offset) as i16,
                    top as i16,
                    (256 + largeur2 - offset) as i16,
                    bottom as i16,
                );
                numPalette = if r.bottom > NB_LIGNES_CIEL as i16 {
                    (NB_LIGNES_CIEL as i32 + NB_LIGNES_TERRE as i32) - r.bottom as i32
                } else {
                    NB_LIGNES_CIEL as i32 - NB_LIGNES_TERRE as i32 - r.bottom as i32
                };
                if numPalette >= NB_DEGRADE_DISTANCE as i32 {
                    numPalette = NB_DEGRADE_DISTANCE as i32 - 1;
                }
                if numPalette < 0 {
                    numPalette = 0;
                }
                /* Trace la partie droite */
                rdep = r;
                let mut kk = 256 - offset;
                while kk < 512 {
                    self.sapwbrt.Affmob3d(
                        self.sapwind.ports.current_mut(),
                        nmob,
                        r,
                        Some(&self.sapw3d.TAB_PALETTE_DISTANCE[numPalette as usize]),
                    );
                    r.OffsetRect(largeur2 as i16, 0);
                    kk += largeur2;
                }
                r = rdep;
                kk = 256 - offset;
                while kk > 0 {
                    r.OffsetRect(-largeur2 as i16, 0);
                    self.sapwbrt.Affmob3d(
                        self.sapwind.ports.current_mut(),
                        nmob,
                        r,
                        Some(&self.sapw3d.TAB_PALETTE_DISTANCE[numPalette as usize]),
                    );
                    kk -= largeur2;
                }
            }
            i += 1;
        }
        self.random.CarboniseSetRandSeed(RandSeedBak);
    }
}

impl Ports {
    /******************************************************************
                        TrameCielOrage
    ******************************************************************/
    fn TrameCielOrage(&mut self, index: i16) {
        /* Origine des données de l'ecran */
        let port = self.current_mut();
        let tailleLigne = port.row_bytes as isize;
        let mut d: isize = 0;
        for _ in 0..NB_LIGNES_CIEL as i32 / 2 {
            for _ in 0..tailleLigne / 2 {
                port.put(d, index as u8);
                port.put(d + tailleLigne + 1, index as u8);
                d += 2;
            }
            d += tailleLigne;
        }
    }

    /******************************************************************
                        Flou
    ******************************************************************/
    fn Flou(&mut self) {
        /* Origine des données de l'ecran */
        let port = self.current_mut();
        let tailleLigne = port.row_bytes as isize;
        for y in 0..200 {
            let deb = tailleLigne * y;
            for x in (0..tailleLigne).step_by(3) {
                if let Some(v) = port.px(deb + x + 1) {
                    port.put(deb + x, v);
                }
            }
        }
    }

    /******************************************************************
                        TracePluie
    => index de la couleur, nombre de gouttes
    ******************************************************************/
    fn TracePluie(&mut self, random: &mut Random, index: i16, n: i16) {
        if random.myRandom(0, 1000) > 500 {
            self.Flou();
        }
        /* Origine des données de l'ecran */
        let port = self.current_mut();
        let tailleLigne = port.row_bytes as isize;
        for _ in 0..n.max(0) {
            let x = random.myRandom(0, tailleLigne as i64) as isize;
            let y = random.myRandom(0, 150) as isize;
            let mut p = tailleLigne * y + x;
            for _ in 0..50 {
                port.put(p, index as u8);
                p += tailleLigne - 1;
            }
        }
    }
}

impl Sapiens {
    /****************************************************************************
                        eff3d
                Effacement de la 3d
    *****************************************************************************/
    pub(crate) fn eff3d(
        &mut self,
        _ciel: u8,  /* NU */
        _terre: u8, /* NU */
        ymeteore: u16,
        xmeteore: u16,
        isLune: u16,
    ) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);

        /* Le degrade du ciel */
        /* Un orage est en cours */
        self.sapwind
            .ports
            .RemplitZoneLignes(0, &self.sapw3d.TAB_COULEURS_CIEL);
        self.AfficheEtoiles();
        /* Le meteore */
        self.Affmobbrt(
            (crate::MOB_SOLEIL!(self.sapwind) as i32 + isLune as i32) as i16,
            xmeteore as i16,
            ymeteore as i16,
            0,
            None,
        );
        if self.sap3d.FreqEclair < 1000 {
            if isLune != 0 {
                self.sapwind
                    .ports
                    .TrameCielOrage(self.sapw3d.COULEUR_ORAGE_NUIT);
            } else {
                self.sapwind
                    .ports
                    .TrameCielOrage(self.sapw3d.COULEUR_ORAGE_JOUR);
            }
        }
        /* Le sol */
        self.AfficheSol();
    }

    /***************************************************************************************
                                     Pluie
    ***************************************************************************************/
    pub(crate) fn Pluie(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);

        if self.sap3d.FreqEclair < 1000 && self.sap3d.FreqEclair > 500 {
            let n = if self.sap3d.FreqEclair > 750 {
                1000 - self.sap3d.FreqEclair
            } else {
                self.sap3d.FreqEclair - 500
            };
            if self.sapglob.heure & 0x200 != 0 {
                self.sapwind.ports.TracePluie(
                    &mut self.random,
                    self.sapw3d.COULEUR_ORAGE_NUIT,
                    n as i16,
                );
            } else {
                self.sapwind.ports.TracePluie(
                    &mut self.random,
                    self.sapw3d.COULEUR_ORAGE_JOUR,
                    n as i16,
                );
            }
            /* Si pas deja de l'eau par terre on mets de l'eau a terre */
            if self.sapglob.liste4.taille > 0
                && self.sapglob.liste4.elements[self.sapglob.liste4.taille as usize - 1] != 2
                && self.sapglob.liste4.taille < self.sapglob.liste4.maxtaille
            {
                self.sapglob.liste4.taille = self.sapglob.liste4.taille.wrapping_add(1);
                self.sapglob.tlpos[4] = self.sapglob.liste4.taille as i8;
                self.sapglob.liste4.elements[self.sapglob.liste4.taille as usize - 1] = 2; /*Eau */
            }
        }
    }

    /***************************************************************************************
                                     PosePluie
    Quand il pleut il ya des flaques par terre
    ***************************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn PosePluie(&mut self) {
        if self.sap3d.FreqEclair < 1000 && self.sap3d.FreqEclair > 500 {
            /* Si pas deja de l'eau par terre on mets de l'eau a terre */
            if self.sapglob.liste4.taille > 0
                && self.sapglob.liste4.elements[self.sapglob.liste4.taille as usize - 1] != 2
                && self.sapglob.liste4.taille < self.sapglob.liste4.maxtaille
            {
                self.sapglob.liste4.taille = self.sapglob.liste4.taille.wrapping_add(1);
                self.sapglob.tlpos[4] = self.sapglob.liste4.taille as i8;
                self.sapglob.liste4.elements[self.sapglob.liste4.taille as usize - 1] = 2; /*Eau */
            }
        }
    }

    /***************************************************************************************
                                                    CalculeAltitude
                                    Calcule l'altitude d'un point du plan
    ***************************************************************************************/
    fn CalculeAltitude(&self, _x: i16, _y: i16) -> i16 {
        let mut z: i32 = 40;

        for mont in &self.sap3d.aigoual {
            if let Some(forme) = mont.forme.map(FormeId::forme)
                && forme.hauteur == 0
            {
                let d = distance(
                    self.sapglob.xj as i64 - mont.posx as i64,
                    self.sapglob.yj as i64 - mont.posy as i64,
                );
                let r = forme.rayon as i64;
                if (d as i64) < r {
                    z += ((r - d as i64) * forme.nbrdisk as i64 * mont.ign as i64 / r) as i32;
                }
            }
        }
        z as i16
    }

    /*****************************************************************************************
                                                    Visu :
    Calcul et visualisation d'un objet 3d
    L'affichage se fait sur ecran 1
    ******************************************************************************************/
    pub(crate) fn visu(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);
        self.sapglob.zj =
            self.CalculeAltitude(self.sapglob.xj as i16, self.sapglob.yj as i16) as u16;
        let angle = ((self.sapglob.dzeta as f32) * (3.1415927_f32)) as f64 / 512.0;
        let ca = (-angle).cos() as f32;
        let sa = (-angle).sin() as f32;

        for m in self.sap3d.chaine() {
            let Some(forme) = m.forme.map(FormeId::forme) else {
                continue;
            };
            if let Some((x, y, dist)) = CalcPoint3D(
                self.sapglob.xj as i64,
                self.sapglob.yj as i64,
                self.sapglob.zj as i64,
                ca,
                sa,
                m.posx as i64,
                m.posy as i64,
                forme.hauteur as i64,
            ) {
                let rd = (forme.rayon as i64 * 2 * 256 / dist as i64) as i16;
                let hauteurDisque =
                    (m.ign as i64 * forme.nbrdisk as i64 * 256 / dist as i64) as i16;
                let r = Rect::new(
                    (x as i32 - rd as i32 / 2) as i16,
                    (y as i32 - hauteurDisque as i32) as i16,
                    (x as i32 + rd as i32 / 2) as i16,
                    y,
                );
                if !r.is_empty()
                    && r.left < LARGEUR_ECRAN as i16
                    && r.right > 0
                    && r.bottom > 0
                    && r.top < HAUTEUR_ECRAN as i16
                {
                    let numPalette = if r.bottom > NB_LIGNES_CIEL as i16 {
                        (NB_LIGNES_CIEL as i32 + NB_LIGNES_TERRE as i32) - r.bottom as i32
                    } else {
                        r.bottom as i32 - (NB_LIGNES_CIEL as i32 - NB_LIGNES_TERRE as i32)
                    };
                    let mut numPalette = numPalette;
                    if numPalette >= NB_DEGRADE_DISTANCE as i32 {
                        numPalette = NB_DEGRADE_DISTANCE as i32 - 1;
                    }
                    if numPalette < 0 {
                        numPalette = 0;
                    }
                    self.sapwbrt.Affmob3d(
                        self.sapwind.ports.current_mut(),
                        self.sapwind.mob3D + forme.numMob3d,
                        r,
                        Some(&self.sapw3d.TAB_PALETTE_DISTANCE[numPalette as usize]),
                    );
                }
            }
        }
    }

    /*;*********************************
    ;               ChaineObjet
    ; Chaine une montagne en entree
    ;*********************************
    ;=> pointeur sur montagne a traiter
    ;: la montagne est chainee
    ;<= 1 si chainage 0 sinon
    */
    pub(crate) fn ChaineObjet(&mut self, idx: MontId) -> bool {
        let mut poschaine: Option<MontId> = None;
        let mut balaye: Option<MontId>;

        /* On calcule la distance de la montagne au joueur*/
        if self.sap3d.aigoual[idx as usize].ign == 0 {
            return false;
        }
        let Some(forme) = self.sap3d.aigoual[idx as usize].forme.map(FormeId::forme) else {
            return false;
        };
        let dist = distance(
            self.sap3d.aigoual[idx as usize].posx as i64 - self.sapglob.xj as i64,
            self.sap3d.aigoual[idx as usize].posy as i64 - self.sapglob.yj as i64,
        );
        self.sap3d.aigoual[idx as usize].distance = dist as u16;
        if dist as i64
            > 256 * forme.nbrdisk as i64 * self.sap3d.aigoual[idx as usize].ign as i64 / 4
        {
            return false;
        }
        if dist as i64 > 256 * forme.rayon as i64 / 3 {
            return false;
        }
        match (self.sapglob.dzeta as i32 >> 8) & 3 {
            0 => {
                if self.sap3d.aigoual[idx as usize].posx < self.sapglob.xj
                    && self.sap3d.aigoual[idx as usize].posy < self.sapglob.yj
                {
                    return false;
                }
            }
            1 => {
                if self.sap3d.aigoual[idx as usize].posx > self.sapglob.xj
                    && self.sap3d.aigoual[idx as usize].posy < self.sapglob.yj
                {
                    return false;
                }
            }
            2 => {
                if self.sap3d.aigoual[idx as usize].posx > self.sapglob.xj
                    && self.sap3d.aigoual[idx as usize].posy > self.sapglob.yj
                {
                    return false;
                }
            }
            _ => {
                if self.sap3d.aigoual[idx as usize].posx < self.sapglob.xj
                    && self.sap3d.aigoual[idx as usize].posy > self.sapglob.yj
                {
                    return false;
                }
            }
        }
        /* Premier element de la chaine */
        balaye = self.sap3d.aigoual[0].chaine;
        while let Some(b) = balaye {
            if self.sap3d.aigoual[b as usize].distance > dist as u16 {
                poschaine = Some(b);
                balaye = self.sap3d.aigoual[b as usize].chaine;
            } else {
                break;
            }
        }
        match poschaine {
            None => {
                self.sap3d.aigoual[idx as usize].chaine = self.sap3d.aigoual[0].chaine;
                self.sap3d.aigoual[0].chaine = Some(idx);
            }
            Some(p) => {
                self.sap3d.aigoual[idx as usize].chaine = self.sap3d.aigoual[p as usize].chaine;
                self.sap3d.aigoual[p as usize].chaine = Some(idx);
            }
        }
        true
    }
}
/*EOF*/
