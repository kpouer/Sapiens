/*
 Prototypage pour /Developpement/Sapiens.src/Sources//saplist.c : Mardi  4 avril 2017 11:52:58
 */
void InverseOption(short menu,short n);
unsigned short ajoutl(struct liste* debliste,unsigned short element);
unsigned short enlevl(struct liste *orgliste,struct liste *receptliste,unsigned short rang);
void gemouvre(unsigned short ligncad1,unsigned short colcad1,unsigned short hautcad1,
              unsigned short largcad1,
              unsigned short ligncad2,unsigned short colcad2,
              unsigned short hautcad2,unsigned short largcad2);
void inimen(unsigned short entmenu);
void afstat(void);
void pos0_mouse(void);
void trpose(void);
void trpren(void);
void trabso(void);
void trsoin(void);
void DoAide(void);
void initprim(void);
void AffichePictAide(short mode);
void TraiteVueLaterale(void);
int main(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//saplist.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapglob.c : Mardi  4 avril 2017 11:52:58
 */
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapglob.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sap3d.c : Mardi  4 avril 2017 11:52:58
 */
long AjouteObjet3d(long nbObj);
void DeplaceHommes(void);
void GenereHommes(void);
void DefautDecor3d(void);
void InitDecor3d(void);
void CreeDecor3d(Boolean display);
void Final(void);
void ChargeEltsDecorsMobiles(FILE * f);
void SauveEltsDecorsMobiles(FILE * f);
void affich(unsigned short tfr);
void clav3d(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sap3d.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//saprel.c : Mardi  4 avril 2017 11:52:58
 */
void efbadi(void);
void scrollbadi(void);
void clrbadi(void);
void sceveu(unsigned short quoi);
void SceveuBulle(unsigned short quoi);
void gainsbour(void);
void pastmp(void);
void tfrene(void);
void comment(unsigned short rang);
void effaceBulles(void);
void bulle(Ptr debut,short qui);
void chrono(short quoi);
void fuite(void);
unsigned short intrsq(unsigned short objet);
unsigned short rapias(void);
void excite(unsigned short valeur);
void modrel(void);
void prends(void);
void trdonn(unsigned short rangb);
void idialo(void);
void quitte(void);
void creecop(unsigned short qui);
void bonnechass(void);
void viensavec(void);
void tuveuquoi(void);
void trdire (unsigned short diquoi);
void vole(void);
void outarm(void);
void trdors(void);
void trobsv(unsigned short regb);
short zonevt(unsigned short okmess);
void  zoninf(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//saprel.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapgest.c : Mardi  4 avril 2017 11:52:58
 */
void cat(void);
void trapsure(short quoi);
void DebutAventure(Boolean isFirst);
void primsap(void);
void primjauge(short jaugeCredit);
void defsap(void);
void DEF3D(void);
void CarboniseCvtPath(char * chaine);
short SetPath(char * libPath,long refNum);
OSErr FixeVolume(short volRefNum,Boolean afficheErreur);
OSErr FixeVolPath(short volRefNum,long path);
void PreservePath(void);
void RestorePath(void);
void SetPathApplication(void);
Boolean SaisitChemin(short * volRefNum,char * libPath,long * numPath);
void SaisitVolume(short * volRefNum);
OSErr FixeChemin(long pathBase,char * path);
OSErr FixeCheminRelatif(short volRef,long pathBase,char * path);
void InitCheminsSapiens(void);
void sauve(void);
void charge(void);
void trgest(unsigned short regb);
void trfab(unsigned short regb);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapgest.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapcomb.c : Mardi  4 avril 2017 11:52:58
 */
void decsou(struct sap * s,unsigned short combien);
void incsou(struct sap * s,unsigned short combien);
void posan(struct sap *sapiens,short anim);
void arret(struct sap *sapiens);
void posvit(struct sap *sapiens);
void pogne(struct sap *sapiens);
short juge(void);
void InitGenerique(void);
void generique(Boolean trace);
void ClickInGenerique(Point pt);
void abando(struct sap *s);
void decvie(struct sap *s,unsigned short valeur);
void touchesag(struct sap *s1,struct sap *s2);
void gersag(struct sap *s1);
void saut(struct sap *sapiens);
void touche(struct sap *s,struct sap *s2);
void iniune(struct sap *s);
void prm(struct sap *s,short type);
void baisse(struct sap *s);
void actbra(struct sap *s);
void deplat(void);
short mvtsap(struct sap *sapiens);
void decvit(struct sap * sapiens);
void incvit(struct sap * sapiens);
void clav(struct sap * s,short caract);
void gercop(struct sap * scop,short type);
void autonhom(struct sap * s2,struct sap * s1);
void bestio(struct sap * s2,struct sap * s1);
void laterb(Boolean TesteClavier);
void latera(void);
void inarme(void);
void combat(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapcomb.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwind.c : Mardi  4 avril 2017 11:52:58
 */
Boolean GetVersionSapiens(char * chaine);
void Collapse(char * format, ... );
Boolean Confirme(char * format, ... );
void MessInit(char * format, ... );
short SetValItem(DialogPtr dialog,short itemHit,char * c);
short SetShortItem(DialogPtr dialog,short itemHit,short val);
short SetValBouton(DialogPtr dialog,short itemHit,Boolean value);
short GetValBouton(DialogPtr dialog,short itemHit);
void SetActiveItem(DialogPtr dialog,short itemHit,Boolean value);
short GetValItem(DialogPtr dialog,short itemHit,char * chaine);
short GetShortItem(DialogPtr dialog,short itemHit,short * val);
void DefAfficheItem(DialogPtr dialog,short item,void pascal (*vect_affiche)(DialogPtr window,short itemid));
short GetNbItemInDialog(short id_dialogue);
void DisposeUserItems(DialogPtr dialog,short idDITL);
void CloseDialogue(DialogPtr  dialog,short idDITL);
short DoAlertSapiens(short num);
void GetRectItem(DialogPtr dialog,short item,Rect * box);
void DrawArrowPopUp(Rect * r);
char * GetStr(short index);
void initrnd(short sem1,short sem2,short sem3);
short rnd0(void);
short Rnd(void);
long myRandom(long min,long max);
void bande(short quelle);
void boxf(short lignedep,short coldep,short lignefin,short colfin,short index);
void irqon(void);
void cadreg(short ligne,short col, short nligne, short ncol);
short hires(void);
void AdjustMenus(void);
void FixeMenusReduit(void);
void FixeMenusComplet(void);
Boolean ConfirmePerteAventure(void);
void DoInfo(void);
void DoMenuCommand(long menuResult);
void delai(long duree);
void GereInterface(void);
Boolean FixeMenu(short n);
Boolean InitMobs(void);
void FixeTitreMainWindow(void);
void Frame3d(short etat);
void MenuFenetres(short etat);
void ValideUpdate(void);
DialogPtr OuvreDialogue(short numDitl);
void SetThemeDefaultSapiens(Collection collection);
void FixeTailleMainWindow(void);
void SapiensExit(short code);
void InitMainWindow(void);
Boolean init_graph(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwind.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwbrt.c : Mardi  4 avril 2017 11:52:58
 */
void DoZoomRect(Rect * rs,Rect *realDest);
void DoZoomRectSiEcranReel(Rect * rs,Rect *realDest);
void DoFrameRect(Rect * aire);
void DoEraseRect(Rect * aire);
void DoPaintRect(Rect * aire);
void DoPenSize(short x,short y);
void DoMoveTo(short x,short y);
void DoLineTo(short x,short y);
void DoLine(short x,short y);
void DoTextSize(short x);
void DoTextBox(char * text,short length,Rect * box,short just);
void DoScrollRect(
   Rect *  r,
  short         dh,
  short         dv,
  RgnHandle     updateRgn);
void CvtPathMacToPC(char * nom);
short FuturChargeFichierBrt(char * nom,short id,short nbMobs,short vide);
short ChargeFichierBrt(char * nom,short id,short nbMobs,short vide);
short * CalculePaletteAttenuee(short num,float coeff,short nbCoul);
void LibereDataMobs(void);
void CalculeAireMob(short num,short x,short y,Rect * r);
void Affmob3d(short num,Rect * aire,short * tabIndex);
void Affmobbrt(short num,short x,short y,short sens,short * tabIndex);
void AffmobCar(short num,short x,short y,short couleur);
void Affmobbrtomb(short num,short x,short y,short deltah,short deltav,Rect * r);
void PlotIndexPixBrt(CGrafPtr port,short x,short y,short coul);
short GetIndexPixBrt(CGrafPtr port,short x,short y);
void SwapCouleur(CGrafPtr port,Rect * r, short coul1,short coul2);
void BoxFill(CGrafPtr port,Rect * r,short couleur);
void RemplitLigneBrut(short y1,short y2,short index);
void RemplitZoneLignes(short y,short taille,IndexColorPtr tabCouleur);
void TraceDisque(short * dg,short * dd,short * dy,short fg,short fd,short epaisseur);
CGrafPtr SapiensCreeOffscreen(short taillex,short tailley);
void LibereOffscreen(CGrafPtr myGrafPort);
void CopyOffscreen(CGrafPtr source,CGrafPtr dest,Rect * rsourceIn,Rect * rdestIn);
void effdemi(void);
void tfrdemi(void);
void effima(short couleur);
void effvirt(short couleur);
void tfrima(void);
void tfrvirt(void);
void tfrmenufenetre(short etat);
void CopyEcran1Sur2(void);
void TfrEcr(Rect * r);
void Affmob(short mob,short y,short x,short inverse);
void AffmobConst(short mob,short y,short x,short inverse);
void AffDebecr(short mob,short y,short x,short transfere);
void affomb(short mob,short ligne,short colonne,short deltav,short deltah,Boolean recopie);
void AffmobPalette(short mob,short y,short x,short inverse,short * tabIndex);
void AffPalette(void);
void GetRectMainEcran(Rect * aireEcran);
void RetraceItem(DialogPtr dialog,short item);
void SetPathInData(char * name);
void AffichePictDansAire(short num,Rect * r);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwbrt.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwmus.c : Mardi  4 avril 2017 11:52:58
 */
void Kmuse(void);
void PushReg(void);
void PopReg(void);
void SortieMusiqueFichier(short quelle);
void bruit(short num);
void attends_bruit(void);
void quel_synthe(short n);
void reset_synthe(void);
void MusicInit(short quelle);
void CheckChangeMusique(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwmus.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwcvt.c : Mardi  4 avril 2017 11:52:58
 */
void FabriqueLuts(void);
void CvtCouleurStEnRgb(short val,RGBColor * rgb);
void CvtCouleurEnRgb(short * palette,short index,RGBColor * rgb);
short CvtRgbEnCouleur(short * palette,RGBColor * rgb);
short CvtIndexEnCouleur(short * palette,short index);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwcvt.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwmess.c : Mardi  4 avril 2017 11:52:58
 */
void TfrBadAss(void);
void efbadass(void);
void scrollbadass(void);
void framebadass(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwmess.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapw3d.c : Mardi  4 avril 2017 11:52:58
 */
short sinshort(short angle);
short cosshort(short angle);
short distance(long dx,long dy);
void visu2(MontPtr mont );
void sapdep(short angle,short vitesse);
void CalculeToutDegrades(void);
void ChangeIotaPalette(float combien);
void FixeDegradesFonctionHeure(struct h_chron * chron);
void CalculeEtoiles(void);
void AfficheEtoiles(void);
void AvanceSol(short dist);
void eff3d(unsigned char ciel, /* NU */
          unsigned char terre, /* NU */
          unsigned short ymeteore,
          unsigned short xmeteore,
          unsigned short isLune);
void Pluie(void);
void PosePluie(void);
void visu(MontPtr mont);
short ChaineObjet(MontPtr mont);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapw3d.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwcar.c : Mardi  4 avril 2017 11:52:58
 */
void InitCouleurFontes(void);
void Gotoxy(short x,short y);
void GotoxyPixel(short x,short y);
short TailleCar(short car,short taille);
char * GetStrBkMot(short car);
void affcar(short car);
void affcar10(short car);
void car2(short car);
void affcarmess(short car);
void motlst2(char *debut);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwcar.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwarb.c : Mardi  4 avril 2017 11:52:58
 */
void arbre(short ligne,short colonne);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwarb.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwcart.c : Mardi  4 avril 2017 11:52:58
 */
void carte(void);
void ChangeZoomCarte(short delta);
void zoocar(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwcart.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapwclav.c : Mardi  4 avril 2017 11:52:58
 */
void PushReg(void);
void PopReg(void);
void InstalleTimeTask( void );
short litclav(void);
void reset_mouse(void);
void reset_clav(void);
void CacheSouris(void);
void MontreSouris(void);
void vq_mouse(short * bouton,short * x,short * y);
void FixeAireNoMouse(Rect * r);
void FixeAireNoMouseAireBasse(void);
void wait_mouse(short * bouton,short * x,short * y);
void eff_sour(void);
void aff_sour(short mob,short ligne,short colonne);
Boolean KeyPressed(KeyMap * km,short ascii);
short GetKeyMap(short * bitmap);
Boolean IsKey(short ascii);
short getkey3d(void);
short getkey(unsigned short quoi);
void bouge_souris(void);
void AttendsFinBouton(void);
short joystick(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapwclav.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//SapWPref.C : Mardi  4 avril 2017 11:52:58
 */
void SetPathPreferences(void);
Boolean ChargePref(void);
void SauvePref(void);
void GetLibLangue(short n,char * str);
void DoPreferences(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//SapWPref.C */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//SapEnreg.C : Mardi  4 avril 2017 11:52:58
 */
unsigned short Code(unsigned short base);
Boolean IsEnreg(void);
Boolean SaisieEnregistrement(void);
void Desenregistre(void);
void NoEnreg(void);
void CheckEnreg(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//SapEnreg.C */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapbonc.c : Mardi  4 avril 2017 11:52:58
 */
void DoPrintBonCommande(void);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapbonc.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//sapscrollbar.c : Mardi  4 avril 2017 11:52:58
 */
short CommonAction(ControlHandle control,short amount);
void ClickInAscenseur(DialogPtr dialog,short item,Point pt);
ControlHandle CarboniseNewScrollBarStd(DialogPtr dialog,short item,short value,short min,short max);
ControlHandle CarboniseNewScrollBarWindow(WindowPtr window);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//sapscrollbar.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//SapColorCvt.c : Mardi  4 avril 2017 11:52:58
 */
short SapColor2Index(RGBColor * rgb);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//SapColorCvt.c */
/*
 Prototypage pour /Developpement/Sapiens.src/Sources//saptxt.c : Mardi  4 avril 2017 11:52:58
 */
struct reference * GetRefer0(void);
struct reference * GetRefer1(void);
struct reference * GetRefer2(void);
struct reference * GetRefer3(void);
struct reference * GetRefer(TypeReference type);
struct reference * GetReferLangue(short l,TypeReference type);
char ** GetMesCon(void);
char ** GetApp(void);
char ** GetBkMot(void);
char ** GetScenme(void);
short CvtLangRess(short l,short id);
short CvtDefaultLangRess(short id);
void InitAndCheckTexts(void);
short CvtCar(short car);
short CvtCarLissage(short car);
/* Fin des prototypes de /Developpement/Sapiens.src/Sources//saptxt.c */
