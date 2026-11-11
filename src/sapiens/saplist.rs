/*************************V1.6 Mardi 9 Dec**********************************
                         Module #1 :Projet SAP
              Gestion de Listes:Le mardi 4 Novembre 1986
**************************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::acam::{FAUX, TickCount, VRAI};
use crate::langue::langue;
use crate::model::Rect;
use crate::port::PortId;
use crate::sapiens::Sapiens;
use crate::sapiens::sapglob::{
    ListeId, SapId, Sapglob, tccol, tclin, tcncol, tcnlin, tcolcdr, tposcur,
};
use crate::sapiens::sapwind::{cadreg, hires};
use crate::sapiens::{ECHELLE, Vue, geminterv, gemnbrecad};
use crate::{DOWN, INCONNU, MOB_CADRE, SELECT, UP};
use log::warn;
use std::cell::RefCell;
use std::rc::Rc;
/***********************************************************************
                InverseOption
***********************************************************************/

pub(crate) struct InverseOption {
    LastMenuInvers: i16,
    Lastn: i16,
    rInvert: Rect,
}

impl Default for InverseOption {
    fn default() -> Self {
        Self {
            LastMenuInvers: -1,
            Lastn: 0,
            rInvert: Rect {
                top: 0,
                left: 0,
                bottom: 0,
                right: 0,
            },
        }
    }
}

impl Sapiens {
    pub(crate) fn InverseOption(&mut self, menusel: i16, n: i16) {
        self.eff_sour();
        if self.saplist.inverse_option.LastMenuInvers >= 0 {
            self.sapwind.ports.debecr.SwapCouleur(
                &self.saplist.inverse_option.rInvert,
                self.sapwcvt.TabCvtCoul[self.sapglob.palbasse.borrow()[4] as usize] as i16,
                self.sapwcvt.TabCvtCoul[self.sapglob.palbasse.borrow()[1] as usize] as i16,
            );
            self.TfrEcr(Some(self.saplist.inverse_option.rInvert));
            self.saplist.inverse_option.LastMenuInvers = -1;
        }
        if menusel >= 0 {
            self.saplist.inverse_option.rInvert = Rect::new(
                (tccol[menusel as usize] as i16) * 6 * ECHELLE as i16,
                (tclin[menusel as usize] as i16 - self.sapglob.tlpos[menusel as usize] as i16
                    + 1
                    + n)
                    * 7
                    * ECHELLE as i16
                    - 1,
                (tccol[menusel as usize] as i16 + tcncol[menusel as usize] as i16)
                    * 6
                    * ECHELLE as i16,
                (tclin[menusel as usize] as i16 - self.sapglob.tlpos[menusel as usize] as i16
                    + 1
                    + n
                    + 1)
                    * 7
                    * ECHELLE as i16
                    - 1,
            );
            self.sapwind.ports.debecr.SwapCouleur(
                &self.saplist.inverse_option.rInvert,
                self.sapwcvt.TabCvtCoul[self.sapglob.palbasse.borrow()[4] as usize] as i16,
                self.sapwcvt.TabCvtCoul[self.sapglob.palbasse.borrow()[1] as usize] as i16,
            );
            self.TfrEcr(Some(self.saplist.inverse_option.rInvert));
        }
        self.saplist.inverse_option.LastMenuInvers = menusel;
        self.saplist.inverse_option.Lastn = n;
    }
}

const EAU: u8 = 2;
const OUTREVIDE: u8 = 26;
const OUTREPLEINE: u8 = 27;

pub(crate) struct Saplist {
    inverse_option: InverseOption,
    sliste: [ListeId; 2],
    pub(crate) pal1: Rc<RefCell<[i16; 32]>>,
    pub(crate) pal2: Rc<RefCell<[i16; 32]>>,
    pal_zero: Rc<RefCell<[i16; 32]>>,
}

impl Default for Saplist {
    fn default() -> Self {
        Self {
            inverse_option: InverseOption::default(),
            sliste: [ListeId::Liste0, ListeId::Liste1],
            pal1: Rc::new(RefCell::new([
                0x006, 0x542, 0x650, 0x777, 0x430, 0x310, 0x320, 0x210, 0x005, 0x432, 0x443, 0x444,
                0x322, 0x332, 0x211, 0x000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])),
            pal2: Rc::new(RefCell::new([
                0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x531, 0x631, 0x641, 0x444, 0x050,
                0x070, 0x770, 0x450, 0x005, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ])),
            pal_zero: Rc::new(RefCell::new([0; 32])),
        }
    }
}

impl Sapiens {
    /************************************************************************
                            TRPOSE:Pose objet
    **********************************************************************
    =>(rega)et (menu)
    */
    pub(crate) fn trpose(&mut self) {
        let erreur =
            self.sapglob
                .enlevl(ListeId::Liste1, ListeId::Liste4, self.sapglob.rega as u16); /*on transfere*/
        if erreur == 1 && self.sapglob.lpos > 1 {
            self.sapglob.lpos -= 1;
        }
        /*elt de rang rega est en fin de liste*/
        self.inimen(self.sapglob.s_menu as u16);
    }

    /************************************************************************
                    INITPRIM:inits primaires
    ***********************************************************************/
    fn initprim(&mut self) {
        self.sapglob.dsposx = 0;
        self.sapglob.sposx = 60;
        self.sapglob.dsposy = 0;
        self.sapglob.sposy = 160;
        /* toute Palettes a zéro*/
        self.sapglob.palhaute = Rc::clone(&self.saplist.pal_zero);
        self.sapglob.pal_chron = Rc::clone(&self.saplist.pal_zero);
        self.sapglob.palbasse = Rc::clone(&self.saplist.pal_zero);
        self.sapglob.ciel_degr = Rc::clone(&self.saplist.pal_zero);
        let _first_init = self.init_graph();
        hires();
        self.sapglob.xj = 0x5800;
        self.sapglob.yj = 0x5400;
        self.sapglob.dzeta = 768;
        self.sapglob.flefba = 0;
        /*test version*/
        self.sapglob.flefba = 0;
        self.primsap();
        let sap1 = &mut self.sapglob.sap1;
        sap1.colonne = 200;
        self.sapglob.present[0] = -1;
        self.prm(SapId::Sap1, -1);
        self.prm(SapId::Sap2, 0);
        self.prm(SapId::Sap3, 0);
        self.prm(SapId::Sap4, 0);
        self.framebadass();
        self.sapglob.palhaute = Rc::clone(&self.saplist.pal2);
        self.sapglob.pal_chron = Rc::clone(&self.saplist.pal2);
        self.sapglob.palbasse = Rc::clone(&self.saplist.pal1);
        /* Mise en place version */
        self.DebutAventure(true);
        self.afstat();
        //# define gemaction  19,1,1,14
        self.gemouvre(1, 1, 23, 62, 19, 1, 1, 14);
        self.encadre(
            tclin[0] as u16,
            tccol[0] as u16,
            tcnlin[0] as u16,
            tcncol[0] as u16,
        ); /*cadre menu 0*/
        self.inimen(0);
        //if(firstInit) DoAide();
    }

    /************************************************************************/
    /*              GEMOUVRE:Transfere le cadre avec effet spécial          */
    /************************************************************************/
    /*  ouvre un cadre de cad1 vers cad2 (64 colonnes * 25 Lignes)*/
    /* le nombre de cadre 'GEMNBRECAD' doit etre pair*/
    pub(crate) fn gemouvre(
        &mut self,
        ligncad1: u16,
        colcad1: u16,
        hautcad1: u16,
        largcad1: u16,
        ligncad2: u16,
        colcad2: u16,
        hautcad2: u16,
        largcad2: u16,
    ) {
        /*calcul des parametres de depart*/
        let mut ligne = (ligncad1 * 7 / 2) as i16;
        let mut colon = (colcad1 * 3 / 2) as i16;
        let mut haut = (hautcad1 * 7 / 2) as i16;
        let mut larg = (largcad1 * 3 / 2) as i16;
        /* on l'injecte ds les N cadres*/

        for i in 0..gemnbrecad as usize {
            self.sapglob.gemzoom[i].ligne = ligne;
            self.sapglob.gemzoom[i].colon = colon;
            self.sapglob.gemzoom[i].haut = haut;
            self.sapglob.gemzoom[i].larg = larg;
        }
        /* on calcule les coordonnées d'arrivée*/
        ligne = (ligncad2 * 7 / 2) as i16;
        colon = (colcad2 * 3 / 2) as i16;
        haut = (hautcad2 * 7 / 2) as i16;
        larg = (largcad2 * 3 / 2) as i16;
        /* on y va*/
        let mut balai = 0;
        loop {
            for i in 0..=balai / geminterv as usize {
                /* on efface le cadre*/
                cadreg(
                    self.sapglob.gemzoom[i].ligne,
                    self.sapglob.gemzoom[i].colon,
                    self.sapglob.gemzoom[i].haut,
                    self.sapglob.gemzoom[i].larg,
                );
                /*on calcule le nlles coordonnées*/
                if self.sapglob.gemzoom[i].ligne < ligne {
                    self.sapglob.gemzoom[i].ligne += 1;
                } else if self.sapglob.gemzoom[i].ligne > ligne {
                    self.sapglob.gemzoom[i].ligne -= 1;
                }

                if self.sapglob.gemzoom[i].colon < colon {
                    self.sapglob.gemzoom[i].colon += 1;
                } else if self.sapglob.gemzoom[i].colon > colon {
                    self.sapglob.gemzoom[i].colon -= 1;
                }

                if self.sapglob.gemzoom[i].haut < haut {
                    self.sapglob.gemzoom[i].haut += 1;
                } else if self.sapglob.gemzoom[i].haut > haut {
                    self.sapglob.gemzoom[i].haut -= 1;
                }

                if self.sapglob.gemzoom[i].larg < larg {
                    self.sapglob.gemzoom[i].larg += 1;
                } else if self.sapglob.gemzoom[i].larg > larg {
                    self.sapglob.gemzoom[i].larg -= 1;
                }
                /*affiche le cadre*/
                cadreg(
                    self.sapglob.gemzoom[i].ligne,
                    self.sapglob.gemzoom[i].colon,
                    self.sapglob.gemzoom[i].haut,
                    self.sapglob.gemzoom[i].larg,
                );
            }
            /*si tout les cadres en cours alors balai ne bouge plus*/
            if balai / geminterv as usize + 1 < gemnbrecad as usize {
                balai += 1;
            }
            /*fini ????: dernier cadre arrivé*/
            if self.sapglob.gemzoom[(gemnbrecad - 1) as usize].ligne == ligne
                && self.sapglob.gemzoom[(gemnbrecad - 1) as usize].colon == colon
                && self.sapglob.gemzoom[(gemnbrecad - 1) as usize].haut == haut
                && self.sapglob.gemzoom[(gemnbrecad - 1) as usize].larg == larg
            {
                return;
            }
        }
    }

    /***********************************************************************
                  ListAttendsFinBouton
    **************************************************************************/
    fn ListAttendsFinBouton(&mut self) {
        loop {
            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) = self.vq_mouse();
            if self.sapglob.bouton <= 0 {
                break;
            }
        }
    }

    /***********************************************************************
                    CLAVM:Gestion des menus au clavier
    ***********************************************************************/
    pub(crate) fn clavm(&mut self) {
        let mut y;
        let mut r;

        /* gestion souris pour clavm */
        self.aff_sour(self.sapglob.icone, self.sapglob.sposy, self.sapglob.sposx);
        loop {
            /* appel de grise mine */
            self.gere_souris();
            /* teste si bouton appuye*/
            if self.sapglob.bouton > 0 {
                if self.sapglob.icone != INCONNU!(self.sapwind) && self.sapglob.icone != 0 {
                    /* calcul des valeurs par defaut pour la liste selectionne*/
                    /* deplacement dans la liste */
                    if (self.sapglob.icone == UP!(self.sapwind))
                        && (self.sapglob.tlpos[self.sapglob.s_menu as usize] > 1)
                    {
                        self.eff_sour();
                        self.sapglob.tlpos[self.sapglob.s_menu as usize] -= 1;
                        self.inimen(self.sapglob.s_menu as u16);
                        self.aff_sour(self.sapglob.icone, self.sapglob.sposy, self.sapglob.sposx);
                    }
                    if (self.sapglob.icone == DOWN!(self.sapwind))
                        && (self.sapglob.tlpos[self.sapglob.s_menu as usize]
                            <= (self.sapglob.liste(self.sapglob.stdlist).taille as u16
                                - tcnlin[self.sapglob.s_menu as usize] as u16)
                                as i8)
                    {
                        self.eff_sour();
                        self.sapglob.tlpos[self.sapglob.s_menu as usize] += 1;
                        self.inimen(self.sapglob.s_menu as u16);
                        self.aff_sour(self.sapglob.icone, self.sapglob.sposy, self.sapglob.sposx);
                    }
                    /* si selection et liste non vide */
                    if (self.sapglob.icone == SELECT!(self.sapwind))
                        && (self.sapglob.liste(self.sapglob.stdlist).taille > 0)
                    {
                        self.eff_sour();
                        /* calcul de l'option selectionn�e */
                        /* calcul position 53 du centre de la souris */
                        y = self.sapglob.sposy / 7;
                        /* calcul de l'option choisie*/
                        self.sapglob.rega = (y - tclin[self.sapglob.s_menu as usize] as i16
                            + self.sapglob.tlpos[self.sapglob.s_menu as usize] as i16
                            - 1) as u8;
                        if self.sapglob.rega < self.sapglob.liste(self.sapglob.stdlist).taille {
                            let refer = self
                                .sap_lang
                                .GetRefer(self.sapglob.liste(self.sapglob.stdlist).typeRef);
                            /*mise en place de stock1 et stock2*/
                            let ref_ = &refer[(self.sapglob.liste(self.sapglob.stdlist).elements
                                [self.sapglob.rega as usize]
                                - 1) as usize];
                            self.sapglob.stock1 = ref_.param1;
                            self.sapglob.stock2 = ref_.param2;
                            /*on affiche l'option choisie*/
                            self.efbadi();
                            if self.sapglob.s_menu <= 0 {
                                self.sapglob.s_action = (self.sapglob.rega + 1) as i16;
                            }
                            self.comment((120 + self.sapglob.s_action) as u16);
                            self.InverseOption(self.sapglob.s_menu, self.sapglob.rega as i16);
                            /* si sous menu affichage et sortie*/
                            if self.sapglob.s_menu > 0 {
                                let type_ref = self.sapglob.liste(self.saplist.sliste[1]).typeRef;
                                self.affcar(b' ' as i16);
                                let refer = self.sap_lang.GetRefer(type_ref);
                                let ref_ =
                                    &refer[(self.sapglob.liste(self.saplist.sliste[1]).elements
                                        [self.sapglob.rega as usize]
                                        - 1) as usize];
                                let mot = ref_.nomref;
                                self.mot(&mot);
                                self.TfrBadAss();
                                if self.sapglob.criter > 0 {
                                    self.sapglob.rega =
                                        self.sapglob.cross.elements[self.sapglob.rega as usize];
                                }
                                return;
                            } else
                            /* si menu principal */
                            /* on affiche (ou reaffiche) le sous-menu*/
                            {
                                self.sapglob.criter = self.sapglob.stock2;
                                if self.sapglob.criter > 0 {
                                    self.sapglob.tlpos[1] = 1;
                                }
                                if self.sapglob.stock1 == 7 {
                                    if self.sapglob.dervue != Vue::LATERALE {
                                        self.primjauge(0);
                                    }
                                    self.afstat();
                                } else {
                                    self.inimen(self.sapglob.stock1 as u16);
                                }
                            }
                            self.sapglob.sposx += 1; /* pour affichage */
                            (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) =
                                self.vq_mouse();
                        }
                        self.aff_sour(self.sapglob.icone, self.sapglob.sposy, self.sapglob.sposx);
                    }
                } else if self.sapglob.icone == 0 {
                    /* La souris est dans la partie haute de l'ecran */
                    /* Bouton et pas d'icone */
                    let pt = self
                        .sapwind
                        .ports
                        .GetMouse()
                        .zoom(self.sapwind.ports.window_zoom());

                    r = Rect::new(
                        (32 * ECHELLE) as i16,
                        ECHELLE as i16,
                        ((32 + 255) * ECHELLE) as i16,
                        ((1 + 100) * ECHELLE) as i16,
                    );
                    /* Il n'y a personne et le click est dans l'aire */
                    if self.sapglob.presence <= 0 && r.pt_in_rect(pt) {
                        /* Click dans l'aire de l'image */
                        /* On ouvre par defaut le menu'voir'*/
                        self.InverseOption(0, 6);
                        self.sapglob.s_action = 7;
                        self.inimen(3);
                        /* Demande le traitement */
                        self.sapglob.rega = 0;
                        return;
                    }
                    self.ClickInGenerique(pt);
                }
            }
        }
    }
}

impl Sapglob {
    /***********************************************************************
                  AJOUTL:ajoute un element a une liste
    ***********************************************************************/
    pub(crate) fn ajoutl(&mut self, debliste_id: ListeId, element: u16) -> bool {
        let debliste = self.liste_mut(debliste_id);
        if debliste_id == ListeId::Listbidon {
            /*le premier element le recoit*/
            debliste.elements[0] = element as u8;
        } else {
            /*liste est-elle pleine*/
            if debliste.taille == debliste.maxtaille {
                return true; /*deja pleine*/
            }
            debliste.elements[debliste.taille as usize] = element as u8;
            debliste.taille += 1;
        }
        false /*ok*/
    }

    /***********************************************************************
                    ENLEVL:transfere un element d'une liste sur une autre
    ***********************************************************************/
    pub(crate) fn enlevl(
        &mut self,
        orgliste_id: ListeId,
        receptliste_id: ListeId,
        rang: u16,
    ) -> u16 {
        let taille = self.liste_mut(orgliste_id).taille;
        if rang < taille as u16 {
            /*rang contenu dans liste de depart*/
            let element = {
                let orgliste = self.liste_mut(orgliste_id);
                orgliste.taille = orgliste.taille.wrapping_sub(1);
                orgliste.elements[rang as usize] as u16
            };
            let erreur = self.ajoutl(receptliste_id, element);
            if erreur {
                /*pas de place en reception*/
                let orgliste = self.liste_mut(orgliste_id);
                orgliste.taille = orgliste.taille.wrapping_add(1);
                1 /*erreur*/
            } else {
                /*on recompacte la liste d'origine*/
                let mut i = rang as u8;
                let orgliste = self.liste_mut(orgliste_id);
                while i != orgliste.taille {
                    orgliste.elements[i as usize] = orgliste.elements[i as usize + 1];
                    i = i.wrapping_add(1);
                }
                if i as u16 == rang {
                    1 /*fin de liste*/
                } else {
                    0
                }
            }
        } else {
            255 /*erreur de logique rang pas ds la liste de depart*/
        }
    }
}

impl Sapiens {
    /***********************************************************************
                  MOT:Affiche un mot selon la reference std
    ***********************************************************************/
    pub(crate) fn mot(&mut self, refer: &[u8]) {
        let mut p = 0;
        while refer[p] != 0 {
            self.affcar(refer[p] as i16);
            p += 1;
        }
    }

    /***********************************************************************
                  AFFMOT:affiche le mot ou des blancs
    ***********************************************************************/
    fn affmot(&mut self, debliste_id: ListeId, rang: u16) {
        let debliste = self.sapglob.liste(debliste_id);
        if (rang as u8) < debliste.taille {
            /*rang à afficher existe*/
            let refer = self.sap_lang.GetRefer(debliste.typeRef);
            let mot = refer[debliste.elements[rang as usize].wrapping_sub(1) as usize].nomref;
            self.mot(&mot);
        }
    }

    /***********************************************************************
                  AFLIST:Affiche liste en entr�e
    ***********************************************************************/
    fn aflist(&mut self, debliste_id: ListeId) {
        let _j: i16 = 0;
        let mut r;

        let _g = self.sapwind.ports.on(PortId::Debecr);
        if self.sapglob.s_menu == self.saplist.inverse_option.LastMenuInvers {
            self.saplist.inverse_option.LastMenuInvers = -1;
        }
        /*ligne et colonne au debut du cadre*/
        self.Gotoxy(self.sapglob.ccol as i16, self.sapglob.clin as i16);
        let mut i = self.sapglob.cnlin as u8; /*i recoit le nombre de ligne du cadre*/
        if self.sapglob.lpos == 0 {
            self.sapglob.lpos = 1;
        }

        let taille = self.sapglob.liste(debliste_id).taille;
        if (self.sapglob.lpos) > taille {
            self.sapglob.lpos = taille;
        }
        let mut travl2 = self.sapglob.lpos.wrapping_sub(1); /*debut d'affichage*/
        /* On efface le fond de la liste */
        r = Rect::new(
            (self.sapglob.colaff) * 6 * ECHELLE as i16,
            (self.sapglob.linaff) * 7 * ECHELLE as i16 - 1,
            (self.sapglob.colaff + self.sapglob.cncol as i16) * 6 * ECHELLE as i16,
            (self.sapglob.linaff + self.sapglob.cnlin as i16) * 7 * ECHELLE as i16,
        );
        self.BoxFill(PortId::Debecr, &mut r, 0x542);
        /* On affiche tous les mots */
        let taille = self.sapglob.liste(debliste_id).taille;
        if taille != 0 {
            /* On n'est pas en haut de la liste, fleche vers le haut */
            if travl2 > 1 {
                self.Gotoxy(
                    (self.sapglob.ccol as i16 + self.sapglob.cncol as i16) - 1,
                    self.sapglob.linaff,
                );
                self.affcar('<' as i16);
                self.Gotoxy(self.sapglob.ccol as i16, self.sapglob.clin as i16);
            }
            while i != 0 {
                i = i.wrapping_sub(1);
                self.affmot(debliste_id, travl2 as u16); /*on affiche le mot*/
                travl2 = travl2.wrapping_add(1);
                self.Gotoxy(self.sapglob.ccol as i16, self.sapglob.linaff + 1);
            }
            /* On n'est pas en bas de la liste, fleche vers le bas */
            let taille = self.sapglob.liste(debliste_id).taille;
            if travl2 < taille {
                self.Gotoxy(
                    (self.sapglob.ccol as i16 + self.sapglob.cncol as i16) - 1,
                    self.sapglob.linaff - 1,
                );
                self.affcar('>' as i16);
            }
        } else {
            /* Liste vide */
            self.mot(b"...\0");
        }
        /* Transfert de la liste sur l'ecran */
        r.InsetRect(-48, -48);
        self.TfrEcr(Some(r));
    }

    /***********************************************************************
                            ENCADRE:dessine le cadre
    ***********************************************************************/
    pub(crate) fn encadre(&mut self, ligne: u16, colon: u16, haut: u16, larg: u16) {
        /* passage au pixel */
        let mut ligne = (ligne * 7) as i16;
        let mut colon = (colon * 6) as i16;
        let mut haut = ligne + 7 * haut as i16;
        let mut larg = colon + 6 * larg as i16;
        /* petit delta pour polymorphisme*/
        ligne -= colon & 3;
        colon = colon - 2 - (haut & 3);
        haut += larg & 3;
        larg += ligne & 3;

        /* coin haut gauche vertical */
        self.affomb(MOB_CADRE!(self.sapwind), ligne - 24, colon - 8, 8, 8, false);
        /* coin haut droit vertical */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 1,
            ligne - 24,
            larg - 8,
            8,
            8,
            false,
        );
        /* coin haut gauche horizontal */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 2,
            ligne - 16,
            colon - 16,
            8,
            8,
            false,
        );
        /* angle  haut gauche */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 3,
            ligne - 8,
            colon - 8,
            8,
            8,
            false,
        );
        /* ligne horiz haute */
        let tailleh = (larg - colon - 1) / 16;
        if tailleh < 2000 {
            for i in 0..tailleh {
                self.affomb(
                    MOB_CADRE!(self.sapwind) + 4,
                    ligne - 8,
                    colon + 8 + i * 16,
                    8,
                    8,
                    false,
                );
            }
        }
        /* angle haut droit */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 5,
            ligne - 8,
            larg - 8,
            8,
            8,
            false,
        );
        /* coin haut droit horizontal */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 6,
            ligne - 16,
            larg + 8,
            8,
            8,
            false,
        );
        /* Block vertical */
        let taillev = (haut - ligne - 1) / 16;
        if taillev < 2000 {
            for i in 0..taillev {
                /* cote gauche */
                self.affomb(
                    MOB_CADRE!(self.sapwind) + 7,
                    ligne + 8 + 16 * i,
                    colon - 8,
                    8,
                    8,
                    false,
                );
                /* centre */
                if tailleh < 2000 {
                    for j in 0..tailleh {
                        self.affomb(
                            MOB_CADRE!(self.sapwind) + 16,
                            ligne + 8 + 16 * i,
                            colon + 8 + 16 * j,
                            8,
                            8,
                            false,
                        );
                    }
                }
                /* cote droit */
                self.affomb(
                    MOB_CADRE!(self.sapwind) + 8,
                    ligne + 8 + 16 * i,
                    larg - 8,
                    8,
                    8,
                    false,
                );
            }
        }
        /* coin bas gauche horizontal */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 9,
            haut - 8,
            colon - 16,
            8,
            8,
            false,
        );
        /* angle  bas gauche */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 10,
            haut - 8,
            colon - 8,
            8,
            8,
            false,
        );
        /* ligne horiz basse */
        if tailleh < 2000 {
            for i in 0..tailleh {
                self.affomb(
                    MOB_CADRE!(self.sapwind) + 11,
                    haut - 8,
                    colon + 8 + i * 16,
                    8,
                    8,
                    false,
                );
            }
        }
        /* angle bas droit */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 12,
            haut - 8,
            larg - 8,
            8,
            8,
            false,
        );
        /* coin bas droit horizontal */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 13,
            haut - 8,
            larg + 8,
            8,
            8,
            false,
        );
        /* coin bas gauche vertical */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 14,
            haut + 7,
            colon - 16,
            8,
            8,
            false,
        );
        /* coin bas droit vertical */
        self.affomb(MOB_CADRE!(self.sapwind) + 15, haut + 7, larg, 8, 8, false);
    }

    /***********************************************************************
                            INIMEN:Initialise Menu
    ***********************************************************************/
    pub(crate) fn inimen(&mut self, entmenu: u16) {
        let mut j = 0;
        let t = TickCount();
        self.sapglob.menu = entmenu; /*force variable globale*/
        self.sapglob.clin = tclin[self.sapglob.menu as usize] as u16;
        self.sapglob.ccol = tccol[self.sapglob.menu as usize] as u16;
        self.sapglob.colcdr = tcolcdr[self.sapglob.menu as usize] as u16;
        self.sapglob.cnlin = tcnlin[self.sapglob.menu as usize] as u16;
        self.sapglob.cncol = tcncol[self.sapglob.menu as usize] as u16;
        self.sapglob.poscur = tposcur[self.sapglob.menu as usize];
        self.sapglob.stdlist = self.sapglob.tstdlist[self.sapglob.menu as usize];
        self.eff_sour();
        if self.sapglob.menu != 0 {
            /*pas menu 0*/
            self.sapglob.smenu[1] = self.sapglob.menu as i8;
            if self.sapglob.menu != self.sapglob.lastmen as u16 {
                /*deja affiche ?*/
                self.gemouvre(
                    19,
                    1,
                    1,
                    14,
                    self.sapglob.clin,
                    self.sapglob.ccol,
                    self.sapglob.cnlin,
                    self.sapglob.cncol,
                );
                self.encadre(
                    self.sapglob.clin,
                    self.sapglob.ccol,
                    self.sapglob.cnlin,
                    self.sapglob.cncol,
                );
                self.sapglob.lastmen = self.sapglob.menu as u8;
            }
        } else {
            self.sapglob.criter = 0; /*menu principal : critere=0*/
            if self.sapglob.presence != 0 && self.sapglob.presence < 5 {
                self.sapglob.liste0.taille = 6;
            } else {
                self.sapglob.liste0.taille = 11;
            }
            if self.sapglob.mort == 0 {
                /* On demarre a zéro */
                self.sapglob.tlpos[0] = 1;
                self.sapglob.liste0.taille = 2;
                self.clrbadi();
                self.comment(136); /* La partie est fini */
            }
        }
        self.sapglob.lpos = self.sapglob.tlpos[self.sapglob.menu as usize] as u8;
        /*reduction du nbre d'elements*/
        if self.sapglob.criter != 0 {
            /*critere retenu*/
            let mut i = 0;
            self.sapglob.lpos = 111;
            while (i as u16) != self.sapglob.liste(self.sapglob.stdlist).taille as u16 {
                let rang = self.sapglob.liste(self.sapglob.stdlist).elements[i as usize]; /*rang de l'objet*/
                let refer1 = self.sap_lang.GetRefer1();
                let valeur = refer1[rang.wrapping_sub(1) as usize].param1 & 0xF0;
                if ((self.sapglob.criter == 1) && (valeur == 0x20))
                    || ((self.sapglob.criter == 2) && (valeur == 0x40))
                {
                    self.sapglob.zoneu.elements[j as usize] = rang;
                    self.sapglob.cross.elements[j as usize] = i as u8;
                    j += 1;
                    if (i as i8) >= self.sapglob.tlpos[self.sapglob.menu as usize]
                        && self.sapglob.lpos == 111
                    {
                        self.sapglob.lpos = j as u8;
                    }
                }
                i += 1;
            }
            if self.sapglob.lpos == 111 {
                self.sapglob.lpos = 1;
            }
            self.sapglob.zoneu.taille = j as u8;
            self.sapglob.stdlist = ListeId::Zoneu;
            /* nlle position ds menu*/
            self.sapglob.lpos = self.sapglob.tlpos[self.sapglob.menu as usize] as u8;
        }
        /*affichage de la liste*/
        if self.sapglob.menu != 0 {
            self.saplist.sliste[1] = self.sapglob.stdlist;
            self.sapglob.smenu[1] = self.sapglob.menu as i8;
        }
        self.sapglob.s_menu = self.sapglob.menu as i16;
        self.aflist(self.sapglob.stdlist);
        while TickCount() < t + 6 {}
    }

    /***********************************************************************
                          POS0_MOUSE
            Remets la souris en pos 0
    ***********************************************************************/
    pub(crate) fn pos0_mouse(&mut self) {
        self.reset_mouse();
    }

    /***********************************************************************
                          GERE_SOURIS
    ***********************************************************************/
    pub(crate) fn gere_souris(&mut self) {
        self.bouge_souris();
        if self.sapglob.sposy < 105 {
            self.sapglob.icone = 0;
            self.eff_sour();
        } else {
            /* calcul position 53 du centre de la souris */
            let x: i16 = (self.sapglob.sposx + 8) / 6;
            let y: i16 = (self.sapglob.sposy + 8) / 7;
            /* calcul de l'icone (par defaut rien) */
            self.sapglob.icone = INCONNU!(self.sapwind);
            self.sapglob.s_menu = -1;
            /* on balaye les menus */
            for i in 0..2 {
                let men = self.sapglob.smenu[i] as i16;
                /*on teste les verticales : men 111 implique pas de sousmenu*/
                if (men != 111)
                    && (x >= tccol[men as usize] as i16)
                    && (x <= tccol[men as usize] as i16 + tcncol[men as usize] as i16)
                {
                    self.sapglob.s_menu = men;
                    self.sapglob.stdlist = self.saplist.sliste[i];
                    /* on teste les horizontales */
                    if y < tclin[men as usize] as i16 {
                        self.sapglob.icone = UP!(self.sapwind);
                    } else if y > tclin[men as usize] as i16 + tcnlin[men as usize] as i16 {
                        self.sapglob.icone = DOWN!(self.sapwind);
                    } else {
                        self.sapglob.icone = SELECT!(self.sapwind);
                    }
                }
            }
        }
    }

    /***********************************************************************
                            TRPREN:Prends Objet
    ***********************************************************************/
    pub(crate) fn trpren(&mut self) {
        if self.sapglob.liste4.elements[self.sapglob.rega as usize] == EAU {
            let mut quoi: u8 = 0;
            self.sapglob.flefba = 0;
            if self.sapglob.soif == 255 {
                self.comment(98); /*tu n'as pas soif*/
            } else {
                self.comment(96); /*Tu bois*/
            }
            self.sapglob.soif = 255;
            /*on remplit les outres*/
            for i in 0..self.sapglob.liste1.taille {
                if self.sapglob.liste1.elements[i as usize] == OUTREVIDE {
                    self.sapglob.liste1.elements[i as usize] = OUTREPLEINE;
                    quoi += 1;
                }
            }
            if quoi != 0 {
                self.comment(97); /*et remplis tes gourdes*/
            }
            self.sapglob.flefba = 0;
            return;
        }
        let old_taille: i16 = self.sapglob.liste1.taille as i16;
        self.sapglob
            .enlevl(ListeId::Liste4, ListeId::Liste1, self.sapglob.rega as u16);
        if self.sapglob.liste1.taille as i16 == old_taille {
            self.sapglob.flefba = 0;
            self.comment(101);
        } else if self.sapglob.liste4.elements[self.sapglob.rega as usize] == 44 {
            self.sapglob.scenpr = 1; /*hache sacree*/
        }
        self.inimen(self.sapglob.s_menu as u16);
    }

    /***********************************************************************
                            TRABSO:Absorbe
    ***********************************************************************/
    pub(crate) fn trabso(&mut self) {
        if (self.sapglob.faim > 192) && (self.sapglob.soif > 192) {
            self.sapglob.flefba = 0;
            self.comment(107);
            return;
        }

        let mut calcul: u8 = self
            .sapglob
            .faim
            .wrapping_add((self.sapglob.stock2 & 0xF0) / 4);
        self.sapglob.faim = if calcul >= self.sapglob.faim {
            calcul
        } else {
            255
        };
        calcul = self
            .sapglob
            .soif
            .wrapping_add((self.sapglob.stock2 & 0x0F) * 4);
        self.sapglob.soif = if calcul >= self.sapglob.soif {
            calcul
        } else {
            255
        };

        let objet: u8 = self.sapglob.liste1.elements[self.sapglob.rega as usize];
        if objet == OUTREPLEINE {
            self.sapglob.liste1.elements[self.sapglob.rega as usize] = OUTREVIDE;
        } else {
            self.sapglob.enlevl(
                ListeId::Liste1,
                ListeId::Listbidon,
                self.sapglob.rega as u16,
            );
        }
        self.inimen(1);
    }

    /***********************************************************************
                            TRSOIN:Soigne
    ***********************************************************************/
    pub(crate) fn trsoin(&mut self) {
        if self.sapglob.bless > 192 {
            self.sapglob.flefba = 0;
            self.comment(109);
            return;
        }
        let calcul: u8 = self
            .sapglob
            .bless
            .wrapping_add((self.sapglob.stock2 & 0x0F) * 10);
        if calcul > self.sapglob.bless {
            self.sapglob.bless = calcul;
        } else {
            self.sapglob.bless = 255;
        }
        self.sapglob.enlevl(
            ListeId::Liste1,
            ListeId::Listbidon,
            self.sapglob.rega as u16,
        );
        self.inimen(1);
    }

    /***********************************************************************
                            D032AYDE
    ***********************************************************************/
    pub(crate) fn DoAide(&mut self) {
        if !crate::dialogs::open_manual(langue()) {
            self.Collapse("Can't open manual.");
        }
    }

    /***********************************************************************
                   AffichePictAide
    => 0 vue laterale, 1 paysage
    ***********************************************************************/
    pub(crate) fn AffichePictAide(&mut self, mode: i16) {
        let _g = self.sapwind.ports.on(PortId::MainWindow);
        let (num, r) = if mode == 0 {
            (131, Rect::new(320 - 125, 230, 320 + 125, 230 + 160))
        } else {
            (132, Rect::new(32, 230, 32 + 250, 230 + 160))
        };
        self.AffichePictDansAire(num, &r);
    }

    /***********************************************************************
                   TraiteVueLaterale
    ***********************************************************************/
    pub(crate) fn TraiteVueLaterale(&mut self) {
        self.MenuFenetres(FAUX as i16);
        self.efbadass();
        self.TfrBadAss();
        self.bruit(9);
        /* On affiche le rectangle d'aide */
        self.AffichePictAide(0);
        self.effaceBulles();
        self.combat();
        self.pos0_mouse();
        self.MenuFenetres(VRAI as i16);
    }

    /***********************************************************************
                    ACTION:Effectue une action
    ***********************************************************************/
    pub(crate) fn action(&mut self) {
        let current_action = self.sapglob.s_action;
        match current_action {
            1 => {
                self.trgest(self.sapglob.rega as u16);
                self.inimen(0);
            }
            8 => {
                self.trabso();
                self.ListAttendsFinBouton();
            }
            4 => {
                self.trpose();
                self.ListAttendsFinBouton();
            }
            3 => {
                self.trpren();
                self.ListAttendsFinBouton();
            }
            9 => {
                self.trsoin();
                self.ListAttendsFinBouton();
            }
            5 => {
                if self.sapglob.presence > 0 {
                    let refer = self
                        .sap_lang
                        .GetRefer(self.sapglob.liste(self.saplist.sliste[1]).typeRef);
                    if self.sapglob.dervue != Vue::LATERALE {
                        self.sapwind.ports.tfrima();
                    }
                    let index = self.sapglob.liste(self.saplist.sliste[1]).elements
                        [self.sapglob.rega as usize]
                        - 1;
                    let nomref = &refer[index as usize].nomref;
                    let texte: Vec<u8> = nomref
                        .iter()
                        .copied()
                        .take_while(|&c| c != 0)
                        .chain(std::iter::once(0))
                        .collect();
                    self.bulle(&texte, 0);
                    self.trdire(self.sapglob.liste2.elements[self.sapglob.rega as usize] as u16); /*on envoie la phrase*/
                    self.ListAttendsFinBouton();
                } else {
                    /* Tu parles tou seul ... */
                    self.clrbadi();
                    self.comment(135);
                }
            }
            6 => {
                self.trdonn(self.sapglob.rega as u16); /*on envoit le rang de l'objet*/
                self.ListAttendsFinBouton();
            }
            7 => {
                self.trobsv(self.sapglob.rega as u16);
            }
            10 => {
                self.MenuFenetres(FAUX as i16);
                self.trdors();
                self.MenuFenetres(VRAI as i16);
            }
            11 => {
                self.MenuFenetres(FAUX as i16);
                self.trfab(self.sapglob.rega as u16);
                self.MenuFenetres(VRAI as i16);
            }
            _ => {
                warn!("Unexpected action {}", current_action);
            }
        }
    }

    pub(crate) fn sapiens_main(&mut self) {
        self.initprim(); /*inits primaires*/
        //Final();
        //	DEF3D();
        loop {
            self.clavm();
            self.action();
            if self.sapglob.flcomb > 0 {
                self.TraiteVueLaterale();
            }
        } /*infini...et meme un peu plus */
    }
}
/*---- FIN -----*/
