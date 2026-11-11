/******************************************************************************************
                             sapbonc.c
                        Impression d'un bon de commande ressources TEXT 0
                        Dg le 16 aout 1995
****************************************************************************************/
#include "sapiens.h"
//#include <memory.h>
#define TAILLE_STR 128
#define TAILLE_STR_REM 255
typedef struct
	{
	char version[TAILLE_STR];
	char date[TAILLE_STR];
	char nom[TAILLE_STR];
	char adresse[TAILLE_STR];
	char codePostal[TAILLE_STR];
	char ville[TAILLE_STR];
	char pays[TAILLE_STR];
	char eMail[TAILLE_STR];
	Boolean isProchaineVersion;
	Boolean isLiquide;
	char trouve[TAILLE_STR];
	char remarque[TAILLE_STR_REM];
	} Coordonnees;
enum
	{
	i_imprimer=1,
	i_annuler,
	i_nom,
	i_adresse,
	i_codePostal,
	i_Ville,
	i_Pays,
	i_Email,
	i_normal,
	i_disque,
	i_liquide,
	i_cheque,
	i_trouve,
	i_remarque,
	i_rect1,
	i_rect2
	};
#define STR_COMMENTAIRE CVTLANG(100)

/*******************************************************************
                        GetCommentaire
********************************************************************/
static char commentaire[255];
static char * GetCommentaire(short index)
{
GetIndString((StringPtr)commentaire,STR_COMMENTAIRE,index-44);
PtoCstr((StringPtr)commentaire);
return(commentaire);
}
/******************************************************************************************
                                VectFrameRect
Encadre l'item
******************************************************************************************/
static void pascal VectFrameRect(DialogPtr window,short itemid)
{
Rect r;

GetRectItem(window,itemid,&r);
CarboniseSetPatBlack();
FrameRect(&r);
return;
}
/***********************************************************************************
                                                dlgFilter
***********************************************************************************/
static pascal Boolean dlgFilter(DialogPtr dlg,EventRecord * event,short * item)
{
CGrafPtr gbak;
unsigned char car;
Boolean result=FAUX;

AcamGetPortOffscreen(&gbak);
CarboniseSetPortDialog(dlg);
if(event->what == keyDown || event->what ==autoKey )
        {
        car=(event->message & charCodeMask);
        /* On elimine les retour chariot */
        if(car==3 || car==13)
                        result=VRAI; 
        /* ESC efface le champs */
        if(car==27)
                {
                SelIText(dlg,CarboniseGetDialogKeyboardFocusItem(dlg)+1,0,32768);
                DlgCut(dlg);
                result=VRAI; 
                }
        }
AcamSetPortOffscreen(gbak);
return(result);
}
/******************************************************************************************
                                        GetCoordonnees
******************************************************************************************/
static Boolean GetCoordonnees(Coordonnees * c)
{
DialogPtr dialog;
short item;
char chaine[255];
CGrafPtr gbak;
DateTimeRec d;
ModalFilterUPP modalUPP;

AcamGetPortOffscreen(&gbak);
dialog=OuvreDialogue(DLOG_COORDONNEES);
if(dialog==NULL)
	{
	AcamSetPortOffscreen(gbak);
    return(FAUX);
    }
SetValBouton(dialog,i_normal,VRAI);
SetValBouton(dialog,i_liquide,VRAI);
DefAfficheItem(dialog,i_rect1,VectFrameRect);
DefAfficheItem(dialog,i_rect2,VectFrameRect);
CarboniseSetCursorArrow();
modalUPP=CarboniseNewModalFilterProc(dlgFilter);
CarboniseShowDialog(dialog);
do
    {
    ModalDialog(modalUPP,&item);
    switch(item)	
    	{
   		case i_normal:
   		case i_disque:
    		SetValBouton(dialog,i_normal,item==i_normal);
    		SetValBouton(dialog,i_disque,item==i_disque);
    		break;
   		case i_liquide:
   		case i_cheque:
    		SetValBouton(dialog,i_liquide,item==i_liquide);
    		SetValBouton(dialog,i_cheque,item==i_cheque);
    		break;

    	}
    } while(item != i_imprimer && item != i_annuler);
if(item == i_imprimer)
        {
        memset(c,0,sizeof(Coordonnees));
        GetVersionSapiens(chaine);
        sprintf(c->version,"%s ",chaine);
        #ifdef PC
        	strcat(c->version,"PC");
        #else
        	strcat(c->version,"Mac");
		#endif
		GetTime(&d);
		if(version==FR)
			sprintf(c->date,"%d/%d/%d",d.day,d.month,d.year);
		else
			sprintf(c->date,"%d/%d/%d",d.month,d.day,d.year);
		
        GetValItem(dialog,i_nom,c->nom);
        GetValItem(dialog,i_adresse,c->adresse);
        GetValItem(dialog,i_codePostal,c->codePostal);
        GetValItem(dialog,i_Ville,c->ville);
        GetValItem(dialog,i_Pays,c->pays);
        GetValItem(dialog,i_Email,c->eMail);
        c->isProchaineVersion=GetValBouton(dialog,i_disque);
        c->isLiquide=GetValBouton(dialog,i_liquide);
        GetValItem(dialog,i_trouve,c->trouve);
        GetValItem(dialog,i_remarque,c->remarque);
        }
DisposeRoutineDescriptor((UniversalProcPtr)modalUPP);
CloseDialogue(dialog,DLOG_COORDONNEES);
AcamSetPortOffscreen(gbak);
return(item==i_imprimer);
}
/******************************************************************************************
                                        Subst
 Dans le handle h substitue la chaine str1 par la chaine str2                                       
******************************************************************************************/
static void Subst(Handle h,Ptr str1,Ptr str2)
{
short taille=GetHandleSize(h);
short i;
Ptr p=*h;


for(i=0;i<taille;i++)
	{
	if(!strncmp(&p[i],str1,strlen(str1)))
		{
		BlockMove(&p[i+strlen(str1)],&p[i],taille-(i+strlen(str1)));
		SetHandleSize(h,taille+strlen(str2)-strlen(str1));
		taille=GetHandleSize(h);
		p=*h;
		BlockMove(&p[i],&p[i+strlen(str2)],taille-i-strlen(str2));
		BlockMove(str2,&p[i],strlen(str2));
		}
	}
}
/******************************************************************************************
                                        DoPrintBonCommande
******************************************************************************************/
void DoPrintBonCommande(void)
{
TPPrPort PrtPort=NULL;
Rect rpage;
TPrStatus prStatus;
CGrafPtr gbak;
THPrint hprint;
Handle h;
TEHandle texte;
short appResFile;
short printResFile;
Coordonnees c;

if(!GetCoordonnees(&c)) return;
AcamGetPortOffscreen(&gbak);
appResFile=CurResFile();
/* On Initialise le driver imprimante */
PrSetError(noErr);
PrOpen();
if(PrError()==noErr)
    {
    printResFile=CurResFile();
    hprint=(THPrint)NewHandleClear(sizeof(TPrint));
    if(hprint)
        {
        PrintDefault(hprint);
        if(PrStlDialog(hprint))
            {
            /* On demande de les modifier et de confirmer l'impression*/
            if(PrJobDialog(hprint))
                {
                UseResFile(printResFile);
                PrtPort=PrOpenDoc(hprint,NULL,NULL);
                if(PrError()==noErr)
                    {
                    /* On la ouvert */
                    PrOpenPage(PrtPort,NULL);
                    if(PrError()==noErr)
                        {
                        UseResFile(appResFile);
                        rpage=(*hprint)->prInfo.rPage;
                        h=GetResource('TEXT',RSRC_TEXT_CARTE_ENREG);
                        UseResFile(printResFile);
                        if(h)
                            {
                            DetachResource(h);
                            TextFont(courier);
                            TextSize(12);
                            texte=TENew(&rpage,&rpage);
                            if(texte)
                                {
                                Subst(h,"$1",c.version);
                                Subst(h,"$2",c.nom);
                                Subst(h,"$3",c.date);
                                Subst(h,"$4",c.adresse);
                                Subst(h,"$5",c.codePostal);
                                Subst(h,"$6",c.ville);
                                Subst(h,"$7",c.pays);
                                if(strlen(c.eMail)) Subst(h,"$8",c.eMail);
                                else Subst(h,"$8",GetCommentaire(46)); /* Non */
                                if(c.isLiquide) Subst(h,"$9",GetCommentaire(49));
                               	else Subst(h,"$9",GetCommentaire(50)); 
                               if(c.isProchaineVersion) Subst(h,"$A",GetCommentaire(48));
                               	else Subst(h,"$A",GetCommentaire(47)); 
 								Subst(h,"$B",c.trouve);
 								Subst(h,"$C",c.remarque);
                                HLock(h);
                                TEInsert(*h,GetHandleSize(h),texte);
                                HUnlock(h);
                                /* On affiche la page */
                                TEUpdate(&rpage,texte);
                                }
                            DisposHandle(h);
                            }
                        }
                    PrClosePage(PrtPort); /* Impression de la page */
                    }
                PrCloseDoc(PrtPort);
                } /* Si selection papier JOB OK */
            else
                 PrSetError (iPrAbort);
            if(PrError()==noErr && (*hprint)->prJob.bJDocLoop== 1 )
                PrPicFile(hprint,NULL,NULL,NULL,&prStatus);
            }
        else
             PrSetError (iPrAbort);
        /* Liberation du hprint */
        DisposHandle((Handle)hprint);
        }
    }
UseResFile(appResFile);
PrClose();
AcamSetPortOffscreen(gbak);
return;
}
/*EOF*/

/*EOF*/
