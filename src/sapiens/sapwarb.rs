#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::Sapiens;
use crate::port::PortId;
use crate::sapiens::sapw3d::{cosshort, sinshort};

pub(crate) struct Sapwarb {
    niveau: i16,    /*niveau de recusrsivite*/
    largtronc: i16, /* Largeur du tronc*/
    numfeu: i16,    /*type de la feuille*/
    amini: i16,
    qutlgn: i16,
    densif: i16,
    fortou: [i16; 4],
    TabIndexFeuilleNuit: Option<Vec<i16>>,

    /* 9 valeurs initiales + 1 slot: SAPWBRT lit tabcoul[9], SAPW3D l'écrit */
    pub(crate) tabcoul: [i16; 10],
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
            TabIndexFeuilleNuit: None,

            /* 9 valeurs initiales + 1 slot: SAPWBRT lit tabcoul[9], SAPW3D l'écrit */
            tabcoul: [3, 4, 5, 6, 6, 5, 4, 3, 3, 0],
        }
    }
}

const tabrnd: [i16; 10] = [20, 30, 8, 1, 6, 12, 20, 6, 10, 4]; /* Rnd , base */
const tram_arbre: [i16; 9] = [1, 3, 4, 5, 6, 5, 4, 3, 1];

impl Sapiens {
    /**************************************************************************
                            calpar
    ***************************************************************************/
    fn calpar(&mut self, largeur: i16) -> (i16, i16, i16) {
        let mut n = self.sapwarb.largtronc as i32
            - ((self.random.rnd0() as i32
                * self.sapwarb.qutlgn as i32
                * self.sapwarb.niveau as i32)
                >> 8);
        if n < 0 {
            n = (self.sapwarb.largtronc as i32 * self.random.rnd0() as i32) >> 8;
        }
        n += 1;
        let lo = n as i16;
        let mut largeur =
            ((self.random.rnd0() as i32 * (largeur >> 1) as i32) >> 8) + (largeur >> 2) as i32 + 1;
        if self.random.rnd0() <= 40 {
            largeur += 1;
        }
        let ang = (((self.random.rnd0() as i32 * 70) >> 8) + self.sapwarb.amini as i32) as i16;
        let lr = largeur as i16;
        (ang, lr, lo)
    }

    /**************************************************************************
                                touffe
        Touffe: trace une touffe de feuilles sur tampon ecran   ----
    ;-> b_ancx,b_ancy: colonne,ligne
    ;   Densif,Fortou,Numfeu
    ; <- oh que c'est joli !
    ***************************************************************************/
    fn touffe(&mut self, colonne: i16, ligne: i16) {
        let mut angle: i16 = 0; /* D5*/
        let mut forme: i16 = 0;

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
                self.sapwbrt.Affmobbrt(
                    self.sapwind.ports.current_mut(),
                    crate::MOB_FEUILLE!(self.sapwind) + self.sapwarb.numfeu + forme,
                    x as i16,
                    y as i16,
                    sens,
                    self.sapwarb.TabIndexFeuilleNuit.as_deref(),
                );
            }
            forme += 1;
            if forme > 3 {
                forme = 0;
            }
            angle = ((angle as i32 + (self.random.rnd0() as i32 + 256)) & 1023) as i16;
        }
    }

    /**************************************************************************
                                tracebranche
    ***************************************************************************/
    fn tracebranche(
        &mut self,
        angle: i16,    /*D1*/
        colonne: i16,  /*D2*/
        ligne: i16,    /*D3*/
        longueur: i16, /*D4*/
        largeur: i16,  /*D5*/
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
                posg as i16,
                posd as i16,
                epaisseur as i16,
            );
            if largeur < 2 {
                self.touffe((AncXG + AncXD) / 2, Ancy);
            } else if Ancy > 0 {
                /* Branche gauche */
                let (mut ang, mut lr, mut lo) = self.calpar(largeur); /*Largeur longueur deltangle */
                ang += angle;
                if ang > 450 {
                    ang = 900 - ang;
                }
                self.tracebranche(ang, AncXG, Ancy, lo, lr);
                let col = AncXG + lr;
                lr = largeur - lr;
                if lr < 1 {
                    lr = 1;
                }
                (ang, _, lo) = self.calpar(largeur); /*Largeur longueur deltangle */
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
    pub(crate) fn arbre(&mut self, ligne: i16, colonne: i16) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);
        /* La position du meteore varie de 74 a 0 puis de 0 a 74 */
        if self.sapglob.heure > 512 {
            self.sapwarb.TabIndexFeuilleNuit = Some(self.CalculePaletteAttenuee(
                crate::MOB_FEUILLE!(self.sapwind),
                0.50 + (self.sapglob.metpos as f32) / 150.0,
                999,
            ));
        } else {
            self.sapwarb.TabIndexFeuilleNuit = None;
        }
        self.sapwarb.niveau = 0;
        self.sapwarb.tabcoul[..tram_arbre.len()].copy_from_slice(&tram_arbre);
        /* On tire au hasard le numero de la feuille */
        self.sapwarb.numfeu = (((self.random.rnd0() as i32 * 9) >> 8) * 4) as i16;
        /* On tire les autres parametres au hasard (5 parametres)*/
        self.sapwarb.amini =
            (((self.random.rnd0() as i32 * tabrnd[0] as i32) >> 8) + tabrnd[1] as i32) as i16;
        self.sapwarb.qutlgn =
            (((self.random.rnd0() as i32 * tabrnd[2] as i32) >> 8) + tabrnd[3] as i32) as i16;
        self.sapwarb.densif =
            (((self.random.rnd0() as i32 * tabrnd[4] as i32) >> 8) + tabrnd[5] as i32) as i16;
        self.sapwarb.fortou[0] =
            (((self.random.rnd0() as i32 * tabrnd[6] as i32) >> 8) + tabrnd[7] as i32) as i16;
        self.sapwarb.fortou[1] =
            (((self.random.rnd0() as i32 * tabrnd[8] as i32) >> 8) + tabrnd[9] as i32) as i16;
        let longueur = (((self.random.rnd0() as i32 * 6) >> 8) + 10) as i16;
        let largeur = (((self.random.rnd0() as i32 * 10) >> 8) + 10) as i16;
        self.sapwarb.largtronc = largeur;
        self.tracebranche(256, colonne, ligne, longueur, largeur);
        self.sapwarb.TabIndexFeuilleNuit = None;
    }
}
