/***********************************************************************
                Module de Gestion de la Trois D v 2.0
                (C) God Le 4 Décembre 1986
***********************************************************************/
#![allow(non_snake_case)]

// Le moteur 3D : formes, objets 3D (ObjetsRef), décor (nuages, éclairs,
// oiseaux, arbres, rochers) et animation finale.
// Les globaux et fonctions exposés ici sont consommés par les fichiers C
// encore compilés (SAPW3D.C, SAPGEST.C, SAPREL.C, SAPCOMB.C, SAPBONC.C) via
// les déclarations de src/bindings.rs.
use std::os::raw::{c_char, c_long, c_short, c_ulong, c_ushort};
use std::ptr::{addr_of, null_mut};

use crate::acam::{
    AcamSetPortOffscreen, CarboniseSetPortWindow, CtoPstr, DisposHandle, FAUX, GetCursor, MemError,
    NewHandle, NewPtrClear, RGBForeColor, SetHandleSize, TickCount, VRAI,
};
use crate::cbridge::watchCursor;
use crate::cbridge::{FILE, feof, fprintf, fscanf, strcmp, strcpy};
use crate::sapiens::sapcomb::nbHerbes;
use crate::sapiens::sapglob::{
    angsol, bouton, ciel, dismin, dsposx, dzeta, flefba, heure, icone, jour_dsk, metpos, mort,
    presence, s_action, sapvit, sposy, terre, xj, yj, zj,
};
use crate::sapiens::saplist::{InverseOption, inimen};
use crate::sapiens::saprel::{chrono, clrbadi, comment, zoninf};
use crate::sapiens::sapw3d::{
    AvanceSol, ChaineObjet, Pluie, PosePluie, distance, eff3d, sapdep, visu,
};
use crate::sapiens::sapwbrt::{DoMoveTo, DoTextSize, affomb, effima, tfrima, tfrvirt};
use crate::sapiens::sapwcart::carte;
use crate::sapiens::sapwclav::{getkey3d, sapiensTimer, vq_mouse};
use crate::sapiens::sapwind::{
    Collapse, Ecran1, GereInterface, ValideUpdate, delai, initrnd, mainWindow, mobBambou, myRandom,
};
use crate::sapiens::{
    BASEMOB, DIST_MIN_FOUDROIE, FREQ_ORAGE, Forme, Mont, MontHandle, MontPtr, NB_ARBRES,
    NB_ARBRES_MAX, NB_HERBE_AV_PLAN, NB_HOMMES, NB_MAX_HERBE_AV_PLAN, NB_NUAGES, NB_NUAGES_MAX,
    NB_OISEAUX, NB_OISEAUX_MAX, NB_ROCHERS, NB_ROCHERS_MAX,
};
use crate::shared::{Boolean, Handle, RGBColor, StringPtr};
use crate::stubs::{CarboniseSetCursorArrow, SetCursor};
use crate::text::{DrawString, StringWidth, TextFont};

const VITESSE_DEPLACEMENT_OISEAUX: c_ushort = 10;
const MONT1: c_short = 1;
const MONT2: c_short = 2;
const MONT3: c_short = 3;
const MONT4: c_short = 4;
const MONT5: c_short = 5;
const TOTEM: c_short = 6;
const NUAGE: c_short = 7;
const FUMEE: c_short = 18;
const FOYER: c_short = 33;
const MOB_OISEAU: c_short = 28;
const MOB_HOMME: c_short = 35;
const ECLAIR: c_short = 36;
const MOB_FOUDROYE: c_short = 38;

pub static mut forme1: Forme = Forme {
    nbrdisk: 5,
    relief: 0,
    hauteur: 0,
    rayon: 900,
    dskrayon: [800, 660, 150, 100, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        9, 9, 6, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    num_mob: BASEMOB + 0,
    numMob3d: MONT1,
};
pub static mut forme2: Forme = Forme {
    nbrdisk: 8,
    relief: 2,
    hauteur: 0,
    rayon: 1900,
    dskrayon: [
        1900, 1700, 1100, 1000, 950, 625, 520, 300, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    dskdelta: [
        0,
        10,
        26,
        0,
        0,
        1,
        0,
        0,
        200u8 as c_char,
        0,
        200u8 as c_char,
        0,
        200u8 as c_char,
        200u8 as c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 1,
    numMob3d: MONT2,
};

pub static mut forme4: Forme = Forme {
    nbrdisk: 9,
    relief: 1,
    hauteur: 0,
    rayon: 5000,
    dskrayon: [
        4100, 3700, 3250, 2900, 2500, 2000, 1500, 1000, 250, 0, 0, 0, 0, 0, 0, 0,
    ],
    dskdelta: [
        240u8 as c_char,
        240u8 as c_char,
        80,
        80,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 2,
    numMob3d: MONT4,
};

pub static mut forme6: Forme = Forme {
    nbrdisk: 14,
    relief: 1,
    hauteur: 0,
    rayon: 10000,
    dskrayon: [
        9100, 7400, 6900, 6490, 5600, 4000, 3150, 2500, 2000, 1500, 1200, 850, 550, 200, 0, 0,
    ],
    dskdelta: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    num_mob: BASEMOB + 3,
    numMob3d: MONT3,
};

pub static mut forme7: Forme = Forme {
    nbrdisk: 16,
    relief: 1,
    hauteur: 0,
    rayon: 8000,
    dskrayon: [
        6500, 4900, 4100, 3200, 2700, 2500, 2300, 2000, 1500, 1200, 1100, 1050, 900, 725, 525, 300,
    ],
    dskdelta: [
        0, 0, 0, 0, 0, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ],
    num_mob: BASEMOB + 4,
    numMob3d: MONT5,
};

pub static mut forme8: Forme = Forme {
    nbrdisk: 12,
    relief: 1,
    hauteur: 0,
    rayon: 2000,
    dskrayon: [
        6000, 4000, 3100, 2300, 1900, 1600, 1400, 1200, 1100, 900, 500, 100, 0, 0, 0, 0,
    ],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        16,
        48,
        16,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        9,
        128u8 as c_char,
        8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 5,
    numMob3d: 30,
};

pub static mut nuage1: Forme = Forme {
    nbrdisk: 8,
    relief: 3,
    hauteur: 6000,
    rayon: 3500,
    dskrayon: [
        7500, 6500, 5500, 7600, 7000, 3000, 3700, 20, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    dskdelta: [
        0,
        0,
        192u8 as c_char,
        192u8 as c_char,
        244u8 as c_char,
        244u8 as c_char,
        0,
        0,
        192u8 as c_char,
        192u8 as c_char,
        244u8 as c_char,
        244u8 as c_char,
        0,
        0,
        192u8 as c_char,
        192u8 as c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: NUAGE,
};

pub static mut nuage2: Forme = Forme {
    nbrdisk: 9,
    relief: 3,
    hauteur: 4000,
    rayon: 3500,
    dskrayon: [
        4000, 5000, 5500, 5000, 1000, 4000, 3000, 500, 3000, 0, 0, 0, 0, 0, 0, 0,
    ],
    dskdelta: [
        -1,
        0,
        192u8 as c_char,
        192u8 as c_char,
        244u8 as c_char,
        244u8 as c_char,
        0,
        0,
        15,
        0,
        244u8 as c_char,
        244u8 as c_char,
        0,
        0,
        -1,
        -1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: NUAGE + 1,
};

pub static mut nuage3: Forme = Forme {
    nbrdisk: 5,
    relief: 3,
    hauteur: 3000,
    rayon: 3500,
    dskrayon: [4000, 3200, 3000, 1000, 900, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        -1,
        0,
        192u8 as c_char,
        192u8 as c_char,
        0,
        4,
        0,
        208u8 as c_char,
        192u8 as c_char,
        192u8 as c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: NUAGE + 2,
};

pub static mut eclair: Forme = Forme {
    nbrdisk: 10,
    relief: 3,
    hauteur: 100,
    rayon: 500,
    dskrayon: [4000, 3200, 3000, 1000, 900, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        -1,
        0,
        192u8 as c_char,
        192u8 as c_char,
        0,
        4,
        0,
        208u8 as c_char,
        192u8 as c_char,
        192u8 as c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: ECLAIR,
};

pub static mut toucheEclair: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 2,
    rayon: 60,
    dskrayon: [4, 4, 20, 23, 20, 22, 21, 15, 10, 1, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        6,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: 37,
};

pub static mut foudroye: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 1,
    rayon: 60,
    dskrayon: [4, 4, 20, 23, 20, 22, 21, 15, 10, 1, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        6,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: MOB_FOUDROYE,
};

pub static mut peupli: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 1,
    rayon: 60,
    dskrayon: [4, 4, 20, 23, 20, 22, 21, 15, 10, 1, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        6,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: 10,
};

pub static mut chene: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 1,
    rayon: 40,
    dskrayon: [10, 6, 10, 40, 45, 47, 40, 35, 26, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: 11,
};

pub static mut noiset: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 1,
    rayon: 40,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: 12,
};

pub static mut fontaine: Forme = Forme {
    nbrdisk: 1,
    relief: 4,
    hauteur: 2,
    rayon: 40,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: 24,
};

pub static mut totem: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 2,
    rayon: 40,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: TOTEM,
};

pub static mut hutte: Forme = Forme {
    nbrdisk: 1,
    relief: 4,
    hauteur: 2,
    rayon: 40,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: 32,
};

pub static mut oiseau: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 500,
    rayon: 40,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: MOB_OISEAU,
};

pub static mut genevr: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 1,
    rayon: 40,
    dskrayon: [4, 4, 15, 13, 11, 9, 8, 6, 4, 2, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: 13,
};

pub static mut hetre: Forme = Forme {
    nbrdisk: 10,
    relief: 4,
    hauteur: 1,
    rayon: 40,
    dskrayon: [6, 5, 4, 3, 22, 19, 20, 16, 12, 2, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        5,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 6,
    numMob3d: 14,
};

pub static mut roc1: Forme = Forme {
    nbrdisk: 8,
    relief: 1,
    hauteur: 0,
    rayon: 210,
    dskrayon: [200, 80, 70, 60, 50, 10, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0, 0, 0, 2, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    num_mob: BASEMOB + 7,
    numMob3d: 15,
};

pub static mut roc2: Forme = Forme {
    nbrdisk: 7,
    relief: 1,
    hauteur: 0,
    rayon: 600,
    dskrayon: [500, 400, 225, 190, 160, 120, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0, 0, 2, 2, 1, 3, 0, 10, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    num_mob: BASEMOB + 7,
    numMob3d: 16,
};

pub static mut roc3: Forme = Forme {
    nbrdisk: 8,
    relief: 1,
    hauteur: 0,
    rayon: 600,
    dskrayon: [
        3000, 2500, 2200, 1800, 800, 750, 650, 600, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    dskdelta: [
        0, 0, 2, 10, -2, 3, 0, 10, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ],
    num_mob: BASEMOB + 7,
    numMob3d: 17,
};

pub static mut fumee: Forme = Forme {
    nbrdisk: 9,
    relief: 3,
    hauteur: 2,
    rayon: 40,
    dskrayon: [2, 10, 20, 70, 100, 160, 100, 200, 20, 0, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0, 0, 2, 2, 15, 15, 15, 15, 0, 0, 15, 15, 16, 16, 32, 32, 15, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    num_mob: BASEMOB + 8,
    numMob3d: FUMEE,
};

pub static mut foyer: Forme = Forme {
    nbrdisk: 1,
    relief: 4,
    hauteur: 2,
    rayon: 20,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: FOYER,
};

pub static mut homme: Forme = Forme {
    nbrdisk: 1,
    relief: 4,
    hauteur: 1,
    rayon: 20,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: -1,
    numMob3d: 35,
};

pub static mut building: Forme = Forme {
    nbrdisk: 12,
    relief: 1,
    hauteur: 0,
    rayon: 100,
    dskrayon: [
        6000, 4000, 3100, 2300, 1900, 1600, 1400, 1200, 1100, 900, 500, 100, 0, 0, 0, 0,
    ],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        16,
        48,
        16,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        9,
        128u8 as c_char,
        8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    num_mob: BASEMOB + 5,
    numMob3d: 35,
};

pub static mut rocher: Forme = Forme {
    /* Nombre de disques : 1 la hauteur sera finement réglée par l'objet */
    nbrdisk: 1,
    /* Type de relief, inutilisé */
    relief: 4,
    /* Hauteur 0=montagne,1=arbre */
    hauteur: 1,
    /* Rayon apprécié : va conditionner l'aire horizontale */
    rayon: 40,
    dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
    dskdelta: [
        0,
        0,
        0,
        0,
        0,
        0,
        128u8 as c_char,
        7,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    /* Si -1 pas de représentation en carte */
    num_mob: -1,
    /* Numéro du mob dans Sapw3d */
    numMob3d: 34,
};

const NB_OBJ_REF: usize = 223;
static mut OBJETS_REF: [Mont; NB_OBJ_REF] = [
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 20000,
        posy: 1590,
        ign: 20,
        ptrforme: core::ptr::addr_of!(forme1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 5120,
        posy: 53760,
        ign: 20,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 26000,
        posy: 16000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 9984,
        posy: 16860,
        ign: 140,
        ptrforme: core::ptr::addr_of!(forme6) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 36133,
        posy: 53402,
        ign: 200,
        ptrforme: core::ptr::addr_of!(forme7) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 50435,
        posy: 17637,
        ign: 80,
        ptrforme: core::ptr::addr_of!(forme8) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22656,
        posy: 21632,
        ign: 80,
        ptrforme: core::ptr::addr_of!(fumee) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22656,
        posy: 21632,
        ign: 15,
        ptrforme: core::ptr::addr_of!(foyer) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22704,
        posy: 21632,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22640,
        posy: 21632,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39680,
        posy: 14336,
        ign: 60,
        ptrforme: core::ptr::addr_of!(fumee) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39680,
        posy: 14336,
        ign: 15,
        ptrforme: core::ptr::addr_of!(foyer) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39744,
        posy: 14400,
        ign: 140,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39808,
        posy: 14432,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39872,
        posy: 14432,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39808,
        posy: 14592,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39552,
        posy: 14592,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39552,
        posy: 14080,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39424,
        posy: 14400,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39424,
        posy: 14848,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12032,
        posy: 52992,
        ign: 60,
        ptrforme: core::ptr::addr_of!(fumee) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12032,
        posy: 52992,
        ign: 15,
        ptrforme: core::ptr::addr_of!(foyer) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 11892,
        posy: 53082,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 11992,
        posy: 53162,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 11792,
        posy: 52992,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 49920,
        posy: 43520,
        ign: 70,
        ptrforme: core::ptr::addr_of!(fumee) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 49920,
        posy: 43520,
        ign: 15,
        ptrforme: core::ptr::addr_of!(foyer) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 50120,
        posy: 43720,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 50220,
        posy: 43720,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 50220,
        posy: 43820,
        ign: 100,
        ptrforme: core::ptr::addr_of!(hutte) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 49952,
        posy: 43520,
        ign: 15,
        ptrforme: core::ptr::addr_of!(totem) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 4000,
        posy: 4000,
        ign: 20,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 9000,
        posy: 5000,
        ign: 33,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 9000,
        posy: 5000,
        ign: 30,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 3500,
        ign: 16,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 8000,
        posy: 1800,
        ign: 26,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40000,
        posy: 5000,
        ign: 33,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 25000,
        posy: 52000,
        ign: 30,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 9000,
        posy: 5000,
        ign: 30,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 37000,
        posy: 30000,
        ign: 16,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 32000,
        ign: 33,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 43000,
        posy: 42000,
        ign: 25,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 4000,
        posy: 50000,
        ign: 46,
        ptrforme: core::ptr::addr_of!(forme2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33000,
        posy: 8000,
        ign: 120,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 55000,
        posy: 5000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 38000,
        posy: 23500,
        ign: 140,
        ptrforme: core::ptr::addr_of!(forme7) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 28000,
        posy: 26000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 37000,
        ign: 120,
        ptrforme: core::ptr::addr_of!(forme7) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 37000,
        posy: 37000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 47000,
        posy: 35000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33000,
        posy: 8000,
        ign: 120,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 55000,
        posy: 42000,
        ign: 120,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 5000,
        posy: 32000,
        ign: 140,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 20000,
        posy: 30000,
        ign: 120,
        ptrforme: core::ptr::addr_of!(forme4) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 25000,
        posy: 40000,
        ign: 120,
        ptrforme: core::ptr::addr_of!(forme7) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22656,
        posy: 21280,
        ign: 8,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22272,
        posy: 20992,
        ign: 8,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22784,
        posy: 20992,
        ign: 8,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 44000,
        posy: 45800,
        ign: 20,
        ptrforme: core::ptr::addr_of!(totem) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 44000,
        posy: 45900,
        ign: 20,
        ptrforme: core::ptr::addr_of!(totem) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 43900,
        posy: 45800,
        ign: 20,
        ptrforme: core::ptr::addr_of!(totem) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 25000,
        posy: 22500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19000,
        posy: 24000,
        ign: 8,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 42500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 3000,
        posy: 30000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 4500,
        posy: 46000,
        ign: 4,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 17000,
        posy: 44000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 17500,
        posy: 49000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 45000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19200,
        posy: 47200,
        ign: 12,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 9000,
        posy: 45000,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32100,
        posy: 15050,
        ign: 15,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32200,
        posy: 15200,
        ign: 12,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32300,
        posy: 15400,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32400,
        posy: 15410,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 15600,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32050,
        posy: 15700,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12000,
        posy: 50000,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 15800,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 31900,
        posy: 15900,
        ign: 15,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32050,
        posy: 15700,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33000,
        posy: 16000,
        ign: 17,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33500,
        posy: 16000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 31900,
        posy: 15900,
        ign: 15,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33000,
        posy: 13500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 38000,
        posy: 12000,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 15000,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32800,
        posy: 18100,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33050,
        posy: 16500,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32800,
        posy: 18200,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 35000,
        posy: 16000,
        ign: 7,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 25000,
        posy: 21500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 10900,
        posy: 28000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 31000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 15000,
        posy: 26000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 29000,
        posy: 20500,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 21700,
        posy: 8500,
        ign: 12,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 23800,
        posy: 6000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 20000,
        posy: 5500,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 24000,
        posy: 5000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 26000,
        posy: 10000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 20000,
        posy: 9000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 15000,
        posy: 6000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 15500,
        posy: 47000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 24000,
        posy: 48500,
        ign: 20,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 20500,
        posy: 18000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40000,
        posy: 200,
        ign: 18,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 34000,
        posy: 2000,
        ign: 16,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 1000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 100,
        posy: 41000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22000,
        posy: 25000,
        ign: 18,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 21000,
        posy: 21000,
        ign: 20,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13500,
        posy: 44000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33500,
        posy: 19000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 42000,
        posy: 17000,
        ign: 20,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 34000,
        posy: 17000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 38000,
        posy: 16000,
        ign: 9,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39050,
        posy: 14050,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 21000,
        posy: 23000,
        ign: 14,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12100,
        posy: 29050,
        ign: 12,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 11000,
        posy: 30000,
        ign: 20,
        ptrforme: core::ptr::addr_of!(peupli) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 46000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 17500,
        posy: 45000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19000,
        posy: 47000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19000,
        posy: 48500,
        ign: 18,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 22000,
        posy: 23000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 42500,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 3500,
        posy: 47000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 7000,
        posy: 41000,
        ign: 11,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32900,
        posy: 17000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40000,
        posy: 14000,
        ign: 11,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40000,
        posy: 15000,
        ign: 8,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39000,
        posy: 12200,
        ign: 11,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32800,
        posy: 18000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 42000,
        posy: 15000,
        ign: 18,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 41500,
        posy: 14000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 42500,
        posy: 16000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 31500,
        posy: 22000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12000,
        posy: 29000,
        ign: 17,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 11900,
        posy: 29100,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12500,
        posy: 30000,
        ign: 15,
        ptrforme: core::ptr::addr_of!(hetre) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 5000,
        posy: 39000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19250,
        posy: 49200,
        ign: 15,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 35000,
        posy: 43000,
        ign: 4,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 41000,
        posy: 2000,
        ign: 2,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 4000,
        posy: 40000,
        ign: 6,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 14000,
        posy: 45000,
        ign: 2,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 14000,
        posy: 45000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19100,
        posy: 47050,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 31500,
        posy: 14500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 34000,
        posy: 15000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 36000,
        posy: 17000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40000,
        posy: 18000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 31000,
        posy: 19000,
        ign: 12,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 14000,
        posy: 30000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 10000,
        posy: 29500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 27000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40000,
        posy: 42000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 41000,
        posy: 44000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 47000,
        posy: 45000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 48000,
        posy: 44000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 48000,
        posy: 48000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(genevr) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 21000,
        posy: 20000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 21000,
        posy: 20000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12400,
        posy: 28500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 12400,
        posy: 28500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 48000,
        posy: 42000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 48000,
        posy: 42000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 2000,
        posy: 2000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 2000,
        posy: 2000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39000,
        posy: 800,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39000,
        posy: 800,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 16000,
        posy: 42000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 16000,
        posy: 42000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 16000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32000,
        posy: 16000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 45000,
        posy: 25500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 45000,
        posy: 25500,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 6000,
        posy: 45000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 6000,
        posy: 45000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 50000,
        posy: 50000,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 50000,
        posy: 50000,
        ign: 5,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 24320,
        posy: 18944,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 24320,
        posy: 18944,
        ign: 10,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32768,
        posy: 17408,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 32768,
        posy: 17408,
        ign: 5,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40192,
        posy: 14336,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 40192,
        posy: 14336,
        ign: 5,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 43776,
        posy: 16896,
        ign: 10,
        ptrforme: core::ptr::addr_of!(noiset) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 43776,
        posy: 16896,
        ign: 5,
        ptrforme: core::ptr::addr_of!(fontaine) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 100,
        posy: 39000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 15000,
        posy: 46000,
        ign: 45,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 42000,
        posy: 16000,
        ign: 75,
        ptrforme: core::ptr::addr_of!(roc3) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 33050,
        posy: 17000,
        ign: 95,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 5000,
        posy: 38000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 7000,
        posy: 39000,
        ign: 150,
        ptrforme: core::ptr::addr_of!(roc3) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 4000,
        posy: 43000,
        ign: 110,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 7000,
        posy: 44000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 10000,
        posy: 42000,
        ign: 150,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 15000,
        posy: 43000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 15000,
        posy: 45000,
        ign: 140,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 18000,
        posy: 47000,
        ign: 98,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 19000,
        posy: 50000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 42000,
        posy: 46000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 44000,
        posy: 45000,
        ign: 200,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 46000,
        posy: 42000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 38000,
        posy: 44000,
        ign: 90,
        ptrforme: core::ptr::addr_of!(roc1) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 35000,
        posy: 1000,
        ign: 110,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 37000,
        posy: 3000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 36000,
        posy: 15000,
        ign: 110,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 37000,
        posy: 3000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39000,
        posy: 13000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 41000,
        posy: 13000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 36000,
        posy: 18000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 39000,
        posy: 17000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 38000,
        posy: 12000,
        ign: 132,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 42000,
        posy: 15000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 20000,
        posy: 22000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 13000,
        posy: 28000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 11000,
        posy: 32000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 46000,
        posy: 47500,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc2) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 9500,
        posy: 50000,
        ign: 100,
        ptrforme: core::ptr::addr_of!(roc3) as *const Forme as *mut Forme,
    },
    Mont {
        chaine: core::ptr::null_mut(),
        distance: 00,
        posx: 1,
        posy: 1,
        ign: 10,
        ptrforme: core::ptr::addr_of!(chene) as *const Forme as *mut Forme,
    },
];
/* Liste des objets présents */
pub static mut aigoual: MontHandle = null_mut();
pub static mut nbreobj: c_long = 0;
static mut PremHomme: c_long = -1;
pub static mut nbNuages: c_long = 0;
pub static mut nbOiseaux: c_long = 0;
pub static mut nbRochers: c_long = 0;
pub static mut nbHommes: c_long = 0;
pub static mut nbArbres: c_long = 0;
pub static mut FreqOrage: c_long = 0;
pub static mut DistMinFoudroie: c_long = 0;
pub static mut VitesseDeplacementNuages: c_long = 0;
static mut PremEclair: c_long = 0;
static mut PremArbre: c_long = 0;
static mut PremRocher: c_long = 0;
static mut ArbreFoudroye: MontPtr = null_mut();
pub static mut nbEclairs: c_long = 0;
pub static mut FreqEclair: c_long = 1000;

/***********************************************************************
                         AjouteObjet3d
Ajoute un objet à la liste des objets 3ds
=> rien
<= index
***********************************************************************/
pub unsafe extern "C" fn AjouteObjet3d(nbObj: c_long) -> c_long {
    unsafe {
        if aigoual.is_null() {
            aigoual = NewHandle(0) as MontHandle;
        }
        nbreobj += nbObj;
        SetHandleSize(
            aigoual as Handle,
            nbreobj * std::mem::size_of::<Mont>() as c_long,
        );
        let err = MemError();
        if err != 0 {
            Collapse(
                std::ffi::CString::new(format!("[Sap3d.AjouteObjet3d.1] Error # {err}"))
                    .unwrap_or_default()
                    .as_ptr() as *mut c_char,
            );
        }
        nbreobj - nbObj
    }
}
/***********************************************************************
                         EstDansMontagne
Dit si une position est dans une montagne
***********************************************************************/

unsafe fn EstDansMontagne(x: c_ushort, y: c_ushort) -> Boolean {
    unsafe {
        let mut i: c_long = 0;
        while i < nbreobj {
            let p = (*aigoual).add(i as usize);
            if !(*p).ptrforme.is_null() && (*(*p).ptrforme).hauteur == 0 {
                let dx = ((*p).posx as c_long - x as c_long).abs();
                let dy = ((*p).posy as c_long - y as c_long).abs();
                let dist = distance(dx as c_long, dy as c_long);
                if dist < (*(*p).ptrforme).rayon as i16 {
                    return VRAI;
                }
            }
            i += 1;
        }
        FAUX
    }
}

/***********************************************************************
                         DeplaceHommes
***********************************************************************/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct GrandRect {
    top: c_ushort,
    left: c_ushort,
    right: c_ushort,
    bottom: c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Homme {
    existe: c_short,
    x: c_short,
    y: c_short,
    vx: c_short,
    vy: c_short,
    rect: GrandRect,
}

type HommePtr = *mut Homme;

static mut TabHomme: HommePtr = null_mut();

fn DeplaceHommes() {
    unsafe {
        let mut h: HommePtr = TabHomme;
        let mut mont: MontPtr = (*aigoual).add(PremHomme as usize);
        let mut i: c_long = 0;
        while i < nbHommes {
            /* L'objet n'est pas défini ? */
            if (*h).existe == 0 {
                (*(*aigoual).add((PremHomme + i) as usize)).ign = myRandom(50, 60) as c_ushort;
                (*h).existe = 1;
                (*h).vx = myRandom(-5, 5) as c_short;
                (*h).vy = myRandom(-5, 5) as c_short;
                match myRandom(0, 4) {
                    _ => {
                        (*h).rect.left = 22672 - 1000;
                        (*h).rect.right = 22672 + 1000;
                        (*h).rect.top = 21632 - 1000;
                        (*h).rect.bottom = 21632 + 1000;
                        (*h).x = 22672;
                        (*h).y = 21632;
                    }
                }
            } else {
                /* L'objet se déplace */
                (*h).x += (*h).vx;
                (*h).y += (*h).vy;
                if (*h).x < (*h).rect.left as c_short || (*h).x > (*h).rect.right as c_short {
                    (*h).vx = -(*h).vx;
                }
                if (*h).y < (*h).rect.top as c_short || (*h).y > (*h).rect.bottom as c_short {
                    (*h).vy = -(*h).vy;
                }
                (*mont).posx = (*h).x as c_ushort;
                (*mont).posy = (*h).y as c_ushort;
                if (*mont).distance < 200 {
                    (*h).existe = 0;
                    (*mont).ign = 0; /* InActif */
                }
            }
            i += 1;
            h = h.add(1);
            mont = mont.add(1);
        }
    }
}

/***********************************************************************
                         GenereHommes
***********************************************************************/
pub unsafe extern "C" fn GenereHommes() {
    unsafe {
        let mut i: c_long = 0;
        let mut index: c_long;

        PremHomme = -1;
        TabHomme =
            NewPtrClear((std::mem::size_of::<Homme>() as c_long * nbHommes) as c_long) as HommePtr;
        while i < nbHommes {
            index = AjouteObjet3d(1);
            if PremHomme < 0 {
                PremHomme = index;
            }
            let p = (*aigoual).add(index as usize);
            (*p).ign = 0; // Non visible
            (*p).ptrforme = addr_of!(homme) as *const Forme as *mut Forme;
            i += 1;
        }
    }
}

/***********************************************************************
                         DefautDecor3d
3D au demarrage du jeu
**********************************************************************/

pub unsafe extern "C" fn DefautDecor3d() {
    unsafe {
        FreqOrage = FREQ_ORAGE as c_long; //9990
        DistMinFoudroie = DIST_MIN_FOUDROIE as c_long; //250
        nbNuages = NB_NUAGES as c_long; //100
        nbOiseaux = NB_OISEAUX as c_long; //20
        nbRochers = NB_ROCHERS as c_long; //300
        nbHommes = NB_HOMMES as c_long; //0
        nbArbres = NB_ARBRES as c_long; //800
        nbHerbes = NB_HERBE_AV_PLAN as c_long;
        FreqEclair = 1000; //1000
        VitesseDeplacementNuages = 20;
    }
}

/***********************************************************************
                         InitDecor3d
Tire au hasard les fréquences à chaque nouvelle aventure
***********************************************************************/
pub fn InitDecor3d() {
    unsafe {
        FreqOrage = myRandom(9900, 10000);
        DistMinFoudroie = myRandom(20, 500);
        nbNuages = myRandom(NB_NUAGES as c_long, NB_NUAGES_MAX as c_long);
        nbOiseaux = myRandom(NB_OISEAUX as c_long, NB_OISEAUX_MAX as c_long);
        nbRochers = myRandom(NB_ROCHERS as c_long, NB_ROCHERS_MAX as c_long);
        nbArbres = myRandom(NB_ARBRES as c_long, NB_ARBRES_MAX as c_long); //800
        nbHerbes = myRandom(NB_HERBE_AV_PLAN as c_long, NB_MAX_HERBE_AV_PLAN as c_long);
        nbHommes = 0; //0
        FreqEclair = 1000;
        VitesseDeplacementNuages = myRandom(1, 120);
    }
}

/***********************************************************************
                         CreeDecor3d
Crée la liste des objets 3ds
***********************************************************************/
pub fn CreeDecor3d(display: Boolean) {
    unsafe {
        let mut index: c_long;
        let mut mont: Mont = Mont::default();

        if display {
            flefba = 0;
            comment(137); /*Je crée le monde*/
            ValideUpdate();
        }
        initrnd(b'M' as c_short, b'Y' as c_short, b'R' as c_short);

        #[cfg(debug_assertions)]
        {
            // Reflet des blocs #if DEBUG>0
            let fo = unsafe { core::ptr::addr_of!(FreqOrage).read() };
            let df = unsafe { core::ptr::addr_of!(DistMinFoudroie).read() };
            let nn = unsafe { core::ptr::addr_of!(nbNuages).read() };
            let no = unsafe { core::ptr::addr_of!(nbOiseaux).read() };
            let nr = unsafe { core::ptr::addr_of!(nbRochers).read() };
            let na = unsafe { core::ptr::addr_of!(nbArbres).read() };
            eprintln!(
                "FreqOrage {}\nDistMinFoudroie {}\nnbNuages {}\nnbOiseaux {}\nnbRochers {}\nnbArbres {}",
                fo, df, nn, no, nr, na
            );
        }
        nbEclairs = nbNuages;
        if !aigoual.is_null() {
            DisposHandle(aigoual as Handle);
        }
        aigoual = null_mut();
        PremEclair = 0;
        PremArbre = 0;
        PremRocher = 0;
        ArbreFoudroye = null_mut();
        nbreobj = 0;
        for _ in 0..nbNuages {
            mont.posx = (myRandom(0, 32767) * 2) as c_ushort;
            mont.posy = (myRandom(0, 32767) * 2) as c_ushort;
            mont.ign = myRandom(100, 200) as c_ushort;
            match myRandom(0, 3) {
                0 => mont.ptrforme = addr_of!(nuage1) as *const Forme as *mut Forme,
                1 => mont.ptrforme = addr_of!(nuage2) as *const Forme as *mut Forme,
                _ => mont.ptrforme = addr_of!(nuage3) as *const Forme as *mut Forme,
            }
            index = AjouteObjet3d(1);
            (*aigoual).add(index as usize).write(mont);
        }
        /* À chaque nuage un éclair est associé */
        for i in 0..nbEclairs {
            mont = (*aigoual).add(i as usize).read(); //Copie du nuage associé
            mont.ign = 0; //Altitude zéro inactif
            mont.ptrforme = addr_of!(eclair) as *const Forme as *mut Forme;
            mont.posx = (mont.posx as c_long
                + (*(*aigoual).add(i as usize).read().ptrforme).rayon as c_long / 2)
                as c_ushort;
            mont.posy = (mont.posy as c_long
                + (*(*aigoual).add(i as usize).read().ptrforme).rayon as c_long / 2)
                as c_ushort;
            index = AjouteObjet3d(1);
            (*aigoual).add(index as usize).write(mont);
            if PremEclair == 0 {
                PremEclair = index;
            }
        }
        for _ in 0..nbOiseaux {
            mont.posx = (myRandom(0, 32767) * 2) as c_ushort;
            mont.posy = (myRandom(0, 32767) * 2) as c_ushort;
            mont.ign = 10;
            mont.ptrforme = addr_of!(oiseau) as *const Forme as *mut Forme;
            index = AjouteObjet3d(1);
            (*aigoual).add(index as usize).write(mont);
        }
        index = AjouteObjet3d(NB_OBJ_REF as c_long);
        /* On crée le tableau avec les objets de base */
        for i in 0..NB_OBJ_REF {
            (*aigoual).add(index as usize + i).write(OBJETS_REF[i]);
        }
        // les arbres
        for _ in 0..nbArbres {
            loop {
                mont.posx = (myRandom(0, 32767) * 2) as c_ushort;
                mont.posy = (myRandom(0, 32767) * 2) as c_ushort;
                if !EstDansMontagne(mont.posx, mont.posy) {
                    break;
                }
            }
            mont.ign = myRandom(5, 20) as c_ushort;
            match myRandom(0, 3) {
                0 => mont.ptrforme = addr_of!(genevr) as *const Forme as *mut Forme,
                1 => mont.ptrforme = addr_of!(chene) as *const Forme as *mut Forme,
                _ => mont.ptrforme = addr_of!(peupli) as *const Forme as *mut Forme,
            }
            index = AjouteObjet3d(1);
            if PremArbre == 0 {
                PremArbre = index;
            }
            (*aigoual).add(index as usize).write(mont);
        }
        // Les rochers
        for _ in 0..nbRochers {
            loop {
                mont.posx = (myRandom(0, 32767) * 2) as c_ushort;
                mont.posy = (myRandom(0, 32767) * 2) as c_ushort;
                if !EstDansMontagne(mont.posx, mont.posy) {
                    break;
                }
            }
            mont.ign = myRandom(15, 60) as c_ushort;
            mont.ptrforme = addr_of!(rocher) as *const Forme as *mut Forme;
            index = AjouteObjet3d(1);
            if PremRocher == 0 {
                PremRocher = index;
            }
            (*aigoual).add(index as usize).write(mont);
        }
        #[cfg(debug_assertions)]
        {
            // printf("nbreobj %ld\n",nbreobj);
            let n = unsafe { core::ptr::addr_of!(nbreobj).read() };
            eprintln!("nbreobj {}", n);
        }
        //GenereHommes();
    }
}

/***********************************************************************
                         WaitTick
=> nombre de tickcount à attendre
***********************************************************************/
fn WaitTick(nb: c_short) {
    let t = TickCount();
    while TickCount() < t + nb as c_long { /* Attente active */ }
}

/***********************************************************************
                         Final
Gestion de l'animation finale
***********************************************************************/
pub unsafe extern "C" fn Final() {
    unsafe {
        let mut n: c_long = 0;
        let mut dx: c_long;
        let mut dy: c_long;
        let mut dist: c_short;
        let mut index: c_long;
        let mut mont: Mont = Mont::default();
        let mut rgb: RGBColor = RGBColor {
            red: 0,
            green: 0,
            blue: 0,
        };
        let mut chaine: [c_char; 255] = [0; 255];

        SetCursor(GetCursor(watchCursor as i16));
        DefautDecor3d();
        heure = 0;
        while heure < 40 {
            chrono(1);
        }
        jour_dsk = 0;
        xj = 0x5800;
        yj = 0x5400;
        dzeta = 768;
        zj = 40;
        clrbadi();
        comment(133); /* Le temps passe */
        for _ in 0..5 {
            /* Les arbres augmentent en nombre */
            for _ in 0..15 {
                for e in 0..nbNuages {
                    let p = (*aigoual).add(e as usize);
                    (*p).posx = (xj as c_long + myRandom(-5000, 5000)) as c_ushort;
                    (*p).posy = (yj as c_long + myRandom(-5000, 5000)) as c_ushort;
                }
                GereInterface();
                for i in PremArbre..PremArbre + nbArbres {
                    let p = (*aigoual).add(i as usize);
                    dx = ((*p).posx as c_long - xj as c_long).abs();
                    dy = ((*p).posy as c_long - yj as c_long).abs();
                    dist = distance(dx, dy);
                    /* L'arbre est éloigné de plus de 2000 m */
                    if dist > 2000 && myRandom(0, 1000) > 900 {
                        /* On le rapproche */
                        (*p).posx = (xj as c_long + myRandom(-750, 750)) as c_ushort;
                        (*p).posy = (yj as c_long + myRandom(-5000, -500)) as c_ushort;
                        dx = ((*p).posx as c_long - xj as c_long).abs();
                        dy = ((*p).posy as c_long - yj as c_long).abs();
                        dist = distance(dx, dy);
                        if dist < 50 {
                            (*p).posx = 65000;
                            (*p).posy = 65000;
                        }
                    }
                }
                AvanceSol(1);
                heure = myRandom(0, 90) as c_ushort;
                affich(dzeta as c_ushort);
                tfrima();
                delai(0);
                WaitTick(5);
            }
            /* Erosion des montagnes */
            for i in 0..nbreobj {
                let p = (*aigoual).add(i as usize);
                if !(*p).ptrforme.is_null() && (*(*p).ptrforme).hauteur == 0 {
                    /* C'est une montagne elle diminue */
                    if (*p).ign > 10 {
                        (*p).ign -= 10;
                    }
                }
            }
            /* Déplacement des rochers */
            for i in PremRocher..PremRocher + nbRochers {
                let p = (*aigoual).add(i as usize);
                dx = ((*p).posx as c_long - xj as c_long).abs();
                dy = ((*p).posy as c_long - yj as c_long).abs();
                dist = distance(dx, dy);
                if dist > 2000 && myRandom(0, 1000) > 900 {
                    (*p).posx = (xj as c_long + myRandom(-1000, 1000)) as c_ushort;
                    (*p).posy = (yj as c_long + myRandom(-1000, -100)) as c_ushort;
                    dx = ((*p).posx as c_long - xj as c_long).abs();
                    dy = ((*p).posy as c_long - yj as c_long).abs();
                    dist = distance(dx, dy);
                    if dist < 50 {
                        (*p).posx = 65000;
                        (*p).posy = 65000;
                    }
                }
            }
            /* Les arbres diminuent en nombre */
            for _ in 0..15 {
                for e in 0..nbNuages {
                    let p = (*aigoual).add(e as usize);
                    (*p).posx = (xj as c_long + myRandom(-5000, 5000)) as c_ushort;
                    (*p).posy = (yj as c_long + myRandom(-5000, 5000)) as c_ushort;
                }
                GereInterface();
                for i in PremArbre..PremArbre + nbArbres {
                    let p = (*aigoual).add(i as usize);
                    dx = ((*p).posx as c_long - xj as c_long).abs();
                    dy = ((*p).posy as c_long - yj as c_long).abs();
                    dist = distance(dx, dy);
                    if dist < 3000 && myRandom(0, 1000) > 900 {
                        (*p).posx = 65000;
                        (*p).posy = 65000;
                    }
                }
                heure = myRandom(0, 90) as c_ushort;
                AvanceSol(1);
                affich(dzeta as c_ushort);
                tfrima();
                delai(0);
                WaitTick(5);
            }
        }
        for e in 0..nbNuages {
            let p = (*aigoual).add(e as usize);
            (*p).posx = (xj as c_long + myRandom(-2000, 2000)) as c_ushort;
            (*p).posy = (yj as c_long + myRandom(-2000, 2000)) as c_ushort;
        }
        for _ in 0..100 {
            FreqEclair = 900;
            /* Erosion des rochers */
            for i in PremRocher..PremRocher + nbRochers {
                let p = (*aigoual).add(i as usize);
                if myRandom(0, 1000) > 900 {
                    (*p).posx = 65000;
                    (*p).posy = 65000;
                }
            }
            /* Disparition des arbres */
            for i in PremArbre..PremArbre + nbArbres {
                let p = (*aigoual).add(i as usize);
                if myRandom(0, 1000) > 900 {
                    (*p).posx = 65000;
                    (*p).posy = 65000;
                }
            }
            mont.ign = myRandom(15, 60) as c_ushort;
            mont.ptrforme = addr_of!(building) as *const Forme as *mut Forme;
            mont.posx = (xj as c_long + myRandom(-750, 750)) as c_ushort;
            mont.posy = (yj as c_long + myRandom(-10000, -500)) as c_ushort;
            index = AjouteObjet3d(1);
            (*aigoual).add(index as usize).write(mont);
            heure = myRandom(0, 90) as c_ushort;
            affich(dzeta as c_ushort);
            tfrima();
            delai(0);
            chrono(1);
            WaitTick(5);
        }
        heure = 512;
        for _ in 0..400 {
            yj = yj.wrapping_sub(30);
            affich(dzeta as c_ushort);
            tfrima();
            delai(0);
            chrono(1);
            WaitTick(2);
        }
        AcamSetPortOffscreen(Ecran1);
        TextFont(0); /* times */
        DoTextSize(64);
        strcpy(chaine.as_mut_ptr(), c"THE END".as_ptr());
        CtoPstr(chaine.as_mut_ptr());
        dx = (512 - StringWidth(chaine.as_mut_ptr() as StringPtr) as c_long) / 2;
        for _ in 0..256 {
            n += 256;
            chrono(1);
            chrono(1);
            rgb.red = n as c_ushort;
            rgb.green = n as c_ushort;
            rgb.blue = n as c_ushort;
            RGBForeColor(&mut rgb);
            DoMoveTo(dx as c_short, 64);
            affich(dzeta as c_ushort);
            delai(0);
            DrawString(chaine.as_mut_ptr() as StringPtr);
            tfrima();
            WaitTick(1);
        }
        CarboniseSetCursorArrow();
        CarboniseSetPortWindow(mainWindow);
    }
}

/***********************************************************************
                         FoudroieJoueur
Applique un éclair sur le plan à la position donnée
**********************************************************************/
unsafe fn FoudroieJoueur(x: c_ushort, y: c_ushort) {
    unsafe {
        /* Le joueur est foudroyé par l'éclair latéral */
        if distance(xj as c_long - x as c_long, yj as c_long - y as c_long)
            < DistMinFoudroie as c_short
        {
            clrbadi();
            comment(134);
            for _ in 0..100 {
                effima(0x077);
                tfrima();
                effima(0x000);
                tfrima();
            }
            /* Le joueur est mort */
            mort = 0;
        }
    }
}

/***********************************************************************
                         FoudroieArbre
Applique un éclair sur le plan à la position donnée
**********************************************************************/
unsafe fn FoudroieArbre(x: c_ushort, y: c_ushort) -> MontPtr {
    unsafe {
        let mut mont: MontPtr = *aigoual;
        let mut i: c_long;

        FoudroieJoueur(x, y);
        i = nbreobj;
        while i > 0 {
            i -= 1;
            /* C'est un arbre et même coordonnée */
            if (*(*mont).ptrforme).hauteur == 1
                && distance(
                    (*mont).posx as c_long - x as c_long,
                    (*mont).posy as c_long - y as c_long,
                ) < DistMinFoudroie as c_short
            {
                (*mont).ptrforme = addr_of!(toucheEclair) as *const Forme as *mut Forme;
                FoudroieJoueur((*mont).posx, (*mont).posy);
                return mont;
            }
            mont = mont.add(1);
        }
        null_mut()
    }
}

/***********************************************************************
                         ChargeEltsDecorsMobiles
***********************************************************************/
pub unsafe extern "C" fn ChargeEltsDecorsMobiles(f: *mut FILE) {
    unsafe {
        let mut chaine: [c_char; 255] = [0; 255];
        let mut x: c_long = 0;
        let mut y: c_long = 0;
        let mut i: c_long;
        let mut mont: MontPtr;

        fscanf(f, c" %s".as_ptr(), chaine.as_mut_ptr());
        if strcmp(chaine.as_ptr(), c"MAGIC".as_ptr()) != 0 {
            return;
        }
        while feof(f) == 0 {
            chaine[0] = 0;
            fscanf(f, c"%s".as_ptr(), chaine.as_mut_ptr());
            if strcmp(chaine.as_ptr(), c"Foudroye".as_ptr()) == 0 {
                fscanf(f, c" %ld %ld".as_ptr(), &mut x, &mut y);
                i = nbreobj;
                mont = *aigoual;
                while i > 0 {
                    i -= 1;
                    if (*(*mont).ptrforme).hauteur == 1
                        && (*mont).posx == x as c_ushort
                        && (*mont).posy == y as c_ushort
                    {
                        (*mont).ptrforme = addr_of!(foudroye) as *const Forme as *mut Forme;
                    }
                    mont = mont.add(1);
                }
            }
        }
    }
}

/***********************************************************************
                         SauveEltsDecorsMobiles
Sauve dans le fichier aventure en entrée les éléments variables du décor :
arbres foudroyés...
**********************************************************************/
pub unsafe extern "C" fn SauveEltsDecorsMobiles(f: *mut FILE) {
    unsafe {
        let mut mont: MontPtr = *aigoual;
        let mut i: c_long = nbreobj;

        fprintf(f, c"\nMAGIC\n".as_ptr());
        while i > 0 {
            i -= 1;
            /* C'est un arbre et même coordonnée */
            if (*mont).ptrforme == addr_of!(foudroye) as *const Forme as *mut Forme {
                fprintf(
                    f,
                    c"Foudroye %ld %ld\n".as_ptr(),
                    (*mont).posx as c_long,
                    (*mont).posy as c_long,
                );
            }
            mont = mont.add(1);
        }
    }
}

/***********************************************************************
                         AFFICH : Affiche 3 d
L'affichage se fait sur l'écran1
**********************************************************************/
pub unsafe extern "C" fn affich(tfr: c_ushort) {
    unsafe {
        let dzetabak: c_short;
        let mut suivant: MontPtr;

        dzetabak = dzeta;
        dzeta = tfr as c_short;

        /* On efface le fond */
        eff3d(
            ciel,
            terre,
            metpos,
            (tfr as c_long + 128 - angsol as c_long) as c_ushort,
            if (heure & 0x200) == 0x200 { 1 } else { 0 },
        );
        /*met la fumée en place*/
        fumee.numMob3d = myRandom(18, 20) as i16;
        forme8.numMob3d = if forme8.numMob3d + 1 == 32 {
            30
        } else {
            forme8.numMob3d + 1
        };
        oiseau.numMob3d = if oiseau.numMob3d + 1 == MOB_OISEAU + 2 {
            MOB_OISEAU
        } else {
            oiseau.numMob3d + 1
        };
        homme.numMob3d = if homme.numMob3d + 1 == MOB_HOMME + 2 {
            MOB_HOMME
        } else {
            homme.numMob3d + 1
        };
        /*déplace nuages vers l'ouest*/
        for i in 0..nbNuages {
            let p = (*aigoual).add(i as usize);
            (*p).posx = (*p).posx.wrapping_add(VitesseDeplacementNuages as c_ushort);
        }
        // Délai avant le premier éclair
        if FreqEclair > 1000 {
            /* Pas d'orage en cours, on en génère un ? */
            if myRandom(0, 10000) > FreqOrage {
                FreqEclair = 0;
            }
        } else {
            FreqEclair += 4;
        }
        if !ArbreFoudroye.is_null() {
            (*ArbreFoudroye).ptrforme = addr_of!(foudroye) as *const Forme as *mut Forme;
            ArbreFoudroye = null_mut();
        }
        suivant = (*aigoual).add(PremEclair as usize);
        for _ in 0..nbEclairs {
            /* Fin de l'éclair */
            if (*suivant).ign != 0 {
                if ArbreFoudroye.is_null() {
                    ArbreFoudroye = FoudroieArbre((*suivant).posx, (*suivant).posy);
                }
                (*suivant).ign = 0;
                #[cfg(debug_assertions)]
                {
                    // if(suivant->ptrforme->numMob3d==7) Collapse("err");
                    if (*(*suivant).ptrforme).numMob3d == 7 {
                        Collapse(c"err".as_ptr() as *mut c_char);
                    }
                }
            } else if myRandom(0, 1000) > FreqEclair && myRandom(0, 1000) > 910 {
                (*suivant).ign = 300; /* Demarre l'éclair */
            }
            /* on déplace toujours l'éclair de façon à ce qu'il soit collé au nuage */
            (*suivant).posx = (*suivant)
                .posx
                .wrapping_add(VitesseDeplacementNuages as c_ushort);
            suivant = suivant.add(1);
        }
        /* Déplace oiseaux vers le sud */
        for i in 0..nbOiseaux {
            let p = (*aigoual).add((nbNuages + i) as usize);
            (*p).posy = (*p).posy.wrapping_add(VITESSE_DEPLACEMENT_OISEAUX);
        }

        /*on trie les montagnes, les nuages, et les arbres*/
        let p0 = *aigoual;
        let old_chaine = (*p0).chaine;
        (*p0).chaine = null_mut(); /* départ de la chaîne sur 1er elt */
        /* (old_chaine conservé par fidélité au C) */
        let _ = old_chaine;
        for i in 1..nbreobj {
            ChaineObjet((*aigoual).add(i as usize));
        }
        /* Deplacement des objets vivants */
        //DeplaceHommes();
        /*on affiche*/
        suivant = (*(*aigoual)).chaine;
        /*on envoie la position en mémoire de l'elt*/
        if !suivant.is_null() {
            visu(suivant);
        }
        /* On traite la pluie */
        Pluie();
        dzeta = dzetabak;
    }
}

/***********************************************************************
                        CLAV3D Routine maitre durant la 3d
***********************************************************************/
static mut okcarte: c_short = 0;

pub unsafe extern "C" fn clav3d() {
    unsafe {
        let mut car: c_short;
        let mut cptemps: c_short = 1;
        let mut cpt_cart: c_short = 0;
        let mut cont: Boolean = VRAI;
        let mut posTimer: c_ulong;

        dismin = 0;
        //reset_mouse();
        /* On ne remet à zéro que la position en colonne */
        loop {
            vq_mouse(&raw mut bouton, &raw mut dsposx, &raw mut sposy);
            if bouton == 0 {
                break;
            }
        }

        loop {
            posTimer = sapiensTimer;
            //bouge_souris();
            affich(dzeta as c_ushort);
            tfrima();
            if okcarte != 0 {
                if cpt_cart == 0 {
                    cpt_cart = 5;
                    carte();
                } else {
                    cpt_cart -= 1;
                }
            }
            sapdep(dzeta, sapvit as c_short);
            if cptemps == 1 {
                cptemps = 5;
                chrono(0);
            } else {
                cptemps -= 1;
                chrono(1);
            }
            zoninf();
            PosePluie();
            car = getkey3d();
            if car != 0 {
                if (car & 1) != 0 && sapvit < 20 {
                    icone = mobBambou + 23;
                    sapvit += 2;
                }
                if (car & 2) != 0 && sapvit != 0 {
                    icone = mobBambou + 24;
                    if sapvit < 4 {
                        sapvit = 0;
                    } else {
                        sapvit -= 4;
                    }
                }
                if (car & 8) != 0 {
                    icone = mobBambou + 26;
                    dzeta = (dzeta - 8) & 0x3FF;
                }
                if (car & 4) != 0 {
                    icone = mobBambou + 25;
                    dzeta = (dzeta + 8) & 0x3FF;
                }
                /* si bouton de gauche */
                if (car & 32) != 0 {
                    while (getkey3d() & 32) != 0 { /* Anti repeat */ }
                    okcarte ^= 1;
                    if okcarte != 0 {
                        cpt_cart = 0;
                    } else {
                        affomb(crate::DEBFLECH!() + 17, 180, 280, 5, 5, VRAI); /* OFF */
                    }
                }
                /* Sortie */
                if (car & 64) != 0 {
                    cont = FAUX;
                }
            } else {
                icone = mobBambou + 20;
            }
            /* Synchronisation de la trame */
            while posTimer > sapiensTimer.wrapping_sub(9) { /* Attente de trame */ }

            if presence != 0 && mort != 0 && cont {
                break;
            }
        }
        /* On transfère l'écran1 sur debecr */
        tfrvirt();
        /* Une présence humaine ? */
        if presence != 0 && presence <= 4 {
            /* On ouvre par défaut le menu 'dire' */
            InverseOption(0, 4);
            s_action = 5;
            inimen(2);
        }
    }
}
