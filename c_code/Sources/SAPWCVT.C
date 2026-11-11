/*************************************************************************
                    SapWcvt.c
                (c) Dg le 19-1-1996
Conversion des couleurs

***************************************************************************/
#include "sapiens.h"
/* En fonction de la couleur ST donne l'index de la couleur PC */
unsigned char TabCvtCoul[0x778];
/* En fonction d'une couleur donne son index en ombre */
unsigned char TabCvtCoulOmbre[256];
/*************************************************************************
                FabriqueLuts
*************************************************************************/
void FabriqueLuts(void)
{
short r,v,b,i;
RGBColor rgb;

/* Calcul de la LUT st vers index */
for(r=0;r<8;r++)
    for(v=0;v<8;v++)
        for(b=0;b<8;b++)
            {
            rgb.red=r*9362;
            rgb.green=v*9362;
            rgb.blue=b*9362;
            TabCvtCoul[(r<<8)+(v<<4)+b]=SapColor2Index(&rgb);
             }
for(i=0;i<256;i++)
	{
	Index2Color(i,&rgb);
    rgb.red>>=1;
    rgb.green>>=1;
    rgb.blue>>=1;
	TabCvtCoulOmbre[i]=SapColor2Index(&rgb);
	}
return;
}
/*************************************************************************
                CvtCouleurStEnRgb
*************************************************************************/
void CvtCouleurStEnRgb(short val,RGBColor * rgb)
{
rgb->red  =(long)((val&0x700)>>8)*9362L;
rgb->green=(long)((val&0x070)>>4)*9362L;
rgb->blue =(long)(val&0x007)*9362L;
return;
}
/*************************************************************************
                CvtCouleurEnRgb
*************************************************************************/
void CvtCouleurEnRgb(short * palette,short index,RGBColor * rgb)
{
CvtCouleurStEnRgb(palette[index],rgb);
return;
}
/*************************************************************************
                CvtRgbEnCouleur
Recherche dans la palette st en entr‚e l'index de la couleur
la plus proche en rvb
*************************************************************************/
short CvtRgbEnCouleur(short * palette,RGBColor * rgb)
{
short i,imin;
RGBColor ref;
unsigned long delta,delmin=-1;

for(i=0;i<16;i++)
    {
    CvtCouleurEnRgb(palette,i,&ref);
    delta=abs(ref.green-rgb->green)+abs(ref.red-rgb->red)+abs(ref.blue-rgb->blue);
    if(delta<delmin)
        {
        if(delta==0) return(i);
        delmin=delta;
        imin=i;
        }
    }
return(imin);
}
/*************************************************************************
                CvtIndexEnCouleur
Convertit un index de palette systeme en index de palette ST
en utilisant la deuxieme section de la palette
*************************************************************************/
short CvtIndexEnCouleur(short * palette,short index)
{
short i;

for(i=0;i<16;i++)
    {
    if(palette[i+16]==index)
        return(i);
    }
Collapse("CvtIndexEnCouleur: Index non trouve");
return(-1);
}
/* EOF */