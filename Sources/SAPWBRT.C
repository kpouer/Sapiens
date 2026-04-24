/******************************************************************
                        SAPWBRT.C
                Chargement et affichages d'objest graphiques sauvegardes
                en RVB 256 couleurs
La structure fichier contient les donn‚es sur la palette suivie des
donn‚es des mobs.
Le tableau des debuts de mobs est compl‚t‚ correctement.
Ecran 1 et Ecran 2:
L'ecran virtuel est compos‚ de deux parties de 100 lignes chaques
    La premiere partie est le fond brut
    La deuxieme est le fond de travail
Pour l'affichage des combats les effacements et transfert sont
partiels
    effdemi transfere la premiere partie sur la deuxieme Ecran1=> Ecran2
    tfrdemi transfere la deuxieme partie sur l'ecran r‚‚l (Ecran2 => mainWindow)
    tfrdemivirt transfere Ecran1 vers debecr

Les tranfserts totaux sont :
    effima  : Efface avec une couleur le virtuel (Ecran sur 200 lignes)
    effvirt : Efface avec une couleur RVB st  l'aire de la vue3d sur debecr
    tfrima  : Transfert des 100 premiere ligne de Ecran vers mainWindow
    tfrvirt : Transfere d'Ecran 1 (premiere partie) vers debecr

TfrEcr : Transfere une aire de debecr vers mainWindow
******************************************************************/
#include "sapiens.h"
#define NB_MAX_PALETTES 16
#define TAILLE_TFR 66
static PalCoul TabPalette[NB_MAX_PALETTES];
static short nbPalettes;
Mob TabMob[NB_MAX_MOB];
Handle CvtOffscreenToPngHandle(CGrafPtr offscreen,Boolean isAlpha,Boolean isForceTrueColor,char * comment,char * title,char * author,char * subject);
#define CONVERT 0
#pragma ___2017___ Kit ECHELLE
/****************************************************************************
            DoZoomRect
*****************************************************************************/
void DoZoomRect(Rect * rs,Rect *realDest)
{
SetRect(realDest,rs->left*SapZoom,rs->top*SapZoom,rs->right*SapZoom,rs->bottom*SapZoom);
}
/****************************************************************************
            DoZoomRectSiEcranReel
*****************************************************************************/
void DoZoomRectSiEcranReel(Rect * rs,Rect *realDest)
{
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	DoZoomRect(rs,realDest);
else
	*realDest=*rs;
}
/****************************************************************************
            DoFrameRect
*****************************************************************************/
void DoFrameRect(Rect * aire)
{
Rect r;

DoZoomRectSiEcranReel(aire,&r);
FrameRect(&r);
}
/****************************************************************************
            DoEraseRect
*****************************************************************************/
void DoEraseRect(Rect * aire)
{
Rect r;

DoZoomRectSiEcranReel(aire,&r);
EraseRect(&r);
}
/****************************************************************************
            DoPaintRect
*****************************************************************************/
void DoPaintRect(Rect * aire)
{
Rect r;

DoZoomRectSiEcranReel(aire,&r);
PaintRect(&r);
}
/****************************************************************************
            DoPenSize
*****************************************************************************/
void DoPenSize(short x,short y)
{
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	PenSize(x*SapZoom,y*SapZoom);
else 
	PenSize(x,y);
}
/****************************************************************************
            DoMoveTo
*****************************************************************************/
void DoMoveTo(short x,short y)
{
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	MoveTo(x*SapZoom,y*SapZoom);
else 
	MoveTo(x,y);
}
/****************************************************************************
            DoLineTo
*****************************************************************************/
void DoLineTo(short x,short y)
{
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	LineTo(x*SapZoom,y*SapZoom);
else 
	LineTo(x,y);
}
/****************************************************************************
            DoLine
*****************************************************************************/
void DoLine(short x,short y)
{
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	Line(x*SapZoom,y*SapZoom);
else 
	Line(	x,y);
}
/****************************************************************************
            DoTextSize
*****************************************************************************/
void DoTextSize(short x)
{
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	TextSize(x*SapZoom);
else 
	TextSize(x);
}
/****************************************************************************
            DoTextBox
*****************************************************************************/
void DoTextBox(char * text,short length,Rect * box,short just)
{
Rect dest;

DoZoomRectSiEcranReel(box,&dest);
TextBox(text,length,&dest,just);
}
/****************************************************************************
            DoScrollRect
*****************************************************************************/
void DoScrollRect(
   Rect *  r,
  short         dh,
  short         dv,
  RgnHandle     updateRgn) 
{
Rect dest;

DoZoomRectSiEcranReel(r,&dest);
if(CarboniseGetCurrentPort()==CarboniseGetWindowPort(mainWindow))
	dh*=SapZoom,dv*=SapZoom;
ScrollRect(&dest,dh,dv,updateRgn);
CarboniseInvalRect(r);
ValideUpdate();
}  
/******************************************************************************************
                                Read
=> Lecture d'une zone
******************************************************************************************/
static  Boolean Read(FILE * f,Ptr p,long taille)
{
while(taille--) *(p++)=getc(f);
return(VRAI);
}
/******************************************************************
                    CvtPathMacToPC
Convertit un chemin mac en chemin PC
******************************************************************/
void CvtPathMacToPC(char * nom)
{
#ifdef PC
if(nom[0]==':') memcpy(nom,nom+1,strlen(nom));
while(*nom)
    {
    if(*nom==':') *nom='\\';
    nom++;
    }
#endif
return;
}
/******************************************************************
                   CvtColorToIndex
******************************************************************/
static short CvtColorToIndex(PalCoul * pal,RGBColor * rgb)
{
short i;
long delta,deltamin=-1,imin;


for(i=0;i<pal->nbEntrees;i++)
	{
	delta=labs((long)rgb->red-(long)pal->tabRgb[i].red);	
	delta+=labs((long)rgb->green-(long)pal->tabRgb[i].green);	
	delta+=labs((long)rgb->blue-(long)pal->tabRgb[i].blue);	
	if (deltamin<0||delta<deltamin) 
		{
		deltamin=delta;
		imin=i;
		}
	}
return(imin);
}
/******************************************************************
                   ChargeFichierBrt
Chargement et conversion en PNG des anciens fichiers MOBS
=> nom du fichier, premier mob
<= Nombre de mobs charg‚s ou -1 si errreur
******************************************************************/
CGrafPtr ConvertPNGFileToOffscreen(char * fileName);
short FuturChargeFichierBrt(char * nom,short id,short nbMobs,short vide)
{
short i;
CGrafPtr gbak;
char nomPng[512];
CGrafPtr offscreenSource;
Rect aire;
short rowByte;
UPtr p;
UPtr ps;
Ptr pd;
short x,y,taille;
RGBColor rgb;
FILE *f;

AcamGetPortOffscreen(&gbak);
PushPath();
FixeChemin(0L,"PAL");
sprintf(nomPng,"%s.pal",nom);
f=fopen(nomPng,"r");
if(f)
	{
	short n;
	
	fscanf(f,"%hd",&n);
	TabPalette[nbPalettes].nbEntrees=n;
	taille=n*sizeof(RGBColor);
	TabPalette[nbPalettes].tabRgb=(RGBColor*)NewPtr(taille);
	TabPalette[nbPalettes].tabIndex=(short*)NewPtr(n*sizeof(short));
	/* On memorise le nombre d'entrees de la PalCoul */
	for(i=0;i<n;i++)
  	  	{
		fscanf(f," %hd %hd %hd %hd",&TabPalette[nbPalettes].tabIndex[i],&TabPalette[nbPalettes].tabRgb[i].red,&TabPalette[nbPalettes].tabRgb[i].green,&TabPalette[nbPalettes].tabRgb[i].blue);
		}
	fclose(f);
	}
PopPath();

PushPath();
FixeChemin(0L,"PNG");
for(i=0;i<nbMobs;i++,id++)
	{
	sprintf(nomPng,"%s-%d.png",nom,id);
	offscreenSource=ConvertPNGFileToOffscreen(nomPng);
	if(offscreenSource)
		{
		ps=AcamGetInfoOffscreen(offscreenSource,&aire,&rowByte,NULL);
		taille=aire.right*aire.bottom;
  	  	TabMob[id].nbcol=aire.right;
    	TabMob[id].nbligne=aire.bottom;
    	TabMob[id].indexPalette=nbPalettes;
    	TabMob[id].datas=pd=NewPtr(taille);
    	TabMob[id].vide=vide;
		for(y=0;y<aire.bottom;y++)
			{
			p=ps+(y*rowByte);
			for(x=0;x<aire.right;x++)
				{
				#ifdef MAC
					rgb.red=(*(p++))<<8;
					rgb.green=(*(p++))<<8;
					rgb.blue=(*(p++))<<8;
					p++;
				#else
					rgb.blue=(*(p++))<<8;
					rgb.green=(*(p++))<<8;
					rgb.red=(*(p++))<<8;
					p++;
				#endif
				*(pd++)=CvtColorToIndex(&TabPalette[nbPalettes],&rgb);
				}
			}
		AcamClosePortOffscreen(offscreenSource);
		}
	
	}
PopPath();
nbPalettes++;
return(nbMobs);
}
/******************************************************************
                    OldChargeFichierBrt
Chargement et conversion en PNG des anciens fichiers MOBS
=> nom du fichier, premier mob
<= Nombre de mobs charg‚s ou -1 si errreur
******************************************************************/
short ChargeFichierBrt(char * nom,short id,short nbMobs,short vide)
{
register short i;
FILE * f;
short n,nbcol,nbligne;
short nb_objets;
long taille;
unsigned char r,g,b;
RGBColor * rgb;
char chaine[255];

strcpy(chaine,nom);
CvtPathMacToPC(chaine);
/*On ouvre le fichier en lecture */
f=fopen(chaine,"rb");
if(f==NULL)
    {
    sprintf(chaine,"Charge fichier Brt : Fichier '%s' non trouv‚",nom);
    Collapse(chaine);
    return(-1);
    }
if(getc(f)!='B' || getc(f)!='R' || getc(f)!='U' || getc(f)!='T')
    {
    Collapse("Charge fichier Brt : Mauvaise en tete");
    fclose(f);
    return(-1);
    }
if(nbPalettes>=NB_MAX_PALETTES)
    {
    Collapse("Plus de PalCoul libre");
    fclose(f);
    return(-1);
    }
/* Nombre d'entrees dans la PalCoul */
if(!Read(f,(Ptr)&r,sizeof(char))) return(-1);
n=r+1;
TabPalette[nbPalettes].nbEntrees=n;
taille=n*sizeof(RGBColor);
TabPalette[nbPalettes].tabRgb=rgb=(RGBColor*)NewPtr(taille);
TabPalette[nbPalettes].tabIndex=(short*)NewPtr(n*sizeof(short));
/* On memorise le nombre d'entrees de la PalCoul */
for(i=0;i<n;i++)
    {
    if(!Read(f,(Ptr)&r,sizeof(char))) return(-1);
    if(!Read(f,(Ptr)&g,sizeof(char))) return(-1);
    if(!Read(f,(Ptr)&b,sizeof(char))) return(-1);
    rgb->red=(((short)r)<<8)+(short)r;
    rgb->green=(((short)g)<<8)+(short)g;
    rgb->blue=(((short)b)<<8)+(short)b;
	TabPalette[nbPalettes].tabIndex[i]=SapColor2Index(rgb);
    rgb++;
    }
#if CONVERT>0
	{
	FILE * fpalette;
	char nomPalette[256];
	
	PushPath();
	FixeChemin(0L,"PAL");
	sprintf(nomPalette,"%s.pal",nom);
	fpalette=fopen(nomPalette,"w");
	if(fpalette)
		{
		fprintf(fpalette,"%d\n",TabPalette[nbPalettes].nbEntrees);
		for(i=0;i<TabPalette[nbPalettes].nbEntrees;i++)
   			 {
			fprintf(fpalette,"%d %d %d %d\n",TabPalette[nbPalettes].tabIndex[i],TabPalette[nbPalettes].tabRgb[i].red,TabPalette[nbPalettes].tabRgb[i].green,TabPalette[nbPalettes].tabRgb[i].blue);
			}
		fclose(fpalette);
		}
	PopPath();
	}
#endif
/*       On charge les donn‚es de configuration */
if(!Read(f,(Ptr)&nb_objets,sizeof(short))) return(-1);
nb_objets=wintel2moto(nb_objets);
if(nb_objets!=nbMobs)
	{
	printf("ChargeFichierBrt err\n");
	SapiensExit(-1);
	}
/* On charge les mobs */
i=nb_objets;
/* On recherche une place libre pour stocker NB mob dans le tableau
des debuts de sprites
*/
while(i--)
    {
    if(!Read(f,(Ptr)&nbcol,sizeof(short))) return(-1);
    if(!Read(f,(Ptr)&nbligne,sizeof(short))) return(-1);
    nbcol=wintel2moto(nbcol);
    nbligne=wintel2moto(nbligne);
    taille=nbcol*nbligne;
    TabMob[id].nbcol=nbcol;
    TabMob[id].nbligne=nbligne;
    TabMob[id].indexPalette=nbPalettes;
    TabMob[id].datas=NewPtr(taille);
    TabMob[id].vide=vide;
    if(TabMob[id].datas==NULL)
        {
        Collapse("Plus de place memoire");
        return(-1);
        }
    if(!Read(f,(Ptr)TabMob[id].datas,taille)) return(-1);
#if CONVERT>0
	if(nbcol>0 && nbligne>0)
		{
		CGrafPtr gbak;
		CGrafPtr offscreenMob;
		CGrafPtr offscreenDest;
		Handle h;
		FILE *fpng;
		char nomPng[512];
		Rect r;
		UPtr p;
		short rowByte;
		short x,y;
		Boolean vide;
		
		SetRect(&r,0,0,nbcol,nbligne);
		AcamGetPortOffscreen(&gbak);
		PushPath();
		FixeChemin(0L,"PNG");
		//offscreenMob=SapiensCreeOffscreen(320*ECHELLE,200*ECHELLE);
		offscreenMob=SapiensCreeOffscreen(nbcol,nbligne);
		AcamSetPortOffscreen(offscreenMob);
        BackColor(whiteColor);
		CarboniseEraseRect(&r);
		Affmobbrt(id,0,0,0,NULL);
		offscreenDest=AcamCreatePortOffscreen(nbcol,nbligne,32);
   	    AcamSetPortOffscreen(offscreenDest);
        BackColor(whiteColor);
        ForeColor(blackColor);
        CopyBits(CarboniseGetPortBitMapForCopyBits(offscreenMob),CarboniseGetPortBitMapForCopyBits(offscreenDest),&r,&r,srcCopy,NULL);
		p=AcamGetInfoOffscreen(offscreenDest,NULL,&rowByte,NULL);
		for(y=0,vide=true;y<nbligne && vide==true;y++)
			{
			for(x=0;x<nbcol;x++)
				{
				if(p[4*x]!=0xFF || p[4*x+1]!=0xFF || p[4*x+2]!=0xFF)
					vide=false;
				}
			p+=rowByte;
			}
			
		if(vide==false)
			{
			h=CvtOffscreenToPngHandle(offscreenDest,false,false,"Sapiens","(c)Myriad 1986-2017","","");
			if(h)
				{
				sprintf(nomPng,"%s-%d.png",nom,id);
				fpng=fopen(nomPng,"wb");
				if(fpng)
					{
					fwrite(*h,GetHandleSize(h),1,fpng);
					fclose(fpng);
					}
				DisposeHandle(h);
				}
			}
		AcamClosePortOffscreen(offscreenMob);
		AcamClosePortOffscreen(offscreenDest);
		AcamSetPortOffscreen(gbak);
		PopPath();
		}
#endif
    id++;
    if(id>=NB_MAX_MOB)
        {
        Collapse("NB_MAX_MOB depasse");
        return(-1);
        }
    }
/* On ferme le fichier */
fclose(f);
nbPalettes++;
return(nb_objets);
}
/******************************************************************
                    CalculePaletteAttenuee
Calcule une palette demi teinte
La lut correspondant est allou‚e c'est a l'appellant de la liberer
-> numero d'un mob appartenant … cette palette, coefficient d'attenuation, nombre maximum de couleurs
a traiter
<- ptr sur le tableau des index sur la palette (short *)
******************************************************************/
short * CalculePaletteAttenuee(short num,float coeff,short nbCoul)
{
short i;
Mob * mob;
PalCoul * palette;
short * tabIndex;
RGBColor rgb;
long coeffl;
long val;

coeffl=256.0*coeff;

mob=&TabMob[num];
if(mob==NULL)
    {
    Collapse("CalculePaletteDemiTeinte : Mob non trouve");
    return(NULL);
    }
/* Palette associ‚e … ce mob */
palette=&TabPalette[mob->indexPalette];
/* On alloue un tableau d'index de taille identique */
tabIndex=(short *)NewPtr(sizeof(short)*palette->nbEntrees);
if(tabIndex)
    {
    for(i=0;i<palette->nbEntrees;i++)
      {
      if(i<nbCoul)
            {
            rgb=palette->tabRgb[i];
            val=(long)((coeffl*(long) rgb.red)>>8L);
            if(val>65535L) val=65535L;
            rgb.red=val;
            val=(long)((coeffl*(long) rgb.green)>>8L);
            if(val>65535L) val=65535L;
            rgb.green=val;
            val=(long)((coeffl*(long) rgb.blue)>>8L);
            if(val>65535L) val=65535L;
            rgb.blue=val;
            tabIndex[i]=SapColor2Index(&rgb);
            }
        else
            {
            tabIndex[i]=SapColor2Index(&palette->tabRgb[i]);
            }
        }
    }
return(tabIndex);
}
/******************************************************************
                    LibereDataMobs
Libere la memoire de toutes les donn‚es des mobs et palettes
******************************************************************/
void LibereDataMobs(void)
{
short i;

for(i=0;i<NB_MAX_MOB;i++)
    {
    if(TabMob[i].datas) DisposePtr(TabMob[i].datas);
    TabMob[i].datas=NULL;
    }
for(i=0;i<nbPalettes;i++)
    {
    DisposPtr((Ptr)TabPalette[i].tabRgb);
    DisposPtr((Ptr)TabPalette[i].tabIndex);
    }
nbPalettes=0;
return;
}
/******************************************************************
                    CalculeAireMob
******************************************************************/
void CalculeAireMob(short num,short x,short y,Rect * r)
{
Mob * mob;

mob=&TabMob[num];
x*=ECHELLE;
y*=ECHELLE;
SetRect(r,x,y,x+mob->nbcol,y+mob->nbligne);
return;
}
/*************************************************************************
                Affmob3d
Affiche un mob sur le port en cours
*************************************************************************/
void Affmob3d(short num,Rect * aire,short * tabIndex)
{
register unsigned char * pdeb;
register unsigned char * ecrDeb;
register long restX, pasX;
register short j;
register unsigned char vide;
unsigned char  * p;
short i;
long restY, pasY;
short tx,ty;
Mob * mob;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
Rect rPort,MonAire;

/* On recherche le debut du mob */
mob=&TabMob[num];
if(mob==NULL)
    {
    DoFrameRect(aire);
    return;
    }
MonAire=*aire;
/* Taille du mob */
tx=mob->nbcol;
ty=mob->nbligne;
vide=mob->vide;
/* Donn‚es du Mob */
p=(unsigned char*)mob->datas;
/* Palette associ‚e … ce mob */
if(tabIndex==NULL) tabIndex=TabPalette[mob->indexPalette].tabIndex;
/* Origine des donn‚es de l'ecran */

data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),&rPort,&tailleLigne,&depth);
if(data==NULL)     return;

/* Calcul des pas en X et en Y */
i=MonAire.right-MonAire.left;
j=MonAire.bottom-MonAire.top;
/* Trop petit */
if(i<=0 || j<=0) return;
pasX=(((long)tx<<16)/(long)i);
pasY=(((long)ty<<16)/(long)j);
/* Calcul des sorties */
if(MonAire.left<0)
    {
    p+=(long)((float)-MonAire.left*(float)pasX/65536.0);
    MonAire.left=0;
    }
if(MonAire.top<0)
    {
    p+=(long)((float)-MonAire.top*(float)pasY/65536.0)*(long)tx;
    MonAire.top=0;
    }
if(MonAire.right>rPort.right)
    {
    MonAire.right=rPort.right;
    }
if(MonAire.bottom>rPort.bottom)
    {
    MonAire.bottom=rPort.bottom;
    }
if(EmptyRect(&MonAire)) return;
data=data+(long)MonAire.left+(long)MonAire.top*(long)tailleLigne;
restY=0L;
i=MonAire.bottom-MonAire.top+1;
while(--i)
    {
    ecrDeb=data;
    pdeb=p;
    restX=0L;
    j=MonAire.right-MonAire.left+1;
    while(--j)
        {
        if(*pdeb!=vide) *(ecrDeb)=tabIndex[*pdeb];
        ecrDeb++;
        restX+=pasX;
        pdeb+=(restX>>16);
        restX&=0xFFFFL;
        }
    restY+=pasY;
    p+=(long)(restY>>16)*(long)tx;
    restY&=0xFFFFL;
    data+=(long)tailleLigne;
    }
return;
}
/******************************************************************
                    Affmobbrt
Affiche un mob en 256 couleurs sur le port courrant
=> Numero du mob, position x (0-320),position y (0-200), sens (0-1),
tableau des index de palette ou NULL si palette standard du Mob
<= rien
******************************************************************/
void Affmobbrt(short num,short x,short y,short sens,short * tabIndex)
{
register unsigned char  * p;
register short i,j;
short tx,ty;
unsigned char coul;
short dx;
Mob * mob;
unsigned char  * pecr;
unsigned char  * pdeb;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
short vide;
short delttx;
Rect rPort;

/* On recherche le debut du mob */
mob=&TabMob[num];
if(mob==NULL)
    {
    Collapse("Mob non trouve");
    return;
    }
x*=ECHELLE;
y*=ECHELLE;
tx=mob->nbcol;
ty=mob->nbligne;
p=(unsigned char*)mob->datas;
/* Palette associ‚e … ce mob */
if(tabIndex==NULL) tabIndex=TabPalette[mob->indexPalette].tabIndex;
/* Le mob est affich‚*/
/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),&rPort,&tailleLigne,&depth);
if(data==NULL)     return;
/* Test de sortie du mob */
/* Sortie Haut */
if(y<0)
    {
    /* Entierment hors ecran */
    if(y+ty<rPort.top) return;
    ty+=y;
    p+=(-y*tx);
    y=0;
    }
/* Sortie basse */
if(y+ty>=rPort.bottom)
    {
    if(y>=rPort.bottom) return;
    ty=rPort.bottom-y;
    }
if(sens)
    {
    dx=-1;
    delttx=0;
    /* Sortie droite */
    if(x>=rPort.right)
        {
        if(x-tx>=rPort.right) return;
        delttx=x-rPort.right;
        x=rPort.right;
        p+=delttx;
        }
    /* Sortie gauche */
    if(x-tx<0)
        {
        if(x<0) return;
        delttx=tx-x;
        }
    tx-=delttx;
    }
else
    {
    dx=1;
    delttx=0;
    /* Sortie droite */
    if(x+tx>=rPort.right)
        {
        if(x>=rPort.right) return;
        delttx=(x+tx)-rPort.right;
        }
    /* Sortie gauche */
    if(x<0)
        {
        if(x+tx<=0) return;
        delttx=-x;
        p+=delttx;
        x=0;
        }
    tx-=delttx;
    }
/* Position d'affichage sur l'ecran */
pdeb=data+((long)y*(long)tailleLigne)+(long)x;
/* Couleur transparente */
vide=mob->vide;
/* Nombre de lignes a tracer */
j=ty+1;
while(--j)
    {
    /* Debut de  ligne */
    pecr=pdeb;
    pdeb+=(long)tailleLigne;
    i=tx+1;
    while(--i)
        {
        /* l'index de la couleur est extrait */
        coul=*(p++);
        /* Si egal a zero alors couleur transparente */
        if(coul!=vide)
                *(pecr)=tabIndex[coul];
        pecr+=dx;
        }
    /* Donn‚es perdues si le mob est coup‚ en colonne */
    p+=(long)delttx;
    }
return;
}
/******************************************************************
                    AffmobCar
Affiche un mob en 256 couleurs sur le port courrant
=> Numero du mob, position x (0-320),position y (0-200), sens (0-1),
tableau des index de palette ou NULL si palette standard du Mob
<= rien
******************************************************************/
void AffmobCar(short num,short x,short y,short couleur)
{
register unsigned char  * p;
register short i,j;
short tx,ty;
unsigned char coul;
short dx;
Mob * mob;
unsigned char  * pecr;
unsigned char  * pdeb;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
short vide;
short delttx;
Rect rPort;
short * tabIndex;

/* On recherche le debut du mob */
mob=&TabMob[num];
if(mob==NULL)
    {
    Collapse("Mob non trouve");
    return;
    }
tx=mob->nbcol;
ty=mob->nbligne;
p=(unsigned char*)mob->datas;
/* Palette associ‚e … ce mob */
tabIndex=TabPalette[mob->indexPalette].tabIndex;
/* Le mob est affich‚*/
/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),&rPort,&tailleLigne,&depth);
if(data==NULL)     return;
/* Test de sortie du mob */
/* Sortie Haut */
if(y<0)
    {
    /* Entierment hors ecran */
    if(y+ty<rPort.top) return;
    ty+=y;
    p+=(-y*tx);
    y=0;
    }
/* Sortie basse */
if(y+ty>=rPort.bottom)
    {
    if(y>=rPort.bottom) return;
    ty=rPort.bottom-y;
    }
dx=1;
delttx=0;
/* Sortie droite */
if(x+tx>=rPort.right)
    {
    if(x>=rPort.right) return;
    delttx=(x+tx)-rPort.right;
    }
/* Sortie gauche */
if(x<0)
    {
    if(x+tx<=0) return;
    delttx=-x;
    p+=delttx;
    x=0;
    }
tx-=delttx;
/* Position d'affichage sur l'ecran */
pdeb=data+((long)y*(long)tailleLigne)+(long)x;
/* Couleur transparente */
vide=mob->vide;
/* Nombre de lignes a tracer */
j=ty+1;
while(--j)
    {
    /* Debut de  ligne */
    pecr=pdeb;
    pdeb+=(long)tailleLigne;
    i=tx+1;
    while(--i)
        {
        /* l'index de la couleur est extrait */
        coul=*(p++);
        /* Si egal a zero alors couleur transparente */
        if(coul!=vide) *(pecr)=couleur;
        pecr+=dx;
        }
    /* Donn‚es perdues si le mob est coup‚ en colonne */
    p+=(long)delttx;
    }
return;
}
/******************************************************************
                    Affmobbrtomb
Affiche un mob en 256 couleurs
=> Debuts des donn‚es de l'ecran,Numero du mob, position x, position y
@= Aire du mob
******************************************************************/
void Affmobbrtomb(short num,short x,short y,short deltah,short deltav,Rect * r)
{
register unsigned char  * p;
register short i,j;
short tx,ty,coul;
Mob * mob;
short * tabIndex;
unsigned char  * pecr;
unsigned char  * pomb;
unsigned char  * pdeb;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
short vide;
Rect rPort;

/* On recherche le debut du mob */
mob=&TabMob[num];
if(mob==NULL) return;
tx=mob->nbcol;
ty=mob->nbligne;
/* Conversion des coordonn‚es */
deltah*=ECHELLE;
deltav*=ECHELLE;
x*=ECHELLE;
y*=ECHELLE;
SetRect(r,x,y,x+tx+deltah,y+ty+deltav);
p=(unsigned char *)mob->datas;
/* Palette associ‚e … ce mob */
tabIndex=TabPalette[mob->indexPalette].tabIndex;
/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),&rPort,&tailleLigne,&depth);
if(data==NULL)     return;
/* Position d'affichage sur l'ecran */
pdeb=data+((long)y*(long)tailleLigne)+(long)x;
/* Couleur transparente */
vide=mob->vide;
j=ty;
/* Pour l'ensemble des lignes */
while(j--)
    {
    y++;
    /* Debut de  ligne */
    pecr=pdeb;
    pomb=pdeb+((long)deltav*(long)tailleLigne)+(long)deltah;
    pdeb+=tailleLigne;
    i=0;
    while(i++<tx)
        {
        /* l'index de la couleur est extrait dans le mob*/
        coul=*(p++);
        /* Si egal a zero alors couleur transparente */
        if(coul!=vide)
                {
                /* Dans l'ecran */
                if(y+deltav<200*ECHELLE)
                    {
                    /* Lecture de l'ecran pour extraire la couleur RGB*/
                    *pomb=TabCvtCoulOmbre[*pomb];
                    }
                /* Conversion de l'index de couleur en index ST */
                *pecr=tabIndex[coul];
                }
        pecr++;
        pomb++;
        }
    }
return;
}
/******************************************************************
                    PlotIndexPixBrt
Ecrit le pixel en coordonn‚e x, y de couleur systeme coul
=> x (0-640) ,y(0-200), index de la couleur systeme
******************************************************************/
void PlotIndexPixBrt(CGrafPtr port,short x,short y,short coul)
{
register unsigned char  * p;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;

/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(port,NULL,&tailleLigne,&depth);
if(data==NULL)     return;
/* Position d'affichage sur l'ecran */
p=data+((long)y*(long)tailleLigne)+(long)x;
*(p)=coul;
return;
}
/******************************************************************
                    GetIndexPixBrt
Retourne l'index de la couleur dans la palette systeme
correspondant … un point dans le port courrant
=> x,y
<= couleur
******************************************************************/
short GetIndexPixBrt(CGrafPtr port,short x,short y)
{
register unsigned char  * p;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;

/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(port,NULL,&tailleLigne,&depth);
if(data==NULL)     return(-1);
/* Position d'affichage sur l'ecran */
p=data+((long)y*(long)tailleLigne)+(long)x;
return(*p);
}
/******************************************************************
                    SwapCouleur
dans un rectangle du port en entr‚e swappe deux couleurs
******************************************************************/
void SwapCouleur(CGrafPtr port,Rect * r, short coul1,short coul2)
{
register unsigned char  * p;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,x,y,depth;

/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(port,NULL,&tailleLigne,&depth);
if(data==NULL)     return;
for(y=r->top;y<r->bottom;y++)
    {
    p=data+((long)y*(long)tailleLigne+(long)r->left);
    x=r->right-r->left;
    while(x--)
        {
        if(*p==coul1) *p=coul2;
        else if(*p==coul2) *p=coul1;
        p++;
        }
    }
}
/****************************************************************************
                            BoxFill
trace une boite sur le port sp‚cifi‚
Les  coordonn‚es sont donn‚es en ecran ST 320x200
(C) Ligne dep,Col dep,Ligne fin,col.fin,couleur ST
***************************************************************************/
void BoxFill(CGrafPtr port,Rect * r,short couleur)
{
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,tx,ty,depth;
Rect aire;

if(EmptyRect(r)) return;
/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(port,&aire,&tailleLigne,&depth);
if(data==NULL)     return;
if(SectRect(r,&aire,&aire))
    {
    couleur=TabCvtCoul[couleur];
    ty=aire.bottom-aire.top;
    tx=aire.right-aire.left;
    /* Position d'affichage sur l'ecran */
    data+=(long)tailleLigne*(long)(aire.top)+aire.left;
    while(ty--)
        {
        memset(data,couleur,tx);
        data+=tailleLigne;
        }
    }
return;
}
/******************************************************************
                    RemplitLigneBrut
Remplit la zone comprise entre deux lignes avec la couleur donn‚e
=> ligne debut, ligne fin ,couleur
******************************************************************/
void RemplitLigneBrut(short y1,short y2,short index)
{
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;

/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
if(data==NULL)     return;
memset(data+(long)tailleLigne*(long)(y1*ECHELLE),index,(long)(y2-y1)*ECHELLE*(long)tailleLigne);
return;
}
/******************************************************************
                    RemplitZoneLignes
                    Travaille sur le port courrant
Remplit la zone  avec le tableau de couleur  donn‚
=> ligne debut (pixel ecran reel), nbre de lignes, Tableau de couleurs
******************************************************************/
void RemplitZoneLignes(short y,short taille,IndexColorPtr tabCouleur)
{
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;

/* Origine des donn‚es de l'ecran */
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
if(data==NULL)     return;
data+=(long)tailleLigne*(long)(y);
while(taille--)
    {
    memset(data,tabCouleur->index,tailleLigne);
    data+=tailleLigne;
    tabCouleur++;
    }
return;
}
static unsigned long DebDeg[]=
    {
       0x80000000L,
       0xC0000000L,
       0xA0000000L,
       0xD0000000L,
       0xC8000000L,
       0xD8000000L,
       0xAC000000L,
       0xD4000000L,
       0xF5000000L,
       0xEC800000L,
       0xEC800000L,
       0xF6800000L,
       0xF5680000L,
       0xFDD00000L,
       0xFAB40000L,
       0xF8d00000L,
       0xFDBC8000L,
       0xFEEC8000L,
       0xFBA90000L,
       0xEEEA0000L,
       0xF6B30000L,
       0xFDAB4000L,
       0xF6b5a000L,
       0xFDD94a00L,
       0xFB652400L,
       0xFDE61800L,
       0xF779B200L,
       0xFDAf2a80L,
       0xFF75EC68L,
       0xFF7D6d94L,
       0xFDD6B6D0L,
       0xFDCB4a14L,
       0xF6EC6560L,
       0xEFBA9300L,
       0xF775B540L
       };
static unsigned short tabpourc[]={3276,9830,19660,32767,45874,55704,62258,65535};

/**************************************************************************
                        TraceDisque
Trac‚ d'un disque
; => AncxD,AncXG_,AncY_: pos depart
;    D5-d6                 Arrivee
;    Epaissr_               Epaisseur
;
***************************************************************************/
void TraceDisque(short * dg,short * dd,short * dy,short fg,short fd,short epaisseur)
{
short fy;
long x1,x2,dx1,dx2,delta;
short sx1,sx2,i,tsx;
register unsigned char  * p;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
unsigned long degrade;
short index1,index2;


if(*dy>0 && epaisseur>0)
    {
    /* Origine des donn‚es de l'ecran */
    data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),NULL,&tailleLigne,&depth);
    if(data==NULL)     return;
    fy=*dy*ECHELLE;
    epaisseur*=ECHELLE;
    x1=((long)(*dg)<<16)*ECHELLE;
    x2=((long)(*dd)<<16)*ECHELLE;
    dx1=ECHELLE*((long)(fg-*dg)<<16)/(long)epaisseur;
    dx2=ECHELLE*((long)(fd-*dd)<<16)/(long)epaisseur;
    while(fy && epaisseur--)
        {
        if(fy<200 )/*Visible ? */
            {
            delta=(x2-x1)>>16;  /* taille en largeur de la ligne */
            sx1=x1>>16;
            for(i=0;i<8;i++)
                {
                index1=TabCvtCoul[palhaute[tabcoul[i]]];
                index2=TabCvtCoul[palhaute[tabcoul[i+1]]];
                sx2=(x1>>16)+((delta*tabpourc[i])>>16);
                if(sx1<0) sx1=0;
                if(sx2>511) sx2=511;
                /* On a sx1 sx2 debut et fin en colonne du morceau de  ligne */
                if(sx1<512 && sx2>0 && sx1!=sx2)
                    {
                    /* Pointeur sur l'ecran */
                    p=data+((long)tailleLigne*(long)fy+(long)sx1);
                    tsx=sx2-sx1; /* Largeur en pixel */
                    degrade=DebDeg[(tsx>32?31:tsx-1)+(fy&3)];
                    while(tsx--)
                        {
                        if(degrade&0x80000000L)
                            *(p)=index1;
                        else
                            *(p)=index2;
                        p++;
                        degrade<<=1;
                        }
                    }
                sx1=sx2;
                }
            }
        x1+=dx1;
        x2+=dx2;
        fy--;
        }
    }

*dg=fg;
*dd=fd;
if(fy<=0) *dy=0;
else *dy=fy/ECHELLE;

return;
}

/**************************************************************************
                                                SapiensCreeOffscreen
Cr‚e un offscreen de taille donn‚e, compatible avec le MaxDevice
=> fenˆtre d'affichage, taille X offscreen, taille Y offscreen
<= pointeur sur CGrafPort ou NULL si erreur
**************************************************************************/
CGrafPtr SapiensCreeOffscreen(short taillex,short tailley)
{
return(AcamCreatePortOffscreen(taillex,tailley,8));
}
/**************************************************************************
                       LibereOffscreen
Libere l'offscreen cr‚‚
**************************************************************************/
void LibereOffscreen(CGrafPtr myGrafPort)
{
AcamClosePortOffscreen(myGrafPort);
return;
}

/**************************************************************************
                        CopyOffscreen
Copy un rectangle entre la source et la destination
**************************************************************************/
void CopyOffscreen(CGrafPtr source,CGrafPtr dest,Rect * rsourceIn,Rect * rdestIn)
{
CGrafPtr gbak;
unsigned char * dataSource;
short tailleLigneSource;
unsigned char * dataDest;
short tailleLigneDest,depthSource,depthDest;
short tx,ty;
Rect boundSource,boundDest,Rtfrs,Rtfrd;
Rect realSource;
Rect realDest;

if(!EmptyRect(rsourceIn))
    {
	if(source==CarboniseGetWindowPort(mainWindow)) DoZoomRect(rsourceIn,&realSource);
	else realSource=*rsourceIn;
	if(dest==CarboniseGetWindowPort(mainWindow)) DoZoomRect(rdestIn,&realDest);
	else realDest=*rdestIn;
	// Sur ecran
    if(dest==CarboniseGetWindowPort(mainWindow))
        {		
        AcamGetPortOffscreen(&gbak);
        AcamSetPortOffscreen(dest);
        BackColor(whiteColor);
        ForeColor(blackColor);
        CopyBits(CarboniseGetPortBitMapForCopyBits(source),CarboniseGetPortBitMapForCopyBits(dest),&realSource,&realDest,srcCopy,NULL);
        AcamSetPortOffscreen(gbak);
        }
    else
        {
        /* Copy entre deux offscreen avec palette identique */
        dataSource=(unsigned char *)AcamGetInfoOffscreen(source,&boundSource,&tailleLigneSource,&depthSource);
        dataDest=(unsigned char *)AcamGetInfoOffscreen(dest,&boundDest,&tailleLigneDest,&depthDest);
        /* Par d‚realSource, le rectangle … transf‚rer est la source */
        Rtfrs=realSource;
        /* On passe le bound destination dans le repere de la source */
        OffsetRect(&boundDest,realSource.left-realDest.left,realSource.top-realDest.top);
        /* On intersecte les 3 rectangles */
        SectRect(&Rtfrs,&boundSource,&Rtfrs);
        SectRect(&Rtfrs,&boundDest,&Rtfrs);
        /* On recalcule le rectangle destination */
        Rtfrd=Rtfrs;
        OffsetRect(&Rtfrd,realDest.left-realSource.left,realDest.top-realSource.top);
        dataSource+=((long)tailleLigneSource*(long)Rtfrs.top)+(long)Rtfrs.left;
        dataDest+=((long)tailleLigneDest*(long)Rtfrd.top)+(long)Rtfrd.left;
        ty=Rtfrs.bottom-Rtfrs.top;
        tx=Rtfrs.right-Rtfrs.left;
        while(ty--)
            {
            BlockMove(dataSource,dataDest,tx);
            dataSource+=tailleLigneSource;
            dataDest+=tailleLigneDest;
            }
        }
    }
return;
}
/*************************************************************************
                effdemi
Efface a partir du virtuel ou se trace les personnages en
recopiant la premiere partie de l'ecran virtuel sur la deuxieme
*************************************************************************/
void effdemi(void)
{
Rect rsourc;

SetRect(&rsourc,0*ECHELLE,32*ECHELLE,255*ECHELLE,(32+TAILLE_TFR)*ECHELLE);
CopyOffscreen(Ecran2,Ecran1,&rsourc,&rsourc);
return;
}
/*************************************************************************
                tfrdemi
Vue lat‚rale :
Transfere la partie du virtuel ou se trace les personnages
sur le reel
*************************************************************************/
void tfrdemi(void)
{
Rect rsourc,rdest;

SetRect(&rsourc,0*ECHELLE,32*ECHELLE,255*ECHELLE,(32+TAILLE_TFR)*ECHELLE);
SetRect(&rdest,32*ECHELLE,33*ECHELLE,(32+255)*ECHELLE,(33+TAILLE_TFR)*ECHELLE);
CopyOffscreen(Ecran1,CarboniseGetWindowPort(mainWindow),&rsourc,&rdest);
return;
}

/*************************************************************************
                Effima :
Efface l'ecran virtuel Ecran en entier
=> couleur ST d'effacement
*************************************************************************/
void effima(short couleur)
{
Rect r;

SetRect(&r,0*ECHELLE,0*ECHELLE,255*ECHELLE,100*ECHELLE);
BoxFill(Ecran1,&r,couleur);
return;
}
/*************************************************************************
                effvirt
Efface la zone de la vue 3d sur debecr
=> Couleur RVB St
*************************************************************************/
void effvirt(short couleur)
{
Rect r;

SetRect(&r,32*ECHELLE,1*ECHELLE,(32+255)*ECHELLE,101*ECHELLE);
BoxFill(debecr,&r,couleur);
return;
}
/*************************************************************************
                tfrima
Vue lat‚rale:
Transfere le virtuel Ecran sur le reel mainWindow
Transfert des 100 premiere ligne de Ecran vers mainWindow
*************************************************************************/
void tfrima(void)
{
Rect rsourc,rdest;

SetRect(&rsourc,0*ECHELLE,0*ECHELLE,255*ECHELLE,100*ECHELLE);
SetRect(&rdest,32*ECHELLE,1*ECHELLE,(32+255)*ECHELLE,(1+100)*ECHELLE);
CopyOffscreen(Ecran1,CarboniseGetWindowPort(mainWindow),&rsourc,&rdest);
CarboniseValideUpdate();
return;
}
/*************************************************************************
                tfrvirt
Transfere l'aire de la vue 3d de l'ecran de travail vers l'ecran
debecr virtuel
Transfere d'Ecran (premiere partie) vers debecr
*************************************************************************/
void tfrvirt(void)
{
Rect rsourc,rdest;

SetRect(&rsourc,0*ECHELLE,0*ECHELLE,255*ECHELLE,100*ECHELLE);
SetRect(&rdest,32*ECHELLE,1*ECHELLE,(32+255)*ECHELLE,(1+100)*ECHELLE);
CopyOffscreen(Ecran1,debecr,&rsourc,&rdest);
return;
}
/*************************************************************************
                tfrmenufenetre
=> actif, inactif
*************************************************************************/
void tfrmenufenetre(short etat)
{
Rect r;
CGrafPtr gbak;
RGBColor rgb;

AcamGetPortOffscreen(&gbak);

SetRect(&r,0*ECHELLE,113*ECHELLE,320*ECHELLE,200*ECHELLE);
if(etat)
    {
    CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&r,&r);
    }
else
    {
    CarboniseSetPortWindow(mainWindow);
    CarboniseSetPatGray();
    rgb.red=15000;
    rgb.green=6000;
    rgb.blue=2000;

    RGBForeColor(&rgb);
    PenMode(patOr);
    DoPaintRect(&r);
    ForeColor(blackColor);
    PenNormal();
    AcamSetPortOffscreen(gbak);
    }
return;
}
/*************************************************************************
                CopyEcran1Sur2
*************************************************************************/
void CopyEcran1Sur2(void)
{
Rect rsourc;

SetRect(&rsourc,0*ECHELLE,0*ECHELLE,255*ECHELLE,100*ECHELLE);
CopyOffscreen(Ecran1,Ecran2,&rsourc,&rsourc);
return;
}
/*************************************************************************
                TfrEcr
Transfere une aire de debecr vers la fenetre visible
Transfere de debecr vers MainWindow
*************************************************************************/
void TfrEcr(Rect * r)
{
Rect r2;

if(r==NULL)
    {
    SetRect(&r2,0,0,320*ECHELLE,200*ECHELLE);
    CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&r2,&r2);
    }
else
    CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),r,r);

return;
}
/*************************************************************************
                Affmob
Affiche un mob sur l'ecran virtuel
=> Numero du mob, position y (0-200) ,position x (0-320), sens
*************************************************************************/
void Affmob(short mob,short y,short x,short inverse)
{
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
Affmobbrt(mob,x,y,inverse,NULL);
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                AffmobConst
Affiche un mob sur l'ecran virtuel constant et l'ecran virtuel volatile
=> Numero du mob, position y (0-200) ,position x (0-320), sens
*************************************************************************/
void AffmobConst(short mob,short y,short x,short inverse)
{
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
Affmobbrt(mob,x,y,inverse,NULL);
AcamSetPortOffscreen(Ecran2);
Affmobbrt(mob,x,y,inverse,NULL);
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                AffDebecr
Affiche un mob sur l'ecran debecr
=> Numero du mob, position y (0-200) ,position x (0-320), transfert
*************************************************************************/
void AffDebecr(short mob,short y,short x,short transfere)
{
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
Affmobbrt(mob,x,y,0,NULL);
if(transfere)
    {
    CalculeAireMob(mob,x,y,&r);
    TfrEcr(&r);
    }
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                affomb
Affiche un mob sur l'ecran de travail (debecr) et le recopie sur le reel
=> Coordonn‚es en 320x200
*************************************************************************/
void affomb(short mob,short ligne,short colonne,short deltav,short deltah,Boolean recopie)
{
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
Affmobbrtomb(mob,colonne,ligne,deltah,deltav,&r);
if(recopie)
    CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&r,&r);
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                AffmobPalette
Affiche un mob sur l'ecran virtuel, en entr‚e on a en plus le tableau des
index de couleurs a utiliser
*************************************************************************/
void AffmobPalette(short mob,short y,short x,short inverse,short * tabIndex)
{
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(Ecran1);
Affmobbrt(mob,x,y,inverse,tabIndex);
AcamSetPortOffscreen(gbak);
return;
}
/*************************************************************************
                AffPalette
Affiche la palette en cours
*************************************************************************/
void AffPalette(void)
{
#define TAILLE_CARRE 16L
register unsigned char  * p;
 short i,j,k,l,coul;
unsigned char  * data;/* Donn‚es de l'ecran */
short tailleLigne,depth;
Rect rPort;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
data=(unsigned char *)AcamGetInfoOffscreen(CarboniseGetCurrentPort(),&rPort,&tailleLigne,&depth);
if(data==NULL)     return;
for(i=0;i<16;i++)
    for(j=0;j<16;j++)
        {
        p=data+(i*TAILLE_CARRE*tailleLigne)+j*TAILLE_CARRE+32L;
        coul=i*16+j;
        for(k=0;k<TAILLE_CARRE;k++)
            {
            for(l=0;l<TAILLE_CARRE;l++)
                p[l]=coul;
            p+=tailleLigne;
            }
        }
TfrEcr(NULL);
AcamSetPortOffscreen(gbak);
return;
}
/*******************************************************************
                        GetStdFont
*******************************************************************
short GetStdFont(void)
{
#ifdef MAC
	return(geneva);
#else
	return(geneva);
#endif
}
S2.4*/

/*********************************************************************
					GetRectMainEcran
Donne l'aire de l'ecran principal
 ********************************************************************/
void GetRectMainEcran(Rect * aireEcran) 
{
GDHandle gd;

gd=GetMainDevice();
*aireEcran=(*gd)->gdRect;
return;
} 


/*********************************************************************
                            RetraceItem
Retrace instanatanemen,t l'item demand‚
********************************************************************/
void RetraceItem(DialogPtr dialog,short item)
{
short itemType;
Handle handle;
Rect box;

GetDItem(dialog,item,&itemType,&handle,&box);
RetraceRect(dialog,&box);
return;
}
/***********************************************************************
               SetPathInData
=> id de la picture,aire              
***********************************************************************/
void SetPathInData(char * name)
{
short vol;
long path;

FixeCheminRelatif(volumeApplication,PathApplication,"data");
SaisitChemin(&vol,NULL,&path);
FixeCheminRelatif(vol,path,name);
}
/***********************************************************************
               AffichePictDansAire
=> id de la picture,aire              
***********************************************************************/
void AffichePictDansAire(short num,Rect * r)
{
Rect rDest,rpng;
PicHandle pict;
char nom[255];
CGrafPtr offscreen;

DoZoomRectSiEcranReel(r,&rDest);
PushPath();
// recherche de la picture en fichier
sprintf(nom,"image_%d.png",CvtDefaultLangRess(num));
SetPathInData("images");
if(GetRectFilePng(nom,&rpng))
	{
	offscreen=ConvertPNGFileToOffscreen(nom);
	if(offscreen)
		{
		BackColor(whiteColor);
        ForeColor(blackColor);
        CopyBits(CarboniseGetPortBitMapForCopyBits(offscreen),CarboniseGetPortBitMapForCopyBits(CarboniseGetCurrentPort()),&rpng,&rDest,srcCopy,NULL);
		AcamClosePortOffscreen(offscreen);
		}
	}
else
	{
	pict=GetPicture(CvtDefaultLangRess(num));
	if(pict==NULL)
		pict=GetPicture(CvtLangRess(GB,num));
	if(pict)
		DrawPicture(pict,&rDest);
	}
PopPath();
}
/* EOF */
