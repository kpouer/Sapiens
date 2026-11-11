/*************************************************************************
                    SapW3d.C
                (c) Og le 19-1-1996
Gestion C de la 3d
***************************************************************************/
#include "sapiens.h"
#include <math.h>
#define M_PI 3.14159265
extern Forme nuage1;
static char TampDens[16];
static char DensDzet[10];
/*; corresp densite->couleur ecran chaque trame*/
static char TabTram[]=
        {3,4,5,6,
         4,5,6,6,
         3,4,4,5,
/*; fumee/nuages*/
         10,10,0,0,
/*;TRONCS*/
         3,3,4,5,
/*;FEUILLES*/
         11,14,12,12,
         11,14,14,12,
         11,11,14,12,
/*;NEIGE*/
         10,0,0,0,
     10,10,0,0};
static short * * TabPaletteDistance;
static IndexColor * TabCouleursCiel;
static short CouleurOrageJour,CouleurOrageNuit;

/**************************************************************************
                        sinshort
***************************************************************************/
short sinshort(short angle)
{
return(255*sin((float)angle*3.1415925/512.0));
}
/**************************************************************************
                        cosshort
***************************************************************************/
short cosshort(short angle)
{
return(255*cos((float)angle*3.1415925/512.0));
}

/*****************************************************************************
                                        distance
*****************************************************************************/
short distance(long dx,long dy)
{
dx=labs(dx);
dy=labs(dy);
if(dx>dy) dx=dx+dy*2L/5L;
else dx=dy+dx*2L/5L;
if(dx>32700) dx=32700;
return(dx);
}
/************************************************************************
                        caldisI
=> x,y,z, rayon d'un disque
@= dx,dy,dz distance
<= erreur (0 ok)
*************************************************************************/
static short caldisI(unsigned short x,unsigned short y,unsigned short z,
                     unsigned short rayon,
                     short *pdx,short *pdy,short *pdz,
                     short *pdistance)
{
long dx,dy,dz;
long dist;

dx=(long)x-(long)xj;
dy=(long)y-(long)yj;
dz=(long)z-(long)zj;
if(labs(dx)<32000 && labs(dy)<32000)
    {
    *pdx=dx;
    *pdy=dy,
    *pdz=dz;
    dist=sqrt(dx*dx+dy*dy)-rayon;
    if(dist<0) dist=0;
    if(dist<20000)
        {
        *pdistance=dist;
        return(0);
        }
    }
*pdistance=-1;
return(2);
}
/************************************************************************
                        disk3dI
=> x,y,z, rayon d'un disque
@= xecr gauche, x ecr droit , distance, delta z, angle
<= erreur (0 ok)
*************************************************************************/
static short disk3dI(unsigned short x,unsigned short y,unsigned short z,
                     unsigned short rayon,
                     short *pxg,short *pxd,short *pdz,short *pdist,
                     short * angabs)
{
short erreur,rayecr;
short dx,dy,dz,dist,angle,posg,posd;

posg=posd=0;
erreur=caldisI(x,y,z,rayon,&dx,&dy,&dz,&dist);
if(!erreur)
    {
    /* Onzedisk */
    if(dist<dismin)
        {
        if(dist) dist=1;
        erreur=1;
        }
    *pdist=dist;
    /* Angle du centre de la montagne */
    if(dx==0)
        {
        if(dy>0)angle=256;
        else angle=768;
        }
    else angle=(atan2(dy,dx)*512.0)/M_PI;
    *angabs=angle;
    angle=(angle-dzeta)&1023;
    /* Angle occup‚ par le disque */
    if(dist==0)
        {
        posg=0;
        posd=255;
        }
    else
        {
        rayecr=(atan2(rayon,dist)*512.0)/M_PI;
        posg=(angle+rayecr)&1023;
        posd=(angle-rayecr)&1023;
        if(posg>512) posg=posg-1024;
        if(posd>512) posd=posd-1024;
        posg=(128-posg);
        posd=(128-posd);
        if(posg>posd) erreur=3;
        }
    }
*pdz=dz;
*pxg=posg;
*pxd=posd;
*pdist=dist;
return(erreur);
}

/*************************************************************************
;_-_-_-_-_-_-_-_-_- Calcoul: calcule tableau coul -_-_-_-_-_-_-_-_-_-_
; => d0: n. trame
**************************************************************************/
static void calcoul(short trame)
{
short i;
short n;

for(i=0;i<9;i++)
    {
    n=DensDzet[i];
    tabcoul[i+1]=TabTram[trame*4+n];
    }
return;
}
/*****************************************************************************
;-_-_-_-_-_-_-_-_-_ Visu: visualise montagnes 3D -_-_-_-_-_-_-_-_-_-_-_
;=> (c) pointeur vers debut structure montagne
;<= montagnes   sur ecran
; WARNING !!! Taille de la structure modifi‚e par le 68000 !!!!!
; Now:
; Ptr Chainage   Dist  PosxB  PosYB  IGN  Ptr Forme    G   D   H   B
;   _ _ _ _       _ _   _ _    _ _   _ _   _  _  _  _  __  __  __  __
;   0 1 2 3       4 5   6 7    8 9  10 11  12 13 14 15 16  18  20  22
; Chaque forme ‚tant compos‚e de:
; NbDisk  Type Trame   Hbase  RayonB  R1 .. R16  DX1  DY1 ... DX16 DY16
;   _         _         _ _    _ _    _ _ ..32oct _    _ ..32oct _   _
;   0         1         2 3    4 5    6 7        38   39        ......
******************************************************************************/
static char TabEcl[]=
    {
         2,2,1,1,1,1,0,0,0,0,0,0,1,1,1,1,
         2,2,2,1,1,1,0,0,0,0,0,0,1,1,1,2,
         2,2,2,1,1,1,1,0,0,0,0,1,1,1,1,2,
         2,2,2,2,1,1,1,0,0,0,0,1,1,1,2,2,
         3,2,2,2,1,1,1,0,0,0,0,1,1,1,2,2,
         3,3,2,2,2,1,1,1,0,0,1,1,1,2,2,2,
         3,3,3,2,2,2,1,1,0,0,1,1,2,2,2,3,
         3,3,3,2,2,2,1,1,0,0,1,1,2,2,2,3,
         3,3,3,3,2,2,2,1,0,0,1,2,2,2,3,3,
         3,3,3,3,2,2,2,1,0,0,1,2,2,2,3,3,
         3,3,3,3,2,2,2,1,1,0,1,2,2,2,3,3,
         3,3,3,3,2,2,2,1,1,0,1,2,2,2,3,3,
         2,3,3,3,3,2,2,1,1,1,1,2,2,3,3,3,
         2,2,3,3,3,2,2,1,1,1,1,2,2,3,3,3,
         2,2,2,3,3,3,2,2,2,1,2,2,3,3,3,2,
         2,2,2,2,3,2,2,2,2,2,2,2,2,3,2,2
    };


void visu2(MontPtr mont )
{
short j;
short i,d1;
FormePtr forme;
short CpTram;
short Oscil=0;
short ligne,gauche,droite,newgauche,newdroite;
short dx,dz,dist,angle,hauteurDisque;
short err;
short xd,yd,zd,rd,hmec,angabs;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
/* Calcule le tableau d'ensoleillement en fonction du jour et de la nuit
Jour ou Nuit */
d1=((((76-metpos)&0xFF)/6)<<4)&240;
if(heure&256) j=d1;
else   j=d1+8;
for(i=0;i<16;i++)
    TampDens[i]=TabEcl[j++];

hmec=10;
while(mont)
    {
    forme=mont->ptrforme;
    CpTram=0;
    Oscil=0;
    xd=mont->posx;
    yd=mont->posy;
    zd=forme->hauteur;
    rd=forme->rayon;
    err=disk3dI(xd,yd,zd,rd,&gauche,&droite,&dz,&dist,&angabs);
    /* Calcul des couleurs */
    j=((angabs-480)&1023)>>6;
    /* 8 couleurs vues par le joueur */
    for(i=0;i<9;i++)
        DensDzet[i]=TampDens[(j++)&15];
    calcoul(forme->relief);
    /* Pas d'erreur */
    angle=(short)((atan2(dz,dist)*512.0)/M_PI)&1023;
    if(angle>512) angle=angle-1024;
    ligne=64-angle;
    hauteurDisque=(short)((atan2(mont->ign,dist)*512.0)/M_PI)&1023;
    /* On boucle pour tous les disques */
    for(i=0;i<forme->nbrdisk && err<2;i++)
        {
        if(err)
            {
            hmec+=mont->ign;
            ligne=100;
            }
        err=0;
        if(ligne<320)
            {
            zd+=mont->ign;
            rd=forme->dskrayon[i];
            dx=forme->dskdelta[i*2];
            /* Changement de trame */
            if((dx&255)==128)
                {
                dx=forme->dskdelta[i*2+1];
                /* On calcule la trame */
                calcoul(dx);
                /* Et pas de delta */
                xd=mont->posx;
                yd=mont->posy;
                }
            else
                {
                /* On applique les delta */
                xd=mont->posx+dx;
                yd=mont->posy+forme->dskdelta[i*2+1];
                }
            err=disk3dI(xd,yd,zd,rd,&newgauche,&newdroite,&dz,&dist,&angabs);
            if(err)
                {
                gauche=newgauche;
                droite=newdroite;
                }
            else
                {
                if(hauteurDisque==0)
                    {
                    Oscil^=1;
                    if(Oscil)
                    TraceDisque(&gauche,&droite,&ligne,newgauche,newdroite,1);
                    }
                else
                    TraceDisque(&gauche,&droite,&ligne,newgauche,newdroite,hauteurDisque);
                }
            }
        }
    mont=mont->chaine;
    }
zj=hmec;
AcamSetPortOffscreen(gbak);
return;
}


/*******************************************
            CalcPoint3D
=> x,y,z joueur, angle visee, x,y,z point
@= x,y ecran, distance
<= VRAI si visible
********************************************/
#define LARGEUR_ECRAN 512
#define HAUTEUR_ECRAN 256
static short CalcPoint3D(long xj,long yj,long zj,
                       float ca,float sa,
                       long xp,long yp,long zp,
                       short * x,short * y,short * dist)
{
long tmp;
long xl,yl;

/* --- Rabat les positions du point en fn du joueur  --- */
xp-=xj;
yp-=yj;
zp-=zj;

/* --- Recalcule les positions 3d en fonction de l'angle de vue --- */
tmp=(float)xp*ca-(float)yp*sa;
yp=(float)xp*sa+(float)yp*ca;
xp=tmp;

/* --- Visible ? --- */
if(xp<=2) return(0);
tmp=distance(xp,yp);
*dist=tmp;
if(tmp==0) return(0);
/* --- Calcule x ‚cran --- */
xl=LARGEUR_ECRAN/2-(long)(LARGEUR_ECRAN/2)*(long)yp/(long)xp;

/* --- Calcule Y ecran --- */
yl=HAUTEUR_ECRAN/2-(long)(HAUTEUR_ECRAN/2)*(long)zp/(long)xp;

if(xl<-32760) xl=-32760;
if(xl>32760) xl=32760;
if(yl<-32760) yl=-32760;
if(yl>32760) yl=32760;
*x=xl;
*y=yl;

return(1);
}

/************************************************************************
;_-_-_-_-_-_-_-_-_-_-_ SapDep : deplacement du joueur -_-_-_-_-_-_-_-_
; Entr‚es -C- 1.Angle d'avanc‚e
;             2.Vitesse de progression
;
************************************************************************/
void sapdep(short angle,short vitesse)
{
long tmp;

tmp=(long)xj+(long)(((long)cosshort(angle)*(long)vitesse)>>8);
if(tmp<0) tmp=0;
if(tmp>65535) tmp=65535;
xj=tmp;
tmp=(long)yj+(long)(((long)sinshort(angle)*(long)vitesse)>>8);
if(tmp<0) tmp=0;
if(tmp>65535) tmp=65535;
yj=tmp;
AvanceSol(vitesse);
return;
}
/****************************************************************************
                    CalculeDegradeDuCiel
Calcul des tableau de degrades du ciel
*****************************************************************************/
static short TabLign[]={0,41*2,48*2,53*2,57*2,59*2,61*2,64*2};
static void CalculeDegradeDuCiel(short * palette,IndexColor * TabCouleursCiel)
{
short j,i;
long dist;
RGBColor rgbDep,rgbDest;
long incRed,incBlue,incGreen;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
for(i=j=0;i<NB_LIGNES_CIEL;i++)
        {
        /* Changement de ligne */
        if(i==TabLign[j])
                {
                CvtCouleurEnRgb(palette,j++,&rgbDep);
                CvtCouleurEnRgb(palette,j,&rgbDest);
                dist=TabLign[j]-TabLign[j-1];
                incRed=((long)rgbDest.red-(long)rgbDep.red)/dist;
                incGreen=(long)((long)rgbDest.green-(long)rgbDep.green)/dist;
                incBlue=(long)((long)rgbDest.blue-(long)rgbDep.blue)/dist;
                }
        rgbDep.red+=incRed;
        rgbDep.green+=incGreen;
        rgbDep.blue+=incBlue;
        TabCouleursCiel[i].rgb=rgbDep;
        TabCouleursCiel[i].index=SapColor2Index(&rgbDep);
        }
AcamSetPortOffscreen(gbak);
return;
}
/****************************************************************************
                    CalculeDegradeDistance
Calcule le degrade des montagnes en fonction de la distance au joueur
Le premier degrade est celui de premier plan, le dernier celui de l'horizon
*****************************************************************************/
static float IotaPaletteStd=0.14;
static void CalculeDegradeDistance(short heure,short **TabPaletteDistance)
{
short i,pos;
float coeff, delta;

if(heure>512-50 || heure<50 )  /* Nuit */
        {
        if(heure<50) pos=heure;
        else  pos=512-heure;
        coeff=0.7+(float)pos/200.0+IotaPaletteStd;
        delta=0.5/(float)NB_DEGRADE_DISTANCE;
        }
else
        {
        /* c'est le jour */
        coeff=1.0+IotaPaletteStd;
        delta=0.4/(float)NB_DEGRADE_DISTANCE;
        }
for(i=0;i<NB_DEGRADE_DISTANCE;i++)
        {
        if(TabPaletteDistance[i]) DisposePtr((Ptr)TabPaletteDistance[i]);
        TabPaletteDistance[i]=CalculePaletteAttenuee(mob3D+1,coeff,33);
        coeff-=delta;
        if(coeff<0) coeff=0;
        }
return;
}
/****************************************************************************
                    CalculeToutDegrades
Calcule l'ensemble des degrades de palette en fonction de l'heure en cours
*****************************************************************************/
void CalculeToutDegrades(void)
{
CGrafPtr gbak;
short i;
RGBColor rgb;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
/* Pour l'ensemble des evenements on calcule le degrade */
for(i=0;i<NBREVTHEURE;i++)
        {
        if(tabheur[i].q)
                {
                if(tabheur[i].TabCouleursCiel) DisposPtr((Ptr)tabheur[i].TabCouleursCiel);
                if(tabheur[i].TabPaletteDistance) DisposPtr((Ptr)tabheur[i].TabPaletteDistance);
                tabheur[i].TabCouleursCiel=(IndexColor*)NewPtrClear(sizeof(IndexColor)*NB_LIGNES_CIEL);
                tabheur[i].TabPaletteDistance=(short**)NewPtrClear(sizeof(short*)*NB_DEGRADE_DISTANCE);
                CalculeDegradeDuCiel(tabheur[i].pal,tabheur[i].TabCouleursCiel);
                CalculeDegradeDistance(tabheur[i].h,tabheur[i].TabPaletteDistance);
                }
        }
rgb.red=rgb.green=rgb.blue=20000;
CouleurOrageJour=SapColor2Index(&rgb);
rgb.red=rgb.green=rgb.blue=8000;
CouleurOrageNuit=SapColor2Index(&rgb);
AcamSetPortOffscreen(gbak);
return;
}
/****************************************************************************
                    ChangeIotaPalette
*****************************************************************************/
void ChangeIotaPalette(float combien)
{
IotaPaletteStd+=combien;
CalculeToutDegrades();
if(dervue==VUE_PAYSAGE)
    {
      affich(dzeta);
      tfrima();
    }
return;
}
/****************************************************************************
                    FixeDegradesFonctionHeure
En fonctione de l'evenement de changement de l'aire injecte les tabelaux de couleurs
*****************************************************************************/
void FixeDegradesFonctionHeure(struct h_chron * chron)
{
TabCouleursCiel=chron->TabCouleursCiel;
TabPaletteDistance=chron->TabPaletteDistance;
return;
}
/****************************************************************************
                    CalculeEtoiles
            Effacement de la 3d
*****************************************************************************/
#define AIRE_X_ETOILE 2048
#define AIRE_Y_ETOILE 128
#define MAX_MAGNITUDE 16
typedef struct
        {
        short x; /* 0 … AIRE_X_ETOILE */
        short y; /* 0 … 128 */
        short magnitude;
        short couleur;
        } Etoile,*EtoilePtr;
static EtoilePtr TabEtoiles;
static short CoulPulse;
static short meteore,xMeteore,yMeteore;
void CalculeEtoiles(void)
{
long i;
CGrafPtr gbak;
RGBColor rgb;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
/* Couleur de l'etoile */
if(TabEtoiles) DisposPtr((Ptr)TabEtoiles);
TabEtoiles=(EtoilePtr)NewPtr((long)NB_ETOILES*(long)sizeof(Etoile));
if(TabEtoiles==NULL) return;
rgb.red=rgb.green=rgb.blue=65535L;
CoulPulse=SapColor2Index(&rgb);
for(i=0;i<NB_ETOILES;i++)
        {
        TabEtoiles[i].x=myRandom(0,AIRE_X_ETOILE);
        TabEtoiles[i].y=myRandom(0,AIRE_Y_ETOILE);
        TabEtoiles[i].magnitude=myRandom(0,MAX_MAGNITUDE);
        rgb.red=rgb.green=55000L-(long)TabEtoiles[i].y*(32768L/(long)AIRE_Y_ETOILE);
        rgb.blue=0;
        TabEtoiles[i].couleur=SapColor2Index(&rgb);
        }
AcamSetPortOffscreen(gbak);
return;
}
/****************************************************************************
                    AfficheEtoiles
Affiche les etoiles sur le ciel
*****************************************************************************/
void AfficheEtoiles(void)
{
long i;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
short x,y;
short magnitude; /* Magnitude maximum d'affichage des etoiles */
EtoilePtr p;

if(heure>480 || heure<20 )  /* Nuit */
        {
        data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
        if(data==NULL) return;
        p=TabEtoiles;
        if(heure<MAX_MAGNITUDE) magnitude=MAX_MAGNITUDE-heure;
        else if(heure<480+MAX_MAGNITUDE) magnitude=heure-(480+MAX_MAGNITUDE);
        else    magnitude=MAX_MAGNITUDE;
        for(i=0;i<NB_ETOILES;i++,p++)
                {
                if(p->magnitude<=magnitude )
                        {
                        x=((dzeta*2-p->x-512)&2047)-512+256;
                        if(x>=0 && x<512)
                                {
                                y=p->y;
                                if( Rnd()>127) *(data+((long)y*(long)tailleLigne)+(x))=CoulPulse;
                                else *(data+((long)y*(long)tailleLigne)+(x))=p->couleur;
                                }
                        }
                }
        if(meteore)
                {
                x=((dzeta*2-xMeteore-512)&2047)-512+256;
                Affmobbrt(mob3D+21,x/ECHELLE,yMeteore/ECHELLE,0,NULL);
                meteore--;
                xMeteore+=5;
                yMeteore+=5;
                }
        else
        if(myRandom(0,1000)>990)
                        {
                        xMeteore=myRandom(0,AIRE_X_ETOILE);
                        yMeteore=myRandom(0,AIRE_Y_ETOILE/2);
                        meteore=myRandom(3,6);
                        }
        /* Etoile polaire */
        x=((dzeta*2+512)%2048)-512+256;
        Affmobbrt(mob3D+myRandom(22,24),x/2,20,0,NULL);
        }
}

/****************************************************************************
                    Affichage du Sol
*****************************************************************************/
#define NB_INTERV 10
#define LARGEUR_BANDE_SOL 80
#define LARGEUR_MOB 256L
#define HAUTEUR_MOB 32L
#define HAUTEUR_MIN_BANDE 5
typedef struct
        {
        short y;
        short hauteur;
        short largeur;
        short offsetEnMob;      /* Position X dans le Mob qui apparait au milieu de l'‚cran */
        short offsetEcran;
        short dist;
        } LigneSol,*LigneSolPtr;

/* Les lignes sont organisees du plus loin au plus pres. */
static  LigneSol TabLigneSol[NB_INTERV];
static short lastDzeta;
static long SolRandSeed=0x2548795AL;


/*****************************************************************************
                                        CalculeDimensionsLigne
                                Calcule les nouvelles dimensions des lignes du sol
******************************************************************************/
static void CalculeDimensionsLignes(void)
{
short i;

for(i=1;i<NB_INTERV;i++)
                {
                TabLigneSol[i].hauteur=TabLigneSol[i].y-TabLigneSol[i-1].y;
                TabLigneSol[i].largeur=(long)LARGEUR_MOB*(long)TabLigneSol[i].hauteur/(long)HAUTEUR_MOB;
                }
TabLigneSol[0].hauteur=TabLigneSol[0].y;
TabLigneSol[0].largeur=(long)LARGEUR_MOB*(long)TabLigneSol[0].hauteur/(long)HAUTEUR_MOB;

return;
}

/*******************************************************************************
                                                                TourneSol
                                Le personnage tourne la tete, le tableau des lignes du sol bouge
*********************************************************************************/
static void TourneSol(void)
{
short i,delta;

if(dzeta !=lastDzeta)
        {
        delta=dzeta-lastDzeta;
        for(i=1;i<NB_INTERV;i++)
                {
                if(TabLigneSol[i].hauteur)
                        {
                        TabLigneSol[i].offsetEnMob=TabLigneSol[i].offsetEnMob+
                                                (long)(delta*2)*(long)HAUTEUR_MOB/(long)TabLigneSol[i].hauteur;
                        TabLigneSol[i].offsetEnMob%=LARGEUR_MOB;
                        TabLigneSol[i].offsetEcran=-TabLigneSol[i].offsetEnMob*(long)TabLigneSol[i].hauteur/(long)HAUTEUR_MOB;
                        }
                }
        lastDzeta=dzeta;
        }
}


/******************************************************************************
                                                        CalcY
        Calcule le Y ecran en fonction du delta d'altitude et de la distance
*******************************************************************************/
static short CalcY(short idx)
{
short dist;

dist=TabLigneSol[idx].dist;
return(256L*(long)(38+zj/20)/(long)dist);
}

/*******************************************************************************
                                                                AvanceSol
                                Le personnage avance, le tableau des lignes du sol bouge
*********************************************************************************/
 void AvanceSol(short dist)
{
short i,d;

if(dist) SolRandSeed=TickCount()+xj+yj;
/* Est-ce l'init ? oui -> on calcule tout*/
if(TabLigneSol[0].dist==0)
        {
        d=70;
        for(i=0;i<NB_INTERV;i++)
                {
                TabLigneSol[NB_INTERV-1-i].dist=d;
                TabLigneSol[NB_INTERV-1-i].y=CalcY(NB_INTERV-1-i);
                d+=LARGEUR_BANDE_SOL;
                }
        }
else

        {
        for(i=0;i<NB_INTERV;i++)
                {
                TabLigneSol[i].dist-=dist;
                TabLigneSol[i].y=CalcY(i);
                }
        /* La ligne la plus proche s'est-elle barr‚e ? */
        while(TabLigneSol[NB_INTERV-1].dist<70)
                {
                /* On d‚cale le tableau */
                for(i=NB_INTERV-1;i>0;i--) TabLigneSol[i]=TabLigneSol[i-1];
                /* On recalcule le dernier ‚l‚ment */
                TabLigneSol[0].dist=TabLigneSol[1].dist+LARGEUR_BANDE_SOL;
                TabLigneSol[0].y=CalcY(0);
                }
        }
return;
}

/*******************************************************************************
                                                                AfficheSol
                                On affiche les lignes du sol
*********************************************************************************/
static void AfficheSol(void)
{
Rect r,rdep;
short i,j,k,hauteur,largeur,offset,numPalette;
short numMob,ymax;
long RandSeedBak;

AvanceSol(0);
CalculeDimensionsLignes();
TourneSol();
/* Trace toutes les rang‚es */
/* On recherch la premiere ligne qui n'est pas au loin */
for(i=1;i<NB_INTERV && TabLigneSol[i].hauteur<4;i++)/**/;

/* On trace toutes les lignes au loin */
if(i<NB_INTERV) ymax=TabLigneSol[i].y+NB_LIGNES_CIEL; /* 1e ligne utile */
else ymax=NB_LIGNES_CIEL+NB_LIGNES_TERRE;
largeur=(long)HAUTEUR_MIN_BANDE*(long)LARGEUR_MOB*4L/(long)HAUTEUR_MOB;
RandSeedBak=CarboniseGetRandSeed();
CarboniseSetRandSeed(SolRandSeed);
for(j=ymax;j>NB_LIGNES_CIEL;j-=HAUTEUR_MIN_BANDE) /* des bandes de 4 */
        {
        offset=(2048-dzeta*2+Rnd())%largeur;
        if(j<NB_LIGNES_CIEL+HAUTEUR_MIN_BANDE)
                SetRect(&r,256-offset,NB_LIGNES_CIEL,256+largeur-offset,j);
        else
                SetRect(&r,256-offset,j-HAUTEUR_MIN_BANDE,256+largeur-offset,j);
        if (r.bottom>NB_LIGNES_CIEL) numPalette=(NB_LIGNES_CIEL+NB_LIGNES_TERRE)-r.bottom;
        else numPalette=NB_LIGNES_CIEL-NB_LIGNES_TERRE-r.bottom;
        if(numPalette>=NB_DEGRADE_DISTANCE) numPalette=NB_DEGRADE_DISTANCE-1;
        if(numPalette<0) numPalette=0;
        /* Trace la partie droite */
        rdep=r;
        for(k=256-offset;k<512;k+=largeur)
                {
                Affmob3d(mob3D+25,&r,TabPaletteDistance[numPalette]);
                OffsetRect(&r,largeur,0);
                }
        r=rdep;
        for(k=256-offset;k>0;k-=largeur)
                {
                OffsetRect(&r,-largeur,0);
                Affmob3d(mob3D+25,&r,TabPaletteDistance[numPalette]);
                }
        }

/* On trace toutes les ligne prŠs */
for(;i<NB_INTERV;i++)
        {
        hauteur=TabLigneSol[i].hauteur;
        largeur=TabLigneSol[i].largeur;
        if(hauteur>0 && largeur>0)
                {
                offset=TabLigneSol[i].offsetEcran;
                numMob=mob3D+20;

                if(i) SetRect(&r,256-offset,TabLigneSol[i-1].y+NB_LIGNES_CIEL,256+largeur-offset,TabLigneSol[i].y+NB_LIGNES_CIEL);
                else SetRect(&r,256-offset,NB_LIGNES_CIEL,256+largeur-offset,TabLigneSol[i].y+NB_LIGNES_CIEL);
                if(r.bottom>NB_LIGNES_CIEL) numPalette=(NB_LIGNES_CIEL+NB_LIGNES_TERRE)-r.bottom;
                else numPalette=NB_LIGNES_CIEL-NB_LIGNES_TERRE-r.bottom;
                if(numPalette>=NB_DEGRADE_DISTANCE) numPalette=NB_DEGRADE_DISTANCE-1;
                if(numPalette<0) numPalette=0;
                /* Trace la partie droite */
                rdep=r;
                for(j=256-offset;j<512;j+=largeur)
                        {
                        Affmob3d(numMob,&r,TabPaletteDistance[numPalette]);
                        OffsetRect(&r,largeur,0);
                        }
                r=rdep;
                for(j=256-offset;j>0;j-=largeur)
                        {
                        OffsetRect(&r,-largeur,0);
                        Affmob3d(numMob,&r,TabPaletteDistance[numPalette]);
                        }
                }
        }
CarboniseSetRandSeed(RandSeedBak);
return;
}
/******************************************************************
                    TrameCielOrage
******************************************************************/
static void TrameCielOrage(short index)
{
unsigned char * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
short i,j;


/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
if(data==NULL)     return;
i=NB_LIGNES_CIEL/2;
while(i--)
    {
    j=tailleLigne/2;
    while(j--)
        {
        *data=index;
        *(data+tailleLigne+1)=index;
        data+=2;
        }
    data+=tailleLigne;
    }
return;
}
/******************************************************************
                    Flou
******************************************************************/
static void Flou(void)
{
unsigned char * data;/* Donn‚es de l'ecran */
unsigned char * p;
short tailleLigne,depth;
short x,y;


/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
if(data==NULL)     return;
for(y=0;y<200;y++)
    {
    p=data+((long)tailleLigne*(long)y);
    for(x=0;x<tailleLigne;x+=3)
        {
        *p=*(p+1);
        p+=3;
        }
    }
return;
}
/******************************************************************
                    TracePluie
=> index de la couleur, nombre de gouttes
******************************************************************/
static void TracePluie(short index,short n)
{
unsigned char * data;/* Donn‚es de l'ecran */
unsigned char * p;
short tailleLigne,depth;
short j;
short x,y;


/* Origine des donn‚es de l'ecran */
if(myRandom(0,1000)>500) Flou();
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
if(data==NULL)     return;
while(n--)
    {
    x=myRandom(0,tailleLigne);
    y=myRandom(0,150);
    j=50;
    p=data+((long)tailleLigne*(long)y)+(long)x;
    while(j--)
        {
        *p=index;
        p+=tailleLigne-1;
        }
    }
return;
}
/****************************************************************************
                    eff3d
            Effacement de la 3d
*****************************************************************************/
void eff3d(unsigned char ciel, /* NU */
          unsigned char terre, /* NU */
          unsigned short ymeteore,
          unsigned short xmeteore,
          unsigned short isLune)
{
CGrafPtr gbak;
AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);

/* Le degrade du ciel */
/* Un orage est en cours */
RemplitZoneLignes(0,NB_LIGNES_CIEL,TabCouleursCiel);
AfficheEtoiles();
/* Le meteore */
Affmobbrt(MOB_SOLEIL+isLune,xmeteore,ymeteore,0,NULL);
if(FreqEclair<1000)
    {
    if(isLune)  TrameCielOrage(CouleurOrageNuit);
    else  TrameCielOrage(CouleurOrageJour);
    }
/* Le sol */
AfficheSol();
AcamSetPortOffscreen(gbak);
return;
}
/***************************************************************************************
                                 Pluie
***************************************************************************************/
void Pluie(void)
{
short n;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);

if(FreqEclair<1000 && FreqEclair>500)
    {
    n=FreqEclair>750?1000-FreqEclair:FreqEclair-500;
    if(heure & 0x200)  TracePluie(CouleurOrageNuit,n);
    else  TracePluie(CouleurOrageJour,n);
    /* Si pas deja de l'eau par terre on mets de l'eau a terre */
    if(liste4.elements[liste4.taille-1]!=2 && liste4.taille<liste4.maxtaille)
        {
        liste4.taille++;
        tlpos[4]=liste4.taille;
        liste4.elements[liste4.taille-1]=2;/*Eau */
        }
    }
AcamSetPortOffscreen(gbak);
return;
}
/***************************************************************************************
                                 PosePluie
Quand il pleut il ya des flaques par terre
***************************************************************************************/
void PosePluie(void)
{
if(FreqEclair<1000 && FreqEclair>500)
    {
    /* Si pas deja de l'eau par terre on mets de l'eau a terre */
    if(liste4.elements[liste4.taille-1]!=2 && liste4.taille<liste4.maxtaille)
        {
        liste4.taille++;
        tlpos[4]=liste4.taille;
        liste4.elements[liste4.taille-1]=2;/*Eau */
        }
    }
return;
}
/***************************************************************************************
                                                CalculeAltitude
                                Calcule l'altitude d'un point du plan
***************************************************************************************/
static short CalculeAltitude(short x,short y)
{
long i,d,r;
short z=40;
MontPtr p=*aigoual;

for (i=0;i<nbreobj;i++,p++)
        {
        if(!p->ptrforme->hauteur)
                {
                d=distance((long)xj-(long)p->posx,(long)yj-(long)p->posy);
                r=p->ptrforme->rayon;
                if(d<r)
                        {
                        z+=(long)(r-d)*(long)p->ptrforme->nbrdisk*(long)p->ign/(long)r;
                        }
                }
        }
return(z);
}
/*****************************************************************************************
                                                Visu :
Calcul et visualisation d'un objet 3d
L'affichage se fait sur ecran 1
******************************************************************************************/
void visu(MontPtr mont)
{
FormePtr forme;
short x,y,dist,hauteurDisque;
short rd;
CGrafPtr gbak;
Rect r;
float angle,sa,ca;
short numPalette;
long i=0;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
zj=CalculeAltitude(xj,yj);
angle=(float)dzeta*(float)3.1415927/512.0;
ca=cos(-angle);
sa=sin(-angle);

//printf("---------\n");
while(mont)
    {
    forme=mont->ptrforme;
	if(CalcPoint3D(xj,yj,zj,ca,sa,mont->posx,mont->posy,forme->hauteur,&x,&y,&dist))
		{
		rd=(long)forme->rayon*2L*256L/(long)dist;
		hauteurDisque=(long)mont->ign*(long)forme->nbrdisk*256L/(long)dist;
		SetRect(&r,x-rd/2,y-hauteurDisque,x+rd/2,y);
		if(!EmptyRect(&r) && r.left<LARGEUR_ECRAN && r.right>0 && r.bottom>0 && r.top<HAUTEUR_ECRAN)
			{
			if(r.bottom>NB_LIGNES_CIEL) numPalette=(NB_LIGNES_CIEL+NB_LIGNES_TERRE)-r.bottom;
			else numPalette=r.bottom-(NB_LIGNES_CIEL-NB_LIGNES_TERRE);
			if(numPalette>=NB_DEGRADE_DISTANCE) numPalette=NB_DEGRADE_DISTANCE-1;
			if(numPalette<0) numPalette=0;
			Affmob3d(mob3D+forme->numMob3d,&r,TabPaletteDistance[numPalette]);
			//if(forme=&nuage1)
			//	printf("%08lx %d %d %d\n",mont,forme->numMob3d,mont->posx,mont->posy);
			#ifdef NO_TEST
			char chaine[255];
			ForeColor(whiteColor);
			sprintf(chaine,"%d %ld %ld",forme->numMob3d,(long)mont->posx,(long)mont->posy);
			CtoPstr(chaine);
			DoMoveTo(r.left,r.top+12);
			DrawString((StringPtr)chaine);
			#endif
			}
		}
	mont=mont->chaine;
	i++;
    }
AcamSetPortOffscreen(gbak);
return;
}

/*;*********************************
;               ChaineObjet
; Chaine une montagne en entree
;*********************************
;=> pointeur sur montagne a traiter
;: la montagne est chainee
;<= 1 si chainage 0 sinon
*/
short ChaineObjet(MontPtr mont)
{
register short dist;
register MontPtr poschaine=NULL;
register MontPtr balaye;
Forme * forme;

/* On calcule la distance de la montagne au joueur*/
if(mont->ign==0) return(0);
forme=mont->ptrforme;
mont->distance=dist=distance((long)mont->posx-(long)xj,(long)mont->posy-(long)yj);
if((long)dist>256L*(long)forme->nbrdisk*(long)mont->ign/4L) return(0);
if((long)dist>256L*(long)forme->rayon/3) return(0);
switch((dzeta>>8)&3)
        {
        case 0:if(mont->posx<xj && mont->posy<yj) return(0);
                        break;
        case 1:if(mont->posx>xj && mont->posy<yj) return(0);
                        break;
        case 2:if(mont->posx>xj && mont->posy>yj) return(0);
                        break;
        case 3:if(mont->posx<xj && mont->posy>yj) return(0);
                        break;

        }
/* Premier element de la chaine */
balaye=(*aigoual)->chaine;
while(balaye!=NULL && balaye->distance>dist)
        {
        poschaine=balaye;
        balaye=balaye->chaine;
        }
if(poschaine==NULL)
        {
        mont->chaine=(*aigoual)[0].chaine;
        (*aigoual)[0].chaine=mont;
        }
else
        {
        mont->chaine=poschaine->chaine;
        poschaine->chaine=mont;
        }
return(1);
}
/*EOF*/
