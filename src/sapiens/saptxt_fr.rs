/**********************************************************************
                          TEXTES VERSION FRANCAISE
                        (C) DIDIER 12 Avril 1987
                            30 mars 2017
**********************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Port Rust de "Sources/SAPTXT_fr.C" : textes de la langue francaise.
// Donnees pures, aucune fonction. Chaines en MacRoman brut ; les
// caracteres accentues sont ecrits selon la substitution d'origine
// (`%`=o `^`=n `&`=a `|`=i `#`=e `_`=u `[`=? `]`=!) et convertis
// par CvtCarLissage (saptxt.rs). Ne pas convertir en UTF-8.
use super::saptxt::{nomref, sc};
use crate::sapiens::{NB_MOTS_BKMOTS, reference};
use std::os::raw::c_char;

pub(crate) struct Saptxt_fr {
    pub(crate) refer0: [reference; 12],
    pub(crate) refer1: [reference; 47],
    pub(crate) refer2: [reference; 29],
    pub(crate) refer3: [reference; 27],
    pub(crate) mescon: [*mut c_char; 138],
    pub(crate) bkmots: [*mut c_char; NB_MOTS_BKMOTS as usize],
    pub(crate) scenme: [*mut c_char; 14],
    pub(crate) app: [*mut c_char; 23],
}

impl Default for Saptxt_fr {
    fn default() -> Self {
        Self {
            refer1: [
    reference {
        nomref: nomref(b"SA PROTECTION"),
        param1: 0x3F,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"EAU"),
        param1: 0x25,
        param2: 0xF,
    },
    reference {
        nomref: nomref(b"HERBE"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"FEUILLE"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"POMME"),
        param1: 0x21,
        param2: 0x12,
    },
    reference {
        nomref: nomref(b"POIRE"),
        param1: 0x21,
        param2: 0x12,
    },
    reference {
        nomref: nomref(b"MYRTILLES"),
        param1: 0x21,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"LARVE"),
        param1: 0x21,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"INSECTE"),
        param1: 0x21,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"CHARDON"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"ORCHIS"),
        param1: 0x21,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"ORTIES"),
        param1: 0x20,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"CHICOREE"),
        param1: 0x21,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"SALSIFIS"),
        param1: 0x21,
        param2: 0x21,
    },
    reference {
        nomref: nomref(b"OS"),
        param1: 0x20,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"MELISSE"),
        param1: 0x41,
        param2: 0x2,
    },
    reference {
        nomref: nomref(b"PISSENLIT"),
        param1: 0x21,
        param2: 0x21,
    },
    reference {
        nomref: nomref(b"PERVENCHE"),
        param1: 0x41,
        param2: 0x2,
    },
    reference {
        nomref: nomref(b"SILEX"),
        param1: 0x35,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"CAILLOU"),
        param1: 0x30,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"HACHE"),
        param1: 0x1A,
        param2: 0x0,
    }, /* 20 */
    reference {
        nomref: nomref(b"MASSUE"),
        param1: 0x18,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"SAGAIE"),
        param1: 0x16,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"RACLOIR"),
        param1: 0x14,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"ONGUENT"),
        param1: 0x48,
        param2: 0xA,
    },
    reference {
        nomref: nomref(b"OUTRE VIDE"),
        param1: 0x38,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"OUTRE PLEINE"),
        param1: 0x2A,
        param2: 0xE,
    },
    reference {
        nomref: nomref(b"POINTE DE SILEX"),
        param1: 0x37,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"TISSUS"),
        param1: 0x3B,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"PANIER"),
        param1: 0x3A,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"PEAU D'OURS"),
        param1: 0x3A,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"PEAU DE LOUP"),
        param1: 0x39,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"PEAU DE LAPIN"),
        param1: 0x33,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"GRIFFE D'OURS"),
        param1: 0x39,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"DENT DE LOUP"),
        param1: 0x38,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"VIANDE SECHEE"),
        param1: 0x27,
        param2: 0x90,
    },
    reference {
        nomref: nomref(b"VIANDE FRAICHE"),
        param1: 0x26,
        param2: 0xA1,
    },
    reference {
        nomref: nomref(b"VIANDE "),
        param1: 0x26,
        param2: 0x91,
    },
    reference {
        nomref: nomref(b"VIANDE RASSIE"),
        param1: 0x24,
        param2: 0x61,
    },
    reference {
        nomref: nomref(b"VIANDE AVARIEE"),
        param1: 0x21,
        param2: 0x30,
    },
    reference {
        nomref: nomref(b"VIANDE POURRIE"),
        param1: 0x30,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Potion de Vie"),
        param1: 0x3D,
        param2: 0x0,
    }, /* 42 */
    reference {
        nomref: nomref(b"Hache d'Ivoire"),
        param1: 0x3D,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Baton Sacr&"),
        param1: 0x3D,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"SAGAIE BRISEE"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"* Disquette *"),
        param1: 0x2F,
        param2: 0xFF,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            refer0: [
    reference {
        nomref: nomref(b"OBSERVER"),
        param1: 0x3,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"ABSORBER"),
        param1: 0x1,
        param2: 0x1,
    },
    reference {
        nomref: nomref(b"DIRE"),
        param1: 0x2,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"POSER"),
        param1: 0x1,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"PRENDRE"),
        param1: 0x4,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"DONNER"),
        param1: 0x1,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"SE SOIGNER"),
        param1: 0x1,
        param2: 0x2,
    },
    reference {
        nomref: nomref(b"DORMIR"),
        param1: 0x5,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"FABRIQUER"),
        param1: 0x6,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"ETAT"),
        param1: 0x7,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"SAPIENS"),
        param1: 0x8,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            refer2: [
    reference {
        nomref: nomref(b"QUI ES-TU ?"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"QUE VEUX-TU ?"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"QUE M'OFFRES-TU ?"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"OU EST TON CHEF ?"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"JE SUIS \x82"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"BONNE CHASSE."),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"SOUMETS-TOI."),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"ACCOMPAGNE-MOI."),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"N'APPROCHE PAS!"),
        param1: 0x1,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"PASSE TON CHEMIN."),
        param1: 0x1,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"HORS DE MA VUE"),
        param1: 0x1,
        param2: 0x30,
    },
    reference {
        nomref: nomref(b"TU N'ES QU'UN COUARD!"),
        param1: 0x1,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"BAISSE LES YEUX"),
        param1: 0x1,
        param2: 0x50,
    },
    reference {
        nomref: nomref(b"JE T'ECRASE D'UNE MAIN!"),
        param1: 0x1,
        param2: 0x60,
    },
    reference {
        nomref: nomref(b"PROSTERNE-TOI,CHIEN!"),
        param1: 0x1,
        param2: 0x70,
    },
    reference {
        nomref: nomref(b"COEUR DE LAPIN!"),
        param1: 0x1,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"PRIMATE DEGENERE!"),
        param1: 0x1,
        param2: 0x90,
    },
    reference {
        nomref: nomref(b"FACE DE PORC!"),
        param1: 0x1,
        param2: 0xA0,
    },
    reference {
        nomref: nomref(b"PAIX,AMI."),
        param1: 0x2,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"GLOIRE A TA TRIBU"),
        param1: 0x2,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"TOUS TE VENERENT!"),
        param1: 0x2,
        param2: 0xC0,
    },
    reference {
        nomref: nomref(b"PITIE,EPARGNE MA VIE!"),
        param1: 0x2,
        param2: 0xF0,
    },
    reference {
        nomref: nomref(b"C'EST MOI LE PLUS FORT!"),
        param1: 0x3,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"RIEN NE ME RESISTE!"),
        param1: 0x3,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"JE MACHE LE SILEX!"),
        param1: 0x3,
        param2: 0x60,
    },
    reference {
        nomref: nomref(b"MEME L'OURS ME FUIT"),
        param1: 0x3,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"LA FOUDRE M'OBEIT."),
        param1: 0x3,
        param2: 0xA0,
    },
    reference {
        nomref: nomref(b"LES ESPRITS ME GUIDENT."),
        param1: 0x3,
        param2: 0xC0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            refer3: [
    reference {
        nomref: nomref(b"PAYSAGE"),
        param1: 0x1,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"VUE LATERALE"),
        param1: 0x1,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"PANORAMA"),
        param1: 0x1,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"CARTE"),
        param1: 0x1,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"UN PEU"),
        param1: 0x0,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"UNE DEMI JOURNEE"),
        param1: 0x0,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"UNE JOURNEE"),
        param1: 0x0,
        param2: 0xFF,
    },
    reference {
        nomref: nomref(b" \x8E"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b" \x8F"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"\x99 "),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"\x99 "),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"\x99 "),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Charge Aventure"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Sauve Aventure"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Nouvelle Aventure"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Aide autonome"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Aide au clavier"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Configuration"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Manipulation"),
        param1: 0,
        param2: 0,
    }, /* 19 */
    reference {
        nomref: nomref(b"Musique 'DMA'"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Musique MIDI"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"GOD!"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"GOD!"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Version Anglaise"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"R&glages MIDI"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Pr&f&rences"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            mescon: [
    sc(b"MERCI DU CADEAU \x00"),
    sc(b"MERCI  \x00"),
    sc(b"MERCI AMI \x00"),
    sc(b"MERCI A TOI \x00"),
    sc(b"QUI ES TU?\x00"),
    sc(b"QUEL EST TON NOM?\x00"),
    sc(b"PRESENTE-TOI!\x00"),
    sc(b"JE NE TE CONNAIS PAS.\x00"),
    sc(b"CE N'EST PAS ASSEZ\x00"),
    sc(b"TU RIGOLES ?!\x00"),
    sc(b"PAS QUESTION!\x00"),
    sc(b"C'EST TROP PEU\x00"),
    sc(b"MARCHE CONCLU\x00"),
    sc(b"D'ACCORD\x00"),
    sc(b"CELA CONVIENT A \x83\x00"),
    sc(b"C'EST BON\x00"),
    sc(b"\x83 ACCEPTE TON CADEAU.\x00"),
    sc(b"BON,\x83 PREND\x00"),
    sc(b"OUAIS,\x83 ACCEPTE\x00"),
    sc(b"\x83 LE PREND.\x00"),
    sc(b" CA NE SUFFIT PAS\x00"),
    sc(b" DONNE ENCORE\x00"),
    sc(b" SOIS PLUS GENEREUX\x00"),
    sc(b" POUR TE FAIRE PLAISIR\x00"),
    sc(b"HUM...\x00"),
    sc(b"A PART CA?\x00"),
    sc(b"TIENS DONC\x00"),
    sc(b"CERTES...\x00"),
    sc(b"FINISSONS-EN\x00"),
    sc(b"ASSEZ DE DISCOURS\x00"),
    sc(b"MAIS ENCORE\x00"),
    sc(b"ABREGEONS\x00"),
    sc(b"\x83  VA TE TUER,CHAROGNE!\x00"),
    sc(b"RAVALE TES INSULTES\x00"),
    sc(b"\x83  VA TE BRISER\x00"),
    sc(b"FATIGUE DE VIVRE ?!\x00"),
    sc(b"TES OS NOURRIRONT LES CORBEAUX.\x00"),
    sc(b"TU VAS PAYER\x00"),
    sc(b"\x83  VA T'ECLATER\x00"),
    sc(b"\x83  BOIRA DANS TON CRANE\x00"),
    sc(b"DEFENDS TOI,CLOPORTE!\x00"),
    sc(b"LA VICTOIRE OU LA MORT\x00"),
    sc(b"PAS DE PITIE\x00"),
    sc(b"QUE LES ESPRITS GUIDENT \x83\x00"),
    sc(b"JE SUIS \x83\x00"),
    sc(b"JE ME NOMME \x83\x00"),
    sc(b"MON NOM EST \x83\x00"),
    sc(b"JE M'APPELLE \x83\x00"),
    sc(b"TA VIE !\x00"),
    sc(b"TE TUER!\x00"),
    sc(b"TON COEUR!\x00"),
    sc(b"TON CRANE\x00"),
    sc(b"\x83  VEUT \x00"),
    sc(b"\x83  DESIRE \x00"),
    sc(b"\x83  VOUDRAIT \x00"),
    sc(b"\x83  A BESOIN DE \x00"),
    sc(b"\x83  T'OFFRE \x00"),
    sc(b"\x83  TE PROPOSE \x00"),
    sc(b"\x83  T'ECHANGE \x00"),
    sc(b"\x83  TE CEDE \x00"),
    sc(b"\x83  NE REPONDRA PAS\x00"),
    sc(b"...\x00"),
    sc(b"AH AH AH!\x00"),
    sc(b"QUI SAIT?\x00"),
    sc(b" ET TOI,\x00"),
    sc(b"RIEN\x00"),
    sc(b" SAPIENS par Didier et Olivier Guillion.\x00"),
    sc(b" (c) 1986-2017\x00"),
    sc(b"Musiques compos&es par Gilles Soulet.\x00"),
    sc(b"&Il y a cent mille ans,le jour se l%ve...\x00"), /* 70 */
    sc(b"BONJOUR ,\x82\x00"),
    sc(b"C'EST DONC TOI ,\x82 ?\x00"),
    sc(b"BONNE CHASSE \x82\x00"),
    sc(b"Des Hurlements de loups.\x00"),
    sc(b"Des Grognements d'ours.\x00"),
    sc(b"Un bruit de Source.\x00"), /* 76 */
    sc(b"Une odeur de Gibier.\x00"),
    sc(b"Une odeur de Fum&e.\x00"),
    sc(b"Une odeur d'Homme.\x00"),
    sc(b"Il s'enfuit.\x00"),
    sc(b"Tu es &vanoui.\x00"),
    sc(b"LA TRIBU EST FIERE DE TOI,\x82\x00"),
    sc(b"TU ES UN GUERRIER VALEUREUX,\x82\x00"),
    sc(b"LA FORCE DE L'AUROCH SEMBLE ETRE EN TOI!\x00"),
    sc(b"Tu possedes une nouvelle arme.\x00"),
    sc(b"Ton silex est trop mal taill&.\x00"),
    sc(b"Tu n'as pas de silex ...\x00"),
    sc(b"Nom ? \x00"),
    sc(b"Homme ou Femme (H,F) ?\x00"), /* 89 */
    sc(b"Nom de l'Aventure a charger ? \x00"),
    sc(b"Nom de l'Aventure a sauver ? \x00"),
    sc(b"Collapse !!!!!\x00"),
    sc(b"La Faim trouble ta vue !\x00"),
    sc(b"La Soif ravine ta gorge !\x00"),
    sc(b"La Fatigue p%se sur ton corps !\x00"),
    sc(b"Tu bois \x00"),
    sc(b"et remplis toutes tes outres\x00"),
    sc(b"Tu n'as pas soif \x00"),
    sc(b"...PAS QUESTION !\x00"),
    sc(b"JE TE SUIS.\x00"), /* 100 */
    sc(b"Tu es beaucoup trop charg& !!!\x00"),
    sc(b"Ta force a impressionn& un Loup.\x00"),
    sc(b"Ton courage a dompt& un Ours.\x00"),
    sc(b"Ton aptitude $ la fuite a s&duit un Lapin.\x00"),
    sc(b"Un Coq t'emboite le pas.\x00"),
    sc(b"Cette version ne marche que sur les ST anglais !\x00"),
    sc(b"Goinfre ! tu n'as ni faim,ni soif !\x00"),
    sc(b"Pas de paresse.Tu n'as pas sommeil.\x00"),
    sc(b"Ta sant& est parfaite,pourquoi te soigner ?\x00"),
    sc(b"Splendide,\x00"),
    sc(b"Ton ami \xAC est hors de combat ...\x00"),
    sc(b"D&sol& \x82,\xAC est Mort !!!\x00"),
    sc(b"Tu as d&ja sauv& une Aventure aujourd'hui...\x00"),
    sc(b"Un nouveau jour se l%ve ...\x00"), /* 114 */
    sc(b"Allez hop, $ la maison !!!\x00"),
    sc(b"La musique ST,c'est beau ...\x00"),
    sc(b"Que de c&l&brit&s !\x00"),
    sc(b"Petit cadeau amical...\x00"),
    sc(b"Salut vieux !!! \x00"),
    sc(b"SAPIENS par D et O Guillion.\x00"), /* 120 */
    sc(b"SAPIENS option choisie :\x00"),
    sc(b"Ton Etat :\x00"),
    sc(b"\x82 prends\x00"),
    sc(b"\x82 pose\x00"),
    sc(b"\x82 dit :\x00"),
    sc(b"\x82 donne\x00"),
    sc(b"\x82 regarde\x00"),
    sc(b"\x82 absorbe\x00"),
    sc(b"\x82 se soigne avec\x00"),
    sc(b"\x82 dors\x00"), /* 130 */
    sc(b"Avec un silex,\x82 fabrique\x00"),
    sc(b"Touche F1 pour de l'aide !\x00"),            /* 132 */
    sc(b"Et le temps passe...\x00"),                  /* 133 */
    sc(b"Tu es touch& par la foudre !!!!!!!!!!\x00"), /* 134 */
    sc(b"Heu...Hummm...Tu es seul...\x00"),           /* 135 */
    sc(b"Tu es mort ! La partie est finie...\x00"),
    sc(b"Je cr&& le monde...\x00"),
    sc(b"$END$\x00"),
            ],
            bkmots: [
    sc(b"$0\x00"), // nomsecondaire
    sc(b"$1\x00"), // nomjoueur
    sc(b"$2\x00"), // nomadversaire
    sc(b"$3\x00"), // nomtribu
    sc(b" CHEF\x00"),
    sc(b" DES PIEDS AGILES\x00"),
    sc(b" DES YEUX MALINS\x00"),
    sc(b" DES BRAS DE ROC\x00"),
    sc(b" DES HYENES FOLLES\x00"),
    sc(b"HOGNOR\x00"),
    sc(b"SL-OOM\x00"),
    sc(b"GRUNGH\x00"),
    sc(b"KROK-G\x00"),
    sc(b" FAIM.\x00"),
    sc(b" SOIF.\x00"),
    sc(b"TA SANTE EST \x00"),
    sc(b"TU MEURS DE\x00"),
    sc(b"TU AS TRES\x00"),
    sc(b"TU AS UN PEU\x00"),
    sc(b"TU N'AS PAS \x00"),
    sc(b"EPUISE\x00"),
    sc(b"TRES \x97 \x00"),
    sc(b" FATIGUE\x00"),
    sc(b"REPOSE\x00"),
    sc(b"TU ES \x00"),
    sc(b"GRAVEMENT\x9C\x00"),
    sc(b"TRES \x9C \x00"),
    sc(b" BLESSE\x00"),
    sc(b"EN BONNE SANTE\x00"),
    sc(b" JOURNEE\x00"),
    sc(b"NORD\x00"),
    sc(b"SUD\x00"),
    sc(b"EST\x00"),
    sc(b"OUEST\x00"),
    sc(b"PEAU D\x00"),
    sc(b"VIANDE \x00"),
    sc(b" AVENTURE\x00"),
    sc(b"MERCI\x00"),
    sc(b"MORT !\x00"),
    sc(b"TRES VULNERABLE\x00"),
    sc(b"VULNERABLE\x00"),
    sc(b"ROBUSTE\x00"),
    sc(b"TRES ROBUSTE\x00"),
    sc(b"$4\x00"), // Nomcopain
    sc(b"ton Compagnon\x00"),
    sc(b"$END$\x00"),
            ],
            scenme: [
    sc(b"Enfant,la Tribu a faim,\r            rapporte des vivres.\x00"),
    sc(b"     Les Jeunes ont mang& des baies\r               V&n&neuses.\r Il nous faut le rem%de des Yeux-Malins.\x00"),
    sc(b"Les Bras-de-Roc nous ont lanc& un d&fi.\rVa,combats leur Chef et vole-lui sa Hache\x00"),
    sc(b"    Guerrier,pr&pare-toi au combat.\r    Les Hy%nes-Folles ont d&rob&\r            le BATON SACRE.\rLes Esprits de nos Anc%tres Le r&clament!\x00"),
    sc(b"Tes Exploits nuisent $ mon prestige.\rQue la Loi de la Force d&signe le Chef!\x00"),
    sc(b"     Voici donc l'Histoire jusqu'alors\r       ignor&e de celui qui,il y a cent\r   mill&naires devint par la Force de son\r         bras le chef de la Tribu des\r              Pieds Agiles.\r     Il v&cut jusqu'$ l'Age de quarante\rPrintemps et mourut de la main d'un jeune\r               Chasseur.\rBonne Chasse !\x00"),
    sc(b"    Nul ne pourrait survivre $ de si\r           terribles blessures...\r       Ici s'ach%ve ta Route,Chasseur.\x00"),
    sc(b"'Il y a Cent Mille Ans ...\r        ...l'Aventure Humaine Commen#ait'\r\r   SAPIENS a &t& con#u et r&alis& par\rDidier et Olivier Guillion pour THOMSON,\r AMSTRAD CPC,Compatibles et ATARI ST.\rMerci $ Jean-Francois Cassan ainsi qu'$\r Gilles Soulet, Jean-Michel Georges et\r    Yves Basquet du groupe 'MYRIAD'\rpour leurs aides,critiques et conseils.\r\r       Toulouse, Juillet-Aout 1987.\r        re(c) 1996-2017\x00"),
    sc(b"Je suis vaincu,le Baton Sacr& est cach&\r   au Sud sous la Protection des Trois\r                Totems !.\x00"),
    sc(b"Vaillant Chasseur ton exemplaire n'est\rpas enregistr&. Envoie 10 euros aux auteurs\rpour aller plus loin dans l'Aventure.\x00"),
    sc(b"     La musique DMA de Sapiens:\r\r- Nombre de voies: trois\r- Pr&cision echantillon: huit bits\r- Fr&quence echantillon: neuf kHz\r- Temps de musique: quinze mn.\r- Temps machine: cinquante pour cent\r- Datas: Cent-cinquante Ko\r- Circuit digitaliseur: Ferranti ZN\r- Montage: Jean-Michel Georges.\r- Soft utilis&: STampler\r- M&thode employ&e: devinez...\x00"),
    sc(b"Ce soft a &t& d&velopp& en C (Didier)\ret en Assembleur (Olivier).\rLes d&cors sont de Didier Guillion,\rLa musique est de Gilles Soulet,\rinterpr&t&e par 'Digital Music Associates'\rsampl&e aux laboratoires 'Myriad Records'.\rLa mise en sc%ne est de D et O Guillion.\rEffets sp&ciaux et trucages: J-M Georges\r\r  R&alis& de Juillet $ Aout MCMLXXXVII  \r             $ Toulouse.\r Copyright D&O Guillion 1986-1996,99,01,05,17\x00"),
    sc(b"Ont collabor& $ la r&alisation de SAPIENS:\r\rY.Basquet:Support logistique et magn&tique\rJ-M Georges: Hard, Interruptions, clavier,\r              disquette.\rG.Soulet: Composition de la musique.\rGuyon James:Traduction en Anglais.\rD.Carri%re: Principe du Sampling.\rJ-F Cassan: Soutien moral et Vodka Orange.\rP.Guegan: Inventeur des b&n&fices n&gatifs\rMotorola: Adressages indirects index&s\r         avec d&placement par rapport $ PC\x00"),
    sc(b"$END$\x00"),
            ],
            app: [
    sc(b"Agilit&\x00"),
    sc(b"Force\x00"),
    sc(b"Charisme\x00"),
    sc(b"Vitalit&\x00"),
    sc(b"Resistance\x00"),
    sc(b"Reste\x00"),
    sc(b"Joueur Secondaire,\x00"),
    sc(b"Joueur Principal, \x00"),
    sc(b"Quelle touche pour \x00"),
    sc(b"Haut\x00"),
    sc(b"Bas\x00"),
    sc(b"Gauche\x00"),
    sc(b"Droite\x00"),
    sc(b"Change Arme/Validation\x00"),
    sc(b"Frappe\x00"),
    sc(b"Sortie\x00"),
    sc(b"?\x00"),
    sc(b"(....)\x00"),
    sc(b" Loup \x00"),
    sc(b" Ours \x00"),
    sc(b"Lapin \x00"),
    sc(b" Coq  \x00"),
    sc(b"$END$\x00"),
            ],
        }
    }
}
