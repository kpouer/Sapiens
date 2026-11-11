/****************************Version 2. 02*******************************
                        VARIABLES GLOBALES
                Mercredi 10 D‚cembre 1986
***********************************************************************/
#include "sapiens.h"
/*d‚finitions des variables globales*/
unsigned char lpos=1,stock1,stock2,criter=0,lastmen=255,rega;

char who;

short dzeta,ldzeta,colaff=1,linaff=1,type_synthe=0;
unsigned char    profond[]={10,8,6,4,2},
                tabprf[]={25,33,34,35},/*objet prefere*/
                tobje[4][4]={  /*objets de la tribu*/
                             23,26,28,38,
                             21,25,29,39,
                             36,27,3,23,
                             25,24,26,23
                           };

unsigned short
                tenerv[4]={256,4864,25600,40960L},/*enervement tribu -etranger*/
                enetoi[4]={0,0,0,0};/*enerv contre toi*/

TypeMuse okmuse=MUSIQUE_HARMONY_QUALITE;

unsigned char
                HOMME='H',FEMME='F',OUI='O',
                listdep[]={21,23,23,27,37,36,19,19,25},
                maxparam[2][6]=
                        {240,180,254,200,130,254,
                         200,254,130,200,200,254},
                agrdep[]={110,150,200,201},
                tagress[]={110,150,200,201};/*agressivite*/

                        /*donnees du menu en cours*/
                        /**************************/
unsigned short
                        menu=0,/*menu en cours*/
                        clin=1,/*debut du cadre en ligne64*/
                        ccol=1,/*debut du cadre en colonne 64*/
                        colcdr=10,/*nbre de colonnes40 du cadre*/
                        cnlin=10,/*nbre de lignes du cadre*/
                        cncol=10,/*nbre de colonnes du cadre*/
                        /*tprm est fixe … 2*/
                        coulcur=1;/*couleur d'affichage du curseur*/

                        short  poscur; /*position curseur*/
                        /**/
                        struct liste *
                        stdlist;/*liste std pour ce cadre*/
                        /**/
                        unsigned short
                        curpos,/*position en memoire sur ecran du curseur*/
                        pcadre,/*position en memoire sur ecran du cadre*/
                        coulfd;/*couleur du fond du cadre*/
/*------- definition pour les gestions de cadre ------*/
struct gemcadre gemzoom[gemnbrecad];


                        struct liste
/*actions*/             liste0={REFER0,11,20,{11,10,5,4,3,6,1,2,7,8,9}},
/*objets portes*/       liste1={REFER1,16,30,{21,23,23,27,19,19,19,8,9,10,11,
                                12,13,14,15,16,17,18,19,20}},
/*phrases*/             liste2={REFER2,28,0,{1,2,3,4,5,6,7,8,9,10,11,12,13,14,
                               15,16,17,18,19,20,21,22,23,24,25,26,27,28}},
 /*vues*/               liste3={REFER3,4,0,{1,2,3,4}},
/*objets au sol*/       liste4={REFER1,2,30,{2,3,4}},
/*temps*/               liste5={REFER3,3,0,{5,6,7}},
/*fabrique*/            liste6={REFER1,2,0,{23,21}},
/*status*/              liste7={REFER3,5,0,{8,9,10,11,12}},
/*gestion*/             liste8={REFER3,5,0,
                                {19,/* Informations */
                                 13,/* Charge aventure */
                                 14,/* Sauve aventure */
                                 15,/* personnage */
                                 26 /* Pr‚f‚rences */
                                 }},
/*objets sap2*/         list10={REFER3,0,20},
                        list11={REFER3,0,20},
                        list12={REFER3,0,20},

                        listbidon={REFER0,0,11},
                        zoneu={REFER1,0,30},/*liste de travail pour critere*/
                        cross={REFER1,0,30};

/***************************************************************************/
/*              Inits des tableaux des menus                               */
        char
                tclin[9]=         {18,19,18,18,19,18,19,20,18},
                tccol[9]=         { 4,22,23,28,26,31,33,23,26},
                tcolcdr[9]=       {11,17,17,18,12,12,12,17,13},
                tcnlin[9]=        { 7, 7, 8, 6, 6, 3, 2, 5, 6},
                tcncol[9]=        {13,16,28,12,16,16,9,26,17},
                tlpos[9]=         {1,1,1,1,1,1,1,1,1},
                tcoulcur[9]=      {9,9,9,9,9,9,9,9,9};
           short tposcur[9]=      {6082,5787,6121,5795,6432,
                                   6118,6441,6445,5792 };

           struct liste *
                tstdlist[9]=      {&liste0,&liste1,&liste2,&liste3,&liste4,
                                   &liste5,&liste6,&liste7,&liste8};
/*------ decouper ici----------*/
/*Variables Globales*/
        short distsp,presence;
        unsigned char
                fin=0,sensmob,nummob;
         char * ptranim;

/*tableaux*/

        char hautlin[]={10,15,25,30},
             tabinp[]={3,5,20,30},
             hautou[]={10,20,27,35},
             tribforce[]={15,15,25,20},
             tribvie[]={160,150,200,180},
             bestlin[]={2,-35,2,2},
             bestvie[]={70,200,1,1},
             bestagress[]={200,180,200,180},
             besthaut[]={2,1,3,3},
             present[]={0,0,0,0},
             qtviande[]={1,2,4,8};

        char /*marche*/
        anjm1[]={0x82,0x81,0,26,0,0x80,-2,0x81,1,26,0,0x81,2,26,0,
                0x80,-1,0x81,3,26,0,0x81,4,26,0,0x80,0,
      /*anjm1b =anjm1+27*/
                0x81,5,26,0,0x81,6,26,0,0x81,7,26,0,0x81,8,26,0,0x81
                ,9,26,0,0xFF},
        /*arret*/
        anjm2[]={0x80,0,0,26,0,0xFF},
        anjm3[]={4,26,0,0xFF},
        anacr1[]={0x80,2,15,27,1,0xFF},
        anacr2[]={16,27,2,0xFF},
        angen1[]={0x80,13,17,19,0,0xFF},
        angen2[]={18,24,0,0xFF},
        anaga1[]={15,27,1,0x80,-1,8,26,0,0x81,0x80,-2,9,26,0,
                  0x81,0x80,-1,0,26,0,0x80,0,0x82,0x86,127,0xFF},
        anaga2[]={16,27,2,5,26,0,6,26,0,6,26,0,0x82,0xff},
        anrga1[]={15,27,1,0x80,-1,0,26,0,0x85,0x80,-2,9,26,0,
                  0x80,-1,0x85,8,26,0,0x80,0,0x82,0x86,127,0xff},
        anrga2[]={16,27,2,6,26,0,6,26,0,5,26,0,0x82,0xff},
        anbras[]={31,12,0,32,12,0,33,12,0,34,12,0,35,12,0,
        /*anbrad = anbras+15*/
                  35,12,0,34,12,0,33,12,0,32,12,0,31,12,0,0xff},
        anbra1[]={34,12,0,0xff},
        anbra2[]={33,12,0,0xff},
        anhac1[]={38,12,0,0xff},
        ansag1[]={36,9,2,0xff},
        anpoin[]={34,12,0,107,12,-1,0x87,3,37,12,1,0x86,252,0xff},
        anhach[]={38,12,0,39,12,-1,40,12,-1,41,14,0,39,12,-1,0x86,
                  252,0xff},
        ansag[]={36,9,2,37,9,1,0x86,252,0x88,0xff},
        anbust[]={120,12,1,0xff},
        anbst1[]={0x80,18,121,12,1,121,12,1,121,12,1,121,12,1,0x84,0xff},
        antete[]={53,0,2,53,0,2,54,0,2,54,0,2,0xff},
        antet1[]={53,0,2,54,0,2,0xff},
        antet2[]={53,0,2,0xff},
        anarm1[]={45,7,-2,47,8,-3,0x87,3,49,6,-4,51,10,-1,
                  45,8,-2,0x86,252,0xff},
        anarm3[]={45,7,-2,0xff},
        anarm4[]={43,11,-1,0xff},
        aneutr[]={0x83,0xff},
        anlou1[]={96,26,0,255},
        anlou2[]={95,26,0,0x81,96,26,0,0x81,
                  97,26,0,0x81,98,26,0,0x81,0x87,1,255},
        anlap1[]={87,35,0,87,35,0,255},
        anlap2[]={87,35,0,0x81,88,35,0,89,35,0,0x81,255},
        anlap3[]={90,28,2,90,28,2,90,28,2,90,28,2,255};
         /*blocks animation*/
       char *debban [13][14]=
   /*marche*/          {anjm1,anjm1,anbras,anbras,anjm1+27,anjm1,anbust,anbust
                       ,antete,antete,0,0,anbras+15,anbras,

  /*arret*/             anjm2,anjm2,anbra1,anbra1,anjm3,anjm3,anbust,anbust
                       ,antet2,antet2,0,0,anbra2,anbra2,

 /*frappe du poing*/    0,0,0,0,0,0,0,
                        0,0,0,aneutr,0,anpoin,0,

/*frappe avec hache*/   0,0,0,0,0,0,0
                        ,0,0,0,anarm1,0,anhach,0,

/*frappe avec sagaie*/  0,0,0,0,0,0,0,
                        0,0,0,aneutr,aneutr,ansag,anbra2,

/*rien en main*/        0,0,0,0,0,0,0,
                        0,0,0,aneutr,aneutr,0,0,

/*hache en main*/       0,0,0,0,0,0,0,
                        0,0,0,anarm3,anarm3,anhac1,anhac1,

/*sagaie en main*/      0,0,0,0,0,0,0,
                        0,0,0,anarm4,anarm4,ansag1,ansag1,

/*accroupi*/            anacr1,anacr1,anbra1,anbra1,anacr2,anacr2,0,
                        0,antet2,antet2,0,0,anbra2,anbra2,

/*a genoux*/            angen1,angen1,anbra1,anbra1,angen2,angen2,anbust,
                        anbust,0,0,0,0,0,0,
/*plat ventre*/
                        aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,anbst1,
                        anbust,0,0,0,0,0,0,

/*en garde avance*/
                        anaga1,0,0,0,anaga2,0,0,
                        0,antet1,0,0,0,0,0,

/*en garde recule*/
                        anrga1,0,0,0,anrga2,0,0,
                        0,antet1,0,0,0,0,0};


        struct sap sap1=
                        {2,32,0,2,0,0,0,240,0,128,255,0,0,0,0,10,71,1,5,
                        37,0,0,255,0,0,
                        8,5, /* ligne colonne position du coeur */
                       { aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr
                        ,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,
                        ( char *)(0L)},
                       { 0,0,0,0,0,0,0,0},&sap1,&liste1
                        };


        struct sap sap2=
                        {2,54,1,2,0,0,0,240,0,128,255,0,0,0,0,10,71,1,5,
                        37,0,0,255,0,0,
                        8,296, /* ligne colonne position du coeur */
                       { aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr
                        ,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,
                        ( char *)(0L)},
                       { 0,0,0,0,0,0,0,0},&sap2,&list10
                        };

        struct sap sap3=
                        {2,54,1,2,0,0,0,240,0,128,255,0,0,0,0,10,71,1,5,
                        37,0,0,255,0,0,
                        56,5, /* ligne colonne position du coeur */
                       { aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr
                        ,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,
                        ( char *)(0L)},
                       { 0,0,0,0,0,0,0,0},&sap2,&list11
                        };
        struct sap sap4=
                        {2,54,1,2,0,0,0,240,0,128,255,0,0,0,0,10,71,1,5,
                        37,0,0,255,0,0,
                        56,296,/* ligne colonne position du coeur */
                       { aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr
                        ,aneutr,aneutr,aneutr,aneutr,aneutr,aneutr,
                        ( char *)(0L)},
                       { 0,0,0,0,0,0,0,0},&sap2,&list12
                        };

/*--------------------------- EVENEMENTS ----------------------------*/
struct evt  destine[NBREVTDESTINE]=
/*chefs*/
{
// Type, posx et y, distmax, Rnd, evt, message
{1,0x58,0x54,1,245,0x81,80},
{1,155,56,1,245,0x82,80},
{1,47,207,1,245,0x83,80},
{1,195,170,1,245,0x84,80},
/*hommes*/
{0,88,81,20,10,1,79},
{0,155,56,30,10,2,79},
{0,47,207,60,20,3,79},
{0,195,170,40,25,4,79},
/*sources*/
{0,82,78,1,255,0,76},
{0,48,111,1,255,0,76},
{0,187,164 ,1,255,0,76},
{0,8,8,1,255,0,76},
{0,152,3,1,255,0,76},
{0,63,164 ,1,255,0,76},
{0,125,63,1,255,0,76},
{0,176,99,1,255,0,76},
{0,24,176,1,255,0,76},
{0,195,196,1,255,0,76},
{0,95,74,1,255,0,76},/*Nlles sources */
{0,128,68,1,255,0,76},/*Nlles sources */
{0,157,56,1,255,0,76},/*Nlles sources */
{0,171,66,1,255,0,76},/*Nlles sources */
/*lapins*/
{0,9,20,40,20,7,77},
{0,20,194,20,10,8,77},
{0,80,84,10,10,7,77},
{1,110,181,10,128,7,77},
{1,110,33,10,128,8,77},
{1,179,234,10,128,7,77},
{1,195,147,10,128,8,77},
{1,202,23,5,128,8,77},
{1,236,154,5,128,8,77},
/*loups*/
{0,50,134,20,30,5,74},
{0,76,214,20,30,5,74},
{0,135,60,20,50,5,74},
{0,236,134,20,30,5,74},
{0,195,170,10,40,5,74},
/*ours*/
{0,30,54,10,10,6,75},
{0,97,140,10,10,6,75},
{0,139,221,10,15,6,75},
{0,185,87,5,20,6,75}
};

short  posxf,posyf,ralent;

unsigned char           soif,fatig,faim,mort,bless,
                        echan,vlchan,flefba,predil,fdialo,fpress,chef,
                        tribu,scenar,flcomb,agress,entvie,sexe=1,
                        dquest,dervue,valsoi,source,minute,sapvit,viecopain,
                        sap3clav,cptsoi;
unsigned char           cptagil,finjou=0,
                        agilite=128,force=128,resistance=128,charisme=128,
                        vie=128,flagbadi,ciel=221,terre=136,freq=10,
                        flgcharism;
char          scenpr;

unsigned short            enerv,xj,yj,zj,derxj,deryj,bidon,kk,
                        dismin=1,dist=1,heure=1,metpos=70,angsol=0,
                        jour_act=0,jour_dsk=0;

unsigned char
                        zoom=64,
                        tabclav1[]={72,80,75,77,28,57,1 },
                        tabclav2[]={18,45,31,32,34,33,1},
                        param[6]={128,128,128,128,128,0};

        struct sap * peuple[]={&sap1,&sap2,&sap3,&sap4};

struct cellule soma[]={
                { &soif ,1},{ &fatig,1},{ &faim ,1},{ &mort ,1},
                { &bless,1},{ &zoom ,1},{ &agilite,1},{ &force,1},
                { &resistance,1},{ &charisme,1},{ &vie,1},{ &sexe,1},
                { &viecopain,1},{ sap1.cross,8},{ sap3.cross,8},
                { &scenar,1 },{ &scenpr,1 },{&ciel,1},{&terre,1},
                { &liste1.taille,1},{ liste1.elements,30},
                { &tagress[0],1 },{ &enetoi[0],2 },
                { &tagress[1],1 },{ &enetoi[1],2 },
                { &tagress[2],1 },{ &enetoi[2],2 },
                { &tagress[3],1 },{ &enetoi[3],2 },
                { &xj,2},{ &yj,2},{ &dzeta,2},{ &heure,2},{ &sap3.vie,1},
                { nomjoueur,8},{ nomcopain,6},{ &present[2],1},{ &jour_act,2},
                { param, 6 },{ tabclav1,7 },{ tabclav2,7 },
                { &liste8.elements[4],1},{ &liste8.elements[5],1},
                { &bidon,1},
				{ &FreqOrage,4},
				{ &DistMinFoudroie,4},
				{ &nbNuages,4},
				{ &nbOiseaux,4},
				{ &nbRochers,4},
				{ &nbHommes,4},
				{ &nbArbres,4},
				{ &nbHerbes,4},
				{ &VitesseDeplacementNuages,4},
                { 0000,0}
                };
                /* Heure,ciel,terre*/

struct h_chron tabheur[NBREVTHEURE]={
                  {0 ,0,{0x777,0x000,0x700,0x221,
                       0x332,0x443,0x554,0x420,
                       0x532,0x641,0x444,0x030,
                       0x040,0x730,0x230,0x005}},

                  {1,1,{0x003,0x003,0x003,0x003,
                         0x003,0x004,0x004,0x005}},

                  {3,1,{0x003,0x003,0x003,0x003,
                         0x103,0x103,0x203,0x203}},

                  {5  ,0,{0x777,0x000,0x700,0x221,
                       0x332,0x443,0x554,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x730,0x340,0x005}},

                  {5,1,{0x013,0x113,0x213,0x212,
                         0x212,0x211,0x301,0x301}},

                  {8,1,{0x013,0x113,0x213,0x212,
                         0x211,0x301,0x312,0x401}},


                  {10,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x730,0x340,0x005}},

                  {12,1,{0x014,0x114,0x214,0x213,
                         0x212,0x301,0x401,0x501}},

                  {14,1,{0x024,0x114,0x223,0x323,
                         0x313,0x402,0x501,0x600}},


                  {15,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x740,0x440,0x005}},

                  {17,1,{0x024,0x124,0x223,0x322,
                         0x322,0x422,0x522,0x622}},

                  {18,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x750,0x440,0x005}},

                  {19,1,{0x025,0x124,0x224,0x324,
                         0x424,0x434,0x533,0x633}},


                  {20,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x760,0x450,0x005}},

                  {21,1,{0x025,0x035,0x235,0x335,
                         0x435,0x445,0x545,0x645}},

                  {24,1,{0x035,0x135,0x235,0x335,
                         0x435,0x435,0x535,0x535}},

                  {25,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x770,0x440,0x005}},

                  {27,1,{0x035,0x035,0x135,0x235,
                         0x335,0x335,0x435,0x435}},

                  {31,1,{0x036,0x036,0x136,0x236,
                         0x336,0x336,0x436,0x436}},

                  {35,1,{0x036,0x036,0x136,0x236,
                         0x246,0x346,0x446,0x446}},

                  {39,1,{0x046,0x046,0x046,0x046,
                         0x056,0x056,0x356,0x356}},

                  {45,1,{0x046,0x046,0x046,0x046,
                         0x047,0x047,0x057,0x057}},

                  {50,1,{0x047,0x047,0x047,0x047,
                         0x057,0x057,0x067,0x067}},


                  {468,1,{0x014,0x025,0x025,0x533,
                         0x434,0x750,0x753,0x754}},

                  {470,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x760,0x440,0x005}},

                  {474,1,{0x014,0x014,0x025,0x536,
                         0x533,0x740,0x750,0x753}},

                  {480,1,{0x014,0x014,0x014,0x523,
                         0x630,0x730,0x740,0x750}},

                  {481,0,{0x777,0x000,0x700,0x320,
                       0x430,0x540,0x650,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x750,0x340,0x005}},

                  {486,1,{0x014,0x014,0x014,0x304,
                         0x523,0x630,0x730,0x740}},

                  {490 ,0,{0x777,0x000,0x700,0x221,
                       0x332,0x443,0x554,0x420,
                       0x532,0x641,0x444,0x040,
                       0x050,0x740,0x340,0x005}},

                  {492,1,{0x014,0x014,0x014,0x205,
                         0x304,0x523,0x630,0x730}},

                  {498,1,{0x003,0x014,0x014,0x005,
                         0x205,0x304,0x523,0x630}},

                  {500 ,0,{0x777,0x000,0x700,0x221,
                       0x332,0x443,0x554,0x420,
                       0x532,0x641,0x444,0x020,
                       0x030,0x666,0x230,0x005}},

                  {504,1,{0x003,0x003,0x003,0x004,
                         0x005,0x205,0x304,0x523}},

                  {510,1,{0x003,0x3,0x3,0x3,
                         0x4,0x5,0x205,0x304}},

                  {516,1,{0x1,0x1,0x1,0x2,
                         0x3,0x4,0x5,0x205}},

                  {1020 ,0,{0x777,0x000,0x700,0x221,
                       0x332,0x443,0x554,0x420,
                       0x532,0x641,0x444,0x030,
                       0x040,0x222,0x230,0x005}}

                };

short *  palhaute, * palbasse, * pal_chron;
short * ciel_degr;
short pos_evt;


short sposx,sposy,dsposx,dsposy,dicone,s_menu,icone,bouton,handle,s_action;
char smenu[]={0,-1};

/********************** END OF DATAS AND VARIABLES **************/
