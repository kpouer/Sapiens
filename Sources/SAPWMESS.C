/*************************************************************************
                    SapWmess.c
                (c) Dg le 19-1-1996
Effacement et affichage Affichage du message central
***************************************************************************/
#include "sapiens.h"
Rect rMess={104*ECHELLE,16*ECHELLE,112*ECHELLE,303*ECHELLE};
/****************************************************************************
            TfrBadAss
Le message s'affiche sur debecr, on le transfere sur le reel
*****************************************************************************/
void TfrBadAss(void)
{
eff_sour();
CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&rMess,&rMess);
CarboniseInvalRect(&rMess);
return;
}
/****************************************************************************
            Effacement du message
*****************************************************************************/
void efbadass(void)
{
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
DoEraseRect(&rMess);
AcamSetPortOffscreen(gbak);
return;
}
/****************************************************************************
            Scrolling du message
*****************************************************************************/
void scrollbadass(void)
{
short i;
CGrafPtr gbak;
Rect reff;

eff_sour();
AcamGetPortOffscreen(&gbak);
CarboniseSetPortWindow(mainWindow);
SetRect(&reff,rMess.left,rMess.top,rMess.left+8,rMess.bottom);
for(i=0;i<rMess.right-rMess.left;i+=8)
	{
    DoScrollRect(&rMess,8,0,NULL);
	DoEraseRect(&reff);
	}
efbadass();
AcamSetPortOffscreen(gbak);
return;
}


/****************************************************************************
            Encadre et efface le message
*****************************************************************************/
void framebadass(void)
{
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
r=rMess;
InsetRect(&r,-1,-1);
DoEraseRect(&r);
DoFrameRect(&r);
DoPenSize(1,1);
AcamSetPortOffscreen(gbak);
return;
}
/*EOF*/
