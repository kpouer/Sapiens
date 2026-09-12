/*************************V1.6 Mardi 9 Dec**********************************
                         Module #1 :Projet SAP
              Gestion de Listes:Le mardi 4 Novembre 1986
**************************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, CarboniseSetPortWindow, FAUX, InsetRect, PtInRect,
    SetRect, TickCount, VRAI,
};
use crate::cbridge::strcat;
use crate::langue::Langue::{ES, FR};
use crate::sapiens::sapcomb::{ClickInGenerique, combat, prm};
use crate::sapiens::sapgest::{DebutAventure, charge, defsap, primjauge, primsap, sauve, trfab};
use crate::sapiens::sapglob::{
    bless, bouton, ccol, ciel_degr, clin, cncol, cnlin, colaff, colcdr, criter, cross, dervue,
    dsposx, dsposy, dzeta, faim, flcomb, flefba, gemzoom, icone, lastmen, linaff, listbidon,
    liste0, liste1, liste2, liste4, lpos, menu, mort, pal_chron, palbasse, palhaute, poscur,
    presence, present, rega, s_action, s_menu, sap1, sap2, sap3, sap4, scenpr, smenu, soif, sposx,
    sposy, stdlist, stock1, stock2, tccol, tclin, tcncol, tcnlin, tcolcdr, tlpos, tposcur,
    tstdlist, xj, yj, zoneu,
};
use crate::sapiens::saprel::{
    afstat, bulle, clrbadi, comment, efbadi, effaceBulles, trdire, trdonn, trdors, trobsv,
};
use crate::sapiens::saptxt::{GetRefer, GetRefer1, langue};
use crate::sapiens::sapwbrt::{
    AffichePictDansAire, BoxFill, SetPathInData, SwapCouleur, TfrEcr, affomb, tfrima,
};
use crate::sapiens::sapwcar::{Gotoxy, affcar};
use crate::sapiens::sapwclav::{
    MontreSouris, aff_sour, bouge_souris, eff_sour, reset_mouse, vq_mouse,
};
use crate::sapiens::sapwcvt::TabCvtCoul;
use crate::sapiens::sapwind::{
    Collapse, ConfirmePerteAventure, FixeMenusComplet, FixeMenusReduit, MenuFenetres, SapZoom,
    cadreg, debecr, hires, init_graph, mainWindow,
};
use crate::sapiens::sapwmess::{TfrBadAss, efbadass, framebadass};
use crate::sapiens::sapwmus::bruit;
use crate::sapiens::sapwpref::DoPreferences;
use crate::sapiens::{ECHELLE, VUE_LATERALE, geminterv, gemnbrecad, liste};
use crate::shared::{CGrafPtr, Point, Ptr, Rect};
use crate::stubs::{GetMouse, GetNomCheminCourant, OpenDocument, PopPath, PushPath};
use crate::{DOWN, INCONNU, MOB_CADRE, SELECT, UP};
use log::warn;
use std::ffi::c_char;
use std::os::raw::{c_float, c_short, c_uchar, c_ushort};
use std::ptr::{addr_of_mut, null_mut};
/***********************************************************************
                InverseOption
***********************************************************************/
static mut LastMenuInvers: c_short = -1;
static mut Lastn: c_short = 0;
static mut rInvert: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
};

#[unsafe(no_mangle)]
pub extern "C" fn InverseOption(menusel: c_short, n: c_short) {
    unsafe {
        eff_sour();
        if LastMenuInvers >= 0 {
            SwapCouleur(
                debecr,
                &raw mut rInvert,
                TabCvtCoul[(*palbasse.add(4)) as usize] as c_short,
                TabCvtCoul[(*palbasse.add(1)) as usize] as c_short,
            );
            TfrEcr(&raw mut rInvert);
            LastMenuInvers = -1;
        }
        if menusel >= 0 {
            SetRect(
                &raw mut rInvert,
                (tccol[menusel as usize] as c_short) * 6 * ECHELLE as c_short,
                (tclin[menusel as usize] as c_short - tlpos[menusel as usize] as c_short + 1 + n)
                    * 7
                    * ECHELLE as c_short
                    - 1,
                (tccol[menusel as usize] as c_short + tcncol[menusel as usize] as c_short)
                    * 6
                    * ECHELLE as c_short,
                (tclin[menusel as usize] as c_short - tlpos[menusel as usize] as c_short
                    + 1
                    + n
                    + 1)
                    * 7
                    * ECHELLE as c_short
                    - 1,
            );
            SwapCouleur(
                debecr,
                &raw mut rInvert,
                TabCvtCoul[(*palbasse.add(1)) as usize] as c_short,
                TabCvtCoul[(*palbasse.add(4)) as usize] as c_short,
            );
            TfrEcr(&raw mut rInvert);
        }
        LastMenuInvers = menusel;
        Lastn = n;
    }
}

const EAU: c_uchar = 2;
const OUTREVIDE: c_uchar = 26;
const OUTREPLEINE: c_uchar = 27;

#[unsafe(no_mangle)]
pub static mut sliste: [*mut liste; 2] =
    [&raw mut liste0 as *mut liste, &raw mut liste1 as *mut liste];

#[unsafe(no_mangle)]
#[allow(dead_code)]
pub static mut pal1: [c_short; 32] = [
    0x006, 0x542, 0x650, 0x777, 0x430, 0x310, 0x320, 0x210, 0x005, 0x432, 0x443, 0x444, 0x322,
    0x332, 0x211, 0x000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[unsafe(no_mangle)]
#[allow(dead_code)]
pub static mut pal2: [c_short; 32] = [
    0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x531, 0x631, 0x641, 0x444, 0x050, 0x070,
    0x770, 0x450, 0x005, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[unsafe(no_mangle)]
pub static mut pal_zero: [c_short; 32] = [0; 32];

/***********************************************************************
                                TRGEST:gestion
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trgest(regb: ::std::os::raw::c_ushort) {
    MontreSouris();
    unsafe {
        match regb {
            /* Informations */
            0 => {
                //DoInfo();
                DoAide();
                //sceveu(8);
            }
            /* Charge aventure */
            1 => {
                if ConfirmePerteAventure() {
                    charge();
                    inimen(8);
                }
            }
            /* Sauve aventure */
            2 => sauve(),
            /* Personnage : nouvlle aventure */
            3 => {
                FixeMenusReduit();
                if ConfirmePerteAventure() {
                    defsap();
                }
                FixeMenusComplet();
            }
            4 => DoPreferences(),
            // #ifdef OLD_VERSION
            //             /* Aide au clavier - AIde autonome */
            //         case 4:
            //             liste8.elements[4] ^= 1;
            //             inimen(8);
            //             break;
            //             /* Musique Midi - Musique DMA */
            //         case 5:
            //             liste8.elements[5] ^= 1;
            //             okmuse = liste8.elements[5] & 1;
            //             MusicInit(1);
            //             reset_synthe();
            //             inimen(8);
            //             break;
            //             /* Reglage Midi */
            //         case 6:
            //             /***********************************************************************
            //                                     AJUSTE:Ajuste MIDI
            //             ***********************************************************************/
            //             void ajuste() {
            //                 short car = 0;
            //
            //                 sceveu(10);
            //                 reset_clav();
            //                 while (!car) {
            //                     car = litclav() & 223;
            //                 }
            //                 if (car >= 'A' && car <= 'C') {
            //                     quel_synthe(car - 'A');
            //                 }
            //                 return;
            //             }
            //             effima(0x443); /*10*/
            //             tfrima();
            //             dervue = VUE_AUCUNE;
            //             ajuste();
            //             break;
            //
            //         case 7:
            //             inimen(8);
            //             break;
            // #endif
            _ => {}
        }
    }
}

/************************************************************************
                        TRPOSE:Pose objet
**********************************************************************
=>(rega)et (menu)
*/
fn trpose() {
    unsafe {
        let erreur = enlevl(&raw mut liste1, &raw mut liste4, rega as c_ushort); /*on transfere*/
        if erreur == 1 && lpos > 1 {
            lpos -= 1;
        }
        /*elt de rang rega est en fin de liste*/
        inimen(s_menu as c_ushort);
    }
}
/************************************************************************
                INITPRIM:inits primaires
***********************************************************************/
fn initprim() {
    unsafe {
        dsposx = 0;
        sposx = 60;
        dsposy = 0;
        sposy = 160;
        /* toute Palettes a zéro*/
        palhaute = addr_of_mut!(pal_zero) as *mut std::os::raw::c_short;
        pal_chron = addr_of_mut!(pal_zero) as *mut std::os::raw::c_short;
        palbasse = addr_of_mut!(pal_zero) as *mut std::os::raw::c_short;
        ciel_degr = addr_of_mut!(pal_zero) as *mut std::os::raw::c_short;
        let _first_init = init_graph();
        hires();
        xj = 0x5800;
        yj = 0x5400;
        dzeta = 768;
        flefba = 0;
        /*test version*/
        flefba = 0;
        primsap();
        sap1.colonne = 200;
        present[0] = -1;
        prm(addr_of_mut!(sap1), -1);
        prm(addr_of_mut!(sap2), 0);
        prm(addr_of_mut!(sap3), 0);
        prm(addr_of_mut!(sap4), 0);
        framebadass();
        palhaute = addr_of_mut!(pal2) as *mut std::os::raw::c_short;
        pal_chron = addr_of_mut!(pal2) as *mut std::os::raw::c_short;
        palbasse = addr_of_mut!(pal1) as *mut std::os::raw::c_short;
        /* Mise en place version */
        DebutAventure(true);
        afstat();
        //# define gemaction  19,1,1,14
        gemouvre(1, 1, 23, 62, 19, 1, 1, 14);
        encadre(
            tclin[0] as u16,
            tccol[0] as u16,
            tcnlin[0] as u16,
            tcncol[0] as u16,
        ); /*cadre menu 0*/
        inimen(0);
        //if(firstInit) DoAide();
        return;
    }
}

/************************************************************************/
/*              GEMOUVRE:Transfere le cadre avec effet sp�cial          */
/************************************************************************/
/*  ouvre un cadre de cad1 vers cad2 (64 colonnes * 25 Lignes)*/
/* le nombre de cadre 'GEMNBRECAD' doit etre pair*/
#[unsafe(no_mangle)]
pub extern "C" fn gemouvre(
    ligncad1: ::std::os::raw::c_ushort,
    colcad1: ::std::os::raw::c_ushort,
    hautcad1: ::std::os::raw::c_ushort,
    largcad1: ::std::os::raw::c_ushort,
    ligncad2: ::std::os::raw::c_ushort,
    colcad2: ::std::os::raw::c_ushort,
    hautcad2: ::std::os::raw::c_ushort,
    largcad2: ::std::os::raw::c_ushort,
) {
    /*calcul des parametres de depart*/
    let mut ligne: c_short = (ligncad1 * 7 / 2) as c_short;
    let mut colon: c_short = (colcad1 * 3 / 2) as c_short;
    let mut haut: c_short = (hautcad1 * 7 / 2) as c_short;
    let mut larg: c_short = (largcad1 * 3 / 2) as c_short;
    /* on l'injecte ds les N cadres*/

    unsafe {
        for i in 0..gemnbrecad as usize {
            gemzoom[i].ligne = ligne;
            gemzoom[i].colon = colon;
            gemzoom[i].haut = haut;
            gemzoom[i].larg = larg;
        }

        /* on calcule les coordonn�es d'arriv�e*/
        ligne = (ligncad2 * 7 / 2) as c_short;
        colon = (colcad2 * 3 / 2) as c_short;
        haut = (hautcad2 * 7 / 2) as c_short;
        larg = (largcad2 * 3 / 2) as c_short;
        /* on y va*/
        let mut balai = 0;
        loop {
            for i in 0..=balai / geminterv as usize {
                /* on efface le cadre*/
                cadreg(
                    gemzoom[i].ligne,
                    gemzoom[i].colon,
                    gemzoom[i].haut,
                    gemzoom[i].larg,
                );
                /*on calcule le nlles coordonnées*/
                if gemzoom[i].ligne < ligne {
                    gemzoom[i].ligne += 1;
                } else if gemzoom[i].ligne > ligne {
                    gemzoom[i].ligne -= 1;
                }

                if gemzoom[i].colon < colon {
                    gemzoom[i].colon += 1;
                } else if gemzoom[i].colon > colon {
                    gemzoom[i].colon -= 1;
                }

                if gemzoom[i].haut < haut {
                    gemzoom[i].haut += 1;
                } else if gemzoom[i].haut > haut {
                    gemzoom[i].haut -= 1;
                }

                if gemzoom[i].larg < larg {
                    gemzoom[i].larg += 1;
                } else if gemzoom[i].larg > larg {
                    gemzoom[i].larg -= 1;
                }
                /*affiche le cadre*/
                cadreg(
                    gemzoom[i].ligne,
                    gemzoom[i].colon,
                    gemzoom[i].haut,
                    gemzoom[i].larg,
                );
            }
            /*si tout les cadres en cours alors balai ne bouge plus*/
            if balai / geminterv as usize + 1 < gemnbrecad as usize {
                balai += 1;
            }
            /*fini ????: dernier cadre arrivé*/
            if gemzoom[(gemnbrecad - 1) as usize].ligne == ligne
                && gemzoom[(gemnbrecad - 1) as usize].colon == colon
                && gemzoom[(gemnbrecad - 1) as usize].haut == haut
                && gemzoom[(gemnbrecad - 1) as usize].larg == larg
            {
                return;
            }
        }
    }
}

/***********************************************************************
              ListAttendsFinBouton
**************************************************************************/
fn ListAttendsFinBouton() {
    loop {
        unsafe {
            vq_mouse(
                &raw mut bouton as *mut _,
                &raw mut sposx as *mut _,
                &raw mut sposy,
            );
            if bouton <= 0 {
                break;
            }
        }
    }
}

/***********************************************************************
                CLAVM:Gestion des menus au clavier
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn clavm() {
    let mut y: c_short = 0;
    let mut pt = Point::default();
    let mut r = Rect::default();

    /* gestion souris pour clavm */
    unsafe {
        aff_sour(icone, sposy, sposx);
        loop {
            /* appel de grise mine */
            gere_souris();
            /* teste si bouton appuye*/
            if bouton > 0 {
                if icone != INCONNU!() && icone != 0 {
                    /* calcul des valeurs par defaut pour la liste selectionne*/
                    /* deplacement dans la liste */
                    if (icone == UP!()) && (tlpos[s_menu as usize] > 1) {
                        eff_sour();
                        tlpos[s_menu as usize] -= 1;
                        inimen(s_menu as c_ushort);
                        aff_sour(icone, sposy, sposx);
                    }
                    if (icone == DOWN!())
                        && (tlpos[s_menu as usize]
                            <= ((*stdlist).taille as u16 - tcnlin[s_menu as usize] as u16) as i8)
                    {
                        eff_sour();
                        tlpos[s_menu as usize] += 1;
                        inimen(s_menu as c_ushort);
                        aff_sour(icone, sposy, sposx);
                    }
                    /* si selection et liste non vide */
                    if (icone == SELECT!()) && ((*stdlist).taille > 0) {
                        eff_sour();
                        /* calcul de l'option selectionn�e */
                        /* calcul position 53 du centre de la souris */
                        y = sposy / 7;
                        /* calcul de l'option choisie*/
                        rega = (y - tclin[s_menu as usize] as c_short
                            + tlpos[s_menu as usize] as c_short
                            - 1) as c_uchar;
                        if rega < (*stdlist).taille {
                            let refer = GetRefer((*stdlist).typeRef);
                            /*mise en place de stock1 et stock2*/
                            let ref_ = refer
                                .wrapping_add(((*stdlist).elements[rega as usize] - 1) as usize);
                            stock1 = (*ref_).param1;
                            stock2 = (*ref_).param2;
                            /*on affiche l'option choisie*/
                            efbadi();
                            if s_menu <= 0 {
                                s_action = (rega + 1) as c_short;
                            }
                            comment((120 + s_action) as c_ushort);
                            InverseOption(s_menu, rega as c_short);
                            /* si sous menu affichage et sortie*/
                            if s_menu > 0 {
                                let refer = GetRefer((*sliste[1]).typeRef);

                                affcar(' ' as u8 as c_short);
                                let ref_ = refer.wrapping_add(
                                    ((*sliste[1]).elements[rega as usize] - 1) as usize,
                                );
                                mot(ref_ as *mut _);
                                TfrBadAss();
                                if criter > 0 {
                                    rega = cross.elements[rega as usize];
                                }
                                return;
                            } else
                            /* si menu principal */
                            /* on affiche (ou reaffiche) le sous-menu*/
                            {
                                criter = stock2;
                                if criter > 0 {
                                    tlpos[1] = 1;
                                }
                                if stock1 == 7 {
                                    if dervue != VUE_LATERALE as u8 {
                                        primjauge(0);
                                    }
                                    afstat();
                                } else {
                                    inimen(stock1 as c_ushort);
                                }
                            }
                            sposx += 1; /* pour affichage */
                            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
                        }
                        aff_sour(icone, sposy, sposx);
                    }
                } else
                /* La souris est dans la partie haute de l'ecran */
                if icone == 0 {
                    /* Bouton et pas d'icone */
                    GetMouse(&mut pt);
                    pt.h = (pt.h as c_float / SapZoom) as i16;
                    pt.v = (pt.v as c_float / SapZoom) as i16;

                    SetRect(
                        &mut r,
                        (32 * ECHELLE) as i16,
                        (1 * ECHELLE) as i16,
                        ((32 + 255) * ECHELLE) as i16,
                        ((1 + 100) * ECHELLE) as i16,
                    );
                    /* Il n'y a personne et le click est dans l'aire */
                    if presence <= 0 && PtInRect(pt, &r) {
                        /* Click dans l'aire de l'image */
                        /* On ouvre par defaut le menu'voir'*/
                        InverseOption(0, 6);
                        s_action = 7;
                        inimen(3);
                        /* Demande le traitement */
                        rega = 0;
                        return;
                    }
                    ClickInGenerique(pt);
                }
            }
        }
    }
}

/***********************************************************************
              AJOUTL:ajoute un element a une liste
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn ajoutl(debliste: *mut liste, element: c_ushort) -> c_ushort {
    unsafe {
        if debliste == &raw mut listbidon {
            /*le premier element le recoit*/
            (*debliste).elements[0] = element as c_uchar;
        } else {
            /*liste est elle pleine*/
            if (*debliste).taille == (*debliste).maxtaille {
                return 1; /*deja pleine*/
            }
            (*debliste).elements[(*debliste).taille as usize] = element as c_uchar;
            (*debliste).taille += 1;
        }
        0 /*ok*/
    }
}

/***********************************************************************
                ENLEVL:transfere un element d'une liste sur une autre
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn enlevl(
    orgliste: *mut liste,
    receptliste: *mut liste,
    rang: c_ushort,
) -> c_ushort {
    unsafe {
        if rang < (*orgliste).taille as c_ushort {
            /*rang contenu dans liste de depart*/
            (*orgliste).taille = (*orgliste).taille.wrapping_sub(1);
            let erreur = ajoutl(receptliste, (*orgliste).elements[rang as usize] as c_ushort);
            if erreur != 0 {
                /*pas de place en reception*/
                (*orgliste).taille = (*orgliste).taille.wrapping_add(1);
                return 1; /*erreur*/
            } else {
                /*on recompacte la liste d'origine*/
                let mut i: c_uchar = rang as c_uchar;
                while i != (*orgliste).taille {
                    (*orgliste).elements[i as usize] = (*orgliste).elements[i as usize + 1];
                    i = i.wrapping_add(1);
                }
                if i as c_ushort == rang {
                    return 1; /*fin de liste*/
                } else {
                    return 0;
                }
            }
        } else {
            return 255; /*erreur de logique rang pas ds la liste de depart*/
        }
    }
}

/***********************************************************************
              MOT:Affiche un mot selon la reference std
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn mot(refer: *mut c_char) {
    unsafe {
        let mut p = refer as *mut c_uchar;
        while *p != 0 {
            affcar(*p as c_short);
            p = p.add(1);
        }
    }
}

/***********************************************************************
              AFFMOT:affiche le mot ou des blancs
***********************************************************************/
fn affmot(debliste: *mut liste, rang: c_ushort) {
    unsafe {
        if (rang as c_uchar) < (*debliste).taille {
            /*rang a afficher existe*/
            let refer = GetRefer((*debliste).typeRef);
            mot(
                refer.add(((*debliste).elements[rang as usize].wrapping_sub(1)) as usize)
                    as *mut c_char,
            );
        }
    }
}

/***********************************************************************
              AFLIST:Affiche liste en entr�e
***********************************************************************/
fn aflist(debliste: *mut liste) {
    unsafe {
        let mut gbak: CGrafPtr = null_mut();
        let _j: c_short = 0;
        let mut r = Rect::default();

        AcamGetPortOffscreen(&mut gbak as *mut _);
        AcamSetPortOffscreen(debecr);
        if s_menu == LastMenuInvers {
            LastMenuInvers = -1;
        }
        /*ligne et colonne au debut du cadre*/
        Gotoxy(ccol as c_short, clin as c_short);
        let mut i = cnlin as c_uchar; /*i recoit le nombre de ligne du cadre*/
        if lpos == 0 {
            lpos = 1;
        }
        if (lpos as c_uchar) > (*debliste).taille {
            lpos = (*debliste).taille;
        }
        let mut travl2 = lpos.wrapping_sub(1); /*debut d'affichage*/
        /* On efface le fond de la liste */
        SetRect(
            &mut r,
            (colaff as c_short) * 6 * ECHELLE as c_short,
            (linaff as c_short) * 7 * ECHELLE as c_short - 1,
            (colaff as c_short + cncol as c_short) * 6 * ECHELLE as c_short,
            (linaff as c_short + cnlin as c_short) * 7 * ECHELLE as c_short,
        );
        BoxFill(debecr, &mut r, 0x542 as c_short);
        /* On affiche tous les mots */
        if (*debliste).taille != 0 {
            /* On n'est pas en haut de la liste, fleche vers le haut */
            if travl2 > 1 {
                Gotoxy((ccol as c_short + cncol as c_short) - 1, linaff as c_short);
                affcar('<' as c_short);
                Gotoxy(ccol as c_short, clin as c_short);
            }
            while i != 0 {
                i = i.wrapping_sub(1);
                affmot(debliste, travl2 as c_ushort); /*on affiche le mot*/
                travl2 = travl2.wrapping_add(1);
                Gotoxy(ccol as c_short, (linaff + 1) as c_short);
            }
            /* On n'est pas en bas de la liste, fleche vers le bas */
            if travl2 < (*debliste).taille {
                Gotoxy(
                    (ccol as c_short + cncol as c_short) - 1,
                    (linaff - 1) as c_short,
                );
                affcar('>' as c_short);
            }
        } else {
            /* Liste vide */
            mot(c"...".as_ptr() as *mut c_char);
        }
        /* Transfert de la liste sur l'ecran */
        InsetRect(&mut r, -48, -48);
        TfrEcr(&mut r);
        AcamSetPortOffscreen(gbak);
    }
}

/***********************************************************************
                        ENCADRE:dessine le cadre
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn encadre(ligne: c_ushort, colon: c_ushort, haut: c_ushort, larg: c_ushort) {
    unsafe {
        /* passage au pixel */
        let mut ligne = (ligne * 7) as c_short;
        let mut colon = (colon * 6) as c_short;
        let mut haut = ligne + 7 * haut as c_short;
        let mut larg = colon + 6 * larg as c_short;
        /* petit delta pour polymorphisme*/
        ligne -= colon & 3;
        colon = colon - 2 - (haut & 3);
        haut += larg & 3;
        larg += ligne & 3;

        /* coin haut gauche vertical */
        affomb(MOB_CADRE!() + 0, ligne - 24, colon - 8, 8, 8, false);
        /* coin haut droit vertical */
        affomb(MOB_CADRE!() + 1, ligne - 24, larg - 8, 8, 8, false);
        /* coin haut gauche horizontal */
        affomb(MOB_CADRE!() + 2, ligne - 16, colon - 16, 8, 8, false);
        /* angle  haut gauche */
        affomb(MOB_CADRE!() + 3, ligne - 8, colon - 8, 8, 8, false);
        /* ligne horiz haute */
        let tailleh = (larg - colon - 1) / 16;
        if tailleh < 2000 {
            for i in 0..tailleh {
                affomb(MOB_CADRE!() + 4, ligne - 8, colon + 8 + i * 16, 8, 8, false);
            }
        }
        /* angle haut droit */
        affomb(MOB_CADRE!() + 5, ligne - 8, larg - 8, 8, 8, false);
        /* coin haut droit horizontal */
        affomb(MOB_CADRE!() + 6, ligne - 16, larg + 8, 8, 8, false);
        /* Block vertical */
        let taillev = (haut - ligne - 1) / 16;
        if taillev < 2000 {
            for i in 0..taillev {
                /* cote gauche */
                affomb(MOB_CADRE!() + 7, ligne + 8 + 16 * i, colon - 8, 8, 8, false);
                /* centre */
                if tailleh < 2000 {
                    for j in 0..tailleh {
                        affomb(
                            MOB_CADRE!() + 16,
                            ligne + 8 + 16 * i,
                            colon + 8 + 16 * j,
                            8,
                            8,
                            false,
                        );
                    }
                }
                /* cote droit */
                affomb(MOB_CADRE!() + 8, ligne + 8 + 16 * i, larg - 8, 8, 8, false);
            }
        }
        /* coin bas gauche horizontal */
        affomb(MOB_CADRE!() + 9, haut - 8, colon - 16, 8, 8, false);
        /* angle  bas gauche */
        affomb(MOB_CADRE!() + 10, haut - 8, colon - 8, 8, 8, false);
        /* ligne horiz basse */
        if tailleh < 2000 {
            for i in 0..tailleh {
                affomb(MOB_CADRE!() + 11, haut - 8, colon + 8 + i * 16, 8, 8, false);
            }
        }
        /* angle bas droit */
        affomb(MOB_CADRE!() + 12, haut - 8, larg - 8, 8, 8, false);
        /* coin bas droit horizontal */
        affomb(MOB_CADRE!() + 13, haut - 8, larg + 8, 8, 8, false);
        /* coin bas gauche vertical */
        affomb(MOB_CADRE!() + 14, haut + 7, colon - 16, 8, 8, false);
        /* coin bas droit vertical */
        affomb(MOB_CADRE!() + 15, haut + 7, larg, 8, 8, false);
    }
}

/***********************************************************************
                        INIMEN:Initialise Menu
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn inimen(entmenu: c_ushort) {
    unsafe {
        let mut j: c_short = 0;
        let t = TickCount();
        let refer1 = GetRefer1();

        menu = entmenu; /*force variable globale*/
        clin = tclin[menu as usize] as c_ushort;
        ccol = tccol[menu as usize] as c_ushort;
        colcdr = tcolcdr[menu as usize] as c_ushort;
        cnlin = tcnlin[menu as usize] as c_ushort;
        cncol = tcncol[menu as usize] as c_ushort;
        poscur = tposcur[menu as usize];
        stdlist = tstdlist[menu as usize];
        eff_sour();
        if menu != 0 {
            /*pas menu 0*/
            (*addr_of_mut!(smenu))[1] = menu as c_char;
            if menu != lastmen as c_ushort {
                /*deja affiche ?*/
                gemouvre(19, 1, 1, 14, clin, ccol, cnlin, cncol);
                encadre(clin, ccol, cnlin, cncol);
                lastmen = menu as c_uchar;
            }
        } else {
            criter = 0; /*menu principal : critere=0*/
            if presence != 0 && presence < 5 {
                liste0.taille = 6;
            } else {
                liste0.taille = 11;
            }
            if mort == 0 {
                /* On demarre a zéro */
                tlpos[0] = 1;
                liste0.taille = 2;
                clrbadi();
                comment(136); /* La partie est fini */
            }
        }
        lpos = tlpos[menu as usize] as c_uchar;
        /*reduction du nbre d'elements*/
        if criter != 0 {
            /*critere retenu*/
            let mut i: c_short = 0;
            i = 0;
            lpos = 111;
            while (i as c_ushort) != (*stdlist).taille as c_ushort {
                let rang: c_uchar = (*stdlist).elements[i as usize]; /*rang de l'objet*/
                let valeur: c_uchar = (*refer1.add(rang.wrapping_sub(1) as usize)).param1 & 0xF0;
                if ((criter == 1) && (valeur == 0x20)) || ((criter == 2) && (valeur == 0x40)) {
                    zoneu.elements[j as usize] = rang;
                    cross.elements[j as usize] = i as c_uchar;
                    j += 1;
                    if (i as c_char) >= tlpos[menu as usize] && lpos == 111 {
                        lpos = j as c_uchar;
                    }
                }
                i += 1;
            }
            if lpos == 111 {
                lpos = 1;
            }
            zoneu.taille = j as c_uchar;
            stdlist = &raw mut zoneu;
            /* nlle position ds menu*/
            lpos = tlpos[menu as usize] as c_uchar;
        }
        /*affichage de la liste*/
        if menu != 0 {
            sliste[1] = stdlist;
            (*addr_of_mut!(smenu))[1] = menu as c_char;
        }
        s_menu = menu as c_short;
        aflist(stdlist);
        while TickCount() < t + 6 {}
    }
}

/***********************************************************************
                      POS0_MOUSE
        Remets la souris en pos 0
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn pos0_mouse() {
    reset_mouse();
}

/***********************************************************************
                      GERE_SOURIS
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn gere_souris() {
    unsafe {
        bouge_souris();
        if sposy < 105 {
            icone = 0;
            eff_sour();
        } else {
            /* calcul position 53 du centre de la souris */
            let x: c_short = (sposx + 8) / 6;
            let y: c_short = (sposy + 8) / 7;
            /* calcul de l'icone (par defaut rien) */
            icone = INCONNU!();
            s_menu = -1;
            /* on balaye les menus */
            for i in 0..2 {
                let men: c_short = (*addr_of_mut!(smenu))[i] as c_short;
                /*on teste les verticales : men 111 implique pas de sousmenu*/
                if (men != 111)
                    && (x >= tccol[men as usize] as c_short)
                    && (x <= tccol[men as usize] as c_short + tcncol[men as usize] as c_short)
                {
                    s_menu = men;
                    stdlist = sliste[i as usize];
                    /* on teste les horizontales */
                    if y < tclin[men as usize] as c_short {
                        icone = UP!();
                    } else if y > tclin[men as usize] as c_short + tcnlin[men as usize] as c_short {
                        icone = DOWN!();
                    } else {
                        icone = SELECT!();
                    }
                }
            }
        }
    }
}

/***********************************************************************
                        TRPREN:Prends Objet
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trpren() {
    unsafe {
        if liste4.elements[rega as usize] == EAU {
            let mut quoi: c_uchar = 0;
            flefba = 0;
            if soif == 255 {
                comment(98); /*tu n'as pas soif*/
            } else {
                comment(96); /*Tu bois*/
            }
            soif = 255;
            /*on remplit les outres*/
            for i in 0..liste1.taille {
                if liste1.elements[i as usize] == OUTREVIDE {
                    liste1.elements[i as usize] = OUTREPLEINE;
                    quoi += 1;
                }
            }
            if quoi != 0 {
                comment(97); /*et remplis tes gourdes*/
            }
            flefba = 0;
            return;
        }
        let old_taille: c_short = liste1.taille as c_short;
        enlevl(&raw mut liste4, &raw mut liste1, rega as c_ushort);
        if liste1.taille as c_short == old_taille {
            flefba = 0;
            comment(101);
        } else if liste4.elements[rega as usize] == 44 {
            scenpr = 1; /*hache sacree*/
        }
        inimen(s_menu as c_ushort);
    }
}

/***********************************************************************
                        TRABSO:Absorbe
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trabso() {
    unsafe {
        if (faim > 192) && (soif > 192) {
            flefba = 0;
            comment(107);
            return;
        }

        let mut calcul: c_uchar = faim.wrapping_add((stock2 & 0xF0) / 4);
        if calcul >= faim {
            faim = calcul;
        } else {
            faim = 255;
        }
        calcul = soif.wrapping_add((stock2 & 0x0F) * 4);
        if calcul >= soif {
            soif = calcul;
        } else {
            soif = 255;
        }

        let objet: c_uchar = liste1.elements[rega as usize];
        if objet == OUTREPLEINE {
            liste1.elements[rega as usize] = OUTREVIDE;
        } else {
            enlevl(&raw mut liste1, &raw mut listbidon, rega as c_ushort);
        }
        inimen(1);
    }
}

/***********************************************************************
                        TRSOIN:Soigne
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trsoin() {
    unsafe {
        if bless > 192 {
            flefba = 0;
            comment(109);
            return;
        }

        let calcul: c_uchar = bless.wrapping_add((stock2 & 0x0F) * 10);
        if calcul > bless {
            bless = calcul;
        } else {
            bless = 255;
        }
        enlevl(&raw mut liste1, &raw mut listbidon, rega as c_ushort);
        inimen(1);
    }
}

/***********************************************************************
                        D032AYDE
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoAide() {
    unsafe {
        let mut chaine: [c_char; 255] = [0; 255];
        PushPath();
        SetPathInData(c"docs".as_ptr() as *mut c_char);
        GetNomCheminCourant(chaine.as_mut_ptr());
        if langue as c_short == FR as c_short {
            strcat(chaine.as_mut_ptr(), c"sapiens_fr.html".as_ptr());
        } else if langue as c_short == ES as c_short {
            strcat(chaine.as_mut_ptr(), c"sapiens_es.html".as_ptr());
        } else {
            strcat(chaine.as_mut_ptr(), c"sapiens_gb.html".as_ptr());
        }
        if OpenDocument(chaine.as_mut_ptr()) != 0 {
            Collapse(c"Can't open manual.".as_ptr() as *mut c_char);
        }
        PopPath();
    }
}

/***********************************************************************
               AffichePictAide
=> 0 vue laterale, 1 paysage
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffichePictAide(mode: c_short) {
    unsafe {
        let mut r = Rect::default();
        let mut gbak: CGrafPtr = null_mut();
        let num: c_short;

        AcamGetPortOffscreen(&mut gbak as *mut _);
        CarboniseSetPortWindow(mainWindow);
        if mode == 0 {
            SetRect(&mut r, 320 - 125, 230, 320 + 125, 230 + 160);
            num = 131;
        } else {
            SetRect(&mut r, 32, 230, 32 + 250, 230 + 160);
            num = 132;
        }
        AffichePictDansAire(num, &mut r);
        AcamSetPortOffscreen(gbak);
    }
}

/***********************************************************************
               TraiteVueLaterale
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn TraiteVueLaterale() {
    unsafe {
        MenuFenetres(FAUX as c_short);
        efbadass();
        TfrBadAss();
        bruit(9);
        /* On affiche le rectangle d'aide */
        AffichePictAide(0);
        effaceBulles();
        combat();
        pos0_mouse();
        MenuFenetres(VRAI as c_short);
    }
}

/***********************************************************************
                ACTION:Effectue une action
***********************************************************************/
pub fn action() {
    unsafe {
        let current_action = s_action;
        match current_action {
            1 => {
                trgest(rega as c_ushort);
                inimen(0);
            }
            8 => {
                trabso();
                ListAttendsFinBouton();
            }
            4 => {
                trpose();
                ListAttendsFinBouton();
            }
            3 => {
                trpren();
                ListAttendsFinBouton();
            }
            9 => {
                trsoin();
                ListAttendsFinBouton();
            }
            5 => {
                if presence > 0 {
                    let refer = GetRefer((*sliste[1]).typeRef);
                    if dervue != VUE_LATERALE as u8 {
                        tfrima();
                    }
                    let index = (*sliste[1]).elements[rega as usize] - 1;
                    let ptr = &(refer.wrapping_add(index as usize)) as *const _ as Ptr;
                    bulle(ptr, 0);
                    trdire(liste2.elements[rega as usize] as c_ushort); /*on envoie la phrase*/
                    ListAttendsFinBouton();
                } else {
                    /* Tu parles tou seul ... */
                    clrbadi();
                    comment(135);
                }
            }
            6 => {
                trdonn(rega as c_ushort); /*on envoit le rang de l'objet*/
                ListAttendsFinBouton();
            }
            7 => {
                trobsv(rega as c_ushort);
            }
            10 => {
                MenuFenetres(FAUX as c_short);
                trdors();
                MenuFenetres(VRAI as c_short);
            }
            11 => {
                MenuFenetres(FAUX as c_short);
                trfab(rega as c_ushort);
                MenuFenetres(VRAI as c_short);
            }
            _ => {
                warn!("Unexpected action {}", current_action);
            }
        }
    }
}

pub fn sapiens_main() {
    unsafe {
        initprim(); /*inits primaires*/
        //Final();
        //	DEF3D();
        clavm();
        action();
        loop {
            if flcomb > 0 {
                TraiteVueLaterale();
            }
        } /*infini...et meme un peu plus */
    }
}
/*---- FIN -----*/
