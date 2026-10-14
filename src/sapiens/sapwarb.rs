#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
use crate::acam::DisposPtr;
use crate::sapiens::sapw3d::{cosshort, sinshort};
use crate::shared::Ptr;
use crate::{Sapiens, port};
use std::os::raw::c_short;

pub(crate) struct Sapwarb {
    niveau: c_short,    /*niveau de recusrsivite*/
    largtronc: c_short, /* Largeur du tronc*/
    numfeu: c_short,    /*type de la feuille*/
    amini: c_short,
    qutlgn: c_short,
    densif: c_short,
    fortou: [c_short; 4],
    TabIndexFeuilleNuit: *mut c_short,

    /* 9 valeurs initiales + 1 slot: SAPWBRT lit tabcoul[9], SAPW3D l'écrit */
    pub(crate) tabcoul: [c_short; 10],
}

impl Default for Sapwarb {
    fn default() -> Self {
        Self {
            niveau: 0,    /*niveau de recusrsivite*/
            largtronc: 0, /* Largeur du tronc*/
            numfeu: 0,    /*type de la feuille*/
            amini: 0,
            qutlgn: 0,
            densif: 0,
            fortou: [0; 4],
            TabIndexFeuilleNuit: std::ptr::null_mut(),

            /* 9 valeurs initiales + 1 slot: SAPWBRT lit tabcoul[9], SAPW3D l'écrit */
            tabcoul: [3, 4, 5, 6, 6, 5, 4, 3, 3, 0],
        }
    }
}

const tabrnd: [c_short; 10] = [20, 30, 8, 1, 6, 12, 20, 6, 10, 4]; /* Rnd , base */
const tram_arbre: [c_short; 9] = [1, 3, 4, 5, 6, 5, 4, 3, 1];

impl Sapiens {
    /**************************************************************************
                            calpar
    ***************************************************************************/
    fn calpar(&mut self, largeur: c_short, ang: *mut c_short, lr: *mut c_short, lo: *mut c_short) {
        let mut n = self.sapwarb.largtronc as i32
            - ((self.random.rnd0() as i32
                * self.sapwarb.qutlgn as i32
                * self.sapwarb.niveau as i32)
                >> 8);
        if n < 0 {
            n = (self.sapwarb.largtronc as i32 * self.random.rnd0() as i32) >> 8;
        }
        n += 1;
        unsafe {
            *lo = n as c_short;
        }
        let mut largeur =
            ((self.random.rnd0() as i32 * (largeur >> 1) as i32) >> 8) + (largeur >> 2) as i32 + 1;
        if self.random.rnd0() <= 40 {
            largeur += 1;
        }
        unsafe {
            *ang = (((self.random.rnd0() as i32 * 70) >> 8) + self.sapwarb.amini as i32) as c_short;
            *lr = largeur as c_short;
        }
    }

    /**************************************************************************
                                touffe
        Touffe: trace une touffe de feuilles sur tampon ecran   ----
    ;-> b_ancx,b_ancy: colonne,ligne
    ;   Densif,Fortou,Numfeu
    ; <- oh que c'est joli !
    ***************************************************************************/
    fn touffe(&mut self, colonne: c_short, ligne: c_short) {
        let mut angle: c_short = 0; /* D5*/
        let mut forme: c_short = 0;

        for _ in 0..self.sapwarb.densif {
            let r = self.random.rnd0();
            let mut y =
                (self.sapwarb.fortou[1] as i64 * sinshort(angle & 0xFF) as i64 * r as i64) >> 16;
            let mut x =
                (self.sapwarb.fortou[0] as i64 * cosshort(angle & 0xFF) as i64 * r as i64) >> 16;
            if angle & 0x100 != 0 {
                x += colonne as i64;
            } else {
                x = colonne as i64 - x;
            }
            if angle & 0x200 != 0 {
                y += ligne as i64;
            } else {
                y = ligne as i64 - y;
            }
            x += 16;
            if x < 256 {
                let sens = self.random.rnd0() & 1;
                self.Affmobbrt(
                    crate::MOB_FEUILLE!(self.sapwind) + self.sapwarb.numfeu + forme,
                    x as c_short,
                    y as c_short,
                    sens,
                    self.sapwarb.TabIndexFeuilleNuit,
                );
            }
            forme += 1;
            if forme > 3 {
                forme = 0;
            }
            angle = ((angle as i32 + (self.random.rnd0() as i32 + 256)) & 1023) as c_short;
        }
    }

    /**************************************************************************
                                tracebranche
    ***************************************************************************/
    fn tracebranche(
        &mut self,
        angle: c_short,    /*D1*/
        colonne: c_short,  /*D2*/
        ligne: c_short,    /*D3*/
        longueur: c_short, /*D4*/
        largeur: c_short,  /*D5*/
    ) {
        self.sapwarb.niveau += 1;
        if self.sapwarb.niveau < 15 {
            /*;---- Mise en place des parametres pour TraceDisque*/
            let mut AncXG = colonne;
            let mut AncXD = colonne + largeur;
            let mut Ancy = ligne;
            /*---- Calcul de l'epaisseur */
            let mut epaisseur = (sinshort(angle).abs() as i32 * longueur as i32) >> 8;
            if epaisseur < 1 {
                epaisseur = 1;
            }
            /* Calcul de la pos haut gauche */
            let mut posg = AncXG as i32 + ((cosshort(angle) as i32 * longueur as i32) >> 8);
            if posg < 0 {
                posg = 0;
            }
            let mut posd = posg + largeur as i32;
            if posd > 255 {
                posg = (255 - largeur) as i32;
                posd = 255;
            }
            /* Trb15*/
            if epaisseur + (Ancy as i32) < 0 {
                epaisseur = Ancy as i32;
            }
            self.TraceDisque(
                &mut AncXG,
                &mut AncXD,
                &mut Ancy,
                posg as c_short,
                posd as c_short,
                epaisseur as c_short,
            );
            if largeur < 2 {
                self.touffe((AncXG + AncXD) / 2, Ancy);
            } else if Ancy > 0 {
                /* Branche gauche */
                let mut ang: c_short = 0;
                let mut lr: c_short = 0;
                let mut lo: c_short = 0;
                let mut bidon: c_short = 0;
                self.calpar(largeur, &mut ang, &mut lr, &mut lo); /*Largeur longueur deltangle */
                ang = ang + angle;
                if ang > 450 {
                    ang = 900 - ang;
                }
                self.tracebranche(ang, AncXG, Ancy, lo, lr);
                let col = AncXG + lr;
                lr = largeur - lr;
                if lr < 1 {
                    lr = 1;
                }
                self.calpar(largeur, &mut ang, &mut bidon, &mut lo); /*Largeur longueur deltangle */
                ang = angle - ang;
                if ang < 10 {
                    ang = 124 - ang;
                }
                self.tracebranche(ang, col, Ancy, lo, lr);
            }
        }
        self.sapwarb.niveau -= 1;
    }

    /**************************************************************************
                                arbre
    ***************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn arbre(&mut self, ligne: c_short, colonne: c_short) {
        unsafe {
            let _g = port::on(self.sapwind.Ecran1);
            /* La position du meteore varie de 74 a 0 puis de 0 a 74 */
            if self.sapglob.heure > 512 {
                self.sapwarb.TabIndexFeuilleNuit = self.CalculePaletteAttenuee(
                    crate::MOB_FEUILLE!(self.sapwind),
                    0.50 + (self.sapglob.metpos as f32) / 150.0,
                    999,
                );
            } else {
                self.sapwarb.TabIndexFeuilleNuit = std::ptr::null_mut();
            }
            self.sapwarb.niveau = 0;
            self.sapwarb.tabcoul[..tram_arbre.len()].copy_from_slice(&tram_arbre);
            /* On tire au hasard le numero de la feuille */
            self.sapwarb.numfeu = (((self.random.rnd0() as i32 * 9) >> 8) * 4) as c_short;
            /* On tire les autres parametres au hasard (5 parametres)*/
            self.sapwarb.amini = (((self.random.rnd0() as i32 * tabrnd[0] as i32) >> 8)
                + tabrnd[1] as i32) as c_short;
            self.sapwarb.qutlgn = (((self.random.rnd0() as i32 * tabrnd[2] as i32) >> 8)
                + tabrnd[3] as i32) as c_short;
            self.sapwarb.densif = (((self.random.rnd0() as i32 * tabrnd[4] as i32) >> 8)
                + tabrnd[5] as i32) as c_short;
            self.sapwarb.fortou[0] = (((self.random.rnd0() as i32 * tabrnd[6] as i32) >> 8)
                + tabrnd[7] as i32) as c_short;
            self.sapwarb.fortou[1] = (((self.random.rnd0() as i32 * tabrnd[8] as i32) >> 8)
                + tabrnd[9] as i32) as c_short;
            let longueur = (((self.random.rnd0() as i32 * 6) >> 8) + 10) as c_short;
            let largeur = (((self.random.rnd0() as i32 * 10) >> 8) + 10) as c_short;
            self.sapwarb.largtronc = largeur;
            self.tracebranche(256, colonne, ligne, longueur, largeur);
            if !self.sapwarb.TabIndexFeuilleNuit.is_null() {
                DisposPtr(self.sapwarb.TabIndexFeuilleNuit as Ptr);
            }
            self.sapwarb.TabIndexFeuilleNuit = std::ptr::null_mut();
        }
    }
}
