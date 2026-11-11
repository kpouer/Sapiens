/***************************************************************************
                                WSAPIENS.C
                            (c) D‚g‚ le 19-3-96
Fichier specifique … la version WACAM de Sapiens
***************************************************************************/
#define WINDOWS_PLUS_ACAM 1
#define WING 1
#include <acam.h>
#include "sapiens.h"

void (*AcamWindowsRetraceWindow)(WindowPtr window)=NULL;
extern HWND AcamFrameWnd;
static Boolean atexitAide;
#define NOM_FICHIER_AIDE_PC "data\\Sapfr.hlp"
#define NOM_FICHIER_AIDE_PC_GB "data\\Sapgb.hlp"

/********************************************************
				AcamInitSettings
*********************************************************/
void AcamInitSettings(void)
{
strcpy(acamSettings.appTitle,"Sapiens (c) Didier & Olivier Guillion 1986-2017");
strcpy(acamSettings.appWindowName,acamSettings.appTitle);
acamSettings.isMonoInstance=true;
#ifdef DEBUG
	acamSettings.afficheErreurRsrc=VRAI;
#else
	acamSettings.afficheErreurRsrc=FAUX;
#endif
acamSettings.desktopMinSizeX=640;
acamSettings.desktopMinSizeY=400+20;
acamSettings.isMenuBar=true;
acamSettings.isDesktopWindow=true;
acamSettings.desktopWindowLook=DESKTOP_NORMAL|DESKTOP_MAXIMIZE;

}	

/***************************************************
				AcamGetCtrlDefProc
***************************************************/
ControlDefProcPtr AcamGetCtrlDefProc(short ProcID)
{
return(NULL);
}
/***************************************************
				AcamAireTotaleChange
L'aire totale de la fenetre principale a changée
On repositionne la fenetre principale de monkey
***************************************************/
void AcamAireTotaleChange(Rect * aireTotale)
{
}
/**********************************************************
                      AcamConfirmeSortieAppl
**********************************************************/
Boolean AcamConfirmeSortieAppl(void)
{
if(AcamFrameWnd)
    return(ConfirmePerteAventure());
return(VRAI);
}

/**********************************************************
                      GetWindowsVersion
**********************************************************/
long GetWindowsVersion(void)
{
return(GetVersion());
}
/**********************************************************
                      AcamTraiteMessageWindows
**********************************************************/
Boolean AcamTraiteMessageWindows(long hWndDetype,unsigned int messageDetype,unsigned long wParamDetype, long lParamDetype)

{
HWND hWnd=(HWND)hWndDetype;
UINT message=(UINT)messageDetype;
unsigned long wParam=wParamDetype;
LPARAM lParam=(LPARAM)lParamDetype;
switch (message)
      {
//      case MM_WOM_DONE:
//                TraiteDoubleBack(wParam,lParam);
//                return(VRAI);

      }
return(FAUX);
}
/**********************************************************
                      AcamScrapWindowsToAcam
Conversions des scraps <= VRAI si Acam doit traiter les TEXT
**********************************************************/
Boolean AcamScrapWindowsToAcam(void)
{
return(VRAI);
}
/**********************************************************
                      AcamScrapAcamToWindows
Conversions des scraps <= VRAI si Acam doit traiter les TEXT
**********************************************************/
Boolean AcamScrapAcamToWindows(void)
{
return(VRAI);
}
/******************************************************************************************
                                Terminate
******************************************************************************************/
Boolean Terminate(void)
{
return(true);
}

/*************************************************************************************************
					OpenDownloadWebPageAndQuit
H963g mise en fonction
Ouvre la page de telechargement correspondant au programme en cours d'execution, et quitte l'appli
**********************************************************************************************/
EXTERNC void OpenDownloadWebPageAndQuit(void)
{
exit(0);
}
/******************************************************************************************
                                Message
Affiche un message simple, en chaine
Indique que l'action a ete accomplie et fourni le resultat de cette action
Ce message peut etre associe a une fenetre sous forme d'un sheet, il faut alors fournir la fenetre
=> fenetre (ou NULL), encodage UTF ou non message
******************************************************************************************/
void Message(WindowPtr window,Boolean isUTF,char * format,...)
{
short item;
va_list  argptr;
char message[512];

va_start( argptr, format );
vsprintf( message, format, argptr );
Collapse(message);
}


/******************************************************************************
                    GetVersionWindows
*******************************************************************************/
long WindowsOSVersion(void);
long GetVersionWindows(void)
{
return(WindowsOSVersion());
}

void CallBackErreur(char * err)
{
Collapse(err);
}

/*EOF*/

