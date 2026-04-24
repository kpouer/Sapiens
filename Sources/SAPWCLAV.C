/*************************************************************************
                    Sapclav.c
                (c) Dg le 12-1-1996
Gestion du clavier et souris
***************************************************************************/
#include "sapiens.h"
void AffichePad(void);

volatile unsigned long sapiensTimer; // Timer 11 millisec
typedef struct
    {
    TMTask task;
    long MonA5;  /* Adresse du pointeur sur les globales */
    } MyTmTask;
static MyTmTask timerRec;
Boolean isScrollMouse;

#ifdef MAC_68K_OBSOLETE
       void PushReg(void)  = { 0x48E7,0xFFFC };
       void PopReg(void)   = { 0x4CDF,0x3FFF };
/**************************************************************************
                    TimerCallbackProc
*************************************************************************/
#pragma a6frames on
static pascal	void	TimerCallbackProc(MyTmTask * tache:__A1)
{
PushReg();
SetA5(tache->MonA5);
sapiensTimer++;
PrimeTime((QElemPtr)&tache->task, 11);	// reset the timer
PopReg();
return;
}
#pragma a6frames off
#else
/**************************************************************************
                    TimerCallbackProc
*************************************************************************/
static pascal	void	TimerCallbackProc(TMTaskPtr	tmTaskPtr)
{
sapiensTimer++;
PrimeTime((QElemPtr)tmTaskPtr, 11);	// reset the timer
}
#endif
/**************************************************************************
                    InstalleTimeTask
   Installe le timer de comptage 11ms pour le ralentissement                 
*************************************************************************/
void InstalleTimeTask( void )
{

timerRec.task.qLink = NULL;
timerRec.task.qType = 0;
timerRec.task.tmAddr = NewTimerProc(TimerCallbackProc);
timerRec.task.tmCount = 0;
timerRec.task.tmWakeUp = 0;
timerRec.task.tmReserved = 0;
#ifdef MAC_68K
timerRec.MonA5=SetCurrentA5();
#endif
InsXTime((QElemPtr) &timerRec.task);
PrimeTime((QElemPtr)&timerRec.task, 11); // 11 milliseconds = 90 fps
return;
}

/**************************************************************************
                    litclav
*************************************************************************/
short litclav(void)
{
short n;

GereInterface();
n=toucheClav;
toucheClav=0;
return(n);
}
/**************************************************************************
                  resetmouse
Fixe le zero de la souris
*************************************************************************/
void reset_mouse(void)
{
short bouton;

vq_mouse(&bouton,&dsposx,&dsposy);
}
/***************************************************************************
                        reset_clav
****************************************************************************/
void reset_clav(void)
{
toucheClav=0;
return;
}
/*************************************************************************
                    CacheSouris
**************************************************************************/
static short hidden;
void CacheSouris(void)
{
if(!hidden)
        {
        HideCursor();
        hidden=VRAI;
        }
}
/*************************************************************************
                    MontreSouris
**************************************************************************/
void MontreSouris(void)
{
if(hidden)
        {
        ShowCursor();
        CarboniseSetCursorArrow();
        hidden=FAUX;
        }
}
/*************************************************************************
                    vq_mouse
**************************************************************************/
static Rect rBasse={116*ECHELLE,0,200*ECHELLE,320*ECHELLE};
static Rect rNoMouse;
void vq_mouse(short * bouton,short * x,short * y)
{
Point pt;
KeyMap theKeys;

CarboniseSetPortWindow(mainWindow);
GereInterface();
GetMouse(&pt);
pt.h/=SapZoom;
pt.v/=SapZoom;
if(PtInRect(pt,&rNoMouse)) CacheSouris();
else MontreSouris();
*x=pt.h/ECHELLE;
*y=pt.v/ECHELLE;
*bouton=Button();
if(*bouton)
    {
    GetKeys(theKeys);
    if(KeyPressed(&theKeys,0x38)) *bouton|=2;
    }
return;
}
/*************************************************************************
                    FixeAireNoMouse
Fixe l'aire dans laquelle il ne faut plus afficher la souris systeme
**************************************************************************/
void FixeAireNoMouse(Rect * r)
{
rNoMouse=*r;
}
/*************************************************************************
                    FixeAireNoMouseAireBasse
**************************************************************************/
void FixeAireNoMouseAireBasse(void)
{
FixeAireNoMouse(&rBasse);
}
/*************************************************************************
                    wait_mouse
**************************************************************************/
void wait_mouse(short * bouton,short * x,short * y)
{
short  orgx=*x;
short orgy=*y;
short orgbouton=*bouton;
do
    {
    vq_mouse(bouton,x,y);
    } while(*x==orgx && *y==orgy && *bouton==orgbouton);
return;
}
/*************************************************************************
            Eff_sour
Efface la souris graphique menus
Pour effacer le souris on recopie debecr sur l'aire de la souris
**************************************************************************/
static Rect rsouris;
void eff_sour(void)
{
CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&rsouris,&rsouris);
SetRect(&rsouris,0,0,0,0);
return;
}
/**************************************************************************
;--     Aff_sour_: Affiche la souris graphique menus
; => N. mob,Ligne,Colonne
**************************************************************************/
void aff_sour(short mob,short ligne,short colonne)
{
CGrafPtr gbak;
Rect rsource;
Rect r;
Rect oldRect;
RgnHandle rgn1,rgn2,rgnClip;
Rect r1,r2;

/* Effacement a l'ancienne position */
oldRect=rsouris;
/* Aire de la souris sur l'ecran */
CalculeAireMob(mob,colonne,ligne,&rsouris);
CalculeAireMob(mob,0,0,&rsource);
/* On gere l'obmre */
rsouris.right+=8;
rsouris.bottom+=8;
rsource.right+=8;
rsource.bottom+=8;
/* Le mob s'affiche sur l'ecran virtuel : on saisie ce qu'il y a
sur l'ecran */
CopyOffscreen(debecr,BuffMouse,&rsouris,&rsource);
/* On copie egalement l'ecran physique sur le buffer de la souris */
/* On affiche la souris */
AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(BuffMouse);
Affmobbrtomb(mob,0,0,4,4,&r);
AcamSetPortOffscreen(gbak);
/* On le recopie sur le reel */
CopyOffscreen(BuffMouse,CarboniseGetWindowPort(mainWindow),&rsource,&rsouris);
/* On efface la difference */
rgn1=NewRgn();
rgn2=NewRgn();
rgnClip=NewRgn();
DoZoomRect(&oldRect,&r1);
DoZoomRect(&rsouris,&r2);
RectRgn(rgn1,&r1);
RectRgn(rgn2,&r2);
DiffRgn(rgn1,rgn2,rgn1);
GetClip(rgnClip);
SetClip(rgn1);
CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&oldRect,&oldRect);
SetClip(rgnClip);
DisposeRgn(rgnClip);
DisposeRgn(rgn1);
DisposeRgn(rgn2);
return;
}
/*********************************************************************
                        KeyPressed
  Dit si une touche d'ascii donn‚es est appuy‚e sur le keymap en entree
********************************************************************/
Boolean KeyPressed(KeyMap * km,short ascii)
{
unsigned long * p=(unsigned long *)km;
#if defined(MAC) && !defined(MAC_ACAM)
unsigned long val;
val=p[ascii>>5];
val=lintel2moto(val);
return((val>>((ascii^24)&31))&1);
#else
return((p[ascii>>5]>>((ascii^24)&31))&1);
#endif
}
/*************************************************************
                    GetKeyMap
***************************************************************/
short GetKeyMap(short * bitmap)
{
KeyMap theKeys;

*bitmap=0;
GetKeys(theKeys);
if(KeyPressed(&theKeys,0x7E)) *bitmap|=1;
if(KeyPressed(&theKeys,0x7D)) *bitmap|=2;
if(KeyPressed(&theKeys,0x7B)) *bitmap|=4;
if(KeyPressed(&theKeys,0x7C)) *bitmap|=8;
if(KeyPressed(&theKeys,0x31)) *bitmap|=32;/* Espace */
if(KeyPressed(&theKeys,0x24)) *bitmap|=16;/* RC */
if(KeyPressed(&theKeys,0x35)) *bitmap|=64;/* ESC*/
if(KeyPressed(&theKeys,0x08)) *bitmap|=32;/* Carte*/
if(KeyPressed(&theKeys,0x2e)) *bitmap|=32;/* Carte*/

return(*bitmap);
}
/*********************************************************************
                        IsKey
  Dit si une touche d'ascii donn‚es est appuy‚e sur le keymap en entree
********************************************************************/
Boolean IsKey(short ascii)
{
KeyMap theKeys;

GetKeys(theKeys);
return(KeyPressed(&theKeys,ascii));
}
/*************************************************************
                    GetKeyMapJoueur2
***************************************************************/
static short GetKeyMapJoueur2(short * bitmap)
{
KeyMap theKeys;

*bitmap=0;
GetKeys(theKeys);
if(KeyPressed(&theKeys,0x5b)) *bitmap|=1;
if(KeyPressed(&theKeys,0x54)) *bitmap|=2;
if(KeyPressed(&theKeys,0x56)) *bitmap|=4;
if(KeyPressed(&theKeys,0x58)) *bitmap|=8;
if(KeyPressed(&theKeys,0x53)) *bitmap|=32;/* Espace */
if(KeyPressed(&theKeys,0x55)) *bitmap|=16;/* RC */
if(KeyPressed(&theKeys,0x42)) *bitmap|=64;/* ESC*/
return(*bitmap);
}
/*************************************************************
                    GETKEY3D
Donne un deplacement / a la position 160,100 de la souris
***************************************************************/
short getkey3d(void)
{
short bitmap=0,delt;

if(!GetKeyMap(&bitmap))
    {
    vq_mouse(&bouton,&sposx,&sposy);
    delt=sposx-dsposx;
    if (delt>0 && delt>16)  bitmap |=8;
    if (delt<0 && -delt>16) bitmap |=4;
    delt=sposy-dsposy;
    if (delt>0 && delt>8) bitmap |=2;
    if (delt<0 && -delt>8) bitmap |=1;
    if (bouton & 1) bitmap |=64;
    if (bouton & 2) bitmap |=64;
    }
else
    reset_mouse();
return(bitmap);
}

/*************************************************************
                    GETKEY
**************************************************************
=> 0 joeur principal, 1 secondaire
*/
short getkey(unsigned short quoi)
{
short bitmap=0,delt;

if (quoi)
     {
     GetKeyMapJoueur2(&bitmap);
     }
else
     {
     /* joueur primaire */
     if(!GetKeyMap(&bitmap))
        {
         vq_mouse(&bouton,&sposx,&sposy);
         delt=sposx-dsposx;
         if (delt>0 && delt>10) bitmap |=8;
         if (delt<0 && -delt>10) bitmap |=4;
         delt=sposy-dsposy;
         if (delt>0 && delt>5) bitmap |=2;
         if (delt<0 && -delt>5) bitmap |=1;
         if (bouton & 1) bitmap |=32;
         if (bouton & 2) bitmap |=16;
         }
    else
        reset_mouse();
     dsposx=sposx;
     dsposy=sposy;
     }
return(bitmap);
}
/*************************************************************
                    BOUGE_SOURIS
=> globales :sposx,sposy,bouton,icone
**************************************************************/
static short lastposx,lastposy;
void bouge_souris(void)
{
GereInterface();
vq_mouse(&bouton,&sposx,&sposy);
/* Attends et prends coord souris */
/* sortie haute ??? */
//if (sposy<105) sposy=105;

/* sortie basse ??? */
if (sposy>184)
    {
    sposy=184;
    }
/* sortie droite */
if (sposx>303)
    {
    sposx=303;
    }
/* si boug‚ alors retrace */
if(((sposx !=lastposx) || (sposy != lastposy) || (icone != dicone)) && icone)
      {
      dicone=icone;
      lastposx=sposx;
      lastposy=sposy;
      aff_sour(icone,sposy,sposx);
      }
return;
}
/**************************************************************************
       AttendsFinBouton
**************************************************************************/
void AttendsFinBouton(void)
{
while(bouton) vq_mouse(&bouton,&sposx,&sposy);
}
/**************************************************************************
        Deuxieme joueur au joystick
**************************************************************************/
short joystick(void)
{
return(0);
}
/*EOF*/
