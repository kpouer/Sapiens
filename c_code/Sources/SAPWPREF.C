/*************************************************************************
                    SapWPref.C
                (c) Dg le 31-1-96
Pr‚f‚rences
***************************************************************************/
#include "sapiens.h"
#define VERSION_PREF "Sapiens Preferences 2.0.0"
enum e_item
        {
        item_ok=1,
        item_annuler,
		item_langue,
        item_bruitages,
        item_musiques,
		item_polices_lissees,
        item_2_auto,
        item_2_clavier,
        item_rect1
        };
#define ID_MENU  1


/*******************************************************************
                        SetPathPreferences
********************************************************************/
 void SetPathPreferences(void)
{
OSErr err;
long result;
short volRefNum;
long dirId;

err=Gestalt(gestaltFindFolderAttr,&result);
if(err==noErr && (result&1))
	{
	err=FindFolder(kOnSystemDisk,kPreferencesFolderType,kCreateFolder,&volRefNum,&dirId);
	if(err==noErr)
		{
		FixeVolPath(volRefNum,dirId);
		}
	}
return;	
}
/***********************************************************************
                        ChargePref
<= VRAi si first init
***********************************************************************/
Boolean ChargePref(void)
{
FILE * f;
char chaine[255];

/* Valeur par defaut de la musique au depart */
PreservePath();
SetPathPreferences();
f=fopen(NOM_PREF,"rb");
RestorePath();
if(f)
	{
	fread(chaine,sizeof(char),strlen(VERSION_PREF)+1,f);
	if(!strcmp(chaine,VERSION_PREF))
		{
		/* Numero de serie */
		fread(&IsBruitages,sizeof(short),1,f);
		fread(&ModeJoueur2,sizeof(short),1,f);
		fread(&okmuse,sizeof(short),1,f);
		fread(&langue,sizeof(short),1,f);
		fread(&SapZoom,sizeof(float),1,f);
		}
	fclose(f);
	}
return(f==NULL);
}
/***********************************************************************
                        SauvePref
***********************************************************************/
void SauvePref(void)
{
FILE * f;

PreservePath();
SetPathPreferences();
f=fopen(NOM_PREF,"wb");
RestorePath();
if(f==NULL) return;
if(fwrite(VERSION_PREF,sizeof(char),strlen(VERSION_PREF)+1,f)!=strlen(VERSION_PREF)+1)
	{
	Collapse("Error writing preferences.");
	fclose(f);
	return;
	}
fwrite(&IsBruitages,sizeof(short),1,f);
fwrite(&ModeJoueur2,sizeof(short),1,f);
fwrite(&okmuse,sizeof(short),1,f);
fwrite(&langue,sizeof(short),1,f);
fwrite(&SapZoom,sizeof(float),1,f);
fclose(f);
return;
}
/*********************************************************************
                        GetLibLangue
********************************************************************/
void GetLibLangue(short n,char * str)
{
GetIndString((StringPtr)str,128,n+1);
PtoCstr((StringPtr)str);
}
/*********************************************************************
                        VectAfficheMenuLangue
********************************************************************/
static pascal void VectAfficheMenuLangue(DialogPtr window,short itemid)
{
CGrafPtr gbak;
Rect r;
char chaine[255];

AcamGetPortOffscreen(&gbak);
CarboniseSetPortDialog(window);
GetRectItem(window,itemid,&r);
GetLibLangue(langue,chaine);
BackColor(whiteColor);
r.right-=14;
TextFont(times);
TextSize(16);
TextFace(0);
DoTextBox(chaine,strlen(chaine),&r,teJustLeft);
r.right+=14;
InsetRect(&r,-1,-1);
DrawArrowPopUp(&r);
FrameRect(&r);
AcamSetPortOffscreen(gbak);
return;
}
/******************************************************************************************
                                CreeMenuLangue
=> Menu a remplir
******************************************************************************************/
static void CreeMenuLangue(MenuHandle menu)
{
char nom[255];
short i;

for(i=0;i<SAP_NB_LANGUAGES;i++)
	{
	GetLibLangue(i,nom);
	CtoPstr(nom);
	InsMenuItem(menu,(StringPtr)nom,9999);
	}
return;
}
/******************************************************************************************
                                VectFrameBouton
Le bouton est enncadre
******************************************************************************************/
static pascal void VectFrameBouton(DialogPtr window,short item)
{
Rect r;

GetRectItem(window,item,&r);
DoFrameRect(&r);
return;
}
/***********************************************************************
                        DoPreferences
***********************************************************************/
void DoPreferences(void)
{
DialogPtr dialog;
short item;
CGrafPtr gbak;
Point pt;
long result;
MenuHandle menuLangue;

AcamGetPortOffscreen(&gbak);
dialog=OuvreDialogue(DIALOG_PREF);
if(dialog==NULL) return;
CarboniseSetPortDialog(dialog);
SetValBouton(dialog,item_bruitages,IsBruitages);
SetValBouton(dialog,item_musiques,okmuse);
SetValBouton(dialog,item_polices_lissees,LissagePolice);
DefAfficheItem(dialog,item_rect1,VectFrameBouton);
DefAfficheItem(dialog,item_langue,VectAfficheMenuLangue);
SetValBouton(dialog,item_2_auto,ModeJoueur2==JOUEUR2_AUTONOME);
SetValBouton(dialog,item_2_clavier,ModeJoueur2==JOUEUR2_CLAVIER);
MontreSouris();
do
        {
        ModalDialog(NULL,&item);
        switch(item)
                {
                case item_2_auto:
                case item_2_clavier:
                        SetValBouton(dialog,item_2_auto,item==item_2_auto);
                        SetValBouton(dialog,item_2_clavier,item==item_2_clavier);
                        break;
                case item_bruitages:
               case item_musiques:
               case item_polices_lissees:
                        SetValBouton(dialog,item,GetValBouton(dialog,item)^1);
                        break;
                case item_langue:
					menuLangue=NewMenu(ID_MENU,(StringPtr)"");
					InsertMenu(menuLangue,-1);
					CreeMenuLangue(menuLangue);
					GetMouse(&pt);
					LocalToGlobal(&pt);
					result=PopUpMenuSelect(menuLangue,pt.v,pt.h,langue+1);
					 DeleteMenu(ID_MENU);
					DisposeMenu(menuLangue);
				   if(result)
						{
						langue=result-1;
						VectAfficheMenuLangue(dialog,item);
						}
                        break;
                }
        } while(item!=item_ok && item !=item_annuler);
AcamSetPortOffscreen(gbak);
if(item==item_ok)
	{
	if(GetValBouton(dialog,item_2_auto)) ModeJoueur2=JOUEUR2_AUTONOME;
	if(GetValBouton(dialog,item_2_clavier)) 
		{
		DoAlertSapiens(ALRT_ERGO_2_JOUEUR);
		ModeJoueur2=JOUEUR2_CLAVIER;
		}
	IsBruitages=GetValBouton(dialog,item_bruitages);
	okmuse=GetValBouton(dialog,item_musiques);
	LissagePolice=GetValBouton(dialog,item_polices_lissees);
	inimen(8);
	SauvePref();
	FixeMenu(MENU_BAR);
	FixeTailleMainWindow();
	inimen(0);
	}
MusicInit(1);
CloseDialogue(dialog,DIALOG_PREF);
return;
}
/*EOF */
