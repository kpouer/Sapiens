/**********************************************************************************
                        SapEnreg.C
Gestion de l'enregistrement du l'exemplaire :
***********************************************************************************/
#include "sapiens.h"

char TabCvt1[]="AEUTNZPFSRDGJKLM";
char TabCvt2[]="123456789-XWZTNH";
unsigned short Code(unsigned short base)
{
unsigned short d0,d1;

d0=base^0x2867;
d1=d0;
d0=((d0&15)<<12)+(d0>>4);
d0^=6;
d0^=d1;
d1=d0;
d0=((d0&15)<<12)+(d0>>4);
d0^=9;
d0^=d1;
d1=d0;
d0=((d0&15)<<12)+(d0>>4);
d0^=11;
d0^=d1;
return(d0);
}
static Boolean VerifNumSerie(register char * str)
{
register short i,j;
register unsigned short clef=0;
register unsigned short base=0,n;

for(i=0;i<8;i+=2)
        {
        n=str[i];
        for(j=0;j<16 && TabCvt1[j]!=n;j++)/**/;
        if(j==16) return(false);
        clef=(clef<<4)+j;
        n=str[i+1];
        for(j=0;j<16 && TabCvt2[j]!=n;j++)/**/;
        if(j==16) return(false);
        base=(base<<4)+j;
        }
n=clef;
clef=Code(base);
if(clef!=n) return(false);
if(str[8]!='S') return(false);
return(true);
}
/**********************************************************************************
                                        IsEnreg
Teste si l'exemplaire est enregistrÇ
<= VRAI si l'exemplaire est enregistrÇ
***********************************************************************************/
Boolean IsEnreg(void)
{
/* On extrait le nom et le numero de serie */
if(strlen(nomJoueur)==0 || strlen(numSerie)==0) return(FAUX);
return(VerifNumSerie(numSerie));
}
enum e_item
        {
        i_ok=1,
        i_annuler,
        i_nom,
        i_serie
        };
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
        if (CarboniseGetDialogKeyboardFocusItem(dlg)+1==i_serie)
                {
                /* Passe le numero de serie en majuscule */
                if(car>='a')
                        {
                        car&=223;
                        event->message=(event->message&0xFFFFFF00L)|car;
                        }
                }
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
/**********************************************************************************
                                        SaisieEnregistrement
Demande Ö l'utilisateur de saisir son numero d'enregistrement et son nom
***********************************************************************************/
Boolean SaisieEnregistrement(void)
{
DialogPtr dialog;
short item;
char str[255],chaine[255];
CGrafPtr gbak;
short i;
ModalFilterUPP modalUPP;

AcamGetPortOffscreen(&gbak);
dialog=OuvreDialogue(DLOG_NUM_SERIE);
if(dialog==NULL)
        {
        AcamSetPortOffscreen(gbak);
        return(FAUX);
        }
modalUPP=CarboniseNewModalFilterProc(dlgFilter);
do
        {
        ModalDialog(modalUPP,&item);
        if(item==i_ok)
                {
                GetValItem(dialog,i_serie,str);
                for(i=0;i<strlen(str);i++) if(str[i]>='a') str[i]&=223;
                if(!VerifNumSerie(str))
                        {
                        SelIText(dialog,i_serie,0,32767);
                        Collapse(GetStr(4)/*"NumÇro d'enregistrement invalide"*/);
                        item=0;
                        }
                else
                        {
                        GetValItem(dialog,i_nom,str);
                        if(strlen(str)==0)
                                {
                                item=0;
                                Collapse(GetStr(5)/*"Vous devez entrer un nom"*/);
                                }
                        else
                                {
                                if(strlen(str)>TAILLE_MAX_NOM-1)
                                        {
                                        item=0;
                                        Collapse(GetStr(6)/*"Le nom est trop long"*/);
                                        }
                                }
                        SelIText(dialog,i_nom,0,32767);
                        }
                if(item)
                        {
                        strcpy(chaine,GetStr(7)/*Confirmez-vous l'enregistrement sous le nom ""*/);
                        GetValItem(dialog,i_nom,str);
                        strcat(chaine,str);
                        strcat(chaine,"' ?");
                        if(!Confirme(chaine))
                                item=0;
                        }
                }
        } while(item != i_ok && item != i_annuler);
DisposeRoutineDescriptor((UniversalProcPtr)modalUPP);
if(item == i_ok)
        {
        GetValItem(dialog,i_serie,numSerie);
        for(i=0;i<strlen(numSerie);i++) if(numSerie[i]>='a') numSerie[i]&=223;
        GetValItem(dialog,i_nom,nomJoueur);
        numSerie[TAILLE_MAX_NOM_USER-1]=0; /* On limite le nom Ö 40 caractäres */
        nomJoueur[TAILLE_MAX_NOM_USER-1]=0; /* On limite le nom Ö 40 caractäres */
        for(i=0;i<strlen(nomJoueur);i++)
                if(nomJoueur[i]==' ') nomJoueur[i]='_';
        SauvePref();
        FixeTitreMainWindow();
        DoAlertSapiens(ALRT_OK_ENREG);
        }
CloseDialogue(dialog,DLOG_NUM_SERIE);
AcamSetPortOffscreen(gbak);
return(item==i_ok);
}
/******************************************************************************************
                                Desenregistre
******************************************************************************************/
void Desenregistre(void)
{
if(Confirme(GetStr(14)))
	{
	PreservePath();
	SetPathPref();
	remove(NOM_PREF);
	memset(numSerie,0,TAILLE_MAX_NOM_USER);
	memset(nomJoueur,0,TAILLE_MAX_NOM_USER);
	FixeTitreMainWindow();
	RestorePath();
	}
return;
}
/******************************************************************************************
                                NoEnreg
******************************************************************************************/
void NoEnreg(void)
{
if(DoAlertSapiens(ALRT_NO_FONCTION)==2) CheckEnreg();
}
/******************************************************************************************
                                CheckEnreg
******************************************************************************************/
void CheckEnreg(void)
{
Boolean cont=VRAI;

MontreSouris();
CarboniseSetCursorArrow();
while(cont)
    {
    switch(DoAlertSapiens(ALRT_ENREG))
        {
        case 1: cont=FAUX;break;    /* Pas encore */
        case 2: if(SaisieEnregistrement()) cont=FAUX;
                    break;
        case 3: DoPrintBonCommande(); break;
        }
    }
return;
}

/*EOF*/
