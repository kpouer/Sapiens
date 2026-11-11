/*************************************************************
				SapColorCvt.c
Conversion de couleurs
Ajoute le 24/05/05 pour Windows pour compenser l'imprecision de 
la conversion HSL sur la palette standard 8 bits.
Repasse en comparaison simple precedente sur le delta rvb
*****************************************************************/
#define WINDOWS_PLUS_ACAM 1
#include "sapiens.h"


short SapColor2Index(RGBColor * rgb)
{
#ifdef MAC
	return(Color2Index(rgb));
#else
extern CTabHandle AcamDefault8BitsColorTable;
short i;
long delta,deltamin=-1,imin;

for(i=0;i<256;i++)
	{
	delta=labs((long)rgb->red-(long)(*AcamDefault8BitsColorTable)->ctTable[i].rgb.red);	
	delta+=labs((long)rgb->green-(long)(*AcamDefault8BitsColorTable)->ctTable[i].rgb.green);	
	delta+=labs((long)rgb->blue-(long)(*AcamDefault8BitsColorTable)->ctTable[i].rgb.blue);	
	if (deltamin<0||delta<deltamin) 
		{
		deltamin=delta;
		imin=i;
		}
	}
return(imin);
#endif
}