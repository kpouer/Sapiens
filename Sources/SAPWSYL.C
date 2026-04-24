/*************************************************************************
                    SapWsyl.c
Gestion graphique du sylex
***************************************************************************/
#include "sapiens.h"
/*
            Gestion du silex
*/
/**********************************************************************
                    lit_sil
Lit un point color‚ dans le silex
=> ligne,colonne ,couleur
***********************************************************************/
short lit_sil(short y,short x)
{
return(GetIndexPixBrt(Ecran,x+53,y)!=palhaute[16]);
}
/**********************************************************************
                    ecrit_sil
Ecrit un point color‚ dans le silex
=> ligne (0-200) ,colonne (0-320),couleur
***********************************************************************/
void ecrit_sil(short y,short x,short couleur)
{
PlotIndexPixBrt(Ecran,x+53,y,palhaute[16+couleur]);
return;
}
/**********************************************************************
                    plot_ecr
Ecrit un point color‚ directement sur l'ecran
=> ligne,colonne ,couleur
***********************************************************************/
void plot_ecr(short y,short x,short couleur)
{
RGBColor rgb;

CvtCouleurEnRgb(palhaute,couleur,&rgb);
RGBForeColor(&rgb);
MoveTo(x*ECHELLE,y*ECHELLE);
Line(ECHELLE,0);
ForeColor(blackColor);
return;
}
/*EOF*/