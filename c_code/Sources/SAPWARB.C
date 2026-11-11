/*************************************************************************
                    SapWarb.c
                (c) Dg le 19-1-1996
Affichage des arbres
***************************************************************************/
#include "sapiens.h"
#define PREMFEUIL bambou_+50*130+2
static short niveau; /*niveau de recusrsivite*/
static short largtronc; /* Largeur du tronc*/
static short numfeu; /*type de la feuille*/
static short amini;
static short qutlgn;
static short densif;
static short fortou[4];
static short tabrnd[]={20,30,8,1,6,12,20,6,10,4}; /* Rnd , base */
static short tram_arbre[]={1,3,4,5,6,5,4,3,1};
short tabcoul[]={3,4,5,6,6,5,4,3,3};
static short * TabIndexFeuilleNuit;

/**************************************************************************
                        calpar
***************************************************************************/
static void calpar(short largeur,short * ang,short *lr,short * lo)
{
short n;

n=largtronc-((rnd0()*qutlgn*niveau)>>8);
if(n<0)
    {
    n=((largtronc*rnd0())>>8);
    }
n++;
*lo=n;
largeur=((rnd0()*(largeur>>1))>>8)+(largeur>>2)+1;
if(rnd0()<=40)
    largeur++;
*ang=((rnd0()*70)>>8)+amini;
*lr=largeur;
return;
}
/**************************************************************************
                            touffe
    Touffe: trace une touffe de feuilles sur tampon ecran   ----
;-> b_ancx,b_ancy: colonne,ligne
;   Densif,Fortou,Numfeu
; <- oh que c'est joli !
***************************************************************************/
static void touffe(short colonne,short ligne)
{
short angle=0; /* D5*/
long y,x;
short r,forme,i;


forme=0;
for(i=0;i<densif;i++)
    {
    r=rnd0();
    y=((long)fortou[1]*(long)sinshort(angle&0xFF)*(long)r)>>16L;
    x=((long)fortou[0]*(long)cosshort(angle&0xFF)*(long)r)>>16L;
    if(angle&0x100) x+=colonne;
    else   x=colonne-x;
    if(angle&0x200) y+=ligne;
    else   y=ligne-y;
    x+=16;
    if(x<256)
         {
         Affmobbrt(MOB_FEUILLE+numfeu+forme,x,y,rnd0()&1,TabIndexFeuilleNuit);
         }
    if(++forme>3) forme=0;
    angle+=(rnd0()+256);
    angle&=1023;
    }
}
/**************************************************************************
                            tracebranche
***************************************************************************/
static void tracebranche(short angle,/*D1*/
                short colonne,       /*D2*/
                short ligne,         /*D3*/
                short longueur       /*D4*/,
                short largeur        /*D5*/)
{
short epaisseur;
short posg,posd;
short AncXG,AncXD,Ancy;
short ang,lo,lr,col,bidon;

if(++niveau<15)
    {
    /*;---- Mise en place des parametres pour TraceDisque*/
    AncXG=colonne;
    AncXD=colonne+largeur;
    Ancy=ligne;
    /*---- Calcul de l'epaisseur */
    epaisseur=(abs(sinshort(angle))*longueur)>>8;
    if(epaisseur<1) epaisseur=1;
    /* Calcul de la pos haut gauche */
    posg=AncXG+((cosshort(angle)*longueur)>>8);
    if(posg<0) posg=0;
    posd=posg+largeur;
    if(posd>255)
        {
        posg=255-largeur;
        posd=255;
        }
    /* Trb15*/
    if(epaisseur+Ancy<0) epaisseur=Ancy;
    TraceDisque(&AncXG,&AncXD,&Ancy,posg,posd,epaisseur);
    if(largeur<2)
        {
        touffe((AncXG+AncXD)/2,Ancy);
        }
    else
        {
        if(Ancy>0)
            {
           /* Branche gauche */
            calpar(largeur,&ang,&lr,&lo); /*Largeur longueur deltangle */
            ang=angle+ang;
            if(ang>450) ang=900-ang;
            tracebranche(ang,AncXG,Ancy,lo,lr);
            col=AncXG+lr;
            lr=largeur-lr;
            if(lr<1) lr=1;
            calpar(largeur,&ang,&bidon,&lo); /*Largeur longueur deltangle */
            ang=angle-ang;
            if(ang<10) ang=124-ang;
            tracebranche(ang,col,Ancy,lo,lr);
            }
        }
    }
niveau--;
return;
}
/**************************************************************************
                            arbre
***************************************************************************/
void arbre(short ligne,short colonne)
{
short i;
unsigned short longueur,largeur;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
/* La position du meteore varie de 74 … 0 puis de 0 … 74 */
if(heure>512)
    TabIndexFeuilleNuit=CalculePaletteAttenuee(MOB_FEUILLE,0.50+(float)(metpos)/150.0,999);
else
	TabIndexFeuilleNuit=NULL;
niveau=0;
for(i=0;i<9;i++)
    tabcoul[i]=tram_arbre[i];
/* On tire au hasard le numero de la feuille */
numfeu=((rnd0()*9)>>8)*4;
/* On tire les autres parametres au hasard (5 parametres)*/
amini=((rnd0()*tabrnd[0])>>8)+tabrnd[1];
qutlgn=((rnd0()*tabrnd[2])>>8)+tabrnd[3];
densif=((rnd0()*tabrnd[4])>>8)+tabrnd[5];
fortou[0]=((rnd0()*tabrnd[6])>>8)+tabrnd[7];
fortou[1]=((rnd0()*tabrnd[8])>>8)+tabrnd[9];
longueur=((rnd0()*6)>>8)+10;
largeur=((rnd0()*10)>>8)+10;
largtronc=largeur;
tracebranche(256,colonne,ligne,longueur,largeur);
if(TabIndexFeuilleNuit) DisposPtr((Ptr)TabIndexFeuilleNuit);
TabIndexFeuilleNuit=NULL;
//GereInterface();
AcamSetPortOffscreen(gbak);
return;
}
/*EOF */
