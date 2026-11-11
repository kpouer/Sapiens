/***********************************************************************
                        Module  # 5 gestion et autres
                           D G le 30 Janvier 1987
***********************************************************************/
#include "sapiens.h"

#define NB_LIGNE_SILEX 200
#define NB_COL_SILEX 300
#define COULEUR_FOND_NUCLEUS 0x000
static unsigned char          forsy1[100]={0,0,0,0,1,1,2,3,4,4,
                           6,6,7,7,8,8,9,9,10,10,
                           11,11,12,12,13,13,14,14,15,15,
                           16,16,16,16,17,17,18,18,18,18,
                           19,19,20,20,20,20,21,21,22,22,
                           22,22,22,22,23,23,23,23,23,23,
                           24,24,24,24,24,24,24,24,23,23,
                           23,23,23,23,22,22,22,22,21,21,
                           20,19,18,17,16,15,14,12,10,8,
                           6,5,3,1,0,0,0,0,0,0},
              forsy2[100]={ 0, 0, 0, 2, 4, 5, 6, 7, 8, 8,
                            8, 9,10,10,11,11,12,13,14,14,
                           15,15,16,16,17,17,18,18,19,19,
                           20,20,21,21,22,22,23,23,24,24,
                           25,25,26,26,27,27,28,28,28,28,
                           29,29,29,29,30,30,30,30,30,30,
                           30,30,31,31,32,32,32,32,32,32,
                           32,32,32,32,32,32,32,32,32,32,
                           32,32,31,31,30,29,28,26,24,22,
                           20,16,12, 6, 0, 0, 0, 0, 0, 0},
              formonde[100]={50,50,50,50,49,49,49,49,48,48,
                             47,47,47,46,46,46,46,45,45,44,
                             43,43,42,42,41,40,39,39,38,38,
                             37,37,36,35,34,34,33,33,32,32,
                             31,30,29,28,27,26,25,25,24,24,
                             23,23,22,21,20,20,19,19,18,18,
                             17,17,16,16,15,15,15,15,14,14,
                             13,13,12,12,11,11,10,10, 9, 9,
                              8, 8, 7, 7, 6, 6, 5, 5, 4, 4,
                              3, 3, 2, 2, 1, 1, 0, 0, 0, 0};


static unsigned char * forsyl;

static Rect AireJaugeForce={(4+87-254/3)*ECHELLE,(40)*ECHELLE,(4+87)*ECHELLE,(48)*ECHELLE};
static Rect AireSuite={100-20,320-16-32-8,100-20+32,320-16-8};
#define NB_COUL_SILEX 8
/*
short pal3[32]={     0x777,
                   0x070,
                   0x556,
                   0x666,
                    0x665,
                    0x664,
                    0x663,
                    0x521,
                     0x555,
                     0x532,
                     0x432,
                     0x541,
                     0x542,
                     0x543,
                     0x544,
                     0x000};
static unsigned epaiss[75]={0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,
                          2,2,2,3,3,3,4,4,4,5,
                          5,5,5,6,6,6,6,6,7,7,
                          9,9,9,10,10,11,12,13,14,14};
*/
static RGBColor TabCoulSilexRGB[NB_COUL_SILEX]=
        {
        /* Couleurs du nucleus */
        {0x0000,0x0000,0x0000},
        {0x3b00,0x1b00,0x1200}, /* 1 sombre */
        {0x6d00,0x4900,0x0000},
        {0x9200,0x6d00,0x0000},
        {0xB600,0x9200,0x0000},
        {0xDB00,0x9200,0x2400},
        {0xDB00,0xB600,0x0000},
        {0xFD00,0xE700,0x0000}  /*7 plus clair */
        /* Couleurs de la casse */
        };

/* Correspondance entre epaisseur et couleur */
static unsigned epaiss[75]={
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,0,0,0,0,0,
                          0,0,0,0,0,
                          1,1,1,1,1,2,2,2,2,3,
                          3,3,3,4,4,4,4,5,5,5,
                          5,6,6,6,6,7,7,7,7,7};
unsigned short palSilex[NB_COUL_SILEX];

/**********************************************************************
                    plot_ecr
Ecrit un point color‚ directement sur l'ecran
=> ligne,colonne ,couleur ST
***********************************************************************/
static void plot_ecr(short y,short x,short couleur)
{
RGBColor rgb;

CvtCouleurStEnRgb(couleur,&rgb);
RGBForeColor(&rgb);
DoMoveTo(x*ECHELLE,y*ECHELLE);
DoPenSize(1,1);
DoLine(1,0);
DoPenSize(1,1);
ForeColor(blackColor);
return;
}
/**********************************************************************
                    lit_sil
Lit un point color‚ dans le silex
=> ligne,colonne
<= couleur 0-NB_COUL_SILEX
***********************************************************************/
static short lit_sil(short y,short x)
{
short i;
short index;

/* On extrait l'index de la couleur sur l'ecran */
index=GetIndexPixBrt(Ecran1,x+106,y);
/* On recherche cet index dans la palette */
for(i=0;i<NB_COUL_SILEX;i++)
    if(palSilex[i]==index) return(i);
Collapse("[Sapgest.lit_sil.1] Color not found");
return(0);
}
/**********************************************************************
                    ecrit_sil
Ecrit un point color‚ dans le silex
=> ligne (0-400) ,colonne (0-640),couleur 0-NB_COUL_SILEX
***********************************************************************/
static void ecrit_sil(short y,short x,short couleur)
{
PlotIndexPixBrt(Ecran1,x+106,y,palSilex[couleur]);
return;
}
/***********************************************************************
                        CAT:Affiche le catalogue courant
************************************************************************/
void cat(void)
{
return;
}

/***********************************************************************
                TRAPSURE:Acces reserve aux initi‚s...
***********************************************************************/
void trapsure(short quoi)
     {
     register short i,j;
     flefba=0;
     if(quoi>0) comment(114+quoi);
     switch (quoi)
          {
          case 0:
		  	DEF3D();
		  #ifdef OBSOLETE
          dervue=VUE_AUCUNE;
          palhaute=pal2;
          effima(0x700);/* palhaute[3]*/
          for (i=0;i<3;i++)
               {
               for (j=0;j<2;j++) Affmob(MOB_PICT+2*i+j,10+27*i,4*(16+16*j),0);
               }
          Affmob(MOB_GUEPARD,40,6*4,0);
          Affmob(MOB_GUEPARD,40,58*4,1);
          tfrima();
		  #endif
          break;
          case 1:
                  xj=22528;yj=21504;
                  break;
          case 2:
                  sceveu(11);
                  break;
          case 3:
                  sceveu(13);
                  break;
          case 4:
                  ajoutl(&liste1,22);
                  ajoutl(&liste1,25);
                  ajoutl(&liste1,46);
                  break;
          case 5:
                  sceveu(12);
                  break;
          case 6:
                cat();
                break;
          }
     return;
     }
# define DEHORS mobBambou+20
# define FRAPPE mobBambou+21
# define CNUL 0
# define CPLEIN 15


/***********************************************************************
                        TABDRT: Etablit la position droite
***********************************************************************/
static short tabdrt(short pos)
{
short i=NB_COL_SILEX+1;

while( !lit_sil(pos,--i) && (i>NB_COL_SILEX/2))/**/;
return(i);
}
/***********************************************************************
                        TABGAU: Etablit la position gauche
***********************************************************************/
static short tabgau(short pos)
{
short i=0;

while( !lit_sil(pos,i++) && (i<NB_COL_SILEX/2) )/**/;
return(i);
}
/***********************************************************************
                        VALSYL:
Evalue l'erreur entre l'etat du nucleus et la forme theorique attendue
***********************************************************************/
static unsigned short valsyl(void)
{
short i;
unsigned long ecart=0;
short trav1,trav2;

for (i=0;i<NB_LIGNE_SILEX;i+=2)
    {
    trav1=(NB_COL_SILEX/2)-2*forsyl[i/2]-tabgau(i);/*ecart par rapport … ma gauche*/
    if (trav1<0) trav1=-trav1;/*valeur absolue*/
    trav2=(NB_COL_SILEX/2)+2*forsyl[i/2]-tabdrt(i);/*ecart … la droite*/
    if (trav2<0) trav2 =-trav2;/*valeur absolue*/
 /*
    j=trav1+trav2;
    while(j--)
        PlotIndexPixBrt(debecr,106+j,i,200);
    TfrEcr(NULL);
*/
    ecart +=trav1+trav2;
    }
return(ecart/3);
}
/***********************************************************************
                        AfficheSuite
Affiche le mob suite
***********************************************************************/
static void AfficheSuite(void)
{
affomb(mobFleches+19,AireSuite.top,AireSuite.left,2,2,1);
return;
}
/***********************************************************************
                        MouseDansSuite
***********************************************************************/
static Boolean MouseDansSuite(short posx,short posy)
{
Point pt;

SetPt(&pt,posx,posy);
if (PtInRect(pt,&AireSuite))
    {
    return(VRAI);
    }
return(FAUX);;
}
/***********************************************************************
                        SILEX: Affiche un silex
Affiche le nucleus et la forme ideale du silex par dessus
***********************************************************************/
static void TraceSilex(void)
{
short i;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
/* on efface la souris */
eff_sour();
/*dessin du noyau*/
tfrvirt();
/*dessin de la forme ideale … obtenir*/
for (i=0;i<NB_LIGNE_SILEX;i+=4)
    {
    if (forsyl[i/2])
        {
        plot_ecr(i/2+1,160+forsyl[i/2],0x007);
        plot_ecr(i/2+1,160-forsyl[i/2],0x007);
        }
     }
/* On affiche la puissance du coup */
//r=AireJaugeForce;
//BoxFill(debecr,&r,0x000);
//InsetRect(&r,1,1);
//BoxFill(debecr,&r,0x444);
AfficheSuite();
/* On transfere sur l'ecran */
TfrEcr(NULL);
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************
                       FabriqueNucleus
Dessine un nucleus brut sur l'ecran 1
***********************************************************************/
static void FabriqueNucleus(void)
{
short y,x;
short delta1,pos1,pos2,kk,couleur,pas;
short posTheo;

/* Palette du silex */
effima(COULEUR_FOND_NUCLEUS);
delta1=60;
/* TIRAGE DU COEFF KK DE LA PIERRE DE 3 A 15 */
kk=myRandom(3,16);
 /* Pour l'ensemble des lignes du silex */
for (y=0;y<NB_LIGNE_SILEX;y++)
    {
    delta1 +=myRandom(-1,2);
    if (delta1<0) delta1=0;
    posTheo=(forsyl[y/2]+forsyl[y/2+1])/2;
    pos1=(NB_COL_SILEX/2)-posTheo-delta1;/* position gauche du nucleus */
    delta1 +=myRandom(-1,2);
    if (delta1 <0) delta1=0;
    pos2=(NB_COL_SILEX/2)+posTheo+delta1;/* position droite du nucleus */
    /* securite : sorties */
    if (pos1<0) pos1=0;
    if (pos2>NB_COL_SILEX) pos2=NB_COL_SILEX;
   /* on ecrit au centre*/
    if(pos1==(NB_COL_SILEX/2)) pas=1; /* Modif DG le 1-6-96*/
    else          pas=750/((NB_COL_SILEX/2)-pos1);
    for (x=pos1;x<=(NB_COL_SILEX/2);x++)
        {
        couleur=1+((x-pos1)*( (y<(NB_LIGNE_SILEX/2)) ? y : NB_LIGNE_SILEX-y)+Rnd()*kk)/1000 ;
        if (couleur>=NB_COUL_SILEX) couleur=NB_COUL_SILEX-1;
        ecrit_sil(y,x,couleur);
        }
    if(pos2==(NB_COL_SILEX/2)) pas=1; /* Modif DG le 1-6-96*/
    else pas=750/(pos2-(NB_COL_SILEX/2));
    for (x=pos2;x>(NB_COL_SILEX/2);x--)
        {
        couleur=1+((pos2-x)*((y<(NB_LIGNE_SILEX/2)) ? y : NB_LIGNE_SILEX-y)+Rnd()*kk)/1000;
        if (couleur>=NB_COUL_SILEX) couleur=NB_COUL_SILEX-1;
        ecrit_sil(y,x,couleur);
        }
    }
return;
}
/***********************************************************************
                       AppliqueNlleEpaiss
Applique une nouvelle epaisseur si la nouvelle est inferieure a l'ancienne
***********************************************************************/
static void AppliqueNlleEpaiss(short y,short x,short epaiss)
{
short ancEpaiss;

ancEpaiss=lit_sil(y,x);
if (ancEpaiss<epaiss || epaiss==0) ecrit_sil(y,x,epaiss);
return;
}
/***********************************************************************
                       AppliqueImpact

=> Position x et y du coup, force du coup
<= Bruit a jouer
***********************************************************************/
static short AppliqueImpact(short posxf,short posyf,short forcef)
{
short penet;
short posimp;
register short i,j,pas,a,paslec;
unsigned short onde,yact,calc,distcb,tfr,senson,cass,okbruit=0;
unsigned short trav1;
short distact;

cass=0;
posxf*=ECHELLE;
posyf*=ECHELLE;
/* Le point d'impact est-il sur le nucleus si non on sort*/
if ((posxf > tabdrt(posyf)) || (posxf < tabgau(posyf)))
    {
    /* Dessin du silex */
    TraceSilex();
    return(okbruit);
    }
/* Sens de l'ondex : coup a gauche ou a droite */
senson=(posxf<(NB_COL_SILEX/2));/*vrai si gauche faux si droite*/
/* Tfr varie de 0 … 15*/
tfr=forcef/16;
/* Penet varie de 0 … 50 */
penet=(forcef*50)/256;
if (senson) /*gauche*/
    {
    tfr += posxf; /*le coup porte vers l'interieur*/
    if (tfr<penet) tfr=penet;
    /*si on depasse le milieu ca casse*/
    if (tfr>(NB_COL_SILEX/2)) cass++;
    posimp=tfr-penet;
    /* Distance maximale */
    distcb=penet+(posimp-tabgau(posyf));
    }
 else  /*droite*/
    {
    tfr=posxf-tfr;
    if (tfr<(NB_COL_SILEX/2)) cass++;/*depasse le milieu casse*/
    posimp=tfr+penet;
    distcb=penet-(posimp-tabdrt(posyf));
    }
if (((forcef-1)*48/256)>distcb)/*force suffisante*/
    {
    if (cass) /*c'est cass‚...*/
        {
        okbruit=3;
        a=74;
        /* On casse le haut du silex */
        if (posyf<NB_LIGNE_SILEX/2)
            {
            for (i=posyf+4;i>=0 && i<NB_LIGNE_SILEX;i--,a-=8)
                        {
                        if (a<0) a=0;
                        for (j=0;j<NB_COL_SILEX;j++)
                            AppliqueNlleEpaiss(i,j,0);
                        }
                    }
             else
                {
                for (i=posyf-5;i>=0 && i<NB_LIGNE_SILEX;i++,a-=10)
                    {
                    if (a<0) a=0;
                    for (j=0;j<NB_COL_SILEX;j++)
                        AppliqueNlleEpaiss(i,j,0);
                    }
                }
        }
    /* Le silex n'est pas cass‚ , on taille un eclat*/
    else
        {
        paslec=(forcef^255)*8+128;
        yact=posyf;
        calc=posyf+1;
        tfr=onde=trav1=0;
        while (onde<100)/*on applique l'onde*/
            {
            if (yact<NB_LIGNE_SILEX)
                {
                if (senson) /*gauche*/
                    {
                    /* a = position en colonne de l'eclat pour  cette ligne */
                    a=(formonde[onde]*forcef)/256+posimp;
                    distact=a-tabgau(yact);
                    if (distact>0)
                        {
                        pas=7400/(distact+1);
                        if (pas==0) pas =1;
                        for (i=7400;i>=0;i-=pas,a--)
                            {
                            AppliqueNlleEpaiss(yact,a,epaiss[i/100]);
                            }
                        }

                    }
                else        /* DROITE */
                    {
                    a=posimp-(formonde[onde]*forcef)/256;
                    distact=tabdrt(yact)-a;
                    if (distact>0)
                        {
                        pas=(7400/distact);
                        if (pas==0) pas =1;
                        for (i=7400;i>=0;i-=pas,a++)
                            AppliqueNlleEpaiss(yact,a,epaiss[i/100]);
                        }
                    }
                } /* si yact <100 */
        /* on fait le symetrique bas */
        if (calc<NB_LIGNE_SILEX)
                {
                if (senson) /* gauche */
                    {
                    a=(formonde[onde]*forcef)/256+posimp;
                    distact=a-tabgau(calc);
                    if (distact>0)
                        {
                        pas=7400/(distact+1);
                        if (pas==0) pas =1;
                        for (i=7400;i>=0;i-=pas,a--)
                              AppliqueNlleEpaiss(calc,a,epaiss[i/100]);
                        }
                    }
                else /* droite */
                    {
                    a=posimp-(formonde[onde]*forcef)/256;
                    distact=tabdrt(calc)-a;
                    if (distact>0)
                        {
                        pas=(7400/distact);
                        if (pas==0) pas =1;
                        for (i=7400;i>=0;i-=pas,a++)
                            AppliqueNlleEpaiss(calc,a,epaiss[i/100]);
                        }
                    }/* fin de droite */
                }/* fin de calc <100*/
            /* on passe a la ligne suivante */
            calc++;
            yact --;  /*ligne suiv haute*/
            trav1 += (paslec & 255);
            /*plus retenue*/
            onde=onde+(trav1>255)+(paslec/256);
            trav1 &=255;/*on garde que le PL*/
            }  /*balaye le silex*/
        okbruit=4;/*aigu*/
        }/*si pas casse*/
    } /*si force suffisante*/
 else
    {
    /* La force est insufisante pour attaquer le nucleus */
    okbruit=5;/*bruit sourd*/
    }
/* Dessin du silex */
TraceSilex();
return(okbruit);
}
/***********************************************************************
                        CURSIL:Taille le silex
=> Silex a tailler
<= Collapse sur le silex
***********************************************************************/
static short cursil(short regb)
 {
 unsigned char okbruit;
 short forcef;
 short bidon;
 short i;
 Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
/* On calcule les index de couleurs */
for(i=0;i<NB_COUL_SILEX;i++)
    palSilex[i]=SapColor2Index(&TabCoulSilexRGB[i]);
 AcamSetPortOffscreen(gbak);
 if (regb==0) forsyl=forsy1;
 else         forsyl=forsy2;


  /* Effacement du centre */

 /**/
 /*on remplit avec une pierre noire*/
 /**/
  FabriqueNucleus();
  /* on fixe l'ecart stochastique gauche et droite */
  flefba=0;
 //comment(132);/*Bouton droit pour valider*/

 /* on l'affiche tel quel */
  /* Mise en place des bords */
 bande(1);
 TraceSilex();
 SetRect(&r,16*ECHELLE,0*ECHELLE,(320-16)*ECHELLE,200*ECHELLE);
 FixeAireNoMouse(&r);

 while(1)
      {
       do
                vq_mouse(&bouton,&posxf,&posyf);
                      while(bouton);
       okbruit=0;
       do
            {
              vq_mouse(&bouton,&posxf,&posyf);
              if ( (posxf<32) || (posxf>320) || (posyf>100) ||(posyf<2) )
                          {
                          if (posyf>100) posyf=100;
                          if (posxf>320-16) posxf=320-16;
                          aff_sour(DEHORS,posyf,posxf);
                          bouton &=2;
                          }
                else
                    {
                    if (MouseDansSuite(posxf,posyf))
                        aff_sour(FRAPPE,posyf,posxf);
                    else
                        aff_sour(mobBambou+22,posyf-25,posxf);
                    }

            } while(!bouton);
    /* On valide le silex */
      if (MouseDansSuite(posxf,posyf))
            {
            affomb(mobFleches+20,AireSuite.top,AireSuite.left,2,2,1);
            FixeAireNoMouseAireBasse();
            return(valsyl());
            }
      /* Point d'application */
      plot_ecr(posyf,posxf,0x700);
      if (bouton & 1)  /*on frappe*/
           {
           unsigned long t=TickCount();
           /* On determine la force du coup */
           forcef=0;
           do
                {
                //r=AireJaugeForce;
                //r.top=(4+87-forcef/3)*ECHELLE;
                //InsetRect(&r,1,1);
                //BoxFill(debecr,&r,0x070);
                //TfrEcr(&r);
                vq_mouse(&bouton,&bidon,&bidon);
                forcef=(TickCount()-t)*4;
                aff_sour(mobBambou+22,posyf-25-forcef/8-2,posxf+forcef/8+2);
                } while (bouton && (forcef<254) );
           if(forcef>254) forcef=254;
           for(i=forcef;i>=0;i-=32)
                 aff_sour(mobBambou+22,posyf-25-i/8,posxf+i/8);
           /* On applique l'impact */
          /* conversion de coordonnees */
         okbruit=AppliqueImpact(posxf-85,posyf-1,forcef);
         aff_sour(mobBambou+22,posyf-25,posxf);
         /* On joue le bruit */
        if (okbruit) bruit(okbruit);
        }/*on frappe*/
	  #if DEBUG>0
	  	printf("Silex score : %d/%d\n",valsyl(),PRECISION_SILEX);
      #endif
	  }
 }
/***********************************************************************
                        GETNOM:Saisi un nom de la taille donn‚e
***********************************************************************/
static short getnom(short mess,char * position,short taille)
     {
     register short i,j;
     register short pos=0,car,oktrap;

 	while(litclav());//Purge
    for (i=0;i<taille;i++) zoneu.elements[i]=32; /*on vide la zone*/
     while (1)
          {
            /* On affiche le commentaire */
           efbadi();
           comment(mess);
           for(i=0;i<taille && zoneu.elements[i]!=' ';i++)
                    affcarmess(zoneu.elements[i]);
           affcarmess(2);/* Curseur */
           TfrBadAss();
           do
                {
                car=litclav();
                } while(!car);
           if (car==13)   /* Validation*/
               {
               /*test des trapsures */
               for (i=0;i<7;i++)
                    {
                    oktrap=1;
                    for (j=0;j<6;j++)
               		     if (((traps[i][j] ) & 223) !=
                      		  (zoneu.elements[j] & 223 )) oktrap=0;
                    if (oktrap)
                         {
                         trapsure(i);
                         pos=0;
                         }
                    }
               if (pos) /*au moins un caractere*/
                    {
                    for (i=0;i<taille;i++) *(position++)=zoneu.elements[i];
                    }
               efbadass();
               TfrBadAss();
               return(pos);
               }
          if (car>' ' && car <='z' && !( car>='0' && car<='9'))
               {
               if (pos<taille)
                    {
                    zoneu.elements[pos++]=car;
                    }
               }
          else
               {
               if(car==8 && pos)
                    {
                    zoneu.elements[--pos]=32;
                    }
               }
          }
     }

/***********************************************************************
                        DebutAventure
Initialise decor 3d, et afiche le copyright
***********************************************************************/
void DebutAventure(Boolean isFirst)
{
register short i;
unsigned long posTimer;

FixeMenusReduit();
primsap();
generique(FAUX);
if(isFirst) DefautDecor3d();
else InitDecor3d();
CreeDecor3d(false);
effvirt(0x000);
effima(0x000);
/*Presentation*/
heure=10;
chrono(0);
affich(dzeta);/* Affiche la vue trois d sur ecran 1*/
tfrvirt();
flefba=1;
InitGenerique();
generique(FAUX);
TfrEcr(NULL);
Frame3d(FAUX);
efbadi();
comment(67);/*d et o guillion*/
for (i=0;i<60;i++)
      {
      posTimer=sapiensTimer;
      chrono(1);
      chrono(0);
      chrono(0);
      affich(dzeta);
      tfrima();
      if (getkey(0) & (32+16)) i=111;
      flefba=0;
      if (i==25) comment(68);
      if (i==50) comment(69);
      /* Synchronisation de la trame */                        
	  while (posTimer+30 >= sapiensTimer)/**/;
      }
comment(70);
FixeMenusComplet();
MenuFenetres(VRAI);
efbadi();
comment(132);
return;
}
/***********************************************************************
                        PRIMSAP:definition primaires des saps
***********************************************************************/
void primsap(void)
 {
 register short i;

 scenar=jour_act=1; scenpr=20;
 fdialo=pos_evt=presence=viecopain=jour_dsk=0;
 dervue =VUE_AUCUNE;
 prm(&sap2,0);
 prm(&sap3,0);
 prm(&sap4,0);

 /*quotas et positions*/
 mort=4;
 faim=soif=fatig=bless=255;
 xj=0x5800 ; yj=0x5400 ;dzeta=768;zoom=120;heure=1;
 zj=40;
 /*objets portes*/
 liste1.taille=9;
 for (i=0;i<9;i++) liste1.elements[i]=listdep[i];
 if(IsEnreg())  liste1.elements[0]=22;/*Massue a la place de hache */

 for (i=0;i<4;i++)
    {
    enetoi[i]=tenerv[i];
     tagress[i]=agrdep[i];
     }
 destine[0].type=1;/*regenere les 4 chefs*/
 destine[1].type=1;
 destine[2].type=1;
 destine[3].type=1;
 liste4.taille=0;

 return;
 }
/***********************************************************************
                        AireJauge
Calcule l'aire ecran totale d'une jauge
***********************************************************************/
static void AireJauge(short i,Rect * r)
{
SetRect(r,114,6+14*i,114+maxparam[sexe][i]/2+2,16+14*i);
return;
}
/***********************************************************************
                        AireValJauge
Calcule l'aire ecran de la valeur interne d'une jauge
***********************************************************************/
static void AireValJauge(short i,Rect * r)
{
SetRect(r,115,7+14*i,115+param[i]/2,15+14*i);
return;
}
/***********************************************************************
                        AfficheCadreJauge
***********************************************************************/
static void AfficheCadreJauge(short i)
{
Rect r;

AireJauge(i,&r);
ForeColor(blackColor);
r.left*=ECHELLE;
r.right*=ECHELLE;
r.top*=ECHELLE;
r.bottom*=ECHELLE;
DoFrameRect(&r);
return;
}
/***********************************************************************
                        JAUGE:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
static void jauge(short i,Boolean tfr)
{
Rect rval;
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
AireValJauge(i,&rval);
AfficheCadreJauge(i);
boxf(rval.top,rval.left,rval.bottom,rval.right,2);
/* Effacement de la partie droite */
AireJauge(i,&r);
if (param[i]/2<maxparam[sexe][i]/2)
    {
    boxf(r.top+1,rval.right,r.bottom-1,r.right-1,14);
    if(param[i])
        {
        boxf(r.bottom-2,r.left+1,r.bottom-1,rval.right-1,9);
        boxf(r.top+1,rval.right-1,r.bottom-1,rval.right,9);
        }
    }
AcamSetPortOffscreen(gbak);
if(tfr)
    {
    r.left*=ECHELLE;
    r.right*=ECHELLE;
    r.top*=ECHELLE;
    r.bottom*=ECHELLE;
    TfrEcr(&r);
    }
return;
}
/***********************************************************************
                        PRIMJAUGE:primaire pour les jauges

***********************************************************************/
void primjauge(short jaugeCredit)
 {
 register short i;
  char ** app=GetApp();
 dervue=VUE_AUCUNE;
 effima(0x050);/* Efface ecran sur 100 ligne */
 tfrvirt();
 /* Affichage sur Ecran */
 Gotoxy(6,1);motlst2(app[0]);
 Gotoxy(6,3);motlst2(app[1]);
 Gotoxy(6,5);motlst2(app[2]);
 Gotoxy(6,7);motlst2(app[3]);
 Gotoxy(6,9);motlst2(app[4]);
 if(jaugeCredit)
	{
	Gotoxy(6,11);
	motlst2(app[5]);
	}

 /*trace des cadres jauges*/
 for (i=0;i<5+jaugeCredit;i++)
	  jauge(i,FAUX);
 TfrEcr(NULL);
 return;
 }
/***********************************************************************
                        DEFSAP: Defini un sapiens
***********************************************************************/
void defsap(void)
 {
 char car,cont,pos;
 register short i;
 short taille,dpos;
 Rect r,rval;
 Point pt;

 sapvit=0;
 if (!(taille=getnom(88,(char*)nomjoueur,8))) return;
 flefba=0;
 nomjoueur[taille]=0;
 /* definitions des quotas personnage*/
 for (i=0;i<5;i++) param[i]=130;
 param[5]=20;
 generique(VRAI);
 comment(89);/*   XX ou XY ???*/
 car=0;
 reset_clav();
 while ( car !=HOMME && car !=FEMME ) car=(litclav() & 223);
 affcarmess(car);
 TfrBadAss();
 if (car== HOMME )
      {
      sexe=1;
      sap1.cross[FORME_BUSTE]=0;
      sap1.cross[FORME_TETE]=0;
      }
 else
      {
      sexe=0;
      sap1.cross[FORME_BUSTE]=8;
      sap1.cross[FORME_TETE]=12;
      }
 pal_chron=palhaute;
 palhaute=pal2;
 primjauge(1);
 AfficheSuite();
 cont=1;pos=0;
 sposx=dsposx=160;
 sposy=dsposy=50;
 dpos=2;
 SetRect(&r,16*ECHELLE,0*ECHELLE,(320-16)*ECHELLE,200*ECHELLE);
 FixeAireNoMouse(&r);

 /* Selection des Kotas de force */
 while(cont)
      {
      do
        {
        vq_mouse(&bouton,&sposx,&sposy);
        aff_sour(FRAPPE,sposy,sposx);
        }  while(!bouton);

      if (MouseDansSuite(sposx,sposy))
           {
            affomb(mobFleches+20,AireSuite.top,AireSuite.left,2,2,1);
           cont=0;  /*esc*/
           if (param[0]<223) agilite=param[0]+32;
           else              agilite=255;
           force=param[1];
           charisme=param[2];
           vie=param[3];
           resistance=param[4];
           while (getkey(0))/**/;/* Epuise le clavier */
           }
      else
        {
        SetPt(&pt,sposx,sposy);
        for(i=0;i<5;i++)
             {
             AireJauge(i,&r);
             if(PtInRect(pt,&r))
                {
                pos=i;
                /* Fleche */
                affomb(MOB_DESIGNE,r.top-4,86,0,0,VRAI);
                do
                    {
                    AireValJauge(pos,&rval);
                    /* On augmente la jauge */
                    if(sposx>rval.right && param[5] && param[pos]<maxparam[sexe][pos])
                        {
                        param[5]--;
                        param[pos]++;
                        }
                    if(sposx<rval.right && param[5]<254 && param[pos])
                        {
                        param[5]++;
                        param[pos]--;
                        }
                    vq_mouse(&bouton,&sposx,&sposy);
                    jauge(pos,VRAI);
                    /* Quantite restante */
                    jauge(5,VRAI);
                    aff_sour(FRAPPE,sposy,sposx);

                    //Delay(1,&bidon);
                    } while(bouton);
                primjauge(1);
                AfficheSuite();
                i=255;
                }
            }
        }
      }
 /*selection de l'apparence*/
 eff_sour();
 cont=1;pos=0;
 prm(&sap1,-1);
 effima(0x050);
 tfrvirt();
 tfrima();
 flefba=0;
// comment(132);/*Bouton droit pour valider*/
 eff_sour();
 while(cont)
      {
      sap1.colonne=28 ;
      sap1.sens=1;
      effima(0x050);//??
      //effdemi();
      mvtsap(&sap1);
      tfrdemi();
      tfrvirt();
      AfficheSuite();

      affomb(MOB_DESIGNE,(pos*2+6)*7+1,100,4,4,VRAI);
      do
        {
        vq_mouse(&bouton,&sposx,&sposy);
        aff_sour(FRAPPE,sposy,sposx);
        }   while(!bouton);

      if (MouseDansSuite(sposx,sposy))
           {
            affomb(mobFleches+20,AireSuite.top,AireSuite.left,2,2,1);
           cont=0;  /*esc*/
           }
      else
        {
        pos=sposy>53;
        dsposy=sposy;
        dsposx=sposx;
        if (bouton & 1) /*gauche*/
           {
           if (pos)
                {
                sap1.cross[FORME_BUSTE]+=2;/*Buste*/
                if (sexe)
                     {
                     if (sap1.cross[FORME_BUSTE] >=8 ) sap1.cross[FORME_BUSTE]=0;
                     }
                else
                     {
                     if (sap1.cross[FORME_BUSTE] >= 16) sap1.cross[FORME_BUSTE]=8;
                     }
                }
           else
                {
                sap1.cross[FORME_TETE]+=2;/*tete*/
                if (sexe)
                     {
                     if (sap1.cross[FORME_TETE] > 6) sap1.cross[FORME_TETE]=0;
                     }
                else
                     {
                     if (sap1.cross[FORME_TETE] > 22) sap1.cross[FORME_TETE]=12;
                     }
                }
           do vq_mouse(&bouton,&sposx,&sposy);
                    while(bouton &1);
           }
        }
      }
 eff_sour();
 /* On est enregistre le personnage a une massue */
 palhaute=pal_chron;
 DebutAventure(false);
 FixeAireNoMouseAireBasse();
 return;
 }
/***********************************************************************
                        AIREJAUGE3d:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
#define CAR_UTILE 180
static AIREJAUGE3d(short i,Rect * r)
{
SetRect(r,75,7+14*i,75+CAR_UTILE,15+14*i);
}
/***********************************************************************
                        JAUGE3d:
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
#define NB_JAUGES_3D 5
static void JAUGE3d(short i,float val)
{
Rect r;

AIREJAUGE3d(i,&r);
boxf(r.top-1,r.left-1,r.bottom+1,r.right+1,16);
boxf(r.top,r.left,r.bottom,r.left+CAR_UTILE*val,14);
boxf(r.top,r.left+CAR_UTILE*val,r.bottom,r.right,4);
return;
}
/***********************************************************************
                        JAUGES3D
Trace une jauge dans debecr
=> numero de la jauge, indeicateur de transfert sur le reel
***********************************************************************/
static void JAUGES3D(void)
{
short i=0;
CGrafPtr gbak;
Rect r;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
boxf(0,32,101,287,5);

AIREJAUGE3d(i++,&r);
SetRect(&r,SapZoom*r.left,SapZoom*r.top,SapZoom*r.right,SapZoom*r.bottom);
r.left=80;
r.right=r.left+64;
Affmob3d(mob3D+7,&r,NULL);//Nuage 

AIREJAUGE3d(i++,&r);
SetRect(&r,SapZoom*r.left,SapZoom*r.top,SapZoom*r.right,SapZoom*r.bottom);
r.left=120;
r.right=r.left+16;
Affmob3d(mob3D+28,&r,NULL); //Oiseau

AIREJAUGE3d(i++,&r);
SetRect(&r,SapZoom*r.left,SapZoom*r.top,SapZoom*r.right,SapZoom*r.bottom);
InsetRect(&r,0,-2);
r.left=110;
r.right=r.left+26;
Affmob3d(mob3D+34,&r,NULL); //Pierre

AIREJAUGE3d(i++,&r);
SetRect(&r,SapZoom*r.left,SapZoom*r.top,SapZoom*r.right,SapZoom*r.bottom);
InsetRect(&r,0,-4);
r.left=110;
r.right=r.left+32;
Affmob3d(mob3D+10,&r,NULL); //Arbre

AIREJAUGE3d(i++,&r);
SetRect(&r,SapZoom*r.left,SapZoom*r.top,SapZoom*r.right,SapZoom*r.bottom);
InsetRect(&r,0,-4);
r.left=110;
r.right=r.left+32;
Affmob3d(MOB_HERBE,&r,NULL); //Herbe


AfficheSuite();

i=0;
JAUGE3d(i++,(float)nbNuages/(NB_NUAGES_MAX));
JAUGE3d(i++,(float)nbOiseaux/(NB_OISEAUX_MAX));
JAUGE3d(i++,(float)nbRochers/(NB_ROCHERS_MAX));
JAUGE3d(i++,(float)nbArbres/(NB_ARBRES_MAX));
JAUGE3d(i++,(float)nbHerbes/(NB_MAX_HERBE_AV_PLAN));
SetRect(&r,0,0,600,400);
TfrEcr(&r);
SetRect(&r,0,0,320*ECHELLE,200*ECHELLE);
CarboniseInvalRect(&r);
ValideUpdate();
AcamSetPortOffscreen(gbak);
}

/***********************************************************************
                        DEF3D: Defini la 3d
***********************************************************************/
void DEF3D(void)
 {
 char car,cont,pos;
 register short i;
 short taille,dpos;
 Rect r,rval;
 Point pt;

 pal_chron=palhaute;
 palhaute=pal2;
 /* Affichage sur Ecran */
 cont=1;pos=0;
 sposx=dsposx=160;
 sposy=dsposy=50;
 dpos=2;
 SetRect(&r,16*ECHELLE,0*ECHELLE,(320-16)*ECHELLE,200*ECHELLE);
 FixeAireNoMouse(&r);
 while(cont)
      {
		
	  JAUGES3D();
       do
        {
        vq_mouse(&bouton,&sposx,&sposy);
        aff_sour(FRAPPE,sposy,sposx);
        }  while(!bouton);
      if (MouseDansSuite(sposx,sposy))
           {
             cont=false;
           }
      else
        {
        SetPt(&pt,sposx,sposy);
        for(i=0;i<NB_JAUGES_3D;i++)
             {
             AIREJAUGE3d(i,&r);
             if(pt.v>=r.top && pt.v<=r.bottom)
                {
                pos=i;
                do
                    {
					float val =(float)(sposx-r.left)/(float)(r.right-r.left);
					
					switch(pos)
						{
						case 0: nbNuages=val*NB_NUAGES_MAX;;break;
						case 1: nbOiseaux=val*NB_OISEAUX_MAX;;break;
						case 2: nbRochers=val*NB_ROCHERS_MAX;;break;
						case 3: nbArbres=val*NB_ARBRES_MAX;;break;
						case 4: nbHerbes=val*NB_MAX_HERBE_AV_PLAN;;break;
						}
					if(nbNuages>NB_NUAGES_MAX) nbNuages=NB_NUAGES_MAX;
					if(nbOiseaux>NB_OISEAUX_MAX) nbOiseaux=NB_OISEAUX_MAX;
					if(nbRochers>NB_ROCHERS_MAX) nbRochers=NB_ROCHERS_MAX;
					if(nbArbres>NB_ARBRES_MAX) nbArbres=NB_ARBRES_MAX;
					if(nbHerbes>NB_MAX_HERBE_AV_PLAN) nbHerbes=NB_MAX_HERBE_AV_PLAN;
					if(nbNuages<0) nbNuages=0;
					if(nbOiseaux<0) nbOiseaux=0;
					if(nbRochers<0) nbRochers=0;
					if(nbArbres<0) nbArbres=0;
					if(nbHerbes<0) nbHerbes=0;
                    vq_mouse(&bouton,&sposx,&sposy);
                    JAUGES3D();
					aff_sour(FRAPPE,sposy,sposx);
                    } while(bouton);
                i=255;
                }
            }
        }
      }
CreeDecor3d(true);
FixeTailleMainWindow();

}
#if TARGET_API_MAC_CARBON>0 && !defined(ACAM2)
#define TAILLE_NOM_DOCUMENT 128
static Ptr ExtensionAdd; /* Extension a ajouter */
static Ptr ExtensionSupp;/* Extension a supprimer */
static NavMenuItemSpec specDefaut;
static short extensionFichier=2;
/*********************************************************************
                           IsExtensionFichierDansFiltre
Determine si l'extension du fichier est dans la liste des extensions du filtre
=> Nom du fichier
[ExtensionAdd][ExtensionSup]
<= VRAI si dans la liste
********************************************************************/
static Boolean IsExtensionFichierDansFiltre(char * nom)
{
short i,j;
char extension[10];
char extensionLst[10];
Boolean cont;

/* On extrait l'extension du fichier en cours */
while(*nom)
	{
    if(*(nom++)=='.')
		{
		strncpy(extension,nom,9);
		extension[9]=0;
		if(ExtensionSupp)
			{
		/* On recherche l'extension dans la liste des extensions a supprimer */
			i=j=0;
			cont=true;
			while(cont)
				{
				if(ExtensionSupp[i]==0 || ExtensionSupp[i]==',')
					{
					if(ExtensionSupp[i]==0) cont=false;
					extensionLst[j]=0;
			        if(!stricmp(extension,extensionLst)) return(false);
					j=0;
					i++;
		        	}
		        else
		        	extensionLst[j++]=ExtensionSupp[i++];
		        }
		   }
		if(ExtensionAdd)
			{
			/* On recherche l'extension dans la liste des extensions possibles */
			i=j=0;
			cont=true;
			 while(cont)
				{
				if(ExtensionAdd[i]==0 || ExtensionAdd[i]==',')
					{
					if(ExtensionAdd[i]==0) cont=false;
					extensionLst[j]=0;
			        if(!stricmp(extension,extensionLst)) return(true);
					j=0;
					i++;
		        	}
		        else
		        	extensionLst[j++]=ExtensionAdd[i++];
		        }
		     return(false);
			}	       
	    return(true);
	    }
	}
return(ExtensionAdd==NULL?true :false);
}

/*********************************************************************
                           myEventProcLoad
Fonction de filtre sur les evenements
<= FAUX si l'objet ne doit pas etre affich‚
********************************************************************/
static pascal void myEventProcLoad(NavEventCallbackMessage callBackSelector,
							NavCBRecPtr callBackParms,NavCallBackUserData callBackUD)
{
NavMenuItemSpec * navItemSel;

switch (callBackSelector)
	{
	case kNavCBEvent:
		break;
	case kNavCBCustomize:/* signal to negotiate customization space*/
		break;
	case kNavCBStart:/* nav dialog box starting up */
				/* On specifie la valeur du pop up par defaut 
				To set the default selection for the Show pop-up menu of an Open dialog box,
				your application passes the kNavCtlSelectAllType constant, described in
				_Custom Control Setting Constants_ (page 96), in the selector parameter of the
				NavCustomControl function (page 81) and passes one of the constants defined in
				the NavPopupMenuItem data type in the parms parameter of the NavCustomControl
				function.*/
				if(specDefaut.menuType>0)
					{
					NavCustomControl(callBackParms->context,kNavCtlSelectCustomType,&specDefaut);
					specDefaut.menuType=0;
					}
		break;
	case kNavCBTerminate:/* nav dialog box closing down */
		break;
	case kNavCBAdjustRect:/* nav dialog box being resized */
		break;
	case kNavCBNewLocation:/* user chose new location*/
		break;
	case kNavCBShowDesktop: /* user navigated to the desktop */
		break;
	case kNavCBSelectEntry: /* user made selection in browser */
		break;
	case kNavCBPopupMenuSelect: /* user made popup menu selection */
		/*	Tells your application that a selection was made from the
		Open dialog box_s Show pop-up menu or Save dialog box_s
		Format pop-up menu. The param field of the NavCBRec
		structure contains a pointer to a structure of type
		NavMenuItemSpec (page 87) describing the pop-up menu
		item selected. This data is valid only during the execution
		of your event-handling function.*/
		navItemSel=(NavMenuItemSpec *)callBackParms->eventData.eventDataParms.param;
		extensionFichier=navItemSel->menuType;
		break;
	case kNavCBAccept:/* user accepted nav dialog box */
		break;
	case kNavCBCancel:/* user cancelled nav dialog box */
		break;
	case kNavCBAdjustPreview:/* preview toggled or resized */
		break;
	}
}
/*********************************************************************
                           myFilterProcSelectExtension
Fonction de filtre sur l'extension pour la selection avec Navigation
<= FAUX si l'objet ne doit pas etre affich‚
********************************************************************/
static pascal Boolean myFilterProcSelectExtension(AEDesc* theItem, void* info,
									NavCallBackUserData callBackUD,
									NavFilterModes filterMode)
{
Boolean display = true;/* Ok afffiche par defaut */
NavFileOrFolderInfo* theInfo = (NavFileOrFolderInfo*)info;
char nom[TAILLE_NOM_DOCUMENT];
FSSpec **documentFSSpec;

/* C'est un fichier */
if (theItem->descriptorType == typeFSS && !theInfo->isFolder)
	{
	documentFSSpec=(FSSpec **)theItem->dataHandle;
	BlockMove((*documentFSSpec)->name,nom,sizeof((*documentFSSpec)->name));
	PtoCstr((StringPtr)nom);
	return(IsExtensionFichierDansFiltre(nom));
	}
return (display);
}
/*********************************************************************
                           GetFileNameLoadNavigationService
Saisi un nom de fichier pour une lecture e passant par lde navigation service
=> Adresse ou ranger le nom,
liste des extension sur laquelle filtrer au format "xxx,yyy" ou NULL si aucune
liste des extension a supprimer au format "xxx,yyy" ou NULL si aucune
@=Position par defaut sur le disque
<= VRAI si ok
********************************************************************/
static Boolean GetFileNameLoadNavigationService(Ptr nom,char * extensionAdd,char * extensionSupp)
{
Boolean isNavigationAvailable=true;
NavReplyRecord reply;
NavDialogOptions dialogOptions;
NavObjectFilterUPP filterProc=NULL ;
OSErr anErr = noErr;
Boolean result=false;
NavEventUPP eventProc = CarboniseNewNavEventProc(myEventProcLoad);
NavMenuItemSpec ** listeItemSpec=NULL;

// On recupere les valeurs par defaut de la boite de saisie
anErr = NavGetDefaultDialogOptions(&dialogOptions);
if (anErr == noErr)
	{
	// On ajuste les options
//	dialogOptions.dialogOptionFlags |= kNavSelectDefaultLocation;
	// Clear this one
	dialogOptions.dialogOptionFlags ^= kNavAllowPreviews;	 // Pas de preview 
	dialogOptions.dialogOptionFlags ^= kNavAllowMultipleFiles;// Pas de selection multiple
	/* Choix du filtre sur l'extension */
	ExtensionAdd=NULL;
	ExtensionSupp=NULL;
	if(extensionAdd || extensionSupp)
		{
		ExtensionAdd=extensionAdd;
		ExtensionSupp=extensionSupp;
		filterProc = CarboniseNewNavObjectFilterProc(myFilterProcSelectExtension);
		}
	/* Chemin par defaut */
	anErr=noErr;
	if (anErr == noErr)
		{	
		// Get 'open' resource. A nil handle being returned is OK,
		// this simply means no automatic file filtering.
		anErr = NavGetFile (NULL,  /* Position de demarrage */
							&reply, 	      /* Reponse */
							&dialogOptions,	  /* Options du dialogue */
							eventProc,		  /* Traitement des events */
							NULL, 			  /* Pas de preview */
							filterProc,		  /* Filtre sur le nom*/
							NULL, 			  /* Pas de filtre sur le type */
							NULL);			  /* Pas de call back parametre */
		if(anErr  && anErr!=userCanceledErr)
			Collapse("	NavGetFile : Err %d",anErr);				
		if (anErr == noErr && reply.validRecord)
			{
			// Deal with multiple file selection
			long count;
			anErr = AECountItems(&(reply.selection), &count);
			if (anErr == noErr)
				{
				long index;
				for (index = 1; index <= count; index++)
					{
					AEKeyword theKeyword;
					DescType actualType;
					Size actualSize;
					FSSpec documentFSSpec;

					anErr = AEGetNthPtr(&(reply.selection), index,
										typeFSS, &theKeyword,
										&actualType,&documentFSSpec,
										sizeof(documentFSSpec),
										&actualSize);
					if (anErr == noErr)
						{
						PtoCstr(documentFSSpec.name);
						strcpy(nom,(Ptr)documentFSSpec.name);
						FixeVolPath(documentFSSpec.vRefNum,documentFSSpec.parID);
						result=true;
						}
					}
				}
			// Dispose of NavReplyRecord, resources, descriptors
			anErr = NavDisposeReply(&reply);
			}
		}
	}
if(listeItemSpec) DisposeHandle((Handle)listeItemSpec);
if(filterProc) CarboniseDisposeNavObjectFilterProc(filterProc);
if(eventProc) CarboniseDisposeNavEventProc(eventProc);
return (result);
}
/*********************************************************************
                           GetFileNameSaveNavigationService
Saisi un nom de fichier pour une ecrture passant par lde navigation service
=> Nom par defaut et Adresse ou ranger le nom,
@=Position par defaut sur le disque
<= VRAI si ok
********************************************************************/
static Boolean GetFileNameSaveNavigationService(Ptr nom)
{
Boolean isNavigationAvailable=true;
NavReplyRecord reply;
NavDialogOptions dialogOptions;
OSErr anErr = noErr;
Boolean result=false;

// On recupere les valeurs par defaut de la boite de saisie
anErr = NavGetDefaultDialogOptions(&dialogOptions);
if (anErr == noErr)
	{
	// On ajuste les options
	dialogOptions.dialogOptionFlags |=kNavDontAddTranslateItems;
	dialogOptions.dialogOptionFlags ^=kNavAllowStationery;
	dialogOptions.dialogOptionFlags |=kNavDontAutoTranslate;
	
	/* Le nom par defaut */
	strcpy((Ptr)dialogOptions.savedFileName,nom);
	CtoPstr((Ptr)dialogOptions.savedFileName);
	/* Chemin par defaut */
	anErr=noErr;
	if (anErr == noErr)
		{	
		// Get 'open' resource. A nil handle being returned is OK,
		// this simply means no automatic file filtering.
		anErr = NavPutFile( NULL,  /* Position de demarrage */
							&reply, 	      /* Reponse */
							&dialogOptions,	  /* Options du dialogue */
							NULL,		  /* Traitement des events */
							'DHAR', 		 /* Type du fichier */
							'Harm', 		  /* Createur */
							NULL);			  /* Pas de call back parametre */
		if(anErr && anErr!=userCanceledErr)
			Collapse("	NavPutFile : Err %d",anErr);				
					
		if (anErr == noErr && reply.validRecord)
			{
			long count;
			anErr = AECountItems(&(reply.selection), &count);
			if (anErr == noErr)
				{
				AEKeyword theKeyword;
				DescType actualType;
				Size actualSize;
				FSSpec documentFSSpec;

				anErr = AEGetNthPtr(&(reply.selection), 1,
									typeFSS, &theKeyword,
									&actualType,&documentFSSpec,
									sizeof(documentFSSpec),
									&actualSize);
				if (anErr == noErr)
					{
					/* On recupere le nom */
					PtoCstr(documentFSSpec.name);
					strcpy(nom,(Ptr)documentFSSpec.name);
					FixeVolPath(documentFSSpec.vRefNum,documentFSSpec.parID);
					result=true;
					}
				}
			// Dispose of NavReplyRecord, resources, descriptors
			anErr = NavDisposeReply(&reply);
			}
		}
	}
return (result);
}
#endif
/*********************************************************************
                           SaisieNomFichierLecture
Saisi un nom de fichier pour une lecture
=> Adresse ou rnager le nom,extension avec laquelle filtrer
<= VRAI si ok
********************************************************************/
static pascal Boolean vect_filtre_selon_extension_sapiens(CInfoPBPtr info);
static char ExtensionCmp[10];
/*---------------------------------------------------------------
                Filtre pour la selection des fichiers selon leur extension
---------------------------------------------------------------*/
static pascal Boolean vect_filtre_selon_extension_sapiens(CInfoPBPtr info)
{
char nom[80];
register short i=0;

BlockMove((char*)info->hFileInfo.ioNamePtr,nom,79);
PtoCstr((StringPtr)nom);
while(nom[i])
    {
    if(nom[i++]=='.')
        {
        if(!stricmp(&nom[i],ExtensionCmp)) return(false);
        else            return(true);
        }
    }
return(true);
}

static Boolean SaisieNomFichierLecture(Ptr nom,char * extension)
{
#if  TARGET_API_MAC_CARBON>0 && !defined(ACAM2)

	return(GetFileNameLoadNavigationService(nom,extension,NULL));	

#else
Point point;
SFReply reply;
char chaine[255];
FileFilterUPP vect_filtre;

SetPt(&point,(qd.screenBits.bounds.right- 348)/2, (qd.screenBits.bounds.bottom-200)/2);
if(strlen(extension))
    {
    strcpy(ExtensionCmp,extension);
    sprintf(chaine,"Chargement du fichier :|*.%s|*.%s",extension,extension);
    vect_filtre=NewFileFilterProc(vect_filtre_selon_extension_sapiens);
    SFGetFile(point,(StringPtr)chaine,vect_filtre,-1,NULL,NULL,&reply);
    DisposeRoutineDescriptor(vect_filtre);
    }
else
    SFGetFile(point,(StringPtr)"",NULL,-1,NULL,NULL,&reply);

if(!reply.good) return(FAUX);
SetVol((StringPtr)"",reply.vRefNum);
PtoCstr((StringPtr)reply.fName);
strcpy(nom,(char*)reply.fName);
return(VRAI);
#endif
}
/*********************************************************************
                       CarboniseCvtPath
Supprime les ':' dans le nom du path
********************************************************************/
void CarboniseCvtPath(char * chaine);
void CarboniseCvtPath(char * chaine)
{
#if  TARGET_API_MAC_CARBON>0
short i,j;

i=j=0;
while(chaine[i])
	{
	if(chaine[i]!=':') chaine[j++]=chaine[i];
	i++;
	}
chaine[j]=0;
#endif
}
/******************************************************************
                    GetCheminCourant
Donne le chemin en cours sous forme d'un identificateur et d'un chemin
******************************************************************/
static Boolean GetCheminCourant(char * libPath,long * numPath)
{
short erreur;
WDPBRec wb;

wb.ioNamePtr=(StringPtr)libPath; /* adresse o— ranger (NULL si aucune) */
erreur=PBHGetVol(&wb,0);
if(erreur !=noErr) return(FAUX);
if(numPath) *numPath=wb.ioWDDirID;
if(libPath)
    PtoCstr((StringPtr)libPath);
return(VRAI);
}
#ifdef OBSOLETE
/******************************************************************
                    SetPath
Fixe le chemin au chemin demand‚.
Si on est en mode MAC c'est toujours le refNum qui sert.
Si on est en mode PC alors le libPath est prioritaire si il existe.
******************************************************************/
short SetPath(char * libPath,long refNum)
{
short erreur;
char chaine[255];
WDPBRec wb;

#ifdef MAC
        libPath=NULL;
#endif
wb.ioNamePtr=(StringPtr)libPath;
wb.ioVRefNum=0;
wb.ioWDDirID=refNum;
erreur=PBHSetVol(&wb,0);
if(erreur !=noErr)
    {
    sprintf(chaine,"SetPath: erreur %d",erreur);
    Collapse(chaine);
    }
return(noErr);
}
#endif
#ifdef OBSOLETE
/******************************************************************
                    FixeVolume
Fixe le volume
=> identificateur du volume, Booleen indiquant si on doit afficher l'erreur
******************************************************************/
OSErr FixeVolume(short volRefNum,Boolean afficheErreur)
{
OSErr err=noErr;
#ifdef MAC
err=SetVol(NULL,volRefNum);
#endif
return(err);
}
/******************************************************************
                    FixeVolPath
Fixe le volume et le path courant sur ce volume
******************************************************************/
OSErr FixeVolPath(short volRefNum,long path)
{
OSErr err;

err=FixeVolume(volRefNum,true);
if(err==noErr)
    {
    err=SetPath(NULL,path);
//    if(err) Collapse("Can't set path [%d,%ld]: error %d",volRefNum,path,err);
    }
return(err);
}


/******************************************************************
                    PreservePath
******************************************************************/
static long vRefNum;
static short volRefNum;
void PreservePath(void)
{
if(vRefNum)
    Collapse("PreservePath");
/* On memorise le Path */
SaisitChemin(&volRefNum,NULL,&vRefNum);
return;
}
/******************************************************************
                RestorePath
Remets le path au path courant
******************************************************************/
void RestorePath(void)
{
if(vRefNum==0)
    Collapse("RestorePath");
FixeVolPath(volRefNum,vRefNum);
vRefNum=0;
return;
}
/******************************************************************
                SetPathApplication
Remets le path de l'application
******************************************************************/
void SetPathApplication(void)
{
FixeVolPath(volumeApplication,PathApplication);
return;
}
/******************************************************************
                    SaisitChemin
Donne le chemin en cours sous forme d'un identificateur et d'un chemin
@= Identificateur du volume, Libelle du chemin, Identificateur du chemin
******************************************************************/
Boolean SaisitChemin(short * volRefNum,char * libPath,long * numPath)
{
short erreur;
WDPBRec wb;

if(volRefNum)
    SaisitVolume(volRefNum);
wb.ioNamePtr=(StringPtr)libPath; /* adresse o— ranger (NULL si aucune) */
erreur=PBHGetVol(&wb,0);
if(erreur !=noErr)
    {
    Collapse("PBHGetVol : error# %d",erreur);
    return(FAUX);
    }
if(numPath) *numPath=wb.ioWDDirID;
if(libPath)
	{
   	PtoCstr((StringPtr)libPath);
    }
return(VRAI);
}
/******************************************************************
                    SaisitVolume
Saisie le volume  en cours
******************************************************************/
void SaisitVolume(short * volRefNum)
{
#ifdef MAC
HVolumeParam hpb;
OSErr err;
char chaine[255];

if(GetVol(NULL,volRefNum)!=noErr)
    Collapse("Impossible de saisir le lecteur en cours");
memset(&hpb,0,sizeof(HVolumeParam));
hpb.ioVolIndex=-1;                  /* Utiliser le nom ou le num */
hpb.ioVRefNum=*volRefNum;           /* Rechercher selon le nom -35 s'il n'existe pas */
hpb.ioNamePtr=(StringPtr)NULL;                  /* Nom a rechercher (termin‚ par : )*/
err=PBHGetVInfo((HParmBlkPtr)&hpb,false);
if(err)
    {
    sprintf(chaine,"SaisitVolume: erreur %d",err);
    Collapse(chaine);
    return;
    }
*volRefNum=hpb.ioVRefNum;
#else
    *volRefNum=1562;//Bidon
#endif
}
/******************************************************************
                FixeChemin
Fixe le chemin donn‚es en texte
=> chemin de base, sous dir
******************************************************************/
OSErr FixeChemin(long pathBase,char * path)
{
OSErr erreur;
char chaine[255];
WDPBRec wb;

strcpy(chaine,path);
CtoPstr(chaine);
wb.ioNamePtr=(StringPtr)chaine;
wb.ioVRefNum=0;
wb.ioWDDirID=pathBase;
erreur=PBHSetVol(&wb,0);
return(erreur);
}
/******************************************************************
                FixeCheminRelatif
Fixe le chemin donn‚es en texte
=> chemin de base, sous dir
******************************************************************/
OSErr FixeCheminRelatif(short volRef,long pathBase,char * path)
{
OSErr erreur;
char chaine[255];
WDPBRec wb;

strcpy(chaine,path);
CarboniseCvtPath(chaine);			
CtoPstr(chaine);
wb.ioNamePtr=(StringPtr)chaine;
wb.ioVRefNum=volRef;
wb.ioWDDirID=pathBase;
erreur=PBHSetVol(&wb,0);
return(erreur);
}
#endif

/******************************************************************
                    InitCheminsSapiens
Initialise les chemins de l'application
******************************************************************/
void InitCheminsSapiens(void)
{
/* On saisi les chemins : chemin par defaut de l'application, chemin par
defaut des donn‚es de l'application*/
SaisitChemin(&volumeApplication,NULL,&PathApplication);
SetPathApplication();
return;
}
/*****************************************************************************
                   AJOUTE_EXTENSION
=> Nom du fichier, Extension du fichier
*****************************************************************************/
static void ajoute_extension(char * nom, char * ext)
{
while(*nom && *nom !='.') nom++;

*nom++='.';
*nom++=*ext++;
*nom++=*ext++;
*nom++=*ext++;
*nom++=0;
return;
}
/*********************************************************************
                           SaisieNomFichierAventureEcriture
Saisi un nom de fichier pour une ecriture
=> Prompt,nom,format ou 0 si aucun
   si protect == NULL aucune saisie n'est faite
<= Format du fichier ou 0 si aucun
********************************************************************/
static Boolean SaisieNomFichierAventureEcriture(Ptr nom)
{
#if  TARGET_API_MAC_CARBON>0 && !defined(ACAM2)

	return(GetFileNameSaveNavigationService(nom));

#else
Point point;
SFReply reply;
char chaine[255];
char titre[255];

SetPt(&point,(qd.screenBits.bounds.right- 304)/2, (qd.screenBits.bounds.bottom-184)/2);
strcpy(chaine,nom);

CtoPstr(chaine);
if(langue==FR) strcpy(titre,"Fichier");
else if(langue==ES) strcpy(titre,"Archivo");
else strcpy(titre,"File");
CtoPstr(titre);
SFPutFile(point,(StringPtr)titre,(StringPtr)chaine,NULL,&reply);
if(!reply.good) return(FAUX);
SetVol((StringPtr)"",reply.vRefNum);
PtoCstr((StringPtr)reply.fName);
strcpy(nom,(Ptr)reply.fName);
return(VRAI);
#endif
}
/***********************************************************************
                                SAUVE:
***********************************************************************/
static 	 char nom[255]="";

void sauve(void)
     {
     FILE * fichier;
     char * ptr;
     short checksum=111;
     short car;
     short i,j;

     flefba=0;
     if (jour_act==jour_dsk)
                {
                comment(113);/*Une seule aventure par jour*/
				#if DEBUG<1
                return;
				#endif
                }
     flefba=0;

     PreservePath();
	 if(strlen(nom)==0)    strcpy(nom,nomjoueur);
     if(!SaisieNomFichierAventureEcriture(nom))
        {
        RestorePath();
        return;
        }
     ajoute_extension(nom,"sap");
     remove(nom);
     fichier=fopen(nom,"wb");
     if (fichier==NULL)
          {
          flefba=0;
          comment(92);/*erreur*/
          RestorePath();
          return;
          }
     j=0;
     ptr=(char *)soma[j].adresse;
     putc('G',fichier);
     putc('O',fichier);
     putc('D',fichier);
     initrnd('S','A','P');
     while( ptr )
          {
          for (i=0;i<soma[j].taille;i++)
               {
               car=*(ptr++)^rnd0();
               checksum +=car;
               putc(car,fichier);
               }
          j++;
          ptr=(char *)soma[j].adresse;
          }
     checksum &=255;
     putc(checksum,fichier);
     SauveEltsDecorsMobiles(fichier);
     fclose(fichier);
     fichier=fopen(nom,"rb");
     if (getc(fichier) != 'G')
                    {
                    flefba=0;
                    comment(92);/*erreur*/
                    RestorePath();
                    return;
                    }
     else
                jour_dsk=jour_act;
     fclose(fichier);
     RestorePath();
     return;
     }
/***********************************************************************
                                CHARGE:
***********************************************************************/
void charge(void)
     {
     FILE * fichier;
     char  nom[255];
     char * ptr;
      short checksum=111;
     short car;
     register short i,j;
     flefba=0;

     PreservePath();
     if(!SaisieNomFichierLecture(nom,"sap"))
            {
            RestorePath();
            return;
            }
     ajoute_extension(nom,"sap");
     fichier=fopen(nom,"rb");
     flefba=0;
     if (fichier==NULL)
          {
          comment(92);
          RestorePath();
          return;
          }
     primsap();
     j=0;
     ptr=(char *)soma[j].adresse;
     initrnd('S','A','P');
     if (getc(fichier) =='G' && getc(fichier)=='O' && getc(fichier)=='D')
          {
          while(ptr)
               {
               for (i=0;i<soma[j].taille;i++)
                    {
                    car=getc(fichier);
                    if (car<0)
                         {
                         flefba=0;
                         comment(92);/*erreur*/
                         fclose(fichier);
                         RestorePath();
                         return;
                         }
                    checksum +=car;
                    car=car^rnd0();
                    *(ptr++)=car;
                    }
               j++;
               ptr=(char *)soma[j].adresse;
               }
          checksum &=255;
          if (checksum != getc(fichier))
               {
               flefba=0;
               comment(92);/*erreur*/
               primsap();/*Nyak nyak !!!*/
               mort=0;
               }
          }
     else
          {
          comment(92);/*erreur*/
          }
     CreeDecor3d(true);
     ChargeEltsDecorsMobiles(fichier);
     fclose(fichier);
     minute=zj=0;
     chrono(0);/*pour couleur*/
     jour_dsk=jour_act;
     affich(dzeta);
     tfrima();
     generique(VRAI);
     if (scenar==6)
        {
        sceveu(6);/* Ouaaais c'est lui !!!!!*/
        }
     RestorePath();
     return;
     }
/***********************************************************************
                                TRGEST:gestion
***********************************************************************/
void trgest(unsigned short regb)
 {

 MontreSouris();
 switch(regb)
      {
      /* Informations */
      case 0: //DoInfo();
      		   DoAide();
              //sceveu(8);
              break;
    /* Charge aventure */
      case 1: if(ConfirmePerteAventure())
                {
                charge();
                inimen(8);
                }
              break;
      /* Sauve aventure */
      case 2: sauve();
                break;
     /* Personnage : nouvlle aventure */
      case 3:
                FixeMenusReduit();
                if(ConfirmePerteAventure())
                     defsap();
                FixeMenusComplet();
             break;
      case 4: DoPreferences();
              break;
      #ifdef OLD_VERSION
      /* Aide au clavier - AIde autonome */
      case 4:
          liste8.elements[4] ^=1 ;
          inimen(8);
          break;
     /* Musique Midi - Musique DMA */
      case 5:
                liste8.elements[5] ^=1 ;
                okmuse=liste8.elements[5] & 1;
                MusicInit(1);
                reset_synthe();
                inimen(8);
                break;
      /* Reglage Midi */
      case 6:
/***********************************************************************
                        AJUSTE:Ajuste MIDI
***********************************************************************/
void ajuste(void)
     {
     short car=0;

     sceveu(10);
     reset_clav();
     while (!car) car=litclav() & 223;
   if (car>='A' && car<='C') quel_synthe(car-'A');
     return;
     }
            effima(0x443);/*10*/
            tfrima();
            dervue=VUE_AUCUNE;
            ajuste();
            break;

     case 7 :
          inimen(8);
          break;
     #endif
      }
 }
/***********************************************************************
                      BRAVO:Affiche resultat de la taille
***********************************************************************/
static void bravo(unsigned short quoi)
{
short * TabIndexPalette;
float coeff;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
eff_sour();
/* Le fond est blanc */
effima(COULEUR_FOND_NUCLEUS);
for(coeff=0.0;coeff<1.0;coeff+=0.05)
    {
    GereInterface();
    TabIndexPalette=CalculePaletteAttenuee(quoi,coeff,999);
    Affmobbrt(quoi,96,1,0,TabIndexPalette);
    if(TabIndexPalette)  DisposPtr((Ptr)TabIndexPalette);
    tfrima();
    }
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************
                                TRFAB:Fabrique silex
***********************************************************************/
void trfab(unsigned short regb)
 {
 char i;
 short trace_syl;
 short noir_noir[8]={0,0,0,0,0,0,0,0};
 short * tfr;

 FixeMenusReduit();
 flefba=0;
 clrbadi();
 /*recherche si silex */
 i=liste1.taille-1;
 while (i >=0 && liste1.elements[i] !=19) i--;
 if (i<0) /*pas trouve ds la liste*/
      {
      comment(87);/*tu n'as pas de silex*/
      return;
      }
  dervue=VUE_AUCUNE;
  tfr=ciel_degr;
/* on taille le silex */
  bruit(10);
 trace_syl=cursil(regb);
 palhaute=pal2;
 flefba=0;
 if (trace_syl>PRECISION_SILEX) /* trac‚ et incertitude du dit*/
      {
      bruit(7);/* HELAS */
      /*affichage du silex cass‚*/
      bravo(MOB_SILEX_CASSE);
      liste1.elements[i]=20;/*un caillou !!!*/
      /*for (i=0;i<5;i++) bruit(3);*/
      comment(86);
      }
 else
      {
      bruit(6);/* VICTOIRE*/
      if (trace_syl<150) comment(110);
      if (regb !=1)
           {
           liste1.elements[i]=23;/*sagaie*/
           bravo(MOB_SILEX_SAGAIE);
           }
      else
           {
           liste1.elements[i]=21;/*hache*/
           bravo(MOB_SILEX_HACHE);
           }
      comment(85);/*tu pssede une nll arme*/

      }
 attends_bruit();
 bande(0);
 generique(VRAI);
 ciel_degr=tfr;
// CalculeDegradeCiel(tfr);
 FixeMenusComplet();
 return;
 }
/*----- FIN -----*/
