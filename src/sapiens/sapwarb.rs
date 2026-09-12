#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
use crate::acam::{AcamGetPortOffscreen, AcamSetPortOffscreen, DisposPtr};
use crate::sapiens::sapglob::{heure, metpos};
use crate::sapiens::sapw3d::{cosshort, sinshort};
use crate::sapiens::sapwbrt::{Affmobbrt, CalculePaletteAttenuee, TraceDisque};
use crate::sapiens::sapwind::{Ecran1, rnd0};
use crate::shared::Ptr;
use std::os::raw::c_short;
use std::ptr::addr_of_mut;

static mut niveau: c_short = 0; /*niveau de recusrsivite*/
static mut largtronc: c_short = 0; /* Largeur du tronc*/
static mut numfeu: c_short = 0; /*type de la feuille*/
static mut amini: c_short = 0;
static mut qutlgn: c_short = 0;
static mut densif: c_short = 0;
static mut fortou: [c_short; 4] = [0; 4];
static mut TabIndexFeuilleNuit: *mut c_short = std::ptr::null_mut();

#[unsafe(no_mangle)]
/* 9 valeurs initiales + 1 slot: SAPWBRT lit tabcoul[9], SAPW3D l'écrit */
pub static mut tabcoul: [c_short; 10] = [3, 4, 5, 6, 6, 5, 4, 3, 3, 0];

const tabrnd: [c_short; 10] = [20, 30, 8, 1, 6, 12, 20, 6, 10, 4]; /* Rnd , base */
const tram_arbre: [c_short; 9] = [1, 3, 4, 5, 6, 5, 4, 3, 1];

/**************************************************************************
                        calpar
***************************************************************************/
unsafe fn calpar(largeur: c_short, ang: *mut c_short, lr: *mut c_short, lo: *mut c_short) {
    let mut n = largtronc as i32 - ((rnd0() as i32 * qutlgn as i32 * niveau as i32) >> 8);
    if n < 0 {
        n = (largtronc as i32 * rnd0() as i32) >> 8;
    }
    n += 1;
    *lo = n as c_short;
    let mut largeur = ((rnd0() as i32 * (largeur >> 1) as i32) >> 8) + (largeur >> 2) as i32 + 1;
    if rnd0() <= 40 {
        largeur += 1;
    }
    *ang = (((rnd0() as i32 * 70) >> 8) + amini as i32) as c_short;
    *lr = largeur as c_short;
}

/**************************************************************************
                            touffe
    Touffe: trace une touffe de feuilles sur tampon ecran   ----
;-> b_ancx,b_ancy: colonne,ligne
;   Densif,Fortou,Numfeu
; <- oh que c'est joli !
***************************************************************************/
unsafe fn touffe(colonne: c_short, ligne: c_short) {
    let mut angle: c_short = 0; /* D5*/
    let mut forme: c_short = 0;

    for _ in 0..densif {
        let r = rnd0();
        let mut y = (fortou[1] as i64 * sinshort(angle & 0xFF) as i64 * r as i64) >> 16;
        let mut x = (fortou[0] as i64 * cosshort(angle & 0xFF) as i64 * r as i64) >> 16;
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
            Affmobbrt(
                crate::MOB_FEUILLE!() + numfeu + forme,
                x as c_short,
                y as c_short,
                rnd0() & 1,
                TabIndexFeuilleNuit,
            );
        }
        forme += 1;
        if forme > 3 {
            forme = 0;
        }
        angle = ((angle as i32 + (rnd0() as i32 + 256)) & 1023) as c_short;
    }
}

/**************************************************************************
                            tracebranche
***************************************************************************/
unsafe fn tracebranche(
    angle: c_short,    /*D1*/
    colonne: c_short,  /*D2*/
    ligne: c_short,    /*D3*/
    longueur: c_short, /*D4*/
    largeur: c_short,  /*D5*/
) {
    niveau += 1;
    if niveau < 15 {
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
        TraceDisque(
            &mut AncXG,
            &mut AncXD,
            &mut Ancy,
            posg as c_short,
            posd as c_short,
            epaisseur as c_short,
        );
        if largeur < 2 {
            touffe((AncXG + AncXD) / 2, Ancy);
        } else if Ancy > 0 {
            /* Branche gauche */
            let mut ang: c_short = 0;
            let mut lr: c_short = 0;
            let mut lo: c_short = 0;
            let mut bidon: c_short = 0;
            calpar(largeur, &mut ang, &mut lr, &mut lo); /*Largeur longueur deltangle */
            ang = ang + angle;
            if ang > 450 {
                ang = 900 - ang;
            }
            tracebranche(ang, AncXG, Ancy, lo, lr);
            let col = AncXG + lr;
            lr = largeur - lr;
            if lr < 1 {
                lr = 1;
            }
            calpar(largeur, &mut ang, &mut bidon, &mut lo); /*Largeur longueur deltangle */
            ang = angle - ang;
            if ang < 10 {
                ang = 124 - ang;
            }
            tracebranche(ang, col, Ancy, lo, lr);
        }
    }
    niveau -= 1;
}

/**************************************************************************
                            arbre
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn arbre(ligne: c_short, colonne: c_short) {
    let mut gbak: *mut std::os::raw::c_void = std::ptr::null_mut();
    unsafe {
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(Ecran1);
        /* La position du meteore varie de 74 a 0 puis de 0 a 74 */
        if heure > 512 {
            TabIndexFeuilleNuit =
                CalculePaletteAttenuee(crate::MOB_FEUILLE!(), 0.50 + (metpos as f32) / 150.0, 999);
        } else {
            TabIndexFeuilleNuit = std::ptr::null_mut();
        }
        niveau = 0;
        let tabcoulp = addr_of_mut!(tabcoul) as *mut c_short;
        for i in 0..9 {
            *tabcoulp.add(i) = tram_arbre[i];
        }
        /* On tire au hasard le numero de la feuille */
        numfeu = (((rnd0() as i32 * 9) >> 8) * 4) as c_short;
        /* On tire les autres parametres au hasard (5 parametres)*/
        amini = (((rnd0() as i32 * tabrnd[0] as i32) >> 8) + tabrnd[1] as i32) as c_short;
        qutlgn = (((rnd0() as i32 * tabrnd[2] as i32) >> 8) + tabrnd[3] as i32) as c_short;
        densif = (((rnd0() as i32 * tabrnd[4] as i32) >> 8) + tabrnd[5] as i32) as c_short;
        fortou[0] = (((rnd0() as i32 * tabrnd[6] as i32) >> 8) + tabrnd[7] as i32) as c_short;
        fortou[1] = (((rnd0() as i32 * tabrnd[8] as i32) >> 8) + tabrnd[9] as i32) as c_short;
        let longueur = (((rnd0() as i32 * 6) >> 8) + 10) as c_short;
        let largeur = (((rnd0() as i32 * 10) >> 8) + 10) as c_short;
        largtronc = largeur;
        tracebranche(256, colonne, ligne, longueur, largeur);
        if !TabIndexFeuilleNuit.is_null() {
            DisposPtr(TabIndexFeuilleNuit as Ptr);
        }
        TabIndexFeuilleNuit = std::ptr::null_mut();
        AcamSetPortOffscreen(gbak);
    }
}
