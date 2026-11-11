/*************************V1.6 Mardi 9 Dec**********************************
                         Module #1 :Projet SAP
              Gestion de Listes:Le mardi 4 Novembre 1986
**************************************************************************/
#include "sapiens.h"

short pal1[32]={    0x006,0x542,0x650,0x777,
                     0x430,0x310,0x320,0x210,
                     0x005,0x432,0x443,0x444,
                     0x322,0x332,0x211,0x000};
short pal2[32] ={    0x777,0x000,0x700,0x320,
                     0x430,0x540,0x650,0x531,
                     0x631,0x641,0x444,0x050,
                     0x070,0x770,0x450,0x005};

short pal_zero[32]={ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
short contrl[12],intin[128],ptsin[128],intout[128],ptsout[128];
short work_out[57],work_in[57];
struct liste * sliste[]={&liste0,&liste1};
/***********************************************************************
                InverseOption
***********************************************************************/
static short lastMenuInvers=-1,lastn;
static Rect rInvert;
void InverseOption(short menu,short n)
{
eff_sour();
if(lastMenuInvers>=0)
    {
    SwapCouleur(debecr,&rInvert,TabCvtCoul[palbasse[4]],TabCvtCoul[palbasse[1]]);
    TfrEcr(&rInvert);
    lastMenuInvers=-1;
    }
if(menu>=0)
    {
    SetRect(&rInvert,tccol [menu]*6*ECHELLE,
           (tclin [menu]-tlpos[menu]+1+n)*7*ECHELLE-1,
           (tccol [menu]+tcncol [menu])*6*ECHELLE,
           (tclin [menu]-tlpos[menu]+1+n+1)*7*ECHELLE-1);
    SwapCouleur(debecr,&rInvert,TabCvtCoul[palbasse[1]],TabCvtCoul[palbasse[4]]);
    TfrEcr(&rInvert);
    }
lastMenuInvers=menu;
lastn=n;
return;
}
/***********************************************************************
              ListAttendsFinBouton
**************************************************************************/
static void ListAttendsFinBouton(void)
{
do
        vq_mouse(&bouton,&sposx,&sposy);
        while(bouton);
}
/***********************************************************************
              AJOUTL:ajoute un element a une liste
**************************************************************************
=>debut de la liste,element … ajouter
<=0 si ok,1 si liste pleine
*/
unsigned short ajoutl(struct liste* debliste,unsigned short element)
{
if (debliste == &listbidon )
     (debliste->elements[0])=element;/*le premier element le recoit*/
else
          {
          /*liste est elle pleine*/
          if ((debliste->taille)==(debliste->maxtaille))
          return(1);/*deja pleine*/
          else
               {
               (debliste->elements[debliste->taille])=element;
               ++(debliste->taille);
               }/*si liste pas pleine*/
          } /*si liste pas bidon*/
     return(0);/*ok...*/
     }
/**************************************************************************
                ENLEVL:transfere un element d'une liste sur une autre
*************************************************************************
=> Liste d'origine,liste de destination,rang de l'elt [0,taille[
<= 0=ok,1:elt est le dernier,255=errreur (liste en reception pleine)
*/
unsigned short enlevl(struct liste *orgliste,struct liste *receptliste,unsigned short rang)
     {
     unsigned char erreur,i;
     if (rang < (orgliste->taille) ) /*rang contenu dans liste de depart*/
          {
          (orgliste->taille)--;/*la taille diminue*/
          erreur=ajoutl(receptliste,orgliste->elements[rang]);
          if (erreur) /*pas de place en reception*/
               {
               (orgliste->taille)++;/*on remets la taille*/
               return(1);/*erreur*/
               }
          else /*transfert ok*/
               {
               /*on recompacte la liste d'origine*/
               i = rang;
               while (i != (orgliste->taille))
                    {
                    orgliste->elements[i]=orgliste->elements[i+1];
                    i++;
                    }
               if(i==rang)/*i n'a pas boug‚*/
                        return(1);/*fin de liste*/
               else
                        return(0);
               }
          }
     else
     return (255);/*erreur de logique rang pas ds la liste de depart*/
     }

/***************************************************************************/
/*              MOT:Affiche un mot selon la reference std                  */
/***************************************************************************/
/*=> on pointe aver refer sur l'elt a afficher
*/
static void mot( char * refer)
{
unsigned char * p=(unsigned char *) refer;

while (*p)  affcar(*(p++));
return;
}
/***************************************************************************/
/*              AFFMOT:affiche le mot ou des blancs                        */
/***************************************************************************/
/*=>liste a traiter,position a afficher
(les variables globales colaff et linaff contiennent la position-‚cran)
*/
static void affmot(struct liste *debliste,unsigned short rang)
{
struct reference * refer;

if (rang < (debliste->taille))/*rang a afficher existe t-il ?*/
          {
          /*on affiche l'element de rang 'rang' du tableau 'refer'*/
          /*donc on envoit l'origine de l'element de reference corrsepondant …
        l'‚l‚ment de rang 'rang' dans la liste … une routine qui extrait
        le nom de la reference et les deux parametres*/
			refer=GetRefer(debliste->typeRef);

          mot((Ptr)&(refer[debliste->elements[rang]-1]));


          /*                               ^ extrait le rang de la ref ^
                  ^ prend l'element dans le tableau des references
                ^pointe sur l'elt.
           */
          }
     /*on termine le mot avec des blancs*/
     return;
     }
/**************************************************************************
                AFLIST:Affiche liste en entr‚e
**************************************************************************
=> liste
<=liste sur ecran
*/
static void aflist(struct liste *debliste)
 {
 unsigned char i,travl2;
 short j;
 Rect r;
 CGrafPtr gbak;

 AcamGetPortOffscreen(&gbak);
 AcamSetPortOffscreen(debecr);
 if(s_menu==lastMenuInvers)
        lastMenuInvers=-1;
 /*ligne et colonne au debut du cadre*/
 Gotoxy(ccol,clin);
 i=cnlin;/*i recoit le nombre de ligne du cadre*/
 /*test validit‚ lpos*/
 if (lpos==0) lpos=1;
 /*on regarde si la position en liste est > … la position max*/
 if (lpos > debliste->taille)
 	lpos=debliste->taille;/*c'est ds le cas des listes reduites*/
 travl2=lpos-1;/*debut d'affichage en lpos-moitie du cadre*/
 /* On efface le fond de la liste */
 SetRect(&r,colaff*6*ECHELLE,linaff*7*ECHELLE-1,
            (colaff+cncol)*6*ECHELLE,(linaff+cnlin)*7*ECHELLE);
 BoxFill(debecr,&r,0x542);
 /* On affiche tout les mots */
 j=0;
 if(debliste->taille)
    {
    /* On n'est pas en haut de la liste, fleche vers le haut  */
    if(travl2>1)
    	{
    	Gotoxy(ccol+cncol-1,linaff);
		affcar('<');
 		Gotoxy(ccol,clin);
    	}
     while (i--)
      {
      affmot(debliste,travl2++);/*on affiche le mot*/
      Gotoxy(ccol,linaff+1);
      }
     /* On n'est pas en haut de la liste, fleche vers le haut  */
    if(travl2<debliste->taille)
    	{
    	Gotoxy(ccol+cncol-1,linaff-1);
		affcar('>');
    	}
   }
else
    {
    /* Liste vide */
    mot("...");
    }
 /* Transfert de la liste sur l'ecran */
 InsetRect(&r,-48,-48);
 TfrEcr(&r);
 AcamSetPortOffscreen(gbak);
 return;
 }
/***********************************************************************
                        ENCADRE:dessine le cadre
***********************************************************************/
static void  encadre(unsigned short ligne,unsigned short colon,unsigned short haut,unsigned short larg)
 {
 short i,j;
 short tailleh,taillev;
 /* passage au pixel */

 ligne *=7;
 colon *=6;
 haut =ligne+7*haut;
 larg =colon+6*larg;
 /* petit delta pour polymorphisme*/
 ligne -=(colon & 3);
 colon =colon-2-(haut&3);
 haut +=larg &3;
 larg +=ligne&3;

 /* coin haut gauche vertical */
 affomb(MOB_CADRE+0,ligne-24,colon-8,8,8,FAUX);
 /* coin haut droit vertical */
 affomb(MOB_CADRE+1,ligne-24,larg-8,8,8,FAUX);
 /* coin haut gauche horizontal */
 affomb(MOB_CADRE+2,ligne-16,colon-16,8,8,FAUX);
 /* angle  haut gauche */
 affomb(MOB_CADRE+3,ligne-8,colon-8,8,8,FAUX);
 /* ligne horiz haute */
 tailleh=(larg-colon-1)/16;
 if (tailleh<2000)
 for (i=0;i<tailleh;i++) affomb(MOB_CADRE+4,ligne-8,colon+8+i*16,8,8,FAUX);
 /* angle haut droit */
 affomb(MOB_CADRE+5,ligne-8,larg-8,8,8,FAUX);
 /* coin haut droit horizontal */
 affomb(MOB_CADRE+6,ligne-16,larg+8,8,8,FAUX);
 /* Block vertical */
taillev=(haut-ligne-1)/16;
if(taillev<2000)
 for (i=0;i<taillev;i++)
      {
      /* cote gauche */
      affomb(MOB_CADRE+7,ligne+8+16*i,colon-8,8,8,FAUX);
      /* centre */
      if (tailleh<2000)
      for (j=0;j<tailleh;j++) affomb(MOB_CADRE+16,ligne+8+16*i,colon+8+16*j,8,8,FAUX);
      /* cote droit */
      affomb(MOB_CADRE+8,ligne+8+16*i,larg-8,8,8,FAUX);
      }
 /* coin bas gauche horizontal */
 affomb(MOB_CADRE+9,haut-8,colon-16,8,8,FAUX);
 /* angle  bas gauche */
 affomb(MOB_CADRE+10,haut-8,colon-8,8,8,FAUX);
 /* ligne horiz basse */
 if (tailleh<2000)
 for (i=0;i<tailleh;i++) affomb(MOB_CADRE+11,haut-8,colon+8+i*16,8,8,FAUX);
 /* angle bas droit */
 affomb(MOB_CADRE+12,haut-8,larg-8,8,8,FAUX);
 /* coin bas droit horizontal */
 affomb(MOB_CADRE+13,haut-8,larg+8,8,8,FAUX);
 /* coin bas gauche vertical */
 affomb(MOB_CADRE+14,haut+7,colon-16,8,8,FAUX);
 /* coin bas droit vertical */
 affomb(MOB_CADRE+15,haut+7,larg,8,8,FAUX);



return;
}

/************************************************************************/
/*              GEMOUVRE:Transfere le cadre avec effet sp‚cial          */
/************************************************************************/
/*  ouvre un cadre de cad1 vers cad2 (64 colonnes * 25 Lignes)*/
/* le nombre de cadre 'GEMNBRECAD' doit etre pair*/
void gemouvre(unsigned short ligncad1,unsigned short colcad1,unsigned short hautcad1,
              unsigned short largcad1,
              unsigned short ligncad2,unsigned short colcad2,
              unsigned short hautcad2,unsigned short largcad2)

     {
     short ligne,colon,haut,larg;
     register short balai,i;
     /*calcul des parametres de depart*/
     ligne=ligncad1*7/2;
     colon=colcad1*3/2;
     haut=hautcad1*7/2;
     larg=largcad1*3/2;
     /* on l'injecte ds les N cadres*/
     for (i=0;i < gemnbrecad ;i++)
          {
          gemzoom[i].ligne=ligne;
          gemzoom[i].colon=colon;
          gemzoom[i].haut =haut ;
          gemzoom[i].larg =larg ;
          }
     /* on calcule les coordonn‚es d'arriv‚e*/
     ligne=ligncad2*7/2;
     colon=colcad2*3/2;
     haut=hautcad2*7/2;
     larg=largcad2*3/2;
     /* on y  va*/
     balai=0;
     while(1)
          {
          for ( i = 0;i <= balai/geminterv;i++)
               {
               /* on efface le cadre*/
               cadreg(gemzoom[i].ligne,gemzoom[i].colon,gemzoom[i].haut,gemzoom[i].larg);
               /*on calcule le nlles coordonn‚es*/
               if(gemzoom[i].ligne<ligne)        gemzoom[i].ligne++;
               else if (gemzoom[i].ligne> ligne) gemzoom[i].ligne--;

               if(gemzoom[i].colon<colon)        gemzoom[i].colon++;
               else if (gemzoom[i].colon> colon) gemzoom[i].colon--;

               if(gemzoom[i].haut <haut )        gemzoom[i].haut ++;
               else if (gemzoom[i].haut > haut ) gemzoom[i].haut --;

               if(gemzoom[i].larg <larg )        gemzoom[i].larg ++;
               else if (gemzoom[i].larg > larg ) gemzoom[i].larg --;
               /*affiche le cadre*/
               cadreg(gemzoom[i].ligne,gemzoom[i].colon,gemzoom[i].haut,gemzoom[i].larg);
               }
          /*si tout les cadres en cours alors balai ne bouge plus*/
          if (balai/geminterv+1 < gemnbrecad) balai++;
          /*fini ????: dernier cadre arriv‚*/
          if (gemzoom[gemnbrecad-1].ligne==ligne &&
          gemzoom[gemnbrecad-1].colon==colon &&
          gemzoom[gemnbrecad-1].haut ==haut  &&
          gemzoom[gemnbrecad-1].larg ==larg )
               {
               return;
               }
          }

     }
/***************************************************************************
                INIMEN:Initialise Menu
**********************************************************************
=> Rang du menu;(lastmen) dernier menu autre que menu0

*/
void inimen(unsigned short entmenu)
     {
     unsigned char rang,valeur;
     register short i=0,j=0;
     long t=TickCount();
      struct reference* refer1=GetRefer1();

     menu=entmenu;/*force variable globale*/
     /*transfert des parametres*/
     clin=tclin [menu];
     ccol=tccol [menu],
     colcdr=tcolcdr [menu];
     cnlin=tcnlin [menu];
     cncol=tcncol [menu];
     poscur=tposcur [menu];
     stdlist=tstdlist[menu];
     eff_sour();
     if (menu !=0) /*pas menu 0*/
          {
          smenu[1]=menu;
          if (menu !=lastmen) /*deja affich‚ ?*/
               {
               gemouvre(gemaction,clin,ccol,cnlin,cncol);
               encadre(clin,ccol,cnlin,cncol);
               lastmen=menu;
               }
          }
     else
          {
          criter=0;/*si le menu demand‚ est le menu principal
                                   alors on est sur que le critere=0*/
          if (presence && presence<5) liste0.taille=6;
          else                        liste0.taille=11;
          if (mort==0)
               {
               /* On demarre a zero */
               tlpos[0]=1;
               liste0.taille=2;
               clrbadi();
               comment(136); /* La partie est fini */
               }
          }
     lpos=tlpos[menu];
     /*reduction du nbre d'elements*/
     if (criter) /*critere retenu*/
          {
          /*transfere en testant,i=0*/
          i=j=0;
          lpos=111;
          while(i !=(stdlist->taille))
               {
               rang=stdlist->elements[i];/*rang de l'objet*/
               valeur=(refer1[rang-1].param1) & 0xF0;
               /*valeur de l'objet selon reference*/

               if (  ( (criter==1)&&(valeur==0x20)) ||
                  (  (criter==2)&&(valeur==0x40) )  )
                    {
                    zoneu.elements[j]=rang;
                    cross.elements[j]=i;
                    j++;
                    if (i>=tlpos[menu] && lpos==111) lpos=j;
                    }/* si Objet correspond au critere*/
               i++;
               }/* on balaye la liste originelle*/
          if(lpos==111) lpos=1;
          zoneu.taille=j;/*transfere taille*/
          stdlist=&zoneu;
          /* nlle position ds menu*/
           lpos=tlpos[menu];
          }
     /*affichage de la liste*/
     if (menu)
               {
               sliste[1]=stdlist;
               smenu[1]=menu;
               }
     s_menu=menu;
     aflist(stdlist);
     while(TickCount()<t+6)/**/;
     return;
     }
/***********************************************************************
                AFSTAT:affiche menu status
***********************************************************************/
void afstat(void)
     {
	 struct reference* refer3=GetRefer3();
     refer3[7].nomref[0]=((faim/64)+0x91);
     refer3[8].nomref[0]=((soif/64)+0x91);
     refer3[9].nomref[1]=((fatig/64)+0x95);
     refer3[10].nomref[1]=((bless/64)+0x9A);
     refer3[11].nomref[1]=(mort+0xA7);
     inimen(7);
     s_action=2;
     return;
     }
/**********************************************************
                     POS0_MOUSE
        Remets la souris en pos 0
*********************************************************/
void pos0_mouse(void)
{
reset_mouse();
return;
}
#pragma mark ___  Mouse wheel
/*******************************************************************
                       CarbonMouseWheel
********************************************************************/
static pascal OSStatus CarbonMouseWheel(EventHandlerCallRef inHandlerCallRef, EventRef event, void *inUserData)
{
long wheelDelta;

GetEventParameter(event,kEventParamMouseWheelDelta,typeLongInteger, NULL, sizeof(long), NULL, &wheelDelta);
if (wheelDelta)
    {
	printf("%ld\n",wheelDelta);
	if(wheelDelta<0) icone=UP;
	else icone=DOWN;
	isScrollMouse=true;
	}
return(eventNotHandledErr);
}
/*******************************************************************
                       InitMouseWheel
Iinstallation du gestionnaire de wheel mouse pour les listes
********************************************************************/
static EventHandlerUPP handlerMouse;
static EventHandlerRef mouseWheel; /* Vecteur de traitement du mouse weel */
static void InitMouseWheel(WindowPtr window)
{
OSErr err;
EventTypeSpec    eventTypes[2];
EventTargetRef target;

eventTypes[0].eventClass = kEventClassMouse;
eventTypes[0].eventKind  = kEventMouseWheelMoved;
if(handlerMouse==NULL)
		handlerMouse=NewEventHandlerUPP(CarbonMouseWheel);
target=GetWindowEventTarget(window);
if(target)
	err=InstallEventHandler(target,handlerMouse,1,eventTypes,(void*)NULL,&mouseWheel);
return;
}
/********************************************************************
                     GERE_SOURIS
*********************************************************************/
static void gere_souris(void)
{
short x,y,men;
register short i;

/*if(isScrollMouse)
	{
	isScrollMouse=false;
	bouton=true;
	s_menu=-1;
	x=(sposx+8)/6;
	y=(sposy+8)/7;
	for(i=0;i<2;i++)
	     {
	     men=smenu[i];
	     if ((men !=111) && (x>=tccol[men]) && (x<=tccol[men]+tcncol[men]) )
			   {
			   s_menu=men;
			   stdlist=sliste[i];
			   }
	      }
	return;
	}
*/
/* affiche souris et determine coordonnees */
bouge_souris();
if(sposy<105)
	{
	icone=0;
	eff_sour();
	}
else
	{
	/* calcul position 53 du centre de la souris */
	x=(sposx+8)/6;
	y=(sposy+8)/7;
	/* calcul de l'icone (par defaut rien) */
	icone=INCONNU;
	s_menu=-1;
	/* on balaye les menus */
	for(i=0;i<2;i++)
	     {
	     men=smenu[i];
	     /*on teste les verticales,men  111 implique pas de sousmenu */
	     if ((men !=111) && (x>=tccol[men]) && (x<=tccol[men]+tcncol[men]) )
			   {
			   s_menu=men;
			   stdlist=sliste[i];
			   /* on teste les horizontales */
			   if (y<tclin[men]) 
					icone=UP;
			   else
					if (y>tclin[men]+tcnlin[men]) 
						icone=DOWN;
					else                          
						icone=SELECT;
				}
	      }
}
return;
}
/***********************************************************************
                CLAVM:Gestion des menus au clavier
***********************************************************************/
static void clavm(void)
{
short y;
struct reference * ref;
Point pt;
Rect r;

/* gestion souris pour clavm */
aff_sour(icone,sposy,sposx);
while (1)
     {
     /* appel de grise mine */
     gere_souris();
     /* teste si bouton appuye*/
     if(bouton)
     	{
     	if(icone != INCONNU && icone !=0)
          	{
	         /* calcul des valeurs par defaut pour la liste selectionne*/
	         /* deplacement dans la liste */
	        if ( (icone==UP) && (tlpos[s_menu]>1) )
	               {
	                eff_sour();
	                tlpos[s_menu]--;
	                inimen(s_menu);
	                aff_sour(icone,sposy,sposx);
	                }
	          if ( (icone==DOWN) &&
	               (tlpos[s_menu]<=(char)(stdlist->taille-tcnlin[s_menu]) ) )
	               {
	               eff_sour();
	               tlpos[s_menu]++;
	               inimen(s_menu);
	               aff_sour(icone,sposy,sposx);
	               }
	          /* si selection et liste non vide */
	          if ( (icone==SELECT) && (stdlist->taille))
	               {
	                eff_sour();
	               /* calcul de l'option selectionn‚e */
	               /* calcul position 53 du centre de la souris */
	               y=sposy/7;
	               /* calcul de l'option choisie*/
	               rega=y-tclin[s_menu]+tlpos[s_menu]-1;
	               if (rega<stdlist->taille)
	                    {
						struct reference * refer=GetRefer(stdlist->typeRef);
	                    /*mise en place de stock1 et stock2*/
	                    ref=&(refer[stdlist->elements[rega]-1]);
	                    stock1=ref->param1;
	                    stock2=ref->param2;
	                    /*on affiche l'option choisie*/
	                    efbadi();
	                    if ( !s_menu) s_action=rega+1;
	                    comment(120+s_action);
	                    InverseOption(s_menu,rega);
	                  /* si sous menu affichage et sortie*/
	                    if (s_menu)
	                         {
							 struct reference * refer=GetRefer(sliste[1]->typeRef);

	                         affcar(' ');
	                         ref=&(refer[sliste[1]->elements[rega]-1]);
	                         mot((Ptr)ref);
	                         TfrBadAss();
	                         if (criter) rega=cross.elements[rega];
	                         return;
	                         }
	                    else
	                    /* si menu principal */
	                    /* on affiche (ou reaffiche) le sous-menu*/
	                         {
	                         criter=stock2;
	                         if(criter)
	                                {
	                                tlpos[1]=1;
	                                }
	                         if (stock1==7)
	                                {
	                                if(dervue != VUE_LATERALE) primjauge(0);
	                                afstat();
	                                }
	                         else inimen(stock1);
	                         }
	                    sposx++; /* pour affichage */
	                    vq_mouse(&bouton,&sposx,&sposy);
	                  }
	                aff_sour(icone,sposy,sposx);
	               }
          	}
     	else
     	/* La souris est dans la partie haute de l'ecran */
     	if(icone==0)
     	  	  {
	     	  /* Bouton et pas d'icone */
	     	  GetMouse(&pt);
			  pt.h/=SapZoom;
			  pt.v/=SapZoom;

	     	  SetRect(&r,32*ECHELLE,1*ECHELLE,(32+255)*ECHELLE,(1+100)*ECHELLE);
	     	  /* Il n'y a personne et le click est dans l'aire */
	     	  if(!presence && PtInRect(pt,&r))
		     	  	{
		     	  	/* Click dans l'aire de l'image */
		     	  	/* On ouvre par defaut le menu'voir'*/
		            InverseOption(0,6);
		            s_action=7;
		            inimen(3);
		            /* Demande le traitement */
					rega=0;
					return;
		     	  	}
		     ClickInGenerique(pt);
		     }
		     
	     }
	 }
}
/************************************************************************
                        TRPOSE:Pose objet
**********************************************************************
=>(rega)et (menu)
*/
void trpose(void)
     {
     unsigned char erreur;

     erreur=enlevl(&liste1,&liste4,rega);/*on transfere*/
     if (erreur==1 && lpos>1) lpos--; /*elt de rang rega est en fin de liste*/
     inimen(s_menu);
     return;
     }
/***********************************************************************
                        TRPREN:Prends Objet
***********************************************************************/
# define eau 2
# define outrevide 26
# define outrepleine 27
void trpren(void)
     {
     unsigned char quoi=0;
     register short i;
     short old_taille;

     if ((liste4.elements[rega])==eau)
          {
          flefba=0;
          if (soif==255)  comment(98);/*tu n'as pas soif*/
          else            comment(96); /*"Tu bois "*/
          soif=255;
          /*on remplit les outres*/
          for (i=0;i !=liste1.taille;i++)
               {
               if (liste1.elements[i]==outrevide)
                    {
                    liste1.elements[i]=outrepleine;
                    quoi++;
                    }
               }
          if (quoi) comment(97); /*.. et remplis tes gourdes */
          flefba = 0;
          return;
          }
     old_taille=liste1.taille;
     enlevl(&liste4,&liste1,rega);
     if (liste1.taille==old_taille)
                {
                flefba=0;
                comment(101);
                }
     else
        if ((liste4.elements[rega])==44) scenpr=1; /*hache sacree*/

     inimen(s_menu);
     return;
     }

/***********************************************************************
                        TRABSO:Absorbe
***********************************************************************/
void trabso(void)
     {
     unsigned char calcul,objet;
     if ((faim>192) && (soif >192))
          {
          flefba=0;
          comment(107);
          return;
          }

     calcul=faim+(stock2 & 0xF0)/4;

     if (calcul>=faim) faim=calcul;
     else             faim=255;
     calcul=soif+(stock2 & 0x0F)*4;

     if (calcul>=soif) soif=calcul;
     else             soif=255;


     objet=liste1.elements[rega];
     if ((objet)==outrepleine)
     liste1.elements[rega]=outrevide;
     else
     enlevl(&liste1,&listbidon,rega);

       inimen(1);

     return;
     }
/***********************************************************************
                        TRSOIN:Soigne
***********************************************************************/
void trsoin(void)
     {
     unsigned char calcul;
     if (bless>192)
          {
          flefba=0;
          comment(109);
          return;
          }

     calcul=bless+((stock2 & 0x0F)*10);
     if (calcul > bless)   bless=calcul;
     else           bless=255;

     enlevl(&liste1,&listbidon,rega);
     inimen(1);

     return;
     }


void DoAide(void)
{
char chaine[255];

PushPath();
SetPathInData("docs");
GetNomCheminCourant(chaine);
if(langue==FR) strcat(chaine,"sapiens_fr.html");
else if(langue==ES) strcat(chaine,"sapiens_es.html");
else strcat(chaine,"sapiens_gb.html");
#ifdef PC
if(AcamOpenDocument(chaine)!=noErr)
#else
if(OpenDocument(chaine)!=noErr)
#endif
    Collapse("Can't open manual.");
PopPath();
}

/************************************************************************
                INITPRIM:inits primaires
***********************************************************************/
void initprim(void)
{
Boolean firstInit;

dsposx=sposx=60;
dsposy=sposy=160;
/* toute Palettes a zero*/
palhaute=pal_chron=palbasse=ciel_degr=pal_zero;
firstInit=init_graph();
hires();
xj=0x5800 ; yj=0x5400 ;dzeta=768;
flefba=0;
/*test version*/
flefba=0;
primsap();
sap1.colonne=200;
present[0]=-1;
prm(&sap1,-1);
prm(&sap2,0);
prm(&sap3,0);
prm(&sap4,0);
framebadass();
palhaute=pal_chron=pal2;
palbasse=pal1;
/* Mise en place version */
DebutAventure(true);
afstat();
gemouvre(1,1,23,62,gemaction);
encadre(tclin[0],tccol[0],tcnlin[0],tcncol[0]);/*cadre menu 0*/
inimen(0);
//if(firstInit) DoAide();
return;
}
/***********************************************************************
                ACTION:Effectue une action
***********************************************************************/
static void action (void)
     {
     switch (s_action)
          {
          case 1:
                 trgest(rega);
                 inimen(0);
                 break;
          case 8:
                trabso();
                ListAttendsFinBouton();
                break;
          case 4:
                trpose();
                ListAttendsFinBouton();
                break;
          case 3:
                trpren();
                ListAttendsFinBouton();
                break;
          case 9:
                trsoin();
                ListAttendsFinBouton();
                break;
          case 5:
	          	if(presence)
	          		{
					struct reference * refer=GetRefer(sliste[1]->typeRef);
	          		if(dervue!=VUE_LATERALE) tfrima();
    	            bulle((Ptr)&(refer[sliste[1]->elements[rega]-1]),0);
        	        trdire(liste2.elements[rega]);/*on envoie la phrase*/
	                ListAttendsFinBouton();
	                }
	             else
	             	{
	             	/* Tu parles tou seul ... */
	             	clrbadi();
	             	comment(135);
	             	}
                break;
          case 6:
                trdonn(rega);/*on envoit le rang de l'objet*/
                ListAttendsFinBouton();
                break;
          case 7:
                trobsv(rega);
                break;
          case 10:
                MenuFenetres(FAUX);
                trdors();
                MenuFenetres(VRAI);
                break;
          case 11:
                MenuFenetres(FAUX);
                trfab(rega);
                MenuFenetres(VRAI);
                break;
          }
     }
/***********************************************************************
               AffichePictAide
=> 0 vue laterale, 1 paysage               
***********************************************************************/
void AffichePictAide(short mode)
{
Rect r;
CGrafPtr gbak;
short num;

AcamGetPortOffscreen(&gbak);
CarboniseSetPortWindow(mainWindow);
if(mode==0)
	{
	SetRect(&r,320-125,230,320+125,230+160);
	num=131;
	}
else
	{
	SetRect(&r,32,230,32+250,230+160);
	num=132;
	}
AffichePictDansAire(num,&r);
AcamSetPortOffscreen(gbak);
return;
}
/***********************************************************************
               TraiteVueLaterale
***********************************************************************/
void TraiteVueLaterale(void)
{
MenuFenetres(FAUX);
efbadass();
TfrBadAss();
bruit(9);
/* On affiche le rectangle d'aide */
AffichePictAide(0);
effaceBulles();
combat();
pos0_mouse();
//inimen(0);
MenuFenetres(VRAI);
return;
}

/************************************************************************/
int main(void)
     {
	
#if defined(XCODE) && defined (DEBUG) 
FixeChemin(0,"MacPro:Developpement:Sapiens.src");
#endif

     initprim();/*inits primaires*/

  //Final();
		//InitMouseWheel(mainWindow);
	//	DEF3D();
      INFINI
               {
               clavm();
               action();
               if (flcomb) TraiteVueLaterale();
               }    /*infini...et meme un peu plus */
      }
/*---- FIN -----*/

