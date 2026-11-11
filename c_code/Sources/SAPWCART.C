/*************************************************************************
                    SapWCart.c
                (c) Dg le 19-1-1996
Gestion de la carte
***************************************************************************/
#include "sapiens.h"
extern Forme * fumee;
static CGrafPtr EcrCarte;
#define TAILLE_X_CARTE 144
#define TAILLE_Y_CARTE 80
#define NB_OBJETS_MAX 200L
typedef struct
    {
    Rect aire;
    short mob;
    } EltCart,*EltCartPtr;
static EltCartPtr TabEltCar;
static Rect AireCarte;
/*;-_-_-_-_-_-_-_-_  Routine 2: transfere carte  sur ecran -_-_-_-_-_-_-_-
*/
static void tfrcar(void)
{
Rect rdest;

SetRect(&rdest,160*ECHELLE,(117)*ECHELLE,(160+TAILLE_X_CARTE)*ECHELLE,(117+TAILLE_Y_CARTE)*ECHELLE);
CopyOffscreen(EcrCarte,debecr,&AireCarte,&rdest);
CopyOffscreen(EcrCarte,CarboniseGetWindowPort(mainWindow),&AireCarte,&rdest);
return;
}
/*
;-_-_-_-_-_-_-_-_  Routine 1: calcule positions sur ecran -_-_-_-_-_-_-_-
; => Pointeur sur la structure montagne ,Zoom_
; <= Xc,Yc (Yc … 0 si montagne invisible)
*/
static Boolean calc_cart(MontPtr mont,EltCartPtr eltCart)
{
FormePtr forme;
long x,y,r;
Rect rb;

forme=mont->ptrforme;
/* Position en ligne */
x=((long)yj-(long)mont->posy)/(long)zoom;
x=x+TAILLE_X_CARTE;
y=((long)xj-(long)mont->posx)/(long)zoom;
y=y+TAILLE_Y_CARTE;
r=forme->rayon/((long)zoom/2L);
if(forme->num_mob==BASEMOB+8) /* Fumee */
    SetRect(&eltCart->aire,x-8,y-32,x+8,y+32);
else
    if(r<16)  SetRect(&eltCart->aire,x-8,y-8,x+8,y+8);
else
    SetRect(&eltCart->aire,x-r/2,y-r/8L,x+r/2,y+r/8L);
if(SectRect(&eltCart->aire,&AireCarte,&rb))
    {
    eltCart->mob=mob3D+mont->ptrforme->numMob3d;
    return(VRAI);
    }
return(FAUX);
}
/*************************************************************************
                Affichage des montagnes sur la carte
=> Nombre d'objets a afficher
***************************************************************************/
static void aff_mont(short nb)
{
short j,jmin;
short minyc;

for(;;)
    {
    minyc=32767;
    for(j=0;j<nb;j++)
        {
        if(TabEltCar[j].aire.bottom<minyc)
            {
            jmin=j;
            minyc=TabEltCar[j].aire.bottom;
            }
        }
    if(minyc==32767) return;
    Affmob3d(TabEltCar[jmin].mob,&TabEltCar[jmin].aire,NULL);
//  ForeColor(whiteColor);
//  DoFrameRect(&TabEltCar[jmin].aire);
    TabEltCar[jmin].aire.bottom=32767;
    }
}
/***********************************************************************
                         EffCarte
Efface le fond de la carte
***********************************************************************/
static void EffCarte(void)
{
Rect r;

SetRect(&r,0*ECHELLE,0*ECHELLE,TAILLE_X_CARTE*ECHELLE,TAILLE_Y_CARTE*ECHELLE);
BoxFill(EcrCarte,&r,0x050);/* palhaute[3]*/
return;
}
/***********************************************************************
                         CARTE:
***********************************************************************/
void carte(void)
{
long i,j;
short angle;
MontPtr mont;
CGrafPtr gbak;
Rect r;

/* deconnecte sous menu */
if(EcrCarte==NULL)
    {
    EcrCarte=SapiensCreeOffscreen(TAILLE_X_CARTE*ECHELLE,TAILLE_Y_CARTE*ECHELLE);
    if(EcrCarte==NULL) return;
    TabEltCar=(EltCartPtr)NewPtr(NB_OBJETS_MAX*(long)sizeof(EltCart));
    if(TabEltCar==NULL) return;
    SetRect(&AireCarte,0,0,TAILLE_X_CARTE*ECHELLE,TAILLE_Y_CARTE*ECHELLE);
    }
AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(EcrCarte);
smenu[1]=111;
/* efface carte */
EffCarte();
/* trie les objets */
HLock((Handle)aigoual);
mont=*aigoual;
i=nbreobj;
j=0;
while(i-- && j<NB_OBJETS_MAX)
    {
    if(mont->ptrforme->num_mob>=0)
        {
        if(calc_cart(mont,&TabEltCar[j])) j++;
        }
    mont++;
    }
HUnlock((Handle)aigoual);
/* affichage */
aff_mont(j);
/* on affiche la fleche */
angle=((dzeta+32)/64)&15;
Affmobbrtomb(DEBFLECH+angle,TAILLE_X_CARTE/2-8,TAILLE_Y_CARTE/2-8,5,5,&r);
/* et on transfere */
tfrcar();
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************
                       ChangeZoomCarte
***********************************************************************/
void ChangeZoomCarte(short delta)
{
zoom+=delta;
if(zoom>160) zoom=160;
if(zoom<16) zoom=16;
carte();
return;
}
/***********************************************************************
                        ZOOCAR:Gere le zoom
***********************************************************************/
void zoocar(void)
{
short bitmap;
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
CarboniseSetPortWindow(mainWindow);

do vq_mouse(&bouton,&sposx,&sposy);
        while(bouton);
SetRect(&r,160*ECHELLE,(117)*ECHELLE,(160+TAILLE_X_CARTE)*ECHELLE,(117+TAILLE_Y_CARTE)*ECHELLE);
InsetRect(&r,-1,-1);
ForeColor(whiteColor);
DoFrameRect(&r);
ForeColor(blackColor);
carte();
do
        {
        GereInterface();
    	if(GetKeyMap(&bitmap))
    		{
    		if(bitmap&1) ChangeZoomCarte(-2);
    		if(bitmap&2) ChangeZoomCarte(2);
    		if(bitmap&64) bouton=1;
    		}
    	else
    		{
    		vq_mouse(&bouton,&sposx,&sposy);
    		if(sposy!=dsposy)
    			{
    			zoom=sposy/2+30;
    			if(zoom<16) zoom=16;
  				if(zoom>160) zoom=160;
    			dsposy=sposy;
    			carte();
    			}
    		}
        } while(!bouton);
DoFrameRect(&r);
AcamSetPortOffscreen(gbak);
return;
}
/*EOF */
