/*********************************************************************
                Modul #3 .Sapcomb(Combats Multiples)
                    Le Jeudi 20 Novembre 1986
Le nombre d'herbe au premier plan est ajuste en fonction de la vitesse processeur
Cx :                 2619   0
PowerMac 6100/66 :   6584
Quadra :             10010  20
***********************************************************************/
#include "sapiens.h"

short nbMobHerbes,nbAvplanSpec;
#define LIGNE_BASE_HERBE_AV_PLAN 63
#define LIGNE_MAX_HERBE_AV_PLAN 70
static short NbHerbeAvPlan;
static struct brin herbe[NB_MAX_HERBE_AV_PLAN];
#define LIMITE_GAUCHE -5
#define LIMITE_DROITE 62
long nbHerbes;
short ModeJoueur2=JOUEUR2_AUTONOME;
/***********************************************************************
                DECSOU:Essoufflement
***********************************************************************/
void decsou(struct sap * s,unsigned short combien)
     {
     unsigned char tfr;
     tfr=s->souffle-combien;
     if (tfr < s->souffle)
     s->souffle=tfr;/* ok souffle diminue */
     else
     decvie(s,1);/*plus de souffle puff puff ...*/
     return;
     }
/***********************************************************************
                INCSOU:Recupere du souffle
***********************************************************************/
void incsou(struct sap * s,unsigned short combien)
     {
     unsigned char tfr;
     tfr = s->souffle+combien;
     if (tfr > s->souffle)
     s->souffle=tfr;
     else
     s->souffle=255;
     return;
     }
/***********************************************************************
                POSAN:
***********************************************************************/
void posan(struct sap *sapiens,short anim)
     {
     register short i;
     anim--;
     for (i=0;i!=14;i++)
          {
          if (debban[anim][i])
               sapiens->sapanim[i]=debban[anim][i];
          }
     return;
     }
/***********************************************************************
                ARRET:
***********************************************************************/
void arret(struct sap *sapiens)
     {
     if ((sapiens->vitesse & 2)==0) /*pas arret*/
     sapiens->vitesse=(sapiens->vitesse ^3) & 3;
     return;
     }
/***********************************************************************
                        POSVIT:Positionne vitesse
***********************************************************************/
void posvit(struct sap *sapiens)
     {
     if (sapiens->hauteur==0)      /*debout*/
          {

          sapiens->sens=(((sapiens->vitesse-1)&2)/2);
          if (sapiens->vitesse & 2)      posan(sapiens,2);/*arret*/
          else
               {
               posan(sapiens,1);
               }
          posan(sapiens,sapiens->objet+6);
          return;
          }
     if (sapiens->hauteur !=1)
          {
          arret(sapiens);
          sapiens->sens=(((sapiens->vitesse-1)&2)/2);
          return;
          }
     if (sapiens->action <128)
          {
          sapiens->action |= 128;
          if (sapiens == &sap1) decsou(sapiens,10);
          posan(sapiens,(((sapiens->vitesse-1)/2)^sapiens->sens)+12);
          sapiens->vitesse = (sapiens->sens + 2);
          }
     arret(sapiens);
     return;
     }
/***********************************************************************
                POGNE:Saisi objet … la main
***********************************************************************/
void pogne(struct sap *sapiens)
     {
     char objet;

     objet=sapiens->objet+1;
     if (objet>2) objet=0;
     sapiens->objet=objet;
     /*si objet est la sagaie et (ou trop pres ou plus de sagaie*/
     if ( ( objet == 2 && ( sapiens->nbresag==0 ||  (sapiens->action & 3)))
     || (objet==1 && sapiens->forcehache==0) )
          {
          pogne(sapiens);/*recursif*/
          }
     else
          {
          posan(sapiens,objet+6);/*positionne animation*/
          }
     return;
     }

/***********************************************************************
                        JUGE:Etablit les adversaires
**********************************************************************
Les adversaires sont positionn‚s selon les pr‚sences et en sortie:
0:Tout le clan 2 est ‚limin‚
1:Tout le clan 1 est ‚limin‚
2:Il en reste des deux clans
*/
short juge(void)
     {
     char compt,nbre;
     /* ----->Mise en place des plans */
     for (compt=0,nbre=0;compt!=4;compt++)  if (peuple[compt]->vie) nbre++;
     /*----->nbre est le nombre d'acteurs*/
     /*------>si 4 alors deux couples sur deux plans*/
     if (nbre==4)   profond[2]=profond[3]=7;
     /*sinon tout sur le premier plan*/
     else           profond[2]=profond[3]=10;
     /*dans tout les cas*/
     profond[1]=profond[0]=10;
     /*si reste seulement clan1 alors sur deux plans*/
     if (nbre==2 && sap1.vie && sap3.vie) profond[2]=7;
     /*On positionne l'aversaire de l'ami*/
     if (sap4.vie) sap3.adversaire=&sap4;
     else          sap3.adversaire=&sap2;
     /*On teste si un des deux clans est vide*/
     if ( (sap1.vie + sap3.vie) == 0) return (1) ;/*clan 1 kaput*/
     if ( (sap2.vie + sap4.vie) == 0) return (0) ;/*clan 2 kaput*/
     /* Joueur toujours pr‚sent ???*/
     if (sap1.vie)
          {
          if (sap3.vie) /*il a de l'aide ????*/
               { /*oui il a de l'aide*/
               sap2.adversaire=&sap1;
               sap4.adversaire=&sap3;
               }
          else
               { /*non il est seul */
               sap2.adversaire=&sap1;
               sap4.adversaire=&sap1;
               }
          }
     else
          {  /*l 'aide est seule */
          sap2.adversaire=&sap3;
          sap4.adversaire=&sap3;
          }
     return(2);
     }


/**********************************************************************
                        InitGenerique
***********************************************************************/
void InitGenerique(void)
{
short i, l,c;
Rect r;
CGrafPtr gbak;

AcamGetPortOffscreen(&gbak);
AcamSetPortOffscreen(debecr);
/* on balaye les quatres sapeisn*/
for (i=0;i<4;i++)
        {
    l=peuple[i]->poslcoeur;
    c=peuple[i]->posccoeur;
    affomb(mobFace+31,l-2,c-2,4,4,FAUX);
    affomb(mobFace+30,l-2+22,c-2,4,4,FAUX);
    SetRect(&r,(c-2)*ECHELLE,(l-2)*ECHELLE,(c+16+2)*ECHELLE,(l+16+2)*ECHELLE);
    InsetRect(&r,-1,-1);
    DoFrameRect(&r);
    OffsetRect(&r,0,22*ECHELLE);
    DoFrameRect(&r);
    }
AcamSetPortOffscreen(gbak);
return;
}
/**********************************************************************
                        EffaceGenerique
***********************************************************************/
static void EffaceGenerique(void)
{
short i, l,c;

/* on balaye les quatres sapeisn*/
for (i=0;i<4;i++)
        {
    l=peuple[i]->poslcoeur;
    c=peuple[i]->posccoeur;
//    AffDebecr(mobFace+31,l-2,c-2,FAUX);
//    AffDebecr(mobFace+30,l-2+22,c-2,FAUX);
    }
return;
}
/**********************************************************************
                        GENERIQUE:
***********************************************************************/
void generique(Boolean trace)
 {
 register short i;
 register short l,c,f;

 /* on balaye les quatres sapeisn*/
 for (i=0;i<4;i++)
      {
      l=peuple[i]->poslcoeur;
      c=peuple[i]->posccoeur;
      f=peuple[i]->cross[FORME_TETE]/2;
         /* fond vide */
          /* Mort => une croix */
      if (!peuple[i]->vie)
            {
            AffDebecr(mobFace+31,l-2,c-2,trace);/* rien*/
            affomb(DEBCRANE+10,l,c,2,2,trace);/* croix */
            affomb(DEBCRANE+10,l+22,c,2,2,trace);/* croix */
            }
      else
           {
                        /* Affichage de l'etat de la sant‚ */
           AffDebecr(mobFace+31,l-2,c-2,trace);/* rien*/
           /* Ce n'est pas un animal */
           if (present[i]==0)  AffDebecr(DEBCRANE+8,l,c,trace);
                   /* Affichage de la tete */
           l+=22;
           AffDebecr(mobFace+30,l-2,c-2,trace);/* rien*/
           if (present[i] && present[i]<5)
                AffDebecr(DEBTETE+f,l,c,trace);
         	else
              AffDebecr(DEBBEST+present[i]-5,l,c,trace);
           }
      }
return;
}
/**********************************************************************
                        AffStrVolatile
L'utilisateur a cliqu‚ dans la zone haute : on regarde si le click a eut lieu
dans une case du g‚nerique
***********************************************************************/
static void AffStrVolatile(Point pt,char * chaine)
{
Rect raff;

if(strlen(chaine))
	{
	SetRect(&raff,pt.h-8,pt.v-8,pt.h+100,pt.v+40);
	if(raff.left<0) OffsetRect(&raff,-raff.left,0);
	if(raff.right>640) OffsetRect(&raff,640-raff.right,0);
	DoEraseRect(&raff);
	InsetRect(&raff,4,4);
	TextFont(geneva);
	DoTextSize(9);
	DoTextBox(chaine,strlen(chaine),&raff,teJustLeft);
	InsetRect(&raff,-4,-4);
	DoFrameRect(&raff);
	delai(60);
	AttendsFinBouton();
	TfrEcr(&raff);
	}
return;
}
/**********************************************************************
                        ClickInGenerique
L'utilisateur a cliqu‚ dans la zone haute : on regarde si le click a eut lieu
dans une case du g‚nerique
***********************************************************************/
void ClickInGenerique(Point pt)
{
short i;
Rect r;
char chaine[255];


 /* on balaye les quatres sapeisn*/
 for (i=0;i<4;i++)
      {
      /* Dans le symbole du coeur */
       SetRect(&r,peuple[i]->posccoeur*ECHELLE-4,peuple[i]->poslcoeur*ECHELLE-4,
      			 peuple[i]->posccoeur*ECHELLE+36,peuple[i]->poslcoeur*ECHELLE+36);
      if(PtInRect(pt,&r))
      		{
      		chaine[0]=0;
      		if(peuple[i]->vie<=0)
      			{
      			strcpy(chaine,GetStr(19));
      			}
      		else 
      			{
	      		if(i==0) strcpy(chaine,GetStr(27));
      			else if(i==2) strcpy(chaine,GetStr(29));
    			else  strcpy(chaine,GetStr(28));
      			}
       		AffStrVolatile(pt,chaine);
     		}
      /* Dans le cadre de la tete */
      OffsetRect(&r,0,44);
      if(PtInRect(pt,&r))
      		{
      		chaine[0]=0;
      		if(peuple[i]->vie<=0)
      			{
      			strcpy(chaine,GetStr(19));
      			}
      		else
      			{
	      		if(i==0) sprintf(chaine,GetStr(15),nomjoueur);
      			else if(i==2) sprintf(chaine,GetStr(16),nomcopain);
    			else if(i==1) strcpy(chaine,GetStr(17));
    			else  strcpy(chaine,GetStr(18));
    			}  		
    		AffStrVolatile(pt,chaine);
			}
	  }
return; 
}
/***********************************************************************
                        ABANDO:Laisse objet qd mort
***********************************************************************/
void abando(struct sap *s)
     {
     register short i;
     if (s->animal)   /*c'est une bestiole*/
          {
          for (i =0;i <qtviande[s->animal-1];i++) ajoutl(&liste4,37);
          switch(s->animal)
               {
               case 1: /*lapin*/
               ajoutl(&liste4,33);/*peau de lapin*/
               break;
               case 3: /*loup*/
               ajoutl(&liste4,35);
               ajoutl(&liste4,32);
               break;
               case 4:/*ours*/
               ajoutl(&liste4,34);
               ajoutl(&liste4,31);
               break;
               }
          }
     else
          { /*homme*/
          if (echan > 2) /* pas ma protection ou rien */
                         ajoutl(&liste4,echan);
          i=(s->possession)->taille;
          while(i) ajoutl(&liste4,(s->possession)->elements[--i]);
          /*on squitte pour les autres morts*/
          (s->possession)->taille=0;
          echan=0;
          }
     return;
     }
/***********************************************************************
                        DECVIE
***********************************************************************/
void decvie(struct sap *s,unsigned short valeur)
 {
 short tfr;
 if (!s->vie) return;
 tfr=(short)((short)s->vie-valeur);
 if ( tfr>0) /*si = 0 alors mort*/
      {      /*ok pas mort*/
      s->vie=tfr;
      s->coeur= DEBCRANE+s->vie/32;
      s->cptrsouffle=1;
      /* On va tester si on frappe notre copain */
      if ( (s== &sap3) && (!presence) && valeur )
           {
           /*initialise le sap3 contre nous*/
           presence = -1;
           sap3clav=0;
           s->adversaire=&sap1;
           viecopain=0;
           }
      }
 else
      {
      /*ok,mort*/
      s->vie=0;
      if (!s->animal)
	    {
	    s->flg_mort=16;
	    bruit(s->cross[FORME_BUSTE]<8?11:12);/*argh*/
	    }
	  else
	    {
	    s->flg_mort=1;
	    }
      }
return;
}
/******************************************************************
                    GER_MORT:Traite la mort
*******************************************************************/
static void ger_mort(struct sap *s)
{
     /* Si il peut le plancher l'appelle */
     if (!s->flg_mort) return;
     s->flg_mort--;
     if ( !(s->flg_mort &2) && (!s->animal))
        {
         baisse(s);
         if (s->hauteur>=3) s->flg_mort=0;
        }
     /* on a atteint le sol */
     if (!s->flg_mort)
     {

     if (presence == -1) /* on va baiser Juge*/
          {
          sap2.vie=sap3.vie;
          sap3.vie=0;
          }
     if (s==&sap3)
          {
          if (presence == -1) /*c'est moi qui le frappe */
               {
               viecopain=chef=0;/*On met le chef … zero pour le scenar 4*/
               if (charisme>10)
                    {
                    charisme -=10;
                    param[2] -=10;
                    }
                }
          else    viecopain--;
          flefba=0;
          if (!viecopain)
                {
                present[2]=0; /*degenere*/
                comment(112); /*Ton ami est mort*/
                }
          else
               comment(111);   /*Il est Evanoui*/
         }
         if (presence  != juge()) fin=juge()+1;
         if (s != &sap1) abando(s);    /*abandonne objets*/
         /* Il est avanoui on l'affiche a terre*/
         if ( ! s->animal)/*c'est un homme*/
               {
               if (s->colonne>LIMITE_DROITE) s->colonne=1;
               if (s->sens) s->colonne+=8;/* pour centrer le mort */

               AffmobConst(MOB_MORT+(s->cross[FORME_TETE])/2,100-34+profond[who],
                                        4*(s->colonne+8*(s->colonne<8)),
                                        s->sens);
               }
          else
                        AffmobConst(MOB_ANIMAL_MORT+s->animal-1,
                                                        100-20+profond[4]-8*(s->animal>2),
                                                        4*s->colonne,
                                                        s->sens);

         /*on me met hors ecran*/
          s->colonne=200;
          generique(VRAI);
     }
     return;
}
/***********************************************************************
                        TOUCHESAG:Teste si Sagaie Touche
***********************************************************************/
/*En entree le Sap qui lance et celui qui est vis‚
  en sortie la sagaie est positionn‚e
*/
void touchesag(struct sap *s1,struct sap *s2)
     {
     char tfr,test;
     /*si l'adversaire est absent sa colonne >64*/
     if (s2->colonne < 200 && (s1->snssag <128))
          {
          /*calcul de la colonne du test*/
          test = s1->colsag- s2->colonne;
          if (test<0) test = -test;
          if ((test<3) && (s1->linsag > hautou [s2->hauteur] ))
               {
               /*ok on touche*/
               s2->blockimpact=4;
               bruit(1);/* La sagaie touche */
               tfr=s1->linsag;
               if (tfr>30) tfr=30; /*pas trop bas*/
               s2->ligneimpact=tfr;
               s2->colimpact=s1->colsag+6*(s1->snssag==0);
               decvie(s2,30);
               AffmobConst(MOB_FLECHE,100-23+profond[who],4*s1->colsag,s1->snssag);/*a terre*/
               ajoutl(&liste4,45);
               s1->snssag=255; /*d‚g‚nŠre sagaie*/
               }   /*si touche*/
          }   /*si presence adversaire*/
     return;
     }
/***********************************************************************
                        GERSAG
********************************************************************/
void gersag(struct sap *s1)
     {
     char tfr;
     if (s1->snssag<128)/*sagaie lanc‚e !!!!!*/
          {
          /*si la sagaie est la on l'affiche*/
          if (s1->snssag)
          tfr=s1->colsag+3;
          else
          tfr=s1->colsag-3;
          if (tfr<0 || tfr>63) s1->snssag=255; /*degenere*/
          else
               {
               s1->colsag=tfr;
               s1->linsag++;
               if (s1->linsag>45)
                    {
                    AffmobConst(MOB_FLECHE,100-23+profond[who],4*s1->colsag,s1->snssag);/*a terre*/
                    if(Rnd()>200) ajoutl(&liste4,SAGAIE);
                    else          ajoutl(&liste4,45);
                    s1->snssag=255; /*degenere*/
                    }
               else
               Affmob(MOB_FLECHE,s1->linsag+profond[who]+32,4*s1->colsag,s1->snssag);

               }
          /* Si c'est le Joueur il peut toucher les 3 autres */
          if (who==0)
               {
               touchesag( s1,&sap2);
               touchesag( s1,&sap3);
               touchesag( s1,&sap4);
               }
          else
          /*Si ce n'est pas le joueur il ne vise qu'une personne en particulier*/
          touchesag(s1,s1->adversaire);

          }/*sagaie existe ?*/
     return;
     }
/***********************************************************************
                          SAUT:
***********************************************************************/
void saut(struct sap *sapiens)
     {
     if ((sapiens->hauteur==0)  || (sapiens->action & 128)) return;
     sapiens->hauteur--;
     sapiens->action=0;
     if (sapiens->hauteur==0)
          {
          posvit(sapiens);
          return;
          }
     posan(sapiens,sapiens->hauteur+8);
     arret(sapiens);
     posan(sapiens,sapiens->objet+6);
     return;

     }
/***********************************************************************
                TOUCHE:
***********************************************************************/
/* en entr‚e le sap qui frappe et celui qui encaisse*/

void touche(struct sap *s,struct sap *s2) /*s frappe s2 encaisse*/
     {
     register short coltest,dommage,tfr;
     /*calcul de la position du test*/
     /**/
     if (s->sens) coltest=s->colonne+ *(ptranim);
     else         coltest=s->colonne- *(ptranim);
     /**/
     /*quel est l'Adversaire*/
     /*valeur absolue de la distance*/
     tfr=coltest-s2->colonne;
     if (tfr < 0) tfr=-tfr;
     /**/
     /*le frappe t'on alors qu'il est assis ?*/
     if ((tfr>3) || ( s2->hauteur  > s->hauteur+1 )) return;
     /**/
     /*positionne l'impact*/
     s->blockimpact=4;
     s->colimpact=s2->colonne+(s2->sens ? -3: 2)+6*(s->sens);
     if (/*s->colimpact<0 ||*/ s->colimpact>128) s->colimpact=0;
     else if (s->colimpact>LIMITE_DROITE+3) s->colimpact=LIMITE_DROITE+3;
     s->ligneimpact=tabinp[s->hauteur];
     if ( ((s->hauteur/2)==(s2->hauteur/2)) && (s->sens!=s2->sens)
     && (s2->action &3) )
          {/*meme hauteur sens oppos‚e et frappent tout deux*/
          s->blockimpact=2;
          if (Rnd()<10)
               {/*ok destroy*/
               bruit(4);
               if (s2->objet==1)
                    {
                    /*destroy hache du sap2*/
                    s2->forcehache=0;
                    pogne(s2);
                    }
               if (s->objet==1)
                    {
                    s->forcehache=0;
                    pogne(s);
                    }
               } /*Rnd insuffisant*/
          else
          	   bruit(2);

          }/*hache se touchent*/
     else
          {
          /*il recule*/
          if (s->sens)
               {
               if (s2->colonne<LIMITE_DROITE-3) s2->colonne +=3;
               }
          else
               {
               if (s2->colonne>LIMITE_GAUCHE+3) s2->colonne -=3;
               }
          dommage=5;
          if (s->objet==1) dommage =s->forcehache;
          if (s->hauteur==0) dommage *=2;
          if (s2==&sap1)  dommage -= resistance/50;
          decvie (s2,dommage);
      if (s->animal)
            {
            if (Rnd()>200) bruit(8);
            }
      else          bruit(0);
          }
     return;
     }
/***********************************************************************
                GCOEUR:
***********************************************************************/
static void gcoeur(struct sap *s)
     {
     char quoi;
     
     if (s->cptrsouffle<32)
          {
          s->cptrsouffle=s->souffle;
          quoi=s->coeur;
          if (quoi<0) quoi=1-quoi;
          affomb(quoi,s->poslcoeur,s->posccoeur,0,0,VRAI);
          s->coeur=-s->coeur;
          }
     else
     	s->cptrsouffle-=32;
     return;
     }
/***********************************************************************
                        IMPAK:Gere les impacts
***********************************************************************/
static void impak(struct sap *s)
 {
 if (s->blockimpact)
      {
      Affmob(s->blockimpact+MOB_IMPACT,s->ligneimpact+profond[who]+32,
                4*s->colimpact,s->sens);
      /* sang par terre et dec impact */
      if (! (--s->blockimpact))
            AffmobConst(MOB_SANG_A_TERRE,100+profond[who]-19-myRandom(0,4),4*s->colimpact,s->sens);
      }
 return;
 }
/***********************************************************************
                        AVASAP:
        (c) God! Departement Intelligence Artificielle
                Avril '86-   D‚cembre '86
***********************************************************************/
static void avasap(struct sap *s)
     {
     if (s->sens==0)
     decvit(s);
     else
     incvit(s);
     }
/***********************************************************************
                        RECUSAP:
***********************************************************************/
static void recusap(struct sap *s)
     {
     if ((s->hauteur==1) && !(s->action & 128))
          {
          if (s->sens==0)
          incvit(s);
          else
          decvit(s);
          }
     }
/***********************************************************************
                        FIXCOLON
***********************************************************************/
static void fixcolon(struct sap *s)
     {
     /* si ami alors meme place*/
     if (s== &sap3)
          {
          s->colonne=sap1.colonne;
          s->sens=sap1.sens;
          }
     else
          {
          if ( (s== &sap1) && (s->colonne>64))
               {
               s->colonne=2;
               s->sens=1;
               return;
               }
          if (s->colonne>32)
               {
               if (s != &sap1)
                        {
                    s->sens=0;
                    s->colonne=53+who;
                    }
               else
                s->colonne=55;
               }
          else

               {
               if (s != &sap1)
                    {
                    s->sens=1;
                    s->colonne=5-who;
                    }
               else s->colonne=1;
               }
          }
     }
/***********************************************************************
                        INIUNE:Inits primaires de vue lat
***********************************************************************/
void iniune(struct sap *s)
     {
     s->sapanim[10]=aneutr;/*supprime arme*/
     s->sapanim[11]=aneutr;
     s->action=0;
     s->ligne=2;
     s->hauteur=0;

     s->objet=0;
     posan(s,2);/*arret par defaut*/
     if (s->colonne >32) s->sens=0;
     else                s->sens=1;
     fixcolon(s);
     s->vitesse=s->sens | 2;/*arret*/
     s->animal=s->deltah=0;
     s->souffle=200;
     s->cptrsouffle=1;
     return;
     }
/***********************************************************************
                PRM:Initialise Rencontres
***********************************************************************
=> Type de la pr‚sence,(chef):
0 non present;
1 Pied agile,2 Yeux malins,3 Bras de roc,4 Hyenes folles
5 Loup,6 ours,7 Lapin,8 Lagopede,0 rien ,-1 Joueur
*/
void prm(struct sap *s,short type)
     {
     register short i;

     s->snssag=255;
     s->fuite=0;
     s->blockimpact=0;
     s->flg_mort=0;


     /*On va redeterminer who pour iniune*/
     if (s==&sap1) who=0;
     if (s==&sap2) who=1;
     if (s==&sap3) who=2;
     if (s==&sap4) who=3;

     present[who]=type;

     if (type==0)
          {
          /*non present*/
          s->colonne=200;
          s->vie=0;
          return;
          }
     if (type < 0 || (who==2 && type<5 ))/*joueur ou aide humaine*/
          {
          if (type==-1) sap1.vie=(vie*50)/256+(bless*93)/256+(faim*46)/256+(soif*46)/256;
          else s->vie=viecopain*60+20;
          s->forcehache=tribforce[type-1];
          iniune(s);
          decvie(s,0);/* met en place coeur*/
          return;
          }
     /*raz du cross du sap*/
     for (i=0;i !=7; i++)  s->cross[i]=0;

     if (type<5) /*homme*/
          {
          do
              {
               s->cross[FORME_BUSTE]=2*myRandom(0,4);
               if (chef && s==&sap2)
                    {
                    i=8;           /*gueule de chef*/
                    s->cross[FORME_BUSTE]=0;
                    }
               else
                    {
                    if (Rnd()<100)
                         {  
                         /*femelles*/
                         i=12+(Rnd()/43)*2;			/*selectionne tete*/
                         s->cross[FORME_BUSTE] +=8; /*buste*/
                         }
                    else
                         {  /*male*/
                         i=2*(Rnd() & 3);/*gueule*/
                         }
                    }
               if (type==4) /*hyene folle*/
              			i=10; /*masque hyene folle*/
               } while (i==sap1.cross[FORME_TETE]);

          s->cross[FORME_TETE]=i;/*tete*/
          /*masse ou hache*/
          s->cross[FORME_ARME]=Rnd() & 1;
          }

     /*si homme ou bestiole*/

     /*colonne sap2*/
     if(sap1.colonne >32) s->colonne=2;
     else                                 s->colonne=54;
     iniune(s);
     /*si Homme*/
     if (type<=4)
          {
          s->nbresag=4; /*4 sagaies*/
          s->agressivite=agress /256;
          s->forcehache=tribforce[type-1];
          s->vie=tribvie[type-1];
          if (chef && (tribu==1))      /*si notre chef alors dur dur*/
               {
               s->agressivite=255;
               s->forcehache=40;
               s->vie=200;
               }
          if (! type)/*personne*/
               {
               s->sapanim[0]=0; /*pas g‚n‚r‚*/
               s->sapanim[1]=0;
               s->colonne=200;
               s->vie=0;
               }
          posvit(s);
          }
     else
     /*bestiole*/
          {
          if (type >= 7) s->sapanim[1]=anlap1;
          else           s->sapanim[1]=anlou1;
          s->sapanim[0]=s->sapanim[1];
          s->sapanim[2]=0;     /*plus d'animation apres 1*/
          s->sapanim[3]=0;
          s->cross[0] = ((type & 1) ^1)*4;
          s->animal=((type-5)^2)+1;
          s->vie=bestvie[type-5];
          s->ligne=bestlin[type-5];
          s->hauteur=besthaut[type-5];
          s->agressivite=bestagress[type-5];
          s->vitesse=0;/*fuite … 0*/
          s->objet=1;/*pour frappe*/
          s->forcehache=10;/*dents*/
          }
     decvie(s,0);/* met en place coeur*/
     generique(FAUX);
     return;
     }
/***********************************************************************
                BAISSE:
***********************************************************************/
void baisse(struct sap *s)
     {
     if (s->hauteur<3) /*on peut encore dscendre*/
          {
          s->hauteur++; /*donc on le fait*/
          s->action=0;
          posan(s,s->hauteur+8);
          arret(s);
          posan(s,s->objet+6);
          }
     return;
     }
/***********************************************************************
                ACTBRA:
**********************************************************************
*/
void actbra(struct sap *s)
     {
     if (!(s->action & 3) && (s->souffle>32 || who))
        /*deja action (ou essouffl‚)*/
          {
          if (s==&sap1) decsou(s,20);
          s->action =s->action  | (s->objet+1);
          if (s->objet==2)  /*objet est la sagaie*/
               {
               if ((s->snssag>128)&&(s->nbresag))/*deja sagaie ? */
                    {
                    s->colsag=s->colonne+2;
                    s->nbresag--;
                    s->linsag=hautlin[s->hauteur];
                    s->snssag=s->sens;
                    } /*si sagaie n'existe pas deja*/
               else
               return;
               } /*si objet =sagaie*/
      posan(s,s->objet+3);
          }/*si souffle correct*/
     return;
     }
/***********************************************************************
                DEPLAT:D‚placement de vue lat‚rale
***********************************************************************/
void deplat(void)
     {/*position en colonne*/
     if (sap1.colonne<32)
          {
          ldzeta=(dzeta+512) & 0x03FF;
          sap1.colonne=LIMITE_DROITE-1;
          }
     else
          {
          ldzeta=dzeta;
          sap1.colonne=LIMITE_GAUCHE+1;
          }
     sapdep(ldzeta,46); /* 46 est la vitesse*/
     minute=0;
     chrono(0);
     }
/***********************************************************************
                MVTSAP:Gere les mouvements
**********************************************************************
=> Pointeur sur le sap a etudier
<= 1 si il faut le rappeler immediatement 0 sinon*/
 short mvtsap(struct sap *sapiens)
     {
     short registerb,nummob,ptrcross,ok;
     char tfr;
     short colmob,linmob,lastcol;


     sensmob= sapiens->sens;
     registerb=0;/*reg b balaye les differents morceuax*/
     ptrcross=0;
     while (1) /*infini*/
          {
          /*on prends l'animation*/
          ptranim=sapiens->sapanim[registerb];
          /*teste si fin de sap:animation=0*/
          if (!ptranim) return(0);
          ok=0;
          while (!ok)
               {/*ok*/
               tfr=*(ptranim++);/*caractŠre suivant*/
               /*teste si le caractŠre lu est une commande*/
               if (tfr>=0)   /*non :pas une commande*/
                    {
                    ok++;
                    nummob=tfr;
                    }
               else
               /*****************************/
               /*oui !:on traite la commande*/
                    {
                    if (tfr==-1)/* 255 fin d'animation*/
                         {/*l'animation reviens … l'animation standart*/
                         ptranim=sapiens->sapanim[registerb+1];
                         continue;/*on revient … la lecture de l'animation*/
                         }
                    tfr&=127;
                    if (tfr==0)/*d‚placement vertical*/
                         {/*le caractŠre suivant donne la hauteur*/
                         sapiens->deltah=*(ptranim++);
                         continue;
                         }
                    if (tfr==1)/*d‚placement horizontal*/
                         {
                         if (registerb==0) /*uniquement sur 1ere jambe*/
                              {
                              lastcol=sapiens->colonne;
                              if (sensmob) /*deplacement vers la droite*/
                                        (sapiens->colonne)++;
                              else
                                         (sapiens->colonne)--;

                              if ((sensmob==0) && (sapiens->colonne<=LIMITE_GAUCHE) ||
                                        (sensmob==1) && (sapiens->colonne>LIMITE_DROITE))
                              /*sortie !!!!!*/
                                   {
                                   if (presence)
                                        {
                                        sapiens->colonne=lastcol;
                                        if (present[who]<5)
                                             {
                                             arret(sapiens);
                                             if (sapiens->hauteur)
                                             posan(sapiens,sapiens->hauteur+8);
                                             else posan(sapiens,2);
                                             posan(sapiens,sapiens->objet+6);
                                             sapiens->action &=127;
                                             return(1);/*on le refait*/
                                             }
                                        }
                                   else

                                        {
                                        fin=128;
                                        if (sapiens==&sap3)
                                             {
                                             if (!sap1.vie) fin=129;
                                             sap1.colonne=sap3.colonne;
                                             sap1.sens=sap3.sens;
                                             }
                                        return(0);
                                        }
                                   }
                              }
                         continue;
                         }
                    if (tfr==2)/*empreinte*/
                         {
                         AffmobConst(MOB_EMPREINTE,100+profond[who]-19,4*(sapiens->colonne+sapiens->sens*5),
                                        sapiens->sens);
                         continue;
                         }
                    if (tfr==3)/*neutre*/
                         {
                         ptrcross++;
                         ok=2;/*on passe au morceau suivant*/
                         break;
                         }
                    if (tfr==4)/*depositionne baisse*/
                         {
                         sapiens->hauteur--;
                         arret(sapiens);
                         posan(sapiens,10);/*… genoux*/
                         return(1);/*on le refait*/
                         }
                    if (tfr==5)/*recule*/
                         {
                         if (registerb==0)
                              {
                              lastcol=sapiens->colonne;
                              if (sensmob)
                              (sapiens->colonne)--;
                              else
                              (sapiens->colonne)++;

                              if ((sensmob==1) && (sapiens->colonne<LIMITE_GAUCHE+3) ||
                              (sensmob==0) && (sapiens->colonne>LIMITE_DROITE+3))
                              /*sortie !!!!!*/
                                   {
                                   if (presence)
                                   sapiens->colonne=lastcol;
                                   else
                                        {
                                        fin=128;
                                        if (sapiens==&sap3)
                                             {
                                             if (!sap1.vie) fin=129;
                                             sap1.colonne=sap3.colonne;
                                             sap1.sens=sap3.sens;
                                             }
                                        return(0);
                                        }
                                   }
                              }
                         continue;
                         }
                    if (tfr==6)/*fin d'arme*/
                         {
                         sapiens->action=sapiens->action & *(ptranim++);
                         continue;
                         }
                    if (tfr==7)/*teste touche*/
                         {
                         /* Si c'est le Joueur il peut toucher les 3 autres */
                         if (who==0)
                              {    /* Frappe le copain ????*/
                              if ( presence < 1)   touche(sapiens,&sap3);
                              else
                                   {
                                   touche(sapiens,&sap2);
                                   touche(sapiens,&sap4);
                                   }
                              }
                         else
                         /*Si ce n'est pas le joueur il ne vise qu'une personne en particulier*/
                         touche(sapiens,sapiens->adversaire);

                         ptranim++;
                         continue;
                         }
                    if (tfr==8)/*fin sagaie*/
                         {
                         pogne(sapiens);
                         continue;
                         }

                    }
               }/*ok*/
          if (ok==1)/*ok on y va sinon anim suiv*/
               {
               /*Le caractŠre lu … ‚t‚ trait‚ soit comme numero de block
                         soit comme commande*/
               linmob=(sapiens->ligne)+ *(ptranim++);
               tfr=*(ptranim++);
               if (sapiens->sens)
                        {
                         tfr=-tfr+6;/*epaisseur*/
                        /* teste l'ours*/
                         if( sapiens->animal==4 ) tfr+=8;
                        }
               sapiens->sapanim[registerb]=ptranim;/*on m‚morise la nlle position*/
               colmob=(short)tfr+sapiens->colonne;
               if (colmob>127) return(0);
               if (nummob>10) linmob+=sapiens->deltah;
               Affmob(nummob+((short)sapiens->cross[ptrcross++])+MOB_CORPS,
                                        (char)linmob+(char)profond[who]+32,4*colmob,sapiens->sens);
               }
          registerb +=2;/*morceau suivant*/
          }/*infini*/
     }/*fin de procedure*/
/***********************************************************************
                        DECVIT:Diminue vitesse
***********************************************************************/
void decvit(struct sap * sapiens)
     {
     unsigned char tfrvit;
     tfrvit =(sapiens->vitesse)-1;
     if (tfrvit)
          {
          sapiens->vitesse=tfrvit;
          posvit(sapiens);
          }
     return;
     }
/***********************************************************************
                        INCVIT:Augmente vitesse
***********************************************************************/
void incvit(struct sap * sapiens)
     {
     unsigned char tfrvit;
     tfrvit=(sapiens->vitesse)+1;
     if (tfrvit<5) /* varie de 1 … 4 */
          {
          sapiens->vitesse=tfrvit;
          posvit(sapiens);
          }
     return;
     }

/***********************************************************************
                CLAV:Clavier durant combat
***********************************************************************/
void clav(struct sap * s,short caract)
 {
 if ( !caract) return;
 if (caract & 4) decvit(s);
 else
 if (caract & 8) incvit(s);
 if (caract & 1) saut(s);
 else
 if (caract & 2) baisse(s);
 if ( ( (caract & 18)==18 || caract & 64) && presence != -1) fin=129;
 if (caract & 16)
        {
        pogne(s);
        while(getkey(0)&16)/**/;
        }
 if (caract & 32) actbra(s);
 return;
 }
/***********************************************************************
                     GERCOP:Gere l'ami humain ou non
***********************************************************************/
/* sap 1 c'est le joueur sap3 le copain*/
void gercop(struct sap * scop,short type)
     {
     unsigned char sens;
     distsp=sap1.colonne-scop->colonne;/*distance entre saps*/
     if (distsp<0)/*valeur absolue*/
          {
          distsp=-distsp;
          sens=0;
          }
     else
     sens=1;


     /*dans le cas ou on gere le sap3 seul avec nous*/
     /*si le sap1 est mort alors il va tendre … sortir*/
     if (distsp>3) /*encore loin*/
          {
          scop->sens=sens;    /*vers le joueur*/
          if (type) /* c'est un humain*/
               {
               if (scop->hauteur) saut(scop);/*si baiss‚ alors monte*/
               else  avasap(scop);/*il marche*/
               }
          else /*c'est une bestiole*/
               {
               if (present[who]>6) scop->sapanim[1]=anlap2;
               else                scop->sapanim[1]=anlou2;
               }
          }
     /* dej… pr‚s*/
     else
          {
          if (!type)                       /*si bestiole*/
               {
               scop->sens=sap1.sens;
        /*arrete*/
                if (present[who]>6) scop->sapanim[1]=scop->sapanim[0]=anlap1;
                else                scop->sapanim[1]=scop->sapanim[0]=anlou1;
               scop->coeur=DEBCRANE+8;
               scop->cptrsouffle=1;
               return;
               }
          /*si humain*/
          if (scop->objet)   pogne(scop); /* plus d'objet … la main*/
          if (scop->hauteur) saut(scop);/*tjs debout*/
          else
               {
               scop->vitesse=sap1.vitesse;/*meme sens*/
               arret(scop);/*arrete :vitesse*/
               posvit(scop);/*il va s'arreter,positionner le sens et l'anim*/
               }
          }
     return;
     }
/***********************************************************************
                AUTONHOM:
***********************************************************************/
void autonhom(struct sap * s2,struct sap * s1)
 {
 unsigned char sens;
/*
test specifique au copain alors que je suis seul a me battre contre un
adversaire, Sapiens etant un jeu eminement MORAL, mon ami peut refuser
de frapper le meme adversaire que moi...(2 Sept 87)
*/
 if (s2==&sap3) /* si c'est mon copain */
    {
    /* si je suis en vie et qu'il n'y a pas d'adversaire pour
     mon ami et que le Rnd est favorable... Il bouge tout de meme
     un peu pour donner le change...
     Mais qu'est ce qu'il me prends de commenter aussi ABONDAMENT ?
     alors que cette trapsure est au moins aussi geniale et
     infaillible que le reste du programme.
     */
    if (sap1.vie && !sap4.vie && Rnd()>60)
        return;/* puissant non ???*/
    }

 distsp=s1->colonne-s2->colonne;/*distance entre saps*/
 if (distsp<0)/*valeur absolue*/
      {
      distsp=-distsp;
      sens=1;
      }
 else
      sens=0;

 if ((s2->vie<20) || (Rnd()>160))
      {
      if ( (s1->hauteur>1) && Rnd()>128) s2->fuite=4;
      else
      if(s2->fuite && Rnd()>200)  s2->fuite=Rnd()/13;
      }
 if (sens==s2->sens)
      {
      if (s2->hauteur) saut (s2);/*il se releve*/
      s2->sens=s2->sens^1;/*il se retourne*/
      return;
      }
 /* La distance entre les 2 saps est inf‚rieure … 8*/
 if (distsp > 8 )/*peut tirer*/
      {
      if (s2->objet==2) /*sagaie*/
           {
           if (Rnd() < s2->agressivite) actbra(s2);/*il frappe*/
           }
      else  /*si pas sagaie en main il va la prendre*/
           {
           if (Rnd()<20 && s2->nbresag)
                {
                pogne(s2);/*il essai*/
                return;
                }
           }
      if (Rnd() < 60)
        {
        /* test si copain et lapin en face */
        if ((s2 != &sap3)||(present[1]<7))
            {
            avasap(s2);
            }
        }
      else
                saut(s2);

      return;
      }
 else /*assez pres*/
      {
      if (distsp< 2 || s2->fuite ) /*beaucoup trop pres*/
           {
           /*il tend a se mettre accroupis puis a reculer*/
           if (s2->hauteur>1)
                {
                saut(s2);
                return;
                }
           if (s2->hauteur==0)
                {
                baisse(s2);
                return;
                }
           if (s2->fuite) s2->fuite--;
           recusap(s2);
           return;
           }
      if (s2->hauteur<2 && Rnd()>180  && (s2->action & 128)==0) baisse(s2);
      if (distsp< 6 )/*il peut frapper*/
           {
           if (Rnd()<(s2->souffle*s2->agressivite)/512+128)
                {
                actbra(s2);/*il frappe*/
                return;
                }
           else
           /*ne frappe pas:alors bouge*/
           if ( (s1->action || distsp<4) && ((s2->action & 128)==0))
                {
                if (s2->hauteur<2)  recusap(s2);
                else
                if (Rnd()>160)  baisse(s2);
                else            saut(s2);
                }
           else
           baisse(s2);
           }
      else /*proche mais pas assez pour frapper*/
           {
           if (s2->hauteur >= 2)
                {
                saut(s2);/*il se releve*/
                return;
                }
           if ( (s2->objet !=1 ) && ( Rnd() > 70 ))
           					pogne(s2);/*prends sa hache*/
           if (Rnd()>80)   avasap(s2);
           return;
           }
      if (s1->action & 3)
           {
           if (s1->hauteur>=2 && Rnd()<10)
           baisse(s2);
           else
           saut(s2);
           return;
           }
      if (Rnd()>s2->agressivite)
           {
           if (s2->hauteur >=2)
                {
                recusap(s2);
                }
           return;
           }
      }
 }
/***********************************************************************
                        BESTIO:Gere les Bestioles
***********************************************************************/
void bestio(struct sap * s2,struct sap * s1)
     {
     unsigned char sens;
     
     distsp=s1->colonne-s2->colonne;
     if (distsp<0)
          {
          distsp=-distsp;
          sens=1;
          }
     else
     	sens=0;
     /*1=Lapin,2=LagopŠde,3=Loup,4=Ours*/
     if (s2->animal >=3)
          {
          /*loup-Ours*/
          /* si col‚ a gauche ou a droite*/
          if ( (s2->colonne<LIMITE_GAUCHE-3))
               {
               s2->sens=1;
               s2->colsag=10+Rnd()/20;
               s2->fuite=2;
               }
          if ( (s2->colonne>LIMITE_DROITE+3))
               {
               s2->sens=0;
               s2->colsag=10+Rnd()/20;
               s2->fuite=2;
               }
          /*si blesse ou avec une faible prob retrait*/
          if ( (s2->vie<40 && Rnd() > 180) || (Rnd()<10) )
               {
               if (!s2->fuite)
                    {
                    s2->sapanim[1]=anlou2;
                    s2->fuite=1;
                    s2->colsag=Rnd()/12;
                    }
               }
          if (s2->colsag)  /*fuite:1 fuite reele 2 eloigne du bord*/
               {
               if (s2->fuite==1)  s2->sens=sens;
               s2->colsag--;
               }
          else
          if (distsp>4 && sens ==s2->sens)/*pas trop pres et tourne le dos*/
               {
               s2->sens=s2->sens ^ 1;/*le sens s'inverse*/
               }
          if (Rnd()<s2->agressivite)
               {
               if (Rnd()>100) s2->sapanim[1]=anlou1;
               else           s2->sapanim[1]=anlou2;
               }
          s2->souffle=128;
          s2->coeur= (s2->coeur<0) ? -(DEBCRANE+8):(DEBCRANE+8);
          }
     else    /*P'TIT Lapin*/
          {
          s2->souffle=1;  /*ca bat vite un p'tit coeur*/
          s2->coeur= (s2->coeur<0) ? -(DEBCRANE+8):(DEBCRANE+8);

          if ( (!s2->fuite) && (Rnd()<60) ) /* rien c'est con un lapin*/
               {
               /* plus l'adversaire est proche plus il s'agite*/
               if ((Rnd()/8)>distsp)
                    {
                    s2->sapanim[0]=anlap3;/*lapin observe*/
                    if ((distsp<10 && Rnd()>60) || (!(s1->vitesse & 2))
                    || (s1->snssag !=255) || (s1->action))
                    /*si proche et bouge ou si sagaie en l'air*/
                         {
                         s2->sapanim[1]=anlap2;/*fuite*/
                         s2->sens=s1->sens;
                         s2->fuite=1;
                         }
                    }
               else
                    {
                    if (Rnd()<s2->agressivite)/*ici la peur*/
                         {
                         s2->sapanim[0]=anlap2;
                         if (Rnd()<25 && distsp<45)  /*Reglage sortie Lapin*/
                              {
                              s2->sens=Rnd() & 1;/*sens au hazard*/
                              }
                         }
                    }
               } /*c'est con il fait rien*/
          if (s2->colonne <LIMITE_GAUCHE+3 || s2->colonne >LIMITE_DROITE-3)
               { /*il sort de l'ecran : on le degenere*/
               s2->sapanim[0]=0;
               s2->colonne=200;
               s2->vie=0; /* a pu lapin*/
               present[who]=0;
               generique(VRAI);
               if (presence  != juge()) fin=juge()+1;
               }
          }
     return;
     }
/***********************************************************************
                MOBS:Deplace les Sapiens
***********************************************************************/
static void mobs(Boolean TesteClavier)
 {
 register short i;
 unsigned char tfragil;
 register struct sap * surqui;

/* Pour les 4 sapiens */
 for ( who=3 ; who>=0 ;who--)
      {
      surqui=peuple[who];
      /* teste si animation mort */
      if (surqui->flg_mort)
	        {
	        ger_mort(surqui);
	        while(mvtsap(surqui))/**/;
	        }
      else
	      {
	      if (surqui->vie)               /*  si defini alors*/
	           {
	           incsou(surqui,1);      /*il se repose*/
	           while(mvtsap(surqui))/**/; /*il s'anime*/
	           if (who==0)
	                {/*Le sap1 agit*/
	                tfragil = cptagil + agilite ;
	                if ( tfragil < cptagil )
	                	if(TesteClavier) clav(surqui,getkey(0));
	                cptagil=tfragil;
	                }
	           else
	           /* Gestion du copain:Il y a t'il une gestion de sap alors
	                     qu'il n'y a plus d'adversaire?*/
	           if (present[who]<5) /*  homme*/
	                {
	                if (sap3clav && who==2) /*au clavier*/
	                        {
	                        if(TesteClavier) clav(surqui,getkey(1));
	                        }
	                else
	                     {
	                     if (!presence)
	                        gercop(surqui,present[who]<5);
	                     else
	                                    autonhom(surqui,surqui->adversaire);
	                     }
	                }
	           else               /*bestiole*/
	                {
	                if (!presence)
	                     gercop(surqui,present[who]<5);
	                else
	                     {
	                     bestio(surqui,surqui->adversaire);
	                     /* si c'est un lapin alors il agit aussi sur
	                             mon ami*/
	                     if (present[who]>6 && present[2])
	                                                     bestio(surqui,&sap3);
	                     }
	                }
	           gcoeur(surqui);/*affichage du coeur*/
	           }
	      }
      gersag(surqui);
      }

 /*tout les impacts devant*/
 for (who=0;who <4;who++) impak(peuple[who]);
 /*Affiche les Herbes au 1er plan*/
 for (i=0;i<NbHerbeAvPlan;i++)
            if (herbe[i].type) Affmob(herbe[i].type, herbe[i].lin, herbe[i].col,herbe[i].sens);
 return;
 }
/***********************************************************************
                        CalculeMobHerbe
Calcule un numero de mmob pour l'herbe
***********************************************************************/
static short CalculeMobHerbe(void)
{
short num;

if(myRandom(0,1000)>950)
        num=mobAvplanSpec+1+myRandom(0,nbAvplanSpec);
else
        num=MOB_HERBE+myRandom(0,nbMobHerbes);
return(num);
}
/***********************************************************************
                        AVPLAN:Calcule avanplan
***********************************************************************/
#define LIGNE_CHEMIN 68
#define GRANULOSITE 10
static short * TabIndexHerbeFond;

static void avplan(void)
{
register short i;
short num;
Rect r;
CGrafPtr gbak;
long nb;
float y;

/* Premier passage */
 if(TabIndexHerbeFond==NULL)
	{
	AcamGetPortOffscreen(&gbak);
	AcamSetPortOffscreen(Ecran1);
	TabIndexHerbeFond=CalculePaletteAttenuee(MOB_HERBE,0.75,999);
	AcamSetPortOffscreen(gbak);
	}
 /*on fixe le Rnd*/
 initrnd(xj/256,yj/256,(xj & 224) | ((yj & 224) /8) );
if(nbHerbes<=20)NbHerbeAvPlan=myRandom(0,nbHerbes);
else NbHerbeAvPlan=myRandom(0,nbHerbes/GRANULOSITE)*GRANULOSITE;

 /*on affiche le chemin*/
 for(i=0;i<8;i++)
            Affmob(MOB_CHEMIN+(rnd0()&3),LIGNE_CHEMIN,i*8*4,0);

 /*on affiche les arbres*/
nb=myRandom(NB_ARBRES_LATERAL,NB_ARBRES_LATERAL_MAX);
for(i=0;i<nb;i++) 
	{
	//arbre(78,1+i*63+rnd0()/4);
	y=68.0+10.0*(float)i/(float)nb;// de 68 a 78
	arbre((short)y,myRandom(0,250));
	}
// herbes arrieres plann
nb=myRandom(NB_HERBE_ARR_PLAN/GRANULOSITE,NB_MAX_HERBE_AR_PLAN/GRANULOSITE)*GRANULOSITE;
 /*affichage des herbes … l'arriere plan */
 for (i=0;i<nb;i++)
      {
      num=CalculeMobHerbe();
      CalculeAireMob(num,0,0,&r);
      //AffmobPalette(num,LIGNE_CHEMIN+12-(r.bottom/2)+(i/3),myRandom(0,256),rnd0()&1,TabIndexHerbeFond);
	  AffmobPalette(num,LIGNE_CHEMIN+12-(r.bottom/2)+myRandom(0,3),myRandom(0,256),rnd0()&1,TabIndexHerbeFond);
      }
 /*on remplit le tableau des herbes avplan*/
 for (i=0;i<NbHerbeAvPlan;i++)
      {
      num=CalculeMobHerbe();
      herbe[i].type=num;
      CalculeAireMob(num,0,0,&r);
      herbe[i].lin=LIGNE_MAX_HERBE_AV_PLAN-(NbHerbeAvPlan-i)*
                        (LIGNE_MAX_HERBE_AV_PLAN-LIGNE_BASE_HERBE_AV_PLAN)/NbHerbeAvPlan-(r.bottom/2)+32;
                        /*166*/
     // herbe[i].col=myRandom(0,220);
      herbe[i].col=myRandom(0,240);
      herbe[i].sens=rnd0() & 1;
      if (herbe[i].type)
                Affmob(herbe[i].type,
                                herbe[i].lin,/*98*/
                                herbe[i].col,
                                herbe[i].sens);
      }
/* L'avplan est recopie sur l'ecrna 2*/
CopyEcran1Sur2();
return;
}
/***********************************************************************
                        LATERB
***********************************************************************/
void laterb(Boolean TesteClavier)
{
unsigned long t=sapiensTimer;

effdemi();
mobs(TesteClavier);
tfrdemi();
GereInterface();
while(t>sapiensTimer-6)/**/;
}
/***********************************************************************
                        LATERA
***********************************************************************/
void latera(void)
 {
 /*affiche noms*/
 EffaceGenerique();
 generique(VRAI);

 dismin=50;
 affich((dzeta+256) & 0x3ff);
 /*  Mise en place 3d fn angle
                    .... avplan */
 avplan();
 tfrima();
 /* (c) God 7 Aout 87 Inits Souris */
 vq_mouse(&bouton,&dsposx,&dsposy);
 reset_clav();
 laterb(FAUX);
 tfrvirt();
 return;
 }
/***********************************************************************
                        INARME:Init des armes
***********************************************************************/
void inarme(void)
     {
     register short i;
     entvie=sap1.vie;
     sap1.forcehache=0;
     sap1.cross[FORME_ARME]=0;/*hache par defaut*/
     sap1.nbresag=0;

     /* Initialise l'agilit‚ active*/

     cptagil=1;
     i=liste1.taille;
     while (i)
          {
          i--;
          if (sap1.forcehache==0)
               {
               if (liste1.elements[i] == 22)/*massue*/
                    {
                    sap1.forcehache=10+(force*10)/256;
                    sap1.cross[FORME_ARME]=1;
                    }
               if (liste1.elements[i]==21)/*hache*/
                    {
                    sap1.forcehache=6+(force*6)/256;
                    }
               }
          if (liste1.elements[i] == 23) sap1.nbresag++;/*sagaie*/
          }
     }

/***********************************************************************
                        COMBAT:GŠre les combats
                On traite la sortie de FIN sur LATERB.
***********************************************************************/
void combat(void)
     {
     unsigned char tfr;
     Boolean quelqun=FAUX;

/* Inits Souris */
//      sposx=dsposx=160;
 //     sposy=dsposy=100;
      reset_mouse();
      if ( dervue !=VUE_LATERALE )/* ‚galement si presence humaine ??? ou ab‚ration:Collapse!!*/
          {
          /* Maquillage et habillage  des 'Bons'*/
          prm(&sap1,-1);
          prm(&sap3,present[2]);
          /* les machinistes montent le decor*/
          latera();
          }
     vq_mouse(&bouton,&dsposx,&dsposy);
     dervue=VUE_LATERALE;
     /* Sapiens une premiere ,Clap !*/
     clrbadi();

     INFINI
          {
          if (ModeJoueur2 && present[2]<5) sap3clav=1;
          else        sap3clav=0;
          inarme();       /* initialise les armes sap1,silence sur le plateau*/
          presence=juge();/* on initialise les adversaires reciproques*/
          /* Si le joueur est seul alors affichage du commentaire */
          if(sap2.vie==0 && sap4.vie==0)
            {
            if(quelqun)
                {
                presence=0;
                inimen(0);
                laterb(VRAI);
                /* On ouvre par defaut le menu'prends'*/
                InverseOption(0,2);
                s_action=3;
                inimen(4);
                return;
                }
            //comment(132);
            }
          else
            quelqun=VRAI;
          fin=0;   /* Moteurs !!!!*/
                 // ScrollComb(sens);
          while( ! fin )   laterb(VRAI);/*  ACTION !!!*/
          /* c'est parfait coupez */

          tfrene();/*on transfere l'enervement*/
          outarm();/*armes en sortie*/
          flcomb=0;
          flefba=0;
          fdialo=0;
          if (presence==-1) present[2]=0;
          if (fin==2)    /*clan 1 perdu*/
               {
               CopyEcran1Sur2();
               dervue=VUE_AUCUNE;
               /* combat ultime */
               if (scenar==5)   mort=0;
               else
                {
                   tfr=bless-100;
                   if (tfr>bless)
                                {  mort--;
                                   tfr=20;
                                }
                 bless=tfr;
                }
               afstat();
        		/* on a perdu */
               if (mort==0)
                    {
                    scenar=7;// Perdu Mort
                    sceveu(7);
                    presence=0;
                    inimen(0);
                    return;
                    }
               comment(81);
                                /*c'est un homme qui … gagn‚*/
               if (present[1]<5 && present[1])   vole();
               prm(&sap2,0);
               prm(&sap4,0);
               presence=0;
               attends_bruit();
               inimen(0);
               return;
               }
          if (fin==1) /*clan 2 perdu*/
               {
               presence=0;
               prm(&sap2,0);
               prm(&sap4,0);
               /* On a eut le chef des Hyene folle*/
               if (scenar==4 && chef && tribu==4)
                    {
                    scenpr=0;
                    sceveu(9);
                    }
                /* Scenario ultime */
               if (scenar==5)
                    {
                    mort=0;/*thats all folks*/
                    scenar=6;
                    jour_dsk++;
                    sceveu(6);/*bravo tu as gagn‚*/
                    presence=0;
                    inimen(0);
                    return;
                    }
               }
          /*si esc */
          if ( ((fin==129) && (!presence)) || !mort)
               {
               if (!sap1.vie) dervue=255;
               presence=0;
               inimen(0);
               return;
               }
          /*Si sortie de l'ecran (nous sommes donc seuls))*/
          if (fin==128)
               {
               clrbadi();
               deplat();
               zoninf();
               if ( presence && presence<5)
                    {
                    prm(&sap1,present[0]);
                    prm(&sap3,present[2]);
                    latera();
                    dervue=VUE_LATERALE;
                    inimen(0);
                    return;
                    }
               fixcolon(&sap1);
               fixcolon(&sap3);
               prm(&sap3,present[2]);
               latera();
               }
          } /*infini .... et un peu plus*/
     }
/*--- Fin de Texte ---*/
