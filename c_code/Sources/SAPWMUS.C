/*************************************************************************
                    Sapwmus
                (c) Dg le 12-1-1996
Musique sous windosw
***************************************************************************/
#include "sapiens.h"
#ifdef THINK_C
	long RestitutionSampleRate;
	short RestitutionNbBitsSample;
	Boolean RestitutionStereo;
	short RestitutionVolume;
	short RestitutionNbVoix;
	Boolean RestitutionQualite;
#endif
short quelle_musique=1;
Boolean IsBruitages=VRAI;
Boolean MusiqueEnCours;
#ifdef OLD_VERSION
extern unsigned char muse1[];
extern unsigned char muse2[];
extern unsigned short dvoie1[];
extern unsigned short dvoie2[];
extern unsigned short dvoie3[];
extern unsigned short dvoie21[];
extern unsigned short dvoie22[];
extern unsigned short dvoie23[];
static unsigned char * debmus;
static unsigned char seqnul[]={0xFF,0};
static short volume;
typedef struct
    {
    TMTask task;
    long MonA5;  /* Adresse du pointeur sur les globales */
    } MyTmTask;
static MyTmTask TacheJoueMusique;

typedef struct
    {
    short canal;
    short program;
    short coupure;
    short decal_note;
    } Instr;
static Instr ins1={1,33,0,0},/* Basse 34*/
             ins2={2,116,0,12},/* Taiko 117 Keyshift 12*/
             ins4={4,116,0,12},/* Taiko 117 Keyshift 12*/
             ins6={6,12,0,0},/* Marimba 13 */
             ins8={8,75,0,12},/* Flute 76, keyshift : 12*/

             ins21={2,7,0,0}, /* Clav 8*/
             ins22={3,115,0,0}, /* Batterie ??? 116 */
             ins24={4,37,0,0}, /* Basse slap 2 38*/
             ins25={5,119,0,0}, /* Batterie ??? 120 */
             ins27={7,117,0,0},  /* Batterie 118*/
             ins28={8,118,0,0},  /* Drum 119*/
             ins29={9,87,0,0},  /* 88*/
             ins2a={6,110,0,0}, /* Fiddle 111*/
             ins2b={11,113,0,0},/* 114 ??? */
             ins2c={12,32,0,0},/* Basse 33*/
             ins2d={13,30,0,12},/* Guitare 30 KS 12*/
             ins2e={14,28,0,0},/* Guitar 29 */
             ins2f={15,118,0,0}; /* Batterie 119*/

static Instr * tabins1[]={
        &ins1,&ins1,&ins2,&ins1,&ins4,&ins1,&ins6,&ins1,
        &ins8,&ins1,&ins1,&ins1,&ins1,&ins1,&ins1,&ins1
        };
static Instr * tabins2[]={
        &ins1,&ins21,&ins22,&ins21,&ins24,&ins25,&ins2c,&ins27,
        &ins28,&ins29,&ins2a,&ins2b,&ins2c,&ins2d,&ins2e,&ins2f
        };
static Instr * * tabins;
typedef struct s_voie
    {
    unsigned short * debvoie;    /* Debut de la voie en memoire */
    unsigned short * posvoie;    /* Position dans la voie */
    unsigned char * debseq;     /* Debut de la sequence en memoire */
    unsigned char * posseq;    /* Position dans la sequence */
    short nbrseq;   /* nbre repetition de la sequence */
    short base;     /* base de la frequence */
    long insdef;    /* Instrument par defaut */
    long freq;      /* Frequence en cours */
    long dephas;    /* Dephasage de la voie */
    long maxvibfreq;/* Delta freq max haut vibrato */
    short partie;   /* partie en cours */
    short note;     /* Note en cours */
    short time;     /* Nbre de 50 eme ecoules */
    short timerel;  /* Nbre de 50 eme avant relache */
    short maxtime;  /* Nbre de 50 eme max*/
    short effet;    /* Numero de l'effet en cours */
    short okeff;    /* Effet on/off */
    long freff;     /* frequence de l'effet */
    long delfreff;  /* Delta freq effet */
    long freffstop; /* freq a atteindre */
    short parameff; /* Parametre de l'effet */
    Instr* instr;      /* Instrument */
    long freqjoue;  /* freqeunce jou‚e */
    short canal_midi;/* canalmidi*/
    long dma_0page;
    long dma_delta;
    short dma_flag;
    long dma_add_freq;
    short ok_midi;
    short fini;     /* A Vrai si voie finie*/
    short delta_ins;    /* 0 pour muse 1,16 pour muse2*/
    short note_on;  /* Note lanc‚e sur midi */
    short kbd;  /* Decalage en note sur le synth‚ */
    } Voie;
static Voie voie1,voie2,voie3;
static TMTask task;
static TimerUPP UniversalDTJoueMusique;
/*************************************************************************
                lancenote
***************************************************************************/
static void lancenote(Voie * voie)
{
if(voie->note)
    EnvoiePaquet(0x90+voie->instr->canal-1,voie->note,64);
}
/*************************************************************************
                FixeVolumeSonore
***************************************************************************/
static void FixeVolumeSonore(void)
{
short i;

for(i=0;i<16;i++)
    EnvoiePaquet(0xB0+i,7,volume*8);
}
/*************************************************************************
                envoierelache
***************************************************************************/
static void envoierelache(Voie * voie)
{
}
/*************************************************************************
                arretvoie
***************************************************************************/
static void arretvoie(Voie * voie)
{
EnvoiePaquet(0x90+voie->instr->canal-1,voie->note,0);
}
/*************************************************************************
                JoueVoie
***************************************************************************/
static short JoueVoie(Voie * voie)
{
Instr * instr;
unsigned char car;
short cont;
unsigned char * p;

if(!voie->fini)
    {
    instr=voie->instr;
    voie->time++;
    if(voie->time>=voie->maxtime)
        {
        /* Autre note */
        arretvoie(voie);
        p=voie->posseq;
        cont=VRAI;
        do
            {
            /* Extraction de l'element suivant */
            car=*(p++);
            if(car>=128) /* Evenement */
                {
                if(car==255) /* Fin de sequence */
                    {
                    voie->effet=0;
                    if(--voie->nbrseq<=0)
                       {
                        car=*(voie->posvoie++);
                        if(car)
                            {
                            /* Nombre de repetition de la sequence
                             */
                            voie->nbrseq=car;
                            /* Position dans la musique */
                            voie->debseq=p=debmus+*(voie->posvoie++);
                            voie->base=*(voie->posvoie++);
                            voie->insdef=*(voie->posvoie++);
                            }
                        else
                            {
                            /* La voie est finie */
                            voie->fini=VRAI;
                            return(VRAI);
                            }
                        }
                    else
                        {
                        /* On boucle au debut de la sequence */
                        p=voie->debseq;
                        }
                    }
                else
                    {
                    /* Evt instrument effet */
                    car &=127;
                    if(car<16)
                        {
                        /* Changement instrument */
                        /* Instrument par defaut ? */
                        if(car==0)
                            car=voie->insdef;
                        voie->instr=instr=tabins[car];
                        /* Changement instrument sur la voie */
                        ;
                        }
                    else
                        {
                        /* Effet*/
                        car&=15;
                        if(car>=3)
                            voie->effet=car;
                        voie->parameff=*(p++);
                        switch(car)
                            {
                            case 9: volume=voie->parameff;
                                    FixeVolumeSonore();
                                    break;
                            case 10: volume+=(char)voie->parameff;
                                    FixeVolumeSonore();
                                    break;
                            case 12: voie->dephas=(long)voie->parameff<<4;
                                    break;
                            }
                        }
                    }
                /* Autre note 2 */
                }
            else
                {
                if(car>0) /* Ce n'est pas un silence */
                    {
                    car+=voie->base+instr->decal_note;
                    }
                voie->note=car;
                voie->partie=0;
                voie->time=0;
                voie->okeff=0;
                voie->maxvibfreq=0;
                voie->delfreff=0;
                voie->freff=0;
                voie->freffstop=-1;
                voie->maxtime=*(p++);
                voie->timerel=voie->maxtime-instr->coupure;
                voie->posseq=p;
                lancenote(voie);
                cont=FAUX;
                }
            }while(cont);
        voie->posseq=p;
        }
    else
        {
        voie->timerel--;
        if(voie->timerel<0)
            {
            voie->partie |=128;
            envoierelache(voie);
            if(voie->effet==6)
                voie->okeff |=2;
            }
        }
    }
return(!voie->fini);
}
/*************************************************************************
                Kmuse
Joue musique au 50 eme de seconde
***************************************************************************/
void Kmuse(void)
{
Boolean fini;

if(MusiqueEnCours)
        {
        fini=VRAI;
        if(JoueVoie(&voie1)) fini=FAUX;
        if(JoueVoie(&voie2)) fini=FAUX;
        if(JoueVoie(&voie3)) fini=FAUX;
        if(fini)
            {
            MusicInit(quelle_musique^3);
            }
        }
return;
}
/*************************************************************************
                Init_voie
***************************************************************************/
static void init_voie(Voie * voie,unsigned short * debvoie,short deltains)
{
voie->debvoie=debvoie;
voie->posvoie=debvoie;
voie->delta_ins=deltains;
voie->debseq=seqnul;
voie->posseq=seqnul;
voie->instr=&ins1;
voie->ok_midi=1;
voie->nbrseq=1;
voie->maxtime=0;
voie->fini=0;
return;
}
/*************************************************************************
                InitInstrument
***************************************************************************/
static void InitInstrument(Instr * instr)
{
EnvoiePaquet(0xC0+instr->canal-1,instr->program,9999);
return;
}
#ifdef MAC
#if (defined (__MWERKS__) && ! defined(powerc))

        void PushReg(void)  = { 0x48E7,0xFFFC };
        void PopReg(void)   = { 0x4CDF,0x3FFF };
#else

#endif
/*********************************************************************
                           VectJoueMusiqueSapiens
Vecteur 200 eme de traitement
********************************************************************/
#ifdef powerc
static pascal void VectJoueMusiqueSapiens(MyTmTask * tache)
{
PrimeTime((QElemPtr)tache,1000L/50L);
Kmuse();
return;
}
#else
static pascal void VectJoueMusiqueSapiens(/*)*/
                #ifdef __MWERKS__
                        MyTmTask * tache:__A1
                #else
                        void
                #endif
                )
{
#ifdef __MWERKS__
        PushReg();
#else
MyTmTask * tache;
asm
        {
        movem.l a0-a5/d0-d7,-(a7)
        move.l a1,tache
        }
#endif
SetA5(tache->MonA5);
PrimeTime((QElemPtr)tache,1000L/50L);
Kmuse();

#ifdef __MWERKS__
        PopReg();
#else
asm
        {
        movem.l (a7)+,a0-a5/d0-d7
        }
#endif
return;
}
#endif
#endif
/*************************************************************************
                LibereTask
***************************************************************************/
static void LibereTask(void)
{
if(TacheJoueMusique.task.tmAddr)
        {
        RmvTime((QElemPtr)&TacheJoueMusique);
        TacheJoueMusique.task.tmAddr=NULL;
        }
return;
}
/*************************************************************************
                InstallTask
***************************************************************************/
static void InstallTask(void)
{
if(TacheJoueMusique.task.tmAddr==NULL)
        {
        #ifdef MAC
                if(UniversalDTJoueMusique==NULL)  UniversalDTJoueMusique=NewTimerProc(VectJoueMusiqueSapiens);
                TacheJoueMusique.task.tmAddr=(TimerUPP)UniversalDTJoueMusique;
                TacheJoueMusique.MonA5=SetCurrentA5();
        #else
                TacheJoueMusique.task.tmAddr=(Register68kProcPtr)Kmuse;
        #endif
        InsXTime((QElemPtr)&TacheJoueMusique);
        }
PrimeTime((QElemPtr)&TacheJoueMusique,1000L/50L);
return;
}
/*******************************************************************
                        SortieMusiqueFichier
********************************************************************/
void SortieMusiqueFichier(short quelle)
{
Boolean fini;
char chaine[255];
short i;

sprintf(chaine,"MUSIC%d.MID",quelle);
InitMusicFichier(chaine);
quelle_musique=quelle;
if(quelle==1)
            {
            debmus=muse1;
            tabins=tabins1;
            init_voie(&voie1,dvoie1,0);
            init_voie(&voie2,dvoie2,0);
            init_voie(&voie3,dvoie3,0);
            }
        else
            {
            debmus=muse2;
            tabins=tabins2;
            init_voie(&voie1,dvoie21,16);
            init_voie(&voie2,dvoie22,16);
            init_voie(&voie3,dvoie23,16);
            }
for(i=0;i<16;i++)
            InitInstrument(tabins[i]);
 do
        {
        fini=VRAI;
        if(JoueVoie(&voie1)) fini=FAUX;
        if(JoueVoie(&voie2)) fini=FAUX;
        if(JoueVoie(&voie3)) fini=FAUX;
        IncMusicFichier();
        } while(!fini);

TerminateMusicFichier();
return;
}
#endif

/*;------------------------------------------
;-      BRUIT : Positionne DMA BRUIT      -
;------------------------------------------
;-> Num de bruit de 0 a 5
*/
static Handle TabSons[25];
static SndChannelPtr canalSnd;
void bruit(short num)
{

if(IsBruitages)
        {
		if(!canalSnd)
			SndNewChannel(&canalSnd,sampledSynth,0x20+initMono,NULL);
        if(TabSons[num]==NULL)
                TabSons[num]=GetResource('snd ',num+1000);
        if(TabSons[num])
                {
				if(canalSnd)
	                SndPlay(canalSnd,(SndListHandle)TabSons[num],VRAI);
				else
					SndPlay(NULL,(SndListHandle)TabSons[num],FAUX);
                }
		#ifdef PC
		if(num==11 || num==12 || num==6) delai(30);
		#endif
        }
else
        delai(10);

return;

}
void attends_bruit(void)
{
}
void quel_synthe(short n)
{
}
void reset_synthe(void)
{
}
/*************************************************************************
                ExitMuse
Lance la musique d‚sir‚e : 1 musique 1, 2 musique 2
***************************************************************************/
static Boolean IsExitMuse;
static void ExitMuse(void)
{
ArreteMusique(true);
return;
}
/*************************************************************************
                MusicInit
Lance la musique d‚sir‚e : 1 musique 1, 2 musique 2
***************************************************************************/
void MusicInit(short quelle)
{
char * nom;

if(okmuse==NO_MUSIQUE) 
	{
	ArreteMusique(true);
	MusiqueEnCours=FAUX;
	return;
	}
SetCursor(*GetCursor(watchCursor));
// S300 l'arret de la musique se fait dans SapiensExit
//if(!IsExitMuse) atexit(ExitMuse);  
//IsExitMuse=VRAI;
quelle_musique=quelle;
PushPath();
SetPathInData("musics");
if(quelle==1) nom="Sapiens1.mux";
else nom="Sapiens2.mux";
JoueMusiqueHarmony(nom,NULL,"",0,"DIGIT_QUALITE",-1,0,0,false);
CarboniseSetCursorArrow();
PopPath();

#ifdef OLD_VERSION
else
        {
        if(quelle==1)
                    {
                    debmus=muse1;
                    tabins=tabins1;
                    init_voie(&voie1,dvoie1,0);
                    init_voie(&voie2,dvoie2,0);
                    init_voie(&voie3,dvoie3,0);
                    }
                else
                    {
                    debmus=muse2;
                    tabins=tabins2;
                    init_voie(&voie1,dvoie21,16);
                    init_voie(&voie2,dvoie22,16);
                    init_voie(&voie3,dvoie23,16);
                    }
        for(i=0;i<16;i++)
                    InitInstrument(tabins[i]);
        if(task.tmAddr==0)
                InstallTask();
        }
#endif
return;
}
/*************************************************************************
                CheckChangeMusique
En mode player sur librairie on teste la fin de la musique
***************************************************************************/
void CheckChangeMusique(void)
{
 if(IsFinDeMusique())
                {
                if(quelle_musique==1) MusicInit(2);
                else if(quelle_musique==2) MusicInit(1);
                }
}
/* EOF */
