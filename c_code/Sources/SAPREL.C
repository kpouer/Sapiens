/***************************************************************************
                     Module #2:Module relationnel
                       (c) god Le 7 Novembre 1986



Scenar = 1 - Viande pour la tribu

Scenar = 2 - Remede des yeux malins (Potion de  vie objet #42)

Scenar = 3 - Hache du chef des Bracs de roc (Hache d'ivoire #43)

Scenar = 4 - Baton sacr‚ des Hyenes folles (Baton sacr‚  #44)

Scenar = 5 - Combat ultime

Scenar = 6 - Bravo tu as gagn‚ fin de partie
Scenar = 7 - Tu as perdu fin de partie
    sceveu 9 : Hyenes folles vaincues
    sceveu 7: Le joueur est mort
***************************************************************************/
#include "sapiens.h"
short LastObsv;
/***********************************************************************
                        MOTLST:Affiche
***********************************************************************/
static void motlst( char * debut)
     {
     unsigned char * p=(unsigned char *)debut;
     while (*p)  affcarmess(*(p++));
     /* Transfere la bande sur ecran reel */
     TfrBadAss();
     return;
     }
/*************************************************************************
                    TailleCarMess
**************************************************************************/
static void TailleChaine(char * debut,short * tx,short * ty);
static void TailleCarMess(char **debut,short * taillex,short * tailley)
{
char * p=*debut;
char * chaine;
short car;
short tx,ty;

car=*(unsigned char *)p;
*((*debut)++);
if(car>128)
    {
	chaine=GetStrBkMot(car);
    tx=ty=0;
    TailleChaine(chaine,&tx,&ty);
    *taillex+=tx;
    *tailley+=ty;
    }
else
    *taillex+=TailleCar(car,10);
return;
}
/*************************************************************************
                    TailleChaine
Affiche un caractere sur le port en entr‚e
=> Chaine a traiter
@= taille maximale en colonne x, nbre de lignes t
**************************************************************************/
static void TailleChaine(char * debut,short * tx,short * ty)
{
short x=0,taillex=0,y=0;

*ty=0;
while(*debut)
    {
    if(*debut=='\r')
        {
        *ty+=1;
        x=0;
        debut++;
        }
    else
        {
        TailleCarMess(&debut,&x,&y);
        if(x>taillex) taillex=x;
        *ty+=y;
        }
    }
*tx=taillex;
return;
}
/***************************************************************************
                Sappause: effectue une pause ou sort si touche au clavier
****************************************************************************
=> dur‚e de la pause
*/
static void Sappause(unsigned short duree)
     {

     while(bouton) vq_mouse(&bouton,&sposx,&sposy);
     dsposx=sposx;
     dsposy=sposy;
     while (duree && !bouton && sposx==dsposx && sposy==dsposy)
          {
          delai(1);
          duree--;
          vq_mouse(&bouton,&sposx,&sposy);
          }
     return;
     }
/************************************************************************
               EFBADI
***********************************************************************/
void efbadi(void)
{
Gotoxy(3,15);
efbadass();
return;
}
/************************************************************************
               scrollBADI
***********************************************************************/
void scrollbadi(void)
{
Gotoxy(3,15);
scrollbadass();
return;
}
/***********************************************************************
                        CLRBADI:efface bande si inutilis‚e
***********************************************************************/
void clrbadi(void)
{
if (flagbadi)
          {
          efbadi();
          flagbadi=0;
          }
return;
}
/***********************************************************************
                        SCEVEU:
Affiche le message correspondant … la phase du scenario
Le message s'affiche sur l'aire 3d
***********************************************************************/
void sceveu(unsigned short quoi)
 {
 unsigned char car;
  char * p;
 short i;
    Rect rsourc,rdest;
char ** scenme=GetScenme();

dervue=VUE_AUCUNE;
if (quoi == 6)
    {
    gemouvre(6,32,1,2,1,8,8,38);
    }
 else            gemouvre(6,32,1,2,1,8,5,38);
 /* on efface la zone */
 effvirt(0x007);

 Gotoxy(12,1);
 i=1;
 p=scenme[quoi-1];
 car=*(p++);
 /* On affiche sur debecr */
 while(car)
      {
      car2(car);
      do
           {
           car=*(p++);
           if (car==13) Gotoxy(12,linaff+1);
           } while(car==13);
      }
 if(quoi==6)
    {
    for(i=0;i<3;i++)
        {
        bruit(6);
        attends_bruit();
        }
    }
 /* Transfert debecr vers reel */
 TfrEcr(NULL);
 if(quoi==6) /* Bravo tu as gagn‚ */
    {
	SetRect(&rdest,0*ECHELLE,0*ECHELLE,255*ECHELLE,100*ECHELLE);
	SetRect(&rsourc,32*ECHELLE,1*ECHELLE,(32+255)*ECHELLE,(1+100)*ECHELLE);
	CopyOffscreen(debecr,Ecran1,&rsourc,&rdest);

    delai(15L*60L);
    Final();
    }
 return;
 }
/***********************************************************************
                        SceveuBulle:
Affiche le message correspondant … la phase du scenario
Le message s'affiche sur l'aire 3d
***********************************************************************/
void SceveuBulle(unsigned short quoi)
{
char ** scenme=GetScenme();

effaceBulles();
bulle(scenme[quoi],1);
return;
}
/***********************************************************************
                        GAINSBOUR:D‚gradation Physique
***********************************************************************/
void gainsbour(void)
     {
     bless -=6-(6*resistance/256);/* de -1 … -6*/
     if (bless>251)
          {
          if (mort) /*pas mort encore*/
               {
               mort--;
               bless=20;
               }
          else
               {
               presence=0;
               fdialo=0;
               }
          }
     }
/***********************************************************************
                        PASTMP:Le temps Passe
***********************************************************************/
void pastmp(void)
     {
     register short i;
     i=liste1.taille;
     while (i)
          {
          i--;
          /*la viande se corromps*/
          if (liste1.elements[i]>=37 && liste1.elements[i]<41)
          liste1.elements[i]++;
          }
     return;
     }
/***********************************************************************
                        TFRENE:
***********************************************************************/
void tfrene(void)
     {
     if (presence<5 && presence)
          {
          if (fpress)
               {
                enetoi[tribu-1]=enerv;
                if ( chef || tribu !=1) tagress[tribu-1]=agress;
               }
          }
     return;
     }
/**************************************************************************
                COMMENT:affiche un commentaire
*************************************************************************
=> rang du commentaire
*/
void comment(unsigned short rang)
 {
  unsigned char * p;
char ** mescon=GetMesCon();

 if (!flefba)
      {
      flefba++;
      scrollbadi();
      }
 flagbadi=1;
 p=(unsigned char *) mescon[rang-1];
 while (*p)
      affcarmess((short)(*p++));
 TfrBadAss();
 return;
 }
/**************************************************************************
              EffaceBulles
*************************************************************************/
#define X_BULLE_DROITE (640-64-16)
#define X_BULLE_GAUCHE (80)
#define Y_BULLE (58)
#define TAILLE_QUEUE 20
static Rect rBulleJoueur,rBulleAdversaire;
void effaceBulles(void)
{
tfrvirt();
if(!EmptyRect(&rBulleJoueur))
    CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&rBulleJoueur,&rBulleJoueur);
SetRect(&rBulleJoueur,0,0,0,0);
if(!EmptyRect(&rBulleAdversaire))
    CopyOffscreen(debecr,CarboniseGetWindowPort(mainWindow),&rBulleAdversaire,&rBulleAdversaire);
SetRect(&rBulleAdversaire,0,0,0,0);
return;
}
/**************************************************************************
                Bulle:affiche une bulle avec un texte
=> message a afficher, qui le dit (0)== joueur (1)== adversaire
*************************************************************************/
void bulle(Ptr debut,short qui)
{
Rect  r,rbase;
RgnHandle rgnClip,rgn;
short x,y,tx,ty;
unsigned char * p=(unsigned char *)debut;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
/* Calcul de la taille de la chaine en pixel */
TailleChaine((char*)p,&tx,&ty);
if(tx<64) tx=64;
if(qui^sap1.sens)
        SetRect(&r,X_BULLE_GAUCHE,Y_BULLE-(ty+2)*12,X_BULLE_GAUCHE+tx+16,Y_BULLE);
else
        SetRect(&r,X_BULLE_DROITE-tx-16,Y_BULLE-(ty+2)*12,X_BULLE_DROITE,Y_BULLE);
r.bottom+=TAILLE_QUEUE;
rbase=r;
/* On efface les bulles en cours */
if(qui==0) /* C'est le joueur */
    {
    effaceBulles();
    rBulleJoueur=r;
    }
else
    {
    /* C'est un adversaire ... */
    /* deja un message dit ??? */
    if(!EmptyRect(&rBulleAdversaire))
        {
        OffsetRect(&r,0,rBulleAdversaire.bottom-rBulleAdversaire.top-TAILLE_QUEUE-4);
        UnionRect(&rBulleAdversaire,&r,&rBulleAdversaire);
        rbase=rBulleAdversaire;
        }
    else
        rBulleAdversaire=r;
    }
r.bottom-=TAILLE_QUEUE;
rgn=NewRgn();
rgnClip=NewRgn();
OpenRgn();
FrameRoundRect(&r,16,16);
/* Bulle a gauche */
if(qui^sap1.sens)
    {
    x=r.left+48;
    y=r.bottom;
    DoMoveTo(x,y);
    LineTo(x-TAILLE_QUEUE,y+TAILLE_QUEUE);
    LineTo(x-8,y);
    LineTo(x,y);
    }
else
    {
    x=r.right-48;
    y=r.bottom;
    DoMoveTo(x,y);
    LineTo(x+TAILLE_QUEUE,y+TAILLE_QUEUE);
    LineTo(x+8,y);
    LineTo(x,y);
    }
CloseRgn(rgn);
BackColor(whiteColor);
EraseRgn(rgn);
DoPenSize(1,1);
FrameRgn(rgn);
GetClip(rgnClip);
SetClip(rgn);
x=r.left+8;
y=r.top+6;
GotoxyPixel(x,y);
while(*p==' ') p++;
while (*p)
    {
    if(*p=='\r')
        {
        y+=14;
        GotoxyPixel(x,y);
        do
            p++;
        while(*p==' ');
        }
    else
        affcar10(*(p++));
    }
SetClip(rgnClip);
DisposeRgn(rgn);
DisposeRgn(rgnClip);
AcamSetPortOffscreen(gbak);
TfrEcr(&rbase);
DoPenSize(1,1);
return;
}
/***********************************************************************
                        CHRONO:Effectue Chronologie
Heure varie de [0 … 1023]
Matin :              0-90
Apres midi :         426-512
Nuit :               512-602
Fin de nuit          938-1023

***********************************************************************/
/*duree du jour 4*90=360 heures*/
/*quoi 0:normal,1:ni param en baisse ni message,2 baisse mais pas mess*/
void chrono(short quoi)
{
 unsigned char avemess;

 if (minute)
      {
      minute--;
      return;
      }
 minute=5;
 heure++;
 if ((heure & 255)==90) heure +=336;
 heure &=0x03ff;

 if ( ! (heure & 0xFF) ) pastmp();
 /*mise en place des evts couleurs*/
while( heure>=tabheur[pos_evt].h)
    {
    /* dois t'on modifier le degrad couleur */
    if( tabheur[pos_evt].q)
            {
            /* Changement du degrade du ciel avec les huits couleurs ST en entr‚e*/
            ciel_degr=&tabheur[pos_evt].pal[0];
            FixeDegradesFonctionHeure(&tabheur[pos_evt]);
            }
    else
            {
            /* On fixe la pal haute */
            palhaute=&tabheur[pos_evt].pal[0];
            }
    if (++pos_evt >= NBREVTHEURE)
        {
        pos_evt=0;
        heure=0;
        }
    }
if (heure==0)
       {
       flefba=0;
       comment(114);/* Un nouveau jour ...*/
       jour_act++;
       }
/* Position du meteore */
 if (heure & 0x100)
      {
      metpos=(heure & 0xFF)-180;
      angsol=256;
      }
 else
      {
      metpos=75-(heure & 0xFF);
      angsol=768;
      }
 if (quoi==1) return;
 finjou =1;/*deperissement de la viande*/
 faim-=2;
 avemess=0;
 if (faim>253)
      {
      faim=5;
      gainsbour();
      avemess=93;
      }
 soif-=4;
 if (soif>251)
      {
      soif=5;
      gainsbour();
      avemess=94;
      }
 fatig-=2;
 if (fatig>253)
      {
      fatig=5;
      gainsbour();
      avemess=95;
      }
 if (avemess && !quoi)
      {
      efbadi();
      comment(avemess);
      }
 return;
 }
/**************************************************************************
                CAUSE: Sort … l'ecran une phrase correspondant
                             … un contexte donn‚
*************************************************************************
=> Contexte de 1 … ....
*/
static void cause(unsigned short context,char * messAdd)
{
short rang;
char chaine[255];
char ** mescon=GetMesCon();

/*y cause ti ?*/
if (fdialo) /*y cause */
    {
    rang=(context-1)*4+(Rnd() & 3);
    strcpy(chaine,mescon[rang]);
    if(messAdd) strcat(chaine,messAdd);
    bulle(chaine,1);/*affiche le commentaire*/
    }
return;
}
/***********************************************************************
                        FUITE:
***********************************************************************/
void fuite(void)
     {
     flefba=0;
     if (agress<245) agress+=10; /*l'agressivite augmente*/
     comment(80);               /*il s'enfuit*/
     quitte();
     }
/***********************************************************************
                INTRSQ:recherche valeur d'un objet
***********************************************************************
=> Objet a rechercher.La recherche se fait sur refer1
<= valeur de l'objet f(presilection)
*/
unsigned short intrsq(unsigned short objet)
     {
     unsigned char valeur;
	 struct reference* refer1=GetRefer1();

     valeur=(refer1[objet-1].param1) & 0xF; /*moins un pour option base*/
     if (objet == predil) valeur=valeur*2;
     return (valeur);
     }
/***************************************************************************
                 RAPIAS:receherche objet interessant ds liste
**************************************************************************
=>tribu
<= valeur et rang de la valeur
*/
unsigned short rapias(void)
     {
     unsigned char max=0,objet=0,valeur;
     register short i=0;
     while (i<liste1.taille)
          {
          valeur=intrsq(liste1.elements[i]);/*prends valeur*/
          if (valeur>max)
               {
               max=valeur;
               objet=liste1.elements[i];
               }
          i++;
          }
     return(objet);/*option base*/
     }
/***********************************************************************
                EXCITE:Augmente enervement
************************************************************************/
void excite(unsigned short valeur)
     {
     unsigned short tfr;
     tfr=enerv+valeur;
     if (enerv <= tfr) enerv=tfr;
     else enerv=65535L;

     if (flefba)
          {
          /*flefba=0;
          Sappause(2000);*/
          }
     if (enerv<51*256)/*tres calme*/
          {
          if (fpress) cause(7,NULL);
          else        cause(2,NULL);
          return;
          }
     if (enerv<102*256) /*peu enerve*/
          {
          cause(8,NULL);
          return;
          }
     if (Rnd()<enerv/512)/*assez enerve*/
          {
          inimen(0);
          flcomb=1;/*combat lanc‚*/
          return;
          }
     if (enerv<153*256)
          {
          cause(9,NULL);
          return;
          }
     if (enerv<204*256)
          {
          cause(10,NULL);
          return;
          }
     cause(11,NULL);
     return;
     }
/***************************************************************************/
/*              MODREL:Initialise l'enervement si pas DONNE ou DIRE        */
/***************************************************************************
=> Les deux parametres correspondants au choix
*/
void modrel(void)
     {
     unsigned short tfr;
     if (tlpos[0]==5 /*dire*/  || tlpos[0]==6 /*donner*/ || !fdialo)  return;
     /*Autre action alors qu'adversaire presentt => enervement*/
     tfr=enerv+100;
     if (enerv<tfr) enerv=tfr;
     if (flefba)
          {
		  /*attends une duree ou une touche*/
          /*Sappause (100);
          flefba=0;*/
          }
     if (enerv<51*256) /*pas tres enerve*/
          {
          if (fpress) cause(7,NULL);
          else cause(2,NULL);
          return;
          }
     if (enerv<102*256) cause(8,NULL);/*moyennement enerve*/
     if (Rnd()>enerv/512)
          {
          if (enerv<153*256) cause(9,NULL);
          else      {
               if (enerv<204*256) cause(10,NULL);
               else cause(11,NULL);
               }
          }
     return;
     }
/***********************************************************************
                      PRENDS:On prends l'objet de rang Rega
                      dans la liste des objets du sap1 et on le transfere
                      dans la liste 10
***********************************************************************/
void prends(void)
 {
 unsigned char erreur;
 erreur=enlevl(&liste1,&list10,rega);
 if (erreur==1 && lpos>1) lpos--;/*elt est le dernier*/
 inimen(s_menu);
 return;
 }
/**************************************************************************
                TRDONN:donne
unsigned short rangb;simule registre B [0,eltmax[
**************************************************************************/
void trdonn(unsigned short rangb)
     {
     unsigned char objet,tfr;
     unsigned short valeur;
char ** mescon=GetMesCon();


     if ( !fdialo ) return;/*pas de dialogue alors rien*/
     objet=liste1.elements[rangb];  /*on prendsl'objet ds la liste*/
     flefba=0;
     if (chef && tribu==1) /*on donne … son chef*/
          {
          if (scenar !=1)
               {
               /* on compare l'objet … l'objet desir‚*/
               if (objet != scenar+40)/*objet du scenario*/
                    {
                    SceveuBulle(scenar-1);/*on raffiche l'objet desire*/
                    return;
                    }
               }
          else /*scenario = viande*/
               {
               if (objet <35 || objet>39)/*viande pourrie ou autre*/
                    {
                    SceveuBulle(scenar-1);
                    return;
                    }
               scenpr=scenpr-(40-objet);/*on enleve au total*/
               if (scenpr > 0) /* pas fini !!!*/
                    {
                    prends();       /* enleve viande*/
                    list10.taille=0;/* mise … zero taille*/
                    effaceBulles();
                    cause(6,NULL);       /* donne encore*/
                    return;
                    }
               }
          if(!IsEnreg())
            {
            SceveuBulle(9);
            return;
            }
          prends();/*on prends la chose*/
          /* Remerciement du chef */
            effaceBulles();
            switch(scenar)
                {
                case 1: bulle(mescon[82-1],1);break;
                case 2: bulle(mescon[83-1],1);break;
                case 3: bulle(mescon[84-1],1);break;
                case 4: bulle(mescon[84-1],1);break;
                }
         scenar++;
          if (scenar == 5 )
               {/*combat ultime*/
               flcomb=1;
               SceveuBulle(scenar-1);
               /* Delay de 5 seconde */
               delai(5L*60L);
               return;
               }
          if (scenar==3)
               {/*bras de roc 'achement enerves et on doit lui voler sa hache*/
               enetoi[2]=60000L;
               }
          /*comment(80 + scenar);*/ /*affiche commentaire selon scenar*/
          return;/*fin partie donne->scenario*/
          }
     /*-----------------------*/
     /* on donne hors scenario  */
     /*-----------------------*/

     /*cherche valeur de l'objet*/
    effaceBulles();
     valeur=enerv-512*intrsq(liste1.elements[rega]);
     /*compense l'enervement*/
     if (valeur>enerv)
          {/*bien compense*/
          valeur=0;
          }
     if (!fpress)/*pas presente => cado */
          {
          if (list10.taille <= list10.maxtaille)
               {
               prends();/*y a de la place : on prends*/
               if (enerv <102*256)
                    {
                    cause(1,NULL);/*merci*/
                    cause(12,NULL);/*je suis ...*/
                    }
               if (agress>0) agress--;
               /*l'agressivite descend*/
               /*Sappause(2000);*/
               enerv=valeur;/*l'enervement diminue*/
               excite(0);/*on affiche l'etat d'enerv*/
               }
          return;
          }
     /*---------*/
     /* echange */
     /*---------*/
     if (enerv<102*256)
          {
          valeur=intrsq(liste1.elements[rega]);
          if ( chef && (scenar==tribu))
               {
               if (liste1.elements[rega] != predil) valeur=0;
               else                                 valeur=200;
               }
          if (valeur>vlchan)/*valeur superieure*/
               { /*ok il prends*/
               vlchan=valeur;
               cause(4,NULL);/*march‚ conclu*/
               if (echan==1)/*protection*/
                    {
                    enetoi[tribu-1]=0; /*enerv =0*/
                    echan=25;          /*onguent*/
                    }
               /*on permute les deux*/
               else
                    {
                    tfr=liste1.elements[rangb];
                    liste1.elements[rangb]=echan;
                    echan=tfr;
                    inimen(s_menu);
                    }
               }
          else /*insuffisant*/
               {
               cause(3,NULL); /*c'est pas assez glandu !!!*/
               excite(50);
               }
          }/*si donne et pas trop enerve*/
     else     /*enerv<e*/
          {
          if (list10.taille<list10.maxtaille)
               {
               /*il ya de la place*/
               prends();
               enerv=valeur;
               cause(5,NULL); /*merci du cado*/
               /*..… suivre...*/
               }
          }
     return;
     }
/***********************************************************************
                IDIALO:Initialise l'adversaire en fn de la presence
***********************************************************************/
void idialo(void)
     {
     register short i;
     unsigned char pos;
     fdialo=flcomb=fpress=flgcharism=dquest=0;
     if (presence==0) return;/*si personne */
     if (presence>4) /*bestiole*/
          {
          echan=0;
          flcomb++;/*on se bat*/
          return;
          }
     fdialo++; /*on discute*/
     enerv =  tenerv[tribu-1];
     agress = tagress[tribu-1];
     predil=tabprf[tribu-1];

     /*objet … echanger*/
     echan=tobje [tribu-1][Rnd() & 3];

     /*objet portes*/
     for ( i=0 ; i<2 ; i++)      list10.elements[i]=tobje[tribu-1][Rnd() & 3];
     list10.taille=2;
     //bkmots[3][1]=(tribu+0x85);/*tribu*/
	 nomtribu[1]=tribu+0x85;
	 nomtribu[2]=0;
	 
     if (chef)
          {/*c'est un chef*/
	  	    nomtribu[0]=0x85;//Chef
		  	nomadversaire[0]=(tribu+0x89),
		  	nomadversaire[1]=0;
       //   bkmots[2][0]=(tribu+0x89);/*nom du chef*/
         // bkmots[2][1]=0;/*fin*/
        //  bkmots[3][0]=0x85;/*'chef'*/
          if (tribu==1)
            {
            fpress=1;
            enerv=enetoi[tribu-1];
            }
          if (scenar==tribu && tribu!=4)
               {
               echan=tribu+40;/*objet vol‚ par la tribu*/
               }
          else             echan=1;/*ma protection*/
          }
     else
          {
          //bkmots[3][0]=32 ;
		   nomtribu[0]=' ';
          for (i=0;i<6;)
               {/*determine le nom*/
               pos=48*Rnd()/256;
               nomadversaire[i++]=maced[pos++];
               nomadversaire[i++]=maced[pos++];
			   nomadversaire[i++]=maced[pos++];
               }
			nomadversaire[i]=0;
          }
     /*tire nom secondaire*/
     for (i=0;i<6;)
          {/*determine le nom*/
          pos=48*Rnd()/256;
          nomsecondaire[i++]=maced[pos++];
          nomsecondaire[i++]=maced[pos++];
          nomsecondaire[i++]=maced[pos++];
          }
	nomsecondaire[i]=0;
     vlchan=intrsq(echan);
     smenu[1]=111;/* idee d'yves */

     inimen(0);
     return;
     }
/***********************************************************************
                        QUITTE:On lui fait peur :il s'enfuit
***********************************************************************/
void quitte(void)
     {
     tfrene();
     flcomb=0;
     presence=0;
     idialo();
     present[1]=0;
     present[3]=0;
     prm(&sap2,0);
     prm(&sap4,0);
     /* inits souris */
     dsposx=sposx;
     dsposy=sposy;
     reset_mouse();
     if (dervue ==VUE_LATERALE) laterb(FAUX);
     else            latera();
     generique(VRAI);
     inimen(0);
     return;
     }
/***********************************************************************
                        CREECOP:Cr‚e un copain
***********************************************************************/
void creecop(unsigned short qui)
     {
     /*c'est ds la poche*/
     /*on recopie sap2 sur sap3*/
     BlockMove(sap2.cross,sap3.cross,8);/*parametres*/
     if (presence<5)   strcpy(nomcopain,nomadversaire); /*nom*/
     else
         {
         nomcopain[0]=(128+45); /*Ton ami*/
         nomcopain[1]=0;
         }
     prm(&sap3,qui);
     sap3.colonne=sap2.colonne;
     sap3.sens=sap2.sens;
     sap3.coeur=sap2.coeur;
     sap3.cptrsouffle=1;
     prm(&sap2,0);
     if (dervue !=VUE_PAYSAGE) /* on est en vue laterale*/
          {
          presence=0;
          trobsv(1);
          }
     echan=0;
     (peuple[2]->possession)->taille=0;
     generique(VRAI);
     return;
     }
/***********************************************************************
                        BONNECHASS
***********************************************************************/
void bonnechass(void)
     {
char ** mescon=GetMesCon();
     if (enerv>102*256) /*‚nerv‚!!!*/
          {
          excite(300);
          }
     else
          {
          if ( ! fpress) /*pas pr‚sent‚*/
               {
               cause(2,NULL);/*qui est tu ???*/
               }
          else
               { /*il s'en va*/
			    /*bonne chasse*/
               bulle(mescon[73-1],1);
               quitte();
               inimen(0);
               effaceBulles();
               }
          }
     return;
     }
/***********************************************************************
                        VIENSAVEC
***********************************************************************/
void viensavec(void)
     {
char ** mescon=GetMesCon();
     if ( fpress && presence && presence<5 &&
     present[2]==0 && present[3]==0 && (!chef) && !flgcharism)
          {
          if ( (charisme*Rnd() > enerv) &&  Rnd()<70)
               {
               creecop(1);
               viecopain=3;
               sap3.vie=200;
			   /*je suis avec toi*/
               bulle(mescon[100-1],1);
               flagbadi=0;
               }
          else
          excite(1500);
          }
     else
          {
		  /*je ne suis pas celle que vous croyez*/
          bulle(mescon[99-1],1);
          excite(2000);
          }
     flgcharism=1;
     return;
     }
/***********************************************************************
                        TUVEUQUOI
***********************************************************************/
void tuveuquoi(void)
     {
     unsigned char valeur;
	      struct reference* refer1=GetRefer1();
char ** mescon=GetMesCon();
char ** scenme=GetScenme();

     if (enerv>102*256)
            cause(13,NULL);/*ta vie !!*/
     else
          {
          if (chef && tribu==1)
               {
               dquest=0;
               bulle(scenme[scenar-1],1);
               }
          else
               {
               if ( (tribu==scenar) && (chef) )
                    {
                    cause(14,(Ptr)&(refer1[predil-1].nomref[0]));
                    return;
                    }
               valeur=intrsq(rapias());
               if (valeur>vlchan)
                    {
                    cause(14,&(refer1[rapias()-1].nomref[0]));/*je veux*/
                    }
               else
                    {
            if (chef)
                {
                cause(14,&(refer1[predil-1].nomref[0]));
                }
            else
                {
				/*rien !!!*/
                bulle(mescon[66-1],1);
                    excite(0);
                    }
                 }
               }
          }
     return;
     }
/***********************************************************************
                        TRDIRE:C‚lakonkose
***********************************************************************
=> DIQuoi : Le num‚ro de la phrase ds la liste des phrases
*/
void trdire (unsigned short diquoi)
     {
     short tfr;
     unsigned short enerv2;
	       struct reference* refer1=GetRefer1();
char ** mescon=GetMesCon();

     if (! fdialo)  return;/*si pas de dialogue alors rien*/
     flefba=1;/* pas d'effacement au prochain*/

     if (diquoi==dquest)
          {
          excite(1000);
          return;
          }
     dquest=diquoi;/*derniere question*/

     if (diquoi==5) /*je suis tedehur*/
          {
          if (!fpress)/*pas encore presente*/
               {
               fpress++;/* on est pr‚sent‚*/
               /*calcul de la reponse en fn de l'enervement*/
               tfr=enerv-tenerv[tribu-1];
               enerv=enetoi[tribu-1];
               enerv2=enerv+tfr;
               if ((tfr>0) && (enerv2<enerv)) enerv2=65535L;
               if ((tfr<0) && (enerv2>enerv)) enerv2=0;
               enerv=enerv2;
               if (enerv>0x3400)
                    {
					/*c'est donc toi*/
                    bulle(mescon[72-1],1);
                    excite(0);
                    }
               else
                    {
					/*bonjour*/
                    bulle(mescon[71-1],1);
                    }
               }
          else
               { /*deja present‚*/
               excite(600);/*on s'enerve*/
               }
          return;
          }

     if (diquoi==6)
         {
          bonnechass();
          return;
          }
     if (diquoi==7 || diquoi==8)   /*viens avec moi*/
          {
          viensavec();
          return;
          }

     /*la phrase n'est ni je suis ni au revoir */
     if (stock1==0)
          {
          if (diquoi==1)/*qui est tu ?*/
               {
               if (enerv>102*256)
                     excite(100);
               else
                    {
                    cause(12,GetStrBkMot(129+3));      /*'je suis'*/
                    if (!fpress && !chef )
                         {
						 /*et toi??*/
                         bulle(mescon[65-1],1);
                         excite(200);
                         }
                    }
               return;
               }
          if (diquoi==2)/*que veux tu*/
               {
               tuveuquoi();
               return;
               }
          if (diquoi==3)/*que m'offres tu ?*/
               {
               if (enerv>256*102) /*trop enerv‚ pour n‚gocier*/
                    {
					/*rien...*/
                    bulle(mescon[66-1],1);
                    tfr=enerv+200;
                    if (enerv<tfr) enerv=tfr;
                    }
               else
                    {
                    if (! fpress)
                        cause(2,NULL);/*qui est tu ???*/
                    else
                         {
                         cause(15,(Ptr)&refer1[echan-1]);/*moi offrir*/
                         }
                    }
               return;
               }
          /*ou est ton chef*/
          if (chef)     /*c'est lui !!!*/
            excite(32700);/*il s'excite*/
          else
               {
               if (enerv>51*256)
                    {
                    cause(16,NULL);/*je dirais rien*/
                    tfr=enerv+256;
                    if (enerv<tfr) enerv=tfr;
                    }
               else
                    {
                    if (! fpress)
                        cause(2,NULL);/*qui est tu*/
                    else
                         {
                         poschef[0]=poschef[2]=32 ;
                         /*determination de la position du chef*/
                         if (xj/256 <destine[tribu-1].posx) poschef[0]=(128+31);
                         if (xj/256 >destine[tribu-1].posx) poschef[0]=(128+32);
                         if (yj/256 >destine[tribu-1].posy) poschef[2]=(128+33);
                         if (yj/256 <destine[tribu-1].posy) poschef[2]=(128+34);
                         bulle(poschef,1);/*affiche le commentaire*/

                         }
                    }
               }
          }
     else    /*stock1 <> 0*/
          {
          /*flagornerie ou insulte*/
          switch (stock1)
               {
               case 1:  /*insulte*/
               excite(200*stock2);
               break;

               case 2:  /*flagornerie*/
               if (agress<200)
                    {
                    agress++;
                    /*enervement diminue*/
                    tfr=enerv-stock2*100;
                    if (tfr<enerv) enerv=tfr;
                    excite(0);
                    break;
                    }

               default:  /*vantardise*/

               if (agress<200 && !chef) /*si trop agressif ‡a marche pas*/
                    {
                    agress++;
                    if (Rnd() > enerv/200)
                    fuite();
                    else
                         {
                         /*enervement diminue*/
                         tfr=enerv-stock2*100;
                         if (tfr<enerv) enerv=tfr;
                         excite(0);
                         }
                    }
               else   /*il le prend pour une insulte*/
               excite(200*stock2);
               break;
               } /*fin du switch*/
          }
     return;
     }
/***********************************************************************
                        VOLE:On a perdu: il nous pique tout
***********************************************************************/
void vole(void)
     {
     register short i;
     liste4.taille=0; /*plus rien … terre*/
     i=liste1.taille;
     while (i)
          {
          i--;
          /* si la valeur est suffisante que ce n'est pas un objet scenario
        et avec un leger Rnd
         */
          if ( (intrsq(liste1.elements[i])>5) && (liste1.elements[i]<40)
          && (Rnd()>128))
            enlevl(&liste1,&listbidon,i);
          }
     tlpos[1]=1;
     tlpos[4]=1;
     }
/***********************************************************************
                        OUTARM :Armes en sorties
***********************************************************************/
 void outarm(void)
 {
 unsigned char i;
 unsigned char tfr;

 tlpos[1]=1;
 tlpos[4]=1;

 i=liste1.taille;
 /*on enleve les sagaies de la liste en fonction de ce qui reste en sortie
bien sur comme la taille va bouger on remet les positions ds les listes
… zero*/
 while(i)
      {
      i--;
      if (liste1.elements[i]==23)
           {
           if (sap1.nbresag)
           sap1.nbresag--;
           else
           enlevl(&liste1,&listbidon,i);
           }
      }

 if (sap1.forcehache==0) /*pas de hache*/
      {
      i=liste1.taille;
      while(i)
           {
           i--;
           if (liste1.elements[i]==21 || liste1.elements[i]==22)
                {
                enlevl(&liste1,&listbidon,i);
                i=0;
                }
           }
      }
 if (sap1.vie != entvie ) /*il a encaiss‚*/
      {
      tfr=fatig-((entvie-sap1.vie)/2);
      if (tfr<fatig)
      fatig=tfr;
      else
      fatig=0;
      }
 }
/***********************************************************************
                        TRDORS:
***********************************************************************/
void trdors(void)
     {
     long t;
     register short i;
     unsigned char tfr;
     if (fatig>192)
          {
          flefba=0;
          comment(108);
          return;
          }

     liste4.taille=0;/*plus rien a terre*/
     tlpos[4]=1;
     tfr=fatig+stock2;

     if (tfr<fatig) tfr=255;
     fatig=255;
     dervue=VUE_PAYSAGE;
     /*affiche ecran n fois */
     /*avec chronos*/
     stock2 =stock2/7;
     while (stock2)
          {
          GereInterface();
          for (i=0;i<9;i++)
               {
               minute=0;
               chrono(1);
              }
           t=TickCount();
           while(TickCount()<t+10) /**/;
          affich(dzeta);
          tfrima();
          minute=0;
          chrono(2);
          stock2--;
          }
     fatig=tfr;
     chrono(0);
     afstat();

     inimen(0);
     }
/**************************************************************************
                    InitVueLaterale
**************************************************************************/
static void InitVueLaterale(void)
{
if (dervue != VUE_LATERALE)
               {
               ldzeta=dzeta;
               }
prm(&sap2,present[1]);
prm(&sap4,present[3]);
flcomb=1;
if (presence)
               {
               if (presence<5) flcomb=0;
               presence &=127;/*elimine bit 7 pour bestiole amie*/
               prm(&sap1,present[0]);
               prm(&sap3,present[2]);
               if (dervue !=VUE_LATERALE) latera();/* un affichage et dialogue*/
               dervue=VUE_LATERALE;
               }
pos0_mouse();
return;
}
/***********************************************************************
                        TROBSV:Traite l'observation
=> 0 vue 3d
   1 vue laterale
   2 panorama
   3 carte
***********************************************************************/
void trobsv(unsigned short regb)
{
unsigned short dzetdep;
unsigned long posTimer=sapiensTimer;

if (presence) return; /* si il ya quelun alors on est poli*/

if (regb==0) /* vue 4 D*/
          {
          
          	  LastObsv=0;
              dismin=0;
              lastmen=255;
              clrbadi();
              dervue=VUE_PAYSAGE;
              /* gestion de la vue 3d */
              MenuFenetres(FAUX);
              AffichePictAide(1);
              clav3d();
              MenuFenetres(VRAI);
              pos0_mouse();
              /* Une presence a ete rencontr‚e : on initialise la
              vue laterale */
              if (presence)
                   {
                   /* On initialise la vue laterale */
                   regb=1;
                   }
          }
if (regb==1) /*vue laterale*/
          {
          LastObsv=1;
		  InitVueLaterale();
          return;
          }
if (regb==2) /*panaroma*/
          {
          while(getkey(0))/**/;
          dervue=VUE_PAYSAGE;
          dzetdep=dzeta;
          do
               {
                posTimer=sapiensTimer;
               affich(dzeta);
               tfrima();
               dzeta=(dzeta+16) & 0x3FF;
               while(posTimer>sapiensTimer-8)/**/;
               } while ( dzetdep != dzeta && !(getkey(0)));
          }
if (regb==3) /*carte*/
          {
          dervue=VUE_PAYSAGE;
          gemouvre(clin,ccol,cnlin,cncol,15,30,8,15);
          zoocar();
          lastmen=255;
          }
     inimen(0);
     pos0_mouse();
     efbadass();
     TfrBadAss();
     return;
     }
/***********************************************************************
        ZONEVT:Determine un evt en fonction de la position
***********************************************************************/
short zonevt(unsigned short okmess)
     {
     register short i;/* balaye tout les evts*/
     struct evt * dieu;/*pointeur sur l'evt ‚tudi‚*/
     unsigned char nuit,qui;
     short dist;

     /*on balaye tout les evts*/
     for (i=0;i<NBREVTDESTINE;i++)
          {
          dieu=&(destine[i]);
          dist=distance((long)(dieu->posx)*256L-xj,(long)(dieu->posy)*256L-yj);
          dist /=256;
          /*dist est mis a jour*/
          if (dist>(dieu->distmax)) /*dans la zone d'influence*/
               {
               dieu->type &=127;/*on revalide la zone en clearant le bit 128*/
               }
          else /*ok on est ds la zone*/
               {
               /*la zone est-elle valide ??,sommes mous assez central*/
               if (!(dieu->type & 128) && ((dist*12+10)<Rnd()) )
                    {
                    /*on compare … la densit‚ probabiliste de la zone*/
                    if (Rnd() >= dieu->rnd)
                         {
                         /* on affiche un message (l'evt n'a???*/
                         if (Rnd()>240)
                              {
                              if (okmess)
                                   {
                                    efbadi();
                                   comment(dieu->message);
                                   okmess--;
                                   return(0);
                                   }
                              }
                         }
                    else
                         {
                         if (dieu->evt ==0) /*eau ??*/
                              {
                              source++;
                              liste4.taille=tlpos[4]=1;
                              liste4.elements[0]=2;
                              efbadi();
                              comment(76);/*tu entends une source*/
                              return(0);
                              }
                         else
                              {
                              /* est ce un chef ???*/
                              chef=(dieu->evt >127);
                              qui=dieu->evt & 127;
                              /* one shot !!!*/
                              if (dieu->type) dieu->type |=128;
                              nuit=(heure>512);
                              /* test sp‚cifiques a la presence et au moment de la journ‚e*/
                              if (     chef
                              || (nuit  && (qui!=5) && (Rnd()>222))
                              || (!nuit && (qui==5) && (Rnd()>222))
                              || (nuit  && (qui==5))
                              || (!nuit && (qui!=5))
                              )
                                return(qui);
                              else
                                qui=0;
                              }/*evt =eau ou vivant*/
                         } /*Rnd correct*/
                    }/*zone valide*/
               }/*dans la zone*/
          } /*pour tout les elements*/
     return(0);
     }
/***********************************************************************
                        ZONINF:Gestion des evts selon la position
***********************************************************************/
/* on travaille sur la position du joueur,en sortie les flags de presence
sont mis … jour
en sortie 0 aucune presence generee sinon type de la presence de 1 … 8
 */
void  zoninf(void)
     {
     unsigned char tfr,i;

     /*si dej… present alors fin*/
     if (presence) return;
     /*la position est-elle differente*/
     if ( derxj==(xj & 0xFFE0) && deryj==(yj & 0xFFE0)) return;
     /*on m‚morise la derniere position*/
     derxj=xj & 0xFFE0;
     deryj=yj & 0xFFE0;
     chef=0;
     flcomb=0;
     source=0;
     present[1]=zonevt(1);/*avec message possible*/
     if (present[1]) /*il ya un evt vivant*/
          {
          /* s'il ya un chef alors il est automatiquement accompagn‚*/
          /*sinon on retire un evt pour le deuxieme*/
          if (! chef) present[3]=zonevt(0);/*sans message possible*/
          else present[3]=present[1];
          /* si il y un homme il est en premier*/
          if( (present[1]>4 && present[3] <5 && present[3] !=0) )
               {
               /*on swape*/
               tfr=present[1];
               present[1]=present[3];
               present[3]=tfr;
               }
          /*ds tout les cas la presence est fix‚e a celle du prioritaire*/
          presence=present[1];
          tribu=presence;
          if (dervue != VUE_LATERALE)
                sap1.colonne=3;
          /* On calcule les positions des adversaires */
          prm(&sap2,present[1]);
          prm(&sap4,present[3]);
          idialo();
          /* on teste  le charisme immediat sur animal*/
          /* si on est seduisant,qu'il y a une seule bestiole et avec un peu de chance*/
          if (presence>4 && charisme>Rnd() && Rnd()<40 && !present[2])
               {
               flefba=0;
               comment(97+presence);/*premier mess 102*/
               creecop(presence);
               viecopain=1;/*pour les bestioles*/
               flefba=0;
               if (dervue==VUE_LATERALE)  flagbadi=0;
               presence=128;/*bit 7 et presence 0*/
               return;
               }
          }
     if (! source)
          { /*aucun evt n'est valide */
          if (scenar==4 && !scenpr)
               {
                /* A la bonne position on trouve la Hache Scr‚e des Hyenes folles */
                if(
                   ( (derxj>0xABE0) ? derxj-0xABE0:0xABE0-derxj)<500 &&
                   ( (deryj>0xB2E0) ? deryj-0xB2E0:0xB2E0-deryj)<500
                   )
                        {
                        liste4.taille=tlpos[4]=1;
                        liste4.elements[0]=44;
                        return;
                        }
               }
          liste4.taille=Rnd() & 7; /*jusqu'a 7 elements … terre*/
          i=liste4.taille;
          tlpos[4]=1;/* debut de liste */
          /*on g‚n‚re les objets a terre*/
          while(i) liste4.elements[--i]=3+(17*Rnd())/256;
          }
     return;
     }
/*----- FIN -----*/
