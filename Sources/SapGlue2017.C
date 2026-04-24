/*************************************************************************
                    SapGlue2017.c
                (c) Dg le 18-03-2017
***************************************************************************/
#include "sapiens.h"
#ifdef MAC
Boolean ExtIsApplicationActive(void) {return(true);}
void ExtInvalideAireItem(DialogPtr dialog,short item)
{
}
void ExtDoModalDialog(void*  contexte,short * item,Point * ptResult)
{
}
 DialogPtr ExtOuvreDialogue(short id,short modePlacement,Boolean IsMenuEdition,Boolean IsPrint,Boolean isModale,void* contexte)
{
}
short ExtSetValItemUTF(DialogPtr dialog,short itemHit,char * c)
{
return(1);
}
void CallBackErreur(char * err)
{
Collapse(err);
}
/*********************************************************************
                        ExtGetNomLogiciel
*******************************************************************/
void ExtGetNomLogiciel(char * str)
{
strcpy(str,"Sapiens");
}
/*********************************************************************
                        ExtGetStrVS
*******************************************************************/
char * ExtGetStrVS(short index)
{
return("Virtual Singer...");
}

Ptr ExtSaisieTexte(char *data, TextStyle * textStyle, char  * backColor){return(NULL);}
 short ExtQuelElementSelectionne(ListHandle liste)
{
}
 
void ExtFermeDialogue(void *  contexte)
{
}
void ExtGetRectItem(DialogPtr dialog,short item,Rect * box)
{
GetRectItem(dialog,item,box);
}
void ExtSelectElementListe(ListHandle liste,short n)
{
}
void ExtCollapse(char * format,...)
{
char chaine[512];
va_list  argptr;

va_start( argptr, format );
vsprintf( chaine, format, argptr );
va_end(argptr);

Collapse("%s",chaine);

}
#endif
/******************************************************************************************
                                    ClavierNoteOn
=> numinstrument (-1 si toute)
******************************************************************************************/
void ClavierNoteOn(short numInstrument,short note,short velocite,unsigned char doigte,Boolean isStopNow)
{
}
/******************************************************************************************
                                    ClavierNoteOff
******************************************************************************************/
void ClavierNoteOff(short numInstrument,short note,Boolean isImmediate)
{
}

/******************************************************************************************
                                    IsClavierActive
******************************************************************************************/
Boolean IsClavierActive(void)
{
return(false);
}

void ScriptTraceLog(char * format,...){}

Boolean Confirmation(WindowPtr window,Boolean isUTF,char * str)
{
return(true);
}
void SetPathTmp(void){}
Boolean ConfirmationVrac(short idStr){return(true);}
void Desenregistre(void){;}
Boolean IsEnreg(void){return(true);}
void CheckEnreg(void){}
#ifdef MAC
#ifndef MAC_ACAM
	/*********************************************************************
	                       LocalCarboniseNewNavObjectFilterProc
	********************************************************************/
	 NavObjectFilterUPP CarboniseNewNavObjectFilterProc(NavObjectFilterProcPtr  userRoutine)
	{
	return(NewNavObjectFilterUPP(userRoutine));
	}
	/*********************************************************************
	                       CarboniseDisposeNavObjectFilterProc
	********************************************************************/
	 void CarboniseDisposeNavObjectFilterProc(NavObjectFilterUPP  filterProc)
	{
	DisposeNavObjectFilterUPP(filterProc);
	}
	/*********************************************************************
	                       CarboniseNewNavEventProc
	********************************************************************/
	 NavEventUPP CarboniseNewNavEventProc(NavEventProcPtr  userRoutine)
	{
	return(NewNavEventUPP(userRoutine));
	}
	/*********************************************************************
	                       CarboniseDisposeNavEventProc
	********************************************************************/
	 void CarboniseDisposeNavEventProc(NavEventUPP  eventProc)
	{
	DisposeNavEventUPP(eventProc);
	}
#endif
#endif


/*EOF */
