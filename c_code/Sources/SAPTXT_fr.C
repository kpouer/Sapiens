/**********************************************************************
                          TEXTES VERSION FRANCAISE
                        (C) DIDIER 12 Avril 1987
							30 mars 2017
***********************************************************************/
#include "sapiens.h"

struct reference refer1_fr[]={
{"SA PROTECTION",0x3F,0x00},
{"EAU",0x25,0xF},
{"HERBE",0x20,0x11},
{"FEUILLE",0x20,0x11},
{"POMME",0x21,0x12},
{"POIRE",0x21,0x12},
{"MYRTILLES",0x21,0x11},
{"LARVE",0x21,0x20},
{"INSECTE",0x21,0x10},
{"CHARDON",0x20,0x11},
{"ORCHIS",0x21,0x20},
{"ORTIES",0x20,0x10},
{"CHICOREE",0x21,0x11},
{"SALSIFIS",0x21,0x21},
{"OS",0x20,0x10},
{"MELISSE",0x41,0x2},
{"PISSENLIT",0x21,0x21},
{"PERVENCHE",0x41,0x2},
{"SILEX",0x35,0x0},
{"CAILLOU",0x30,0x0},
{"HACHE",0x1A,0x0},  /*20*/
{"MASSUE",0x18,0x0},
{"SAGAIE",0x16,0x0},
{"RACLOIR",0x14,0x0},
{"ONGUENT",0x48,0xA},
{"OUTRE VIDE",0x38,0x0},
{"OUTRE PLEINE",0x2A,0xE},
{"POINTE DE SILEX",0x37,0x0},
{"TISSUS",0x3B,0x0},
{"PANIER",0x3A,0x0},
{"PEAU D'OURS",0x3A,0x0},
{"PEAU DE LOUP",0x39,0x0},
{"PEAU DE LAPIN",0x33,0x0},
{"GRIFFE D'OURS",0x39,0x0},
{"DENT DE LOUP",0x38,0x0},
{"VIANDE SECHEE",0x27,0x90},
{"VIANDE FRAICHE",0x26,0xA1},
{"VIANDE ",0x26,0x91},
{"VIANDE RASSIE",0x24,0x61},
{"VIANDE AVARIEE",0x21,0x30},
{"VIANDE POURRIE",0x30,0x0},
{"Potion de Vie",0x3D,0x0},  /* 42*/
{"Hache d'Ivoire",0x3D,0x0},
{"Baton Sacr&",0x3D,0x0},
{"SAGAIE BRISEE",0,0},
{"* Disquette *",0x2F,0xFF},
{"$END$",0,0}
},
refer0_fr[]={
{"OBSERVER",0x3,0x0},
{"ABSORBER",0x1,0x1},
{"DIRE",0x2,0x0},
{"POSER",0x1,0x0},
{"PRENDRE",0x4,0x0},
{"DONNER",0x1,0x0},
{"SE SOIGNER",0x1,0x2},
{"DORMIR",0x5,0x0},
{"FABRIQUER",0x6,0x0},
{"ETAT",0x7,0x0},
{"SAPIENS",0x8,0x0},
{"$END$",0,0}
},
refer2_fr[]={
{"QUI ES-TU ?",0x0,0x0},
{"QUE VEUX-TU ?",0x0,0x0},
{"QUE M'OFFRES-TU ?",0x0,0x0},
{"OU EST TON CHEF ?",0x0,0x0},
{"JE SUIS \x82",0x0,0x0},
{"BONNE CHASSE.",0x0,0x0},
{"SOUMETS-TOI.",0x0,0x0},
{"ACCOMPAGNE-MOI.",0,0},
{"N'APPROCHE PAS!",0x1,0x10},
{"PASSE TON CHEMIN.",0x1,0x20},
{"HORS DE MA VUE",0x1,0x30},
{"TU N'ES QU'UN COUARD!",0x1,0x40},
{"BAISSE LES YEUX",0x1,0x50},
{"JE T'ECRASE D'UNE MAIN!",0x1,0x60},
{"PROSTERNE-TOI,CHIEN!",0x1,0x70},
{"COEUR DE LAPIN!",0x1,0x80},
{"PRIMATE DEGENERE!",0x1,0x90},
{"FACE DE PORC!",0x1,0xA0},
{"PAIX,AMI.",0x2,0x40},
{"GLOIRE A TA TRIBU",0x2,0x80},
{"TOUS TE VENERENT!",0x2,0xC0},
{"PITIE,EPARGNE MA VIE!",0x2,0xF0},
{"C'EST MOI LE PLUS FORT!",0x3,0x20},
{"RIEN NE ME RESISTE!",0x3,0x40},
{"JE MACHE LE SILEX!",0x3,0x60},
{"MEME L'OURS ME FUIT",0x3,0x80},
{"LA FOUDRE M'OBEIT.",0x3,0xA0},
{"LES ESPRITS ME GUIDENT.",0x3,0xC0},
{"$END$",0,0}
},
refer3_fr[]={
{"PAYSAGE",0x1,0x0},
{"VUE LATERALE",0x1,0x0},
{"PANORAMA",0x1,0x0},
{"CARTE",0x1,0x0},
{"UN PEU",0x0,0x20},
{"UNE DEMI JOURNEE",0x0,0x80},
{"UNE JOURNEE",0x0,0xFF},
{ " \x8E",0x0,0x0},
{ " \x8F",0x0,0x0},
{ "\x99 ",0x0,0x0},
{ "\x99 ",0x0,0x0},
{ "\x99 ",0x0,0x0},
{"Charge Aventure",0x0,0x0},
{"Sauve Aventure",0x0,0x0},
{"Nouvelle Aventure",0x0,0x0},
{"Aide autonome",0,0},
{"Aide au clavier",0,0},
{"Configuration",0,0},
{"Manipulation",0,0},           /*19*/
{"Musique 'DMA'",0,0},
{"Musique MIDI",0,0},
{"GOD!",0,0},
{"GOD!",0,0},
{"Version Anglaise",0,0},
{"R&glages MIDI",0,0},
{"Pr&f&rences",0,0},
{"$END$",0,0}
};


char * mescon_fr[]={
"MERCI DU CADEAU ",
"MERCI  ",
"MERCI AMI ",
"MERCI A TOI ",
"QUI ES TU?",
"QUEL EST TON NOM?",
"PRESENTE-TOI!",
"JE NE TE CONNAIS PAS.",
"CE N'EST PAS ASSEZ",
"TU RIGOLES ?!",
"PAS QUESTION!",
"C'EST TROP PEU",
"MARCHE CONCLU",
"D'ACCORD",
"CELA CONVIENT A \x83",
"C'EST BON",
"\x83 ACCEPTE TON CADEAU.",
"BON,\x83 PREND",
"OUAIS,\x83 ACCEPTE",
"\x83 LE PREND.",
" CA NE SUFFIT PAS",
" DONNE ENCORE",
" SOIS PLUS GENEREUX",
" POUR TE FAIRE PLAISIR",
"HUM...",
"A PART CA?",
"TIENS DONC",
"CERTES...",
"FINISSONS-EN",
"ASSEZ DE DISCOURS",
"MAIS ENCORE",
"ABREGEONS",
"\x83  VA TE TUER,CHAROGNE!",
"RAVALE TES INSULTES",
"\x83  VA TE BRISER",
"FATIGUE DE VIVRE ?!",
"TES OS NOURRIRONT LES CORBEAUX.",
"TU VAS PAYER",
"\x83  VA T'ECLATER",
"\x83  BOIRA DANS TON CRANE",
"DEFENDS TOI,CLOPORTE!",
"LA VICTOIRE OU LA MORT",
"PAS DE PITIE",
"QUE LES ESPRITS GUIDENT \x83",
"JE SUIS \x83",
"JE ME NOMME \x83",
"MON NOM EST \x83",
"JE M'APPELLE \x83",
"TA VIE !",
"TE TUER!",
"TON COEUR!",
"TON CRANE",
"\x83  VEUT ",
"\x83  DESIRE ",
"\x83  VOUDRAIT ",
"\x83  A BESOIN DE ",
"\x83  T'OFFRE ",
"\x83  TE PROPOSE ",
"\x83  T'ECHANGE ",
"\x83  TE CEDE ",
"\x83  NE REPONDRA PAS",
"...",
"AH AH AH!",
"QUI SAIT?",
" ET TOI,",
"RIEN",
" SAPIENS par Didier et Olivier Guillion.",
" (c) 1986-2017",
"Musiques compos&es par Gilles Soulet.",
"&Il y a cent mille ans,le jour se l%ve...",/*70*/
"BONJOUR ,\x82",
"C'EST DONC TOI ,\x82 ?",
"BONNE CHASSE \x82",
"Des Hurlements de loups.",
"Des Grognements d'ours.",
"Un bruit de Source.",                  /*76*/
"Une odeur de Gibier.",
"Une odeur de Fum&e.",
"Une odeur d'Homme.",
"Il s'enfuit.",
"Tu es &vanoui.",
"LA TRIBU EST FIERE DE TOI,\x82",
"TU ES UN GUERRIER VALEUREUX,\x82",
"LA FORCE DE L'AUROCH SEMBLE ETRE EN TOI!",
"Tu possedes une nouvelle arme.",
"Ton silex est trop mal taill&.",
"Tu n'as pas de silex ...",
"Nom ? ",
"Homme ou Femme (H,F) ?",                    /* 89*/
"Nom de l'Aventure a charger ? ",
"Nom de l'Aventure a sauver ? ",
"Collapse !!!!!",
"La Faim trouble ta vue !",
"La Soif ravine ta gorge !",
"La Fatigue p%se sur ton corps !",
"Tu bois ",
"et remplis toutes tes outres",
"Tu n'as pas soif ",
"...PAS QUESTION !",
"JE TE SUIS.",                             /*100*/
"Tu es beaucoup trop charg& !!!",
"Ta force a impressionn& un Loup.",
"Ton courage a dompt& un Ours.",
"Ton aptitude $ la fuite a s&duit un Lapin.",
"Un Coq t'emboite le pas.",
"Cette version ne marche que sur les ST anglais !",
"Goinfre ! tu n'as ni faim,ni soif !",
"Pas de paresse.Tu n'as pas sommeil.",
"Ta sant& est parfaite,pourquoi te soigner ?",
"Splendide,",
"Ton ami \xAC est hors de combat ...",
"D&sol& \x82,\xAC est Mort !!!",
"Tu as d&ja sauv& une Aventure aujourd'hui...",
"Un nouveau jour se l%ve ...",                  /*114*/
"Allez hop, $ la maison !!!",
"La musique ST,c'est beau ...",
"Que de c&l&brit&s !",
"Petit cadeau amical...",
"Salut vieux !!! ",
"SAPIENS par D et O Guillion.",            /*120*/
"SAPIENS option choisie :",
"Ton Etat :",
"\x82 prends",
"\x82 pose",
"\x82 dit :",
"\x82 donne",
"\x82 regarde",
"\x82 absorbe",
"\x82 se soigne avec",
"\x82 dors",                              /*130*/
"Avec un silex,\x82 fabrique",
"Touche F1 pour de l'aide !",		/* 132*/
"Et le temps passe...",                 /* 133 */
"Tu es touch& par la foudre !!!!!!!!!!", /* 134*/
"Heu...Hummm...Tu es seul...",			/*135*/
"Tu es mort ! La partie est finie...",
"Je cr&& le monde...",
"$END$"
};


char * bkmots_fr[NB_MOTS_BKMOTS]={
"$0", //nomsecondaire
"$1",//  nomjoueur
"$2", //  nomadversaire
"$3", // nomtribu
" CHEF",
" DES PIEDS AGILES",
" DES YEUX MALINS",
" DES BRAS DE ROC",
" DES HYENES FOLLES",
"HOGNOR",
"SL-OOM",
"GRUNGH",
"KROK-G",
" FAIM.",
" SOIF.",
"TA SANTE EST ",
"TU MEURS DE",
"TU AS TRES",
"TU AS UN PEU",
"TU N'AS PAS ",
"EPUISE",
"TRES \x97 ",
" FATIGUE",
"REPOSE",
"TU ES ",
"GRAVEMENT\x9C",
"TRES \x9C ",
" BLESSE",
"EN BONNE SANTE",
" JOURNEE",
"NORD",
"SUD",
"EST",
"OUEST",
"PEAU D",
"VIANDE ",
" AVENTURE",
"MERCI",
"MORT !",
"TRES VULNERABLE",
"VULNERABLE",
"ROBUSTE",
"TRES ROBUSTE",
"$4",//Nomcopain
"ton Compagnon",
"$END$"
};

char *scenme_fr[]={
"Enfant,la Tribu a faim,\r\
            rapporte des vivres.",

"     Les Jeunes ont mang& des baies\r\
               V&n&neuses.\r\
 Il nous faut le rem%de des Yeux-Malins.",

 "Les Bras-de-Roc nous ont lanc& un d&fi.\r\
Va,combats leur Chef et vole-lui sa Hache",

"    Guerrier,pr&pare-toi au combat.\r\
    Les Hy%nes-Folles ont d&rob&\r\
            le BATON SACRE.\r\
Les Esprits de nos Anc%tres Le r&clament!",

"Tes Exploits nuisent $ mon prestige.\r\
Que la Loi de la Force d&signe le Chef!",

"     Voici donc l'Histoire jusqu'alors\r\
       ignor&e de celui qui,il y a cent\r\
   mill&naires devint par la Force de son\r\
         bras le chef de la Tribu des\r\
              Pieds Agiles.\r\
     Il v&cut jusqu'$ l'Age de quarante\r\
Printemps et mourut de la main d'un jeune\r\
               Chasseur.\r\
Bonne Chasse !",

"    Nul ne pourrait survivre $ de si\r\
           terribles blessures...\r\
       Ici s'ach%ve ta Route,Chasseur.",

"'Il y a Cent Mille Ans ...\r\
        ...l'Aventure Humaine Commen#ait'\r\
\r\
   SAPIENS a &t& con#u et r&alis& par\r\
Didier et Olivier Guillion pour THOMSON,\r\
 AMSTRAD CPC,Compatibles et ATARI ST.\r\
Merci $ Jean-Francois Cassan ainsi qu'$\r\
 Gilles Soulet, Jean-Michel Georges et\r\
    Yves Basquet du groupe 'MYRIAD'\r\
pour leurs aides,critiques et conseils.\r\
\r\
       Toulouse, Juillet-Aout 1987.\r\
        re(c) 1996-2017",

"Je suis vaincu,le Baton Sacr& est cach&\r\
   au Sud sous la Protection des Trois\r\
                Totems !.",

"Vaillant Chasseur ton exemplaire n'est\r\
pas enregistr&. Envoie 10 euros aux auteurs\r\
pour aller plus loin dans l'Aventure.",

"     La musique DMA de Sapiens:\r\
\r\
- Nombre de voies: trois\r\
- Pr&cision echantillon: huit bits\r\
- Fr&quence echantillon: neuf kHz\r\
- Temps de musique: quinze mn.\r\
- Temps machine: cinquante pour cent\r\
- Datas: Cent-cinquante Ko\r\
- Circuit digitaliseur: Ferranti ZN\r\
- Montage: Jean-Michel Georges.\r\
- Soft utilis&: STampler\r\
- M&thode employ&e: devinez...",

"Ce soft a &t& d&velopp& en C (Didier)\r\
et en Assembleur (Olivier).\r\
Les d&cors sont de Didier Guillion,\r\
La musique est de Gilles Soulet,\r\
interpr&t&e par 'Digital Music Associates'\r\
sampl&e aux laboratoires 'Myriad Records'.\r\
La mise en sc%ne est de D et O Guillion.\r\
Effets sp&ciaux et trucages: J-M Georges\r\
\r\
  R&alis& de Juillet $ Aout MCMLXXXVII  \r\
             $ Toulouse.\r\
 Copyright D&O Guillion 1986-1996,99,01,05,17",

"Ont collabor& $ la r&alisation de SAPIENS:\r\
\r\
Y.Basquet:Support logistique et magn&tique\r\
J-M Georges: Hard, Interruptions, clavier,\r\
              disquette.\r\
G.Soulet: Composition de la musique.\r\
Guyon James:Traduction en Anglais.\r\
D.Carri%re: Principe du Sampling.\r\
J-F Cassan: Soutien moral et Vodka Orange.\r\
P.Guegan: Inventeur des b&n&fices n&gatifs\r\
Motorola: Adressages indirects index&s\r\
         avec d&placement par rapport $ PC",
"$END$"

    };

char * app_fr []=
{
 "Agilit&",
 "Force",
 "Charisme",
 "Vitalit&",
 "Resistance",
 "Reste" ,
 "Joueur Secondaire,",
 "Joueur Principal, ",
 "Quelle touche pour ",
 "Haut",
 "Bas",
 "Gauche",
 "Droite",
 "Change Arme/Validation",
 "Frappe",
 "Sortie",
 "?",
 "(....)",
 " Loup ",
 " Ours ",
 "Lapin ",
 " Coq  ",
 "$END$"

 };


//EOF