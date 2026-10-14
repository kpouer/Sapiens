/*************************V1.6 Mardi 9 Dec**********************************
                         Module #1 :Projet SAP
              Gestion de Listes:Le mardi 4 Novembre 1986
**************************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::acam::{FAUX, SetRect, TickCount, VRAI};
use crate::cbridge::strcat;
use crate::langue::Langue::{ES, FR};
use crate::langue::langue_i16;
use crate::port;
use crate::sapiens::sapglob::{ListeId, SapId, tccol, tclin, tcncol, tcnlin, tcolcdr, tposcur};
use crate::sapiens::sapwbrt::{AffichePictDansAire, SetPathInData, SwapCouleur};
use crate::sapiens::sapwind::{cadreg, hires};
use crate::sapiens::{ECHELLE, Vue, geminterv, gemnbrecad};
use crate::shared::{Ptr, Rect};
use crate::stubs::{GetMouse, GetNomCheminCourant, OpenDocument, PopPath, PushPath};
use crate::{DOWN, INCONNU, MOB_CADRE, SELECT, Sapiens, UP};
use log::warn;
use std::ffi::c_char;
use std::os::raw::{c_short, c_uchar, c_ushort};
use std::ptr::addr_of_mut;
/***********************************************************************
                InverseOption
***********************************************************************/

pub(crate) struct InverseOption {
    LastMenuInvers: c_short,
    Lastn: c_short,
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
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn InverseOption(&mut self, menusel: c_short, n: c_short) {
        unsafe {
            self.eff_sour();
            if self.saplist.inverse_option.LastMenuInvers >= 0 {
                SwapCouleur(
                    self.sapwind.debecr,
                    &raw mut self.saplist.inverse_option.rInvert,
                    self.sapwcvt.TabCvtCoul[(*self.sapglob.palbasse.add(4)) as usize] as c_short,
                    self.sapwcvt.TabCvtCoul[(*self.sapglob.palbasse.add(1)) as usize] as c_short,
                );
                self.TfrEcr(Some(&self.saplist.inverse_option.rInvert));
                self.saplist.inverse_option.LastMenuInvers = -1;
            }
            if menusel >= 0 {
                SetRect(
                    &raw mut self.saplist.inverse_option.rInvert,
                    (tccol[menusel as usize] as c_short) * 6 * ECHELLE as c_short,
                    (tclin[menusel as usize] as c_short
                        - self.sapglob.tlpos[menusel as usize] as c_short
                        + 1
                        + n)
                        * 7
                        * ECHELLE as c_short
                        - 1,
                    (tccol[menusel as usize] as c_short + tcncol[menusel as usize] as c_short)
                        * 6
                        * ECHELLE as c_short,
                    (tclin[menusel as usize] as c_short
                        - self.sapglob.tlpos[menusel as usize] as c_short
                        + 1
                        + n
                        + 1)
                        * 7
                        * ECHELLE as c_short
                        - 1,
                );
                SwapCouleur(
                    self.sapwind.debecr,
                    &raw mut self.saplist.inverse_option.rInvert,
                    self.sapwcvt.TabCvtCoul[(*self.sapglob.palbasse.add(1)) as usize] as c_short,
                    self.sapwcvt.TabCvtCoul[(*self.sapglob.palbasse.add(4)) as usize] as c_short,
                );
                self.TfrEcr(Some(&self.saplist.inverse_option.rInvert));
            }
            self.saplist.inverse_option.LastMenuInvers = menusel;
            self.saplist.inverse_option.Lastn = n;
        }
    }
}

const EAU: c_uchar = 2;
const OUTREVIDE: c_uchar = 26;
const OUTREPLEINE: c_uchar = 27;

pub(crate) struct Saplist {
    inverse_option: InverseOption,
    sliste: [ListeId; 2],
    pub(crate) pal1: [c_short; 32],
    pub(crate) pal2: [c_short; 32],
    pal_zero: [c_short; 32],
}

impl Default for Saplist {
    fn default() -> Self {
        Self {
            inverse_option: InverseOption::default(),
            sliste: [ListeId::Liste0, ListeId::Liste1],
            pal1: [
                0x006, 0x542, 0x650, 0x777, 0x430, 0x310, 0x320, 0x210, 0x005, 0x432, 0x443, 0x444,
                0x322, 0x332, 0x211, 0x000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            pal2: [
                0x777, 0x000, 0x700, 0x320, 0x430, 0x540, 0x650, 0x531, 0x631, 0x641, 0x444, 0x050,
                0x070, 0x770, 0x450, 0x005, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            pal_zero: [0; 32],
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
        let erreur = self.enlevl(
            ListeId::Liste1,
            ListeId::Liste4,
            self.sapglob.rega as c_ushort,
        ); /*on transfere*/
        if erreur == 1 && self.sapglob.lpos > 1 {
            self.sapglob.lpos -= 1;
        }
        /*elt de rang rega est en fin de liste*/
        self.inimen(self.sapglob.s_menu as c_ushort);
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
        self.sapglob.palhaute = addr_of_mut!(self.saplist.pal_zero) as *mut std::os::raw::c_short;
        self.sapglob.pal_chron = addr_of_mut!(self.saplist.pal_zero) as *mut std::os::raw::c_short;
        self.sapglob.palbasse = addr_of_mut!(self.saplist.pal_zero) as *mut std::os::raw::c_short;
        self.sapglob.ciel_degr = addr_of_mut!(self.saplist.pal_zero) as *mut std::os::raw::c_short;
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
        self.sapglob.palhaute = addr_of_mut!(self.saplist.pal2) as *mut std::os::raw::c_short;
        self.sapglob.pal_chron = addr_of_mut!(self.saplist.pal2) as *mut std::os::raw::c_short;
        self.sapglob.palbasse = addr_of_mut!(self.saplist.pal1) as *mut std::os::raw::c_short;
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
        ligncad1: c_ushort,
        colcad1: c_ushort,
        hautcad1: c_ushort,
        largcad1: c_ushort,
        ligncad2: c_ushort,
        colcad2: c_ushort,
        hautcad2: c_ushort,
        largcad2: c_ushort,
    ) {
        /*calcul des parametres de depart*/
        let mut ligne: c_short = (ligncad1 * 7 / 2) as c_short;
        let mut colon: c_short = (colcad1 * 3 / 2) as c_short;
        let mut haut: c_short = (hautcad1 * 7 / 2) as c_short;
        let mut larg: c_short = (largcad1 * 3 / 2) as c_short;
        /* on l'injecte ds les N cadres*/

        for i in 0..gemnbrecad as usize {
            self.sapglob.gemzoom[i].ligne = ligne;
            self.sapglob.gemzoom[i].colon = colon;
            self.sapglob.gemzoom[i].haut = haut;
            self.sapglob.gemzoom[i].larg = larg;
        }
        /* on calcule les coordonn�es d'arriv�e*/
        ligne = (ligncad2 * 7 / 2) as c_short;
        colon = (colcad2 * 3 / 2) as c_short;
        haut = (hautcad2 * 7 / 2) as c_short;
        larg = (largcad2 * 3 / 2) as c_short;
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
        let mut y: c_short;
        let mut r = Rect::default();

        /* gestion souris pour clavm */
        self.aff_sour(self.sapglob.icone, self.sapglob.sposy, self.sapglob.sposx);
        unsafe {
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
                            self.inimen(self.sapglob.s_menu as c_ushort);
                            self.aff_sour(
                                self.sapglob.icone,
                                self.sapglob.sposy,
                                self.sapglob.sposx,
                            );
                        }
                        if (self.sapglob.icone == DOWN!(self.sapwind))
                            && (self.sapglob.tlpos[self.sapglob.s_menu as usize]
                                <= (self.sapglob.liste(self.sapglob.stdlist).taille as u16
                                    - tcnlin[self.sapglob.s_menu as usize] as u16)
                                    as i8)
                        {
                            self.eff_sour();
                            self.sapglob.tlpos[self.sapglob.s_menu as usize] += 1;
                            self.inimen(self.sapglob.s_menu as c_ushort);
                            self.aff_sour(
                                self.sapglob.icone,
                                self.sapglob.sposy,
                                self.sapglob.sposx,
                            );
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
                            self.sapglob.rega = (y - tclin[self.sapglob.s_menu as usize] as c_short
                                + self.sapglob.tlpos[self.sapglob.s_menu as usize] as c_short
                                - 1) as c_uchar;
                            if self.sapglob.rega < self.sapglob.liste(self.sapglob.stdlist).taille {
                                let refer =
                                    self.GetRefer(self.sapglob.liste(self.sapglob.stdlist).typeRef);
                                /*mise en place de stock1 et stock2*/
                                let ref_ = refer.wrapping_add(
                                    (self.sapglob.liste(self.sapglob.stdlist).elements
                                        [self.sapglob.rega as usize]
                                        - 1) as usize,
                                );
                                self.sapglob.stock1 = (*ref_).param1;
                                self.sapglob.stock2 = (*ref_).param2;
                                /*on affiche l'option choisie*/
                                self.efbadi();
                                if self.sapglob.s_menu <= 0 {
                                    self.sapglob.s_action = (self.sapglob.rega + 1) as c_short;
                                }
                                self.comment((120 + self.sapglob.s_action) as c_ushort);
                                self.InverseOption(
                                    self.sapglob.s_menu,
                                    self.sapglob.rega as c_short,
                                );
                                /* si sous menu affichage et sortie*/
                                if self.sapglob.s_menu > 0 {
                                    let refer = self.GetRefer(
                                        self.sapglob.liste(self.saplist.sliste[1]).typeRef,
                                    );

                                    self.affcar(' ' as u8 as c_short);
                                    let ref_ = refer.wrapping_add(
                                        (self.sapglob.liste(self.saplist.sliste[1]).elements
                                            [self.sapglob.rega as usize]
                                            - 1) as usize,
                                    );
                                    self.mot(ref_ as *mut _);
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
                                        self.inimen(self.sapglob.stock1 as c_ushort);
                                    }
                                }
                                self.sapglob.sposx += 1; /* pour affichage */
                                (self.sapglob.bouton, self.sapglob.sposx, self.sapglob.sposy) =
                                    self.vq_mouse();
                            }
                            self.aff_sour(
                                self.sapglob.icone,
                                self.sapglob.sposy,
                                self.sapglob.sposx,
                            );
                        }
                    } else
                    /* La souris est dans la partie haute de l'ecran */
                    if self.sapglob.icone == 0 {
                        /* Bouton et pas d'icone */
                        let pt = GetMouse().zoom(port::window_zoom());

                        SetRect(
                            &mut r,
                            (32 * ECHELLE) as i16,
                            (1 * ECHELLE) as i16,
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

    /***********************************************************************
                  AJOUTL:ajoute un element a une liste
    ***********************************************************************/
    pub(crate) fn ajoutl(&mut self, debliste_id: ListeId, element: c_ushort) -> c_ushort {
        let debliste = self.sapglob.liste_mut(debliste_id);
        if debliste_id == ListeId::Listbidon {
            /*le premier element le recoit*/
            debliste.elements[0] = element as c_uchar;
        } else {
            /*liste est elle pleine*/
            if debliste.taille == debliste.maxtaille {
                return 1; /*deja pleine*/
            }
            debliste.elements[debliste.taille as usize] = element as c_uchar;
            debliste.taille += 1;
        }
        0 /*ok*/
    }

    /***********************************************************************
                    ENLEVL:transfere un element d'une liste sur une autre
    ***********************************************************************/
    pub(crate) fn enlevl(
        &mut self,
        orgliste_id: ListeId,
        receptliste_id: ListeId,
        rang: c_ushort,
    ) -> c_ushort {
        let taille = self.sapglob.liste_mut(orgliste_id).taille;
        if rang < taille as c_ushort {
            /*rang contenu dans liste de depart*/
            let element = {
                let orgliste = self.sapglob.liste_mut(orgliste_id);
                orgliste.taille = orgliste.taille.wrapping_sub(1);
                orgliste.elements[rang as usize] as c_ushort
            };
            let erreur = self.ajoutl(receptliste_id, element);
            if erreur != 0 {
                /*pas de place en reception*/
                let orgliste = self.sapglob.liste_mut(orgliste_id);
                (*orgliste).taille = (*orgliste).taille.wrapping_add(1);
                1 /*erreur*/
            } else {
                /*on recompacte la liste d'origine*/
                let mut i: c_uchar = rang as c_uchar;
                let orgliste = self.sapglob.liste_mut(orgliste_id);
                while i != orgliste.taille {
                    orgliste.elements[i as usize] = orgliste.elements[i as usize + 1];
                    i = i.wrapping_add(1);
                }
                if i as c_ushort == rang {
                    1 /*fin de liste*/
                } else {
                    0
                }
            }
        } else {
            255 /*erreur de logique rang pas ds la liste de depart*/
        }
    }

    /***********************************************************************
                  MOT:Affiche un mot selon la reference std
    ***********************************************************************/
    pub(crate) fn mot(&mut self, refer: *mut c_char) {
        unsafe {
            let mut p = refer as *mut c_uchar;
            while *p != 0 {
                self.affcar(*p as c_short);
                p = p.add(1);
            }
        }
    }

    /***********************************************************************
                  AFFMOT:affiche le mot ou des blancs
    ***********************************************************************/
    fn affmot(&mut self, debliste_id: ListeId, rang: c_ushort) {
        let debliste = self.sapglob.liste(debliste_id);
        unsafe {
            if (rang as c_uchar) < (*debliste).taille {
                /*rang a afficher existe*/
                let refer = self.GetRefer((*debliste).typeRef);
                self.mot(
                    refer.add(((*debliste).elements[rang as usize].wrapping_sub(1)) as usize)
                        as *mut c_char,
                );
            }
        }
    }

    /***********************************************************************
                  AFLIST:Affiche liste en entr�e
    ***********************************************************************/
    fn aflist(&mut self, debliste_id: ListeId) {
        let _j: c_short = 0;
        let mut r = Rect::default();

        let _g = port::on(self.sapwind.debecr);
        if self.sapglob.s_menu == self.saplist.inverse_option.LastMenuInvers {
            self.saplist.inverse_option.LastMenuInvers = -1;
        }
        /*ligne et colonne au debut du cadre*/
        self.Gotoxy(self.sapglob.ccol as c_short, self.sapglob.clin as c_short);
        let mut i = self.sapglob.cnlin as c_uchar; /*i recoit le nombre de ligne du cadre*/
        if self.sapglob.lpos == 0 {
            self.sapglob.lpos = 1;
        }

        let taille = self.sapglob.liste(debliste_id).taille;
        if (self.sapglob.lpos as c_uchar) > taille {
            self.sapglob.lpos = taille;
        }
        let mut travl2 = self.sapglob.lpos.wrapping_sub(1); /*debut d'affichage*/
        /* On efface le fond de la liste */
        SetRect(
            &mut r,
            (self.sapglob.colaff as c_short) * 6 * ECHELLE as c_short,
            (self.sapglob.linaff as c_short) * 7 * ECHELLE as c_short - 1,
            (self.sapglob.colaff as c_short + self.sapglob.cncol as c_short)
                * 6
                * ECHELLE as c_short,
            (self.sapglob.linaff as c_short + self.sapglob.cnlin as c_short)
                * 7
                * ECHELLE as c_short,
        );
        self.BoxFill(self.sapwind.debecr, &mut r, 0x542 as c_short);
        /* On affiche tous les mots */
        let taille = self.sapglob.liste(debliste_id).taille;
        if taille != 0 {
            /* On n'est pas en haut de la liste, fleche vers le haut */
            if travl2 > 1 {
                self.Gotoxy(
                    (self.sapglob.ccol as c_short + self.sapglob.cncol as c_short) - 1,
                    self.sapglob.linaff as c_short,
                );
                self.affcar('<' as c_short);
                self.Gotoxy(self.sapglob.ccol as c_short, self.sapglob.clin as c_short);
            }
            while i != 0 {
                i = i.wrapping_sub(1);
                self.affmot(debliste_id, travl2 as c_ushort); /*on affiche le mot*/
                travl2 = travl2.wrapping_add(1);
                self.Gotoxy(
                    self.sapglob.ccol as c_short,
                    (self.sapglob.linaff + 1) as c_short,
                );
            }
            /* On n'est pas en bas de la liste, fleche vers le bas */
            let taille = self.sapglob.liste(debliste_id).taille;
            if travl2 < taille {
                self.Gotoxy(
                    (self.sapglob.ccol as c_short + self.sapglob.cncol as c_short) - 1,
                    (self.sapglob.linaff - 1) as c_short,
                );
                self.affcar('>' as c_short);
            }
        } else {
            /* Liste vide */
            self.mot(c"...".as_ptr() as *mut c_char);
        }
        /* Transfert de la liste sur l'ecran */
        r.InsetRect(-48, -48);
        self.TfrEcr(Some(&r));
    }

    /***********************************************************************
                            ENCADRE:dessine le cadre
    ***********************************************************************/
    pub(crate) fn encadre(&self, ligne: c_ushort, colon: c_ushort, haut: c_ushort, larg: c_ushort) {
        /* passage au pixel */
        let mut ligne = (ligne * 7) as c_short;
        let mut colon = (colon * 6) as c_short;
        let mut haut = ligne + 7 * haut as c_short;
        let mut larg = colon + 6 * larg as c_short;
        /* petit delta pour polymorphisme*/
        ligne -= colon & 3;
        colon = colon - 2 - (haut & 3);
        haut += larg & 3;
        larg += ligne & 3;

        /* coin haut gauche vertical */
        self.affomb(
            MOB_CADRE!(self.sapwind) + 0,
            ligne - 24,
            colon - 8,
            8,
            8,
            false,
        );
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
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn inimen(&mut self, entmenu: c_ushort) {
        unsafe {
            let mut j: c_short = 0;
            let t = TickCount();
            let refer1 = self.GetRefer1();

            self.sapglob.menu = entmenu; /*force variable globale*/
            self.sapglob.clin = tclin[self.sapglob.menu as usize] as c_ushort;
            self.sapglob.ccol = tccol[self.sapglob.menu as usize] as c_ushort;
            self.sapglob.colcdr = tcolcdr[self.sapglob.menu as usize] as c_ushort;
            self.sapglob.cnlin = tcnlin[self.sapglob.menu as usize] as c_ushort;
            self.sapglob.cncol = tcncol[self.sapglob.menu as usize] as c_ushort;
            self.sapglob.poscur = tposcur[self.sapglob.menu as usize];
            self.sapglob.stdlist = self.sapglob.tstdlist[self.sapglob.menu as usize];
            self.eff_sour();
            if self.sapglob.menu != 0 {
                /*pas menu 0*/
                (*addr_of_mut!(self.sapglob.smenu))[1] = self.sapglob.menu as c_char;
                if self.sapglob.menu != self.sapglob.lastmen as c_ushort {
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
                    self.sapglob.lastmen = self.sapglob.menu as c_uchar;
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
            self.sapglob.lpos = self.sapglob.tlpos[self.sapglob.menu as usize] as c_uchar;
            /*reduction du nbre d'elements*/
            if self.sapglob.criter != 0 {
                /*critere retenu*/
                let mut i: c_short = 0;
                i = 0;
                self.sapglob.lpos = 111;
                while (i as c_ushort) != self.sapglob.liste(self.sapglob.stdlist).taille as c_ushort
                {
                    let rang: c_uchar =
                        self.sapglob.liste(self.sapglob.stdlist).elements[i as usize]; /*rang de l'objet*/
                    let valeur: c_uchar =
                        (*refer1.add(rang.wrapping_sub(1) as usize)).param1 & 0xF0;
                    if ((self.sapglob.criter == 1) && (valeur == 0x20))
                        || ((self.sapglob.criter == 2) && (valeur == 0x40))
                    {
                        self.sapglob.zoneu.elements[j as usize] = rang;
                        self.sapglob.cross.elements[j as usize] = i as c_uchar;
                        j += 1;
                        if (i as c_char) >= self.sapglob.tlpos[self.sapglob.menu as usize]
                            && self.sapglob.lpos == 111
                        {
                            self.sapglob.lpos = j as c_uchar;
                        }
                    }
                    i += 1;
                }
                if self.sapglob.lpos == 111 {
                    self.sapglob.lpos = 1;
                }
                self.sapglob.zoneu.taille = j as c_uchar;
                self.sapglob.stdlist = ListeId::Zoneu;
                /* nlle position ds menu*/
                self.sapglob.lpos = self.sapglob.tlpos[self.sapglob.menu as usize] as c_uchar;
            }
            /*affichage de la liste*/
            if self.sapglob.menu != 0 {
                self.saplist.sliste[1] = self.sapglob.stdlist;
                self.sapglob.smenu[1] = self.sapglob.menu as c_char;
            }
            self.sapglob.s_menu = self.sapglob.menu as c_short;
            self.aflist(self.sapglob.stdlist);
            while TickCount() < t + 6 {}
        }
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
        unsafe {
            if self.sapglob.sposy < 105 {
                self.sapglob.icone = 0;
                self.eff_sour();
            } else {
                /* calcul position 53 du centre de la souris */
                let x: c_short = (self.sapglob.sposx + 8) / 6;
                let y: c_short = (self.sapglob.sposy + 8) / 7;
                /* calcul de l'icone (par defaut rien) */
                self.sapglob.icone = INCONNU!(self.sapwind);
                self.sapglob.s_menu = -1;
                /* on balaye les menus */
                for i in 0..2 {
                    let men: c_short = (*addr_of_mut!(self.sapglob.smenu))[i] as c_short;
                    /*on teste les verticales : men 111 implique pas de sousmenu*/
                    if (men != 111)
                        && (x >= tccol[men as usize] as c_short)
                        && (x <= tccol[men as usize] as c_short + tcncol[men as usize] as c_short)
                    {
                        self.sapglob.s_menu = men;
                        self.sapglob.stdlist = self.saplist.sliste[i];
                        /* on teste les horizontales */
                        if y < tclin[men as usize] as c_short {
                            self.sapglob.icone = UP!(self.sapwind);
                        } else if y > tclin[men as usize] as c_short
                            + tcnlin[men as usize] as c_short
                        {
                            self.sapglob.icone = DOWN!(self.sapwind);
                        } else {
                            self.sapglob.icone = SELECT!(self.sapwind);
                        }
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
            let mut quoi: c_uchar = 0;
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
        let old_taille: c_short = self.sapglob.liste1.taille as c_short;
        self.enlevl(
            ListeId::Liste4,
            ListeId::Liste1,
            self.sapglob.rega as c_ushort,
        );
        if self.sapglob.liste1.taille as c_short == old_taille {
            self.sapglob.flefba = 0;
            self.comment(101);
        } else if self.sapglob.liste4.elements[self.sapglob.rega as usize] == 44 {
            self.sapglob.scenpr = 1; /*hache sacree*/
        }
        self.inimen(self.sapglob.s_menu as c_ushort);
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

        let mut calcul: c_uchar = self
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

        let objet: c_uchar = self.sapglob.liste1.elements[self.sapglob.rega as usize];
        if objet == OUTREPLEINE {
            self.sapglob.liste1.elements[self.sapglob.rega as usize] = OUTREVIDE;
        } else {
            self.enlevl(
                ListeId::Liste1,
                ListeId::Listbidon,
                self.sapglob.rega as c_ushort,
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
        let calcul: c_uchar = self
            .sapglob
            .bless
            .wrapping_add((self.sapglob.stock2 & 0x0F) * 10);
        if calcul > self.sapglob.bless {
            self.sapglob.bless = calcul;
        } else {
            self.sapglob.bless = 255;
        }
        self.enlevl(
            ListeId::Liste1,
            ListeId::Listbidon,
            self.sapglob.rega as c_ushort,
        );
        self.inimen(1);
    }

    /***********************************************************************
                            D032AYDE
    ***********************************************************************/
    pub(crate) fn DoAide(&self) {
        let mut chaine: [c_char; 255] = [0; 255];
        PushPath();
        SetPathInData(&self.sapgest, c"docs".as_ptr() as *mut c_char);
        GetNomCheminCourant(chaine.as_mut_ptr());
        unsafe {
            if langue_i16() == FR as c_short {
                strcat(chaine.as_mut_ptr(), c"sapiens_fr.html".as_ptr());
            } else if langue_i16() == ES as c_short {
                strcat(chaine.as_mut_ptr(), c"sapiens_es.html".as_ptr());
            } else {
                strcat(chaine.as_mut_ptr(), c"sapiens_gb.html".as_ptr());
            }
        }
        if OpenDocument(chaine.as_mut_ptr()) != 0 {
            self.Collapse("Can't open manual.");
        }
        PopPath();
    }

    /***********************************************************************
                   AffichePictAide
    => 0 vue laterale, 1 paysage
    ***********************************************************************/
    pub(crate) fn AffichePictAide(&self, mode: c_short) {
        let mut r = Rect::default();
        let _g = port::on(self.sapwind.mainWindow);
        let num = if mode == 0 {
            SetRect(&mut r, 320 - 125, 230, 320 + 125, 230 + 160);
            131
        } else {
            SetRect(&mut r, 32, 230, 32 + 250, 230 + 160);
            132
        };
        AffichePictDansAire(&self.sapgest, num, &r);
    }

    /***********************************************************************
                   TraiteVueLaterale
    ***********************************************************************/
    pub(crate) fn TraiteVueLaterale(&mut self) {
        self.MenuFenetres(FAUX as c_short);
        self.efbadass();
        self.TfrBadAss();
        self.bruit(9);
        /* On affiche le rectangle d'aide */
        self.AffichePictAide(0);
        self.effaceBulles();
        self.combat();
        self.pos0_mouse();
        self.MenuFenetres(VRAI as c_short);
    }

    /***********************************************************************
                    ACTION:Effectue une action
    ***********************************************************************/
    pub(crate) fn action(&mut self) {
        let current_action = self.sapglob.s_action;
        match current_action {
            1 => {
                self.trgest(self.sapglob.rega as c_ushort);
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
                    let refer = self.GetRefer(self.sapglob.liste(self.saplist.sliste[1]).typeRef);
                    if self.sapglob.dervue != Vue::LATERALE {
                        self.tfrima();
                    }
                    let index = self.sapglob.liste(self.saplist.sliste[1]).elements
                        [self.sapglob.rega as usize]
                        - 1;
                    let ptr = &(refer.wrapping_add(index as usize)) as *const _ as Ptr;
                    self.bulle(ptr, 0);
                    self.trdire(
                        self.sapglob.liste2.elements[self.sapglob.rega as usize] as c_ushort,
                    ); /*on envoie la phrase*/
                    self.ListAttendsFinBouton();
                } else {
                    /* Tu parles tou seul ... */
                    self.clrbadi();
                    self.comment(135);
                }
            }
            6 => {
                self.trdonn(self.sapglob.rega as c_ushort); /*on envoit le rang de l'objet*/
                self.ListAttendsFinBouton();
            }
            7 => {
                self.trobsv(self.sapglob.rega as c_ushort);
            }
            10 => {
                self.MenuFenetres(FAUX as c_short);
                self.trdors();
                self.MenuFenetres(VRAI as c_short);
            }
            11 => {
                self.MenuFenetres(FAUX as c_short);
                self.trfab(self.sapglob.rega as c_ushort);
                self.MenuFenetres(VRAI as c_short);
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
        self.clavm();
        self.action();
        loop {
            if self.sapglob.flcomb > 0 {
                self.TraiteVueLaterale();
            }
        } /*infini...et meme un peu plus */
    }
}
/*---- FIN -----*/
