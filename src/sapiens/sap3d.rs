/***********************************************************************
                Module de Gestion de la Trois D v 2.0
                (C) God Le 4 Décembre 1986
***********************************************************************/
// Le moteur 3D : formes, objets 3D (ObjetsRef), décor (nuages, éclairs,
// oiseaux, arbres, rochers) et animation finale.
// Consommé par les autres modules du port (SAPGEST.C → sapgest.rs, etc.).
use crate::acam::{CarboniseSetCursorArrow, FAUX, GetCursor, SetCursor, TickCount, VRAI};
use crate::cbridge::watchCursor;
use crate::model::RGBColor;
use crate::port::PortId;
use crate::sapiens::Sapiens;
use crate::sapiens::sapw3d::distance;
use crate::sapiens::sapwbrt::FileError;
use crate::sapiens::sapwind::ValideUpdate;
use crate::sapiens::{
    BASEMOB, DIST_MIN_FOUDROIE, FREQ_ORAGE, Forme, FormeId, Mont, MontId, NB_ARBRES, NB_ARBRES_MAX,
    NB_HERBE_AV_PLAN, NB_HOMMES, NB_MAX_HERBE_AV_PLAN, NB_NUAGES, NB_NUAGES_MAX, NB_OISEAUX,
    NB_OISEAUX_MAX, NB_ROCHERS, NB_ROCHERS_MAX,
};
use crate::shared::Boolean;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicI16, Ordering};

const VITESSE_DEPLACEMENT_OISEAUX: u16 = 10;
const MONT1: i16 = 1;
const MONT2: i16 = 2;
const MONT3: i16 = 3;
const MONT4: i16 = 4;
const MONT5: i16 = 5;
const TOTEM: i16 = 6;
const NUAGE: i16 = 7;
const FUMEE: i16 = 18;
const FOYER: i16 = 33;
const MOB_OISEAU: i16 = 28;
const MOB_HOMME: i16 = 35;
const ECLAIR: i16 = 36;
const MOB_FOUDROYE: i16 = 38;

/// Une forme animée : seule l'image courante `numMob3d` varie. Elle est
/// pilotée par une atomique pour éviter un `static mut` (fidèle à l'ancien
/// `ptrforme` du C).
struct FormeAnimee {
    base: Forme,
    image: AtomicI16,
}

impl FormeAnimee {
    const fn new(base: Forme) -> Self {
        let numMob3d = base.numMob3d;
        Self {
            base,
            image: AtomicI16::new(numMob3d),
        }
    }

    /// Copie de la forme avec l'image d'animation courante.
    fn forme(&self) -> Forme {
        let mut f = self.base;
        f.numMob3d = self.image.load(Ordering::Relaxed);
        f
    }

    /// Fixe l'image courante (fumée : valeur aléatoire).
    fn set_image(&self, image: i16) {
        self.image.store(image, Ordering::Relaxed);
    }

    /// Image suivante, en bouclant sur `[debut, fin[`.
    fn avance(&self, debut: i16, fin: i16) {
        let image = self.image.load(Ordering::Relaxed);
        let suivante = if image + 1 == fin { debut } else { image + 1 };
        self.image.store(suivante, Ordering::Relaxed);
    }
}

/// Toutes les formes du moteur 3d (29 entrées, dont 4 animées).
struct Formes {
    forme1: Forme,
    forme2: Forme,
    forme4: Forme,
    forme6: Forme,
    forme7: Forme,
    forme8: FormeAnimee,
    nuage1: Forme,
    nuage2: Forme,
    nuage3: Forme,
    eclair: Forme,
    toucheEclair: Forme,
    foudroye: Forme,
    peupli: Forme,
    chene: Forme,
    noiset: Forme,
    fontaine: Forme,
    totem: Forme,
    hutte: Forme,
    oiseau: FormeAnimee,
    genevr: Forme,
    hetre: Forme,
    roc1: Forme,
    roc2: Forme,
    roc3: Forme,
    fumee: FormeAnimee,
    foyer: Forme,
    homme: FormeAnimee,
    building: Forme,
    rocher: Forme,
}

/// Registre unique des formes : les entrées animées (`forme8`, `oiseau`,
/// `fumee`, `homme`) ne changent que par leur `image`.
#[rustfmt::skip]
static FORMES: Formes = Formes {
    forme1: Forme {
        nbrdisk: 5,
        relief: 0,
        hauteur: 0,
        rayon: 900,
        dskrayon: [800,660,150,100,1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        dskdelta: [9,9, 6,8, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        num_mob: BASEMOB,
        numMob3d: MONT1,
    },
    forme2: Forme {
        nbrdisk: 8,
        relief: 2,
        hauteur: 0,
        rayon: 1900,
        dskrayon: [1900,1700,1100,1000,950,625,520,300, 0, 0, 0, 0, 0, 0, 0, 0, ],
        dskdelta: [0,10, 26,0, 0,1,0,0, 200u8 as i8,0, 200u8 as i8,0, 200u8 as i8, 200u8 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        num_mob: BASEMOB+1,
        numMob3d: MONT2,
    },

    forme4: Forme {
        nbrdisk: 9,
        relief: 1,
        hauteur: 0,
        rayon: 5000,
        dskrayon: [4100,3700,3250,2900,2500,2000,1500,1000,250, 0, 0, 0, 0, 0, 0, 0, ],
        dskdelta: [240u8 as i8, 240u8 as i8, 80,80, 0,1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        num_mob: BASEMOB + 2,
        numMob3d: MONT4,
    },

    forme6: Forme {
        nbrdisk: 14,
        relief: 1,
        hauteur: 0,
        rayon: 10000,
        dskrayon: [9100,7400,6900,6490,5600,4000,3150,2500,2000,1500,1200, 850,550,200, 0, 0, ],
        dskdelta:
        [0,0 ,0,0 ,0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        num_mob: BASEMOB+3,
        numMob3d: MONT3,
    },

    forme7: Forme {
        nbrdisk: 16,
        relief: 1,
        hauteur: 0,
        rayon: 8000,
        dskrayon: [
            6500,4900,4100,3200,2700,2500,2300,2000,1500,1200,1100,1050,
            900,725,525,300,
        ],
        dskdelta: [
            0,0, 0,0, 0,16, 16,16, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ],
        num_mob: BASEMOB+4,
        numMob3d: MONT5,
    },

    forme8: FormeAnimee::new(Forme {
        nbrdisk: 12,
        relief: 1,
        hauteur: 0,
        rayon: 2000,
        dskrayon: [
            6000,4000,3100,2300,1900,1600,1400,1200,1100,900,500,100, 0, 0, 0, 0,
        ],
        dskdelta: [
            0,0, 0,0, 0,16, 48,16, 0,0 ,0,0, 0,0, 0,0, 128u8 as i8,9, 128u8 as i8,8, 0,0,
            0,0, 0, 0, 0, 0, 0, 0, 0,0,
        ],
        num_mob: BASEMOB+5,
        numMob3d: 30,
    }),

    nuage1: Forme {
        nbrdisk: 8,
        relief: 3,
        hauteur: 6000,
        rayon: 3500,
        dskrayon: [
            7500,6500,5500,7600,7000,3000,3700,20, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        dskdelta: [
            0,0, 192u8 as i8, 192u8 as i8, 244u8 as i8, 244u8 as i8, 0, 0, 192u8 as i8, 192u8 as i8, 244u8 as i8, 244u8 as i8, 0, 0, 192u8 as i8, 192u8 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: NUAGE,
    },

    nuage2: Forme {
        nbrdisk: 9,
        relief: 3,
        hauteur: 4000,
        rayon: 3500,
        dskrayon: [
            4000,5000,5500,5000,1000,4000,3000,500,3000, 0, 0, 0, 0, 0, 0, 0,
        ],
        dskdelta: [
            -1,0, 192u8 as i8, 192u8 as i8, 244u8 as i8, 244u8 as i8, 0,0, 15,0, 244u8 as i8, 244u8 as i8, 0,0, -1,-1, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: NUAGE+1,
    },

    nuage3: Forme {
        nbrdisk: 5,
        relief: 3,
        hauteur: 3000,
        rayon: 3500,
        dskrayon: [4000,3200,3000,1000,900, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            -1,0, 192u8 as i8, 192u8 as i8, 0,4, 0, 208u8 as i8, 192u8 as i8, 192u8 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: NUAGE+2,
    },

    eclair: Forme {
        nbrdisk: 10,
        relief: 3,
        hauteur: 100,
        rayon: 500,
        dskrayon: [4000,3200,3000,1000,900, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            -1,0, 192u8 as i8, 192u8 as i8, 0,4, 0, 208u8 as i8, 192u8 as i8, 192u8 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: ECLAIR,
    },

    toucheEclair: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 2,
        rayon: 60,
        dskrayon: [4,4,20,23,20,22,21,15,10,1, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,6, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: 37,
    },

    foudroye: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 1,
        rayon: 60,
        dskrayon: [4,4,20,23,20,22,21,15,10,1, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,6, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: MOB_FOUDROYE,
    },

    peupli: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 1,
        rayon: 60,
        dskrayon: [4,4,20,23,20,22,21,15,10,1, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,6, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: 10,
    },

    chene: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 1,
        rayon: 40,
        dskrayon: [10,6,10,40,45,47,40,35,26,5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 0,0, 128u8 as i8,5, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: 11,
    },

    noiset: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 1,
        rayon: 40,
        dskrayon: [10,6,20,50,45,47,30,25,16,5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: 12,
    },

    fontaine: Forme {
        nbrdisk: 1,
        relief: 4,
        hauteur: 2,
        rayon: 40,
        dskrayon: [10,6,20,50,45,47,30,25,16,5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: 24,
    },

    totem: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 2,
        rayon: 40,
        dskrayon: [10,6,20,50,45,47,30,25,16,5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: TOTEM,
    },

    hutte: Forme {
        nbrdisk: 1,
        relief: 4,
        hauteur: 2,
        rayon: 40,
        dskrayon: [10,6,20,50,45,47,30,25,16,5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: 32,
    },

    oiseau: FormeAnimee::new(Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 500,
        rayon: 40,
        dskrayon: [10,6,20,50,45,47,30,25,16,5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: MOB_OISEAU,
    }),

    genevr: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 1,
        rayon: 40,
        dskrayon: [4,4,15,13,11,9,8,6,4,2, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8,5,  0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: 13,
    },

    hetre: Forme {
        nbrdisk: 10,
        relief: 4,
        hauteur: 1,
        rayon: 40,
        dskrayon: [6,5,4,3,22,19,20,16,12,2, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0,0, 0,0, 0,0, 0,0, 0,0, 128u8 as i8,5, 0,0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+6,
        numMob3d: 14,
    },

    roc1: Forme {
        nbrdisk: 8,
        relief: 1,
        hauteur: 0,
        rayon: 210,
        dskrayon: [200,80,70,60,50,10,5,2, 0,0, 0,0, 0,0, 0,0],
        dskdelta: [
        0,0, 0,2, 1,3, 0, 0, 0,0, 0,0, 0,0,0, 0,0, 0, 0, 0,0, 0,0, 0,0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB+7,
        numMob3d: 15,
    },

    roc2: Forme {
        nbrdisk: 7,
        relief: 1,
        hauteur: 0,
        rayon: 600,
        dskrayon: [500, 400, 225, 190, 160, 120, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0, 0, 2, 2, 1, 3, 0, 10, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB + 7,
        numMob3d: 16,
    },

    roc3: Forme {
        nbrdisk: 8,
        relief: 1,
        hauteur: 0,
        rayon: 600,
        dskrayon: [
            3000, 2500, 2200, 1800, 800, 750, 650, 600, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        dskdelta: [
            0, 0, 2, 10, -2, 3, 0, 10, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB + 7,
        numMob3d: 17,
    },

    fumee: FormeAnimee::new(Forme {
        nbrdisk: 9,
        relief: 3,
        hauteur: 2,
        rayon: 40,
        dskrayon: [2, 10, 20, 70, 100, 160, 100, 200, 20, 0, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0, 0, 2, 2, 15, 15, 15, 15, 0, 0, 15, 15, 16, 16, 32, 32, 15, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB + 8,
        numMob3d: FUMEE,
    }),

    foyer: Forme {
        nbrdisk: 1,
        relief: 4,
        hauteur: 2,
        rayon: 20,
        dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0, 0, 0, 0, 0, 0, 128u8 as i8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: FOYER,
    },

    homme: FormeAnimee::new(Forme {
        nbrdisk: 1,
        relief: 4,
        hauteur: 1,
        rayon: 20,
        dskrayon: [10, 6, 20, 50, 45, 47, 30, 25, 16, 5, 0, 0, 0, 0, 0, 0],
        dskdelta: [
            0, 0, 0, 0, 0, 0, 128u8 as i8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1,
        numMob3d: 35,
    }),

    building: Forme {
        nbrdisk: 12,
        relief: 1,
        hauteur: 0,
        rayon: 100,
        dskrayon: [
            6000, 4000, 3100, 2300, 1900, 1600, 1400, 1200, 1100, 900, 500, 100, 0, 0, 0, 0,
        ],
        dskdelta: [
            0, 0, 0, 0, 0, 16, 48, 16, 0, 0, 0, 0, 0, 0, 0, 0, 128u8 as i8, 9, 128u8 as i8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: BASEMOB + 5,
        numMob3d: 35,
    },

    rocher: Forme {
        nbrdisk: 1,  /* Nombre de disques : 1 la hauteur sera finement réglée par l'objet */
        relief: 4 , /* Type de relief, inutilisé */
        hauteur: 1,  /* Hauteur 0=montagne,1=arbre */
        rayon: 40, /* Rayon apprécié : va conditionner l'aire horizontale */
        dskrayon: [10,6,20,50,45,47,30,25,16,5, 0, 0, 0, 0, 0, 0],/* Disques : inutilisés */
        dskdelta: [
            0,0, 0,0, 0,0, 128u8 as i8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        num_mob: -1, /* Si -1 Pas de représentation en carte */
        numMob3d: 34   /* Numéro du mob dans Sapw3d */
    },
};

impl FormeId {
    /// Copie de la forme correspondant à l'identifiant, telle qu'animée
    /// actuellement (`numMob3d` courant pour `forme8`, `oiseau`, `fumee`).
    ///
    /// Retournée par valeur : il n'y a plus de `static mut` ni de pointeur
    /// brut, comme le faisait l'ancien `ptrforme`.
    pub(crate) fn forme(self) -> Forme {
        match self {
            FormeId::Forme1 => FORMES.forme1,
            FormeId::Forme2 => FORMES.forme2,
            FormeId::Forme4 => FORMES.forme4,
            FormeId::Forme6 => FORMES.forme6,
            FormeId::Forme7 => FORMES.forme7,
            FormeId::Forme8 => FORMES.forme8.forme(),
            FormeId::Nuage1 => FORMES.nuage1,
            FormeId::Nuage2 => FORMES.nuage2,
            FormeId::Nuage3 => FORMES.nuage3,
            FormeId::Eclair => FORMES.eclair,
            FormeId::ToucheEclair => FORMES.toucheEclair,
            FormeId::Foudroye => FORMES.foudroye,
            FormeId::Peupli => FORMES.peupli,
            FormeId::Chene => FORMES.chene,
            FormeId::Noiset => FORMES.noiset,
            FormeId::Fontaine => FORMES.fontaine,
            FormeId::Totem => FORMES.totem,
            FormeId::Hutte => FORMES.hutte,
            FormeId::Oiseau => FORMES.oiseau.forme(),
            FormeId::Genevr => FORMES.genevr,
            FormeId::Hetre => FORMES.hetre,
            FormeId::Roc1 => FORMES.roc1,
            FormeId::Roc2 => FORMES.roc2,
            FormeId::Roc3 => FORMES.roc3,
            FormeId::Fumee => FORMES.fumee.forme(),
            FormeId::Foyer => FORMES.foyer,
            FormeId::Building => FORMES.building,
            FormeId::Rocher => FORMES.rocher,
        }
    }
}

const NB_OBJ_REF: usize = 223;
#[rustfmt::skip]
static OBJETS_REF: [Mont; NB_OBJ_REF] = [
    Mont { chaine: None, distance: 00, posx: 20000, posy: 1590, ign: 20, forme: Some(FormeId::Forme1) },
    Mont { chaine: None, distance: 00, posx: 5120, posy: 53760, ign: 20, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 26000, posy: 16000, ign: 100, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 9984, posy: 16860, ign: 140, forme: Some(FormeId::Forme6) },
    Mont { chaine: None, distance: 00, posx: 36133, posy: 53402, ign: 200, forme: Some(FormeId::Forme7) },
    Mont { chaine: None, distance: 00, posx: 50435, posy: 17637, ign: 80, forme: Some(FormeId::Forme8) },
                                                /* Pieds agiles */
    Mont { chaine: None, distance: 00, posx: 22656, posy: 21632, ign: 80, forme: Some(FormeId::Fumee) }, // 22672,21632
    Mont { chaine: None, distance: 00, posx: 22656, posy: 21632, ign: 15, forme: Some(FormeId::Foyer) },
    Mont { chaine: None, distance: 00, posx: 22704, posy: 21632, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 22640, posy: 21632, ign: 100, forme: Some(FormeId::Hutte) },
                                                /* Camps 2 */
    Mont { chaine: None, distance: 00, posx: 39680, posy: 14336, ign: 60, forme: Some(FormeId::Fumee) }, // 39680,14336
    Mont { chaine: None, distance: 00, posx: 39680, posy: 14336, ign: 15, forme: Some(FormeId::Foyer) },
    Mont { chaine: None, distance: 00, posx: 39744, posy: 14400, ign: 140, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 39808, posy: 14432, ign: 100, forme: Some(FormeId::Hutte) }, 
    Mont { chaine: None, distance: 00, posx: 39872, posy: 14432, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 39808, posy: 14592, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 39552, posy: 14592, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 39552, posy: 14080, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 39424, posy: 14400, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 39424,posy: 14848,ign: 100, forme: Some(FormeId::Hutte) },

    Mont { chaine: None, distance: 00, posx: 12032, posy: 52992, ign: 60, forme: Some(FormeId::Fumee) },       // 12032,52992
    Mont { chaine: None, distance: 00, posx: 12032, posy: 52992, ign: 15, forme: Some(FormeId::Foyer) },
    Mont { chaine: None, distance: 00, posx: 11892, posy: 53082, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 11992, posy: 53162, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00,posx: 11792, posy: 52992,ign: 100, forme: Some(FormeId::Hutte) },
                                                /* Camps 4 : hyenes folles */
Mont {chaine: None,distance: 00, posx: 49920, posy: 43520, ign: 70, forme: Some(FormeId::Fumee) },       // 49920,43520
    Mont { chaine: None, distance: 00, posx: 49920, posy: 43520, ign: 15, forme: Some(FormeId::Foyer) },
    Mont { chaine: None, distance: 00, posx: 50120, posy: 43720, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None,distance: 00, posx: 50220, posy: 43720, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 50220,posy: 43820, ign: 100, forme: Some(FormeId::Hutte) },
    Mont { chaine: None, distance: 00, posx: 49952, posy: 43520, ign: 15, forme: Some(FormeId::Totem) },

    Mont { chaine: None, distance: 00, posx: 4000, posy: 4000, ign: 20, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00,posx: 9000, posy: 5000,ign: 33, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 9000, posy: 5000, ign: 30, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00,posx: 13000, posy: 3500, ign: 16, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 8000,posy: 1800, ign: 26, forme: Some(FormeId::Forme2) },
    Mont {chaine: None, distance: 00,posx: 40000, posy: 5000, ign: 33, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 25000,posy: 52000, ign: 30, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 9000, posy: 5000, ign: 30, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 37000,posy: 30000, ign: 16, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 32000, posy: 32000, ign: 33, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00,posx: 43000,posy: 42000, ign: 25, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00,posx: 4000, posy: 50000, ign: 46, forme: Some(FormeId::Forme2) },
    Mont { chaine: None, distance: 00, posx: 33000, posy: 8000, ign: 120, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 55000, posy: 5000, ign: 100, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 38000, posy: 23500, ign: 140, forme: Some(FormeId::Forme7) },
    Mont { chaine: None, distance: 00, posx: 28000,posy: 26000,ign: 100,forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00,posx: 13000, posy: 37000, ign: 120, forme: Some(FormeId::Forme7) },
    Mont { chaine: None, distance: 00, posx: 37000, posy: 37000, ign: 100, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 47000, posy: 35000, ign: 100, forme: Some(FormeId::Forme4) },
    Mont {chaine: None,distance: 00, posx: 33000, posy: 8000, ign: 120,forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 55000, posy: 42000,ign: 120, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00,posx: 5000, posy: 32000,ign: 140, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 20000, posy: 30000, ign: 120, forme: Some(FormeId::Forme4) },
    Mont { chaine: None, distance: 00, posx: 25000, posy: 40000, ign: 120, forme: Some(FormeId::Forme7) },
    Mont { chaine: None, distance: 00, posx: 22656, posy: 21280, ign: 8, forme: Some(FormeId::Chene) },
    Mont { chaine: None,distance: 00, posx: 22272, posy: 20992, ign: 8, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 22784, posy: 20992, ign: 8, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 44000, posy: 45800, ign: 20, forme: Some(FormeId::Totem) },
    Mont {chaine: None, distance: 00, posx: 44000, posy: 45900, ign: 20, forme: Some(FormeId::Totem) },
    Mont { chaine: None, distance: 00,posx: 43900, posy: 45800, ign: 20, forme: Some(FormeId::Totem) },
    Mont { chaine: None, distance: 00, posx: 25000, posy: 22500, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 19000, posy: 24000, ign: 8, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 13000, posy: 42500, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None,distance: 00, posx: 3000, posy: 30000, ign: 12, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 4500, posy: 46000, ign: 4, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 17000,posy: 44000, ign: 15, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 17500, posy: 49000, ign: 9, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32000, posy: 45000, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 19200, posy: 47200, ign: 12, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 9000, posy: 45000, ign: 7, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32100, posy: 15050,ign: 15, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32200, posy: 15200, ign: 12, forme: Some(FormeId::Chene) },
    Mont { chaine: None,distance: 00,posx: 32300, posy: 15400, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None,distance: 00, posx: 32400, posy: 15410, ign: 10,forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32000, posy: 15600, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32050, posy: 15700, ign: 7, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 12000, posy: 50000, ign: 7, forme: Some(FormeId::Chene) },
    Mont {chaine: None, distance: 00,posx: 32000,posy: 15800, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00,posx: 31900, posy: 15900, ign: 15, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32050, posy: 15700, ign: 7, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 33000, posy: 16000,ign: 17, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00,posx: 33500, posy: 16000, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 31900, posy: 15900,ign: 15, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00,posx: 33000, posy: 13500, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 38000, posy: 12000, ign: 7,forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32000, posy: 15000, ign: 7, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32800, posy: 18100, ign: 10,forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 33050, posy: 16500, ign: 9, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 32800, posy: 18200,ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 35000, posy: 16000, ign: 7, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 25000, posy: 21500, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None,distance: 00,posx: 10900,posy: 28000,ign: 12, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 13000, posy: 31000, ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 15000,posy: 26000,ign: 10, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 29000, posy: 20500, ign: 9, forme: Some(FormeId::Chene) },
    Mont {chaine: None, distance: 00, posx: 21700, posy: 8500, ign: 12, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00,posx: 23800, posy: 6000, ign: 9,forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 20000, posy: 5500,ign: 9, forme: Some(FormeId::Chene) },
    Mont {chaine: None,distance: 00, posx: 24000, posy: 5000, ign: 9, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 26000, posy: 10000, ign: 9,forme: Some(FormeId::Chene) },
    Mont {chaine: None,distance: 00, posx: 20000, posy: 9000, ign: 9, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 15000, posy: 6000, ign: 9, forme: Some(FormeId::Chene) },
    Mont { chaine: None, distance: 00, posx: 15500,posy: 47000, ign: 15, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 24000, posy: 48500, ign: 20, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 20500, posy: 18000, ign: 12, forme: Some(FormeId::Peupli) },
    Mont {chaine: None,distance: 00, posx: 40000, posy: 200, ign: 18, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00,posx: 34000, posy: 2000, ign: 16,forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 32000, posy: 1000, ign: 12, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00,posx: 100, posy: 41000, ign: 12, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 22000,posy: 25000, ign: 18, forme: Some(FormeId::Peupli) },
    Mont {chaine: None, distance: 00,posx: 21000, posy: 21000, ign: 20, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 13500, posy: 44000, ign: 12,forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00,posx: 33500, posy: 19000, ign: 12, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 42000,posy: 17000, ign: 20, forme: Some(FormeId::Peupli) },
    Mont {chaine: None, distance: 00, posx: 34000, posy: 17000, ign: 12, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 38000, posy: 16000, ign: 9, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 39050, posy: 14050,ign: 12, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 21000, posy: 23000, ign: 14, forme: Some(FormeId::Peupli) },
    Mont { chaine: None, distance: 00, posx: 12100, posy: 29050,ign: 12, forme: Some(FormeId::Peupli) },
    Mont {chaine: None, distance: 00, posx: 11000, posy: 30000, ign: 20, forme: Some(FormeId::Peupli) },
    Mont {chaine: None, distance: 00, posx: 13000, posy: 46000, ign: 10, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00,posx: 17500, posy: 45000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 19000, posy: 47000, ign: 10,forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 19000, posy: 48500, ign: 18, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 22000, posy: 23000, ign: 10, forme: Some(FormeId::Hetre) },
    Mont {chaine: None, distance: 00, posx: 13000, posy: 42500, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 3500, posy: 47000, ign: 10, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 7000, posy: 41000, ign: 11, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 32900, posy: 17000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 40000, posy: 14000, ign: 11, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 40000, posy: 15000, ign: 8, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 39000, posy: 12200, ign: 11, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 32800, posy: 18000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 42000,posy: 15000,ign: 18, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 41500, posy: 14000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 42500, posy: 16000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 31500, posy: 22000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 12000, posy: 29000, ign: 17,forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00,posx: 11900, posy: 29100,ign: 15, forme: Some(FormeId::Hetre) },
    Mont {chaine: None,distance: 00, posx: 12500, posy: 30000, ign: 15, forme: Some(FormeId::Hetre) },
    Mont { chaine: None, distance: 00, posx: 5000, posy: 39000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont {chaine: None, distance: 00, posx: 19250, posy: 49200, ign: 15, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 35000, posy: 43000, ign: 4, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 41000,posy: 2000, ign: 2, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 4000, posy: 40000, ign: 6, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 14000, posy: 45000, ign: 2,forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00,posx: 14000, posy: 45000, ign: 12, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 19100, posy: 47050, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00,posx: 31500, posy: 14500,ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 34000, posy: 15000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00,posx: 36000, posy: 17000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 40000,posy: 18000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 31000, posy: 19000, ign: 12,forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 14000,posy: 30000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 10000, posy: 29500, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00,posx: 13000,posy: 27000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00,posx: 40000, posy: 42000,ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 41000, posy: 44000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 47000, posy: 45000,ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00,posx: 48000, posy: 44000, ign: 10, forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 48000,posy: 48000,ign: 10,forme: Some(FormeId::Genevr) },
    Mont { chaine: None, distance: 00, posx: 21000,posy: 20000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 21000, posy: 20000, ign: 10,forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 12400, posy: 28500, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00,posx: 12400,posy: 28500, ign: 10,forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 48000, posy: 42000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 48000, posy: 42000,ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 2000, posy: 2000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 2000, posy: 2000, ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None,distance: 00,posx: 39000, posy: 800,ign: 10,forme: Some(FormeId::Noiset) },
    Mont { chaine: None,distance: 00, posx: 39000, posy: 800, ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 16000, posy: 42000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 16000, posy: 42000, ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 32000, posy: 16000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 32000,posy: 16000,ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 45000, posy: 25500, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 45000, posy: 25500, ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 6000, posy: 45000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None,distance: 00, posx: 6000, posy: 45000, ign: 10, forme: Some(FormeId::Fontaine) },
    Mont {chaine: None, distance: 00, posx: 50000, posy: 50000, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 50000, posy: 50000,ign: 5, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 24320,posy: 18944, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00,posx: 24320, posy: 18944, ign: 10, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None,distance: 00, posx: 32768, posy: 17408, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 32768, posy: 17408, ign: 5, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 40192, posy: 14336, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None, distance: 00, posx: 40192, posy: 14336, ign: 5, forme: Some(FormeId::Fontaine) },
    Mont { chaine: None, distance: 00, posx: 43776, posy: 16896, ign: 10, forme: Some(FormeId::Noiset) },
    Mont { chaine: None,distance: 00,posx: 43776, posy: 16896, ign: 5,forme: Some(FormeId::Fontaine) },

    Mont { chaine: None, distance: 00, posx: 100, posy: 39000, ign: 100, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00, posx: 15000,posy: 46000, ign: 45, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 42000, posy: 16000, ign: 75, forme: Some(FormeId::Roc3) },
    Mont {chaine: None, distance: 00, posx: 33050, posy: 17000, ign: 95, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00, posx: 5000,posy: 38000,ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 7000, posy: 39000,ign: 150, forme: Some(FormeId::Roc3) },
    Mont { chaine: None, distance: 00, posx: 4000, posy: 43000, ign: 110, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00,posx: 7000, posy: 44000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 10000, posy: 42000, ign: 150, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00,posx: 15000, posy: 43000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 15000, posy: 45000, ign: 140, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00, posx: 18000, posy: 47000, ign: 98, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 19000, posy: 50000, ign: 100, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00, posx: 42000, posy: 46000, ign: 100, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00, posx: 44000,posy: 45000,ign: 200, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 46000, posy: 42000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 38000, posy: 44000, ign: 90, forme: Some(FormeId::Roc1) },
    Mont { chaine: None, distance: 00, posx: 35000, posy: 1000,ign: 110, forme: Some(FormeId::Roc2) },
    Mont {chaine: None, distance: 00, posx: 37000, posy: 3000, ign: 100,forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00,posx: 36000, posy: 15000,ign: 110, forme: Some(FormeId::Roc2) },
    Mont {chaine: None,distance: 00, posx: 37000, posy: 3000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 39000, posy: 13000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 41000, posy: 13000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 36000, posy: 18000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 39000,posy: 17000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont {chaine: None,distance: 00, posx: 38000, posy: 12000, ign: 132, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 42000, posy: 15000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00,posx: 20000, posy: 22000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00,posx: 13000, posy: 28000, ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 11000, posy: 32000,ign: 100, forme: Some(FormeId::Roc2) },
    Mont { chaine: None, distance: 00, posx: 46000, posy: 47500, ign: 100, forme: Some(FormeId::Roc2) },
    Mont {chaine: None, distance: 00, posx: 9500, posy: 50000, ign: 100, forme: Some(FormeId::Roc3) },
    Mont { chaine: None, distance: 00,posx: 1,posy: 1, ign: 10, forme: Some(FormeId::Chene) },
];

pub(crate) struct Sap3d {
    /* Liste des objets présents */
    pub(crate) aigoual: Vec<Mont>,
    pub(crate) nbNuages: i64,
    pub(crate) nbOiseaux: i64,
    pub(crate) nbRochers: i64,
    pub(crate) nbHommes: i64,
    pub(crate) nbArbres: i64,
    pub(crate) FreqOrage: i64,
    pub(crate) DistMinFoudroie: i64,
    pub(crate) VitesseDeplacementNuages: i64,
    PremEclair: i64,
    PremArbre: i64,
    PremRocher: i64,
    ArbreFoudroye: Option<MontId>,
    nbEclairs: i64,
    pub(crate) FreqEclair: i64,
    okcarte: i16,
}

impl Default for Sap3d {
    fn default() -> Self {
        Self {
            aigoual: Vec::new(),
            nbNuages: 0,
            nbOiseaux: 0,
            nbRochers: 0,
            nbHommes: 0,
            nbArbres: 0,
            FreqOrage: 0,
            DistMinFoudroie: 0,
            VitesseDeplacementNuages: 0,
            PremEclair: 0,
            PremArbre: 0,
            PremRocher: 0,
            ArbreFoudroye: None,
            nbEclairs: 0,
            FreqEclair: 1000,
            okcarte: 0,
        }
    }
}

/// Parcourt la chaîne d'objets 3d, triés par distance décroissante.
///
/// Emprunte la liste `Sap3d::aigoual` et suit les liens `chaine`. La tête est
/// stockée dans le champ `chaine` de l'élément 0 (fidèle à l'ancien code C).
pub(crate) struct MontChaine<'a> {
    objets: &'a [Mont],
    courant: Option<MontId>,
}

impl<'a> Iterator for MontChaine<'a> {
    type Item = &'a Mont;

    fn next(&mut self) -> Option<&'a Mont> {
        let id = self.courant?;
        let mont = self.objets.get(id as usize)?;
        self.courant = mont.chaine;
        Some(mont)
    }
}

impl Sap3d {
    /// Tête de la chaîne de rendu (stockée dans le champ `chaine` de l'élément 0).
    pub(crate) fn chaine(&self) -> MontChaine<'_> {
        MontChaine {
            objets: &self.aigoual,
            courant: self.aigoual.first().and_then(|m| m.chaine),
        }
    }

    /***********************************************************************
                             EstDansMontagne
    Dit si une position est dans une montagne
    ***********************************************************************/
    fn EstDansMontagne(&self, x: u16, y: u16) -> Boolean {
        for p in &self.aigoual {
            if let Some(forme) = p.forme.map(FormeId::forme)
                && forme.hauteur == 0
            {
                let dx = (p.posx as i64 - x as i64).abs();
                let dy = (p.posy as i64 - y as i64).abs();
                let dist = distance(dx, dy);
                if dist < forme.rayon as i16 {
                    return VRAI;
                }
            }
        }
        FAUX
    }
}

impl Sapiens {
    /***********************************************************************
                             DefautDecor3d
    3D au demarrage du jeu
    **********************************************************************/
    pub(crate) fn DefautDecor3d(&mut self) {
        self.sap3d.FreqOrage = FREQ_ORAGE as i64; //9990
        self.sap3d.DistMinFoudroie = DIST_MIN_FOUDROIE as i64; //250
        self.sap3d.nbNuages = NB_NUAGES as i64; //100
        self.sap3d.nbOiseaux = NB_OISEAUX as i64; //20
        self.sap3d.nbRochers = NB_ROCHERS as i64; //300
        self.sap3d.nbHommes = NB_HOMMES as i64; //0
        self.sap3d.nbArbres = NB_ARBRES as i64; //800
        self.sapcomb.nbHerbes = NB_HERBE_AV_PLAN as i64;
        self.sap3d.FreqEclair = 1000; //1000
        self.sap3d.VitesseDeplacementNuages = 20;
    }

    /***********************************************************************
                             InitDecor3d
    Tire au hasard les fréquences à chaque nouvelle aventure
    ***********************************************************************/
    pub(crate) fn InitDecor3d(&mut self) {
        self.sap3d.FreqOrage = self.random.myRandom(9900, 10000);
        self.sap3d.DistMinFoudroie = self.random.myRandom(20, 500);
        self.sap3d.nbNuages = self.random.myRandom(NB_NUAGES as i64, NB_NUAGES_MAX as i64);
        self.sap3d.nbOiseaux = self
            .random
            .myRandom(NB_OISEAUX as i64, NB_OISEAUX_MAX as i64);
        self.sap3d.nbRochers = self
            .random
            .myRandom(NB_ROCHERS as i64, NB_ROCHERS_MAX as i64);
        self.sap3d.nbArbres = self.random.myRandom(NB_ARBRES as i64, NB_ARBRES_MAX as i64); //800
        self.sapcomb.nbHerbes = self
            .random
            .myRandom(NB_HERBE_AV_PLAN as i64, NB_MAX_HERBE_AV_PLAN as i64);
        self.sap3d.nbHommes = 0; //0
        self.sap3d.FreqEclair = 1000;
        self.sap3d.VitesseDeplacementNuages = self.random.myRandom(1, 120);
    }

    /***********************************************************************
                             CreeDecor3d
    Crée la liste des objets 3ds
    ***********************************************************************/
    pub(crate) fn CreeDecor3d(&mut self, display: Boolean) {
        let mut mont: Mont = Mont::default();

        if display {
            self.sapglob.flefba = 0;
            self.comment(137); //Je crée le monde
            ValideUpdate();
        }
        self.random.initrnd(b'M' as i16, b'Y' as i16, b'R' as i16);

        #[cfg(debug_assertions)]
        {
            // Reflet des blocs #if DEBUG>0
            let fo = self.sap3d.FreqOrage;
            let df = self.sap3d.DistMinFoudroie;
            let nn = self.sap3d.nbNuages;
            let no = self.sap3d.nbOiseaux;
            let nr = self.sap3d.nbRochers;
            let na = self.sap3d.nbArbres;
            eprintln!(
                "FreqOrage {}\nDistMinFoudroie {}\nnbNuages {}\nnbOiseaux {}\nnbRochers {}\nnbArbres {}",
                fo, df, nn, no, nr, na
            );
        }
        self.sap3d.nbEclairs = self.sap3d.nbNuages;
        self.sap3d.aigoual.clear();
        self.sap3d.PremEclair = 0;
        self.sap3d.PremArbre = 0;
        self.sap3d.PremRocher = 0;
        self.sap3d.ArbreFoudroye = None;
        for _ in 0..self.sap3d.nbNuages {
            mont.posx = (self.random.myRandom(0, 32767) * 2) as u16;
            mont.posy = (self.random.myRandom(0, 32767) * 2) as u16;
            mont.ign = self.random.myRandom(100, 200) as u16;
            match self.random.myRandom(0, 3) {
                0 => mont.forme = Some(FormeId::Nuage1),
                1 => mont.forme = Some(FormeId::Nuage2),
                _ => mont.forme = Some(FormeId::Nuage3),
            }
            self.sap3d.aigoual.push(mont);
        }
        /* À chaque nuage un éclair est associé */
        for i in 0..self.sap3d.nbEclairs {
            let nuage = self.sap3d.aigoual[i as usize]; //Copie du nuage associé
            mont = nuage;
            mont.ign = 0; //Altitude zéro inactif
            mont.forme = Some(FormeId::Eclair);
            // L'éclair est placé au centre de son nuage
            let rayon = nuage.forme.expect("nuage").forme().rayon;
            mont.posx = (mont.posx as i64 + rayon as i64 / 2) as u16;
            mont.posy = (mont.posy as i64 + rayon as i64 / 2) as u16;
            self.sap3d.aigoual.push(mont);
            if self.sap3d.PremEclair == 0 {
                self.sap3d.PremEclair = self.sap3d.aigoual.len() as i64 - 1;
            }
        }
        for _ in 0..self.sap3d.nbOiseaux {
            mont.posx = (self.random.myRandom(0, 32767) * 2) as u16;
            mont.posy = (self.random.myRandom(0, 32767) * 2) as u16;
            mont.ign = 10;
            mont.forme = Some(FormeId::Oiseau);
            self.sap3d.aigoual.push(mont);
        }
        /* On crée le tableau avec les objets de base */
        self.sap3d.aigoual.extend_from_slice(&OBJETS_REF);
        // les arbres
        for _ in 0..self.sap3d.nbArbres {
            loop {
                mont.posx = (self.random.myRandom(0, 32767) * 2) as u16;
                mont.posy = (self.random.myRandom(0, 32767) * 2) as u16;
                if !self.sap3d.EstDansMontagne(mont.posx, mont.posy) {
                    break;
                }
            }
            mont.ign = self.random.myRandom(5, 20) as u16;
            match self.random.myRandom(0, 3) {
                0 => mont.forme = Some(FormeId::Genevr),
                1 => mont.forme = Some(FormeId::Chene),
                _ => mont.forme = Some(FormeId::Peupli),
            }
            self.sap3d.aigoual.push(mont);
            if self.sap3d.PremArbre == 0 {
                self.sap3d.PremArbre = self.sap3d.aigoual.len() as i64 - 1;
            }
        }
        // Les rochers
        for _ in 0..self.sap3d.nbRochers {
            loop {
                mont.posx = (self.random.myRandom(0, 32767) * 2) as u16;
                mont.posy = (self.random.myRandom(0, 32767) * 2) as u16;
                if !self.sap3d.EstDansMontagne(mont.posx, mont.posy) {
                    break;
                }
            }
            mont.ign = self.random.myRandom(15, 60) as u16;
            mont.forme = Some(FormeId::Rocher);
            self.sap3d.aigoual.push(mont);
            if self.sap3d.PremRocher == 0 {
                self.sap3d.PremRocher = self.sap3d.aigoual.len() as i64 - 1;
            }
        }
        #[cfg(debug_assertions)]
        {
            // printf("nbreobj %ld\n",nbreobj);
            eprintln!("nbreobj {}", self.sap3d.aigoual.len());
        }
        //GenereHommes();
    }
}

/***********************************************************************
                         WaitTick
=> nombre de tickcount à attendre
***********************************************************************/
fn WaitTick(nb: i16) {
    let t = TickCount();
    while TickCount() < t + nb as i64 { /**/ }
}

impl Sapiens {
    /***********************************************************************
                             Final
    Gestion de l'animation finale
    ***********************************************************************/
    pub(crate) fn Final(&mut self) {
        let mut n: i64 = 0;
        let mut dx: i64;
        let mut dy: i64;
        let mut dist: i16;
        let mut mont: Mont = Mont::default();
        let mut rgb: RGBColor = RGBColor {
            red: 0,
            green: 0,
            blue: 0,
        };
        SetCursor(GetCursor(watchCursor as i16));
        self.DefautDecor3d();
        self.sapglob.heure = 0;
        while self.sapglob.heure < 40 {
            self.chrono(1);
        }
        self.sapglob.jour_dsk = 0;
        self.sapglob.xj = 0x5800;
        self.sapglob.yj = 0x5400;
        self.sapglob.dzeta = 768;
        self.sapglob.zj = 40;
        self.clrbadi();
        self.comment(133); /* Le temps passe */
        for _ in 0..5 {
            /* Les arbres augmentent en nombre */
            for _ in 0..15 {
                for e in 0..self.sap3d.nbNuages {
                    let mont = &mut self.sap3d.aigoual[e as usize];
                    mont.posx = (self.sapglob.xj as i64 + self.random.myRandom(-5000, 5000)) as u16;
                    mont.posy = (self.sapglob.yj as i64 + self.random.myRandom(-5000, 5000)) as u16;
                }
                self.GereInterface();
                for i in self.sap3d.PremArbre..self.sap3d.PremArbre + self.sap3d.nbArbres {
                    let mut p = self.sap3d.aigoual[i as usize];
                    dx = (p.posx as i64 - self.sapglob.xj as i64).abs();
                    dy = (p.posy as i64 - self.sapglob.yj as i64).abs();
                    dist = distance(dx, dy);
                    /* L'arbre est éloigné de plus de 2000 m */
                    if dist > 2000 && self.random.myRandom(0, 1000) > 900 {
                        /* On le rapproche */
                        p.posx = (self.sapglob.xj as i64 + self.random.myRandom(-750, 750)) as u16;
                        p.posy =
                            (self.sapglob.yj as i64 + self.random.myRandom(-5000, -500)) as u16;
                        dx = (p.posx as i64 - self.sapglob.xj as i64).abs();
                        dy = (p.posy as i64 - self.sapglob.yj as i64).abs();
                        dist = distance(dx, dy);
                        if dist < 50 {
                            p.posx = 65000;
                            p.posy = 65000;
                        }
                    }
                    self.sap3d.aigoual[i as usize] = p;
                }
                self.AvanceSol(1);
                self.sapglob.heure = self.random.myRandom(0, 90) as u16;
                self.affich(self.sapglob.dzeta as u16);
                self.sapwind.ports.tfrima();
                self.delai(0);
                WaitTick(5);
            }
            /* Erosion des montagnes */
            for i in 0..self.sap3d.aigoual.len() {
                let p = &mut self.sap3d.aigoual[i];
                if p.forme.is_some_and(|id| id.forme().hauteur == 0) {
                    /* C'est une montagne elle diminue */
                    if p.ign > 10 {
                        p.ign -= 10;
                    }
                }
            }
            /* Déplacement des rochers */
            for i in self.sap3d.PremRocher..self.sap3d.PremRocher + self.sap3d.nbRochers {
                let mut p = self.sap3d.aigoual[i as usize];
                dx = (p.posx as i64 - self.sapglob.xj as i64).abs();
                dy = (p.posy as i64 - self.sapglob.yj as i64).abs();
                dist = distance(dx, dy);
                if dist > 2000 && self.random.myRandom(0, 1000) > 900 {
                    p.posx = (self.sapglob.xj as i64 + self.random.myRandom(-1000, 1000)) as u16;
                    p.posy = (self.sapglob.yj as i64 + self.random.myRandom(-1000, -100)) as u16;
                    dx = (p.posx as i64 - self.sapglob.xj as i64).abs();
                    dy = (p.posy as i64 - self.sapglob.yj as i64).abs();
                    dist = distance(dx, dy);
                    if dist < 50 {
                        p.posx = 65000;
                        p.posy = 65000;
                    }
                }
                self.sap3d.aigoual[i as usize] = p;
            }
            /* Les arbres diminuent en nombre */
            for _ in 0..15 {
                for e in 0..self.sap3d.nbNuages {
                    let mont = &mut self.sap3d.aigoual[e as usize];
                    mont.posx = (self.sapglob.xj as i64 + self.random.myRandom(-5000, 5000)) as u16;
                    mont.posy = (self.sapglob.yj as i64 + self.random.myRandom(-5000, 5000)) as u16;
                }
                self.GereInterface();
                for i in self.sap3d.PremArbre..self.sap3d.PremArbre + self.sap3d.nbArbres {
                    let mut p = self.sap3d.aigoual[i as usize];
                    dx = (p.posx as i64 - self.sapglob.xj as i64).abs();
                    dy = (p.posy as i64 - self.sapglob.yj as i64).abs();
                    dist = distance(dx, dy);
                    if dist < 3000 && self.random.myRandom(0, 1000) > 900 {
                        p.posx = 65000;
                        p.posy = 65000;
                    }
                    self.sap3d.aigoual[i as usize] = p;
                }
                self.sapglob.heure = self.random.myRandom(0, 90) as u16;
                self.AvanceSol(1);
                self.affich(self.sapglob.dzeta as u16);
                self.sapwind.ports.tfrima();
                self.delai(0);
                WaitTick(5);
            }
        }
        for e in 0..self.sap3d.nbNuages {
            let mont = &mut self.sap3d.aigoual[e as usize];
            mont.posx = (self.sapglob.xj as i64 + self.random.myRandom(-2000, 2000)) as u16;
            mont.posy = (self.sapglob.yj as i64 + self.random.myRandom(-2000, 2000)) as u16;
        }
        for _ in 0..100 {
            self.sap3d.FreqEclair = 900;
            /* Erosion des rochers */
            for i in self.sap3d.PremRocher..self.sap3d.PremRocher + self.sap3d.nbRochers {
                if self.random.myRandom(0, 1000) > 900 {
                    self.sap3d.aigoual[i as usize].posx = 65000;
                    self.sap3d.aigoual[i as usize].posy = 65000;
                }
            }
            /* Disparition des arbres */
            for i in self.sap3d.PremArbre..self.sap3d.PremArbre + self.sap3d.nbArbres {
                if self.random.myRandom(0, 1000) > 900 {
                    self.sap3d.aigoual[i as usize].posx = 65000;
                    self.sap3d.aigoual[i as usize].posy = 65000;
                }
            }
            mont.ign = self.random.myRandom(15, 60) as u16;
            mont.forme = Some(FormeId::Building);
            mont.posx = (self.sapglob.xj as i64 + self.random.myRandom(-750, 750)) as u16;
            mont.posy = (self.sapglob.yj as i64 + self.random.myRandom(-10000, -500)) as u16;
            self.sap3d.aigoual.push(mont);
            self.sapglob.heure = self.random.myRandom(0, 90) as u16;
            self.affich(self.sapglob.dzeta as u16);
            self.sapwind.ports.tfrima();
            self.delai(0);
            self.chrono(1);
            WaitTick(5);
        }
        self.sapglob.heure = 512;
        for _ in 0..400 {
            self.sapglob.yj = self.sapglob.yj.wrapping_sub(30);
            self.affich(self.sapglob.dzeta as u16);
            self.sapwind.ports.tfrima();
            self.delai(0);
            self.chrono(1);
            WaitTick(2);
        }
        self.sapwind.ports.set(PortId::Ecran1);
        self.sapwind.ports.TextFont(0); /* times */
        self.sapwind.ports.DoTextSize(64);
        let chaine = "THE END";
        dx = (512 - self.sapwind.ports.StringWidth(chaine) as i64) / 2;
        for _ in 0..256 {
            n += 256;
            self.chrono(1);
            self.chrono(1);
            rgb.red = n as u16;
            rgb.green = n as u16;
            rgb.blue = n as u16;
            self.sapwind.ports.RGBForeColor(&rgb);
            self.sapwind.ports.DoMoveTo(dx as i16, 64);
            self.affich(self.sapglob.dzeta as u16);
            self.delai(0);
            self.sapwind.ports.DrawString(chaine);
            self.sapwind.ports.tfrima();
            WaitTick(1);
        }
        CarboniseSetCursorArrow();
        self.sapwind.ports.set(PortId::MainWindow);
    }
}

impl Sapiens {
    /***********************************************************************
                             FoudroieJoueur
    Applique un éclair sur le plan à la position donnée
    **********************************************************************/
    fn FoudroieJoueur(&mut self, x: u16, y: u16) {
        /* Le joueur est foudroyé par l'éclair latéral */
        if distance(
            self.sapglob.xj as i64 - x as i64,
            self.sapglob.yj as i64 - y as i64,
        ) < self.sap3d.DistMinFoudroie as i16
        {
            self.clrbadi();
            self.comment(134);
            for _ in 0..100 {
                self.effima(0x077);
                self.sapwind.ports.tfrima();
                self.effima(0x000);
                self.sapwind.ports.tfrima();
            }
            /* Le joueur est mort */
            self.sapglob.mort = 0;
        }
    }

    /***********************************************************************
                             FoudroieArbre
    Applique un éclair sur le plan à la position donnée
    **********************************************************************/
    fn FoudroieArbre(&mut self, x: u16, y: u16) -> Option<MontId> {
        self.FoudroieJoueur(x, y);
        for i in 0..self.sap3d.aigoual.len() {
            let mont = self.sap3d.aigoual[i];
            /* C'est un arbre et même coordonnée */
            if mont.forme.is_some_and(|id| id.forme().hauteur == 1)
                && distance(mont.posx as i64 - x as i64, mont.posy as i64 - y as i64)
                    < self.sap3d.DistMinFoudroie as i16
            {
                self.sap3d.aigoual[i].forme = Some(FormeId::ToucheEclair);
                self.FoudroieJoueur(mont.posx, mont.posy);
                return Some(i as MontId);
            }
        }
        None
    }
}

impl Sap3d {
    /***********************************************************************
                             ChargeEltsDecorsMobiles
    ***********************************************************************/
    pub(crate) fn ChargeEltsDecorsMobiles(&mut self, f: &mut impl Read) -> Result<(), FileError> {
        let mut contenu = Vec::new();
        f.read_to_end(&mut contenu)?;
        let contenu = String::from_utf8_lossy(&contenu);
        let mut tokens = contenu.split_whitespace();
        if tokens.next() != Some("MAGIC") {
            return Err(FileError::BadHeader);
        }
        while let Some(token) = tokens.next() {
            if token != "Foudroye" {
                continue;
            }
            let (Some(x), Some(y)) = (
                tokens.next().and_then(|v| v.parse::<i64>().ok()),
                tokens.next().and_then(|v| v.parse::<i64>().ok()),
            ) else {
                break;
            };
            for mont in self.aigoual.iter_mut() {
                if mont.forme.is_some_and(|id| id.forme().hauteur == 1)
                    && mont.posx == x as u16
                    && mont.posy == y as u16
                {
                    mont.forme = Some(FormeId::Foudroye);
                }
            }
        }
        Ok(())
    }

    /***********************************************************************
                             SauveEltsDecorsMobiles
    Sauve dans le fichier aventure en entrée les éléments variables du décor :
    arbres foudroyés...
    **********************************************************************/
    pub(crate) fn SauveEltsDecorsMobiles(&self, f: &mut File) -> Result<(), std::io::Error> {
        f.write_all(b"\nMAGIC\n")?;
        for mont in self.aigoual.iter() {
            /* C'est un arbre et même coordonnée */
            if mont.forme == Some(FormeId::Foudroye) {
                // c'est in format de sauvegarde il est pas dépendant de l'OS
                #[allow(clippy::write_with_newline)]
                write!(f, "Foudroye {} {}\n", mont.posx as i64, mont.posy as i64)?;
            }
        }
        Ok(())
    }
}

impl Sapiens {
    /***********************************************************************
                             AFFICH : Affiche 3 d
    L'affichage se fait sur l'écran1
    **********************************************************************/
    pub(crate) fn affich(&mut self, tfr: u16) {
        let vitesse = self.sap3d.VitesseDeplacementNuages as u16;

        let dzetabak = self.sapglob.dzeta;
        self.sapglob.dzeta = tfr as i16;

        /* On efface le fond */
        self.eff3d(
            self.sapglob.ciel,
            self.sapglob.terre,
            self.sapglob.metpos,
            (tfr as i64 + 128 - self.sapglob.angsol as i64) as u16,
            if (self.sapglob.heure & 0x200) == 0x200 {
                1
            } else {
                0
            },
        );
        /*met la fumée en place*/
        FORMES.fumee.set_image(self.random.myRandom(18, 20) as i16);
        FORMES.forme8.avance(30, 32);
        FORMES.oiseau.avance(MOB_OISEAU, MOB_OISEAU + 2);
        FORMES.homme.avance(MOB_HOMME, MOB_HOMME + 2);
        /*déplace nuages vers l'ouest*/
        for i in 0..self.sap3d.nbNuages {
            let p = &mut self.sap3d.aigoual[i as usize];
            p.posx = p.posx.wrapping_add(vitesse);
        }
        // Délai avant le premier éclair
        if self.sap3d.FreqEclair > 1000 {
            /* Pas d'orage en cours, on en génère un ? */
            if self.random.myRandom(0, 10000) > self.sap3d.FreqOrage {
                self.sap3d.FreqEclair = 0;
            }
        } else {
            self.sap3d.FreqEclair += 4;
        }
        if let Some(id) = self.sap3d.ArbreFoudroye {
            self.sap3d.aigoual[id as usize].forme = Some(FormeId::Foudroye);
            self.sap3d.ArbreFoudroye = None;
        }
        for suivant in (self.sap3d.PremEclair as usize..).take(self.sap3d.nbEclairs as usize) {
            /* Fin de l'éclair */
            if self.sap3d.aigoual[suivant].ign != 0 {
                if self.sap3d.ArbreFoudroye.is_none() {
                    let (x, y) = {
                        let m = self.sap3d.aigoual[suivant];
                        (m.posx, m.posy)
                    };
                    self.sap3d.ArbreFoudroye = self.FoudroieArbre(x, y);
                }
                self.sap3d.aigoual[suivant].ign = 0;
                #[cfg(debug_assertions)]
                {
                    // if(suivant->ptrforme->numMob3d==7) Collapse("err");
                    if self.sap3d.aigoual[suivant]
                        .forme
                        .is_some_and(|id| id.forme().numMob3d == 7)
                    {
                        self.Collapse("err");
                    }
                }
            } else if self.random.myRandom(0, 1000) > self.sap3d.FreqEclair
                && self.random.myRandom(0, 1000) > 910
            {
                self.sap3d.aigoual[suivant].ign = 300; /* Demarre l'éclair */
            }
            /* on déplace toujours l'éclair de façon à ce qu'il soit collé au nuage */
            self.sap3d.aigoual[suivant].posx =
                self.sap3d.aigoual[suivant].posx.wrapping_add(vitesse);
        }
        /* Déplace oiseaux vers le sud */
        for i in 0..self.sap3d.nbOiseaux {
            let p = &mut self.sap3d.aigoual[(self.sap3d.nbNuages + i) as usize];
            p.posy = p.posy.wrapping_add(VITESSE_DEPLACEMENT_OISEAUX);
        }

        /*on trie les montagnes, les nuages, et les arbres*/
        self.sap3d.aigoual[0].chaine = None; /* départ de la chaîne sur 1er elt */
        for i in 1..self.sap3d.aigoual.len() {
            self.ChaineObjet(i as MontId);
        }
        /* Deplacement des objets vivants */
        //DeplaceHommes();
        /*on affiche : visu lit lui-même la tête de chaîne dans l'élément 0*/
        if self.sap3d.aigoual[0].chaine.is_some() {
            self.visu();
        }
        /* On traite la pluie */
        self.Pluie();
        self.sapglob.dzeta = dzetabak;
    }
}

impl Sapiens {
    pub(crate) fn clav3d(&mut self) {
        let mut car: i16;
        let mut cptemps: i16 = 1;
        let mut cpt_cart: i16 = 0;
        let mut cont: Boolean = VRAI;
        let mut posTimer: u64;

        self.sapglob.dismin = 0;
        //reset_mouse();
        /* On ne remet à zéro que la position en colonne */
        loop {
            (self.sapglob.bouton, self.sapglob.dsposx, self.sapglob.sposy) = self.vq_mouse();
            if self.sapglob.bouton == 0 {
                break;
            }
        }
        loop {
            posTimer = self.sapwclav.sapiens_timer.load(Ordering::Relaxed);
            //bouge_souris();
            self.affich(self.sapglob.dzeta as u16);
            self.sapwind.ports.tfrima();
            if self.sap3d.okcarte != 0 {
                if cpt_cart == 0 {
                    cpt_cart = 5;
                    self.carte();
                } else {
                    cpt_cart -= 1;
                }
            }
            self.sapdep(self.sapglob.dzeta, self.sapglob.sapvit as i16);
            if cptemps == 1 {
                cptemps = 5;
                self.chrono(0);
            } else {
                cptemps -= 1;
                self.chrono(1);
            }
            self.zoninf();
            self.PosePluie();
            car = self.getkey3d();
            if car != 0 {
                if (car & 1) != 0 && self.sapglob.sapvit < 20 {
                    self.sapglob.icone = self.sapwind.mobBambou + 23;
                    self.sapglob.sapvit += 2;
                }
                if (car & 2) != 0 && self.sapglob.sapvit != 0 {
                    self.sapglob.icone = self.sapwind.mobBambou + 24;
                    if self.sapglob.sapvit < 4 {
                        self.sapglob.sapvit = 0;
                    } else {
                        self.sapglob.sapvit -= 4;
                    }
                }
                if (car & 8) != 0 {
                    self.sapglob.icone = self.sapwind.mobBambou + 26;
                    self.sapglob.dzeta = (self.sapglob.dzeta - 8) & 0x3FF;
                }
                if (car & 4) != 0 {
                    self.sapglob.icone = self.sapwind.mobBambou + 25;
                    self.sapglob.dzeta = (self.sapglob.dzeta + 8) & 0x3FF;
                }
                /* si bouton de gauche */
                if (car & 32) != 0 {
                    while (self.getkey3d() & 32) != 0 { /* Anti repeat */ }
                    self.sap3d.okcarte ^= 1;
                    if self.sap3d.okcarte != 0 {
                        cpt_cart = 0;
                    } else {
                        self.affomb(crate::DEBFLECH!(self.sapwind) + 17, 180, 280, 5, 5, VRAI); /* OFF */
                    }
                }
                /* Sortie */
                if (car & 64) != 0 {
                    cont = FAUX;
                }
            } else {
                self.sapglob.icone = self.sapwind.mobBambou + 20;
            }
            /* Synchronisation de la trame */
            while posTimer
                > self
                    .sapwclav
                    .sapiens_timer
                    .load(Ordering::Relaxed)
                    .wrapping_sub(9)
            { /* Attente de trame */ }

            if self.sapglob.presence != 0 || self.sapglob.mort == 0 || !cont {
                break;
            }
        }

        /* On transfère l'écran1 sur Debecr */
        self.tfrvirt();
        /* Une présence humaine ? */
        if self.sapglob.presence != 0 && self.sapglob.presence <= 4 {
            /* On ouvre par défaut le menu 'dire' */
            self.InverseOption(0, 4);
            self.sapglob.s_action = 5;
            self.inimen(2);
        }
    }
}

#[cfg(test)]
mod tests_elts_decors_mobiles {
    use super::*;
    use std::io::{Cursor, Seek};

    fn arbre() -> FormeId {
        [
            FormeId::Peupli,
            FormeId::Chene,
            FormeId::Noiset,
            FormeId::Genevr,
            FormeId::Hetre,
        ]
        .into_iter()
        .find(|id| id.forme().hauteur == 1)
        .expect("aucune forme d'arbre de hauteur 1")
    }

    fn sap3d_avec_arbres(id: FormeId) -> Sap3d {
        let mut s = Sap3d::default();
        for (x, y) in [(10, 20), (30, 40), (50, 60)] {
            s.aigoual.push(Mont {
                posx: x,
                posy: y,
                forme: Some(id),
                ..Mont::default()
            });
        }
        s
    }

    #[test]
    fn sauve_puis_charge_arbres_foudroyes() {
        let id = arbre();
        let mut source = sap3d_avec_arbres(id);
        source.aigoual[1].forme = Some(FormeId::Foudroye);

        let dir = std::env::temp_dir().join("sapiens_test_elts_decors");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("foudroye.bin");
        {
            let mut f = File::create(&path).unwrap();
            f.write_all(b"prefixe binaire \xff\x00").unwrap();
            source.SauveEltsDecorsMobiles(&mut f).unwrap();
        }
        let mut f = File::open(&path).unwrap();
        // On saute le préfixe comme le fait `charge` avant d'appeler la fonction.
        f.seek_relative(b"prefixe binaire \xff\x00".len() as i64)
            .unwrap();
        let mut cible = sap3d_avec_arbres(id);
        cible.ChargeEltsDecorsMobiles(&mut f).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(cible.aigoual[0].forme, Some(id));
        assert_eq!(cible.aigoual[1].forme, Some(FormeId::Foudroye));
        assert_eq!(cible.aigoual[2].forme, Some(id));
    }

    #[test]
    fn mauvais_entete() {
        let mut s = sap3d_avec_arbres(arbre());
        let mut c = Cursor::new(b"\nPAS_MAGIC\nFoudroye 10 20\n".to_vec());
        assert!(matches!(
            s.ChargeEltsDecorsMobiles(&mut c),
            Err(FileError::BadHeader)
        ));
        assert!(s.aigoual.iter().all(|m| m.forme != Some(FormeId::Foudroye)));
    }

    #[test]
    fn fichier_vide_ou_sans_entree() {
        let mut s = sap3d_avec_arbres(arbre());
        assert!(
            s.ChargeEltsDecorsMobiles(&mut Cursor::new(Vec::new()))
                .is_err()
        );
        let mut c = Cursor::new(b"\nMAGIC\n".to_vec());
        s.ChargeEltsDecorsMobiles(&mut c).unwrap();
        assert!(s.aigoual.iter().all(|m| m.forme != Some(FormeId::Foudroye)));
    }

    #[test]
    fn entree_tronquee_ignoree() {
        let mut s = sap3d_avec_arbres(arbre());
        let mut c = Cursor::new(b"\nMAGIC\nFoudroye 10 20\nFoudroye 30".to_vec());
        s.ChargeEltsDecorsMobiles(&mut c).unwrap();
        assert_eq!(s.aigoual[0].forme, Some(FormeId::Foudroye));
        assert_ne!(s.aigoual[1].forme, Some(FormeId::Foudroye));
    }
}
