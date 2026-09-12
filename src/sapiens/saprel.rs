#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
/***********************************************************************
                SAPREL : Module relationnel (SAPREL.C porté en Rust)
                    (c) god Le 7 Novembre 1986
***********************************************************************/
use std::os::raw::{c_char, c_short, c_uchar, c_ulong, c_ushort, c_void};
use std::ptr::{addr_of, addr_of_mut};

use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, BackColor, BlockMove, CarboniseGetWindowPort,
    CloseRgn, DisposeRgn, EmptyRect, EraseRgn, FAUX, FrameRgn, FrameRoundRect, GetClip, LineTo,
    NewRgn, OffsetRect, OpenRgn, SetClip, SetRect, TickCount, UnionRect, VRAI,
};
use crate::sapiens::sapcomb::{generique, latera, laterb, prm};
use crate::sapiens::sapenreg::IsEnreg;
use crate::sapiens::sapglob::{
    agress, angsol, bless, bouton, ccol, charisme, chef, ciel_degr, clin, cncol, cnlin, dervue,
    derxj, deryj, destine, dismin, dquest, dsposx, dsposy, dzeta, echan, enerv, enetoi, entvie,
    faim, fatig, fdialo, finjou, flagbadi, flcomb, flefba, flgcharism, fpress, heure, jour_act,
    lastmen, ldzeta, linaff, list10, listbidon, liste1, liste4, lpos, metpos, minute, mort,
    palhaute, peuple, pos_evt, predil, presence, present, rega, resistance, s_action, s_menu, sap1,
    sap2, sap3, sap4, scenar, scenpr, smenu, soif, source, sposx, sposy, stock1, stock2, tabheur,
    tabprf, tagress, tenerv, tlpos, tobje, tribu, viecopain, vlchan, xj, yj,
};
use crate::sapiens::saptxt::{
    GetMesCon, GetRefer1, GetRefer3, GetScenme, maced, nomadversaire, nomcopain, nomsecondaire,
    nomtribu, poschef,
};
use crate::sapiens::sapwbrt::{
    CopyOffscreen, DoMoveTo, DoPenSize, TfrEcr, effvirt, tfrima, tfrvirt,
};
use crate::sapiens::sapwind::{
    Ecran1, GereInterface, MenuFenetres, Rnd, debecr, delai, mainWindow,
};
use crate::sapiens::sapwmus::{attends_bruit, bruit};

const ECHELLE: c_short = 2;

#[unsafe(no_mangle)]
pub static mut LastObsv: c_short = 0;

const X_BULLE_DROITE: c_short = 640 - 64 - 16;
const X_BULLE_GAUCHE: c_short = 80;
const Y_BULLE: c_short = 58;
const TAILLE_QUEUE: c_short = 20;

#[allow(non_upper_case_globals)]
static mut rBulleJoueur: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
};
#[allow(non_upper_case_globals)]
static mut rBulleAdversaire: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 0,
    right: 0,
};

/***********************************************************************
                        CLRBADI:efface bande si inutilisée
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn clrbadi() {
    unsafe {
        if flagbadi as u8 > 0 {
            efbadi();
            flagbadi = 0;
        }
    }
}

/***********************************************************************
                AFSTAT:affiche menu status
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn afstat() {
    unsafe {
        let refer3 = GetRefer3();
        (*refer3.wrapping_add(7)).nomref[0] = ((faim / 64) + 0x91) as c_char;
        (*refer3.wrapping_add(8)).nomref[0] = ((soif / 64) + 0x91) as c_char;
        (*refer3.wrapping_add(9)).nomref[1] = ((fatig / 64) + 0x95) as c_char;
        (*refer3.wrapping_add(10)).nomref[1] = ((bless / 64) + 0x9A) as c_char;
        (*refer3.wrapping_add(11)).nomref[1] = (mort + 0xA7) as c_char;
        inimen(7);
        s_action = 2;
    }
}

/*************************************************************************
                    TailleChaine
Affiche un caractere sur le port en entrée
=> Chaine a traiter
@= taille maximale en colonne x, nbre de lignes t
**************************************************************************/
unsafe fn TailleChaine(mut debut: *mut c_char, tx: *mut c_short, ty: *mut c_short) {
    unsafe {
        let mut x: c_short = 0;
        let mut taillex: c_short = 0;
        let mut y: c_short = 0;

        *ty = 0;
        while *debut != 0 {
            if *debut == b'\r' as c_char {
                *ty += 1;
                x = 0;
                debut = debut.add(1);
            } else {
                TailleCarMess(&mut debut, &mut x, &mut y);
                if x > taillex {
                    taillex = x;
                }
                *ty += y;
            }
        }
        *tx = taillex;
        return;
    }
}

unsafe fn TailleCarMess(debut: *mut *mut c_char, taillex: *mut c_short, tailley: *mut c_short) {
    unsafe {
        let p = *debut;
        let car = *p as u8 as c_short;
        *debut = p.add(1);
        if car > 128 {
            let chaine = GetStrBkMot(car);
            let mut tx = 0;
            let mut ty = 0;
            TailleChaine(chaine, &mut tx, &mut ty);
            *taillex += tx;
            *tailley += ty;
        } else {
            *taillex += TailleCar(car, 10);
        }
        return;
    }
}

/***************************************************************************
                Sappause: effectue une pause ou sort si touche au clavier
****************************************************************************
=> durée de la pause
*/
#[allow(dead_code)]
unsafe fn Sappause(mut duree: c_ushort) {
    unsafe {
        while bouton != 0 {
            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
        }
        dsposx = sposx;
        dsposy = sposy;
        while duree != 0 && bouton == 0 && sposx == dsposx && sposy == dsposy {
            delai(1);
            duree -= 1;
            vq_mouse(&raw mut bouton, &raw mut sposx, &raw mut sposy);
        }
        return;
    }
}

/***********************************************************************
                        EFBADI
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn efbadi() {
    unsafe {
        Gotoxy(3, 15);
        efbadass();
    }
}

/***********************************************************************
                        scrollBADI
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn scrollbadi() {
    unsafe {
        Gotoxy(3, 15);
        scrollbadass();
    }
}

/***********************************************************************
                        SCEVEU:
Affiche le message correspondant à la phase du scenario
Le message s'affiche sur l'aire 3d
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn sceveu(quoi: c_ushort) {
    unsafe {
        let mut rsourc = Rect {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        };
        let mut rdest = Rect {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        };
        let scenme = GetScenme();

        dervue = VUE_AUCUNE as u8;
        if quoi == 6 {
            gemouvre(6, 32, 1, 2, 1, 8, 8, 38);
        } else {
            gemouvre(6, 32, 1, 2, 1, 8, 5, 38);
        }
        /* on efface la zone */
        effvirt(0x007);

        Gotoxy(12, 1);
        let mut p = *scenme.add((quoi - 1) as usize);
        let mut car = *p as u8;
        p = p.add(1);
        /* On affiche sur debecr */
        while car != 0 {
            car2(car as c_short);
            loop {
                car = *p as u8;
                p = p.add(1);
                if car == 13 {
                    Gotoxy(12, linaff + 1);
                }
                if car != 13 {
                    break;
                }
            }
        }
        if quoi == 6 {
            for _i in 0..3 {
                bruit(6);
                attends_bruit();
            }
        }
        /* Transfert debecr vers reel */
        TfrEcr(core::ptr::null_mut());
        if quoi == 6
        /* Bravo tu as gagné */
        {
            SetRect(
                &mut rdest,
                0,
                0,
                (255 * ECHELLE) as c_short,
                (100 * ECHELLE) as c_short,
            );
            SetRect(
                &mut rsourc,
                (32 * ECHELLE) as c_short,
                ECHELLE as c_short,
                ((32 + 255) * ECHELLE) as c_short,
                ((1 + 100) * ECHELLE) as c_short,
            );

            CopyOffscreen(debecr, Ecran1, &mut rsourc, &mut rdest);
            delai(15 * 60);
            Final();
        }
        return;
    }
}

/***********************************************************************
                        SceveuBulle:
Affiche le message correspondant à la phase du scenario
Le message s'affiche sur l'aire 3d
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn SceveuBulle(quoi: c_ushort) {
    unsafe {
        let scenme = GetScenme();
        effaceBulles();
        bulle(*scenme.add(quoi as usize) as Ptr, 1);
        return;
    }
}

/***********************************************************************
                        GAINSBOUR:Dégradation Physique
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn gainsbour() {
    unsafe {
        bless = bless.wrapping_sub((6 - 6 * resistance as c_short / 256) as c_uchar);
        if bless > 251 {
            if mort != 0
            /*pas mort encore*/
            {
                mort -= 1;
                bless = 20;
            } else {
                presence = 0;
                fdialo = 0;
            }
        }
    }
}

/***********************************************************************
                        PASTMP:Le temps Passe
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn pastmp() {
    unsafe {
        let mut i = liste1.taille;
        while i != 0 {
            i -= 1;
            /*la viande se corromps*/
            if liste1.elements[i as usize] >= 37 && liste1.elements[i as usize] < 41 {
                liste1.elements[i as usize] += 1;
            }
        }
        return;
    }
}

/***********************************************************************
                        TFRENE:
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn tfrene() {
    unsafe {
        if presence < 5 && presence != 0 {
            if fpress != 0 {
                enetoi[tribu as usize - 1] = enerv;
                if chef != 0 || tribu != 1 {
                    tagress[tribu as usize - 1] = agress;
                }
            }
        }
        return;
    }
}

/**************************************************************************
                COMMENT:affiche un commentaire
*************************************************************************
=> rang du commentaire
*/
#[unsafe(no_mangle)]
pub extern "C" fn comment(rang: c_ushort) {
    unsafe {
        let mut p: *mut c_uchar;
        let mescon = GetMesCon();

        if flefba == 0 {
            flefba += 1;
            scrollbadi();
        }
        flagbadi = 1;
        p = *mescon.add((rang - 1) as usize) as *mut c_uchar;
        while *p != 0 {
            affcarmess(*p as c_short);
            p = p.add(1);
        }
        TfrBadAss();
        return;
    }
}

/**************************************************************************
                EffaceBulles
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn effaceBulles() {
    unsafe {
        tfrvirt();
        if !EmptyRect(&raw const rBulleJoueur) {
            CopyOffscreen(
                debecr,
                CarboniseGetWindowPort(mainWindow),
                &raw mut rBulleJoueur,
                &raw mut rBulleJoueur,
            );
        }
        SetRect(&raw mut rBulleJoueur, 0, 0, 0, 0);
        if !EmptyRect(&raw const rBulleAdversaire) {
            CopyOffscreen(
                debecr,
                CarboniseGetWindowPort(mainWindow),
                &raw mut rBulleAdversaire,
                &raw mut rBulleAdversaire,
            );
        }
        SetRect(&raw mut rBulleAdversaire, 0, 0, 0, 0);
        return;
    }
}

/**************************************************************************
                Bulle:affiche une bulle avec un texte
=> message a afficher, qui le dit (0)== joueur (1)== adversaire
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn bulle(debut: Ptr, qui: c_short) {
    unsafe {
        let mut r = Rect {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        };
        let mut rbase: Rect;
        let rgnclip: RgnHandle;
        let rgn: RgnHandle;
        let mut gbak: CGrafPtr = core::ptr::null_mut();
        let mut x: c_short;
        let mut y: c_short;
        let mut tx: c_short = 0;
        let mut ty: c_short = 0;
        let mut p: Ptr = debut;

        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
        /* Calcul de la taille de la chaine en pixel */
        TailleChaine(p as *mut c_char, &mut tx, &mut ty);
        if tx < 64 {
            tx = 64;
        }
        if (qui ^ sap1.sens as c_short) != 0 {
            SetRect(
                &mut r,
                X_BULLE_GAUCHE,
                Y_BULLE - (ty + 2) * 12,
                X_BULLE_GAUCHE + tx + 16,
                Y_BULLE,
            );
        } else {
            SetRect(
                &mut r,
                X_BULLE_DROITE - tx - 16,
                Y_BULLE - (ty + 2) * 12,
                X_BULLE_DROITE,
                Y_BULLE,
            );
        }
        r.bottom += TAILLE_QUEUE;
        rbase = r;
        /* On efface les bulles en cours */
        if qui == 0
        /* C'est le joueur */
        {
            effaceBulles();
            rBulleJoueur = r;
        } else {
            /* C'est un adversaire ... */
            /* deja un message dit ??? */
            if !EmptyRect(&raw const rBulleAdversaire) {
                OffsetRect(
                    &mut r,
                    0,
                    rBulleAdversaire.bottom - rBulleAdversaire.top - TAILLE_QUEUE - 4,
                );
                UnionRect(
                    &raw const rBulleAdversaire,
                    &raw const r,
                    &raw mut rBulleAdversaire,
                );
                rbase = rBulleAdversaire;
            } else {
                rBulleAdversaire = r;
            }
        }
        r.bottom -= TAILLE_QUEUE;
        rgn = NewRgn();
        rgnclip = NewRgn();
        OpenRgn();
        FrameRoundRect(&r, 16, 16);
        /* Bulle a gauche */
        if (qui ^ sap1.sens as c_short) != 0 {
            x = r.left + 48;
            y = r.bottom;
            DoMoveTo(x, y);
            LineTo(x - TAILLE_QUEUE, y + TAILLE_QUEUE);
            LineTo(x - 8, y);
            LineTo(x, y);
        } else {
            x = r.right - 48;
            y = r.bottom;
            DoMoveTo(x, y);
            LineTo(x + TAILLE_QUEUE, y + TAILLE_QUEUE);
            LineTo(x + 8, y);
            LineTo(x, y);
        }
        CloseRgn(rgn);
        BackColor(whiteColor as c_short);
        EraseRgn(rgn);
        DoPenSize(1, 1);
        FrameRgn(rgn);
        GetClip(rgnclip);
        SetClip(rgn);
        x = r.left + 8;
        y = r.top + 6;
        GotoxyPixel(x, y);
        while *p == b' ' {
            p = p.add(1);
        }
        while *p != 0 {
            if *p == b'\r' {
                y += 14;
                GotoxyPixel(x, y);
                loop {
                    p = p.add(1);
                    if *p != b' ' {
                        break;
                    }
                }
            } else {
                affcar10(*p as c_short);
                p = p.add(1);
            }
        }
        SetClip(rgnclip);
        DisposeRgn(rgn);
        DisposeRgn(rgnclip);
        AcamSetPortOffscreen(gbak);
        TfrEcr(&mut rbase);
        DoPenSize(1, 1);
        return;
    }
}

/***********************************************************************
                        CHRONO:Effectue Chronologie
Heure varie de [0 à 1023]
Matin :              0-90
Apres midi :         426-512
Nuit :               512-602
Fin de nuit          938-1023

***********************************************************************/
/*duree du jour 4*90=360 heures*/
/*quoi 0:normal,1:ni param en baisse ni message,2 baisse mais pas mess*/
#[unsafe(no_mangle)]
pub extern "C" fn chrono(quoi: c_short) {
    unsafe {
        let mut avemess: c_uchar;

        if minute != 0 {
            minute -= 1;
            return;
        }
        minute = 5;
        heure += 1;
        if (heure & 255) == 90 {
            heure += 336;
        }
        heure &= 0x03ff;

        if (heure & 0xFF) == 0 {
            pastmp();
        }
        /*mise en place des evts couleurs*/
        while heure >= tabheur[pos_evt as usize].h as c_ushort {
            /* dois t'on modifier le degrad couleur */
            if tabheur[pos_evt as usize].q != 0 {
                /* Changement du degrade du ciel avec les huits couleurs ST en entrée*/
                ciel_degr = tabheur[pos_evt as usize].pal.as_mut_ptr();
                FixeDegradesFonctionHeure(addr_of_mut!(tabheur[pos_evt as usize]));
            } else {
                /* On fixe la pal haute */
                palhaute = tabheur[pos_evt as usize].pal.as_mut_ptr();
            }
            pos_evt += 1;
            if pos_evt >= NBREVTHEURE as c_short {
                pos_evt = 0;
                heure = 0;
            }
        }
        if heure == 0 {
            flefba = 0;
            comment(114); /* Un nouveau jour ...*/
            jour_act += 1;
        }
        /* Position du meteore */
        if heure & 0x100 != 0 {
            metpos = (heure & 0xFF).wrapping_sub(180);
            angsol = 256;
        } else {
            metpos = 75u16.wrapping_sub(heure & 0xFF);
            angsol = 768;
        }
        if quoi == 1 {
            return;
        }
        finjou = 1; /*deperissement de la viande*/
        faim = faim.wrapping_sub(2);
        avemess = 0;
        if faim > 253 {
            faim = 5;
            gainsbour();
            avemess = 93;
        }
        soif = soif.wrapping_sub(4);
        if soif > 251 {
            soif = 5;
            gainsbour();
            avemess = 94;
        }
        fatig = fatig.wrapping_sub(2);
        if fatig > 253 {
            fatig = 5;
            gainsbour();
            avemess = 95;
        }
        if avemess != 0 && quoi == 0 {
            efbadi();
            comment(avemess as c_ushort);
        }
        return;
    }
}

/**************************************************************************
                CAUSE: Sort à l'ecran une phrase correspondant
                             à un contexte donné
*************************************************************************
=> Contexte de 1 à ....
*/
unsafe fn cause(context: c_ushort, mess_add: *const c_char) {
    unsafe {
        let rang: c_short;
        let mut chaine = [0 as c_char; 255];
        let mescon = GetMesCon();

        /*y cause ti ?*/
        if fdialo != 0
        /*y cause */
        {
            rang = ((context - 1) * 4 + (Rnd() & 3) as c_ushort) as c_short;
            strcpy(chaine.as_mut_ptr(), *mescon.add(rang as usize));
            if !mess_add.is_null() {
                strcat(chaine.as_mut_ptr(), mess_add);
            }
            bulle(chaine.as_mut_ptr() as Ptr, 1); /*affiche le commentaire*/
        }
        return;
    }
}

/***********************************************************************
                        FUITE:
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn fuite() {
    unsafe {
        flefba = 0;
        if agress < 245 {
            agress = agress.wrapping_add(10); /*l'agressivite augmente*/
        }
        comment(80); /*il s'enfuit*/
        quitte();
    }
}

/***********************************************************************
                INTRSQ:recherche valeur d'un objet
***********************************************************************
=> Objet a rechercher.La recherche se fait sur refer1
<= valeur de l'objet f(presilection)
*/
#[unsafe(no_mangle)]
pub extern "C" fn intrsq(objet: c_ushort) -> c_ushort {
    unsafe {
        let refer1 = GetRefer1();

        let mut valeur = (*refer1.add((objet - 1) as usize)).param1 & 0xF;
        if objet == predil as c_ushort {
            valeur = valeur * 2;
        }
        return valeur as c_ushort;
    }
}

/***************************************************************************
                 RAPIAS:receherche objet interessant ds liste
**************************************************************************
=>tribu
<= valeur et rang de la valeur
*/
#[unsafe(no_mangle)]
pub extern "C" fn rapias() -> c_ushort {
    unsafe {
        let mut max: c_uchar = 0;
        let mut objet: c_uchar = 0;
        let mut valeur: c_uchar;
        let mut i: c_short = 0;
        while (i as c_uchar) < liste1.taille {
            valeur = intrsq(liste1.elements[i as usize] as c_ushort) as c_uchar;
            if valeur > max {
                max = valeur;
                objet = liste1.elements[i as usize];
            }
            i += 1;
        }
        return objet as c_ushort;
    }
}

/***********************************************************************
                EXCITE:Augmente enervement
************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn excite(valeur: c_ushort) {
    unsafe {
        let tfr = enerv.wrapping_add(valeur);
        if enerv <= tfr {
            enerv = tfr;
        } else {
            enerv = 65535;
        }

        if flefba != 0 {
            /*flefba=0;
            Sappause(2000);*/
        }
        if enerv < 51 * 256
        /*tres calme*/
        {
            if fpress != 0 {
                cause(7, core::ptr::null());
            } else {
                cause(2, core::ptr::null());
            }
            return;
        }
        if enerv < 102 * 256
        /*peu enerve*/
        {
            cause(8, core::ptr::null());
            return;
        }
        if (Rnd() as c_ushort) < enerv / 512
        /*assez enerve*/
        {
            inimen(0);
            flcomb = 1; /*combat lancé*/
            return;
        }
        if enerv < 153 * 256 {
            cause(9, core::ptr::null());
            return;
        }
        if enerv < 204 * 256 {
            cause(10, core::ptr::null());
            return;
        }
        cause(11, core::ptr::null());
        return;
    }
}

/***************************************************************************/
/*              MODREL:Initialise l'enervement si pas DONNE ou DIRE        */
/***************************************************************************
=> Les deux parametres correspondants au choix
*/
#[unsafe(no_mangle)]
pub extern "C" fn modrel() {
    unsafe {
        let tfr: c_ushort;
        if tlpos[0] as u8 == 5 /*dire*/ || tlpos[0] as u8 == 6 /*donner*/ || fdialo == 0 {
            return;
        }
        /*Autre action alors qu'adversaire presentt => enervement*/
        tfr = enerv.wrapping_add(100);
        if enerv < tfr {
            enerv = tfr;
        }
        if flefba != 0 {
            /*attends une duree ou une touche*/
            /*Sappause (100);
            flefba=0;*/
        }
        if enerv < 51 * 256
        /*pas tres enerve*/
        {
            if fpress != 0 {
                cause(7, core::ptr::null());
            } else {
                cause(2, core::ptr::null());
            }
            return;
        }
        if enerv < 102 * 256 {
            cause(8, core::ptr::null()); /*moyennement enerve*/
        }
        if (Rnd() as c_ushort) > enerv / 512 {
            if enerv < 153 * 256 {
                cause(9, core::ptr::null());
            } else {
                if enerv < 204 * 256 {
                    cause(10, core::ptr::null());
                } else {
                    cause(11, core::ptr::null());
                }
            }
        }
        return;
    }
}

/***********************************************************************
                      PRENDS:On prends l'objet de rang Rega
                      dans la liste des objets du sap1 et on le transfere
                      dans la liste 10
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn prends() {
    unsafe {
        let erreur = enlevl(&raw mut liste1, &raw mut list10, rega as c_ushort);
        if erreur == 1 && lpos > 1 {
            lpos -= 1; /*elt est le dernier*/
        }
        inimen(s_menu as c_ushort);
        return;
    }
}

/**************************************************************************
                TRDONN:donne
unsigned short rangb;simule registre B [0,eltmax[
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trdonn(rangb: c_ushort) {
    unsafe {
        let objet: c_uchar;
        let tfr: c_uchar;
        let mut valeur: c_ushort;
        let mescon = GetMesCon();

        if fdialo == 0 {
            return; /*pas de dialogue alors rien*/
        }
        objet = liste1.elements[rangb as usize]; /*on prendsl'objet ds la liste*/
        flefba = 0;
        if chef != 0 && tribu == 1
        /*on donne à son chef*/
        {
            if scenar != 1 {
                /* on compare l'objet
                l'objet desiré*/
                if objet != (scenar as c_uchar) + 40
                /*objet du scenario*/
                {
                    SceveuBulle((scenar - 1) as c_ushort); /*on raffiche l'objet desire*/
                    return;
                }
            } else
            /*scenario = viande*/
            {
                if objet < 35 || objet > 39
                /*viande pourrie ou autre*/
                {
                    SceveuBulle((scenar - 1) as c_ushort);
                    return;
                }
                scenpr = ((40 - objet) as i32 - 1) as c_char; /*on enleve au total*/
                if scenpr > 0
                /* pas fini !!!*/
                {
                    prends(); /* enleve viande*/
                    list10.taille = 0; /* mise à zéro taille*/
                    effaceBulles();
                    cause(6, core::ptr::null()); /* donne encore*/
                    return;
                }
            }
            if !IsEnreg() {
                SceveuBulle(9);
                return;
            }
            prends(); /*on prends la chose*/
            /* Remerciement du chef */
            effaceBulles();
            match scenar {
                1 => bulle(*mescon.add(81) as Ptr, 1),
                2 => bulle(*mescon.add(82) as Ptr, 1),
                3 => bulle(*mescon.add(83) as Ptr, 1),
                4 => bulle(*mescon.add(83) as Ptr, 1),
                _ => {}
            }
            scenar += 1;
            if scenar == 5 {
                /*combat ultime*/
                flcomb = 1;
                SceveuBulle((scenar - 1) as c_ushort);
                /* Delay de 5 seconde */
                delai(5 * 60);
                return;
            }
            if scenar == 3 {
                /*bras de roc 'achement enerves et on doit lui voler sa hache*/
                enetoi[2] = 60000;
            }
            /*comment(80 + scenar);*/
            /*affiche commentaire selon scenar*/
            return; /*fin partie donne->scenario*/
        }
        /*-----------------------*/
        /* on donne hors scenario  */
        /*-----------------------*/

        /*cherche valeur de l'objet*/
        effaceBulles();
        valeur = enerv.wrapping_sub(512 * intrsq(liste1.elements[rega as usize] as c_ushort));
        /*compense l'enervement*/
        if valeur > enerv {
            /*bien compense*/
            valeur = 0;
        }
        if fpress == 0
        /*pas presente => cado */
        {
            if list10.taille <= list10.maxtaille {
                prends(); /*y a de la place : on prends*/
                if enerv < 102 * 256 {
                    cause(1, core::ptr::null()); /*merci*/
                    cause(12, core::ptr::null()); /*je suis ...*/
                }
                if agress > 0 {
                    agress -= 1;
                }
                /*l'agressivite descend*/
                /*Sappause(2000);*/
                enerv = valeur; /*l'enervement diminue*/
                excite(0); /*on affiche l'etat d'enerv*/
            }
            return;
        }
        /*---------*/
        /* echange */
        /*---------*/
        if enerv < 102 * 256 {
            valeur = intrsq(liste1.elements[rega as usize] as c_ushort);
            if chef != 0 && scenar as c_ushort == tribu as c_ushort {
                if liste1.elements[rega as usize] != predil {
                    valeur = 0;
                } else {
                    valeur = 200;
                }
            }
            if valeur > vlchan as c_ushort
            /*valeur superieure*/
            {
                /*ok il prends*/
                vlchan = valeur as c_uchar;
                cause(4, core::ptr::null()); /*marché conclu*/
                if echan == 1
                /*protection*/
                {
                    enetoi[tribu as usize - 1] = 0; /*enerv =0*/
                    echan = 25; /*onguent*/
                }
                /*on permute les deux*/
                else {
                    tfr = liste1.elements[rangb as usize];
                    liste1.elements[rangb as usize] = echan;
                    echan = tfr;
                    inimen(s_menu as c_ushort);
                }
            } else
            /*insuffisant*/
            {
                cause(3, core::ptr::null()); /*c'est pas assez glandu !!!*/
                excite(50);
            }
        }
        /*si donne et pas trop enerve*/
        else
        /*enerv<e*/
        {
            if list10.taille < list10.maxtaille {
                /*il ya de la place*/
                prends();
                enerv = valeur;
                cause(5, core::ptr::null()); /*merci du cado*/
                /*..
                suivre...*/
            }
        }
        return;
    }
}

/***********************************************************************
                IDIALO:Initialise l'adversaire en fn de la presence
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn idialo() {
    unsafe {
        let mut i: c_short;
        let mut pos: c_uchar;
        fdialo = 0;
        flcomb = 0;
        fpress = 0;
        flgcharism = 0;
        dquest = 0;
        if presence == 0 {
            return; /*si personne */
        }
        if presence > 4
        /*bestiole*/
        {
            echan = 0;
            flcomb += 1; /*on se bat*/
            return;
        }
        fdialo += 1; /*on discute*/
        enerv = tenerv[tribu as usize - 1];
        agress = tagress[tribu as usize - 1];
        predil = tabprf[tribu as usize - 1];

        /*objet
        echanger*/
        echan = tobje[tribu as usize - 1][(Rnd() & 3) as usize];

        /*objet portes*/
        for i in 0..2 {
            list10.elements[i] = tobje[tribu as usize - 1][(Rnd() & 3) as usize];
        }
        list10.taille = 2;
        /*bkmots[3][1]=(tribu+0x85);*/
        /*tribu*/
        nomtribu[1] = (tribu + 0x85) as c_char;
        nomtribu[2] = 0;

        if chef != 0 {
            /*c'est un chef*/
            nomtribu[0] = 0x85u8 as c_char; /*Chef*/
            nomadversaire[0] = (tribu + 0x89) as c_char;
            nomadversaire[1] = 0;
            /*   bkmots[2][0]=(tribu+0x89);*/
            /*nom du chef*/
            /* bkmots[2][1]=0;*/
            /*fin*/
            /*  bkmots[3][0]=0x85;*/
            /*'chef'*/
            if tribu == 1 {
                fpress = 1;
                enerv = enetoi[tribu as usize - 1];
            }
            if scenar == tribu && tribu != 4 {
                echan = tribu + 40; /*objet volé par la tribu*/
            } else {
                echan = 1; /*ma protection*/
            }
        } else {
            /*bkmots[3][0]=32 ;*/
            nomtribu[0] = 0x20;
            i = 0;
            while i < 6 {
                /*determine le nom*/
                pos = (48 * Rnd() as c_short / 256) as c_uchar;
                nomadversaire[i as usize] = maced[pos as usize];
                i += 1;
                pos += 1;
                nomadversaire[i as usize] = maced[pos as usize];
                i += 1;
                pos += 1;
                nomadversaire[i as usize] = maced[pos as usize];
                i += 1;
            }
            nomadversaire[i as usize] = 0;
        }
        /*tire nom secondaire*/
        i = 0;
        while i < 6 {
            /*determine le nom*/
            pos = (48 * Rnd() as c_short / 256) as c_uchar;
            nomsecondaire[i as usize] = maced[pos as usize];
            i += 1;
            pos += 1;
            nomsecondaire[i as usize] = maced[pos as usize];
            i += 1;
            pos += 1;
            nomsecondaire[i as usize] = maced[pos as usize];
            i += 1;
        }
        nomsecondaire[i as usize] = 0;
        vlchan = intrsq(echan as c_ushort) as c_uchar;
        smenu[1] = 111; /* idee d'yves */

        inimen(0);
        return;
    }
}

/***********************************************************************
                        QUITTE:On lui fait peur :il s'enfuit
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn quitte() {
    unsafe {
        tfrene();
        flcomb = 0;
        presence = 0;
        idialo();
        present[1] = 0;
        present[3] = 0;
        prm(&raw mut sap2, 0);
        prm(&raw mut sap4, 0);
        /* inits souris */
        dsposx = sposx;
        dsposy = sposy;
        reset_mouse();
        if dervue == VUE_LATERALE as u8 {
            laterb(false);
        } else {
            latera();
        }
        generique(true);
        inimen(0);
        return;
    }
}

/***********************************************************************
                        CREECOP:Crée un copain
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn creecop(qui: c_ushort) {
    unsafe {
        /*c'est ds la poche*/
        /*on recopie sap2 sur sap3*/
        BlockMove(
            addr_of!(sap2.cross) as *const c_void,
            addr_of_mut!(sap3.cross) as *mut c_void,
            8,
        ); /*parametres*/
        if presence < 5 {
            strcpy(
                addr_of!(nomcopain) as *mut c_char,
                addr_of!(nomadversaire) as *const c_char,
            ); /*nom*/
        } else {
            nomcopain[0] = (128 + 45) as c_char; /*Ton ami*/
            nomcopain[1] = 0;
        }
        prm(&raw mut sap3, qui as c_short);
        sap3.colonne = sap2.colonne;
        sap3.sens = sap2.sens;
        sap3.coeur = sap2.coeur;
        sap3.cptrsouffle = 1;
        prm(&raw mut sap2, 0);
        if dervue != VUE_PAYSAGE as u8
        /* on est en vue laterale*/
        {
            presence = 0;
            trobsv(1);
        }
        echan = 0;
        (*(**addr_of_mut!(peuple[2])).possession).taille = 0;
        generique(true);
        return;
    }
}

/***********************************************************************
                        BONNECHASS
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn bonnechass() {
    unsafe {
        let mescon = GetMesCon();
        if enerv > 102 * 256
        /*énervé!!!*/
        {
            excite(300);
        } else {
            if fpress == 0
            /*pas présenté*/
            {
                cause(2, core::ptr::null()); /*qui est tu ???*/
            } else {
                /*il s'en va*/
                /*bonne chasse*/
                bulle(*mescon.add(72) as Ptr, 1);
                quitte();
                inimen(0);
                effaceBulles();
            }
        }
        return;
    }
}

/***********************************************************************
                        VIENSAVEC
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn viensavec() {
    unsafe {
        let mescon = GetMesCon();
        if fpress != 0
            && presence != 0
            && presence < 5
            && present[2] == 0
            && present[3] == 0
            && chef == 0
            && flgcharism == 0
        {
            if (charisme as i32 * Rnd() as i32) > enerv as i32 && Rnd() < 70 {
                creecop(1);
                viecopain = 3;
                sap3.vie = 200;
                /*je suis avec toi*/
                bulle(*mescon.add(99) as Ptr, 1);
                flagbadi = 0;
            } else {
                excite(1500);
            }
        } else {
            /*je ne suis pas celle que vous croyez*/
            bulle(*mescon.add(98) as Ptr, 1);
            excite(2000);
        }
        flgcharism = 1;
        return;
    }
}

/***********************************************************************
                        TUVEUQUOI
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn tuveuquoi() {
    unsafe {
        let valeur: c_uchar;
        let refer1 = GetRefer1();
        let mescon = GetMesCon();
        let scenme = GetScenme();

        if enerv > 102 * 256 {
            cause(13, core::ptr::null()); /*ta vie !!*/
        } else {
            if chef != 0 && tribu == 1 {
                dquest = 0;
                bulle(*scenme.add((scenar as usize) - 1) as Ptr, 1);
            } else {
                if tribu == scenar && chef != 0 {
                    cause(
                        14,
                        addr_of_mut!((*refer1.add((predil.wrapping_sub(1)) as usize)).nomref[0]),
                    );
                    return;
                }
                valeur = intrsq(rapias()) as c_uchar;
                if valeur > vlchan {
                    cause(
                        14,
                        addr_of_mut!((*refer1.add((rapias() - 1) as usize)).nomref[0]),
                    ); /*je veux*/
                } else {
                    if chef != 0 {
                        cause(
                            14,
                            addr_of_mut!(
                                (*refer1.add((predil.wrapping_sub(1)) as usize)).nomref[0]
                            ),
                        );
                    } else {
                        /*rien !!!*/
                        bulle(*mescon.add(65) as Ptr, 1);
                        excite(0);
                    }
                }
            }
        }
        return;
    }
}

/***********************************************************************
                        TRDIRE:Célakonköse
***********************************************************************
=> DIQuoi : Le numéro de la phrase ds la liste des phrases
*/
#[unsafe(no_mangle)]
pub extern "C" fn trdire(diquoi: c_ushort) {
    unsafe {
        let tfr: c_short;
        let mut enerv2: c_ushort;
        let refer1 = GetRefer1();
        let mescon = GetMesCon();

        if fdialo == 0 {
            return; /*si pas de dialogue alors rien*/
        }
        flefba = 1; /* pas d'effacement au prochain*/

        if diquoi == dquest as c_ushort {
            excite(1000);
            return;
        }
        dquest = diquoi as c_uchar; /*derniere question*/

        if diquoi == 5
        /*je suis tedehur*/
        {
            if fpress == 0
            /*pas encore presente*/
            {
                fpress += 1; /* on est présenté*/
                /*calcul de la reponse en fn de l'enervement*/
                tfr = (enerv as i32 - tenerv[tribu as usize - 1] as i32) as c_short;
                enerv = enetoi[tribu as usize - 1];
                enerv2 = (enerv as i32).wrapping_add(tfr as i32) as c_ushort;
                if tfr > 0 && enerv2 < enerv {
                    enerv2 = 65535;
                }
                if tfr < 0 && enerv2 > enerv {
                    enerv2 = 0;
                }
                enerv = enerv2;
                if enerv > 0x3400 {
                    /*c'est donc toi*/
                    bulle(*mescon.add(71) as Ptr, 1);
                    excite(0);
                } else {
                    /*bonjour*/
                    bulle(*mescon.add(70) as Ptr, 1);
                }
            } else {
                /*deja présenté*/
                excite(600); /*on s'enerve*/
            }
            return;
        }

        if diquoi == 6 {
            bonnechass();
            return;
        }
        if diquoi == 7 || diquoi == 8
        /*viens avec moi*/
        {
            viensavec();
            return;
        }

        /*la phrase n'est ni je suis ni au revoir */
        if stock1 == 0 {
            if diquoi == 1
            /*qui est tu ?*/
            {
                if enerv > 102 * 256 {
                    excite(100);
                } else {
                    cause(12, GetStrBkMot(129 + 3)); /*'je suis'*/
                    if fpress == 0 && chef == 0 {
                        /*et toi??*/
                        bulle(*mescon.add(64) as Ptr, 1);
                        excite(200);
                    }
                }
                return;
            }
            if diquoi == 2
            /*que veux tu*/
            {
                tuveuquoi();
                return;
            }
            if diquoi == 3
            /*que m'offres tu ?*/
            {
                if enerv > 256 * 102
                /*trop énervé pour négocier*/
                {
                    /*rien...*/
                    bulle(*mescon.add(65) as Ptr, 1);
                    tfr = (enerv as i32).wrapping_add(200) as c_short;
                    if (enerv as i32) < (tfr as i32) {
                        enerv = tfr as c_ushort;
                    }
                } else {
                    if fpress == 0 {
                        cause(2, core::ptr::null()); /*qui est tu ???*/
                    } else {
                        cause(
                            15,
                            addr_of_mut!((*refer1.add(echan.wrapping_sub(1) as usize)).nomref[0]),
                        ); /*moi offrir*/
                    }
                }
                return;
            }
            /*ou est ton chef*/
            if chef != 0
            /*c'est lui !!!*/
            {
                excite(32700); /*il s'excite*/
            } else {
                if enerv > 51 * 256 {
                    cause(16, core::ptr::null()); /*je dirais rien*/
                    tfr = (enerv as i32).wrapping_add(256) as c_short;
                    if (enerv as i32) < (tfr as i32) {
                        enerv = tfr as c_ushort;
                    }
                } else {
                    if fpress == 0 {
                        cause(2, core::ptr::null()); /*qui est tu*/
                    } else {
                        poschef[0] = 32;
                        poschef[2] = 32;
                        /*determination de la position du chef*/
                        if xj / 256 < destine[tribu as usize - 1].posx as c_ushort {
                            poschef[0] = (128 + 31) as c_char;
                        }
                        if xj / 256 > destine[tribu as usize - 1].posx as c_ushort {
                            poschef[0] = (128 + 32) as c_char;
                        }
                        if yj / 256 > destine[tribu as usize - 1].posy as c_ushort {
                            poschef[2] = (128 + 33) as c_char;
                        }
                        if yj / 256 < destine[tribu as usize - 1].posy as c_ushort {
                            poschef[2] = (128 + 34) as c_char;
                        }
                        bulle(addr_of!(poschef) as *mut c_char as Ptr, 1); /*affiche le commentaire*/
                    }
                }
            }
        } else
        /*stock1 <> 0*/
        {
            /*flagornerie ou insulte*/
            match stock1 {
                1 => {
                    /*insulte*/
                    excite((200 * stock2) as c_ushort);
                }
                2 => {
                    /*flagornerie*/
                    if agress < 200 {
                        agress += 1;
                        /*enervement diminue*/
                        tfr = (enerv as i32 - 100 * stock2 as i32) as c_short;
                        if (tfr as i32) < (enerv as i32) {
                            enerv = tfr as c_ushort;
                        }
                        excite(0);
                    } else {
                        /*vantardise*/
                        if agress < 200 && chef == 0
                        /*si trop agressif ça marche pas*/
                        {
                            agress += 1;
                            if Rnd() > (enerv / 200) as c_short {
                                fuite();
                            } else {
                                /*enervement diminue*/
                                tfr = (enerv as i32 - 100 * stock2 as i32) as c_short;
                                if (tfr as i32) < (enerv as i32) {
                                    enerv = tfr as c_ushort;
                                }
                                excite(0);
                            }
                        } else {
                            /*il le prend pour une insulte*/
                            excite((200 * stock2) as c_ushort);
                        }
                    }
                }
                _ => {
                    /*vantardise par defaut*/
                    if agress < 200 && chef == 0
                    /*si trop agressif ça marche pas*/
                    {
                        agress += 1;
                        if Rnd() > (enerv / 200) as c_short {
                            fuite();
                        } else {
                            /*enervement diminue*/
                            tfr = (enerv as i32 - 100 * stock2 as i32) as c_short;
                            if (tfr as i32) < (enerv as i32) {
                                enerv = tfr as c_ushort;
                            }
                            excite(0);
                        }
                    } else {
                        /*il le prend pour une insulte*/
                        excite((200 * stock2) as c_ushort);
                    }
                }
            } /*fin du switch*/
        }
        return;
    }
}

/***********************************************************************
                        VOLE:On a perdu: il nous pique tout
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn vole() {
    unsafe {
        let mut i: c_short;
        liste4.taille = 0; /*plus rien à terre*/
        i = liste1.taille as c_short;
        while i != 0 {
            i -= 1;
            /* si la valeur est suffisante que ce n'est pas un objet scenario
            et avec un leger Rnd
             */
            if intrsq(liste1.elements[i as usize] as c_ushort) > 5
                && liste1.elements[i as usize] < 40
                && Rnd() > 128
            {
                enlevl(&raw mut liste1, &raw mut listbidon, i as c_ushort);
            }
        }
        tlpos[1] = 1;
        tlpos[4] = 1;
        return;
    }
}

/***********************************************************************
                        OUTARM :Armes en sorties
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn outarm() {
    unsafe {
        let mut i: c_uchar;
        let tfr: c_uchar;

        tlpos[1] = 1;
        tlpos[4] = 1;

        i = liste1.taille;
        /*on enleve les sagaies de la liste en fonction de ce qui reste en sortie
        bien sur comme la taille va bouger on remet les positions ds les listes

        zéro*/
        while i != 0 {
            i -= 1;
            if liste1.elements[i as usize] == 23 {
                if sap1.nbresag != 0 {
                    sap1.nbresag -= 1;
                } else {
                    enlevl(&raw mut liste1, &raw mut listbidon, i as c_ushort);
                }
            }
        }

        if sap1.forcehache == 0
        /*pas de hache*/
        {
            i = liste1.taille;
            while i != 0 {
                i -= 1;
                if liste1.elements[i as usize] == 21 || liste1.elements[i as usize] == 22 {
                    enlevl(&raw mut liste1, &raw mut listbidon, i as c_ushort);
                    i = 0;
                }
            }
        }
        if sap1.vie != entvie
        /*il a encaissé*/
        {
            tfr = fatig.wrapping_sub(entvie.wrapping_sub(sap1.vie) / 2);
            if tfr < fatig {
                fatig = tfr;
            } else {
                fatig = 0;
            }
        }
        return;
    }
}

/***********************************************************************
                        TRDORS:
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trdors() {
    unsafe {
        let mut tfr: c_uchar;
        if fatig > 192 {
            flefba = 0;
            comment(108);
            return;
        }

        liste4.taille = 0; /*plus rien a terre*/
        tlpos[4] = 1;
        tfr = fatig.wrapping_add(stock2);

        if tfr < fatig {
            tfr = 255;
        }
        fatig = 255;
        dervue = VUE_PAYSAGE as u8;
        /*affiche ecran n fois */
        /*avec chronos*/
        stock2 = stock2 / 7;
        while stock2 != 0 {
            GereInterface();
            for _i in 0..9 {
                minute = 0;
                chrono(1);
            }
            let t = TickCount();
            while TickCount() < t + 10 { /* vide */ }
            affich(dzeta as c_ushort);
            tfrima();
            minute = 0;
            chrono(2);
            stock2 -= 1;
        }
        fatig = tfr;
        chrono(0);
        afstat();

        inimen(0);
        return;
    }
}

/**************************************************************************
                    InitVueLaterale
**************************************************************************/
unsafe fn InitVueLaterale() {
    if dervue != VUE_LATERALE as u8 {
        ldzeta = dzeta;
    }
    prm(&raw mut sap2, present[1] as c_short);
    prm(&raw mut sap4, present[3] as c_short);
    flcomb = 1;
    if presence != 0 {
        if presence < 5 {
            flcomb = 0;
        }
        presence &= 127; /*elimine bit 7 pour bestiole amie*/
        prm(&raw mut sap1, present[0] as c_short);
        prm(&raw mut sap3, present[2] as c_short);
        if dervue != VUE_LATERALE as u8 {
            latera(); /* un affichage et dialogue*/
        }
        dervue = VUE_LATERALE as u8;
    }
    pos0_mouse();
    return;
}

/***********************************************************************
                        TROBSV:Traite l'observation
=> 0 vue 3d
   1 vue laterale
   2 panorama
   3 carte
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn trobsv(regb: c_ushort) {
    unsafe {
        let dzetdep: c_ushort;
        let mut regb = regb;
        let mut posTimer: c_ulong;

        if presence != 0 {
            return; /* si il ya quelun alors on est poli*/
        }

        if regb == 0
        /* vue 4 D*/
        {
            LastObsv = 0;
            dismin = 0;
            lastmen = 255;
            clrbadi();
            dervue = VUE_PAYSAGE as u8;
            /* gestion de la vue 3d */
            MenuFenetres(FAUX as c_short);
            AffichePictAide(1);
            clav3d();
            MenuFenetres(VRAI as c_short);
            pos0_mouse();
            /* Une presence a ete rencontrée : on initialise la
            vue laterale */
            if presence != 0 {
                /* On initialise la vue laterale */
                regb = 1;
            }
        }
        if regb == 1
        /*vue laterale*/
        {
            LastObsv = 1;
            InitVueLaterale();
            return;
        }
        if regb == 2
        /*panaroma*/
        {
            while getkey(0) != 0 { /* vide */ }
            dervue = VUE_PAYSAGE as u8;
            dzetdep = dzeta as c_ushort;
            loop {
                posTimer = sapiensTimer;
                affich(dzeta as c_ushort);
                tfrima();
                dzeta = ((dzeta as i32 + 16) & 0x3FF) as c_short;
                while posTimer > sapiensTimer.wrapping_sub(8) { /* vide */ }
                if dzetdep as i32 == dzeta as i32 {
                    break;
                }
                if getkey(0) != 0 {
                    break;
                }
            }
        }
        if regb == 3
        /*carte*/
        {
            dervue = VUE_PAYSAGE as u8;
            gemouvre(clin, ccol, cnlin, cncol, 15, 30, 8, 15);
            zoocar();
            lastmen = 255;
        }
        inimen(0);
        pos0_mouse();
        efbadass();
        TfrBadAss();
        return;
    }
}

/***********************************************************************
        ZONEVT:Determine un evt en fonction de la position
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn zonevt(okmess: c_ushort) -> c_short {
    unsafe {
        let mut dieu: *mut evt;
        let mut nuit: bool;
        let mut qui: c_uchar;
        let mut dist: c_short;

        /*on balaye tout les evts*/
        for i in 0..NBREVTDESTINE as c_short {
            dieu = &raw mut destine[i as usize];
            dist = distance(
                (*dieu).posx as i64 * 256 - xj as i64,
                (*dieu).posy as i64 * 256 - yj as i64,
            );
            dist /= 256;
            /*dist est mis a jour*/
            if dist > (*dieu).distmax as c_short
            /*dans la zone d'influence*/
            {
                (*dieu).type_ &= 127; /*on revalide la zone en clearant le bit 128*/
            } else
            /*ok on est ds la zone*/
            {
                /*la zone est-elle valide ??,sommes mous assez central*/
                if (*dieu).type_ & 128 == 0 && (dist as i32 * 12 + 10) < (Rnd() as c_ushort) as i32
                {
                    /*on compare
                    la densité probabiliste de la zone*/
                    if Rnd() >= (*dieu).rnd as c_short {
                        /* on affiche un message (l'evt n'a???*/
                        if Rnd() > 240 {
                            if okmess != 0 {
                                efbadi();
                                comment((*dieu).message as c_ushort);
                                return 0;
                            }
                        }
                    } else {
                        if (*dieu).evt == 0
                        /*eau ??*/
                        {
                            source += 1;
                            liste4.taille = 1;
                            tlpos[4] = 1;
                            liste4.elements[0] = 2;
                            efbadi();
                            comment(76); /*tu entends une source*/
                            return 0;
                        } else {
                            /* est ce un chef ???*/
                            chef = if (*dieu).evt > 127 { 1 } else { 0 };
                            qui = (*dieu).evt & 127;
                            /* one shot !!!*/
                            if (*dieu).type_ != 0 {
                                (*dieu).type_ |= 128;
                            }
                            nuit = heure > 512;
                            /* test spécifiques a la presence et au moment de la journée*/
                            if chef != 0
                                || (nuit && qui != 5 && Rnd() > 222)
                                || (!nuit && qui == 5 && Rnd() > 222)
                                || (nuit && qui == 5)
                                || (!nuit && qui != 5)
                            {
                                return qui as c_short;
                            }
                        } /*evt =eau ou vivant*/
                    } /*Rnd correct*/
                } /*zone valide*/
            } /*dans la zone*/
        } /*pour tout les elements*/
        return 0;
    }
}

/***********************************************************************
                        ZONINF:Gestion des evts selon la position
***********************************************************************/
/* on travaille sur la position du joueur,en sortie les flags de presence
sont mis à jour
en sortie 0 aucune presence generee sinon type de la presence de 1 à 8
 */
#[unsafe(no_mangle)]
pub extern "C" fn zoninf() {
    unsafe {
        let tfr: c_uchar;
        let mut i: c_uchar;

        /*si dej
        present alors fin*/
        if presence != 0 {
            return;
        }
        /*la position est-elle differente*/
        if derxj == (xj & 0xFFE0) && deryj == (yj & 0xFFE0) {
            return;
        }
        /*on mémorise la derniere position*/
        derxj = xj & 0xFFE0;
        deryj = yj & 0xFFE0;
        chef = 0;
        flcomb = 0;
        source = 0;
        present[1] = zonevt(1) as c_char; /*avec message possible*/
        if present[1] != 0
        /*il ya un evt vivant*/
        {
            /* s'il ya un chef alors il est automatiquement accompagné*/
            /*sinon on retire un evt pour le deuxieme*/
            if chef == 0 {
                present[3] = zonevt(0) as c_char; /*sans message possible*/
            } else {
                present[3] = present[1];
            }
            /* si il y un homme il est en premier*/
            if (present[1] as u8) > 4 && (present[3] as u8) < 5 && present[3] != 0 {
                /*on swape*/
                tfr = present[1] as c_uchar;
                present[1] = present[3];
                present[3] = tfr as c_char;
            }
            /*ds tout les cas la presence est fixée a celle du prioritaire*/
            presence = present[1] as c_short;
            tribu = presence as c_uchar;
            if dervue != VUE_LATERALE as u8 {
                sap1.colonne = 3;
            }
            /* On calcule les positions des adversaires */
            prm(&raw mut sap2, present[1] as c_short);
            prm(&raw mut sap4, present[3] as c_short);
            idialo();
            /* on teste  le charisme immediat sur animal*/
            /* si on est seduisant,qu'il y a une seule bestiole et avec un peu de chance*/
            if presence > 4 && (charisme as c_short) > Rnd() && Rnd() < 40 && present[2] == 0 {
                flefba = 0;
                comment((97 + presence) as c_ushort); /*premier mess 102*/
                creecop(presence as c_ushort);
                viecopain = 1; /*pour les bestioles*/
                flefba = 0;
                if dervue == VUE_LATERALE as u8 {
                    flagbadi = 0;
                }
                presence = 128; /*bit 7 et presence 0*/
                return;
            }
        }
        if source == 0 {
            /*aucun evt n'est valide */
            if scenar == 4 && scenpr == 0 {
                /* A la bonne position on trouve la Hache Scrée des Hyenes folles */
                if ((if derxj > 0xABE0 {
                    derxj - 0xABE0
                } else {
                    0xABE0 - derxj
                }) < 500)
                    && ((if deryj > 0xB2E0 {
                        deryj - 0xB2E0
                    } else {
                        0xB2E0 - deryj
                    }) < 500)
                {
                    liste4.taille = 1;
                    tlpos[4] = 1;
                    liste4.elements[0] = 44;
                    return;
                }
            }
            liste4.taille = Rnd() as c_uchar & 7; /*jusqu'a 7 elements à terre*/
            i = liste4.taille;
            tlpos[4] = 1; /* debut de liste */
            /*on génère les objets a terre*/
            while i != 0 {
                i -= 1;
                liste4.elements[i as usize] = (3 + (17 * Rnd()) / 256) as c_uchar;
            }
        }
        return;
    }
}

/***********************************************************************
                        s_action:pour afstat
***********************************************************************/
use crate::cbridge::{strcat, strcpy, whiteColor};
use crate::sapiens::sap3d::{Final, affich, clav3d};
use crate::sapiens::saplist::{AffichePictAide, enlevl, gemouvre, inimen, pos0_mouse};
use crate::sapiens::sapw3d::{FixeDegradesFonctionHeure, distance};
use crate::sapiens::sapwcar::{
    GetStrBkMot, Gotoxy, GotoxyPixel, TailleCar, affcar10, affcarmess, car2,
};
use crate::sapiens::sapwcart::zoocar;
use crate::sapiens::sapwclav::{getkey, reset_mouse, sapiensTimer, vq_mouse};
use crate::sapiens::sapwmess::{TfrBadAss, efbadass, scrollbadass};
use crate::sapiens::{NBREVTDESTINE, NBREVTHEURE, VUE_AUCUNE, VUE_LATERALE, VUE_PAYSAGE, evt};
use crate::shared::{CGrafPtr, Ptr, Rect, RgnHandle};
