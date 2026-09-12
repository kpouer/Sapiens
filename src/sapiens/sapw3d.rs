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

use crate::acam::{
    AcamGetInfoOffscreen, AcamGetPortOffscreen, AcamSetPortOffscreen, CarboniseGetCurrentPort,
    CarboniseGetRandSeed, CarboniseSetRandSeed, DisposPtr, EmptyRect, NewPtr, NewPtrClear,
    OffsetRect, SetRect, TickCount,
};
use crate::sapiens::sap3d::{FreqEclair, affich, aigoual, nbreobj};
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapglob::{
    dervue, dismin, dzeta, heure, liste4, metpos, tabheur, tlpos, xj, yj, zj,
};
use crate::sapiens::sapwarb::tabcoul;
use crate::sapiens::sapwbrt::{
    Affmob3d, Affmobbrt, CalculePaletteAttenuee, RemplitZoneLignes, TraceDisque, tfrima,
};
use crate::sapiens::sapwcvt::CvtCouleurEnRgb;
use crate::sapiens::sapwind::{Ecran1, Rnd, mob3D, myRandom};
use crate::sapiens::{
    ECHELLE, IndexColor, MontPtr, NB_DEGRADE_DISTANCE, NB_ETOILES, NB_LIGNES_CIEL, NB_LIGNES_TERRE,
    NBREVTHEURE, VUE_PAYSAGE, h_chron,
};
use crate::shared::{Ptr, RGBColor, Rect};
use crate::stubs::DisposePtr;

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

const TAB_ECL: [c_uchar; 256] = [
    2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2,
    2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 2, 2,
    3, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 2, 2, 2, 1, 1, 1, 0, 0, 1, 1, 1, 2, 2, 2,
    3, 3, 3, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 2, 3,
    3, 3, 3, 3, 2, 2, 2, 1, 0, 0, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 1, 0, 0, 1, 2, 2, 2, 3, 3,
    3, 3, 3, 3, 2, 2, 2, 1, 1, 0, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 1, 1, 0, 1, 2, 2, 2, 3, 3,
    2, 3, 3, 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3, 3, 2, 2, 3, 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3, 3,
    2, 2, 2, 3, 3, 3, 2, 2, 2, 1, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2,
];

static mut TAMP_DENS: [c_char; 16] = [0; 16];
static mut DENS_DZET: [c_char; 10] = [0; 10];
static mut TAB_PALETTE_DISTANCE: *mut *mut c_short = null_mut();
static mut TAB_COULEURS_CIEL: *mut IndexColor = null_mut();
static mut COULEUR_ORAGE_JOUR: c_short = 0;
static mut COULEUR_ORAGE_NUIT: c_short = 0;
static mut IOTA_PALETTE_STD: f32 = 0.14;
static mut TAB_ETOILES: *mut Etoile = null_mut();
static mut COUL_PULSE: c_short = 0;
static mut METEORE: c_short = 0;
static mut X_METEORE: c_short = 0;
static mut Y_METEORE: c_short = 0;
static mut LAST_DZETA: c_short = 0;
static mut SOL_RAND_SEED: c_long = 0x2548795A;

/* Les lignes sont organisees du plus loin au plus pres. */
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct LigneSol {
    y: c_short,
    hauteur: c_short,
    largeur: c_short,
    offsetEnMob: c_short, /* Position X dans le Mob qui apparait au milieu de l'écran */
    offsetEcran: c_short,
    dist: c_short,
}
const LIGNE_SOL_INIT: LigneSol = LigneSol {
    y: 0,
    hauteur: 0,
    largeur: 0,
    offsetEnMob: 0,
    offsetEcran: 0,
    dist: 0,
};
static mut TAB_LIGNE_SOL: [LigneSol; NB_INTERV] = [LIGNE_SOL_INIT; NB_INTERV];

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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinshort(angle: c_short) -> c_short {
    (255.0 * (angle as f32 as f64 * 3.1415925 / 512.0).sin()) as c_short
}

/**************************************************************************
                        cosshort
***************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cosshort(angle: c_short) -> c_short {
    (255.0 * (angle as f32 as f64 * 3.1415925 / 512.0).cos()) as c_short
}

/*****************************************************************************
                                        distance
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn distance(dx: c_long, dy: c_long) -> c_short {
    let mut dx = dx.abs();
    let dy = dy.abs();
    if dx > dy {
        dx = dx + dy * 2 / 5;
    } else {
        dx = dy + dx * 2 / 5;
    }
    if dx > 32700 {
        dx = 32700;
    }
    dx as c_short
}

/************************************************************************
                        caldisI
=> x,y,z, rayon d'un disque
@= dx,dy,dz distance
<= erreur (0 ok)
*************************************************************************/
unsafe fn caldisI(
    x: c_ushort,
    y: c_ushort,
    z: c_ushort,
    rayon: c_ushort,
    pdx: *mut c_short,
    pdy: *mut c_short,
    pdz: *mut c_short,
    pdistance: *mut c_short,
) -> c_short {
    let dx = x as c_long - xj as c_long;
    let dy = y as c_long - yj as c_long;
    let dz = z as c_long - zj as c_long;
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
    let mut erreur: c_short = 0;
    let mut dx: c_short = 0;
    let mut dy: c_short = 0;
    let mut dz: c_short = 0;
    let mut dist: c_short = 0;
    let mut angle: c_short = 0;
    let mut posg: c_short = 0;
    let mut posd: c_short = 0;

    erreur = caldisI(x, y, z, rayon, &mut dx, &mut dy, &mut dz, &mut dist);
    if erreur == 0 {
        /* Onzedisk */
        if (dist as i32) < (dismin as i32) {
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
        angle = ((angle as i32 - dzeta as i32) & 1023) as c_short;
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

/*************************************************************************
;-_-_-_-_-_-_-_-_-_- Calcoul: calcule tableau coul -_-_-_-_-_-_-_-_-_-_
; => d0: n. trame
**************************************************************************/
unsafe fn calcoul(trame: c_short) {
    let tc = addr_of_mut!(tabcoul) as *mut c_short;
    for i in 0..9 {
        let n = DENS_DZET[i] as i32;
        *tc.add(i + 1) = TAB_TRAM[(trame as i32 * 4 + n) as usize] as c_short;
    }
}

/*****************************************************************************
;-_-_-_-_-_-_-_-_-_ Visu: visualise montagnes 3D -_-_-_-_-_-_-_-_-_-_-_
;=> (c) pointeur vers debut structure montagne
;<= montagnes   sur ecran
; WARNING !!! Taille de la structure modifiée par le 68000 !!!!!
******************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn visu2(mut mont: MontPtr) {
    let mut j: c_short;
    let mut i: i32;
    let mut ligne: c_short = 0;
    let mut gauche: c_short = 0;
    let mut droite: c_short = 0;
    let mut newgauche: c_short = 0;
    let mut newdroite: c_short = 0;
    let mut dx: c_short = 0;
    let mut dz: c_short = 0;
    let mut dist: c_short = 0;
    let mut angle: c_short = 0;
    let mut hauteurDisque: c_short = 0;
    let mut err: c_short = 0;
    let mut xd: c_short = 0;
    let mut yd: c_short = 0;
    let mut zd: c_short = 0;
    let mut rd: c_short = 0;
    let mut hmec: c_short = 10;
    let mut angabs: c_short = 0;
    let mut gbak: *mut std::os::raw::c_void = null_mut();

    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);
    /* Calcule le tableau d'ensoleillement en fonction du jour et de la nuit
    Jour ou Nuit */
    // d1=((((76-metpos)&0xFF)/6)<<4)&240; if(heure&256) j=d1; else j=d1+8;
    let jj = (((76 - metpos as i32) & 0xFF) / 6) << 4 & 240;
    if heure & 256 != 0 {
        j = jj as c_short;
    } else {
        j = (jj as i32 + 8) as c_short;
    }
    for i in 0..16i32 {
        TAMP_DENS[i as usize] = TAB_ECL[j as usize] as c_char;
        j = (j as i32 + 1) as c_short;
    }

    hmec = 10;
    while !mont.is_null() {
        let forme = (*mont).ptrforme;
        let mut oscil: c_short = 0;
        xd = (*mont).posx as c_short;
        yd = (*mont).posy as c_short;
        zd = (*forme).hauteur as c_short;
        rd = (*forme).rayon as c_short;
        err = disk3dI(
            xd as c_ushort,
            yd as c_ushort,
            zd as c_ushort,
            rd as c_ushort,
            &mut gauche,
            &mut droite,
            &mut dz,
            &mut dist,
            &mut angabs,
        );
        /* Calcul des couleurs */
        j = (((angabs as i32 - 480) & 1023) >> 6) as c_short;
        /* 8 couleurs vues par le joueur */
        for i in 0..9i32 {
            DENS_DZET[i as usize] = TAMP_DENS[(j as i32 & 15) as usize];
            j = (j as i32 + 1) as c_short;
        }
        calcoul((*forme).relief as c_short);
        /* Pas d'erreur */
        angle =
            ((((f64::atan2(dz as f64, dist as f64) * 512.0) / M_PI) as c_short) & 1023) as c_short;
        if angle as i32 > 512 {
            angle = (angle as i32 - 1024) as c_short;
        }
        ligne = 64 - angle as i32 as c_short;
        hauteurDisque = ((((f64::atan2((*mont).ign as f64, dist as f64) * 512.0) / M_PI)
            as c_short)
            & 1023) as c_short;
        /* On boucle pour tous les disques */
        i = 0;
        while i < (*forme).nbrdisk as i32 && err < 2 {
            if err != 0 {
                hmec = (hmec as i32 + (*mont).ign as i32) as c_short;
                ligne = 100;
            }
            err = 0;
            if ligne < 320 {
                zd = (zd as i32 + (*mont).ign as i32) as c_short;
                rd = (*forme).dskrayon[i as usize] as c_short;
                dx = (*forme).dskdelta[i as usize * 2] as c_short;
                /* Changement de trame */
                if (dx as i32 & 255) == 128 {
                    dx = (*forme).dskdelta[i as usize * 2 + 1] as c_short;
                    /* On calcule la trame */
                    calcoul(dx);
                    /* Et pas de delta */
                    xd = (*mont).posx as c_short;
                    yd = (*mont).posy as c_short;
                } else {
                    /* On applique les delta */
                    xd = ((*mont).posx as i32 + dx as i32) as c_short;
                    yd = ((*mont).posy as i32 + (*forme).dskdelta[i as usize * 2 + 1] as i32)
                        as c_short;
                }
                err = disk3dI(
                    xd as c_ushort,
                    yd as c_ushort,
                    zd as c_ushort,
                    rd as c_ushort,
                    &mut newgauche,
                    &mut newdroite,
                    &mut dz,
                    &mut dist,
                    &mut angabs,
                );
                if err != 0 {
                    gauche = newgauche;
                    droite = newdroite;
                } else {
                    if hauteurDisque == 0 {
                        oscil ^= 1;
                        if oscil != 0 {
                            TraceDisque(
                                &mut gauche,
                                &mut droite,
                                &mut ligne,
                                newgauche,
                                newdroite,
                                1,
                            );
                        }
                    } else {
                        TraceDisque(
                            &mut gauche,
                            &mut droite,
                            &mut ligne,
                            newgauche,
                            newdroite,
                            hauteurDisque,
                        );
                    }
                }
            }
            i += 1;
        }
        mont = (*mont).chaine;
    }
    zj = hmec as c_ushort;
    AcamSetPortOffscreen(gbak);
}

/*******************************************
            CalcPoint3D
=> x,y,z joueur, angle visee, x,y,z point
@= x,y ecran, distance
<= VRAI si visible
********************************************/
unsafe fn CalcPoint3D(
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
    *dist = tmp as c_short;
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
    *x = xl as c_short;
    *y = yl as c_short;

    1
}

/************************************************************************
;-_-_-_-_-_-_-_-_-_-_-_-_ SapDep : deplacement du joueur -_-_-_-_-_-_-_-_
; Entrées -C- 1.Angle d'avancée
;             2.Vitesse de progression
;
************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sapdep(angle: c_short, vitesse: c_short) {
    let mut tmp = xj as c_long + (((cosshort(angle) as c_long) * (vitesse as c_long)) >> 8);
    if tmp < 0 {
        tmp = 0;
    }
    if tmp > 65535 {
        tmp = 65535;
    }
    xj = tmp as c_ushort;
    tmp = yj as c_long + (((sinshort(angle) as c_long) * (vitesse as c_long)) >> 8);
    if tmp < 0 {
        tmp = 0;
    }
    if tmp > 65535 {
        tmp = 65535;
    }
    yj = tmp as c_ushort;
    AvanceSol(vitesse);
}

/****************************************************************************
                    CalculeDegradeDuCiel
Calcul des tableau de degrades du ciel
*****************************************************************************/
const TAB_LIGN: [c_short; 8] = [0, 41 * 2, 48 * 2, 53 * 2, 57 * 2, 59 * 2, 61 * 2, 64 * 2];

unsafe fn CalculeDegradeDuCiel(palette: *mut c_short, tab_couleurs_ciel: *mut IndexColor) {
    let mut rgbDep = RGBColor::default();
    let mut rgbDest = RGBColor::default();
    let mut j: i32 = 0;
    let mut incRed: i64 = 0;
    let mut incGreen: i64 = 0;
    let mut incBlue: i64 = 0;
    let mut gbak: *mut std::os::raw::c_void = null_mut();

    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);
    for i in 0..NB_LIGNES_CIEL as i32 {
        /* Changement de ligne */
        if i == TAB_LIGN[j as usize] as i32 {
            CvtCouleurEnRgb(palette, j as c_short, &mut rgbDep);
            j += 1;
            CvtCouleurEnRgb(palette, j as c_short, &mut rgbDest);
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
        idx.index = SapColor2Index(&mut rgbDep);
    }
    AcamSetPortOffscreen(gbak);
}

/****************************************************************************
                    CalculeDegradeDistance
Calcule le degrade des montagnes en fonction de la distance au joueur
Le premier degrade est celui de premier plan, le dernier celui de l'horizon
*****************************************************************************/
unsafe fn CalculeDegradeDistance(h: c_short, tab_palette_distance: *mut *mut c_short) {
    let mut coeff: f32;
    let delta: f32;
    if h as i32 > 512 - 50 || (h as i32) < 50 {
        /* Nuit */
        let pos = if (h as i32) < 50 {
            h as i32
        } else {
            512 - h as i32
        };
        coeff = (0.7 + (pos as f32) as f64 / 200.0 + IOTA_PALETTE_STD as f64) as f32;
        delta = (0.5 / NB_DEGRADE_DISTANCE as f64) as f32;
    } else {
        /* c'est le jour */
        coeff = (1.0 + IOTA_PALETTE_STD as f64) as f32;
        delta = (0.4 / NB_DEGRADE_DISTANCE as f64) as f32;
    }
    for i in 0..NB_DEGRADE_DISTANCE as i32 {
        let prev = *tab_palette_distance.add(i as usize);
        if !prev.is_null() {
            DisposePtr(prev as Ptr);
        }
        *tab_palette_distance.add(i as usize) = CalculePaletteAttenuee(mob3D + 1, coeff, 33);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculeToutDegrades() {
    let mut gbak: *mut std::os::raw::c_void = null_mut();
    let mut rgb = RGBColor::default();

    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);
    /* Pour l'ensemble des evenements on calcule le degrade */
    let t = addr_of_mut!(tabheur) as *mut h_chron;
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
                (std::mem::size_of::<*mut c_short>() as c_long) * (NB_DEGRADE_DISTANCE as c_long),
            ) as *mut *mut c_short;
            CalculeDegradeDuCiel(e.pal.as_mut_ptr(), e.TabCouleursCiel);
            CalculeDegradeDistance(e.h, e.TabPaletteDistance);
        }
    }
    rgb.red = 20000;
    rgb.green = 20000;
    rgb.blue = 20000;
    COULEUR_ORAGE_JOUR = SapColor2Index(&mut rgb);
    rgb.red = 8000;
    rgb.green = 8000;
    rgb.blue = 8000;
    COULEUR_ORAGE_NUIT = SapColor2Index(&mut rgb);
    AcamSetPortOffscreen(gbak);
}

/****************************************************************************
                    ChangeIotaPalette
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeIotaPalette(combien: f32) {
    IOTA_PALETTE_STD += combien;
    CalculeToutDegrades();
    if dervue == VUE_PAYSAGE as c_uchar {
        affich(dzeta as c_ushort);
        tfrima();
    }
}

/****************************************************************************
                    FixeDegradesFonctionHeure
En fonctione de l'evenement de changement de l'aire injecte les tabelaux de couleurs
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixeDegradesFonctionHeure(chron: *mut h_chron) {
    TAB_COULEURS_CIEL = (*chron).TabCouleursCiel;
    TAB_PALETTE_DISTANCE = (*chron).TabPaletteDistance;
}

/****************************************************************************
                    CalculeEtoiles
            Effacement de la 3d
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculeEtoiles() {
    let mut gbak: *mut std::os::raw::c_void = null_mut();
    let mut rgb = RGBColor::default();

    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);
    /* Couleur de l'etoile */
    if !TAB_ETOILES.is_null() {
        DisposPtr(TAB_ETOILES as Ptr);
    }
    TAB_ETOILES =
        NewPtr((NB_ETOILES as c_long) * (std::mem::size_of::<Etoile>() as c_long)) as *mut Etoile;
    if TAB_ETOILES.is_null() {
        return;
    }
    rgb.red = 65535;
    rgb.green = 65535;
    rgb.blue = 65535;
    COUL_PULSE = SapColor2Index(&mut rgb);
    for i in 0..NB_ETOILES as i64 {
        let e = &mut *TAB_ETOILES.add(i as usize);
        e.x = myRandom(0, AIRE_X_ETOILE as c_long) as c_short;
        e.y = myRandom(0, AIRE_Y_ETOILE as c_long) as c_short;
        e.magnitude = myRandom(0, MAX_MAGNITUDE as c_long) as c_short;
        rgb.red = (55000 - e.y as c_long * (32768 / AIRE_Y_ETOILE as c_long)) as c_ushort;
        rgb.green = rgb.red;
        rgb.blue = 0;
        e.couleur = SapColor2Index(&mut rgb);
    }
    AcamSetPortOffscreen(gbak);
}

/****************************************************************************
                    AfficheEtoiles
Affiche les etoiles sur le ciel
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AfficheEtoiles() {
    if heure as i32 > 480 || (heure as i32) < 20 {
        /* Nuit */
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            null_mut(),
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        let mut p = TAB_ETOILES;
        let magnitude: c_short = if (heure as i32) < MAX_MAGNITUDE {
            (MAX_MAGNITUDE - heure as i32) as c_short
        } else if (heure as i32) < 480 + MAX_MAGNITUDE {
            (heure as i32 - (480 + MAX_MAGNITUDE)) as c_short
        } else {
            MAX_MAGNITUDE as c_short
        };
        for _ in 0..NB_ETOILES as i64 {
            if (*p).magnitude <= magnitude {
                let x = (((dzeta as i32 * 2 - (*p).x as i32 - 512) & 2047) - 512 + 256) as i16;
                if x >= 0 && x < 512 {
                    let y = (*p).y as i32;
                    let off = (y as i64 * tailleLigne as i64 + x as i64) as isize;
                    let coul = if Rnd() > 127 {
                        COUL_PULSE
                    } else {
                        (*p).couleur
                    };
                    *data.offset(off) = coul as u8;
                }
            }
            p = p.add(1);
        }
        if METEORE != 0 {
            let x = (((dzeta as i32 * 2 - X_METEORE as i32 - 512) & 2047) - 512 + 256) as c_short;
            Affmobbrt(
                mob3D + 21,
                x / ECHELLE as c_short,
                Y_METEORE / ECHELLE as c_short,
                0,
                null_mut(),
            );
            METEORE = (METEORE as i32 - 1) as c_short;
            X_METEORE = (X_METEORE as i32 + 5) as c_short;
            Y_METEORE = (Y_METEORE as i32 + 5) as c_short;
        } else if myRandom(0, 1000) > 990 {
            X_METEORE = myRandom(0, AIRE_X_ETOILE as c_long) as c_short;
            Y_METEORE = myRandom(0, (AIRE_Y_ETOILE / 2) as c_long) as c_short;
            METEORE = myRandom(3, 6) as c_short;
        }
        /* Etoile polaire */
        let x = (((dzeta as i32 * 2 + 512) % 2048) - 512 + 256) as c_short;
        Affmobbrt(
            (mob3D as i32 + myRandom(22, 24) as i32) as c_short,
            x / 2,
            20,
            0,
            null_mut(),
        );
    }
}

/*****************************************************************************
                                        CalculeDimensionsLigne
                                Calcule les nouvelles dimensions des lignes du sol
******************************************************************************/
unsafe fn CalculeDimensionsLignes() {
    for i in 1..NB_INTERV {
        TAB_LIGNE_SOL[i].hauteur =
            (TAB_LIGNE_SOL[i].y as i32 - TAB_LIGNE_SOL[i - 1].y as i32) as c_short;
        TAB_LIGNE_SOL[i].largeur =
            (LARGEUR_MOB * TAB_LIGNE_SOL[i].hauteur as i64 / HAUTEUR_MOB) as c_short;
    }
    TAB_LIGNE_SOL[0].hauteur = TAB_LIGNE_SOL[0].y;
    TAB_LIGNE_SOL[0].largeur =
        (LARGEUR_MOB * TAB_LIGNE_SOL[0].hauteur as i64 / HAUTEUR_MOB) as c_short;
}

/*******************************************************************************
                                                                TourneSol
                                Le personnage tourne la tete, le tableau des lignes du sol bouge
*********************************************************************************/
unsafe fn TourneSol() {
    if dzeta != LAST_DZETA {
        let delta = dzeta as i32 - LAST_DZETA as i32;
        for i in 1..NB_INTERV {
            if TAB_LIGNE_SOL[i].hauteur != 0 {
                TAB_LIGNE_SOL[i].offsetEnMob = ((datum(delta, i)) % LARGEUR_MOB as i32) as c_short;
                TAB_LIGNE_SOL[i].offsetEcran = (-(TAB_LIGNE_SOL[i].offsetEnMob as i64)
                    * TAB_LIGNE_SOL[i].hauteur as i64
                    / HAUTEUR_MOB) as c_short;
            }
        }
        LAST_DZETA = dzeta;
    }
}

// offsetEnMob = offsetEnMob + (long)(delta*2)*(long)HAUTEUR_MOB/(long)hauteur
unsafe fn datum(delta: i32, i: usize) -> i32 {
    (TAB_LIGNE_SOL[i].offsetEnMob as i64
        + (delta as i64 * 2) * HAUTEUR_MOB / TAB_LIGNE_SOL[i].hauteur as i64) as i32
}

/******************************************************************************
                                                        CalcY
        Calcule le Y ecran en fonction du delta d'altitude et de la distance
*******************************************************************************/
unsafe fn CalcY(idx: c_short) -> c_short {
    let dist = TAB_LIGNE_SOL[idx as usize].dist as i64;
    (256 * (38 + zj as i64 / 20) / dist) as c_short
}

/*******************************************************************************
                                                                AvanceSol
                                Le personnage avance, le tableau des lignes du sol bouge
*********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AvanceSol(dist: c_short) {
    if dist != 0 {
        SOL_RAND_SEED = TickCount() + xj as c_long + yj as c_long;
    }
    /* Est-ce l'init ? oui -> on calcule tout*/
    if TAB_LIGNE_SOL[0].dist == 0 {
        let mut d: i32 = 70;
        for i in 0..NB_INTERV {
            TAB_LIGNE_SOL[NB_INTERV - 1 - i].dist = d as c_short;
            TAB_LIGNE_SOL[NB_INTERV - 1 - i].y = CalcY((NB_INTERV - 1 - i) as c_short);
            d += LARGEUR_BANDE_SOL;
        }
    } else {
        for i in 0..NB_INTERV {
            TAB_LIGNE_SOL[i].dist = (TAB_LIGNE_SOL[i].dist as i32 - dist as i32) as c_short;
            TAB_LIGNE_SOL[i].y = CalcY(i as c_short);
        }
        /* La ligne la plus proche s'est-elle barrée ? */
        while TAB_LIGNE_SOL[NB_INTERV - 1].dist < 70 {
            /* On décale le tableau */
            for i in (1..NB_INTERV).rev() {
                TAB_LIGNE_SOL[i] = TAB_LIGNE_SOL[i - 1];
            }
            /* On recalcule le dernier élément */
            TAB_LIGNE_SOL[0].dist = (TAB_LIGNE_SOL[1].dist as i32 + LARGEUR_BANDE_SOL) as c_short;
            TAB_LIGNE_SOL[0].y = CalcY(0);
        }
    }
}

/*******************************************************************************
                                                                AfficheSol
                                On affiche les lignes du sol
*********************************************************************************/
unsafe fn AfficheSol() {
    let mut r: Rect = Rect::default();
    let mut rdep: Rect;
    let mut i: i32;
    let mut numPalette: i32;
    let numMob: c_short;
    let ymax: i32;
    let largeur: i32;
    let RandSeedBak: c_long;

    AvanceSol(0);
    CalculeDimensionsLignes();
    TourneSol();
    /* Trace toutes les rangées */
    /* On recherch la premiere ligne qui n'est pas au loin */
    i = 1;
    while i < NB_INTERV as i32 && TAB_LIGNE_SOL[i as usize].hauteur < 4 {
        i += 1;
    }

    /* On trace toutes les lignes au loin */
    if i < NB_INTERV as i32 {
        ymax = TAB_LIGNE_SOL[i as usize].y as i32 + NB_LIGNES_CIEL as i32;
    } else {
        ymax = NB_LIGNES_CIEL as i32 + NB_LIGNES_TERRE as i32;
    }
    largeur = HAUTEUR_MIN_BANDE * LARGEUR_MOB as i32 * 4 / HAUTEUR_MOB as i32;
    RandSeedBak = CarboniseGetRandSeed();
    CarboniseSetRandSeed(SOL_RAND_SEED);
    let mut j = ymax;
    while j > NB_LIGNES_CIEL as i32 {
        /* des bandes de 4 */
        let offset = ((2048 - dzeta as i32 * 2 + Rnd() as i32) % largeur) as c_short;
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
            Affmob3d(
                mob3D + 25,
                &mut r,
                *TAB_PALETTE_DISTANCE.add(numPalette as usize),
            );
            OffsetRect(&mut r, largeur as c_short, 0);
            k += largeur;
        }
        r = rdep;
        k = 256 - offset as i32;
        while k > 0 {
            OffsetRect(&mut r, -largeur as c_short, 0);
            Affmob3d(
                mob3D + 25,
                &mut r,
                *TAB_PALETTE_DISTANCE.add(numPalette as usize),
            );
            k -= largeur;
        }
        j -= HAUTEUR_MIN_BANDE;
    }

    /* On trace toutes les ligne präs */
    let _ = numMob;
    while i < NB_INTERV as i32 {
        let hauteur = TAB_LIGNE_SOL[i as usize].hauteur as i32;
        let largeur2 = TAB_LIGNE_SOL[i as usize].largeur as i32;
        if hauteur > 0 && largeur2 > 0 {
            let offset = TAB_LIGNE_SOL[i as usize].offsetEcran as i32;
            let nmob = mob3D + 20;
            let top = if i != 0 {
                TAB_LIGNE_SOL[(i - 1) as usize].y as i32 + NB_LIGNES_CIEL as i32
            } else {
                NB_LIGNES_CIEL as i32
            };
            let bottom = TAB_LIGNE_SOL[i as usize].y as i32 + NB_LIGNES_CIEL as i32;
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
                Affmob3d(nmob, &mut r, *TAB_PALETTE_DISTANCE.add(numPalette as usize));
                OffsetRect(&mut r, largeur2 as c_short, 0);
                kk += largeur2;
            }
            r = rdep;
            kk = 256 - offset;
            while kk > 0 {
                OffsetRect(&mut r, -largeur2 as c_short, 0);
                Affmob3d(nmob, &mut r, *TAB_PALETTE_DISTANCE.add(numPalette as usize));
                kk -= largeur2;
            }
        }
        i += 1;
    }
    CarboniseSetRandSeed(RandSeedBak);
}

/******************************************************************
                    TrameCielOrage
******************************************************************/
unsafe fn TrameCielOrage(index: c_short) {
    let mut tailleLigne: c_short = 0;
    let mut depth: c_short = 0;

    /* Origine des données de l'ecran */
    let data = AcamGetInfoOffscreen(
        CarboniseGetCurrentPort(),
        null_mut(),
        &mut tailleLigne,
        &mut depth,
    ) as *mut u8;
    if data.is_null() {
        return;
    }
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
    let mut tailleLigne: c_short = 0;
    let mut depth: c_short = 0;

    /* Origine des données de l'ecran */
    let data = AcamGetInfoOffscreen(
        CarboniseGetCurrentPort(),
        null_mut(),
        &mut tailleLigne,
        &mut depth,
    ) as *mut u8;
    if data.is_null() {
        return;
    }
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
unsafe fn TracePluie(index: c_short, n: c_short) {
    let mut tailleLigne: c_short = 0;
    let mut depth: c_short = 0;

    if myRandom(0, 1000) > 500 {
        Flou();
    }
    /* Origine des données de l'ecran */
    let data = AcamGetInfoOffscreen(
        CarboniseGetCurrentPort(),
        null_mut(),
        &mut tailleLigne,
        &mut depth,
    ) as *mut u8;
    if data.is_null() {
        return;
    }
    let mut n = n as i32;
    while n > 0 {
        n -= 1;
        let x = myRandom(0, tailleLigne as c_long) as c_short;
        let y = myRandom(0, 150) as c_short;
        let mut j = 50;
        let mut p = data.add((tailleLigne as i64 * y as i64 + x as i64) as usize);
        while j > 0 {
            j -= 1;
            *p = index as u8;
            p = p.add((tailleLigne as i64 - 1) as usize);
        }
    }
}

/****************************************************************************
                    eff3d
            Effacement de la 3d
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn eff3d(
    _ciel: c_uchar,  /* NU */
    _terre: c_uchar, /* NU */
    ymeteore: c_ushort,
    xmeteore: c_ushort,
    isLune: c_ushort,
) {
    let mut gbak: *mut std::os::raw::c_void = null_mut();
    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);

    /* Le degrade du ciel */
    /* Un orage est en cours */
    RemplitZoneLignes(0, NB_LIGNES_CIEL as c_short, TAB_COULEURS_CIEL);
    AfficheEtoiles();
    /* Le meteore */
    Affmobbrt(
        (crate::MOB_SOLEIL!() as i32 + isLune as i32) as c_short,
        xmeteore as c_short,
        ymeteore as c_short,
        0,
        null_mut(),
    );
    if FreqEclair < 1000 {
        if isLune != 0 {
            TrameCielOrage(COULEUR_ORAGE_NUIT);
        } else {
            TrameCielOrage(COULEUR_ORAGE_JOUR);
        }
    }
    /* Le sol */
    AfficheSol();
    AcamSetPortOffscreen(gbak);
}

/***************************************************************************************
                                 Pluie
***************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Pluie() {
    let mut gbak: *mut std::os::raw::c_void = null_mut();

    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);

    if FreqEclair < 1000 && FreqEclair > 500 {
        let n = if FreqEclair > 750 {
            1000 - FreqEclair
        } else {
            FreqEclair - 500
        };
        if heure & 0x200 != 0 {
            TracePluie(COULEUR_ORAGE_NUIT, n as c_short);
        } else {
            TracePluie(COULEUR_ORAGE_JOUR, n as c_short);
        }
        /* Si pas deja de l'eau par terre on mets de l'eau a terre */
        if liste4.taille > 0
            && liste4.elements[liste4.taille as usize - 1] != 2
            && liste4.taille < liste4.maxtaille
        {
            liste4.taille = liste4.taille.wrapping_add(1);
            tlpos[4] = liste4.taille as c_char;
            liste4.elements[liste4.taille as usize - 1] = 2; /*Eau */
        }
    }
    AcamSetPortOffscreen(gbak);
}

/***************************************************************************************
                                 PosePluie
Quand il pleut il ya des flaques par terre
***************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PosePluie() {
    if FreqEclair < 1000 && FreqEclair > 500 {
        /* Si pas deja de l'eau par terre on mets de l'eau a terre */
        if liste4.taille > 0
            && liste4.elements[liste4.taille as usize - 1] != 2
            && liste4.taille < liste4.maxtaille
        {
            liste4.taille = liste4.taille.wrapping_add(1);
            tlpos[4] = liste4.taille as c_char;
            liste4.elements[liste4.taille as usize - 1] = 2; /*Eau */
        }
    }
}

/***************************************************************************************
                                                CalculeAltitude
                                Calcule l'altitude d'un point du plan
***************************************************************************************/
unsafe fn CalculeAltitude(_x: c_short, _y: c_short) -> c_short {
    let mut i: c_long;
    let mut z: i32 = 40;
    let p = *aigoual;

    i = 0;
    while i < nbreobj {
        let mont = &*p.offset(i as isize);
        if (*(*mont).ptrforme).hauteur == 0 {
            let d = distance(
                xj as c_long - (*mont).posx as c_long,
                yj as c_long - (*mont).posy as c_long,
            );
            let r = (*(*mont).ptrforme).rayon as i64;
            if (d as i64) < r {
                z += ((r - d as i64) * (*(*mont).ptrforme).nbrdisk as i64 * (*mont).ign as i64 / r)
                    as i32;
            }
        }
        i += 1;
    }
    let _ = i;
    z as c_short
}

/*****************************************************************************************
                                                Visu :
Calcul et visualisation d'un objet 3d
L'affichage se fait sur ecran 1
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn visu(mont: MontPtr) {
    let mut x: c_short = 0;
    let mut y: c_short = 0;
    let mut dist: c_short = 0;
    let mut rd: c_short = 0;
    let mut gbak: *mut std::os::raw::c_void = null_mut();
    let mut r: Rect = Rect::default();

    AcamGetPortOffscreen(&mut gbak);
    AcamSetPortOffscreen(Ecran1);
    zj = CalculeAltitude(xj as c_short, yj as c_short) as c_ushort;
    let angle = ((dzeta as f32) * (3.1415927 as f32)) as f64 / 512.0;
    let ca = (-angle).cos() as f32;
    let sa = (-angle).sin() as f32;

    let mut m = mont;
    while !m.is_null() {
        let forme = (*m).ptrforme;
        if CalcPoint3D(
            xj as c_long,
            yj as c_long,
            zj as c_long,
            ca,
            sa,
            (*m).posx as c_long,
            (*m).posy as c_long,
            (*forme).hauteur as c_long,
            &mut x,
            &mut y,
            &mut dist,
        ) != 0
        {
            rd = ((*forme).rayon as c_long * 2 * 256 / dist as c_long) as c_short;
            let hauteurDisque =
                ((*m).ign as c_long * (*forme).nbrdisk as c_long * 256 / dist as c_long) as c_short;
            SetRect(
                &mut r,
                (x as i32 - rd as i32 / 2) as c_short,
                (y as i32 - hauteurDisque as i32) as c_short,
                (x as i32 + rd as i32 / 2) as c_short,
                y,
            );
            if !EmptyRect(&r)
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
                Affmob3d(
                    mob3D + (*forme).numMob3d,
                    &mut r,
                    *TAB_PALETTE_DISTANCE.add(numPalette as usize),
                );
            }
        }
        m = (*m).chaine;
    }
    AcamSetPortOffscreen(gbak);
}

/*;*********************************
;               ChaineObjet
; Chaine une montagne en entree
;*********************************
;=> pointeur sur montagne a traiter
;: la montagne est chainee
;<= 1 si chainage 0 sinon
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChaineObjet(mont: MontPtr) -> c_short {
    let mut poschaine: MontPtr = null_mut();
    let mut balaye: MontPtr;

    /* On calcule la distance de la montagne au joueur*/
    if (*mont).ign == 0 {
        return 0;
    }
    let forme = (*mont).ptrforme;
    let dist = distance(
        (*mont).posx as c_long - xj as c_long,
        (*mont).posy as c_long - yj as c_long,
    );
    (*mont).distance = dist as c_ushort;
    if dist as i64 > 256 * (*forme).nbrdisk as i64 * (*mont).ign as i64 / 4 {
        return 0;
    }
    if dist as i64 > 256 * (*forme).rayon as i64 / 3 {
        return 0;
    }
    match (dzeta as i32 >> 8) & 3 {
        0 => {
            if (*mont).posx < xj && (*mont).posy < yj {
                return 0;
            }
        }
        1 => {
            if (*mont).posx > xj && (*mont).posy < yj {
                return 0;
            }
        }
        2 => {
            if (*mont).posx > xj && (*mont).posy > yj {
                return 0;
            }
        }
        _ => {
            if (*mont).posx < xj && (*mont).posy > yj {
                return 0;
            }
        }
    }
    /* Premier element de la chaine */
    balaye = (*(*aigoual)).chaine;
    while !balaye.is_null() && (*balaye).distance > dist as c_ushort {
        poschaine = balaye;
        balaye = (*balaye).chaine;
    }
    if poschaine.is_null() {
        (*mont).chaine = (*(*aigoual)).chaine;
        (*(*aigoual)).chaine = mont;
    } else {
        (*mont).chaine = (*poschaine).chaine;
        (*poschaine).chaine = mont;
    }
    1
}
/*EOF*/
