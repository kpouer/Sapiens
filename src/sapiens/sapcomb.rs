/*********************************************************************
                Modul #3 .Sapcomb(Combats Multiples)
                    Le Jeudi 20 Novembre 1986
Le nombre d'herbe au premier plan est ajuste en fonction de la vitesse processeur
Cx :                 2619   0
PowerMac 6100/66 :   6584
Quadra :             10010  20
***********************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::DEBBEST;
use crate::DEBCRANE;
use crate::DEBTETE;
use crate::MOB_ANIMAL_MORT;
use crate::MOB_CHEMIN;
use crate::MOB_CORPS;
use crate::MOB_EMPREINTE;
use crate::MOB_FLECHE;
use crate::MOB_HERBE;
use crate::MOB_IMPACT;
use crate::MOB_MORT;
use crate::MOB_SANG_A_TERRE;
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, FAUX, InsetRect, OffsetRect, PtInRect, SetRect,
    VRAI,
};
use crate::cbridge::{geneva, teJustLeft};
use crate::sapiens::sap3d::affich;
use crate::sapiens::sapglob::{
    agilite, agress, aneutr, anlap1, anlap2, anlap3, anlou1, anlou2, bestagress, besthaut, bestlin,
    bestvie, bless, bouton, charisme, chef, cptagil, debban, dervue, dismin, distsp, dsposx,
    dsposy, dzeta, echan, entvie, faim, fdialo, fin, flcomb, flefba, force, hautlin, hautou,
    jour_dsk, ldzeta, liste1, liste4, minute, mort, param, peuple, presence, present, profond,
    ptranim, qtviande, resistance, s_action, sap1, sap2, sap3, sap3clav, sap4, scenar, scenpr,
    sensmob, soif, tabinp, tribforce, tribu, tribvie, vie, viecopain, who, xj, yj,
};
use crate::sapiens::saplist::{InverseOption, ajoutl, inimen};
use crate::sapiens::saprel::{
    afstat, chrono, clrbadi, comment, outarm, sceveu, tfrene, vole, zoninf,
};
use crate::sapiens::saptxt::{nomcopain, nomjoueur};
use crate::sapiens::sapw3d::sapdep;
use crate::sapiens::sapwarb::arbre;
use crate::sapiens::sapwbrt::{
    AffDebecr, Affmob, AffmobConst, AffmobPalette, CalculeAireMob, CalculePaletteAttenuee,
    CopyEcran1Sur2, DoEraseRect, DoFrameRect, DoTextBox, DoTextSize, TfrEcr, affomb, effdemi,
    tfrdemi, tfrima, tfrvirt,
};
use crate::sapiens::sapwclav::{
    AttendsFinBouton, getkey, reset_clav, reset_mouse, sapiensTimer, vq_mouse,
};
use crate::sapiens::sapwind::{
    Ecran1, GereInterface, GetStr, Rnd, debecr, delai, initrnd, mobAvplanSpec, mobFace, myRandom,
    rnd0,
};
use crate::sapiens::sapwmus::{attends_bruit, bruit};
use crate::sapiens::sapwpref::JOUEUR2_AUTONOME;
use crate::sapiens::{
    ECHELLE, FORME_ARME, FORME_BUSTE, FORME_TETE, NB_ARBRES_LATERAL, NB_ARBRES_LATERAL_MAX,
    NB_HERBE_ARR_PLAN, NB_MAX_HERBE_AR_PLAN, NB_MAX_HERBE_AV_PLAN, SAGAIE, VUE_AUCUNE,
    VUE_LATERALE, brin, sap,
};
use crate::shared::{Boolean, CGrafPtr, Point, Rect};
use crate::text::TextFont;
use std::ffi::CStr;
use std::os::raw::{c_char, c_long, c_short, c_uchar, c_ushort};
use std::ptr::addr_of_mut;

pub static mut nbMobHerbes: c_short = 0;
pub static mut nbAvplanSpec: c_short = 0;
const LIGNE_BASE_HERBE_AV_PLAN: c_short = 63;
const LIGNE_MAX_HERBE_AV_PLAN: c_short = 70;
static mut NbHerbeAvPlan: c_short = 0;
static mut herbe: [brin; NB_MAX_HERBE_AV_PLAN as usize] = [brin {
    type_: 0,
    lin: 0,
    col: 0,
    sens: 0,
}; NB_MAX_HERBE_AV_PLAN as usize];
const LIMITE_GAUCHE: c_short = -5;
const LIMITE_DROITE: c_short = 62;
pub static mut nbHerbes: c_long = 0;
pub static mut ModeJoueur2: c_short = JOUEUR2_AUTONOME as c_short;

/***********************************************************************
                DECSOU:Essoufflement
***********************************************************************/
pub extern "C" fn decsou(s: *mut sap, combien: c_ushort) {
    unsafe {
        let tfr = (*s).souffle.wrapping_sub(combien as u8);
        if tfr < (*s).souffle {
            (*s).souffle = tfr; /* ok souffle diminue */
        } else {
            decvie(s, 1); /*plus de souffle puff puff ...*/
        }
    }
}

/***********************************************************************
                INCSOU:Recupere du souffle
***********************************************************************/
pub extern "C" fn incsou(s: *mut sap, combien: c_ushort) {
    unsafe {
        let tfr = (*s).souffle.wrapping_add(combien as u8);
        if tfr > (*s).souffle {
            (*s).souffle = tfr;
        } else {
            (*s).souffle = 255;
        }
    }
}

/***********************************************************************
                POSAN:
***********************************************************************/
pub extern "C" fn posan(sapiens: *mut sap, mut anim: ::std::os::raw::c_short) {
    anim -= 1;
    for i in 0..14 {
        unsafe {
            if debban[anim as usize][i] as u8 > 0 {
                (*sapiens).sapanim[i] = debban[anim as usize][i];
            }
        }
    }
}

/***********************************************************************
                ARRET:
***********************************************************************/
pub extern "C" fn arret(sapiens: *mut sap) {
    unsafe {
        if ((*sapiens).vitesse & 2) == 0
        /*pas arret*/
        {
            (*sapiens).vitesse = ((*sapiens).vitesse ^ 3) & 3;
        }
    }
}

/***********************************************************************
                        POSVIT:Positionne vitesse
***********************************************************************/
pub extern "C" fn posvit(sapiens: *mut sap) {
    unsafe {
        if (*sapiens).hauteur == 0
        /*debout*/
        {
            (*sapiens).sens = ((((*sapiens).vitesse - 1) & 2) / 2) as c_uchar;
            if ((*sapiens).vitesse & 2) != 0 {
                posan(sapiens, 2); /*arret*/
            } else {
                posan(sapiens, 1);
            }
            posan(sapiens, (*sapiens).objet as c_short + 6);
            return;
        }
        if (*sapiens).hauteur != 1 {
            arret(sapiens);
            (*sapiens).sens = ((((*sapiens).vitesse - 1) & 2) / 2) as c_uchar;
            return;
        }
        if (*sapiens).action < 128 {
            (*sapiens).action |= 128;
            if sapiens == addr_of_mut!(sap1) {
                decsou(sapiens, 10);
            }
            posan(
                sapiens,
                ((((*sapiens).vitesse - 1) / 2) as c_short ^ (*sapiens).sens as c_short) + 12,
            );
            (*sapiens).vitesse = (*sapiens).sens + 2;
        }
        arret(sapiens);
        return;
    }
}

/***********************************************************************
                POGNE:Saisi objet à la main
***********************************************************************/
pub extern "C" fn pogne(sapiens: *mut sap) {
    unsafe {
        let mut objet = (*sapiens).objet + 1;
        if objet > 2 {
            objet = 0;
        }
        (*sapiens).objet = objet;
        /*si objet est la sagaie et (ou trop pres ou plus de sagaie*/
        if (objet == 2 && ((*sapiens).nbresag == 0 || ((*sapiens).action & 3) != 0))
            || (objet == 1 && (*sapiens).forcehache == 0)
        {
            pogne(sapiens); /*recursif*/
        } else {
            posan(sapiens, objet as c_short + 6); /*positionne animation*/
        }
    }
}

/***********************************************************************
                        JUGE:Etablit les adversaires
**********************************************************************
Les adversaires sont positionnes selon les presences et en sortie:
0:Tout le clan 2 est elimine
1:Tout le clan 1 est elimine
2:Il en reste des deux clans
*/
pub extern "C" fn juge() -> c_short {
    unsafe {
        let mut nbre: c_char = 0;
        /* ----->Mise en place des plans */
        for i in 0..4 {
            if (*peuple[i]).vie != 0 {
                nbre += 1;
            }
        }
        /*----->nbre est le nombre d'acteurs*/
        /*------>si 4 alors deux couples sur deux plans*/
        if nbre == 4 {
            profond[2] = 7;
            profond[3] = 7;
        }
        /*sinon tout sur le premier plan*/
        else {
            profond[2] = 10;
            profond[3] = 10;
        }
        /*dans tout les cas*/
        profond[1] = 10;
        profond[0] = 10;
        /*si reste seulement clan1 alors sur deux plans*/
        if nbre == 2 && sap1.vie != 0 && sap3.vie != 0 {
            profond[2] = 7;
        }
        /*On positionne l'adversaire de l'ami*/
        if sap4.vie != 0 {
            sap3.adversaire = addr_of_mut!(sap4);
        } else {
            sap3.adversaire = addr_of_mut!(sap2);
        }
        /*On teste si un des deux clans est vide*/
        if sap1.vie + sap3.vie == 0 {
            return 1; /*clan 1 kaput*/
        }
        if sap2.vie + sap4.vie == 0 {
            return 0; /*clan 2 kaput*/
        }
        /* Joueur toujours present ???*/
        if sap1.vie != 0 {
            if sap3.vie != 0
            /*il a de l'aide ????*/
            {
                /*oui il a de l'aide*/
                sap2.adversaire = addr_of_mut!(sap1);
                sap4.adversaire = addr_of_mut!(sap3);
            } else {
                /*non il est seul */
                sap2.adversaire = addr_of_mut!(sap1);
                sap4.adversaire = addr_of_mut!(sap1);
            }
        } else {
            /*l'aide est seule */
            sap2.adversaire = addr_of_mut!(sap3);
            sap4.adversaire = addr_of_mut!(sap3);
        }
        2
    }
}
/**********************************************************************
                        InitGenerique
***********************************************************************/
pub extern "C" fn InitGenerique() {
    unsafe {
        let mut gbak: CGrafPtr = core::ptr::null_mut();
        AcamGetPortOffscreen(&raw mut gbak);
        AcamSetPortOffscreen(debecr);
        /* on balaye les quatre sapiens*/
        for i in 0..4 {
            let l = (*peuple[i]).poslcoeur as c_short;
            let c = (*peuple[i]).posccoeur as c_short;
            affomb(mobFace + 31, l - 2, c - 2, 4, 4, FAUX);
            affomb(mobFace + 30, l - 2 + 22, c - 2, 4, 4, FAUX);
            let mut r: Rect = Rect::default();
            SetRect(
                &raw mut r,
                (c - 2) * ECHELLE as c_short,
                (l - 2) * ECHELLE as c_short,
                (c + 16 + 2) * ECHELLE as c_short,
                (l + 16 + 2) * ECHELLE as c_short,
            );
            InsetRect(&raw mut r, -1, -1);
            DoFrameRect(&raw mut r);
            OffsetRect(&raw mut r, 0, 22 * ECHELLE as c_short);
            DoFrameRect(&raw mut r);
        }
        AcamSetPortOffscreen(gbak);
    }
}

/**********************************************************************
                        EffaceGenerique
***********************************************************************/
fn EffaceGenerique() {
    unsafe {
        /* on balaye les quatre sapiens*/
        for i in 0..4 {
            let _l = (*peuple[i]).poslcoeur;
            let _c = (*peuple[i]).posccoeur;
            /*    AffDebecr(mobFace+31,l-2,c-2,FAUX);*/
            /*    AffDebecr(mobFace+30,l-2+22,c-2,FAUX);*/
        }
    }
}

/**********************************************************************
                        GENERIQUE:
***********************************************************************/
pub extern "C" fn generique(trace: Boolean) {
    unsafe {
        /* on balaye les quatre sapiens*/
        for i in 0..4 {
            let mut l = (*peuple[i]).poslcoeur as c_short;
            let c = (*peuple[i]).posccoeur as c_short;
            let f = (*peuple[i]).cross[FORME_TETE as usize] as c_short / 2;
            /* fond vide */
            /* Mort => une croix */
            if (*peuple[i]).vie == 0 {
                AffDebecr(mobFace + 31, l - 2, c - 2, trace as c_short); /* rien*/
                affomb(DEBCRANE!() + 10, l, c, 2, 2, trace); /* croix */
                affomb(DEBCRANE!() + 10, l + 22, c, 2, 2, trace); /* croix */
            } else {
                /* Affichage de l'etat de la sante */
                AffDebecr(mobFace + 31, l - 2, c - 2, trace as c_short); /* rien*/
                /* Ce n'est pas un animal */
                if present[i] == 0 {
                    AffDebecr(DEBCRANE!() + 8, l, c, trace as c_short);
                }
                /* Affichage de la tete */
                l += 22;
                AffDebecr(mobFace + 30, l - 2, c - 2, trace as c_short); /* rien*/
                if present[i] != 0 && (present[i] as c_short) < 5 {
                    AffDebecr(DEBTETE!() + f, l, c, trace as c_short);
                } else {
                    AffDebecr(
                        DEBBEST!() + present[i] as c_short - 5,
                        l,
                        c,
                        trace as c_short,
                    );
                }
            }
        }
    }
}

/**********************************************************************
                        AffStrVolatile
L'utilisateur a cliqué dans la zone haute : on regarde si le click a eu lieu
dans une case du génerique
***********************************************************************/
fn AffStrVolatile(pt: Point, chaine: *mut c_char) {
    unsafe {
        if !chaine.is_null() && CStr::from_ptr(chaine).to_bytes().len() != 0 {
            let mut raff: Rect = Rect::default();
            SetRect(&raw mut raff, pt.h - 8, pt.v - 8, pt.h + 100, pt.v + 40);
            if raff.left < 0 {
                OffsetRect(&raw mut raff, -raff.left, 0);
            }
            if raff.right > 640 {
                OffsetRect(&raw mut raff, 640 - raff.right, 0);
            }
            DoEraseRect(&raw mut raff);
            InsetRect(&raw mut raff, 4, 4);
            TextFont(geneva as c_short);
            DoTextSize(9);
            DoTextBox(
                chaine,
                CStr::from_ptr(chaine).to_bytes().len() as c_short,
                &raw mut raff,
                teJustLeft as c_short,
            );
            InsetRect(&raw mut raff, -4, -4);
            DoFrameRect(&raw mut raff);
            delai(60);
            AttendsFinBouton();
            TfrEcr(&raw mut raff);
        }
    }
}

/**********************************************************************
                        ClickInGenerique
L'utilisateur a clique dans la zone haute : on regarde si le click a eu lieu
dans une case du generique
***********************************************************************/
pub extern "C" fn ClickInGenerique(pt: Point) {
    unsafe {
        /* on balaye les quatre sapiens*/
        for i in 0..4 {
            let mut r: Rect = Rect::default();
            /* Dans le symbole du coeur */
            SetRect(
                &raw mut r,
                (*peuple[i]).posccoeur as c_short * ECHELLE as c_short - 4,
                (*peuple[i]).poslcoeur as c_short * ECHELLE as c_short - 4,
                (*peuple[i]).posccoeur as c_short * ECHELLE as c_short + 36,
                (*peuple[i]).poslcoeur as c_short * ECHELLE as c_short + 36,
            );
            if PtInRect(pt, &raw mut r) {
                let mut chaine: [c_char; 255] = [0; 255];
                if (*peuple[i]).vie <= 0 {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(19));
                } else if i == 0 {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(27));
                } else if i == 2 {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(29));
                } else {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(28));
                }
                AffStrVolatile(pt, &raw mut chaine as *mut c_char);
            }
            /* Dans le cadre de la tete */
            OffsetRect(&raw mut r, 0, 44);
            if PtInRect(pt, &raw mut r) {
                let mut chaine: [c_char; 255] = [0; 255];
                if (*peuple[i]).vie <= 0 {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(19));
                } else if i == 0 {
                    sprintf_nom(
                        &raw mut chaine as *mut c_char,
                        GetStr(15),
                        &raw const nomjoueur as *mut c_char,
                    );
                } else if i == 2 {
                    sprintf_nom(
                        &raw mut chaine as *mut c_char,
                        GetStr(16),
                        &raw const nomcopain as *mut c_char,
                    );
                } else if i == 1 {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(17));
                } else {
                    copie_chaine(&raw mut chaine as *mut c_char, GetStr(18));
                }
                AffStrVolatile(pt, &raw mut chaine as *mut c_char);
            }
        }
    }
}

/***********************************************************************
                        ABANDO:Laisse objet qd mort
***********************************************************************/
pub extern "C" fn abando(s: *mut sap) {
    unsafe {
        if (*s).animal != 0
        /*c'est une bestiole*/
        {
            for _ in 0..qtviande[(*s).animal as usize - 1] {
                ajoutl(&raw mut liste4, 37);
            }
            match (*s).animal {
                1 => {
                    /*lapin*/
                    ajoutl(&raw mut liste4, 33); /*peau de lapin*/
                }
                3 => {
                    /*loup*/
                    ajoutl(&raw mut liste4, 35);
                    ajoutl(&raw mut liste4, 32);
                }
                4 => {
                    /*ours*/
                    ajoutl(&raw mut liste4, 34);
                    ajoutl(&raw mut liste4, 31);
                }
                _ => {}
            }
        } else {
            /*homme*/
            if echan > 2
            /* pas ma protection ou rien */
            {
                ajoutl(&raw mut liste4, echan as c_ushort);
            }
            let mut i = (*(*s).possession).taille;
            while i != 0 {
                i -= 1;
                ajoutl(
                    &raw mut liste4,
                    (*(*s).possession).elements[i as usize] as c_ushort,
                );
            }
            /*on quitte pour les autres morts*/
            (*(*s).possession).taille = 0;
            echan = 0;
        }
    }
}

/***********************************************************************
                        DECVIE
***********************************************************************/
pub fn decvie(s: *mut sap, valeur: ::std::os::raw::c_ushort) {
    unsafe {
        if (*s).vie == 0 {
            return;
        }
        let tfr = (((*s).vie as i32) - (valeur as i32)) as std::os::raw::c_short;
        if tfr > 0
        /*si = 0 alors mort*/
        {
            /*ok pas mort*/
            (*s).vie = tfr as c_uchar;
            (*s).coeur = DEBCRANE!() + (*s).vie as c_short / 32;
            (*s).cptrsouffle = 1;
            /* On va tester si on frappe notre copain */
            if (s == addr_of_mut!(sap3)) && (presence <= 0) && valeur > 0 {
                /*initialise le sap3 contre nous*/
                presence = -1;
                sap3clav = 0;
                (*s).adversaire = addr_of_mut!(sap1);
                viecopain = 0;
            }
        } else {
            /*ok,mort*/
            (*s).vie = 0;
            if (*s).animal <= 0 {
                (*s).flg_mort = 16;
                if (*s).cross[FORME_BUSTE as usize] < 8 {
                    bruit(11);
                } else {
                    bruit(12); /*argh*/
                }
            } else {
                (*s).flg_mort = 1;
            }
        }
    }
}

/******************************************************************
                    GER_MORT:Traite la mort
*******************************************************************/
fn ger_mort(s: *mut sap) {
    unsafe {
        /* Si il peut le plancher l'appelle */
        if (*s).flg_mort == 0 {
            return;
        }
        (*s).flg_mort -= 1;
        if ((*s).flg_mort & 2) == 0 && (*s).animal == 0 {
            baisse(s);
            if (*s).hauteur >= 3 {
                (*s).flg_mort = 0;
            }
        }
        /* on a atteint le sol */
        if (*s).flg_mort == 0 {
            if presence == -1
            /* on va baiser Juge*/
            {
                sap2.vie = sap3.vie;
                sap3.vie = 0;
            }
            if s == addr_of_mut!(sap3) {
                if presence == -1
                /*c'est moi qui le frappe */
                {
                    viecopain = 0;
                    chef = 0; /*On met le chef a zéro pour le scenar 4*/
                    if charisme > 10 {
                        charisme -= 10;
                        param[2] = param[2].wrapping_sub(10);
                    }
                } else {
                    viecopain -= 1;
                }
                flefba = 0;
                if viecopain == 0 {
                    present[2] = 0; /*degenere*/
                    comment(112); /*Ton ami est mort*/
                } else {
                    comment(111); /*Il est Evanoui*/
                }
            }
            if presence as c_short != juge() {
                fin = (juge() + 1) as c_uchar;
            }
            if s != addr_of_mut!(sap1) {
                abando(s); /*abandonne objets*/
            }
            /* Il est evanoui on l'affiche a terre*/
            if (*s).animal == 0
            /*c'est un homme*/
            {
                if (*s).colonne > LIMITE_DROITE {
                    (*s).colonne = 1;
                }
                if (*s).sens != 0 {
                    (*s).colonne += 8; /* pour centrer le mort */
                }
                AffmobConst(
                    MOB_MORT!() + (*s).cross[FORME_TETE as usize] as c_short / 2,
                    100 - 34 + profond[who as usize] as c_short,
                    4 * ((*s).colonne + 8 * ((*s).colonne < 8) as c_short),
                    (*s).sens as c_short,
                );
            } else {
                AffmobConst(
                    MOB_ANIMAL_MORT!() + (*s).animal as c_short - 1,
                    100 - 20 + profond[4] as c_short - 8 * ((*s).animal > 2) as c_short,
                    4 * (*s).colonne,
                    (*s).sens as c_short,
                );
            }
            /*on me met hors ecran*/
            (*s).colonne = 200;
            generique(VRAI);
        }
    }
}

/***********************************************************************
                        TOUCHESAG:Teste si Sagaie Touche
***********************************************************************/
/*En entree le Sap qui lance et celui qui est vise
  en sortie la sagaie est positionnee
*/
pub extern "C" fn touchesag(s1: *mut sap, s2: *mut sap) {
    unsafe {
        let mut test: c_short;
        /*si l'adversaire est absent sa colonne >64*/
        if (*s2).colonne < 200 && (*s1).snssag < 128 {
            /*calcul de la colonne du test*/
            test = (*s1).colsag - (*s2).colonne;
            if test < 0 {
                test = -test;
            }
            if test < 3 && (*s1).linsag > hautou[(*s2).hauteur as usize] as c_short {
                /*ok on touche*/
                (*s2).blockimpact = 4;
                bruit(1); /* La sagaie touche */
                let mut tfr = (*s1).linsag;
                if tfr > 30 {
                    tfr = 30; /*pas trop bas*/
                }
                (*s2).ligneimpact = tfr as c_uchar;
                (*s2).colimpact = ((*s1).colsag + 6 * ((*s1).snssag == 0) as c_short) as c_uchar;
                decvie(s2, 30);
                AffmobConst(
                    MOB_FLECHE!(),
                    100 - 23 + profond[who as usize] as c_short,
                    4 * (*s1).colsag,
                    (*s1).snssag as c_short,
                ); /*a terre*/
                ajoutl(&raw mut liste4, 45);
                (*s1).snssag = 255; /*degenere sagaie*/
            } /*si touche*/
        } /*si presence adversaire*/
    }
}

/***********************************************************************
                        GERSAG
********************************************************************/
pub extern "C" fn gersag(s1: *mut sap) {
    unsafe {
        if (*s1).snssag < 128
        /*sagaie lancee !!!!!*/
        {
            /*si la sagaie est la on l'affiche*/
            let tfr: c_short;
            if (*s1).snssag != 0 {
                tfr = (*s1).colsag + 3;
            } else {
                tfr = (*s1).colsag - 3;
            }
            if tfr < 0 || tfr > 63 {
                (*s1).snssag = 255; /*degenere*/
            } else {
                (*s1).colsag = tfr;
                (*s1).linsag += 1;
                if (*s1).linsag > 45 {
                    AffmobConst(
                        MOB_FLECHE!(),
                        100 - 23 + profond[who as usize] as c_short,
                        4 * (*s1).colsag,
                        (*s1).snssag as c_short,
                    ); /*a terre*/
                    if Rnd() > 200 {
                        ajoutl(&raw mut liste4, SAGAIE as c_ushort);
                    } else {
                        ajoutl(&raw mut liste4, 45);
                    }
                    (*s1).snssag = 255; /*degenere*/
                } else {
                    Affmob(
                        MOB_FLECHE!(),
                        (*s1).linsag + profond[who as usize] as c_short + 32,
                        4 * (*s1).colsag,
                        (*s1).snssag as c_short,
                    );
                }
            }
            /* Si c'est le Joueur il peut toucher les 3 autres */
            if who == 0 {
                touchesag(s1, addr_of_mut!(sap2));
                touchesag(s1, addr_of_mut!(sap3));
                touchesag(s1, addr_of_mut!(sap4));
            } else
            /*Si ce n'est pas le joueur il ne vise qu'une personne en particulier*/
            {
                touchesag(s1, (*s1).adversaire);
            }
        } /*sagaie existe ?*/
    }
}

/***********************************************************************
                          SAUT:
***********************************************************************/
pub extern "C" fn saut(sapiens: *mut sap) {
    unsafe {
        if (*sapiens).hauteur == 0 || ((*sapiens).action & 128) != 0 {
            return;
        }
        (*sapiens).hauteur -= 1;
        (*sapiens).action = 0;
        if (*sapiens).hauteur == 0 {
            posvit(sapiens);
            return;
        }
        posan(sapiens, (*sapiens).hauteur as c_short + 8);
        arret(sapiens);
        posan(sapiens, (*sapiens).objet as c_short + 6);
    }
}

/***********************************************************************
                TOUCHE:
***********************************************************************/
/* en entree le sap qui frappe et celui qui encaisse*/
pub extern "C" fn touche(s: *mut sap, s2: *mut sap) /*s frappe s2 encaisse*/
{
    unsafe {
        let coltest: c_short;
        let mut dommage: c_short;
        let mut tfr: c_short;
        /*calcul de la position du test*/
        if (*s).sens != 0 {
            coltest = (*s).colonne + *ptranim as c_short;
        } else {
            coltest = (*s).colonne - *ptranim as c_short;
        }
        /*quel est l'Adversaire*/
        /*valeur absolue de la distance*/
        tfr = coltest - (*s2).colonne;
        if tfr < 0 {
            tfr = -tfr;
        }
        /*le frappe-t'on alors qu'il est assis ?*/
        if tfr > 3 || (*s2).hauteur as c_short > (*s).hauteur as c_short + 1 {
            return;
        }
        /*positionne l'impact*/
        (*s).blockimpact = 4;
        (*s).colimpact = ((*s2).colonne
            + if (*s2).sens != 0 { -3 } else { 2 }
            + 6 * ((*s).sens != 0) as c_short) as c_uchar;
        if (*s).colimpact > 128 {
            (*s).colimpact = 0;
        } else if (*s).colimpact as c_short > LIMITE_DROITE + 3 {
            (*s).colimpact = (LIMITE_DROITE + 3) as c_uchar;
        }
        (*s).ligneimpact = tabinp[(*s).hauteur as usize] as c_uchar;
        if ((*s).hauteur as c_short / 2 == (*s2).hauteur as c_short / 2)
            && ((*s).sens != (*s2).sens)
            && ((*s2).action & 3) != 0
        {
            /*meme hauteur sens oppose et frappent tout deux*/
            (*s).blockimpact = 2;
            if Rnd() < 10 {
                /*ok destroy*/
                bruit(4);
                if (*s2).objet == 1 {
                    /*destroy hache du sap2*/
                    (*s2).forcehache = 0;
                    pogne(s2);
                }
                if (*s).objet == 1 {
                    (*s).forcehache = 0;
                    pogne(s);
                }
            }
            /*Rnd insuffisant*/
            else {
                bruit(2);
            }
        }
        /*hache se touchent*/
        else {
            /*il recule*/
            if (*s).sens != 0 {
                if (*s2).colonne < LIMITE_DROITE - 3 {
                    (*s2).colonne += 3;
                }
            } else {
                if (*s2).colonne > LIMITE_GAUCHE + 3 {
                    (*s2).colonne -= 3;
                }
            }
            dommage = 5;
            if (*s).objet == 1 {
                dommage = (*s).forcehache as c_short;
            }
            if (*s).hauteur == 0 {
                dommage *= 2;
            }
            if s2 == addr_of_mut!(sap1) {
                dommage -= resistance as c_short / 50;
            }
            decvie(s2, dommage as c_ushort);
            if (*s).animal != 0 {
                if Rnd() > 200 {
                    bruit(8);
                }
            } else {
                bruit(0);
            }
        }
    }
}

/***********************************************************************
                GCOEUR:
***********************************************************************/
fn gcoeur(s: *mut sap) {
    unsafe {
        let mut quoi: c_short = 0;
        if (*s).cptrsouffle < 32 {
            (*s).cptrsouffle = (*s).souffle as c_short;
            quoi = (*s).coeur;
            if quoi < 0 {
                quoi = 1 - quoi;
            }
            affomb(
                quoi,
                (*s).poslcoeur as c_short,
                (*s).posccoeur as c_short,
                0,
                0,
                VRAI,
            );
            (*s).coeur = -(*s).coeur;
        } else {
            (*s).cptrsouffle -= 32;
        }
    }
}

/***********************************************************************
                        IMPAK:Gere les impacts
***********************************************************************/
fn impak(s: *mut sap) {
    unsafe {
        if (*s).blockimpact != 0 {
            Affmob(
                (*s).blockimpact as c_short + MOB_IMPACT!(),
                (*s).ligneimpact as c_short + profond[who as usize] as c_short + 32,
                4 * (*s).colimpact as c_short,
                (*s).sens as c_short,
            );
            (*s).blockimpact -= 1;
            /* sang par terre et dec impact */
            if (*s).blockimpact == 0 {
                AffmobConst(
                    MOB_SANG_A_TERRE!(),
                    100 + profond[who as usize] as c_short - 19 - myRandom(0, 4) as c_short,
                    4 * (*s).colimpact as c_short,
                    (*s).sens as c_short,
                );
            }
        }
    }
}

/***********************************************************************
                        AVASAP:
        (c) God! Departement Intelligence Artificielle
                Avril '86-   Décembre '86
***********************************************************************/
fn avasap(s: *mut sap) {
    unsafe {
        if (*s).sens == 0 {
            decvit(s);
        } else {
            incvit(s);
        }
    }
}

/***********************************************************************
                        RECUSAP:
***********************************************************************/
fn recusap(s: *mut sap) {
    unsafe {
        if (*s).hauteur == 1 && ((*s).action & 128) == 0 {
            if (*s).sens == 0 {
                incvit(s);
            } else {
                decvit(s);
            }
        }
    }
}

/***********************************************************************
                        FIXCOLON
***********************************************************************/
pub extern "C" fn fixcolon(s: *mut sap) {
    /* si ami alors meme place*/
    unsafe {
        if s == addr_of_mut!(sap3) {
            (*s).colonne = sap1.colonne;
            (*s).sens = sap1.sens;
        } else {
            if (s == addr_of_mut!(sap1)) && ((*s).colonne > 64) {
                (*s).colonne = 2;
                (*s).sens = 1;
                return;
            }
            if (*s).colonne > 32 {
                if s != addr_of_mut!(sap1) {
                    (*s).sens = 0;
                    (*s).colonne = 53 + who as c_short;
                } else {
                    (*s).colonne = 55;
                }
            } else {
                if s != addr_of_mut!(sap1) {
                    (*s).sens = 1;
                    (*s).colonne = 5 - who as c_short;
                } else {
                    (*s).colonne = 1;
                }
            }
        }
    }
}
/***********************************************************************
                        INIUNE:Inits primaires de vue lat
***********************************************************************/
pub fn iniune(s: *mut sap) {
    unsafe {
        (*s).sapanim[10] = addr_of_mut!(aneutr) as *mut std::os::raw::c_char; /*supprime arme*/
        (*s).sapanim[11] = addr_of_mut!(aneutr) as *mut std::os::raw::c_char;
        (*s).action = 0;
        (*s).ligne = 2;
        (*s).hauteur = 0;

        (*s).objet = 0;
        posan(s, 2); /*arret par defaut*/
        if (*s).colonne > 32 {
            (*s).sens = 0;
        } else {
            (*s).sens = 1;
        }
        fixcolon(s);
        (*s).vitesse = (*s).sens | 2; /*arret*/
        (*s).animal = 0;
        (*s).deltah = 0;
        (*s).souffle = 200;
        (*s).cptrsouffle = 1;
        return;
    }
}

/***********************************************************************
                PRM:Initialise Rencontres
***********************************************************************
=> Type de la pr�sence,(chef):
0 non present;
1 Pied agile,2 Yeux malins,3 Bras de roc,4 Hyenes folles
5 Loup,6 ours,7 Lapin,8 Lagopede,0 rien ,-1 Joueur
*/
pub extern "C" fn prm(s: *mut sap, type_: ::std::os::raw::c_short) {
    let mut i;

    unsafe {
        (*s).snssag = 255;
        (*s).fuite = 0;
        (*s).blockimpact = 0;
        (*s).flg_mort = 0;
        /*On va redeterminer who pour iniune*/
        if s == addr_of_mut!(sap1) {
            who = 0;
        }
        if s == addr_of_mut!(sap2) {
            who = 1;
        }
        if s == addr_of_mut!(sap3) {
            who = 2;
        }
        if s == addr_of_mut!(sap4) {
            who = 3;
        }
        present[who as usize] = type_ as c_char;
        if type_ == 0 {
            /*non present*/
            (*s).colonne = 200;
            (*s).vie = 0;
            return;
        }
        if type_ < 0 || (who == 2 && type_ < 5)
        /*joueur ou aide humaine*/
        {
            if type_ == -1 {
                sap1.vie = ((vie as u32 * 50) / 256
                    + (bless as u32 * 93) / 256
                    + (faim as u32 * 46) / 256
                    + (soif as u32 * 46) / 256) as c_uchar;
            } else {
                (*s).vie = (viecopain as u16 * 60 + 20) as c_uchar;
            }
            if type_ < 0 || type_ >= 4 {
                // Note : bug dans le code original
                (*s).forcehache = 0;
            } else {
                (*s).forcehache = tribforce[type_ as usize - 1] as c_uchar;
            }
            iniune(s);
            decvie(s, 0); /* met en place coeur*/
            return;
        }
        /*raz du cross du sap*/
        for i in 0..7 {
            (*s).cross[i] = 0;
        }

        if type_ < 5
        /*homme*/
        {
            loop {
                (*s).cross[FORME_BUSTE as usize] = (2 * myRandom(0, 4)) as c_uchar;
                if chef > 0 && s == addr_of_mut!(sap2) {
                    i = 8; /*gueule de chef*/
                    (*s).cross[FORME_BUSTE as usize] = 0;
                } else {
                    if Rnd() < 100 {
                        /*femelles*/
                        i = 12 + (Rnd() / 43) * 2; /*selectionne tete*/
                        (*s).cross[FORME_BUSTE as usize] += 8; /*buste*/
                    } else {
                        /*male*/
                        i = 2 * (Rnd() & 3); /*gueule*/
                    }
                }
                if type_ == 4
                /*hyene folle*/
                {
                    i = 10; /*masque hyene folle*/
                }
                if i != sap1.cross[FORME_TETE as usize] as i16 {
                    break;
                }
            }

            (*s).cross[FORME_TETE as usize] = i as c_uchar; /*tete*/
            /*masse ou hache*/
            (*s).cross[FORME_ARME as usize] = (Rnd() & 1) as c_uchar;
        }

        /*si homme ou bestiole*/

        /*colonne sap2*/
        if sap1.colonne > 32 {
            (*s).colonne = 2;
        } else {
            (*s).colonne = 54;
        }
        iniune(s);
        /*si Homme*/
        if type_ <= 4 {
            (*s).nbresag = 4; /*4 sagaies*/
            (*s).agressivite = (agress as u16 / 256) as c_uchar;
            (*s).forcehache = tribforce[type_ as usize - 1] as c_uchar;
            (*s).vie = tribvie[type_ as usize - 1] as c_uchar;
            if chef > 0 && tribu == 1
            /*si notre chef alors dur dur*/
            {
                (*s).agressivite = 255;
                (*s).forcehache = 40;
                (*s).vie = 200;
            }
            if type_ == 0
            /*personne*/
            {
                (*s).sapanim[0] = std::ptr::null_mut(); /*pas généré*/
                (*s).sapanim[1] = std::ptr::null_mut();
                (*s).colonne = 200;
                (*s).vie = 0;
            }
            posvit(s);
        } else
        /*bestiole*/
        {
            if type_ >= 7 {
                (*s).sapanim[1] = addr_of_mut!(anlap1) as *mut c_char;
            } else {
                (*s).sapanim[1] = addr_of_mut!(anlou1) as *mut c_char;
            }
            (*s).sapanim[0] = (*s).sapanim[1];
            (*s).sapanim[2] = std::ptr::null_mut(); /*plus d'animation apres 1*/
            (*s).sapanim[3] = std::ptr::null_mut();
            (*s).cross[0] = (((type_ & 1) ^ 1) * 4) as c_uchar;
            (*s).animal = ((type_ - 5) ^ 2) + 1;
            (*s).vie = bestvie[type_ as usize - 5] as c_uchar;
            (*s).ligne = bestlin[type_ as usize - 5] as c_uchar;
            (*s).hauteur = besthaut[type_ as usize - 5] as c_uchar;
            (*s).agressivite = bestagress[type_ as usize - 5] as c_uchar;
            (*s).vitesse = 0; /*fuite à 0*/
            (*s).objet = 1; /*pour frappe*/
            (*s).forcehache = 10; /*dents*/
        }
        decvie(s, 0); /* met en place coeur*/
        generique(FAUX);
    }
}

/***********************************************************************
                BAISSE:
***********************************************************************/
pub extern "C" fn baisse(s: *mut sap) {
    unsafe {
        if (*s).hauteur < 3
        /*on peut encore descendre*/
        {
            (*s).hauteur += 1; /*donc on le fait*/
            (*s).action = 0;
            posan(s, (*s).hauteur as c_short + 8);
            arret(s);
            posan(s, (*s).objet as c_short + 6);
        }
    }
}

/***********************************************************************
                ACTBRA:
**********************************************************************/
pub extern "C" fn actbra(s: *mut sap) {
    unsafe {
        if ((*s).action & 3) == 0 && ((*s).souffle > 32 || who != 0)
        /*deja action (ou essouffle)*/
        {
            if s == addr_of_mut!(sap1) {
                decsou(s, 20);
            }
            (*s).action = (*s).action | ((*s).objet + 1);
            if (*s).objet == 2
            /*objet est la sagaie*/
            {
                if (*s).snssag > 128 && (*s).nbresag != 0
                /*deja sagaie ? */
                {
                    (*s).colsag = (*s).colonne + 2;
                    (*s).nbresag -= 1;
                    (*s).linsag = hautlin[(*s).hauteur as usize] as c_short;
                    (*s).snssag = (*s).sens as c_short;
                }
                /*si sagaie n'existe pas deja*/
                else {
                    return;
                }
            } /*si objet =sagaie*/
            posan(s, (*s).objet as c_short + 3);
        } /*si souffle correct*/
    }
}

/***********************************************************************
                DEPLAT:Deplacement de vue laterale
***********************************************************************/
pub extern "C" fn deplat() {
    unsafe {
        /*position en colonne*/
        if sap1.colonne < 32 {
            ldzeta = ((dzeta + 512) & 0x03FF) as c_short;
            sap1.colonne = LIMITE_DROITE - 1;
        } else {
            ldzeta = dzeta;
            sap1.colonne = LIMITE_GAUCHE + 1;
        }
        sapdep(ldzeta, 46); /* 46 est la vitesse*/
        minute = 0;
        chrono(0);
    }
}

/***********************************************************************
                MVTSAP:Gere les mouvements
**********************************************************************
=> Pointeur sur le sap a etudier
<= 1 si il faut le rappeler immediatement 0 sinon*/
pub extern "C" fn mvtsap(sapiens: *mut sap) -> c_short {
    unsafe {
        let mut nummob: c_short = 0;
        let mut tfr: c_char = 0;
        let mut lastcol: c_short = 0;

        sensmob = (*sapiens).sens;
        let mut registerb: c_short = 0; /*reg b balaye les differents morceaux*/
        let mut ptrcross: c_short = 0;
        loop {
            /*infini*/
            /*on prend l'animation*/
            ptranim = (*sapiens).sapanim[registerb as usize];
            /*teste si fin de sap:animation=0*/
            if ptranim.is_null() {
                return 0;
            }
            let mut ok: c_short = 0;
            while ok == 0 {
                /*ok*/
                tfr = *ptranim; /*caractere suivant*/
                ptranim = ptranim.add(1);
                /*teste si le caractere lu est une commande*/
                if tfr >= 0
                /*non :pas une commande*/
                {
                    ok += 1;
                    nummob = tfr as c_short;
                } else
                /*****************************/
                /*oui !:on traite la commande*/
                {
                    if tfr == -1
                    /* 255 fin d'animation*/
                    {
                        /*l'animation revient a l'animation standard*/
                        ptranim = (*sapiens).sapanim[registerb as usize + 1];
                        continue; /*on revient a la lecture de l'animation*/
                    }
                    tfr &= 127;
                    if tfr == 0
                    /*deplacement vertical*/
                    {
                        /*le caractere suivant donne la hauteur*/
                        (*sapiens).deltah = *ptranim as c_uchar;
                        ptranim = ptranim.add(1);
                        continue;
                    }
                    if tfr == 1
                    /*deplacement horizontal*/
                    {
                        if registerb == 0
                        /*uniquement sur 1ere jambe*/
                        {
                            lastcol = (*sapiens).colonne;
                            if sensmob != 0
                            /*deplacement vers la droite*/
                            {
                                (*sapiens).colonne += 1;
                            } else {
                                (*sapiens).colonne -= 1;
                            }
                            if (sensmob == 0 && (*sapiens).colonne <= LIMITE_GAUCHE)
                                || (sensmob == 1 && (*sapiens).colonne > LIMITE_DROITE)
                            /*sortie !!!!!*/
                            {
                                if presence != 0 {
                                    (*sapiens).colonne = lastcol;
                                    if (present[who as usize] as c_short) < 5 {
                                        arret(sapiens);
                                        if (*sapiens).hauteur != 0 {
                                            posan(sapiens, (*sapiens).hauteur as c_short + 8);
                                        } else {
                                            posan(sapiens, 2);
                                        }
                                        posan(sapiens, (*sapiens).objet as c_short + 6);
                                        (*sapiens).action &= 127;
                                        return 1; /*on le refait*/
                                    }
                                } else {
                                    fin = 128;
                                    if sapiens == addr_of_mut!(sap3) {
                                        if sap1.vie == 0 {
                                            fin = 129;
                                        }
                                        sap1.colonne = sap3.colonne;
                                        sap1.sens = sap3.sens;
                                    }
                                    return 0;
                                }
                            }
                        }
                        continue;
                    }
                    if tfr == 2
                    /*empreinte*/
                    {
                        AffmobConst(
                            MOB_EMPREINTE!(),
                            100 + profond[who as usize] as c_short - 19,
                            4 * ((*sapiens).colonne + (*sapiens).sens as c_short * 5),
                            (*sapiens).sens as c_short,
                        );
                        continue;
                    }
                    if tfr == 3
                    /*neutre*/
                    {
                        ptrcross += 1;
                        ok = 2; /*on passe au morceau suivant*/
                        break;
                    }
                    if tfr == 4
                    /*depositionne baisse*/
                    {
                        (*sapiens).hauteur -= 1;
                        arret(sapiens);
                        posan(sapiens, 10); /*a genoux*/
                        return 1; /*on le refait*/
                    }
                    if tfr == 5
                    /*recule*/
                    {
                        if registerb == 0 {
                            lastcol = (*sapiens).colonne;
                            if sensmob != 0 {
                                (*sapiens).colonne -= 1;
                            } else {
                                (*sapiens).colonne += 1;
                            }
                            if (sensmob == 1 && (*sapiens).colonne < LIMITE_GAUCHE + 3)
                                || (sensmob == 0 && (*sapiens).colonne > LIMITE_DROITE + 3)
                            /*sortie !!!!!*/
                            {
                                if presence != 0 {
                                    (*sapiens).colonne = lastcol;
                                } else {
                                    fin = 128;
                                    if sapiens == addr_of_mut!(sap3) {
                                        if sap1.vie == 0 {
                                            fin = 129;
                                        }
                                        sap1.colonne = sap3.colonne;
                                        sap1.sens = sap3.sens;
                                    }
                                    return 0;
                                }
                            }
                        }
                        continue;
                    }
                    if tfr == 6
                    /*fin d'arme*/
                    {
                        (*sapiens).action = (*sapiens).action & (*ptranim as c_uchar);
                        ptranim = ptranim.add(1);
                        continue;
                    }
                    if tfr == 7
                    /*teste touche*/
                    {
                        /* Si c'est le Joueur il peut toucher les 3 autres */
                        if who == 0 {
                            /* Frappe le copain ????*/
                            if presence < 1 {
                                touche(sapiens, addr_of_mut!(sap3));
                            } else {
                                touche(sapiens, addr_of_mut!(sap2));
                                touche(sapiens, addr_of_mut!(sap4));
                            }
                        } else {
                            /*Si ce n'est pas le joueur il ne vise qu'une personne en particulier*/
                            touche(sapiens, (*sapiens).adversaire);
                        }
                        ptranim = ptranim.add(1);
                        continue;
                    }
                    if tfr == 8
                    /*fin sagaie*/
                    {
                        pogne(sapiens);
                        continue;
                    }
                }
            } /*ok*/
            if ok == 1
            /*ok on y va sinon anim suiv*/
            {
                /*Le caractere lu a ete traite soit comme numero de block
                soit comme commande*/
                let mut linmob: c_short = (*sapiens).ligne as c_short + *ptranim as c_short;
                ptranim = ptranim.add(1);
                tfr = *ptranim;
                ptranim = ptranim.add(1);
                if (*sapiens).sens != 0 {
                    tfr = (-(tfr as c_short) + 6) as c_char;
                    /* teste l'ours*/
                    if (*sapiens).animal == 4 {
                        tfr += 8;
                    }
                }
                (*sapiens).sapanim[registerb as usize] = ptranim; /*on memorise la nlle position*/
                let colmob: c_short = tfr as c_short + (*sapiens).colonne;
                if colmob > 127 {
                    return 0;
                }
                if nummob > 10 {
                    linmob += (*sapiens).deltah as c_short;
                }
                Affmob(
                    nummob + (*sapiens).cross[ptrcross as usize] as c_short + MOB_CORPS!(),
                    (linmob as i8) as c_short + profond[who as usize] as i8 as c_short + 32,
                    4 * colmob,
                    (*sapiens).sens as c_short,
                );
            }
            registerb += 2; /*morceau suivant*/
        } /*infini*/
    }
} /*fin de procedure*/

/***********************************************************************
                        DECVIT:Diminue vitesse
***********************************************************************/
pub extern "C" fn decvit(sapiens: *mut sap) {
    unsafe {
        let tfrvit = (*sapiens).vitesse.wrapping_sub(1);
        if tfrvit != 0 {
            (*sapiens).vitesse = tfrvit;
            posvit(sapiens);
        }
    }
}

/***********************************************************************
                        INCVIT:Augmente vitesse
***********************************************************************/
pub extern "C" fn incvit(sapiens: *mut sap) {
    unsafe {
        let tfrvit = (*sapiens).vitesse.wrapping_add(1);
        if tfrvit < 5
        /* varie de 1 a 4 */
        {
            (*sapiens).vitesse = tfrvit;
            posvit(sapiens);
        }
    }
}

/***********************************************************************
                CLAV:Clavier durant combat
***********************************************************************/
pub extern "C" fn clav(s: *mut sap, caract: c_short) {
    unsafe {
        if caract == 0 {
            return;
        }
        if (caract & 4) != 0 {
            decvit(s);
        } else if (caract & 8) != 0 {
            incvit(s);
        }
        if (caract & 1) != 0 {
            saut(s);
        } else if (caract & 2) != 0 {
            baisse(s);
        }
        if ((caract & 18) == 18 || (caract & 64) != 0) && presence != -1 {
            fin = 129;
        }
        if (caract & 16) != 0 {
            pogne(s);
            while (getkey(0) & 16) != 0 {}
        }
        if (caract & 32) != 0 {
            actbra(s);
        }
    }
}

/***********************************************************************
                     GERCOP:Gere l'ami humain ou non
***********************************************************************/
/* sap 1 c'est le joueur sap3 le copain*/
pub extern "C" fn gercop(scop: *mut sap, type_: c_short) {
    unsafe {
        let mut sens: c_uchar = 0;
        distsp = sap1.colonne - (*scop).colonne; /*distance entre saps*/
        if distsp < 0
        /*valeur absolue*/
        {
            distsp = -distsp;
            sens = 0;
        } else {
            sens = 1;
        }

        /*dans le cas ou on gere le sap3 seul avec nous*/
        /*si le sap1 est mort alors il va tendre a sortir*/
        if distsp > 3
        /*encore loin*/
        {
            (*scop).sens = sens; /*vers le joueur*/
            if type_ != 0
            /* c'est un humain*/
            {
                if (*scop).hauteur != 0 {
                    saut(scop); /*si baisse alors monte*/
                } else {
                    avasap(scop); /*il marche*/
                }
            } else
            /*c'est une bestiole*/
            {
                if (present[who as usize] as c_short) > 6 {
                    (*scop).sapanim[1] = addr_of_mut!(anlap2) as *mut c_char;
                } else {
                    (*scop).sapanim[1] = addr_of_mut!(anlou2) as *mut c_char;
                }
            }
        }
        /* deja present*/
        else {
            if type_ == 0
            /*si bestiole*/
            {
                (*scop).sens = sap1.sens;
                /*arrete*/
                if (present[who as usize] as c_short) > 6 {
                    (*scop).sapanim[1] = addr_of_mut!(anlap1) as *mut c_char;
                    (*scop).sapanim[0] = addr_of_mut!(anlap1) as *mut c_char;
                } else {
                    (*scop).sapanim[1] = addr_of_mut!(anlou1) as *mut c_char;
                    (*scop).sapanim[0] = addr_of_mut!(anlou1) as *mut c_char;
                }
                (*scop).coeur = DEBCRANE!() + 8;
                (*scop).cptrsouffle = 1;
                return;
            }
            /*si humain*/
            if (*scop).objet != 0 {
                pogne(scop); /* plus d'objet a la main*/
            }
            if (*scop).hauteur != 0 {
                saut(scop); /*tjs debout*/
            } else {
                (*scop).vitesse = sap1.vitesse; /*meme sens*/
                arret(scop); /*arrete :vitesse*/
                posvit(scop); /*il va s'arreter,positionner le sens et l'anim*/
            }
        }
    }
}

/***********************************************************************
                AUTONHOM:
***********************************************************************/
pub extern "C" fn autonhom(s2: *mut sap, s1: *mut sap) {
    unsafe {
        let mut sens: c_uchar = 0;
        /*
        test specifique au copain alors que je suis seul a me battre contre un
        adversaire, Sapiens etant un jeu eminement MORAL, mon ami peut refuser
        de frapper le meme adversaire que moi...(2 Sept 87)
        */
        if s2 == addr_of_mut!(sap3)
        /* si c'est mon copain */
        {
            /* si je suis en vie et qu'il n'y a pas d'adversaire pour
               mon ami et que le Rnd est favorable... Il bouge tout de meme
               un peu pour donner le change...
            Mais qu'est ce qu'il me prends de commenter aussi ABONDAMENT ?
            alors que cette trapsure est au moins aussi geniale et
            infaillible que le reste du programme.
               */
            if sap1.vie != 0 && sap4.vie == 0 && Rnd() > 60 {
                return; /* puissant non ???*/
            }
        }

        distsp = (*s1).colonne - (*s2).colonne; /*distance entre saps*/
        if distsp < 0
        /*valeur absolue*/
        {
            distsp = -distsp;
            sens = 1;
        } else {
            sens = 0;
        }

        if (*s2).vie < 20 || Rnd() > 160 {
            if (*s1).hauteur > 1 && Rnd() > 128 {
                (*s2).fuite = 4;
            } else if (*s2).fuite != 0 && Rnd() > 200 {
                (*s2).fuite = Rnd() / 13;
            }
        }
        if sens == (*s2).sens {
            if (*s2).hauteur != 0 {
                saut(s2); /*il se releve*/
            }
            (*s2).sens ^= 1; /*il se retourne*/
            return;
        }
        /* La distance entre les 2 saps est inferieure a 8*/
        if distsp > 8
        /*peut tirer*/
        {
            if (*s2).objet == 2
            /*sagaie*/
            {
                if Rnd() < (*s2).agressivite as c_short {
                    actbra(s2); /*il frappe*/
                }
            } else
            /*si pas sagaie en main il va la prendre*/
            {
                if Rnd() < 20 && (*s2).nbresag != 0 {
                    pogne(s2); /*il essai*/
                    return;
                }
            }
            if Rnd() < 60 {
                /* test si copain et lapin en face */
                if s2 != addr_of_mut!(sap3) || (present[1] as c_short) < 7 {
                    avasap(s2);
                }
            } else {
                saut(s2);
            }
            return;
        } else
        /*assez pres*/
        {
            if distsp < 2 || (*s2).fuite != 0
            /*beaucoup trop pres*/
            {
                /*il tend a se mettre accroupis puis a reculer*/
                if (*s2).hauteur > 1 {
                    saut(s2);
                    return;
                }
                if (*s2).hauteur == 0 {
                    baisse(s2);
                    return;
                }
                if (*s2).fuite != 0 {
                    (*s2).fuite -= 1;
                }
                recusap(s2);
                return;
            }
            if (*s2).hauteur < 2 && Rnd() > 180 && ((*s2).action & 128) == 0 {
                baisse(s2);
            }
            if distsp < 6
            /*il peut frapper*/
            {
                let chance = ((*s2).souffle as i32 * (*s2).agressivite as i32) / 512 + 128;
                if (Rnd() as i32) < chance {
                    actbra(s2); /*il frappe*/
                    return;
                } else
                /*ne frappe pas:alors bouge*/
                {
                    if ((*s1).action != 0 || distsp < 4) && ((*s2).action & 128) == 0 {
                        if (*s2).hauteur < 2 {
                            recusap(s2);
                        } else if Rnd() > 160 {
                            baisse(s2);
                        } else {
                            saut(s2);
                        }
                    } else {
                        baisse(s2);
                    }
                }
            } else
            /*proche mais pas assez pour frapper*/
            {
                if (*s2).hauteur >= 2 {
                    saut(s2); /*il se releve*/
                    return;
                }
                if (*s2).objet != 1 && Rnd() > 70 {
                    pogne(s2); /*prends sa hache*/
                }
                if Rnd() > 80 {
                    avasap(s2);
                }
                return;
            }
            if ((*s1).action & 3) != 0 {
                if (*s1).hauteur >= 2 && Rnd() < 10 {
                    baisse(s2);
                } else {
                    saut(s2);
                }
                return;
            }
            if Rnd() > (*s2).agressivite as c_short {
                if (*s2).hauteur >= 2 {
                    recusap(s2);
                }
                return;
            }
        }
    }
}

/***********************************************************************
                        BESTIO:Gere les Bestioles
***********************************************************************/
pub extern "C" fn bestio(s2: *mut sap, s1: *mut sap) {
    unsafe {
        let mut sens: c_uchar = 0;

        distsp = (*s1).colonne - (*s2).colonne;
        if distsp < 0 {
            distsp = -distsp;
            sens = 1;
        } else {
            sens = 0;
        }
        /*1=Lapin,2=Lagopede,3=Loup,4=Ours*/
        if (*s2).animal >= 3 {
            /*loup-Ours*/
            /* si cote a gauche ou a droite*/
            if (*s2).colonne < LIMITE_GAUCHE - 3 {
                (*s2).sens = 1;
                (*s2).colsag = 10 + Rnd() / 20;
                (*s2).fuite = 2;
            }
            if (*s2).colonne > LIMITE_DROITE + 3 {
                (*s2).sens = 0;
                (*s2).colsag = 10 + Rnd() / 20;
                (*s2).fuite = 2;
            }
            /*si blesse ou avec une faible prob retrait*/
            if ((*s2).vie < 40 && Rnd() > 180) || Rnd() < 10 {
                if (*s2).fuite == 0 {
                    (*s2).sapanim[1] = addr_of_mut!(anlou2) as *mut c_char;
                    (*s2).fuite = 1;
                    (*s2).colsag = Rnd() / 12;
                }
            }
            if (*s2).colsag != 0
            /*fuite:1 fuite reelle 2 éloigne du bord*/
            {
                if (*s2).fuite == 1 {
                    (*s2).sens = sens;
                }
                (*s2).colsag -= 1;
            } else if distsp > 4 && sens == (*s2).sens
            /*pas trop pres et tourne le dos*/
            {
                (*s2).sens ^= 1; /*le sens s'inverse*/
            }
            if Rnd() < (*s2).agressivite as c_short {
                if Rnd() > 100 {
                    (*s2).sapanim[1] = addr_of_mut!(anlou1) as *mut c_char;
                } else {
                    (*s2).sapanim[1] = addr_of_mut!(anlou2) as *mut c_char;
                }
            }
            (*s2).souffle = 128;
            (*s2).coeur = if (*s2).coeur < 0 {
                -(DEBCRANE!() + 8)
            } else {
                DEBCRANE!() + 8
            };
        } else
        /*P'TIT Lapin*/
        {
            (*s2).souffle = 1; /*ca bat vite un p'tit coeur*/
            (*s2).coeur = if (*s2).coeur < 0 {
                -(DEBCRANE!() + 8)
            } else {
                DEBCRANE!() + 8
            };

            if (*s2).fuite == 0 && Rnd() < 60
            /* rien c'est con un lapin*/
            {
                /* plus l'adversaire est proche plus il s'agite*/
                if Rnd() / 8 > distsp {
                    (*s2).sapanim[0] = addr_of_mut!(anlap3) as *mut c_char; /*lapin observe*/
                    if (distsp < 10 && Rnd() > 60)
                        || ((*s1).vitesse & 2) == 0
                        || (*s1).snssag != 255
                        || (*s1).action != 0
                    /*si proche et bouge ou si sagaie en l'air*/
                    {
                        (*s2).sapanim[1] = addr_of_mut!(anlap2) as *mut c_char; /*fuite*/
                        (*s2).sens = (*s1).sens;
                        (*s2).fuite = 1;
                    }
                } else {
                    if Rnd() < (*s2).agressivite as c_short
                    /*ici la peur*/
                    {
                        (*s2).sapanim[0] = addr_of_mut!(anlap2) as *mut c_char;
                        if Rnd() < 25 && distsp < 45
                        /*Reglage sortie Lapin*/
                        {
                            (*s2).sens = (Rnd() & 1) as c_uchar; /*sens au hasard*/
                        }
                    }
                }
            } /*c'est con il fait rien*/
            if (*s2).colonne < LIMITE_GAUCHE + 3 || (*s2).colonne > LIMITE_DROITE - 3 {
                /*il sort de l'ecran : on le degenere*/
                (*s2).sapanim[0] = core::ptr::null_mut();
                (*s2).colonne = 200;
                (*s2).vie = 0; /* a pu lapin*/
                present[who as usize] = 0;
                generique(VRAI);
                if presence as c_short != juge() {
                    fin = (juge() + 1) as c_uchar;
                }
            }
        }
    }
}

/***********************************************************************
                MOBS:Deplace les Sapiens
***********************************************************************/
fn mobs(TesteClavier: Boolean) {
    unsafe {
        /* Pour les 4 sapiens */
        for who_c in (0..4).rev() {
            who = who_c as c_char;
            let surqui = peuple[who_c as usize];
            /* teste si animation mort */
            if (*surqui).flg_mort != 0 {
                ger_mort(surqui);
                while mvtsap(surqui) != 0 {}
            } else if (*surqui).vie != 0
            /*  si defini alors*/
            {
                incsou(surqui, 1); /*il se repose*/
                while mvtsap(surqui) != 0 {} /*il s'anime*/
                if who_c == 0 {
                    /*Le sap1 agit*/
                    let tfragil = cptagil.wrapping_add(agilite);
                    if tfragil < cptagil && TesteClavier {
                        clav(surqui, getkey(0));
                    }
                    cptagil = tfragil;
                } else
                /* Gestion du copain:Il y a-t-il une gestion de sap alors
                qu'il n'y a plus d'adversaire?*/
                if (present[who_c] as c_short) < 5
                /*  homme*/
                {
                    if sap3clav != 0 && who_c == 2
                    /*au clavier*/
                    {
                        if TesteClavier {
                            clav(surqui, getkey(1));
                        }
                    } else if presence == 0 {
                        gercop(surqui, ((present[who_c] as c_short) < 5) as c_short);
                    } else {
                        autonhom(surqui, (*surqui).adversaire);
                    }
                } else
                /*bestiole*/
                {
                    if presence == 0 {
                        gercop(surqui, ((present[who_c] as c_short) < 5) as c_short);
                    } else {
                        bestio(surqui, (*surqui).adversaire);
                        /* si c'est un lapin alors il agit aussi sur
                        mon ami*/
                        if (present[who_c] as c_short) > 6 && present[2] != 0 {
                            bestio(surqui, addr_of_mut!(sap3));
                        }
                    }
                }
                gcoeur(surqui); /*affichage du coeur*/
            }
            gersag(surqui);
        }

        /*tout les impacts devant*/
        for who_c in 0..4 {
            who = who_c as c_char;
            impak(peuple[who_c as usize]);
        }
        /*Affiche les Herbes au 1er plan*/
        for i in 0..NbHerbeAvPlan {
            if herbe[i as usize].type_ != 0 {
                Affmob(
                    herbe[i as usize].type_ as c_short,
                    herbe[i as usize].lin as c_short,
                    herbe[i as usize].col as c_short,
                    herbe[i as usize].sens as c_short,
                );
            }
        }
    }
}

/***********************************************************************
                        CalculeMobHerbe
Calcule un numero de mob pour l'herbe
***********************************************************************/
fn CalculeMobHerbe() -> c_short {
    unsafe {
        let num: c_short;
        if myRandom(0, 1000) > 950 {
            num = (mobAvplanSpec as i64 + 1 + myRandom(0, nbAvplanSpec as i64)) as c_short;
        } else {
            num = (MOB_HERBE!() as i64 + myRandom(0, nbMobHerbes as i64)) as c_short;
        }
        num
    }
}

/***********************************************************************
                        AVPLAN:Calcule avant-plan
***********************************************************************/
const LIGNE_CHEMIN: c_short = 68;
const GRANULOSITE: c_short = 10;
static mut TabIndexHerbeFond: *mut c_short = core::ptr::null_mut();

fn avplan() {
    unsafe {
        let mut num: c_short = 0;
        let mut r: Rect = Rect::default();
        let mut gbak: CGrafPtr = core::ptr::null_mut();
        let mut nb: c_long = 0;

        /* Premier passage */
        if TabIndexHerbeFond.is_null() {
            AcamGetPortOffscreen(&raw mut gbak);
            AcamSetPortOffscreen(Ecran1);
            TabIndexHerbeFond = CalculePaletteAttenuee(MOB_HERBE!(), 0.75, 999);
            AcamSetPortOffscreen(gbak);
        }
        /*on fixe le Rnd*/
        initrnd(
            (xj / 256) as c_short,
            (yj / 256) as c_short,
            ((xj & 224) | ((yj & 224) / 8)) as c_short,
        );
        if nbHerbes <= 20 {
            NbHerbeAvPlan = myRandom(0, nbHerbes as i64) as c_short;
        } else {
            NbHerbeAvPlan = (myRandom(0, (nbHerbes as i64) / GRANULOSITE as i64)
                * GRANULOSITE as i64) as c_short;
        }

        /*on affiche le chemin*/
        for i in 0..8 {
            Affmob(MOB_CHEMIN!() + (rnd0() & 3), LIGNE_CHEMIN, i * 8 * 4, 0);
        }

        /*on affiche les arbres*/
        nb = myRandom(NB_ARBRES_LATERAL as i64, NB_ARBRES_LATERAL_MAX as i64);
        for i in 0..nb {
            //arbre(78,1+i*63+rnd0()/4);
            let y = 68.0 + 10.0 * (i as f32) / (nb as f32); // de 68 a 78
            arbre(y as c_short, myRandom(0, 250) as c_short);
        }
        // herbes arrieres plan
        nb = myRandom(
            (NB_HERBE_ARR_PLAN as c_short) as i64 / GRANULOSITE as i64,
            (NB_MAX_HERBE_AR_PLAN as c_short) as i64 / GRANULOSITE as i64,
        ) * GRANULOSITE as i64;
        /*affichage des herbes a l'arriere plan */
        for _ in 0..nb {
            num = CalculeMobHerbe();
            CalculeAireMob(num, 0, 0, &raw mut r);
            //AffmobPalette(num,LIGNE_CHEMIN+12-(r.bottom/2)+(i/3),myRandom(0,256),rnd0()&1,TabIndexHerbeFond);
            AffmobPalette(
                num,
                (LIGNE_CHEMIN as i64 + 12 - (r.bottom / 2) as i64 + myRandom(0, 3)) as c_short,
                myRandom(0, 256) as c_short,
                rnd0() & 1,
                TabIndexHerbeFond,
            );
        }
        /*on remplit le tableau des herbes avplan*/
        for i in 0..NbHerbeAvPlan {
            num = CalculeMobHerbe();
            herbe[i as usize].type_ = num as c_ushort;
            CalculeAireMob(num, 0, 0, &raw mut r);
            herbe[i as usize].lin = (LIGNE_MAX_HERBE_AV_PLAN as i32
                - (NbHerbeAvPlan as i32 - i as i32)
                    * (LIGNE_MAX_HERBE_AV_PLAN as i32 - LIGNE_BASE_HERBE_AV_PLAN as i32)
                    / NbHerbeAvPlan as i32
                - (r.bottom / 2) as i32
                + 32) as c_ushort;
            /*166*/
            // herbe[i].col=myRandom(0,220);
            herbe[i as usize].col = myRandom(0, 240) as c_ushort;
            herbe[i as usize].sens = (rnd0() & 1) as c_ushort;
            if herbe[i as usize].type_ != 0 {
                Affmob(
                    herbe[i as usize].type_ as c_short,
                    herbe[i as usize].lin as c_short, /*98*/
                    herbe[i as usize].col as c_short,
                    herbe[i as usize].sens as c_short,
                );
            }
        }
        /* L'avplan est recopie sur l'ecran 2*/
        CopyEcran1Sur2();
    }
}

/***********************************************************************
                        LATERB
***********************************************************************/
pub extern "C" fn laterb(TesteClavier: Boolean) {
    unsafe {
        let t = sapiensTimer;

        effdemi();
        mobs(TesteClavier);
        tfrdemi();
        GereInterface();
        while t > sapiensTimer.wrapping_sub(6) {}
    }
}

/***********************************************************************
                        LATERA
***********************************************************************/
pub extern "C" fn latera() {
    unsafe {
        /*affiche noms*/
        EffaceGenerique();
        generique(VRAI);

        dismin = 50;
        affich(((dzeta as i32 + 256) & 0x3ff) as c_ushort);
        /*  Mise en place 3d fn angle
        .... avplan */
        avplan();
        tfrima();
        /* (c) God 7 Aout 87 Inits Souris */
        vq_mouse(&raw mut bouton, &raw mut dsposx, &raw mut dsposy);
        reset_clav();
        laterb(FAUX);
        tfrvirt();
    }
}

/***********************************************************************
                        INARME:Init des armes
***********************************************************************/
pub extern "C" fn inarme() {
    unsafe {
        entvie = sap1.vie;
        sap1.forcehache = 0;
        sap1.cross[FORME_ARME as usize] = 0; /*hache par defaut*/
        sap1.nbresag = 0;

        /* Initialise l'agilite active*/

        cptagil = 1;
        let mut i = liste1.taille;
        while i != 0 {
            i -= 1;
            if sap1.forcehache == 0 {
                if liste1.elements[i as usize] == 22
                /*massue*/
                {
                    sap1.forcehache = (10 + (force as u16 * 10) / 256) as c_uchar;
                    sap1.cross[FORME_ARME as usize] = 1;
                }
                if liste1.elements[i as usize] == 21
                /*hache*/
                {
                    sap1.forcehache = (6 + (force as u16 * 6) / 256) as c_uchar;
                }
            }
            if liste1.elements[i as usize] == 23 {
                sap1.nbresag += 1; /*sagaie*/
            }
        }
    }
}

/***********************************************************************
                        COMBAT:Gere les combats
                On traite la sortie de FIN sur LATERB.
***********************************************************************/
pub extern "C" fn combat() {
    unsafe {
        let mut tfr: c_uchar = 0;
        let mut quelqun: Boolean = false;

        /* Inits Souris */
        reset_mouse();
        if dervue != VUE_LATERALE as c_uchar
        /* egalement si presence humaine ??? ou aberration:Collapse!!*/
        {
            /* Maquillage et habillage des 'Bons'*/
            prm(addr_of_mut!(sap1), -1);
            prm(addr_of_mut!(sap3), present[2] as c_short);
            /* les machinistes montent le decor*/
            latera();
        }
        vq_mouse(&raw mut bouton, &raw mut dsposx, &raw mut dsposy);
        dervue = VUE_LATERALE as c_uchar;
        /* Sapiens une premiere ,Clap !*/
        clrbadi();

        loop {
            if ModeJoueur2 != 0 && (present[2] as c_short) < 5 {
                sap3clav = 1;
            } else {
                sap3clav = 0;
            }
            inarme(); /* initialise les armes sap1,silence sur le plateau*/
            presence = juge(); /* on initialise les adversaires reciproques*/
            /* Si le joueur est seul alors affichage du commentaire */
            if sap2.vie == 0 && sap4.vie == 0 {
                if quelqun {
                    presence = 0;
                    inimen(0);
                    laterb(VRAI);
                    /* On ouvre par defaut le menu'prends'*/
                    InverseOption(0, 2);
                    s_action = 3;
                    inimen(4);
                    return;
                }
                //comment(132);
            } else {
                quelqun = true;
            }
            fin = 0; /* Moteurs !!!!*/
            // ScrollComb(sens);
            while fin == 0 {
                laterb(VRAI); /*  ACTION !!!*/
            }
            /* c'est parfait coupez */

            tfrene(); /*on transfere l'enervement*/
            outarm(); /*armes en sortie*/
            flcomb = 0;
            flefba = 0;
            fdialo = 0;
            if presence == -1 {
                present[2] = 0;
            }
            if fin == 2
            /*clan 1 perdu*/
            {
                CopyEcran1Sur2();
                dervue = VUE_AUCUNE as c_uchar;
                /* combat ultime */
                if scenar == 5 {
                    mort = 0;
                } else {
                    tfr = bless.wrapping_sub(100);
                    if tfr > bless {
                        mort -= 1;
                        tfr = 20;
                    }
                    bless = tfr;
                }
                afstat();
                /* on a perdu */
                if mort == 0 {
                    scenar = 7; // Perdu Mort
                    sceveu(7);
                    presence = 0;
                    inimen(0);
                    return;
                }
                comment(81);
                /*c'est un homme qui a gagne*/
                if (present[1] as c_short) < 5 && present[1] != 0 {
                    vole();
                }
                prm(addr_of_mut!(sap2), 0);
                prm(addr_of_mut!(sap4), 0);
                presence = 0;
                attends_bruit();
                inimen(0);
                return;
            }
            if fin == 1
            /*clan 2 perdu*/
            {
                presence = 0;
                prm(addr_of_mut!(sap2), 0);
                prm(addr_of_mut!(sap4), 0);
                /* On a eu le chef des Hyene folle*/
                if scenar == 4 && chef != 0 && tribu == 4 {
                    scenpr = 0;
                    sceveu(9);
                }
                /* Scenario ultime */
                if scenar == 5 {
                    mort = 0; /*thats all folks*/
                    scenar = 6;
                    jour_dsk += 1;
                    sceveu(6); /*bravo tu as gagne*/
                    presence = 0;
                    inimen(0);
                    return;
                }
            }
            /*si esc */
            if (fin == 129 && presence == 0) || mort == 0 {
                if sap1.vie == 0 {
                    dervue = 255;
                }
                presence = 0;
                inimen(0);
                return;
            }
            /*Si sortie de l'ecran (nous sommes donc seuls)*/
            if fin == 128 {
                clrbadi();
                deplat();
                zoninf();
                if presence != 0 && presence < 5 {
                    prm(addr_of_mut!(sap1), present[0] as c_short);
                    prm(addr_of_mut!(sap3), present[2] as c_short);
                    latera();
                    dervue = VUE_LATERALE as c_uchar;
                    inimen(0);
                    return;
                }
                fixcolon(addr_of_mut!(sap1));
                fixcolon(addr_of_mut!(sap3));
                prm(addr_of_mut!(sap3), present[2] as c_short);
                latera();
            }
        } /*infini .... et un peu plus*/
    }
}

/* Copie une chaine C dans un tampon C (strcpy) */
#[deprecated]
unsafe fn copie_chaine(buf: *mut c_char, s: *mut c_char) {
    unsafe {
        if s.is_null() {
            if !buf.is_null() {
                *buf = 0;
            }
            return;
        }
        let src = CStr::from_ptr(s);
        for (i, b) in src.to_bytes().iter().enumerate() {
            *buf.add(i) = *b as c_char;
        }
        *buf.add(src.to_bytes().len()) = 0;
    }
}

/* Remplace un sprintf(format, %s -> nom) minimaliste */
#[deprecated]
unsafe fn sprintf_nom(buf: *mut c_char, format: *mut c_char, nom: *mut c_char) {
    unsafe {
        let fmt = CStr::from_ptr(format);
        let f = fmt.to_string_lossy();
        let nn = if nom.is_null() {
            String::new()
        } else {
            CStr::from_ptr(nom).to_string_lossy().into_owned()
        };
        let out = f.replacen("%s", &nn, 1);
        for (i, b) in out.as_bytes().iter().enumerate() {
            *buf.add(i) = *b as c_char;
        }
        *buf.add(out.len()) = 0;
    }
}
