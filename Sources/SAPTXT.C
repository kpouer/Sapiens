/**********************************************************************
                          TEXTES TOUTES VERSIONS

                        (C) DIDIER 12 Avril 1987
***********************************************************************/
#include "sapiens.h"
char maced[]="HARGIONGIKRUNHOGROKHOULANIRKHUGOTHORNIUPAISNIPHYGLO";

 short langue=-1;//0 Francais, 1 anglais, -1 non defini

 char nomjoueur[9]={"Torgan"},
              nomcopain[]= {"GABU.."};

char nomadversaire[255];
char nomtribu[255];
char nomsecondaire[255];

  char  poschef[]={". ."};
// Francais
extern struct reference refer0_fr[],refer1_fr[],refer2_fr[],refer3_fr[];
extern char *mescon_fr[],*app_fr[],*bkmots_fr[],*scenme_fr[];
//Anglais
extern struct reference refer0_gb[],refer1_gb[],refer2_gb[],refer3_gb[];
extern char *mescon_gb[],*app_gb[],*bkmots_gb[],*scenme_gb[];
//Spanish_es
extern struct reference refer0_es[],refer1_es[],refer2_es[],refer3_es[];
extern char *mescon_es[],*app_es[],*bkmots_es[],*scenme_es[];

#define IDX_NOM_JOUEUR 1
#define IDX_NOM_ADVERSAIRE 2
#define IDX_NOM_TRIBU 3
char * traps[7]= {"CHAUVET","GROTTE","MUSICS","DETAIL","AUROCH","MYRIAD","CAT   "    };

struct reference * GetRefer0(void)
{
if(langue==FR) return(refer0_fr);
if(langue==ES) return(refer0_es);
return(refer0_gb);
}
struct reference * GetRefer1(void)
{
if(langue==FR)  return(refer1_fr);
 if(langue==ES) return(refer1_es);
return(refer1_gb);
}
struct reference * GetRefer2(void)
{
if(langue==FR)  return(refer2_fr);
if(langue==ES) return(refer2_es);
return(refer2_gb);
}
struct reference * GetRefer3(void)
{
if(langue==FR)  return(refer3_fr);
if(langue==ES) return(refer3_es);
return(refer3_gb);
}
struct reference * GetRefer(TypeReference type)
{
switch(type)
	{
	case REFER0:return(GetRefer0());
	case REFER1:return(GetRefer1());
	case REFER2:return(GetRefer2());
	case REFER3:return(GetRefer3());
	default:Collapse("GetRefer bad type");
		break;
	}
return(NULL);
}
struct reference * GetReferLangue(short l,TypeReference type)
{
short lbak=langue;
struct reference * ref;

langue=l;
ref=GetRefer(type);
langue=lbak;
return(ref);
}

char ** GetMesCon(void)
{
if(langue==FR)return(mescon_fr);
if(langue==ES) return(mescon_es);
return(mescon_gb);
}
char ** GetApp(void)
{
if(langue==FR)return(app_fr);
if(langue==ES) return(app_es);
return(app_gb);
}
char ** GetBkMot(void)
{
if(langue==FR)return(bkmots_fr);
if(langue==ES) return(bkmots_es);
return(bkmots_gb);
}
char ** GetScenme(void)
{
if(langue==FR)return(scenme_fr);
if(langue==ES) return(scenme_es);
return(scenme_gb);
}
// Convertit un numero de ressource en numero de ressource langue pour la langue donnee
short CvtLangRess(short l,short id)
{
return(id+(DELTA_LANGUE*l));
}
// Convertit un numero de ressource en numero de ressource langue pour la langue defaut
short CvtDefaultLangRess(short id)
{
return(CvtLangRess(langue,id));
}
/*******************************************************************
                        TailleRef
********************************************************************/
static short TailleRef(struct reference * ref)
{
short n=0;

while(strcmp(ref->nomref,"$END$")) n++,ref++;
return(n);
}
/*******************************************************************
                        TailleBankMots
********************************************************************/
static short TailleBankMots(char* * ref)
{
short n=0;

while(strcmp(ref[n],"$END$")) n++;
return(n);
}
/*******************************************************************
                        CheckCoherenceTexteLangue
********************************************************************/
static void CheckCoherenceTexteLangue(short l)
{
short i;
short lbak=langue;

langue=l;
for(i=REFER0;i<=REFER3;i++)
	if(TailleRef(GetReferLangue(FR,i))!=TailleRef(GetReferLangue(l,i)))
		Collapse("CheckCoherenceTexteLangue : err langue %d reference REFER%d",l,i);
if(TailleBankMots(mescon_fr)!=TailleBankMots(GetMesCon()))
		Collapse("CheckCoherenceTexteLangue : err langue %d GetMesCon",l);
if(TailleBankMots(app_fr)!=TailleBankMots(GetApp()))
		Collapse("CheckCoherenceTexteLangue : err langue %d GetApp",l);
if(TailleBankMots(bkmots_fr)!=TailleBankMots(GetBkMot()))
		Collapse("CheckCoherenceTexteLangue : err langue %d GetBkMot",l);
if(TailleBankMots(scenme_fr)!=TailleBankMots(GetScenme()))
		Collapse("CheckCoherenceTexteLangue : err langue %d GetScenme",l);

langue=lbak;
}

/*******************************************************************
                        InitAndCheckTexts
********************************************************************/
void InitAndCheckTexts(void)
{
#if DEBUG>0

	CheckCoherenceTexteLangue(GB);
	CheckCoherenceTexteLangue(ES);
	
#endif
switch(langue)
	{
	case FR: HOMME='H';FEMME='F';OUI='O';break;
	case ES: HOMME='H';FEMME='M';OUI='S';break;
	default:
	case GB: HOMME='M';FEMME='F';OUI='Y';break;
	}
}
/*************************************************************************
                    CvtCar
Conversion d'un caractere special en caractere encode Windows selon la langue
**************************************************************************/
short CvtCar(short car)
{
switch(car)
        {
        case '%': car=0x8a;break; //ê 0x90
        case '&': car=0x82;break; //é 0x8e
        case '$': car=0x85;break; //à 0x88
        case '#': car=0x87;break; //ç 0x8d
        }
return(car);
}
/*************************************************************************
                    CvtCarLissage
Conversion d'un caractere special en caractere encode mac selon la langue
pour le lissage
**************************************************************************/
short CvtCarLissage(short car)
{
// Conversion
if(langue==FR)
	{
	switch(car)
		{
		case '%': car=0x90;break; //ê 0x90
		case '&': car=0x8E;break; //é 0x8e
		case '$': car=0x88;break; //à 0x88
		case '#': car=0x8d;break; //ç 0x8d
		}
	}
else if(langue==ES)
	{
	switch(car)
		{
		case '[': car=0xC0;break;  //Point d'interrogation inverse
		case ']': car=0xC1;break;   //Point d'excalamtion inverse
		case '%': car=0x97;break;  // O accent grave
		case '^': car=0x96;break;  // n tilde
		case '&': car=0x87;break;  // a accent grave
		case '|': car=0x92;break;  // i accent grave
		case '#': car=0x8E;break;  // e accent aigu
		case '_': car=0x9C;break;  // u accent grave
		}
	}
return(car);
}
//EOF