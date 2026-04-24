/*************************************************************************************
                        SAPPC.C
Code sp‚cifique au Macinstosh pour SAPIENS
(c) DG le 20/1/96
**************************************************************************************/
#define WINDOWS_PLUS_ACAM
#include <acam.h>
#include "sapiens.h"
unsigned char sem1,sem2,sem3;
short TabIndexPeriph[W95_NB_PERIPHS];
/*********************************************************************
                    IsPlayerExterne
********************************************************************/
Boolean IsPlayerExterne(void)
{
return(FAUX);
}
/*********************************************************************
                     FixeFormeOndeCarre
Initialise la forme de l'onde … une onde carr‚e
********************************************************************/
void FixeFormeOndeCarre(Ptr forme){}
/******************************************************************************************
                                GetNumDriverWindows95
=> index du peripherique Windows
<= Index du driver ou -1 si aucun
******************************************************************************************/
short GetNumDriverWindows95(TypePeriphW95 typePeriph)
{
return(TabIndexPeriph[typePeriph]);
}



/*EOF */
