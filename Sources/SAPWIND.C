/*************************************************************************
                    SapWind.c
                (c) Dg le 12-1-1996
***************************************************************************/
#include "sapiens.h"
#include <math.h>
#include <stdarg.h>

void InitProfile(short,short);
short GetStdFont(void);
void DumpProfile(void);
float SapZoom=2.0;
WindowPtr mainWindow;
CGrafPtr Ecran1,Ecran2,debecr,BuffMouse;
short toucheClav;
short mobBambou,mobFace,mobFeuille,mobFleches,mobSapmob,mobAvplan,mobAvplanSpec,mobSilex,mob3D;
short Fonte14,Fonte10;
#define menuPomme CvtDefaultLangRess(128)
#define menuFichier CvtDefaultLangRess(129)
#define menuVoir CvtDefaultLangRess(130)
#define menuFenetre CvtDefaultLangRess(131)

enum
    {
    itemInformation=1,
    itemAide
    };
enum
    {
    itemNouveau=1,
    itemCharger,
    itemSauver,
    iVide0,
    itemPreferences,
    iVide1,
    itemQuitter
    };
enum
    {
    iPaysage=1,
    iLaterale,
    iPanorama,
    iCarte
    };
enum
    {
    iSon=1
    };
enum
    {
    iEchelle100=1,
    iEchelle200,
    iEchelle300,
    iEchelle400
	
    };
static Boolean MenuReduit;
/******************************************************************************************
                          GetVersionSapiens
Affiche la version du logiciel dans l'user item en entr‚e. Le texte est centr‚
Structure des 'vers'
2 octets de version en hexa
3 octets de release
1 octet de pays
taille de la chaine courte suivi de la chaine courte
taille de la chaine longue suivi de la chaine longue
******************************************************************************************/
#define VERS    0x76657273L
Boolean GetVersionSapiens(char * chaine)
{
Handle ress;
Ptr p;

chaine[0]=0;
ress=GetResource(VERS,1);
if(ress==NULL)return(FAUX);
HLock(ress);
p=*ress;
/* On recherche le pointeur sur la chaine longue de la version */
p+=6;           /* On saute jusqu'a la short version string */
BlockMove(p+1,chaine,*p);
chaine[*p]=0;
#ifdef DEBUG
    strcat(chaine,",DEBUG");
#endif
HUnlock(ress);
ReleaseResource(ress);
return(VRAI);
}
/***********************************************************************************
                 Collapse
Affichage d'un message d'erreur format‚
    RMQ: Cette fonction supporte un affichage format‚ identique au printf du C.
=> Message … afficher sous la forme "format", arguments
<= rien
***********************************************************************************/
void Collapse(char * format, ... )
{
char * chaine;
va_list  argptr;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
if(mainWindow) CarboniseSetPortWindow(mainWindow);

/* On fixe la zone … la zone de secours de l'interface*/
chaine=NewPtr(TAILLE_STR_MESS_FORMATE);
if(chaine)
    {
    va_start( argptr, format );
    vsprintf( chaine, format, argptr );
    CarboniseSetCursorArrow();
    CtoPstr(chaine);
    ParamText((StringPtr)chaine,NULL,NULL,NULL);
    DoAlertSapiens(ALRT_COLLAPSE);
    DisposePtr(chaine);
    }
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************************
                 Confirme
Affichage d'un message d'erreur format‚
    RMQ: Cette fonction supporte un affichage format‚ identique au printf du C.
=> Message … afficher sous la forme "format", arguments
<= VRAI si confirmation
***********************************************************************************/
Boolean Confirme(char * format, ... )
{
char * chaine;
va_list  argptr;
short etat;

/* On fixe la zone … la zone de secours de l'interface*/
chaine=NewPtr(TAILLE_STR_MESS_FORMATE);
if(chaine)
    {
    va_start( argptr, format );
    vsprintf( chaine, format, argptr );
    CarboniseSetCursorArrow();
    CtoPstr(chaine);
    ParamText((StringPtr)chaine,NULL,NULL,NULL);
    etat=DoAlertSapiens(ALRT_CONFIRMATION);
    DisposPtr(chaine);
    }
return(etat==1);
}
/***********************************************************************************
                        MessInit
Affichage d'un message durant l'initialisation
***********************************************************************************/
void MessInit(char * format, ... )
{
return;
#ifdef OBSOLETE
char * chaine;
va_list  argptr;
Rect r;
CGrafPtr gbak;

/* On fixe la zone … la zone de secours de l'interface*/
chaine=NewPtr(TAILLE_STR_MESS_FORMATE);
if(chaine)
    {
    AcamGetPortOffscreen(&gbak);
    CarboniseSetPortWindow(mainWindow);
    va_start( argptr, format );
    vsprintf( chaine, format, argptr );
    TextFont(helvetica);
    DoTextSize(14);
    TextFace(bold);
    SetRect(&r,16,60+120,640-16,60+120+18);
    DoTextBox(chaine,strlen(chaine),&r,teJustCenter);
    DoFrameRect(&r);
    DisposPtr(chaine);
    AcamSetPortOffscreen(gbak);
	CarboniseValideUpdate();
    }
return;
#endif
}
/**********************************************************************************
                                SetValItem
=> Fixe une valeur type chainedans un item
<= VRAI si la valeur est bonne, FAUX si la valeur a ‚t‚ tronqu‚e
**********************************************************************************/
 short SetValItem(DialogPtr dialog,short itemHit,char * c)
{
short itemType;
Handle control;
Rect box;
char chaine[255];

strcpy(chaine,c);
/* C'est un texte ??? */
GetDItem(dialog,itemHit,&itemType,&control,&box);
 #if  TARGET_API_MAC_CARBON>0
if(CarboniseIsApparenceManager())
			GetDialogItemAsControl(dialog,itemHit,(ControlRef*)&control);
#endif
   /* On fixe la chaine */
CtoPstr(chaine);
if(itemType & (editText | statText))   SetIText(control,(StringPtr)chaine);
if(itemType & ctrlItem)  SetCTitle((ControlHandle)control,(StringPtr)chaine);
return(VRAI);
}

/***********************************************************************************
                          SetShortItem
 Fixe une valeur de type short dans un item en respectant le formatage
**********************************************************************************/
short SetShortItem(DialogPtr dialog,short itemHit,short val)
{
char chaine[80];

sprintf(chaine,"%d",val);
return(SetValItem(dialog,itemHit,chaine));
}
/***********************************************************************************
                                SetValBouton
=> Fixe une valeur VRAI ou FAUX dans un bouton
**********************************************************************************/
short SetValBouton(DialogPtr dialog,short itemHit,Boolean value)
{
short itemType;
Handle control;
Rect box;

/* C'est un texte ??? */
GetDItem(dialog,itemHit,&itemType,&control,&box);
if(itemType & (ctrlItem+radCtrl))
    {
    GetDItem(dialog,itemHit,&itemType,&control,&box);
    SetCtlValue((ControlHandle)control,value);
    return(VRAI);
    }
return(FAUX);
}
/***********************************************************************************
                                GetValBouton
=>lit une valeur VRAI ou FAUX dans un bouton
**********************************************************************************/
short GetValBouton(DialogPtr dialog,short itemHit)
{
short itemType;
Handle control;
Rect box;
short value;

/* C'est un bouton ??? */
GetDItem(dialog,itemHit,&itemType,&control,&box);
if(itemType & (ctrlItem+radCtrl))
    {
    GetDItem(dialog,itemHit,&itemType,&control,&box);
    value=GetCtlValue((ControlHandle)control);
    return(value);
    }
return(FAUX);
}
/***********************************************************************************
                                SetActiveItem
=> Fixe un Item … actif ou inactif si c'est un control
**********************************************************************************/
void SetActiveItem(DialogPtr dialog,short itemHit,Boolean value)
{
short itemType;
Handle control;
Rect box;

GetDItem(dialog,itemHit,&itemType,&control,&box);
if(value) itemType &=0xFFFF-itemDisable;
else     itemType |=itemDisable;
if( (itemType &  editText) && value==FAUX)
    {
    itemType=statText;
    }
if((itemType &  statText) && value==VRAI )
    {
    itemType=editText;
    }
SetDItem(dialog,itemHit,itemType,control,&box);

if(itemType &  ctrlItem )
    {
    HiliteControl((ControlHandle)control,value?0:255);
    }

return;
}
/***********************************************************************************
                                GetValItem
Lit  une valeur type chaine dans un item texte editable
**********************************************************************************/
short GetValItem(DialogPtr dialog,short itemHit,char * chaine)
{
short itemType;
Handle handle;
Rect box;

/* C'est un texte ??? */
GetDItem(dialog,itemHit,&itemType,&handle,&box);
if(itemType & (editText | statText))
    {
    GetIText(handle,(StringPtr)chaine);
    PtoCstr((StringPtr)chaine);
    return(VRAI);
    }
return(FAUX);
}
/***********************************************************************************
                                GetShortItem
 Lit une valeur de type short dans un item en respectant le formatage
**********************************************************************************/
short GetShortItem(DialogPtr dialog,short itemHit,short * val)
{
char chaine[255];

if(GetValItem(dialog,itemHit,chaine))
    sscanf(chaine," %hd",val);
return(VRAI);
}
/******************************************************************************************
                                    DefAfficheItem
 D‚fini le vecteur d'affichage d'un item de Type User
=> Dialog, Nø de l'item, vecteur
******************************************************************************************/
void DefAfficheItem(DialogPtr dialog,short item,void pascal (*vect_affiche)(DialogPtr window,short itemid))
{
Handle handle;
Rect rect;
short itemType;

GetDialogItem(dialog,item,&itemType,&handle,&rect);
if((itemType&127) == userItem)
    {
    /* Deja un vecteur pour cet item ? */
    if(handle) DisposeRoutineDescriptor((UniversalProcPtr)handle);
    SetDialogItem(dialog,item,itemType,(Handle)CarboniseNewUserItemProc(vect_affiche),&rect);
    }
return;
}
/***********************************************************************************
                        GetNbItemInDialog
Supppl‚e … une lacune du MACOS: Va chercher en fonction de l'id d'un dialogue,
le nombre d'item dans le DITL de ceui-ci
=> ID du dialog
<= Nombre d'item ou -1 si erreur
**********************************************************************************/
short GetNbItemInDialog(short id_dialogue)
{
Handle handle;
short id_ditl;

handle=GetResource(DLOG,id_dialogue);
if(handle)
    {
    id_ditl=*(short far *)((*handle)+18);
    #ifdef PC
        id_ditl=wintel2moto(id_ditl);
    #endif
    /* ON libŠre la ressource du DLOG */
    ReleaseResource(handle);
    handle=GetResource(DITL,id_ditl);
    if(handle)
        {
        id_ditl=(*(short  *)(*handle));
        #ifdef PC
        id_ditl=wintel2moto(id_ditl);
        #endif
        /* ON libŠre la ressource du DITL */
        ReleaseResource(handle);
        return(id_ditl+1);
        }
    }
Collapse("GetNbItemInDialog:Ressource non trouvee.");
return(-1);
}
/******************************************************************************************
                                    DisposeUserItems
Sur power PC … chaque user item … ‚t‚ allou‚ un UniversalProcPtr, on doit les liberer
en fermture de la boite
=> dialog,id du ditl
******************************************************************************************/
void DisposeUserItems(DialogPtr dialog,short idDITL)
{
#ifdef powerc
short i,nbItems;
Handle handle;
Rect rect;
short itemType;

nbItems=GetNbItemInDialog(idDITL);
for(i=1;i<=nbItems;i++)
        {
         GetDItem(dialog,i,&itemType,&handle,&rect);
        if(((itemType&127) == userItem) && handle)
                {
                DisposeRoutineDescriptor((UniversalProcPtr)handle);
                }
        }
#endif
}
/******************************************************************************************
                                CloseDialogue
Ferme la fenetre, libere le dialogue libere les ressource DITL et DLOG associ‚es
=> Pointeur sur la fenetre a ferme, ID des ressources a liberer
<= Ptr sur le dialogue
******************************************************************************************/
void CloseDialogue(DialogPtr  dialog,short idDITL)
{
DisposeUserItems(dialog,idDITL);
DisposDialog(dialog);
return;
}
/******************************************************************************************
                                    DoAlertSapiens
******************************************************************************************/
short DoAlertSapiens(short num)
{
short n=Alert(num,NULL);
tfrvirt();
TfrEcr(NULL);
return(n);
}
/******************************************************************************************
                                    GetRectItem
=> Recupere le cadre d'un item donn‚
******************************************************************************************/
void GetRectItem(DialogPtr dialog,short item,Rect * box)
{
short itemType;
Handle handle;

 /* On recupere l'item coorespondant … l'aire (seule l' aire nous interesse) */
GetDItem(dialog,item,&itemType,&handle,box);
return;
}
/*****************************************************************************
                       DrawArrowPopUp
****************************************************************************/
void DrawArrowPopUp(Rect * r)
{
short i;
short y=(r->bottom+r->top)/2+3;
Rect box;

SetRect(&box,r->right-15,r->top+1,r->right-1,r->bottom-1);
DoEraseRect(&box);
for(i=0;i<6;i++)
    {
    DoMoveTo(r->right-10-i,y-i);
    DoLine(i*2,0);
    }
return;
}
/******************************************************************************************
                                    GetStr
Recupere une chaine ind‚x‚e dans une locale et la passe au format C
<= Ptr sur la locale
******************************************************************************************/
static char OneString[200];
char * GetStr(short index)
{
GetIndString((StringPtr)OneString,CvtDefaultLangRess(STR_VRAC), index);
PtoCstr((StringPtr)OneString);
return(OneString);
}
/**********************************************************************
                    initrnd
***********************************************************************/
void initrnd(short sem1,short sem2,short sem3)
{
CarboniseSetRandSeed(((long)sem1<<16)+((long)sem2<<8)+sem3);
return;
}
/**********************************************************************
                    rnd0
***********************************************************************/
short rnd0(void)
{
return(Random()&0xFF);
}
/**********************************************************************
                    Rnd
***********************************************************************/
short Rnd(void)
{
return(Random()&0xFF);
}
/**********************************************************************
                    myRandom
Sort une valeur entre deux bornes
***********************************************************************/
long myRandom(long min,long max)
{
unsigned short rdm;
long range,t;

rdm=(long)Rnd();
rdm+=(long)Rnd()*256L;
range=max-min;
t=((long)rdm*(long)range)/65536L;
return(min+t);
}
/**********************************************************************
                    bande
Affiche une bande sur les cot‚s
***********************************************************************/
/*;********* BANDE_: 0= bande jeu, 1= bande silex *******************/
void bande(short quelle)
{
}
/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                        SAPDISK.S
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
/****************************************************************************
Boxf: trace une boite sur le port sp‚cifi‚
Les  coordonn‚es sont donn‚es en ecran ST 320x200
(C) Ligne dep,Col dep,Ligne fin,col.fin,index couleur ST
***************************************************************************/
void boxf(short lignedep,short coldep,short lignefin,short colfin,short index)
{
Rect aire;

SetRect(&aire,coldep*ECHELLE,lignedep*ECHELLE,colfin*ECHELLE,lignefin*ECHELLE);
BoxFill(CarboniseGetCurrentPort(),&aire,palhaute[index]);
return;
}
void irqon(void)
{
}
/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
               ECRAN64.S
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
/*;------------------------------------------------------
;               CADREG:Trace un cadre graphique en pointille
;-------------------------------------------------------
;=> numero ligne pixel deut *2,col octet,n ligne/2,n col
;<= sur ecran cadre xor
*/
void cadreg(short ligne,short col, short nligne, short ncol)
{
//Rect r;

//SetRect(&r,col*8*ECHELLE,ligne*8*ECHELLE,col*8+ncol*8,ligne*8+nligne*8);
//ForeColor(blackColor);
//DoFrameRect(&r);
return;
}

/*;-----------------------------------------------------------------
;               HIRES;Passe en resolution et detourne Erreurs Disques
;-----------------------------------------------------------------------
*/
short hires(void)
{
return(1);
}
/******************************************************************************************
                                AdjustMenus
******************************************************************************************/
void AdjustMenus(void)
{
MenuHandle  menu;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
CarboniseSetPortWindow(mainWindow);

menu = GetMHandle(menuPomme);
menu = GetMHandle(menuFichier);
if(MenuReduit) DisableItem(menu,0);
else           EnableItem(menu,0);

menu = GetMHandle(menuVoir);
if(MenuReduit || mort==0) DisableItem(menu,0);
else           EnableItem(menu,0);
menu = GetMHandle(menuFenetre);
CheckItem(menu, iEchelle100, SapZoom==1.0 );
CheckItem(menu, iEchelle200, SapZoom==2.0 );
CheckItem(menu, iEchelle300, SapZoom==3.0 );
CheckItem(menu, iEchelle400, SapZoom==4.0 );
DrawMenuBar();
AcamSetPortOffscreen(gbak);
return;
}
/******************************************************************************************
                                FixeMenusReduit
******************************************************************************************/
void FixeMenusReduit(void)
{
if(!MenuReduit)
    {
    MenuReduit=VRAI;
    AdjustMenus();
    }
return;
}
/******************************************************************************************
                                FixeMenusComplet
******************************************************************************************/
void FixeMenusComplet(void)
{
if(MenuReduit)
    {
    MenuReduit=FAUX;
    AdjustMenus();
    }
return;
}
/******************************************************************************************
                                ConfirmePerteAventure
******************************************************************************************/
Boolean ConfirmePerteAventure(void)
{
MontreSouris();
return(DoAlertSapiens(ALRT_PERTE_PARTIE)==1);
}
/******************************************************************************************
                                DoInfo
******************************************************************************************/
void DoInfo(void)
{
char str[255];

GetVersionSapiens(str);    
CtoPstr(str);
ParamText((StringPtr)str,(StringPtr)"",(StringPtr)"",(StringPtr)"");
DoAlertSapiens(ALRT_INFO);
return;
}
/******************************************************************************************
                                DoMenuCommand
******************************************************************************************/
void DoMenuCommand(long menuResult)
{
short       menuID, menuItem;
Str255      daName;

AdjustMenus();
MontreSouris();
menuID = HiWord(menuResult);    /* use macros for efficiency to... */
menuItem = LoWord(menuResult);  /* get menu item number and menu number */
if(menuID==menuPomme)
    {
    switch(menuItem)
            {
            case itemInformation:DoInfo();break;
            case itemAide:
                      DoAide();
                      break;
            default: GetItem(GetMHandle(menuPomme), menuItem, daName);
                     OpenDeskAcc(daName);
                     break;
            }
    }
else
if(menuID==menuFichier)
    {
        switch(menuItem)
            {
            case itemNouveau: trgest(3);
                    break;
            case itemCharger:
                    trgest(1);
                    inimen(0);
                    break;
            case itemSauver:
                    trgest(2);
                    break;
            case itemPreferences: DoPreferences();break;
            case itemQuitter: if(ConfirmePerteAventure()) SapiensExit(0); break;
            }
    }
else
if(menuID==menuVoir)
    {
        switch(menuItem)
            {
            case iPaysage: 
							trobsv(0); 
            				if (flcomb) TraiteVueLaterale();
            			    break;
            case iLaterale:  
            				trobsv(1);
                            TraiteVueLaterale();
                            break;
            case iPanorama: trobsv(2); break;
            case iCarte: trobsv(3); break;
            }
	}
else
if(menuID==menuFenetre)
    {
        switch(menuItem)
            {
            case iEchelle100: SapZoom=1.0;break;
            case iEchelle200: SapZoom=2.0;break;
            case iEchelle300: SapZoom=3.0;break;
            case iEchelle400: SapZoom=4.0;break;
			}
		FixeTailleMainWindow();
    }
HiliteMenu(0);
AdjustMenus();
DrawMenuBar();
return;
}
/*******************************************************************
                        DoEvtClavier
Gestion du clavier
=> evenement a traiter
********************************************************************/
static void DoEvtClavier(EventRecord * event)
{
short car;
char chaine[255];
short i;
long t;


if(event->modifiers & cmdKey)
    {
    car=event->message & 0xFF;
    AdjustMenus();
    /*if(car==30) ChangeIotaPalette(0.01);
    else if(car==31) ChangeIotaPalette(-0.01);
    else
    */
    DoMenuCommand(MenuKey(event->message & charCodeMask));
    }
else
if(event->modifiers & optionKey)
    {
    car=(event->message&0xFF00)>>8;
    if(car==0x11/*T*/) /* Transfert Ecran */
        {
        t=TickCount();
        for(i=0;i<200;i++)
            TfrEcr(NULL);
        sprintf(chaine,"%d transferts totaux de l'ecran : %ld ticks",i, TickCount()-t);
        Collapse(chaine);

        }
   if(car==0x02/*'D'*/) /* 3d */
        {
        t=TickCount();
        //InitProfile(200,200);
        for(i=0;i<50;i++)
            {
            affich(dzeta);
            }
        //DumpProfile();
        sprintf(chaine,"%d images 3d : %ld ticks",i, TickCount()-t);
        Collapse(chaine);
        }
    if(car==0x23/*'P'*/)
        {
        AffPalette();
        }
    if(car==0x29/*'M'*/)
        {
        if(quelle_musique==1) MusicInit(2);
        else MusicInit(1);
        }
    if(car==0x06/*'T'*/)
        {
        for(i=0;i<64;i++) chrono(1);
        }
    if(car==0x0f/*R*/)
        {
        InitMobs();
        }
    if(car==0x01/*S*/)
        {
        MaxMem(&t);
        //bruit(9);
        }
    }
else
    {
    car=event->message&charCodeMask;
    /* Touches de fonction*/
    if(car==0x10)
        {
        DoAide();
        }
    }
return;
}
/*******************************************************************
                        delai
=> Nbre de 60eme de seocndes a atteindre
********************************************************************/
void delai(long duree)
{
long t=TickCount()+duree;

do
    {
    GereInterface();
    } while(t>TickCount());
}



/*******************************************************************
                        GereInterface
Gestion  g‚n‚rale de l'interface
********************************************************************/
void GereInterface(void)
{
short part;
WindowPtr windowResult;
CGrafPtr gbak;
EventRecord event;
WindowPtr window;
Rect bounds;

AcamGetPortOffscreen(&gbak);
CarboniseSetPortWindow(mainWindow);
window=FrontWindow();
CheckChangeMusique();
while(GetNextEvent(everyEvent,&event))
    {
    switch(event.what)
        {
        case keyDown:
                toucheClav=event.message &charCodeMask;
                DoEvtClavier(&event);
                break;
        case mouseDown:
         /* On cherche quelle fenetre a ‚t‚ cliqu‚e */
        part = FindWindow(event.where, &windowResult);
        /* O— a s t'on cliqu‚  ???*/
        switch ( part )
            {
            /* Dans la barre de menu , alors on le traite */
            case inMenuBar:
                    DoMenuCommand(MenuSelect(event.where));
                  break;
           /* Sur le bureau, alors le bureau se d‚brouille ... sauf si c'est un pseudo document
             */
            case inSysWindow:
                    SystemClick(&event, windowResult);
                    break;
           /* Dans le deplacement */
            case inDrag:
                    SelectWindow(windowResult);
					CarboniseGetQDRectScreenBits(&bounds);
                    DragWindow(windowResult, event.where, &bounds);
                    break;
          /* Dans une fenetre : alors on le traite */
            case inContent:
                    if( windowResult != window )
                             SelectWindow(windowResult);
                     //if(window==mainWindow)
                     //       DoClick(event.where);
                       break;
             }
         break;
        case updateEvt:
                if(window==mainWindow)
                    {
//                    AcamGetPortOffscreen(&gbak);
                    CarboniseSetPortWindow(window);
                    BeginUpdate(window);
                    TfrEcr(NULL);
                    tfrima();
                    if (dervue == VUE_LATERALE) tfrdemi();
                    EndUpdate(window);
//                    AcamSetPortOffscreen(gbak);
                    }
                break;
       case activateEvt:
                    if(event.message&1)
                        {
                        /* on reprends le traitement */

                        }
                    else
                        {
                        /* On est arr‚te */
                        MontreSouris();
                        }

                break;
        case app4Evt:
                if((event.message >> 24)==1)
                    {
                    if(event.message&1)
                        {
                        /* on reprends le traitement */
                        CarboniseSetPortWindow(mainWindow);
                        TfrEcr(NULL);
                        tfrima();
                        if (dervue == VUE_LATERALE) tfrdemi();
                        }
                    else
                        {
                        /* On est arr‚te */
                        MontreSouris();
                        }
                    }
                break;

         }
    }
SystemTask();
AcamSetPortOffscreen(gbak);
return;
}

/*******************************************************************
                        CalcIndexPalette
Calcule la deuxieme section de la palette ST en entr‚e :
index de la couleur dans la palette systeme
********************************************************************/
static void CalcIndexPalette(short * palette)
{
short i;
RGBColor rgb;

for(i=0;i<16;i++)
    {
    CvtCouleurEnRgb(palette,i,&rgb);
    palette[16+i]=SapColor2Index(&rgb);
    }
}
/*******************************************************************
                        FixeMenu
=> Numero de la menu Bar … installer, Numero du menu Pomme de cette menuBar
********************************************************************/
Boolean FixeMenu(short n)
{
Handle  menuBar;
MenuHandle menu;

menuBar = GetNewMBar(n);         /* read menus into menu bar */
if ( menuBar == NULL ) return(FAUX);
SetMenuBar(menuBar);                    /* install menus */
DisposHandle(menuBar);
menu=GetMHandle(menuPomme);
if(menu)
    AddResMenu(menu, DRVR);   /* add DA names to Apple menu */
DrawMenuBar();
return(VRAI);
}
/******************************************************************
                    InitMobs
Charge ou recharge les mobs
******************************************************************/
Boolean InitMobs(void)
{
short nb;
CGrafPtr gbak;
short vol;
long path;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);

LibereDataMobs();
PushPath();
SetPathInData("Mobs");
mobBambou=0;
nb=ChargeFichierBrt("WBAMBOU.BRT", mobBambou+1,26,0);      /*0, 21 Mobs */
if(nb<0) return(FAUX);
mobFace=mobBambou+nb;
nb=ChargeFichierBrt("WFACE.BRT",mobFace+1,31,0);     /* Base 21,29 mobs */
if(nb<0) return(FAUX);
mobFeuille=mobFace+nb;
nb=ChargeFichierBrt("WFEUILLE.BRT",mobFeuille+1,36,11);    /* Base 50 36 mobs */
if(nb<0) return(FAUX);
mobFleches=mobFeuille+nb;
nb=ChargeFichierBrt("WFLECHES.BRT",mobFleches+1,20,0);     /* Base 8618 mobs */
if(nb<0) return(FAUX);
mobSapmob=mobFleches+nb;
nb=ChargeFichierBrt("WSAPMOB.BRT",mobSapmob+1,141,0);   /*Base 120+1,151 mobs */
if(nb<0) return(FAUX);
mobAvplan=mobSapmob+nb;
nb=nbMobHerbes=ChargeFichierBrt("WAVPLAN.BRT",mobAvplan+1,9,13);
if(nb<0) return(FAUX);
mobAvplanSpec=mobAvplan+nb;
nb=nbAvplanSpec=ChargeFichierBrt("WAVSPEC.BRT",mobAvplanSpec+1,4,0);
if(nb<0) return(FAUX);
mobSilex=mobAvplanSpec+nb;
nb=ChargeFichierBrt("WSILEX.BRT",mobSilex+1,15,0);
if(nb<0) return(FAUX);
mob3D=mobSilex+nb;
nb=ChargeFichierBrt("W3D.BRT",mob3D+1,38,0);
if(nb<0) return(FAUX);
Fonte14=mob3D+nb;
nb=ChargeFichierBrt("Geneva14.BRT",Fonte14+1,120,0);
if(nb<0) return(FAUX);
Fonte10=Fonte14+nb;
nb=ChargeFichierBrt("Geneva10.BRT",Fonte10+1,120,0);
if(nb<0) return(FAUX);
AcamSetPortOffscreen(gbak);
PopPath();
return(VRAI);
}
/***********************************************************************************
                                 AfficheFond
Affiche le tramage de fond de la fenetre
***********************************************************************************/
static void AfficheFond(CGrafPtr port)
{
short x,y;
Rect r;
PicHandle picMotif;
CGrafPtr gbak;
Rect portRect;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(port);
CarboniseGetRectPort(port,&portRect);
/* Le fond au dessus de l'awale */
picMotif=GetPicture(128);
for(y=0;y<portRect.bottom;y+=64)
    for(x=0;x<portRect.right+16;x+=64)
        {
        BackColor(whiteColor);
        ForeColor(blackColor);
        SetRect(&r,x,y,x+64,y+64);
        DrawPicture(picMotif,&r);
        }
ReleaseResource((Handle)picMotif);
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************************
                                 FixeTitreMainWindow
***********************************************************************************/
void FixeTitreMainWindow(void)
{
char chaine[255];
char str[255];
short i;

strcpy(chaine,"Sapiens ");
GetVersionSapiens(str);
strcat(chaine,str);
CtoPstr(chaine);
CarboniseSetPortWindow(mainWindow);
SetWTitle(mainWindow,(StringPtr)chaine);
return;
}

/****************************************************************************
            Encadre l'aire 3d
=> etat 1 actif
*****************************************************************************/
 void Frame3d(short etat)
{
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
SetRect(&r,62,0,640-64,204);
if(etat) ForeColor(blackColor);
else ForeColor(whiteColor);
DoPenSize(2,2);
DoFrameRect(&r);
ForeColor(blackColor);
PenNormal();
CarboniseSetPortWindow(mainWindow);
if(etat) ForeColor(blackColor);
else ForeColor(whiteColor);
DoPenSize(2,2);
DoFrameRect(&r);
PenNormal();
ForeColor(blackColor);
AcamSetPortOffscreen(gbak);
return;
}
/**********************************************************************************
                    MenuFenetres
Entre ou sort du menu fenetre actif
=> etat =1 si actif
***********************************************************************************/
void MenuFenetres(short etat)
{
Frame3d(etat);
tfrmenufenetre(etat);
if(etat) FixeMenusComplet();
else FixeMenusReduit();
dicone=-1;
return;
}
/********************************************************************************************** 
												ValideUpdate
***********************************************************************************************/
void ValideUpdate(void)
{
EventRecord event;
CarboniseValideUpdate();
#ifdef aMAC
WaitNextEvent(everyEvent,&event,0L,NULL);
#endif
}
/********************************************************************************************** 
                                                                AffStr
***********************************************************************************************/
static void AffStr(char * chaine,short fonte,short size,short face,short x,short y)
{
Rect r;

CtoPstr(chaine);
TextFont(fonte);
TextSize(size);
TextFace(face);
if(x<0)  x=(640-StringWidth((StringPtr)chaine))/2;
ForeColor(blackColor);
DoTextSize(size);
DoMoveTo(x+1,y+1);
DrawString((StringPtr)chaine);
ForeColor(whiteColor);
DoMoveTo(x,y);
DrawString((StringPtr)chaine);
DoMoveTo(x-1,y);
DrawString((StringPtr)chaine);
ForeColor(blackColor);
PtoCstr((StringPtr)chaine);
SetRect(&r,0,0,320*ECHELLE*SapZoom,200*ECHELLE*SapZoom);

return;
}

/******************************************************************************************
                                SetFontDialog
Fixe la fonte du dialogue en entr‚e
******************************************************************************************/
static void SetFontDialog(DialogPtr dialog,short font,short taille)
{
FontInfo info;
TEHandle te;

CarboniseSetPortDialog(dialog);
TextFont(font);
TextSize(taille);
GetFontInfo(&info);
te=CarboniseGetDialogTextEditHandle(dialog);
if(te)
    {
    (*te)->txFont=font;
    (*te)->txSize=taille;
    (*te)->lineHeight=info.descent+info.ascent+info.leading;
    (*te)->fontAscent=info.ascent;
    }
return;
}
/*******************************************************************
                        OuvreDialogue
********************************************************************/
DialogPtr OuvreDialogue(short numDitl)
{
DialogPtr dialog;

dialog=GetNewDialog(numDitl,NULL,(WindowPtr)-1L);
if(dialog)
	{
	SetFontDialog(dialog,geneva,9);
	CarboniseAdapteStaticText(dialog,numDitl,GetStdFont(),9);
	SetDialogDefaultItem(dialog,1);
	}
return(dialog);
}

#ifdef PC
/*****************************************************************
				SetThemeDefaultSapiens
Fixe dans le theme du bureau le theme sapiens par defaut
*******************************************************************/
#define NOM_FICHIER_BACKGROUND "desert.bmp"
void SetThemeDefaultSapiens(Collection collection)
{
#ifdef OBSOLETE
Handle h;
char bgFileName[255];
short typeFond;
RGBColor couleur;
short volumeCourant;
long pathCourant;


strcpy(bgFileName, NOM_FICHIER_BACKGROUND);

/* Image de fond */
AddCollectionItem(collection,THEMESTRING,acamBackgroundBMPName,strlen(bgFileName)+1,bgFileName);
SaisitChemin(&volumeCourant,NULL,&pathCourant);
SetPathApplication();
h=AcamReadBMPFile(bgFileName);
FixeVolPath(volumeCourant,pathCourant);
if(h)
	{
	AddCollectionItem(collection,THEMEBMP,0,GetHandleSize(h),*h);
	DisposeHandle(h);
	}
/* Type d'affichage de fond de bureau */
typeFond=acamBackgroundStretch;
AddCollectionItem(collection,THEMEVALUE,acamBackgroundType,sizeof(short),(Ptr)&typeFond);

/* Couleur du theme */
couleur.red=0xA200;
couleur.green=0x5D00;
couleur.blue=0x0000;
AddCollectionItem(collection,THEMERGBCOLOR,acamReplacementThemeRGBColor,sizeof(RGBColor),(Ptr)&couleur);
#endif
}

#endif
/*******************************************************************
                        FixeTailleMainWindow
********************************************************************/
void FixeTailleMainWindow(void)
{
CGrafPtr gbak;
Rect rect;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(CarboniseGetWindowPort(mainWindow));
SetRect(&rect,0,0, 320*ECHELLE*SapZoom,200*ECHELLE*SapZoom);
SizeWindow(mainWindow,rect.right,rect.bottom,true);
CarboniseInvalRect(&rect);
AcamSetPortOffscreen(gbak);
}

/*******************************************************************
                        SapiensExit
********************************************************************/
void SapiensExit(short code)
{
ArreteMusique(true);
exit(code);
}

/*******************************************************************
                        InitMainWindow
********************************************************************/
void InitMainWindow(void)
{
Rect rect,aireDevice;

AcamInitOffscreenSystem(&aireDevice);
/* Creation de la fenetre principale */
/* Aire de la fenetre */
SetRect(&rect,0,0,320*ECHELLE*SapZoom,200*ECHELLE*SapZoom);
/* On la centre sur l'aire generale de l'ecran */
OffsetRect(&rect,
			aireDevice.left+(aireDevice.right-aireDevice.left-320*ECHELLE*SapZoom)/2,
			aireDevice.top+(CarboniseGetMenuBarHeight()+8)+(aireDevice.bottom-aireDevice.top-CarboniseGetMenuBarHeight()-200*ECHELLE*SapZoom)/2);
/* On cree la fenetre principale */
mainWindow=NewCWindow(NULL,&rect,(StringPtr)"",VRAI,noGrowDocProc/*plainDBox*/,(WindowPtr)-1L,FAUX,0L);
return;
}
/*******************************************************************
                        init_graph
********************************************************************/
Boolean init_graph(void)
{
short i,y,num;
PicHandle pict;
Rect r,rdest;
char chaine[255];
long timeBase;
Boolean firstInit;

#ifdef ACAM2
	//ChargeValeurDefautThemeWindows();
	MaxApplZone();
	InitGraf((GrafPtr *) &qd.thePort);
	InitFonts();
	InitWindows();
	InitMenus();
	TEInit();
	InitDialogs(NULL);
	//InitCursor();
	qd.randSeed=TickCount();
#endif
CarboniseInitQuickdDraw();
/* Init des chemins standards */
InitCheminsSapiens();
SetCursor(*GetCursor(watchCursor));
#ifdef PC
	AcamSetDefaultTheme("XP",SetThemeDefaultSapiens);
	InitTheme();
#endif
FixeAireNoMouseAireBasse();
#ifdef PC
if(OpenResFile("Rsrc\\Sapiens.rsr")==-1)
    {
    AcamErreur("Fichier Sapiens.rsr absent. Reinstallez le logiciel");
    SapiensExit(-1);
    }
#else
/* Teste la version du systeme */
#endif
/* On charge les preferences */
firstInit=ChargePref();
InitDefautConfigSortieNumerique();
MusicInit(1);               /* Lance musique 1 */
/* Choix de la langue*/
if(langue==-1)
    {
    CarboniseSetCursorArrow();
    langue=Alert(ALRT_VERSION,NULL)-1;
    SetCursor(*GetCursor(watchCursor));
    }
InitAndCheckTexts();
InitMainWindow();
FixeTitreMainWindow();
CarboniseSetPortWindow(mainWindow);
AfficheFond(CarboniseGetWindowPort(mainWindow));
#ifdef ACAM2
   	AcamSetWindowTranslucentRatio(mainWindow,0);
	AcamSetDeferredShow(mainWindow,false);
#endif
/* On affiche le logo centre */
pict=GetPicture(134);
if(pict)
    {
    r=(*pict)->picFrame;
    rintel2moto(&r);
    OffsetRect(&r,(320*ECHELLE-(r.right-r.left))/2,60);
	DoZoomRectSiEcranReel(&r,&rdest);
    DrawPicture(pict,&rdest);
    ReleaseResource((Handle)pict);
    }
/* Affichage de la version de sapiens */
GetVersionSapiens(chaine);
AffStr(chaine,helvetica,14,bold,-1,r.bottom+16);
/* Affichage des autres commentaires */
strcpy(chaine,"Copyright Didier & Olivier Guillion, 1986,1996,2005,2017");
AffStr(chaine,helvetica,16,bold,-1,r.bottom+32);
i=1;
y=60+120+18+32;
num=CvtDefaultLangRess(1);
do
	{
	GetIndString((StringPtr)chaine,num,i++);
	PtoCstr((StringPtr)chaine);
	if(strlen(chaine))
			AffStr(chaine,helvetica,16,bold,-1,y);
	y+=18;
	} while(strlen(chaine));
timeBase=TickCount()+5L*60L;

SetRect(&r,0,0,320*ECHELLE*SapZoom,200*ECHELLE*SapZoom);
CarboniseInvalRect(&r);
ValideUpdate();
/* On initialise le menu */
MessInit(GetStr(26));/* Initialisation du menu */
if(!FixeMenu(MENU_BAR))
    {
    Collapse(GetStr(11) /*"Reinstallez le logiciel SVP"*/);
    SapiensExit(-1);
    }
FixeMenusReduit();
CarboniseSetPortWindow(mainWindow);
MessInit(GetStr(20));/* Initialsations des ecrans */
CalcIndexPalette(pal1);
CalcIndexPalette(pal2); 
Ecran1=SapiensCreeOffscreen(256*ECHELLE,100*ECHELLE);
if(Ecran1==NULL)
        {
        Collapse("[SapWind.init_graph.1] Unable to create offscreen");
        SapiensExit(-2);
        }
Ecran2=SapiensCreeOffscreen(256*ECHELLE,100*ECHELLE);
AcamSetPortOffscreen(Ecran1);
MessInit(GetStr(21));/* Initialsations des palettes */
/* Fabrique les luts */
FabriqueLuts();
debecr=SapiensCreeOffscreen(320*ECHELLE,200*ECHELLE);
AcamSetPortOffscreen(debecr);
AfficheFond(debecr);
/* Buffer de la souris*/
BuffMouse=SapiensCreeOffscreen(64*ECHELLE,64*ECHELLE);
MessInit(GetStr(22));/*Chargement des graphismes */
/* On charge les mobs */
if(!InitMobs())
    {
    Collapse(GetStr(11) /*"Reinstallez le logiciel SVP"*/);
    SapiensExit(-1);
    }
InitCouleurFontes();
MessInit(GetStr(23));/*Chargement des d‚grad‚s */
/* On calcule tout les degrades*/
CalculeToutDegrades();
MessInit(GetStr(24));/*Chargement des d‚cors trois 3 */
CalculeEtoiles();
MessInit(GetStr(25));/* Initialisation de la musique */
/* Initialisation de la musique */
#ifndef aDEBUG
while(timeBase>TickCount())/**/;
#endif
/* Installation du timer de comptage de frame */
InstalleTimeTask();
/* Transfert de l'ecran */
CarboniseSetCursorArrow();
return(firstInit);
}
/* EOF */
