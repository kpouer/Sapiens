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

use std::os::raw::{c_char, c_long, c_short, c_uchar, c_ushort};
use std::ptr::{addr_of_mut, null_mut};

use crate::acam::{DisposPtr, NewPtr, NewPtrClear, Random, SetRect, TickCount};
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapwbrt::RemplitZoneLignes;
use crate::sapiens::sapwcvt::Sapwcvt;
use crate::sapiens::{
    ECHELLE, FormeId, IndexColor, MontId, NB_DEGRADE_DISTANCE, NB_ETOILES, NB_LIGNES_CIEL,
    NB_LIGNES_TERRE, NBREVTHEURE, h_chron,
};
use crate::shared::{Ptr, RGBColor, Rect};
use crate::stubs::DisposePtr;
use crate::{Sapiens, port};

const M_PI: f64 = 3.14159265;

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

/*; corresp densite->couleur ecran chaque trame*/
const TAB_TRAM: [c_short; 40] = [
    3, 4, 5, 6, 4, 5, 6, 6, 3, 4, 4, 5, 10, 10, 0, 0, 3, 3, 4, 5, 11, 14, 12, 12, 11, 14, 14, 12,
    11, 11, 14, 12, 10, 0, 0, 0, 10, 10, 0, 0,
];

const TAB_ECL: [u8; 256] = [
    2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2,
    2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 2, 2,
    3, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 2, 2, 2, 1, 1, 1, 0, 0, 1, 1, 1, 2, 2, 2,
    3, 3, 3, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 2, 3,
    3, 3, 3, 3, 2, 2, 2, 1, 0, 0, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 1, 0, 0, 1, 2, 2, 2, 3, 3,
    3, 3, 3, 3, 2, 2, 2, 1, 1, 0, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 1, 1, 0, 1, 2, 2, 2, 3, 3,
    2, 3, 3, 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3, 3, 2, 2, 3, 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3, 3,
    2, 2, 2, 3, 3, 3, 2, 2, 2, 1, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2,
];

/* Les lignes sont organisees du plus loin au plus pres. */
#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
struct LigneSol {
    y: c_short,
    hauteur: c_short,
    largeur: c_short,
    offsetEnMob: c_short, /* Position X dans le Mob qui apparait au milieu de l'écran */
    offsetEcran: c_short,
    dist: c_short,
}

pub(crate) struct Sapw3d {
    TAMP_DENS: [u8; 16],
    DENS_DZET: [u8; 10],
    TAB_PALETTE_DISTANCE: *mut *mut c_short,
    TAB_COULEURS_CIEL: *mut IndexColor,
    COULEUR_ORAGE_JOUR: c_short,
    COULEUR_ORAGE_NUIT: c_short,
    IOTA_PALETTE_STD: f32,
    TAB_ETOILES: *mut Etoile,
    COUL_PULSE: c_short,
    METEORE: c_short,
    X_METEORE: c_short,
    Y_METEORE: c_short,
    LAST_DZETA: c_short,
    SOL_RAND_SEED: c_long,
    TAB_LIGNE_SOL: [LigneSol; NB_INTERV],
}

impl Default for Sapw3d {
    fn default() -> Self {
        Self {
            TAMP_DENS: [0; 16],
            DENS_DZET: [0; 10],
            TAB_PALETTE_DISTANCE: null_mut(),
            TAB_COULEURS_CIEL: null_mut(),
            COULEUR_ORAGE_JOUR: 0,
            COULEUR_ORAGE_NUIT: 0,
            IOTA_PALETTE_STD: 0.14,
            TAB_ETOILES: null_mut(),
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
    x: c_short, /* 0 à AIRE_X_ETOILE */
    y: c_short, /* 0 à 128 */
    magnitude: c_short,
    couleur: c_short,
}

/**************************************************************************
                        sinshort
***************************************************************************/
pub(crate) fn sinshort(angle: c_short) -> c_short {
    (255.0 * (angle as f32 as f64 * 3.1415925 / 512.0).sin()) as c_short
}

/**************************************************************************
                        cosshort
***************************************************************************/
pub(crate) fn cosshort(angle: c_short) -> c_short {
    (255.0 * (angle as f32 as f64 * 3.1415925 / 512.0).cos()) as c_short
}

/*****************************************************************************
                                        distance
*****************************************************************************/
pub(crate) const fn distance(dx: c_long, dy: c_long) -> c_short {
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
    dx as c_short
}

impl Sapiens {
    /************************************************************************
                            caldisI
    => x,y,z, rayon d'un disque
    @= dx,dy,dz distance
    <= erreur (0 ok)
    *************************************************************************/
    unsafe fn caldisI(
        &self,
        x: c_ushort,
        y: c_ushort,
        z: c_ushort,
        rayon: c_ushort,
        pdx: *mut c_short,
        pdy: *mut c_short,
        pdz: *mut c_short,
        pdistance: *mut c_short,
    ) -> c_short {
        let dx = x as c_long - self.sapglob.xj as c_long;
        let dy = y as c_long - self.sapglob.yj as c_long;
        let dz = z as c_long - self.sapglob.zj as c_long;
        if dx.abs() < 32000 && dy.abs() < 32000 {
            *pdx = dx as c_short;
            *pdy = dy as c_short;
            *pdz = dz as c_short;
            let mut dist =
                ((dx as f64 * dx as f64 + dy as f64 * dy as f64).sqrt() - rayon as f64) as c_long;
            if dist < 0 {
                dist = 0;
            }
            if dist < 20000 {
                *pdistance = dist as c_short;
                return 0;
            }
        }
        *pdistance = -1;
        2
    }

    /************************************************************************
                            disk3dI
    => x,y,z, rayon d'un disque
    @= xecr gauche, x ecr droit , distance, delta z, angle
    <= erreur (0 ok)
    *************************************************************************/
    unsafe fn disk3dI(
        &self,
        x: c_ushort,
        y: c_ushort,
        z: c_ushort,
        rayon: c_ushort,
        pxg: *mut c_short,
        pxd: *mut c_short,
        pdz: *mut c_short,
        pdist: *mut c_short,
        pangabs: *mut c_short,
    ) -> c_short {
        let mut erreur: c_short;
        let mut dx: c_short = 0;
        let mut dy: c_short = 0;
        let mut dz: c_short = 0;
        let mut dist: c_short = 0;
        let mut angle: c_short;
        let mut posg: c_short = 0;
        let mut posd: c_short = 0;

        erreur = self.caldisI(x, y, z, rayon, &mut dx, &mut dy, &mut dz, &mut dist);
        if erreur == 0 {
            /* Onzedisk */
            if (dist as i32) < (self.sapglob.dismin as i32) {
                if dist != 0 {
                    dist = 1;
                }
                erreur = 1;
            }
            *pdist = dist;
            /* Angle du centre de la montagne */
            if dx == 0 {
                if dy > 0 {
                    angle = 256;
                } else {
                    angle = 768;
                }
            } else {
                angle = ((f64::atan2(dy as f64, dx as f64) * 512.0) / M_PI) as c_short;
            }
            *pangabs = angle;
            angle = ((angle as i32 - self.sapglob.dzeta as i32) & 1023) as c_short;
            /* Angle occupé par le disque */
            if dist == 0 {
                posg = 0;
                posd = 255;
            } else {
                let rayecr = ((f64::atan2(rayon as f64, dist as f64) * 512.0) / M_PI) as c_short;
                posg = ((angle as i32 + rayecr as i32) & 1023) as c_short;
                posd = ((angle as i32 - rayecr as i32) & 1023) as c_short;
                if posg as i32 > 512 {
                    posg = (posg as i32 - 1024) as c_short;
                }
                if posd as i32 > 512 {
                    posd = (posd as i32 - 1024) as c_short;
                }
                posg = (128 - posg as i32) as c_short;
                posd = (128 - posd as i32) as c_short;
                if posg as i32 > posd as i32 {
                    erreur = 3;
                }
            }
        }
        *pdz = dz;
        *pxg = posg;
        *pxd = posd;
        *pdist = dist;
        erreur
    }
}

/*******************************************
            CalcPoint3D
=> x,y,z joueur, angle visee, x,y,z point
@= x,y ecran, distance
<= VRAI si visible
********************************************/
fn CalcPoint3D(
    pxj: c_long,
    pyj: c_long,
    pzj: c_long,
    ca: f32,
    sa: f32,
    mut xp: c_long,
    mut yp: c_long,
    mut zp: c_long,
    x: *mut c_short,
    y: *mut c_short,
    dist: *mut c_short,
) -> c_short {
    let mut tmp: c_long;
    /* --- Rabat les positions du point en fn du joueur  --- */
    xp -= pxj;
    yp -= pyj;
    zp -= pzj;

    /* --- Recalcule les positions 3d en fonction de l'angle de vue --- */
    tmp = (xp as f32 * ca - yp as f32 * sa) as c_long;
    yp = (xp as f32 * sa + yp as f32 * ca) as c_long;
    xp = tmp;

    /* --- Visible ? --- */
    if xp <= 2 {
        return 0;
    }
    tmp = distance(xp, yp) as c_long;
    unsafe {
        *dist = tmp as c_short;
    }
    if tmp == 0 {
        return 0;
    }
    /* --- Calcule x écran --- */
    let xl = LARGEUR_ECRAN as c_long / 2 - (LARGEUR_ECRAN as c_long / 2) * yp / xp;

    /* --- Calcule Y ecran --- */
    let yl = HAUTEUR_ECRAN as c_long / 2 - (HAUTEUR_ECRAN as c_long / 2) * zp / xp;

    let mut xl = xl;
    let mut yl = yl;
    if xl < -32760 {
        xl = -32760;
    }
    if xl > 32760 {
        xl = 32760;
    }
    if yl < -32760 {
        yl = -32760;
    }
    if yl > 32760 {
        yl = 32760;
    }
    unsafe {
        *x = xl as c_short;
        *y = yl as c_short;
    }

    1
}

impl Sapiens {
    /************************************************************************
    ;-_-_-_-_-_-_-_-_-_-_-_-_ SapDep : deplacement du joueur -_-_-_-_-_-_-_-_
    ; Entrées -C- 1.Angle d'avancée
    ;             2.Vitesse de progression
    ;
    ************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn sapdep(&mut self, angle: c_short, vitesse: c_short) {
        let mut tmp =
            self.sapglob.xj as c_long + (((cosshort(angle) as c_long) * (vitesse as c_long)) >> 8);
        if tmp < 0 {
            tmp = 0;
        }
        if tmp > 65535 {
            tmp = 65535;
        }
        self.sapglob.xj = tmp as c_ushort;
        tmp =
            self.sapglob.yj as c_long + (((sinshort(angle) as c_long) * (vitesse as c_long)) >> 8);
        if tmp < 0 {
            tmp = 0;
        }
        if tmp > 65535 {
            tmp = 65535;
        }
        self.sapglob.yj = tmp as c_ushort;
        self.AvanceSol(vitesse);
    }
}

/****************************************************************************
                    CalculeDegradeDuCiel
Calcul des tableau de degrades du ciel
*****************************************************************************/
const TAB_LIGN: [c_short; 8] = [0, 41 * 2, 48 * 2, 53 * 2, 57 * 2, 59 * 2, 61 * 2, 64 * 2];

impl Sapiens {
    unsafe fn CalculeDegradeDuCiel(
        &self,
        palette: *mut c_short,
        tab_couleurs_ciel: *mut IndexColor,
    ) {
        let mut rgbDep = RGBColor::default();
        let mut rgbDest = RGBColor::default();
        let mut j: i32 = 0;
        let mut incRed: i64 = 0;
        let mut incGreen: i64 = 0;
        let mut incBlue: i64 = 0;

        let _g = port::on(self.sapwind.Ecran1);
        for i in 0..NB_LIGNES_CIEL as i32 {
            /* Changement de ligne */
            if i == TAB_LIGN[j as usize] as i32 {
                Sapwcvt::CvtCouleurEnRgb(palette, j as c_short, &mut rgbDep);
                j += 1;
                Sapwcvt::CvtCouleurEnRgb(palette, j as c_short, &mut rgbDest);
                let dist = TAB_LIGN[j as usize] as i32 - TAB_LIGN[(j - 1) as usize] as i32;
                incRed = (rgbDest.red as i64 - rgbDep.red as i64) / dist as i64;
                incGreen = (rgbDest.green as i64 - rgbDep.green as i64) / dist as i64;
                incBlue = (rgbDest.blue as i64 - rgbDep.blue as i64) / dist as i64;
            }
            rgbDep.red = (rgbDep.red as i64 + incRed) as c_ushort;
            rgbDep.green = (rgbDep.green as i64 + incGreen) as c_ushort;
            rgbDep.blue = (rgbDep.blue as i64 + incBlue) as c_ushort;
            let idx = &mut *tab_couleurs_ciel.add(i as usize);
            idx.rgb = rgbDep;
            idx.index = SapColor2Index(&rgbDep);
        }
    }

    /****************************************************************************
                        CalculeDegradeDistance
    Calcule le degrade des montagnes en fonction de la distance au joueur
    Le premier degrade est celui de premier plan, le dernier celui de l'horizon
    *****************************************************************************/
    unsafe fn CalculeDegradeDistance(&self, h: c_short, tab_palette_distance: *mut *mut c_short) {
        let mut coeff: f32;
        let delta: f32;
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
            let prev = *tab_palette_distance.add(i as usize);
            if !prev.is_null() {
                DisposePtr(prev as Ptr);
            }
            *tab_palette_distance.add(i as usize) =
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

        let _g = port::on(self.sapwind.Ecran1);
        /* Pour l'ensemble des evenements on calcule le degrade */
        let t = addr_of_mut!(self.sapglob.tabheur) as *mut h_chron;
        unsafe {
            for i in 0..NBREVTHEURE as i32 {
                let e = &mut *t.add(i as usize);
                if e.q != 0 {
                    if !e.TabCouleursCiel.is_null() {
                        DisposPtr(e.TabCouleursCiel as Ptr);
                    }
                    if !e.TabPaletteDistance.is_null() {
                        DisposPtr(e.TabPaletteDistance as Ptr);
                    }
                    e.TabCouleursCiel = NewPtrClear(
                        (std::mem::size_of::<IndexColor>() as c_long) * (NB_LIGNES_CIEL as c_long),
                    ) as *mut IndexColor;
                    e.TabPaletteDistance = NewPtrClear(
                        (std::mem::size_of::<*mut c_short>() as c_long)
                            * (NB_DEGRADE_DISTANCE as c_long),
                    ) as *mut *mut c_short;
                    self.CalculeDegradeDuCiel(e.pal.as_mut_ptr(), e.TabCouleursCiel);
                    self.CalculeDegradeDistance(e.h, e.TabPaletteDistance);
                }
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

impl Sapiens {
    /****************************************************************************
                        FixeDegradesFonctionHeure
    En fonctione de l'evenement de changement de l'aire injecte les tabelaux de couleurs
    *****************************************************************************/
    pub(crate) unsafe extern "C" fn FixeDegradesFonctionHeure(&mut self, chron: *mut h_chron) {
        self.sapw3d.TAB_COULEURS_CIEL = (*chron).TabCouleursCiel;
        self.sapw3d.TAB_PALETTE_DISTANCE = (*chron).TabPaletteDistance;
    }

    /****************************************************************************
                        CalculeEtoiles
                Effacement de la 3d
    *****************************************************************************/
    pub(crate) fn CalculeEtoiles(&mut self) {
        let mut rgb = RGBColor::default();

        unsafe {
            let _g = port::on(self.sapwind.Ecran1);
            /* Couleur de l'etoile */
            if !self.sapw3d.TAB_ETOILES.is_null() {
                DisposPtr(self.sapw3d.TAB_ETOILES as Ptr);
            }
            self.sapw3d.TAB_ETOILES =
                NewPtr((NB_ETOILES as c_long) * (std::mem::size_of::<Etoile>() as c_long))
                    as *mut Etoile;
            if self.sapw3d.TAB_ETOILES.is_null() {
                return;
            }
            rgb.red = 65535;
            rgb.green = 65535;
            rgb.blue = 65535;
            self.sapw3d.COUL_PULSE = SapColor2Index(&rgb);
            for i in 0..NB_ETOILES as i64 {
                let e = &mut *self.sapw3d.TAB_ETOILES.add(i as usize);
                e.x = self.random.myRandom(0, AIRE_X_ETOILE as c_long) as c_short;
                e.y = self.random.myRandom(0, AIRE_Y_ETOILE as c_long) as c_short;
                e.magnitude = self.random.myRandom(0, MAX_MAGNITUDE as c_long) as c_short;
                rgb.red = (55000 - e.y as c_long * (32768 / AIRE_Y_ETOILE as c_long)) as c_ushort;
                rgb.green = rgb.red;
                rgb.blue = 0;
                e.couleur = SapColor2Index(&rgb);
            }
        }
    }

    /****************************************************************************
                        AfficheEtoiles
    Affiche les etoiles sur le ciel
    *****************************************************************************/
    pub(crate) fn AfficheEtoiles(&mut self) {
        unsafe {
            if self.sapglob.heure as i32 > 480 || (self.sapglob.heure as i32) < 20 {
                /* Nuit */
                let Some(info) = port::info(port::current()) else {
                    return;
                };
                let data = info.pixels;
                let tailleLigne = info.row_bytes;
                let mut p = self.sapw3d.TAB_ETOILES;
                let magnitude: c_short = if (self.sapglob.heure as i32) < MAX_MAGNITUDE {
                    (MAX_MAGNITUDE - self.sapglob.heure as i32) as c_short
                } else if (self.sapglob.heure as i32) < 480 + MAX_MAGNITUDE {
                    (self.sapglob.heure as i32 - (480 + MAX_MAGNITUDE)) as c_short
                } else {
                    MAX_MAGNITUDE as c_short
                };
                for _ in 0..NB_ETOILES as i64 {
                    if (*p).magnitude <= magnitude {
                        let x = (((self.sapglob.dzeta as i32 * 2 - (*p).x as i32 - 512) & 2047)
                            - 512
                            + 256) as i16;
                        if x >= 0 && x < 512 {
                            let y = (*p).y as i32;
                            let off = (y as i64 * tailleLigne as i64 + x as i64) as isize;
                            let coul = if self.random.Rnd() > 127 {
                                self.sapw3d.COUL_PULSE
                            } else {
                                (*p).couleur
                            };
                            *data.offset(off) = coul as u8;
                        }
                    }
                    p = p.add(1);
                }
                if self.sapw3d.METEORE != 0 {
                    let x = (((self.sapglob.dzeta as i32 * 2 - self.sapw3d.X_METEORE as i32 - 512)
                        & 2047)
                        - 512
                        + 256) as c_short;
                    self.Affmobbrt(
                        self.sapwind.mob3D + 21,
                        x / ECHELLE as c_short,
                        self.sapw3d.Y_METEORE / ECHELLE as c_short,
                        0,
                        null_mut(),
                    );
                    self.sapw3d.METEORE = (self.sapw3d.METEORE as i32 - 1) as c_short;
                    self.sapw3d.X_METEORE = (self.sapw3d.X_METEORE as i32 + 5) as c_short;
                    self.sapw3d.Y_METEORE = (self.sapw3d.Y_METEORE as i32 + 5) as c_short;
                } else if self.random.myRandom(0, 1000) > 990 {
                    self.sapw3d.X_METEORE =
                        self.random.myRandom(0, AIRE_X_ETOILE as c_long) as c_short;
                    self.sapw3d.Y_METEORE =
                        self.random.myRandom(0, (AIRE_Y_ETOILE / 2) as c_long) as c_short;
                    self.sapw3d.METEORE = self.random.myRandom(3, 6) as c_short;
                }
                /* Etoile polaire */
                let x = (((self.sapglob.dzeta as i32 * 2 + 512) % 2048) - 512 + 256) as c_short;
                let num =
                    (self.sapwind.mob3D as i32 + self.random.myRandom(22, 24) as i32) as c_short;
                self.Affmobbrt(num, x / 2, 20, 0, null_mut());
            }
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
                as c_short;
            self.sapw3d.TAB_LIGNE_SOL[i].largeur = (LARGEUR_MOB
                * self.sapw3d.TAB_LIGNE_SOL[i].hauteur as i64
                / HAUTEUR_MOB) as c_short;
        }
        self.sapw3d.TAB_LIGNE_SOL[0].hauteur = self.sapw3d.TAB_LIGNE_SOL[0].y;
        self.sapw3d.TAB_LIGNE_SOL[0].largeur =
            (LARGEUR_MOB * self.sapw3d.TAB_LIGNE_SOL[0].hauteur as i64 / HAUTEUR_MOB) as c_short;
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
                        ((self.datum(delta, i)) % LARGEUR_MOB as i32) as c_short;
                    self.sapw3d.TAB_LIGNE_SOL[i].offsetEcran =
                        (-(self.sapw3d.TAB_LIGNE_SOL[i].offsetEnMob as i64)
                            * self.sapw3d.TAB_LIGNE_SOL[i].hauteur as i64
                            / HAUTEUR_MOB) as c_short;
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
    const fn CalcY(&self, idx: c_short) -> c_short {
        let dist = self.sapw3d.TAB_LIGNE_SOL[idx as usize].dist as i64;
        (256 * (38 + self.sapglob.zj as i64 / 20) / dist) as c_short
    }

    /*******************************************************************************
                                                                    AvanceSol
                                    Le personnage avance, le tableau des lignes du sol bouge
    *********************************************************************************/
    pub(crate) fn AvanceSol(&mut self, dist: c_short) {
        if dist != 0 {
            self.sapw3d.SOL_RAND_SEED =
                TickCount() + self.sapglob.xj as c_long + self.sapglob.yj as c_long;
        }
        /* Est-ce l'init ? oui -> on calcule tout*/
        if self.sapw3d.TAB_LIGNE_SOL[0].dist == 0 {
            let mut d: i32 = 70;
            for i in 0..NB_INTERV {
                self.sapw3d.TAB_LIGNE_SOL[NB_INTERV - 1 - i].dist = d as c_short;
                self.sapw3d.TAB_LIGNE_SOL[NB_INTERV - 1 - i].y =
                    self.CalcY((NB_INTERV - 1 - i) as c_short);
                d += LARGEUR_BANDE_SOL;
            }
        } else {
            for i in 0..NB_INTERV {
                self.sapw3d.TAB_LIGNE_SOL[i].dist =
                    (self.sapw3d.TAB_LIGNE_SOL[i].dist as i32 - dist as i32) as c_short;
                self.sapw3d.TAB_LIGNE_SOL[i].y = self.CalcY(i as c_short);
            }
            /* La ligne la plus proche s'est-elle barrée ? */
            while self.sapw3d.TAB_LIGNE_SOL[NB_INTERV - 1].dist < 70 {
                /* On décale le tableau */
                for i in (1..NB_INTERV).rev() {
                    self.sapw3d.TAB_LIGNE_SOL[i] = self.sapw3d.TAB_LIGNE_SOL[i - 1];
                }
                /* On recalcule le dernier élément */
                self.sapw3d.TAB_LIGNE_SOL[0].dist =
                    (self.sapw3d.TAB_LIGNE_SOL[1].dist as i32 + LARGEUR_BANDE_SOL) as c_short;
                self.sapw3d.TAB_LIGNE_SOL[0].y = self.CalcY(0);
            }
        }
    }

    /*******************************************************************************
                                                                    AfficheSol
                                    On affiche les lignes du sol
    *********************************************************************************/
    unsafe fn AfficheSol(&mut self) {
        let mut r: Rect = Rect::default();
        let mut rdep: Rect;
        let mut i: i32;
        let mut numPalette: i32;
        let numMob: c_short;
        let ymax: i32;
        let largeur: i32;
        let RandSeedBak: c_long;

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
        if i < NB_INTERV as i32 {
            ymax = self.sapw3d.TAB_LIGNE_SOL[i as usize].y as i32 + NB_LIGNES_CIEL as i32;
        } else {
            ymax = NB_LIGNES_CIEL as i32 + NB_LIGNES_TERRE as i32;
        }
        largeur = HAUTEUR_MIN_BANDE * LARGEUR_MOB as i32 * 4 / HAUTEUR_MOB as i32;
        RandSeedBak = self.random.CarboniseGetRandSeed();
        self.random.CarboniseSetRandSeed(self.sapw3d.SOL_RAND_SEED);
        let mut j = ymax;
        while j > NB_LIGNES_CIEL as i32 {
            /* des bandes de 4 */
            let offset = ((2048 - self.sapglob.dzeta as i32 * 2 + self.random.Rnd() as i32)
                % largeur) as c_short;
            let top = if j < NB_LIGNES_CIEL as i32 + HAUTEUR_MIN_BANDE {
                NB_LIGNES_CIEL as i32
            } else {
                j - HAUTEUR_MIN_BANDE
            };
            SetRect(
                &mut r,
                (256 - offset as i32) as c_short,
                top as c_short,
                (256 + largeur - offset as i32) as c_short,
                j as c_short,
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
                self.Affmob3d(
                    self.sapwind.mob3D + 25,
                    &mut r,
                    *self.sapw3d.TAB_PALETTE_DISTANCE.add(numPalette as usize),
                );
                r.OffsetRect(largeur as c_short, 0);
                k += largeur;
            }
            r = rdep;
            k = 256 - offset as i32;
            while k > 0 {
                r.OffsetRect(-largeur as c_short, 0);
                self.Affmob3d(
                    self.sapwind.mob3D + 25,
                    &mut r,
                    *self.sapw3d.TAB_PALETTE_DISTANCE.add(numPalette as usize),
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
                SetRect(
                    &mut r,
                    (256 - offset) as c_short,
                    top as c_short,
                    (256 + largeur2 - offset) as c_short,
                    bottom as c_short,
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
                    self.Affmob3d(
                        nmob,
                        &mut r,
                        *self.sapw3d.TAB_PALETTE_DISTANCE.add(numPalette as usize),
                    );
                    r.OffsetRect(largeur2 as c_short, 0);
                    kk += largeur2;
                }
                r = rdep;
                kk = 256 - offset;
                while kk > 0 {
                    r.OffsetRect(-largeur2 as c_short, 0);
                    self.Affmob3d(
                        nmob,
                        &mut r,
                        *self.sapw3d.TAB_PALETTE_DISTANCE.add(numPalette as usize),
                    );
                    kk -= largeur2;
                }
            }
            i += 1;
        }
        self.random.CarboniseSetRandSeed(RandSeedBak);
    }
}

/******************************************************************
                    TrameCielOrage
******************************************************************/
unsafe fn TrameCielOrage(index: c_short) {
    /* Origine des données de l'ecran */
    let Some(info) = port::info(port::current()) else {
        return;
    };
    let data = info.pixels;
    let tailleLigne = info.row_bytes;
    let mut d = data;
    let mut i = NB_LIGNES_CIEL as i32 / 2;
    while i > 0 {
        i -= 1;
        let mut j = tailleLigne as i32 / 2;
        while j > 0 {
            j -= 1;
            *d = index as u8;
            *d.offset((tailleLigne as i64 + 1) as isize) = index as u8;
            d = d.add(2);
        }
        d = d.add(tailleLigne as usize);
    }
}

/******************************************************************
                    Flou
******************************************************************/
unsafe fn Flou() {
    /* Origine des données de l'ecran */
    let Some(info) = port::info(port::current()) else {
        return;
    };
    let data = info.pixels;
    let tailleLigne = info.row_bytes;
    for y in 0..200i32 {
        let mut p = data.add((tailleLigne as i64 * y as i64) as usize);
        let mut x = 0;
        while x < tailleLigne as i32 {
            *p = *p.add(1);
            p = p.add(3);
            x += 3;
        }
    }
}

/******************************************************************
                    TracePluie
=> index de la couleur, nombre de gouttes
******************************************************************/
unsafe fn TracePluie(random: &mut Random, index: c_short, n: c_short) {
    if random.myRandom(0, 1000) > 500 {
        Flou();
    }
    /* Origine des données de l'ecran */
    let Some(info) = port::info(port::current()) else {
        return;
    };
    let data = info.pixels;
    let tailleLigne = info.row_bytes;
    let mut n = n as i32;
    while n > 0 {
        n -= 1;
        let x = random.myRandom(0, tailleLigne as c_long) as c_short;
        let y = random.myRandom(0, 150) as c_short;
        let mut j = 50;
        let mut p = data.add((tailleLigne as i64 * y as i64 + x as i64) as usize);
        while j > 0 {
            j -= 1;
            *p = index as u8;
            p = p.add((tailleLigne as i64 - 1) as usize);
        }
    }
}

impl Sapiens {
    /****************************************************************************
                        eff3d
                Effacement de la 3d
    *****************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn eff3d(
        &mut self,
        _ciel: c_uchar,  /* NU */
        _terre: c_uchar, /* NU */
        ymeteore: c_ushort,
        xmeteore: c_ushort,
        isLune: c_ushort,
    ) {
        let _g = port::on(self.sapwind.Ecran1);

        /* Le degrade du ciel */
        /* Un orage est en cours */
        RemplitZoneLignes(0, NB_LIGNES_CIEL as c_short, self.sapw3d.TAB_COULEURS_CIEL);
        self.AfficheEtoiles();
        /* Le meteore */
        self.Affmobbrt(
            (crate::MOB_SOLEIL!(self.sapwind) as i32 + isLune as i32) as c_short,
            xmeteore as c_short,
            ymeteore as c_short,
            0,
            null_mut(),
        );
        if self.sap3d.FreqEclair < 1000 {
            if isLune != 0 {
                TrameCielOrage(self.sapw3d.COULEUR_ORAGE_NUIT);
            } else {
                TrameCielOrage(self.sapw3d.COULEUR_ORAGE_JOUR);
            }
        }
        /* Le sol */
        self.AfficheSol();
    }

    /***************************************************************************************
                                     Pluie
    ***************************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn Pluie(&mut self) {
        let _g = port::on(self.sapwind.Ecran1);

        if self.sap3d.FreqEclair < 1000 && self.sap3d.FreqEclair > 500 {
            let n = if self.sap3d.FreqEclair > 750 {
                1000 - self.sap3d.FreqEclair
            } else {
                self.sap3d.FreqEclair - 500
            };
            if self.sapglob.heure & 0x200 != 0 {
                TracePluie(
                    &mut self.random,
                    self.sapw3d.COULEUR_ORAGE_NUIT,
                    n as c_short,
                );
            } else {
                TracePluie(
                    &mut self.random,
                    self.sapw3d.COULEUR_ORAGE_JOUR,
                    n as c_short,
                );
            }
            /* Si pas deja de l'eau par terre on mets de l'eau a terre */
            if self.sapglob.liste4.taille > 0
                && self.sapglob.liste4.elements[self.sapglob.liste4.taille as usize - 1] != 2
                && self.sapglob.liste4.taille < self.sapglob.liste4.maxtaille
            {
                self.sapglob.liste4.taille = self.sapglob.liste4.taille.wrapping_add(1);
                self.sapglob.tlpos[4] = self.sapglob.liste4.taille as c_char;
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
                self.sapglob.tlpos[4] = self.sapglob.liste4.taille as c_char;
                self.sapglob.liste4.elements[self.sapglob.liste4.taille as usize - 1] = 2; /*Eau */
            }
        }
    }

    /***************************************************************************************
                                                    CalculeAltitude
                                    Calcule l'altitude d'un point du plan
    ***************************************************************************************/
    fn CalculeAltitude(&self, _x: c_short, _y: c_short) -> c_short {
        let mut z: i32 = 40;

        for mont in &self.sap3d.aigoual {
            if let Some(forme) = mont.forme.map(FormeId::forme) {
                if forme.hauteur == 0 {
                    let d = distance(
                        self.sapglob.xj as c_long - mont.posx as c_long,
                        self.sapglob.yj as c_long - mont.posy as c_long,
                    );
                    let r = forme.rayon as i64;
                    if (d as i64) < r {
                        z += ((r - d as i64) * forme.nbrdisk as i64 * mont.ign as i64 / r) as i32;
                    }
                }
            }
        }
        z as c_short
    }

    /*****************************************************************************************
                                                    Visu :
    Calcul et visualisation d'un objet 3d
    L'affichage se fait sur ecran 1
    ******************************************************************************************/
    pub(crate) unsafe fn visu(&mut self) {
        let mut x: c_short = 0;
        let mut y: c_short = 0;
        let mut dist: c_short = 0;
        let mut rd: c_short;
        let mut r: Rect = Rect::default();

        let _g = port::on(self.sapwind.Ecran1);
        self.sapglob.zj = self
            .CalculeAltitude(self.sapglob.xj as c_short, self.sapglob.yj as c_short)
            as c_ushort;
        let angle = ((self.sapglob.dzeta as f32) * (3.1415927 as f32)) as f64 / 512.0;
        let ca = (-angle).cos() as f32;
        let sa = (-angle).sin() as f32;

        for m in self.sap3d.chaine() {
            let Some(forme) = m.forme.map(FormeId::forme) else {
                continue;
            };
            if CalcPoint3D(
                self.sapglob.xj as c_long,
                self.sapglob.yj as c_long,
                self.sapglob.zj as c_long,
                ca,
                sa,
                m.posx as c_long,
                m.posy as c_long,
                forme.hauteur as c_long,
                &mut x,
                &mut y,
                &mut dist,
            ) != 0
            {
                rd = (forme.rayon as c_long * 2 * 256 / dist as c_long) as c_short;
                let hauteurDisque =
                    (m.ign as c_long * forme.nbrdisk as c_long * 256 / dist as c_long) as c_short;
                SetRect(
                    &mut r,
                    (x as i32 - rd as i32 / 2) as c_short,
                    (y as i32 - hauteurDisque as i32) as c_short,
                    (x as i32 + rd as i32 / 2) as c_short,
                    y,
                );
                if !r.is_empty()
                    && r.left < LARGEUR_ECRAN as c_short
                    && r.right > 0
                    && r.bottom > 0
                    && r.top < HAUTEUR_ECRAN as c_short
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
                    self.Affmob3d(
                        self.sapwind.mob3D + forme.numMob3d,
                        &mut r,
                        *self.sapw3d.TAB_PALETTE_DISTANCE.add(numPalette as usize),
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
            self.sap3d.aigoual[idx as usize].posx as c_long - self.sapglob.xj as c_long,
            self.sap3d.aigoual[idx as usize].posy as c_long - self.sapglob.yj as c_long,
        );
        self.sap3d.aigoual[idx as usize].distance = dist as c_ushort;
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
            if self.sap3d.aigoual[b as usize].distance > dist as c_ushort {
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
