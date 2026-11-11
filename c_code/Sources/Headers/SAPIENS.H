#pragma once
/**************************************************************************
                        SAPIENS.H
        (c) DG le 12-1-96
****************************************************************************/
#define WING 1
#include <acam.h>
#include <ctype.h>
#include <STDIO.H>
#include <string.h>
#include <stdlib.h>
#ifdef PC
    #include <malloc.h>
	#include "ExtendedTextEdit.h"
	#define GetTailleHandle GetHandleSize
	#define NewHandleMem(a,b) NewHandle(a)
	#define NewHandleMemClear(a,b) NewHandleClear(a)
	#define DisposeHandleMem DisposeHandle
	#define NewPtrMemClear(a,b) NewPtrClear(a)
	#define NewPtrMem(a,b) NewPtr(a)
	#define DisposePtrMem DisposePtr
	#define PtrAndHandMem PtrAndHand
#endif
#define NOM_PREF "Sapiens.cfg"
#define FREQ_ORAGE 9990
#define DIST_MIN_FOUDROIE 250
#define NB_NUAGES 100 
#define NB_OISEAUX 20
#define NB_ROCHERS 300
#define NB_HOMMES 0
#define NB_ARBRES 800
#define PRECISION_SILEX 300
#define NB_ETOILES 10000L //10000
#define NB_NUAGES_MAX NB_NUAGES*20L 
#define NB_OISEAUX_MAX NB_OISEAUX*20L 
#define NB_ROCHERS_MAX (50000L)
#define NB_ARBRES_MAX (100000L) 

// Vue laterale
#define NB_ARBRES_LATERAL 3 
#define NB_HERBE_AV_PLAN 20
#define NB_HERBE_ARR_PLAN 15
#define NB_ARBRES_LATERAL_MAX 20
#define NB_MAX_HERBE_AV_PLAN 100
#define NB_MAX_HERBE_AR_PLAN 100
extern long nbNuages,nbEclairs,nbOiseaux,nbRochers,nbHommes,nbArbres,FreqOrage,FreqEclair,DistMinFoudroie,VitesseDeplacementNuages;
extern long nbHerbes;
extern long PathApplication;
extern short volumeApplication;
extern float SapZoom;
#define ECHELLE (2L)
# define INFINI for(;;)
#define TAILLE_STR_MESS_FORMATE 512
#define NB_MAX_MOB 640
typedef struct
    {
    short nbEntrees;        /* Nbres de couleurs */
    short * tabIndex;       /* Indexs dans la palette ecran */
    RGBColor * tabRgb;      /* Couleurs RVB */
    } PalCoul;
/* Pointeur sur les d‚buts de mob */
typedef struct
    {
    short indexPalette; /* PalCoul associ‚e */
    short nbcol;
    short nbligne;
    short vide;         /* Couleur transparente */
    Ptr datas;
    } Mob;
extern Mob TabMob[NB_MAX_MOB];

typedef enum
    {
    NO_MUSIQUE,
    MUSIQUE_HARMONY_QUALITE,
    MUSIQUE_HARMONY_VITESSE,
    MUSIQUE_MIDI,
    MUSIQUE_QUICKTIME,
    NB_TYPE_MUSIQUE
    } TypeMuse;
extern TypeMuse okmuse;
extern Boolean LissagePolice;
#define POLICE_LISSAGE helvetica
#define STYLE_LISSAGE bold
Boolean IsFinDeMusique(void);

extern Fixed RestitutionSampleRate;     /* Frequence de restitution */
extern short RestitutionNbBitsSample;           /* Nbre de bits par sample */
extern Boolean RestitutionStereo;
extern short RestitutionVolume;         /*0 … 127 */
extern short RestitutionNbVoix;         /*1 … NB_VOIX_DIGIT_MAX */
extern Boolean RestitutionQualite;
extern Boolean ModeDigitCompat;
extern long NbMsBufferDigit;
extern short NbBuffSortieDigit;
extern short quelle_musique;

enum {
	FR,GB,ES,SAP_NB_LANGUAGES
};
#define FR 0
#define GB 1
#define ES 2
#define DELTA_LANGUE 1000
#define STR_VRAC 0
#define ALRT_INFO CvtDefaultLangRess(2)
#define ALRT_VERSION 133
#define ALRT_COLLAPSE CvtDefaultLangRess(134)
#define ALRT_CONFIRMATION CvtDefaultLangRess(135)
#define ALRT_ERGO_2_JOUEUR CvtDefaultLangRess(136)
#define ALRT_PERTE_PARTIE CvtDefaultLangRess(1)
#define DIALOG_PREF CvtDefaultLangRess(137)
#define STR_COMMENTAIRE CvtDefaultLangRess(100)
#define RSRC_TEXT_CARTE_ENREG CvtDefaultLangRess(0)
#define MENU_BAR CvtDefaultLangRess(0)



#define SIZELISTE 8
/*definition de structures*/
/*definitions des EQU*/
#define SAGAIE 23
#define dstsag 10
#define dstfra 4
enum
    {
    JOUEUR2_AUTONOME=0,
    JOUEUR2_CLAVIER=1
    };
#define TAILLE_MAX_NOM_USER 80
#define TAILLE_MAX_NOM 32

typedef struct
    {
    short index;    /* Index de la couleur */
    RGBColor rgb;   /* Valeur rgb de la couleur */
    } IndexColor,*IndexColorPtr,**IndexColorHandle;


extern short ModeJoueur2;


extern short distsp,presence;
extern unsigned char fin,sensmob,nummob;
extern  char * ptranim;
/*tableaux*/

#define NB_MOTS_BKMOTS 46
 
/*hauteur: 0 debout ,1 accroupis ,2 … genoux, 3 plat ventre
  sens : 0<= ,1 =>
  vitesse 0:marche           1:              2:arret
*/

        struct sap {
        short colonne;
        unsigned char   ligne,sens,vitesse,objet,hauteur,
                        action,vie,force,agressivite,souffle,deltah,
                        blockimpact,ligneimpact,colimpact,forcehache;
        short           coeur,cptrsouffle,nbresag,animal,linsag,colsag,snssag,
                        fuite,flg_mort;
        short            poslcoeur,posccoeur;
        char   *sapanim [15];
        unsigned char   cross[8];
        struct sap *    adversaire;
        struct liste *  possession;
                    };
struct reference
    {
    char nomref[64];/*libelle de la reference*/
	unsigned char param1;/*premier parametre*/
    unsigned char param2;/*deuxieme parametre*/
    };
typedef enum
	{
	REFER0,
	REFER1,
	REFER2,
	REFER3
	}TypeReference;
struct liste
    {
   // struct reference *refer;        /*pointe sur le tableau des references*/
   TypeReference typeRef;
    unsigned char taille;           /*taille de la liste*/
    unsigned char maxtaille;        /*taille maximum de la liste*/
    unsigned char elements [30];    /*elements de la liste*/
    };


struct cellule {
                void * adresse;
                unsigned char taille;
                };

struct brin {
             unsigned short type,lin,col,sens;
             };
#define NBREVTHEURE 37
#define NB_LIGNES_CIEL (128)
#define NB_LIGNES_TERRE (36*2)
#define NB_DEGRADE_DISTANCE (NB_LIGNES_TERRE)
struct h_chron {
               short h;
               short q;/* 0=modif palhaute 1=modif pal ciel */
               short pal[32];   /* Palette ST Associ‚e */
               IndexColor * TabCouleursCiel;
               short * * TabPaletteDistance;
               };

#define NBREVTDESTINE 40
struct evt {
           unsigned char
                           type,posx,posy,distmax,rnd,evt,message;
           };

# define gemnbrecad 4
# define geminterv  4
# define gemaction  19,1,1,14
# define gemvue 1,6,12,52
struct gemcadre {
                short ligne,colon,haut,larg;
                };



# define BASEMOB 0  /*152*/
/*l'Ign variable permet de reafficher la meme forme dilat‚e ou compress‚e...*/

typedef struct Forme
             {
             unsigned char nbrdisk;         /* 0 : */
             unsigned char relief;          /* 1 : */
             unsigned short hauteur;        /* 2*/
             unsigned short rayon;          /* 4*/
             unsigned short dskrayon[16];     /* 6*/
             char dskdelta[32];             /* 38*/
             short num_mob;                 /* Affichage de la carte */
             short numMob3d;                    /* Numero de mob pour la 3d */
             }Forme,*FormePtr,**FormeHandle;

typedef struct Mont {
            struct Mont * chaine;
            unsigned short distance;
            unsigned short posx;
            unsigned short posy;
            unsigned short ign;
            FormePtr ptrforme;
            } Mont,*MontPtr,**MontHandle;
extern MontHandle aigoual;
extern long nbreobj;

enum
    {
    VUE_AUCUNE=0,
    VUE_PAYSAGE=1,
    VUE_LATERALE=2
    };
enum
    {
    FORME_BUSTE=3,
    FORME_TETE=4,
    FORME_ARME=5
    };
/*****************************Externes ***********************************/

extern short langue;

extern short sposx,sposy;
extern unsigned char lpos,stock1,stock2,criter,lastmen,rega;

extern char who;

extern char gabu;
extern  char  nomjoueur[];
extern  char  nomadversaire[];
extern char nomtribu[];
extern char nomsecondaire[];
extern  char  nomcopain[];
extern  char  poschef[];

extern  char * traps[7];

char * GetStrBkMot(short car);



extern unsigned short tenerv[4],enetoi[4];


extern unsigned char HOMME,FEMME,OUI,
                listdep[],
                maxparam[2][6],
                agrdep[],
                tagress[];
extern char  smenu[];
extern short quel_clav;
extern short dzeta,ldzeta,colaff,linaff,type_synthe;
extern unsigned char    profond[],
                tabprf[],
                tobje[4][4];
extern unsigned short enerv,xj,yj,zj,derxj,deryj,kk,xc,yc,rc,
                        dismin,dist,heure,metpos,angsol,
                        jour_act,jour_dsk;

extern unsigned char
                        zoom,
                        tabclav1[],
                        tabclav2[],
                        param[6];
extern unsigned char    soif,fatig,faim,mort,bless,
                        echan,vlchan,flefba,predil,fdialo,fpress,chef,
                        tribu,scenar,flcomb,agress,entvie,sexe,
                        dquest,dervue,valsoi,source,minute,sapvit,viecopain,
                        sap3clav,cptsoi,valsoi,cptagil,finjou,
                        agilite,force,resistance,charisme,
                        vie,flagbadi,ciel,terre,freq,
                        flgcharism;
extern char          scenpr;

extern         struct sap sap1;
extern         struct sap sap2;
extern         struct sap sap3;
extern         struct sap sap4;


extern         struct sap * peuple[];
extern         struct liste
/*actions*/             liste0,
/*objets portes*/       liste1,
/*phrases*/             liste2,
 /*vues*/               liste3,
/*objets au sol*/       liste4,
/*temps*/               liste5,
/*fabrique*/            liste6,
/*status*/              liste7,
/*gestion*/             liste8,
/*objets sap2*/         list10,
                        list11,
                        list12,
                        listbidon,
                        zoneu,
                        cross;

extern      char hautlin[],
             tabinp[],
             hautou[],
             tribforce[],
             tribvie[],
             bestlin[],
             bestvie[],
             bestagress[],
             besthaut[],
             present[],
             qtviande[];

extern struct cellule soma[];
extern         char /*marche*/
        anjm1[],
        anjm2[],
        anjm3[],
        anacr1[],
        anacr2[],
        angen1[],
        angen2[],
        anaga1[],
        anaga2[],
        anrga1[],
        anrga2[],
        anbras[],
        anbra1[],
        anbra2[],
        anhac1[],
        ansag1[],
        anpoin[],
        anhach[],
        ansag[],
        anbust[],
        anbst1[],
        antete[],
        antet1[],
        antet2[],
        anarm1[],
        anarm3[],
        anarm4[],
        aneutr[],
        anlou1[],
        anlou2[],
        anlap1[],
        anlap2[],
        anlap3[];
extern   char *debban [13][14];


extern struct h_chron tabheur[NBREVTHEURE];
extern short sposx,sposy,bouton,dsposx,dsposy,handle;
extern Boolean  isScrollMouse;


extern char tabher[6];
extern short *  palhaute, * palbasse, * pal_chron;
extern short * ciel_degr;
extern short pos_evt;
extern short pal1[32],pal2[32],pal_zero[32];
extern struct evt  destine[NBREVTDESTINE];
extern short  posxf,posyf,ralent;
extern  unsigned short
                        menu,/*menu en cours*/
                        clin,/*debut du cadre en ligne64*/
                        ccol,/*debut du cadre en colonne 64*/
                        colcdr,/*nbre de colonnes40 du cadre*/
                        cnlin,/*nbre de lignes du cadre*/
                        cncol,/*nbre de colonnes du cadre*/
                        /*tprm est fixe … 2*/
                        coulcur;/*couleur d'affichage du curseur*/

extern                        short  poscur; /*position curseur*/
                        /**/
extern                        struct liste * stdlist;/*liste std pour ce cadre*/
                        /**/
extern unsigned short
                        curpos,/*position en memoire sur ecran du curseur*/
                        pcadre,/*position en memoire sur ecran du cadre*/
                        coulfd;/*couleur du fond du cadre*/
extern struct gemcadre gemzoom[gemnbrecad];
extern         char
                tclin[9],
                tccol[9],
                tcolcdr[9],
                tcnlin[9],
                tcncol[9],
                tlpos[9],
                tcoulcur[9];
extern           short tposcur[9];
extern           struct liste * tstdlist[9] ;
extern short sposx,sposy,dsposx,dsposy,dicone,s_menu,icone,bouton,handle,s_action;
extern char smenu[];
extern char maced[];
extern CGrafPtr Ecran1,Ecran2,debecr;
extern WindowPtr mainWindow;
extern unsigned char TabCvtCoul[0x778];
extern short toucheClav;
extern short tabcoul[];
extern CGrafPtr Ecran,debecr,BuffMouse;
extern Rect rMess;
void AcamErreur(char * format,...);
#ifdef MAC
    int stricmp( const char * c1, const char * c2);
#endif
CGrafPtr AcamCreatePortOffscreen(short taillex,short tailley,short prof);
void AcamClosePortOffscreen(CGrafPtr myGrafPort);
short AcamInitOffscreenSystem(Rect * r);
unsigned char far * AcamGetPtrDataOffscreen(BitMap * p);
unsigned short AcamGetRowbytesOffscreen(BitMap * p);
void  AcamCopyBitsOffscreen(BitMap * scrMap,BitMap * destMap,Rect * srcRect,Rect * destRect,short mode,RgnHandle rgn);
unsigned char far * AcamGetPtrDataOffscreen(BitMap * p);
unsigned short AcamGetRowbytesOffscreen(BitMap * p);
void  AcamCopyBitsOffscreen(BitMap * scrMap,BitMap * destMap,Rect * srcRect,Rect * destRect,short mode,RgnHandle rgn);
void AcamSetPortOffscreen(CGrafPtr myGrafPort);
void AcamGetPortOffscreen(CGrafPtr * myGrafPort);
void EnvoiePaquet(short car1,short car2,short car3);
short joystick(void);
void DoAide(void);
extern short LastObsv;
Boolean JoueMusiqueHarmony(char * nom,Handle donneesXML,char * listeMesures,short indexAbc,char * organe,
			short volume,
			short tempo,
			short decalageDemiTon,
			Boolean isMetronome);
void InitMusicFichier(char * nomFichier);
void IncMusicFichier(void);
void TerminateMusicFichier(void);
void SortieMusiqueFichier(short quelle);



extern Point ptEcrCar;
extern Boolean MusiqueEnCours;
extern Boolean IsBruitages;
/* Liens avec les mobs */
extern short mobBambou,mobFace,mobFeuille,mobFleches,mobSapmob,mobAvplan,mobAvplanSpec,
             mobSilex,mob3D;
extern short Fonte14,Fonte10;
extern short nbMobHerbes,nbAvplanSpec;
extern unsigned char TabCvtCoulOmbre[256];
#define MOB_FLECHE mobSapmob+49
#define MOB_IMPACT mobSapmob+32
#define MOB_SANG_A_TERRE mobSapmob+87
#define MOB_EMPREINTE mobSapmob+20
#define MOB_CORPS mobSapmob+6
#define MOB_CHEMIN mobSapmob+83
#define MOB_ANIMAL_MORT mobSapmob+109
#define MOB_PICT mobSilex+1
#define MOB_GUEPARD mobSilex+12
#define MOB_SILEX_SAGAIE mobSilex+13
#define MOB_SILEX_HACHE mobSilex+14
#define MOB_SILEX_CASSE mobSilex+15
#define MOB_MORT mobSapmob+114

#define MOB_SOLEIL mobFace+28

#define MOB_FEUILLE mobFeuille+1
#define MOB_HERBE mobAvplan+1

#define MOB_CADRE mobBambou+1
#define UP mobBambou+18
#define DOWN mobBambou+19
#define INCONNU mobBambou+20
#define SELECT mobBambou+21
#define DEBFLECH mobFleches+1
#define DEBCRANE mobFace+13
#define DEBTETE mobFace+1
#define DEBBEST mobFace+24
#define MOB_DESIGNE mobFleches+17
#define MOB_AIDE mobBambou+27

/************************************************************************************************
                            Sp‚cifiques au compilateur
***********************************************************************************************/
//#ifdef __MWERKS__
//    #define PtoCstr p2cstr
//#endif

#ifdef PC
typedef enum
    {
    W95_DIGIT_OUT,
    W95_DIGIT_IN,
    W95_MIDI_OUT1,
    W95_MIDI_OUT2,
    W95_MIDI_OUT3,
    W95_MIDI_IN,
    W95_NB_PERIPHS
    } TypePeriphW95;
#endif
#include "saprot.h"
#include "CarboniseProt.h"
void RestorePath(void);
void PreservePath(void);
OSErr FixeVolPath(short volRefNum,long path);
extern volatile unsigned long sapiensTimer; // Timer 11 millisec
#define PtrAndHandMem PtrAndHand
#ifndef iPrAbort
	#define iPrAbort 0x0080
#endif
#if TARGET_API_MAC_CARBON>0
	#define SystemTask()
#endif
/* EOF */
