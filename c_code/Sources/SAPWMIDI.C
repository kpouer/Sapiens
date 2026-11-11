/*****************************************************************************
                            SAPWMIDI.C
            Gestion hardware de la midi sur W95
******************************************************************************/
#define WINDOWS_PLUS_ACAM 1
#include "sapiens.h"
#include <mmsystem.h>

typedef struct
    {
    HMIDIOUT hDriver;
    UINT numDriver;
    }MidiPort,*MidiPortPtr,**MidiPortHandle;

/* Handles sur les drivers MIDI */
static MidiPort MidiPortOut;
static Boolean DejaInitMidi;

/********************************************
          ErreurMidiW95
=> message, nø erreur,
   Booleen VRAI si erreur de sortie, FAUX si entree
********************************************/
static Boolean InErreurMidi;
static void ErreurMidiW95(char * mess, short err,Boolean IsOutput)
{
char chaine[255];
char messErr[128];

if(InErreurMidi) return;
InErreurMidi=VRAI;
if(IsOutput) midiOutGetErrorText(err,messErr,200);
else midiInGetErrorText(err,messErr,200);

sprintf(chaine,"%s :\n %s \n(erreur %d)",mess,messErr,err);
AcamErreur(chaine);

InErreurMidi=FAUX;
return;
}
/********************************************
          LiberePortsMidiManager
Liberation vectoris‚e … l'exit du port MIDI
********************************************/
static void LiberePortsMidiManager(void)
{
short err;
char chaine[255];

if(!DejaInitMidi) return;
if(MidiPortOut.hDriver)
   {
   err=midiOutReset(MidiPortOut.hDriver); /* On vide les buffers midi */
        if(err)
            {
            midiOutGetErrorText(err,chaine,200);
            AcamErreur(chaine);
            }
        err=midiOutClose(MidiPortOut.hDriver); /* On ferme la midi */
        if(err)
            {
            midiOutGetErrorText(err,chaine,200);
            AcamErreur(chaine);
            }
    MidiPortOut.hDriver=NULL;
    }
MidiPortOut.numDriver=NULL;
DejaInitMidi=FAUX;
return;
}
/******************************************************************************
                        EnvoiePaquetMidiW95
                Envoie un paquet sur la MIDI
=> 3 caracteres ou 2 + la valeur 9999 si deux caracteres dans la paquet
******************************************************************************/
void EnvoiePaquet(short car1,short car2,short car3)
{
short err;
DWORD param;

/* Midi non implementee */
if(MidiPortOut.hDriver==0) return;

if(car3<9999) param=car1+((long)car2<<8)+((long)car3<<16);
else param=car1+((long)car2<<8);

/* Send the message */
err=midiOutShortMsg(MidiPortOut.hDriver,param);
if(err)
    {
    /* On desactive la midi */
    LiberePortsMidiManager();
    ErreurMidiW95("EnvoiePaquet",err,VRAI);
    }
return;
}
/******************************************************************************
                        MidiInit
   L'application est initialis‚e, d‚finie comme un client,
******************************************************************************/
static Boolean IsMidiInit[2],IsBadInit[2];
Boolean MidiInit(short port)
{
short err;

if(IsBadInit[port]) return(FAUX);
if(IsMidiInit[port]) return(VRAI);
IsBadInit[port]=VRAI;
/* @@@@ Normalement, le nø est d‚fini dans la config */
MidiPortOut.numDriver=port;
err=midiOutOpen((LPHMIDIOUT)&(MidiPortOut.hDriver),/* Variable recevant le handle du periph. de sortie */
                        MidiPortOut.numDriver,         /* Id du p_riph_rique */
                        NULL,            /* Adresse de la fonction callback*/
                        NULL,
                        NULL);

if(err)
            {
            ErreurMidiW95("Initialisation de la MIDI impossible",err,VRAI);
            return(FAUX);
            }
atexit(LiberePortsMidiManager);
IsMidiInit[port]=VRAI;
IsBadInit[port]=FAUX;
return(VRAI);
}
/*EOF */
