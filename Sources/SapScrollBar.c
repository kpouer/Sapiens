#include "sapiens.h"
#pragma mark ------ Operations sur les Controls---------
/******************************************************************************************
                                CommonAction
Change la valeur de l'ascenseur du control demande.
=> Control, delta a appliquer
<= Valeur reellement appliquee
******************************************************************************************/
short CommonAction(ControlHandle control,short amount)
{
short       value, max;

value = GetCtlValue(control);   /* get current value */
max = GetCtlMax(control);       /* and maximum value */
amount = value + amount;
if ( amount < 0 ) amount = 0;
else if ( amount > max )    amount = max;
SetControlValue(control, amount);
return(amount-value);
}
/******************************************************************************************
                               ActionInfo
******************************************************************************************/
static short whatItem;
static ControlHandle WhatControl;
static pascal void ActionInfo(ControlHandle control,short part)
{
short amount=0;
long t,t2;

if ( part != 0)
    {
    switch ( part )
        {
        case inUpButton:
        case inDownButton:
            amount = 1;
            break;
        case inPageUp:
        case inPageDown:
            amount = 1;
            break;
        }
    if ( (part == inDownButton) || (part == inPageDown) ) amount = -amount;
    if(amount)
        {
  	  	amount=CommonAction(control, -amount);
  	 
        t=TickCount();
        if(whatItem>0)
            RetraceItem(CarboniseGetDialogFromControl(control),whatItem);
        do
            t2=TickCount();
                while(t2<t+10);
        }
    }
return;
}
/*********************************************************************
                        ClickInAscenseur
=> Dialogue, item a retracer en fonction de l'asenseur
Un anti repeat est gere sur l'ascenseur
********************************************************************/
void ClickInAscenseur(DialogPtr dialog,short item,Point pt)
{
short part;
ControlHandle control;
ControlActionUPP ActionUPP;

/* Dans le control ? */
whatItem=item;
/* On determine le control qui a ete clique */
part = CarboniseFindControlDialog(pt, dialog, &control);
WhatControl=control;
/* En fonction de l'endroit du click, on le gere */
switch ( part )
        {
        /* Nulle part d'actif */
        case 0:break;
        /* Dans l'onglet de l'ascenseur */
        case inThumb:
                    CarboniseTrackControl(control, pt, NULL);
                    if(whatItem>0)
                        RetraceItem(CarboniseGetDialogFromControl(control),whatItem);
                    break;

        /* Ailleur que dans l'onglet scroll bar */
          default:

        			 ActionUPP=CarboniseNewControlActionProc(ActionInfo);
                     CarboniseTrackControl(control, pt, ActionUPP);
                     CarboniseDisposeControlActionProc(ActionUPP);
                     break;
        }
return;
}
/*****************************************************************************
                                CarboniseNewScrollBarStd
*****************************************************************************/
ControlHandle CarboniseNewScrollBarStd(DialogPtr dialog,short item,short value,short min,short max)
{
ControlHandle Control;
Rect aireTot;
OSErr err;

/* On saisie les valeurs actuelles*/
GetRectItem(dialog,item,&aireTot);
Control=NewControl(CarboniseGetDialogWindow(dialog),&aireTot,(StringPtr)"",VRAI/* Visible*/,value /* Valeur initiale */,
                         min/* Minimum*/,max /* Maximum */,scrollBarProc,'Scro'/* Reference*/);
err=CarboniseSetNewControl(dialog,item,Control);
if(err)
	Collapse("CarboniseSetNewControl %d",err);
/* On le montre */
ShowControl(Control);
return(Control);
}
/*****************************************************************************
                                CarboniseNewScrollBarWindow
Ajoute un scroll une fenetre
*****************************************************************************/
ControlHandle CarboniseNewScrollBarWindow(WindowPtr window)
{
ControlHandle Control;
Rect aireTot;
OSErr err;

/* On saisie les valeurs actuelles*/
SetRect(&aireTot,0,0,16,16);
Control=NewControl(window,&aireTot,(StringPtr)"",VRAI/* Visible*/,0 /* Valeur initiale */,
                         0/* Minimum*/,0 /* Maximum */,scrollBarProc,'Scro'/* Reference*/);
#if  TARGET_API_MAC_CARBON>0
/* On inclus le control*/
err=AutoEmbedControl(Control,window);
#endif
/* On le montre */
ShowControl(Control);
return(Control);
}
/*EOF*/