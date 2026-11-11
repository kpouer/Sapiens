/*************************************************************************
                    SapWmcar.c
                (c) Dg le 19-1-1996
Affichage des caracteres
***************************************************************************/
#include "sapiens.h"
#define USE_SYSTEM_DRAW_CHAR 1
static RGBColor rgbWhite={0xFFFFL,0xFFFFL,0xFFFFL},
                rgbBlack={0x0,0,0},
                rgbBrown={34368,27840,7008},
                rgbGray={50000,50000,50000};
static short indexWhite,indexBlack,indexBrown,indexGray;
Boolean LissagePolice=true ;
/*************************************************************************
                    InitCouleurFontes
**************************************************************************/
void InitCouleurFontes(void)
{
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
indexWhite=SapColor2Index(&rgbWhite);
indexBlack=SapColor2Index(&rgbBlack);
indexBrown=SapColor2Index(&rgbBrown);
indexGray=SapColor2Index(&rgbGray);
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                    Gotoxy
Positionne l'ecriture des caracteres en y et x
=> position en colonne et ligne caracteres
**************************************************************************/
Point ptEcrCar;
void Gotoxy(short x,short y)
{
SetPt(&ptEcrCar,(x*ECHELLE)*6,(y*ECHELLE)*7-1);
colaff=x;
linaff=y;
return;
}
/*************************************************************************
                    GotoxyPixel
Positionne l'ecriture des caracteres en y et x pixel
=> position en colonne et ligne caracteres
**************************************************************************/
void GotoxyPixel(short x,short y)
{
SetPt(&ptEcrCar,x,y);
return;
}


/*************************************************************************
                    TailleCar
**************************************************************************/
short TailleCar(short car,short taille)
{
short num;

if(LissagePolice || langue==ES)// ES Pour les caracteres speciaux
	{
	car=CvtCarLissage(car);
	TextFont(POLICE_LISSAGE);
	TextSize(taille);
	TextFace(STYLE_LISSAGE);
	return(CharWidth(car));
	}
else
	{
	car=CvtCar(car);
	if(taille==10) 
		{
		num=Fonte10+car-31;
		return(TabMob[num].nbcol+2);
		}
	else 
		{
		num=Fonte14+car-31;
		return(TabMob[num].nbcol+3);
		}
	}
}
/*************************************************************************
                    SetColorFont
**************************************************************************/
static void SetColorFont(short id)
{
if(id==indexWhite) RGBForeColor(&rgbWhite);
else if(id==indexBlack) RGBForeColor(&rgbBlack);
else if(id==indexBrown) RGBForeColor(&rgbBrown);
else if(id==indexGray) RGBForeColor(&rgbGray);
else Collapse("SetColorFont: %d unknown",id);
}
/*************************************************************************
                    AffCarBrt
Affiche un caractere brut … la position courante
=> Caractere … afficher
**************************************************************************/
static void AffCarBrt(short car,short couleur,short couleurOmbre,short taille)
{
short num;
Rect rgb;

if(LissagePolice || langue==ES)// ES Pour les caracteres speciaux
	{
	FontInfo info;
	short dx=0,dy=0;
	
	car=CvtCarLissage(car);
	if(car=='<') car=0xC9,dx+=-1;
	if(car=='>') car=0xC9,dx+=-1;
	TextFont(POLICE_LISSAGE);
	TextSize(taille);
	TextFace(STYLE_LISSAGE);
	GetFontInfo(&info);
	SetColorFont(couleurOmbre);
	MoveTo(ptEcrCar.h+dx,ptEcrCar.v+1+info.ascent+dy+1);
	DrawChar(car);
	SetColorFont(couleur);
	MoveTo(ptEcrCar.h+dx,ptEcrCar.v+info.ascent+dy);
	DrawChar(car);
	ptEcrCar.h+=CharWidth(car);
	}
else
	{
	car=CvtCar(car);
	if(taille==10) 
		{
		num=Fonte10+car-31;
		AffmobCar(num,ptEcrCar.h+1,ptEcrCar.v+1,couleurOmbre);
		AffmobCar(num,ptEcrCar.h,ptEcrCar.v,couleur);
		ptEcrCar.h+=TabMob[num].nbcol+2;
		}
	else 
		{
		num=Fonte14+car-31;
		AffmobCar(num,ptEcrCar.h+2,ptEcrCar.v+1,couleurOmbre);
		AffmobCar(num,ptEcrCar.h+1,ptEcrCar.v,couleur);
		AffmobCar(num,ptEcrCar.h,ptEcrCar.v,couleur);
		ptEcrCar.h+=TabMob[num].nbcol+3;
		}
	}
return;
}
/*************************************************************************
                    GetStrBkMot
**************************************************************************/
char * GetStrBkMot(short car)
{
Ptr p="";
char ** bkmots=GetBkMot();

car-=129;
if(car<NB_MOTS_BKMOTS)
	{
	p=bkmots[car];
	if(*p=='$') // Substitution
		{
		if(*(p+1)=='0') p=nomsecondaire;
		else if(*(p+1)=='1') p=nomjoueur;
		else if(*(p+1)=='2') p=nomadversaire;
		else if(*(p+1)=='3') p=nomtribu;
		else if(*(p+1)=='4') p=nomcopain;
		}
	}
else
	printf("GetStrBkMot error\n");
return(p);
}
/*************************************************************************
                    AffCarBank
Affiche un caractere sur le port en entr‚e
**************************************************************************/
static void AffCarBank(CGrafPtr port,short car,short couleur1,short couleur2,short taille)
{
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(port);
if(car>128)
    {
	char * p;
	p=GetStrBkMot(car);
    while(*p) AffCarBank(port,(*(unsigned char *)p++),couleur1,couleur2,taille);
	}
else
    {
    AffCarBrt(car,couleur1,couleur2,taille);
    colaff++;
    }
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                    affcar
Affiche le caractere demand‚ … la position courante
Les caracteres >128 sont des macros substitions
**************************************************************************/
void affcar(short car)
{
AffCarBank(debecr,car,indexBlack,indexBrown,14);
return;
}
/*************************************************************************
                    affcar10
Affiche le caractere demand‚ … la position courante
Les caracteres >128 sont des macros substitions
**************************************************************************/
void affcar10(short car)
{
AffCarBank(debecr,car,indexBlack,indexGray,10);
return;
}
/*************************************************************************
                    Car2
Affiche un caractere blanc sur une ombre noire
Le caractere est affich‚ sur debecr et n'est pas transf‚r‚
**************************************************************************/
void car2(short car)
{
AffCarBank(debecr,car,indexWhite,indexBlack,14);
return;
}
/*************************************************************************
                    affcarmess
Affiche le caractere demand‚ … la position courante
Les caracteres >128 sont des macros substitions
Le caractere est affiche sur debecr
**************************************************************************/
void affcarmess(short car)
{
Rect r;
RgnHandle rgn;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
rgn=NewRgn();
GetClip(rgn);
ClipRect(&rMess);
/* Effacement et affichage curseur */
if(car<32)
        {
        SetRect(&r,ptEcrCar.h,ptEcrCar.v,ptEcrCar.h+8,ptEcrCar.v+11);
        if(car==2)
            {
            CarboniseSetPatBlack();
            DoPaintRect(&r);
            }
        if(car==1)
            DoEraseRect(&r);
        }
else
        AffCarBank(debecr,car,indexBlack,indexGray,14);
SetClip(rgn);
DisposeRgn(rgn);
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************
                        MOTLST2:Affiche Pour jauges
***********************************************************************/
void motlst2(char *debut)
 {
 unsigned char * p=(unsigned char *) debut;

 while (*p)  car2(*(p++));
 return;
 }
/*EOF*/
