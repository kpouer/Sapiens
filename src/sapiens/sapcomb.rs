/*********************************************************************
                Modul #3 .Sapcomb(Combats Multiples)
                    Le Jeudi 20 Novembre 1986
Le nombre d'herbe au premier plan est ajuste en fonction de la vitesse processeur
Cx :                 2619   0
PowerMac 6100/66 :   6584
Quadra :             10010  20
***********************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::DEBCRANE;
use crate::DEBTETE;
use crate::MOB_ANIMAL_MORT;
use crate::MOB_CHEMIN;
use crate::MOB_CORPS;
use crate::MOB_EMPREINTE;
use crate::MOB_FLECHE;
use crate::MOB_HERBE;
use crate::MOB_IMPACT;
use crate::MOB_MORT;
use crate::MOB_SANG_A_TERRE;
use crate::acam::{FAUX, VRAI};
use crate::cbridge::{geneva, teJustLeft};
use crate::port::PortId;
use crate::sapiens::sapglob::{
    AnimPtr, AnimTable, ListeId, SapId, Sapglob, bestagress, besthaut, bestlin, bestvie, hautlin,
    hautou, qtviande, tabinp, tribforce, tribvie,
};
use crate::sapiens::sapwmus::attends_bruit;
use crate::sapiens::sapwpref::JOUEUR2_AUTONOME;
use crate::sapiens::{
    ECHELLE, FormeVue, NB_ARBRES_LATERAL, NB_ARBRES_LATERAL_MAX, NB_HERBE_ARR_PLAN,
    NB_MAX_HERBE_AR_PLAN, NB_MAX_HERBE_AV_PLAN, SAGAIE, Vue, brin, sap,
};
use crate::shared::{Boolean, Point, Rect};
use crate::{DEBBEST, Sapiens};
use std::sync::atomic::Ordering;
use strum::IntoEnumIterator;

const LIGNE_BASE_HERBE_AV_PLAN: i16 = 63;
const LIGNE_MAX_HERBE_AV_PLAN: i16 = 70;
const LIMITE_GAUCHE: i16 = -5;
const LIMITE_DROITE: i16 = 62;

pub(crate) struct Sapcomb {
    pub(crate) nbMobHerbes: i16,
    pub(crate) nbAvplanSpec: i16,
    NbHerbeAvPlan: i16,
    herbe: [brin; NB_MAX_HERBE_AV_PLAN as usize],
    pub(crate) nbHerbes: i64,
    pub(crate) ModeJoueur2: i16,
    TabIndexHerbeFond: Option<Vec<i16>>,
}

impl Default for Sapcomb {
    fn default() -> Self {
        Self {
            nbMobHerbes: 0,
            nbAvplanSpec: 0,
            NbHerbeAvPlan: 0,
            herbe: [brin {
                type_: 0,
                lin: 0,
                col: 0,
                sens: 0,
            }; NB_MAX_HERBE_AV_PLAN as usize],
            nbHerbes: 0,
            ModeJoueur2: JOUEUR2_AUTONOME,
            TabIndexHerbeFond: None,
        }
    }
}

impl Sapiens {
    /***********************************************************************
                    DECSOU:Essoufflement
    ***********************************************************************/
    pub(crate) fn decsou(&mut self, sap_id: SapId, combien: u8) {
        let s = self.sapglob.sap_mut(sap_id);
        let tfr = s.souffle.wrapping_sub(combien);
        if tfr < s.souffle {
            s.souffle = tfr; /* ok souffle diminue */
            return;
        }
        self.decvie(sap_id, 1); /*plus de souffle puff puff ...*/
    }
}

/***********************************************************************
                INCSOU:Recupere du souffle
***********************************************************************/
pub(crate) const fn incsou(s: &mut sap, combien: u8) {
    let tfr = s.souffle.wrapping_add(combien);
    if tfr > s.souffle {
        s.souffle = tfr;
    } else {
        s.souffle = 255;
    }
}

impl Sapglob {
    /***********************************************************************
                    POSAN:
    ***********************************************************************/
    pub(crate) fn posan(&mut self, sap_id: SapId, mut anim: i16) {
        anim -= 1;
        for i in 0..14 {
            if self.debban[anim as usize][i].is_some() {
                self.sap_mut(sap_id).sapanim[i] = self.debban[anim as usize][i];
            }
        }
    }

    /***********************************************************************
                    ARRET:
    ***********************************************************************/
    pub(crate) fn arret(&mut self, sap_id: SapId) {
        let sapiens = self.sap_mut(sap_id);
        if (sapiens.vitesse & 2) == 0
        /*pas arret*/
        {
            sapiens.vitesse = (sapiens.vitesse ^ 3) & 3;
        }
    }
}

impl Sapiens {
    /***********************************************************************
                            POSVIT:Positionne vitesse
    ***********************************************************************/
    pub(crate) fn posvit(&mut self, sap_id: SapId) {
        let hauteur = self.sapglob.sap(sap_id).hauteur;
        if hauteur == 0 {
            /*debout*/
            let vitesse = {
                let sapiens = self.sapglob.sap_mut(sap_id);
                sapiens.sens = ((sapiens.vitesse - 1) & 2) / 2;
                sapiens.vitesse
            };
            if (vitesse & 2) != 0 {
                self.sapglob.posan(sap_id, 2); /*arret*/
            } else {
                self.sapglob.posan(sap_id, 1);
            }
            let objet = self.sapglob.sap(sap_id).objet;
            self.sapglob.posan(sap_id, objet as i16 + 6);
            return;
        }
        let hauteur = self.sapglob.sap(sap_id).hauteur;
        if hauteur != 1 {
            self.sapglob.arret(sap_id);
            let sapiens = self.sapglob.sap_mut(sap_id);
            sapiens.sens = ((sapiens.vitesse - 1) & 2) / 2;
            return;
        }
        let action = self.sapglob.sap(sap_id).action;
        if action < 128 {
            self.sapglob.sap_mut(sap_id).action |= 128;
            if sap_id == SapId::Sap1 {
                self.decsou(sap_id, 10);
            }
            let (vitesse, sens) = {
                let sapiens = self.sapglob.sap(sap_id);
                (sapiens.vitesse, sapiens.sens)
            };
            self.sapglob
                .posan(sap_id, (((vitesse - 1) / 2) as i16 ^ sens as i16) + 12);
            let sapiens = self.sapglob.sap_mut(sap_id);
            sapiens.vitesse = sapiens.sens + 2;
        }
        self.sapglob.arret(sap_id);
    }
}

impl Sapglob {
    /***********************************************************************
                    POGNE:Saisi objet à la main
    ***********************************************************************/
    pub(crate) fn pogne(&mut self, sap_id: SapId) {
        let sapiens = self.sap_mut(sap_id);
        let mut objet = sapiens.objet + 1;
        if objet > 2 {
            objet = 0;
        }
        sapiens.objet = objet;
        /*si objet est la sagaie et (ou trop pres ou plus de sagaie*/
        if (objet == 2 && (sapiens.nbresag == 0 || (sapiens.action & 3) != 0))
            || (objet == 1 && sapiens.forcehache == 0)
        {
            self.pogne(sap_id); /*recursif*/
        } else {
            self.posan(sap_id, objet as i16 + 6); /*positionne animation*/
        }
    }

    /***********************************************************************
                            JUGE:Etablit les adversaires
    **********************************************************************
    Les adversaires sont positionnes selon les presences et en sortie:
    0:Tout le clan 2 est elimine
    1:Tout le clan 1 est elimine
    2:Il en reste des deux clans
    */
    pub(crate) fn juge(&mut self) -> i16 {
        let mut nbre: i8 = 0;
        /* ----->Mise en place des plans */
        for id in SapId::iter() {
            if self.peuple(id).vie != 0 {
                nbre += 1;
            }
        }
        /*----->nbre est le nombre d'acteurs*/
        /*------>si 4 alors deux couples sur deux plans*/
        if nbre == 4 {
            self.profond[2] = 7;
            self.profond[3] = 7;
        }
        /*sinon tout sur le premier plan*/
        else {
            self.profond[2] = 10;
            self.profond[3] = 10;
        }
        /*dans tout les cas*/
        self.profond[1] = 10;
        self.profond[0] = 10;
        /*si reste seulement clan1 alors sur deux plans*/
        let sap1 = &self.sap1;
        if nbre == 2 && sap1.vie != 0 && self.sap3.vie != 0 {
            self.profond[2] = 7;
        }
        /*On positionne l'adversaire de l'ami*/
        self.sap3.adversaire = if self.sap4.vie != 0 {
            SapId::Sap4
        } else {
            SapId::Sap2
        };
        /*On teste si un des deux clans est vide*/
        if sap1.vie == 0 && self.sap3.vie == 0 {
            return 1; /*clan 1 kaput*/
        }
        if self.sap2.vie == 0 && self.sap4.vie == 0 {
            return 0; /*clan 2 kaput*/
        }
        /* Joueur toujours present ???*/
        if sap1.vie != 0 {
            if self.sap3.vie != 0
            /*il a de l'aide ????*/
            {
                /*oui il a de l'aide*/
                self.sap2.adversaire = SapId::Sap1;
                self.sap4.adversaire = SapId::Sap3;
            } else {
                /*non il est seul */
                self.sap2.adversaire = SapId::Sap1;
                self.sap4.adversaire = SapId::Sap1;
            }
        } else {
            /*l'aide est seule */
            self.sap2.adversaire = SapId::Sap3;
            self.sap4.adversaire = SapId::Sap3;
        }
        2
    }
}

impl Sapiens {
    /**********************************************************************
                            InitGenerique
    ***********************************************************************/
    pub(crate) fn InitGenerique(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        /* on balaye les quatre sapiens*/
        for id in SapId::iter() {
            let l = self.sapglob.peuple(id).poslcoeur;
            let c = self.sapglob.peuple(id).posccoeur;
            self.affomb(self.sapwind.mobFace + 31, l - 2, c - 2, 4, 4, FAUX);
            self.affomb(self.sapwind.mobFace + 30, l - 2 + 22, c - 2, 4, 4, FAUX);
            let mut r = Rect::new(
                (c - 2) * ECHELLE as i16,
                (l - 2) * ECHELLE as i16,
                (c + 16 + 2) * ECHELLE as i16,
                (l + 16 + 2) * ECHELLE as i16,
            );
            r.InsetRect(-1, -1);
            self.sapwind.ports.DoFrameRect(&r);
            r.OffsetRect(0, 22 * ECHELLE as i16);
            self.sapwind.ports.DoFrameRect(&r);
        }
    }
}

impl Sapglob {
    /**********************************************************************
                            EffaceGenerique
    ***********************************************************************/
    fn EffaceGenerique(&self) {
        /* on balaye les quatre sapiens*/
        for sap in self.peuple_iter() {
            let _l = sap.poslcoeur;
            let _c = sap.posccoeur;
            /*    AffDebecr(mobFace+31,l-2,c-2,FAUX);*/
            /*    AffDebecr(mobFace+30,l-2+22,c-2,FAUX);*/
        }
    }
}

impl Sapiens {
    /**********************************************************************
                            GENERIQUE:
    ***********************************************************************/
    pub(crate) fn generique(&mut self, trace: Boolean) {
        /* on balaye les quatre sapiens*/
        for i in SapId::iter() {
            let mut l = self.sapglob.peuple(i).poslcoeur;
            let c = self.sapglob.peuple(i).posccoeur;
            let f = self.sapglob.peuple(i).cross[FormeVue::TETE as usize] as i16 / 2;
            /* fond vide */
            /* Mort => une croix */
            if self.sapglob.peuple(i).vie == 0 {
                self.AffDebecr(self.sapwind.mobFace + 31, l - 2, c - 2, trace as i16); /* rien*/
                self.affomb(DEBCRANE!(self.sapwind) + 10, l, c, 2, 2, trace); /* croix */
                self.affomb(DEBCRANE!(self.sapwind) + 10, l + 22, c, 2, 2, trace); /* croix */
            } else {
                /* Affichage de l'etat de la santé */
                self.AffDebecr(self.sapwind.mobFace + 31, l - 2, c - 2, trace as i16); /* rien*/
                /* Ce n'est pas un animal */
                if self.sapglob.present[i as usize] == 0 {
                    self.AffDebecr(DEBCRANE!(self.sapwind) + 8, l, c, trace as i16);
                }
                /* Affichage de la tete */
                l += 22;
                self.AffDebecr(self.sapwind.mobFace + 30, l - 2, c - 2, trace as i16); /* rien*/
                if self.sapglob.present[i as usize] != 0
                    && (self.sapglob.present[i as usize] as i16) < 5
                {
                    self.AffDebecr(DEBTETE!(self.sapwind) + f, l, c, trace as i16);
                } else {
                    self.AffDebecr(
                        DEBBEST!(self.sapwind) + self.sapglob.present[i as usize] as i16 - 5,
                        l,
                        c,
                        trace as i16,
                    );
                }
            }
        }
    }

    /**********************************************************************
                            AffStrVolatile
    L'utilisateur a cliqué dans la zone haute : on regarde si le click a eu lieu
    dans une case du génerique
    ***********************************************************************/
    fn AffStrVolatile(&mut self, pt: Point, chaine: &str) {
        if !chaine.is_empty() {
            let mut raff = Rect::new(pt.h - 8, pt.v - 8, pt.h + 100, pt.v + 40);
            if raff.left < 0 {
                raff.OffsetRect(-raff.left, 0);
            }
            if raff.right > 640 {
                raff.OffsetRect(640 - raff.right, 0);
            }
            self.sapwind.ports.DoEraseRect(&raff);
            raff.InsetRect(4, 4);
            self.sapwind.ports.TextFont(geneva as i16);
            self.sapwind.ports.DoTextSize(9);
            self.sapwind
                .ports
                .DoTextBox(chaine, &raff, teJustLeft as i16);
            raff.InsetRect(-4, -4);
            self.sapwind.ports.DoFrameRect(&raff);
            self.delai(60);
            self.AttendsFinBouton();
            self.TfrEcr(Some(raff));
        }
    }

    /**********************************************************************
                            ClickInGenerique
    L'utilisateur a clique dans la zone haute : on regarde si le click a eu lieu
    dans une case du generique
    ***********************************************************************/
    pub(crate) fn ClickInGenerique(&mut self, pt: Point) {
        /* on balaye les quatre sapiens*/
        for i in SapId::iter() {
            /* Dans le symbole du coeur */
            let r = Rect::new(
                self.sapglob.peuple(i).posccoeur * ECHELLE as i16 - 4,
                self.sapglob.peuple(i).poslcoeur * ECHELLE as i16 - 4,
                self.sapglob.peuple(i).posccoeur * ECHELLE as i16 + 36,
                self.sapglob.peuple(i).poslcoeur * ECHELLE as i16 + 36,
            );
            if r.pt_in_rect(pt) {
                let chaine: String = if self.sapglob.peuple(i).vie == 0 {
                    self.sapwind.GetStr(19).to_string()
                } else if i as usize == 0 {
                    fmt_nom(self.sapwind.GetStr(15), &self.saptxt.nomjoueur)
                } else if i as usize == 2 {
                    fmt_nom(self.sapwind.GetStr(16), &self.saptxt.nomcopain)
                } else if i as usize == 1 {
                    self.sapwind.GetStr(17).to_string()
                } else {
                    self.sapwind.GetStr(18).to_string()
                };
                self.AffStrVolatile(pt, &chaine);
            }
        }
    }

    /***********************************************************************
                            ABANDO:Laisse objet qd mort
    ***********************************************************************/
    pub(crate) fn abando(&mut self, sap_id: SapId) {
        let animal = self.sapglob.sap_mut(sap_id).animal;
        if animal != 0 {
            /*c'est une bestiole*/
            for _ in 0..qtviande[animal as usize - 1] {
                self.sapglob.ajoutl(ListeId::Liste4, 37);
            }
            match animal {
                1 => {
                    /*lapin*/
                    self.sapglob.ajoutl(ListeId::Liste4, 33); /*peau de lapin*/
                }
                3 => {
                    /*loup*/
                    self.sapglob.ajoutl(ListeId::Liste4, 35);
                    self.sapglob.ajoutl(ListeId::Liste4, 32);
                }
                4 => {
                    /*ours*/
                    self.sapglob.ajoutl(ListeId::Liste4, 34);
                    self.sapglob.ajoutl(ListeId::Liste4, 31);
                }
                _ => {}
            }
        } else {
            /*homme*/
            if self.sapglob.echan > 2
            /* pas ma protection ou rien */
            {
                self.sapglob
                    .ajoutl(ListeId::Liste4, self.sapglob.echan as u16);
            }
            let mut i = {
                let s = self.sapglob.sap(sap_id);
                self.sapglob.liste(s.possession).taille
            };
            while i != 0 {
                i -= 1;
                let element = {
                    let s = self.sapglob.sap(sap_id);
                    self.sapglob.liste(s.possession).elements[i as usize] as u16
                };
                self.sapglob.ajoutl(ListeId::Liste4, element);
            }
            /*on quitte pour les autres morts*/
            let liste_id = self.sapglob.sap_mut(sap_id).possession;
            self.sapglob.liste_mut(liste_id).taille = 0;
            self.sapglob.echan = 0;
        }
    }

    /***********************************************************************
                            DECVIE
    ***********************************************************************/
    pub(crate) fn decvie(&mut self, sap_id: SapId, valeur: u16) {
        let vie = self.sapglob.sap(sap_id).vie;
        if vie == 0 {
            return;
        }
        let tfr = ((vie as i32) - (valeur as i32)) as i16;
        if tfr > 0
        /*si = 0 alors mort*/
        {
            /*ok pas mort*/
            {
                let s = self.sapglob.sap_mut(sap_id);
                s.vie = tfr as u8;
                s.coeur = DEBCRANE!(self.sapwind) + s.vie as i16 / 32;
                s.cptrsouffle = 1;
            }
            /* On va tester si on frappe notre copain */
            if sap_id == SapId::Sap3 && (self.sapglob.presence <= 0) && valeur > 0 {
                /*initialise le sap3 contre nous*/
                self.sapglob.presence = -1;
                self.sapglob.sap3clav = 0;
                let s = self.sapglob.sap_mut(sap_id);
                s.adversaire = SapId::Sap1;
                self.sapglob.viecopain = 0;
            }
        } else {
            /*ok,mort*/
            let (animal, cross) = {
                let s = self.sapglob.sap_mut(sap_id);
                s.vie = 0;
                (s.animal, s.cross[FormeVue::BUSTE as usize])
            };
            if animal <= 0 {
                self.sapglob.sap_mut(sap_id).flg_mort = 16;
                if cross < 8 {
                    self.bruit(11);
                } else {
                    self.bruit(12); /*argh*/
                }
            } else {
                self.sapglob.sap_mut(sap_id).flg_mort = 1;
            }
        }
    }

    /******************************************************************
                        GER_MORT:Traite la mort
    *******************************************************************/
    fn ger_mort(&mut self, sap_id: SapId) {
        /* Si il peut le plancher l'appelle */
        let flg_mort = {
            let (flg_mort, animal, hauteur) = {
                let s = self.sapglob.sap_mut(sap_id);
                if s.flg_mort == 0 {
                    return;
                }
                s.flg_mort -= 1;
                (s.flg_mort, s.animal, s.hauteur)
            };
            if (flg_mort & 2) == 0 && animal == 0 {
                self.baisse(sap_id);
                if hauteur >= 3 {
                    self.sapglob.sap_mut(sap_id).flg_mort = 0;
                }
            }
            flg_mort
        };
        /* on a atteint le sol */
        if flg_mort == 0 {
            if self.sapglob.presence == -1 {
                /* on va baiser Juge*/
                self.sapglob.sap2.vie = self.sapglob.sap3.vie;
                self.sapglob.sap3.vie = 0;
            }
            if sap_id == SapId::Sap3 {
                if self.sapglob.presence == -1 {
                    /*c'est moi qui le frappe */
                    self.sapglob.viecopain = 0;
                    self.sapglob.chef = 0; /*On met le chef a zéro pour le scenar 4*/
                    if self.sapglob.charisme > 10 {
                        self.sapglob.charisme -= 10;
                        self.sapglob.param[2] = self.sapglob.param[2].wrapping_sub(10);
                    }
                } else {
                    self.sapglob.viecopain -= 1;
                }
                self.sapglob.flefba = 0;
                if self.sapglob.viecopain == 0 {
                    self.sapglob.present[2] = 0; /*degenere*/
                    self.comment(112); /*Ton ami est mort*/
                } else {
                    self.comment(111); /*Il est Evanoui*/
                }
            }
            if self.sapglob.presence != self.sapglob.juge() {
                self.sapglob.fin = (self.sapglob.juge() + 1) as u8;
            }
            if sap_id != SapId::Sap1 {
                self.abando(sap_id); /*abandonne objets*/
            }
            /* Il est evanoui on l'affiche a terre*/
            if self.sapglob.sap(sap_id).animal == 0 {
                /*c'est un homme*/
                let profond = self.sapglob.profond[self.sapglob.who as usize];
                let s = self.sapglob.sap_mut(sap_id);
                if s.colonne > LIMITE_DROITE {
                    s.colonne = 1;
                }
                if s.sens != 0 {
                    s.colonne += 8; /* pour centrer le mort */
                }
                let mob = MOB_MORT!(self.sapwind) + s.cross[FormeVue::TETE as usize] as i16 / 2;
                let display_x = 4 * (s.colonne + 8 * (s.colonne < 8) as i16);
                let inverse = s.sens as i16;
                self.AffmobConst(mob, 100 - 34 + profond as i16, display_x, inverse);
            } else {
                let profond = self.sapglob.profond[4];
                let s = self.sapglob.sap_mut(sap_id);
                let mob = MOB_ANIMAL_MORT!(self.sapwind) + s.animal - 1;
                let y = 100 - 20 + profond as i16 - 8 * (s.animal > 2) as i16;
                let x = 4 * s.colonne;
                let inverse = s.sens as i16;
                self.AffmobConst(mob, y, x, inverse);
            }
            /*on me met hors ecran*/
            let s = self.sapglob.sap_mut(sap_id);
            s.colonne = 200;
            self.generique(VRAI);
        }
    }

    /***********************************************************************
                            TOUCHESAG:Teste si Sagaie Touche
    ***********************************************************************/
    /*En entree le Sap qui lance et celui qui est vise
      en sortie la sagaie est positionnee
    */
    pub(crate) fn touchesag(&mut self, sap_id1: SapId, sap_id2: SapId) {
        let mut test: i16;
        /*si l'adversaire est absent sa colonne >64*/
        let (colsag, linsag, snssag, colonne2, hauteur2) = {
            let s1 = self.sapglob.sap(sap_id1);
            let s2 = self.sapglob.sap(sap_id2);
            (s1.colsag, s1.linsag, s1.snssag, s2.colonne, s2.hauteur)
        };
        if colonne2 < 200 && snssag < 128 {
            /*calcul de la colonne du test*/
            test = colsag - colonne2;
            if test < 0 {
                test = -test;
            }
            if test < 3 && linsag > hautou[hauteur2 as usize] as i16 {
                /*ok on touche*/
                {
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    s2.blockimpact = 4;
                }
                self.bruit(1); /* La sagaie touche */
                {
                    let (linsag, colsag, snssag) = {
                        let s1 = self.sapglob.sap(sap_id1);
                        (s1.linsag, s1.colsag, s1.snssag)
                    };
                    let mut tfr = linsag;
                    if tfr > 30 {
                        tfr = 30; /*pas trop bas*/
                    }
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    s2.ligneimpact = tfr as u8;
                    s2.colimpact = (colsag + 6 * (snssag == 0) as i16) as u8;
                }
                self.decvie(sap_id2, 30);
                let (colsag, snssag) = {
                    let s1 = self.sapglob.sap(sap_id1);
                    (s1.colsag, s1.snssag)
                };
                self.AffmobConst(
                    MOB_FLECHE!(self.sapwind),
                    100 - 23 + self.sapglob.profond[self.sapglob.who as usize] as i16,
                    4 * colsag,
                    snssag,
                ); /*à terre*/
                self.sapglob.ajoutl(ListeId::Liste4, 45);
                let s1 = self.sapglob.sap_mut(sap_id1);
                s1.snssag = 255; /*degenere sagaie*/
            } /*si touche*/
        } /*si presence adversaire*/
    }

    /***********************************************************************
                            GERSAG
    ********************************************************************/
    pub(crate) fn gersag(&mut self, sap_id1: SapId) {
        let (colsag, snssag) = {
            let s1 = self.sapglob.sap(sap_id1);
            (s1.colsag, s1.snssag)
        };
        if snssag < 128
        /*sagaie lancee !!!!!*/
        {
            /*si la sagaie est la on l'affiche*/
            let tfr = if snssag != 0 { colsag + 3 } else { colsag - 3 };
            if !(0..=63).contains(&tfr) {
                let s1 = self.sapglob.sap_mut(sap_id1);
                s1.snssag = 255; /*degenere*/
            } else {
                let linsag = {
                    let s1 = self.sapglob.sap_mut(sap_id1);
                    s1.colsag = tfr;
                    s1.linsag += 1;
                    s1.linsag
                };
                if linsag > 45 {
                    let (colsag, snssag) = {
                        let s1 = self.sapglob.sap(sap_id1);
                        (s1.colsag, s1.snssag)
                    };
                    self.AffmobConst(
                        MOB_FLECHE!(self.sapwind),
                        100 - 23 + self.sapglob.profond[self.sapglob.who as usize] as i16,
                        4 * colsag,
                        snssag,
                    ); /*a terre*/
                    if self.random.Rnd() > 200 {
                        self.sapglob.ajoutl(ListeId::Liste4, SAGAIE as u16);
                    } else {
                        self.sapglob.ajoutl(ListeId::Liste4, 45);
                    }
                    let s1 = self.sapglob.sap_mut(sap_id1);
                    s1.snssag = 255; /*degenere*/
                } else {
                    let profond = self.sapglob.profond[self.sapglob.who as usize];
                    let s1 = self.sapglob.sap_mut(sap_id1);
                    let y = s1.linsag + profond as i16 + 32;
                    let x = 4 * s1.colsag;
                    let mob_arrow = MOB_FLECHE!(self.sapwind);
                    let inverse = s1.snssag;
                    self.Affmob(mob_arrow, y, x, inverse);
                }
            }
            /* Si c'est le Joueur il peut toucher les 3 autres */
            if self.sapglob.who == 0 {
                self.touchesag(sap_id1, SapId::Sap2);
                self.touchesag(sap_id1, SapId::Sap3);
                self.touchesag(sap_id1, SapId::Sap4);
            } else {
                /*Si ce n'est pas le joueur il ne vise qu'une personne en particulier*/
                self.touchesag(sap_id1, self.sapglob.sap(sap_id1).adversaire);
            }
        } /*sagaie existe ?*/
    }

    /***********************************************************************
                              SAUT:
    ***********************************************************************/
    pub(crate) fn saut(&mut self, sap_id: SapId) {
        let (hauteur, objet) = {
            let sapiens = self.sapglob.sap_mut(sap_id);
            if sapiens.hauteur == 0 || (sapiens.action & 128) != 0 {
                return;
            }
            sapiens.hauteur -= 1;
            sapiens.action = 0;
            if sapiens.hauteur == 0 {
                self.posvit(sap_id);
                return;
            }
            (sapiens.hauteur, sapiens.objet)
        };
        self.sapglob.posan(sap_id, hauteur as i16 + 8);
        self.sapglob.arret(sap_id);
        self.sapglob.posan(sap_id, objet as i16 + 6);
    }

    /***********************************************************************
                    TOUCHE:
    ***********************************************************************/
    /* en entree le sap qui frappe et celui qui encaisse*/
    pub(crate) fn touche(&mut self, sap_id: SapId, sap_id2: SapId) {
        /*s frappe s2 encaisse*/
        let mut dommage: i16;
        let mut tfr: i16;
        /*calcul de la position du test*/
        let (sens, colonne, hauteur) = {
            let s = self.sapglob.sap(sap_id);
            (s.sens, s.colonne, s.hauteur)
        };
        let coltest = if sens != 0 {
            colonne + self.sapglob.anim_peek() as i16
        } else {
            colonne - self.sapglob.anim_peek() as i16
        };
        /*quel est l'Adversaire*/
        /*valeur absolue de la distance*/
        let (action2, colonne2, hauteur2, sens2) = {
            let s2 = self.sapglob.sap(sap_id2);
            (s2.action, s2.colonne, s2.hauteur, s2.sens)
        };
        tfr = coltest - colonne2;
        if tfr < 0 {
            tfr = -tfr;
        }
        /*le frappe-t'on alors qu'il est assis ?*/
        if tfr > 3 || hauteur2 as i16 > hauteur as i16 + 1 {
            return;
        }
        /*positionne l'impact*/
        let (hauteur, sens) = {
            let s = self.sapglob.sap_mut(sap_id);
            s.blockimpact = 4;
            s.colimpact =
                (colonne2 + if sens2 != 0 { -3 } else { 2 } + 6 * (s.sens != 0) as i16) as u8;
            if s.colimpact > 128 {
                s.colimpact = 0;
            } else if s.colimpact as i16 > LIMITE_DROITE + 3 {
                s.colimpact = (LIMITE_DROITE + 3) as u8;
            }
            s.ligneimpact = tabinp[s.hauteur as usize] as u8;
            (s.hauteur, s.sens)
        };
        if (hauteur as i16 / 2 == hauteur2 as i16 / 2) && (sens != sens2) && (action2 & 3) != 0 {
            /*meme hauteur sens oppose et frappent tout deux*/
            {
                let s = self.sapglob.sap_mut(sap_id);
                s.blockimpact = 2;
            }
            if self.random.Rnd() < 10 {
                /*ok destroy*/
                self.bruit(4);
                let s2 = self.sapglob.sap_mut(sap_id2);
                if s2.objet == 1 {
                    /*destroy hache du sap2*/
                    s2.forcehache = 0;
                    self.sapglob.pogne(sap_id2);
                }
                let s = self.sapglob.sap_mut(sap_id);
                if s.objet == 1 {
                    s.forcehache = 0;
                    self.sapglob.pogne(sap_id);
                }
            }
            /*Rnd insuffisant*/
            else {
                self.bruit(2);
            }
        }
        /*hache se touchent*/
        else {
            /*il recule*/
            let (sens, objet, forcehache, hauteur) = {
                let s = self.sapglob.sap(sap_id);
                (s.sens, s.objet, s.forcehache, s.hauteur)
            };
            let s2 = self.sapglob.sap_mut(sap_id2);
            if sens != 0 {
                if s2.colonne < LIMITE_DROITE - 3 {
                    s2.colonne += 3;
                }
            } else {
                if s2.colonne > LIMITE_GAUCHE + 3 {
                    s2.colonne -= 3;
                }
            }
            dommage = 5;
            if objet == 1 {
                dommage = forcehache as i16;
            }
            if hauteur == 0 {
                dommage *= 2;
            }
            if sap_id2 == SapId::Sap1 {
                dommage -= self.sapglob.resistance as i16 / 50;
            }
            self.decvie(sap_id2, dommage as u16);
            let animal = self.sapglob.sap(sap_id).animal;
            if animal != 0 {
                if self.random.Rnd() > 200 {
                    self.bruit(8);
                }
            } else {
                self.bruit(0);
            }
        }
    }

    /***********************************************************************
                    GCOEUR:
    ***********************************************************************/
    fn gcoeur(&mut self, sap_id: SapId) {
        let s = self.sapglob.peuple_mut(sap_id);
        let mut quoi: i16;
        if s.cptrsouffle < 32 {
            s.cptrsouffle = s.souffle as i16;
            quoi = s.coeur;
            if quoi < 0 {
                quoi = 1 - quoi;
            }
            let ligne = s.poslcoeur;
            let colonne = s.posccoeur;
            self.affomb(quoi, ligne, colonne, 0, 0, VRAI);
            let s = self.sapglob.peuple_mut(sap_id);
            s.coeur = -s.coeur;
        } else {
            s.cptrsouffle -= 32;
        }
    }

    /***********************************************************************
                            IMPAK:Gere les impacts
    ***********************************************************************/
    fn impak(&mut self, s_id: SapId) {
        let profond = self.sapglob.profond[self.sapglob.who as usize];
        let s = self.sapglob.peuple_mut(s_id);
        if s.blockimpact != 0 {
            let mob = s.blockimpact as i16 + MOB_IMPACT!(self.sapwind);
            let y = s.ligneimpact as i16 + profond as i16 + 32;
            let x = 4 * s.colimpact as i16;
            let inverse = s.sens as i16;
            self.Affmob(mob, y, x, inverse);
            let s = self.sapglob.peuple_mut(s_id);
            s.blockimpact -= 1;
            /* sang par terre et dec impact */
            if s.blockimpact == 0 {
                let mob = MOB_SANG_A_TERRE!(self.sapwind);
                let y = 100 + profond as i16 - 19 - self.random.myRandom(0, 4) as i16;
                let x = 4 * s.colimpact as i16;
                let inverse = s.sens as i16;
                self.AffmobConst(mob, y, x, inverse);
            }
        }
    }

    /***********************************************************************
                            AVASAP:
            (c) God! Departement Intelligence Artificielle
                    Avril '86-   Décembre '86
    ***********************************************************************/
    fn avasap(&mut self, sap_id: SapId) {
        let s = self.sapglob.sap(sap_id);
        if s.sens == 0 {
            self.decvit(sap_id);
        } else {
            self.incvit(sap_id);
        }
    }

    /***********************************************************************
                            RECUSAP:
    ***********************************************************************/
    fn recusap(&mut self, sap_id: SapId) {
        let s = self.sapglob.sap(sap_id);
        if s.hauteur == 1 && (s.action & 128) == 0 {
            if s.sens == 0 {
                self.incvit(sap_id);
            } else {
                self.decvit(sap_id);
            }
        }
    }

    /***********************************************************************
                            FIXCOLON
    ***********************************************************************/
    pub(crate) fn fixcolon(&mut self, sap_id: SapId) {
        /* si ami alors meme place*/
        if sap_id == SapId::Sap3 {
            let colonne = self.sapglob.sap1.colonne;
            let sens = self.sapglob.sap1.sens;
            let s = self.sapglob.sap_mut(sap_id);
            s.colonne = colonne;
            s.sens = sens;
        } else {
            let who = self.sapglob.who;
            let s = self.sapglob.sap_mut(sap_id);
            if (sap_id == SapId::Sap1) && (s.colonne > 64) {
                s.colonne = 2;
                s.sens = 1;
                return;
            }
            if s.colonne > 32 {
                if sap_id != SapId::Sap1 {
                    s.sens = 0;
                    s.colonne = 53 + who as i16;
                } else {
                    s.colonne = 55;
                }
            } else {
                if sap_id != SapId::Sap1 {
                    s.sens = 1;
                    s.colonne = 5 - who as i16;
                } else {
                    s.colonne = 1;
                }
            }
        }
    }

    /***********************************************************************
                            INIUNE:Inits primaires de vue lat
    ***********************************************************************/
    pub(crate) fn iniune(&mut self, sap_id: SapId) {
        {
            {
                let s = self.sapglob.sap_mut(sap_id);
                s.sapanim[10] = Some(AnimPtr::new(AnimTable::Aneutr)); /*supprime arme*/
                s.sapanim[11] = Some(AnimPtr::new(AnimTable::Aneutr));
                s.action = 0;
                s.ligne = 2;
                s.hauteur = 0;

                s.objet = 0;
            }
            self.sapglob.posan(sap_id, 2); /*arret par defaut*/
            let s = self.sapglob.sap_mut(sap_id);
            if s.colonne > 32 {
                s.sens = 0;
            } else {
                s.sens = 1;
            }
        }
        self.fixcolon(sap_id);
        let s = self.sapglob.sap_mut(sap_id);
        s.vitesse = s.sens | 2; /*arret*/
        s.animal = 0;
        s.deltah = 0;
        s.souffle = 200;
        s.cptrsouffle = 1;
    }

    /***********************************************************************
                    PRM:Initialise Rencontres
    ***********************************************************************
    => Type de la présence,(chef):
    0 non present;
    1 Pied agile,2 Yeux malins,3 Bras de roc,4 Hyenes folles
    5 Loup,6 ours,7 Lapin,8 Lagopede,0 rien ,-1 Joueur
    */
    pub(crate) fn prm(&mut self, sap_id: SapId, type_: i16) {
        let mut i;

        {
            {
                let s = self.sapglob.sap_mut(sap_id);
                s.snssag = 255;
                s.fuite = 0;
                s.blockimpact = 0;
                s.flg_mort = 0;
            }
            /*On va redeterminer who pour iniune*/
            if sap_id == SapId::Sap1 {
                self.sapglob.who = 0;
            }
            if sap_id == SapId::Sap2 {
                self.sapglob.who = 1;
            }
            if sap_id == SapId::Sap3 {
                self.sapglob.who = 2;
            }
            if sap_id == SapId::Sap4 {
                self.sapglob.who = 3;
            }
            self.sapglob.present[self.sapglob.who as usize] = type_ as i8;
            if type_ == 0 {
                /*non present*/
                let s = self.sapglob.sap_mut(sap_id);
                s.colonne = 200;
                s.vie = 0;
                return;
            }
        }
        if type_ < 0 || (self.sapglob.who == 2 && type_ < 5)
        /*joueur ou aide humaine*/
        {
            if type_ == -1 {
                self.sapglob.sap1.vie = ((self.sapglob.vie as u32 * 50) / 256
                    + (self.sapglob.bless as u32 * 93) / 256
                    + (self.sapglob.faim as u32 * 46) / 256
                    + (self.sapglob.soif as u32 * 46) / 256)
                    as u8;
            } else {
                let viecopain = self.sapglob.viecopain;
                let s = self.sapglob.sap_mut(sap_id);
                s.vie = (viecopain as u16 * 60 + 20) as u8;
            }
            {
                let s = self.sapglob.sap_mut(sap_id);
                if !(0..4).contains(&type_) {
                    // Note : bug dans le code original
                    s.forcehache = 0;
                } else {
                    s.forcehache = tribforce[type_ as usize - 1] as u8;
                }
            }
            self.iniune(sap_id);
            self.decvie(sap_id, 0); /* met en place coeur*/
            return;
        }
        /*raz du cross du sap*/
        {
            let s = self.sapglob.sap_mut(sap_id);
            for i in 0..7 {
                s.cross[i] = 0;
            }
        }

        if type_ < 5 {
            /*homme*/
            loop {
                self.sapglob.sap_mut(sap_id).cross[FormeVue::BUSTE as usize] =
                    (2 * self.random.myRandom(0, 4)) as u8;
                if self.sapglob.chef > 0 && sap_id == SapId::Sap2 {
                    i = 8; /*gueule de chef*/
                    self.sapglob.sap_mut(sap_id).cross[FormeVue::BUSTE as usize] = 0;
                } else {
                    if self.random.Rnd() < 100 {
                        /*femelles*/
                        i = 12 + (self.random.Rnd() / 43) * 2; /*selectionne tete*/
                        self.sapglob.sap_mut(sap_id).cross[FormeVue::BUSTE as usize] += 8; /*buste*/
                    } else {
                        /*male*/
                        i = 2 * (self.random.Rnd() & 3); /*gueule*/
                    }
                }
                if type_ == 4 {
                    /*hyene folle*/
                    i = 10; /*masque hyene folle*/
                }
                if i != self.sapglob.sap1.cross[FormeVue::TETE as usize] as i16 {
                    break;
                }
            }
            let s = self.sapglob.sap_mut(sap_id);
            s.cross[FormeVue::TETE as usize] = i as u8; /*tete*/
            /*masse ou hache*/
            s.cross[FormeVue::ARME as usize] = (self.random.Rnd() & 1) as u8;
        }

        /*si homme ou bestiole*/

        /*colonne sap2*/
        {
            let colonne = self.sapglob.sap1.colonne;
            let s = self.sapglob.sap_mut(sap_id);
            if colonne > 32 {
                s.colonne = 2;
            } else {
                s.colonne = 54;
            }
        }
        self.iniune(sap_id);
        /*si Homme*/
        if type_ <= 4 {
            let agress = self.sapglob.agress;
            let chef = self.sapglob.chef;
            let tribu = self.sapglob.tribu;
            let s = self.sapglob.sap_mut(sap_id);
            s.nbresag = 4; /*4 sagaies*/
            s.agressivite = (agress as u16 / 256) as u8;
            s.forcehache = tribforce[type_ as usize - 1] as u8;
            s.vie = tribvie[type_ as usize - 1] as u8;
            if chef > 0 && tribu == 1 {
                /*si notre chef alors dur dur*/
                s.agressivite = 255;
                s.forcehache = 40;
                s.vie = 200;
            }
            if type_ == 0 {
                /*personne*/
                s.sapanim[0] = None; /*pas généré*/
                s.sapanim[1] = None;
                s.colonne = 200;
                s.vie = 0;
            }
            self.posvit(sap_id);
        } else {
            /*bestiole*/
            if type_ >= 7 {
                let s = self.sapglob.sap_mut(sap_id);
                s.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlap1));
            } else {
                let s = self.sapglob.sap_mut(sap_id);
                s.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlou1));
            }
            let s = self.sapglob.sap_mut(sap_id);
            s.sapanim[0] = s.sapanim[1];
            s.sapanim[2] = None; /*plus d'animation apres 1*/
            s.sapanim[3] = None;
            s.cross[0] = (((type_ & 1) ^ 1) * 4) as u8;
            s.animal = ((type_ - 5) ^ 2) + 1;
            s.vie = bestvie[type_ as usize - 5] as u8;
            s.ligne = bestlin[type_ as usize - 5] as u8;
            s.hauteur = besthaut[type_ as usize - 5] as u8;
            s.agressivite = bestagress[type_ as usize - 5] as u8;
            s.vitesse = 0; /*fuite à 0*/
            s.objet = 1; /*pour frappe*/
            s.forcehache = 10; /*dents*/
        }
        self.decvie(sap_id, 0); /* met en place coeur*/
        self.generique(FAUX);
    }

    /***********************************************************************
                    BAISSE:
    ***********************************************************************/
    pub(crate) fn baisse(&mut self, sap_id: SapId) {
        let (mut hauteur, objet) = {
            let s = self.sapglob.sap(sap_id);
            (s.hauteur, s.objet)
        };
        if hauteur < 3 {
            /*on peut encore descendre*/
            {
                let s = self.sapglob.sap_mut(sap_id);
                hauteur += 1;
                s.hauteur = hauteur; /*donc on le fait*/
                s.action = 0;
            }
            self.sapglob.posan(sap_id, hauteur as i16 + 8);
            self.sapglob.arret(sap_id);
            self.sapglob.posan(sap_id, objet as i16 + 6);
        }
    }

    /***********************************************************************
                    ACTBRA:
    **********************************************************************/
    pub(crate) fn actbra(&mut self, sap_id: SapId) {
        let (action, souffle) = {
            let s = self.sapglob.sap(sap_id);
            (s.action, s.souffle)
        };
        if (action & 3) == 0 && (souffle > 32 || self.sapglob.who != 0)
        /*deja action (ou essouffle)*/
        {
            if sap_id == SapId::Sap1 {
                self.decsou(sap_id, 20);
            }
            let objet = {
                let s = self.sapglob.sap_mut(sap_id);
                s.action |= s.objet + 1;
                if s.objet == 2
                /*objet est la sagaie*/
                {
                    if s.snssag > 128 && s.nbresag != 0
                    /*deja sagaie ? */
                    {
                        s.colsag = s.colonne + 2;
                        s.nbresag -= 1;
                        s.linsag = hautlin[s.hauteur as usize] as i16;
                        s.snssag = s.sens as i16;
                    }
                    /*si sagaie n'existe pas deja*/
                    else {
                        return;
                    }
                } /*si objet =sagaie*/
                s.objet
            };
            self.sapglob.posan(sap_id, objet as i16 + 3);
        } /*si souffle correct*/
    }

    /***********************************************************************
                    DEPLAT:Deplacement de vue laterale
    ***********************************************************************/
    pub(crate) fn deplat(&mut self) {
        /*position en colonne*/
        let sap1 = &mut self.sapglob.sap1;
        if sap1.colonne < 32 {
            self.sapglob.ldzeta = (self.sapglob.dzeta + 512) & 0x03FF;
            sap1.colonne = LIMITE_DROITE - 1;
        } else {
            self.sapglob.ldzeta = self.sapglob.dzeta;
            sap1.colonne = LIMITE_GAUCHE + 1;
        }
        self.sapdep(self.sapglob.ldzeta, 46); /* 46 est la vitesse*/
        self.sapglob.minute = 0;
        self.chrono(0);
    }

    /***********************************************************************
                    MVTSAP:Gere les mouvements
    **********************************************************************
    => Pointeur sur le sap a etudier
    <= 1 si il faut le rappeler immediatement 0 sinon*/
    pub(crate) fn mvtsap(&mut self, sap_id: SapId) -> i16 {
        let mut nummob: i16 = 0;
        let mut tfr;
        let mut lastcol: i16;

        {
            let sapiens = self.sapglob.sap_mut(sap_id);
            self.sapglob.sensmob = sapiens.sens;
        }
        let mut registerb: i16 = 0; /*reg b balaye les differents morceaux*/
        let mut ptrcross: i16 = 0;
        loop {
            /*infini*/
            /*on prend l'animation*/
            {
                let sapiens = self.sapglob.sap_mut(sap_id);
                self.sapglob.ptranim = sapiens.sapanim[registerb as usize];
            }
            /*teste si fin de sap:animation=0*/
            if self.sapglob.ptranim.is_none() {
                return 0;
            }
            let mut ok: i16 = 0;
            while ok == 0 {
                /*ok*/
                tfr = self.sapglob.anim_read(); /*caractere suivant*/
                /*teste si le caractere lu est une commande*/
                if tfr >= 0
                /*non :pas une commande*/
                {
                    ok += 1;
                    nummob = tfr as i16;
                } else
                /*****************************/
                /*oui !:on traite la commande*/
                {
                    if tfr == -1
                    /* 255 fin d'animation*/
                    {
                        /*l'animation revient a l'animation standard*/
                        let sapiens = self.sapglob.sap_mut(sap_id);
                        self.sapglob.ptranim = sapiens.sapanim[registerb as usize + 1];
                        continue; /*on revient a la lecture de l'animation*/
                    }
                    tfr &= 127;
                    if tfr == 0
                    /*deplacement vertical*/
                    {
                        /*le caractere suivant donne la hauteur*/
                        let x = self.sapglob.anim_peek() as u8;
                        let sapiens = self.sapglob.sap_mut(sap_id);
                        sapiens.deltah = x;
                        self.sapglob.anim_skip();
                        continue;
                    }
                    if tfr == 1
                    /*deplacement horizontal*/
                    {
                        if registerb == 0
                        /*uniquement sur 1ere jambe*/
                        {
                            let colonne = {
                                let sensmob = self.sapglob.sensmob;
                                let sapiens = self.sapglob.sap_mut(sap_id);
                                lastcol = sapiens.colonne;
                                if sensmob != 0
                                /*deplacement vers la droite*/
                                {
                                    sapiens.colonne += 1;
                                } else {
                                    sapiens.colonne -= 1;
                                }
                                sapiens.colonne
                            };
                            if (self.sapglob.sensmob == 0 && colonne <= LIMITE_GAUCHE)
                                || (self.sapglob.sensmob == 1 && colonne > LIMITE_DROITE)
                            /*sortie !!!!!*/
                            {
                                if self.sapglob.presence != 0 {
                                    {
                                        let sapiens = self.sapglob.sap_mut(sap_id);
                                        sapiens.colonne = lastcol;
                                    }
                                    if (self.sapglob.present[self.sapglob.who as usize] as i16) < 5
                                    {
                                        self.sapglob.arret(sap_id);
                                        let hauteur = self.sapglob.sap(sap_id).hauteur;
                                        if hauteur != 0 {
                                            self.sapglob.posan(sap_id, hauteur as i16 + 8);
                                        } else {
                                            self.sapglob.posan(sap_id, 2);
                                        }
                                        let objet = self.sapglob.sap(sap_id).objet;
                                        self.sapglob.posan(sap_id, objet as i16 + 6);
                                        self.sapglob.sap_mut(sap_id).action &= 127;
                                        return 1; /*on le refait*/
                                    }
                                } else {
                                    self.sapglob.fin = 128;
                                    if sap_id == SapId::Sap3 {
                                        let sap1 = &mut self.sapglob.sap1;
                                        let sap3 = &self.sapglob.sap3;
                                        if sap1.vie == 0 {
                                            self.sapglob.fin = 129;
                                        }
                                        sap1.colonne = sap3.colonne;
                                        sap1.sens = sap3.sens;
                                    }
                                    return 0;
                                }
                            }
                        }
                        continue;
                    }
                    if tfr == 2
                    /*empreinte*/
                    {
                        let profond = self.sapglob.profond[self.sapglob.who as usize];
                        let sapiens = self.sapglob.sap_mut(sap_id);
                        let inverse = sapiens.sens as i16;
                        let x = 4 * (sapiens.colonne + inverse * 5);
                        let mob = MOB_EMPREINTE!(self.sapwind);
                        self.AffmobConst(mob, 100 + profond as i16 - 19, x, inverse);
                        continue;
                    }
                    if tfr == 3
                    /*neutre*/
                    {
                        ptrcross += 1;
                        ok = 2; /*on passe au morceau suivant*/
                        break;
                    }
                    if tfr == 4
                    /*depositionne baisse*/
                    {
                        let sapiens = self.sapglob.sap_mut(sap_id);
                        sapiens.hauteur -= 1;
                        self.sapglob.arret(sap_id);
                        self.sapglob.posan(sap_id, 10); /*a genoux*/
                        return 1; /*on le refait*/
                    }
                    if tfr == 5
                    /*recule*/
                    {
                        let sensmob = self.sapglob.sensmob;
                        let presence = self.sapglob.presence;
                        let sapiens = self.sapglob.sap_mut(sap_id);
                        if registerb == 0 {
                            lastcol = sapiens.colonne;
                            if sensmob != 0 {
                                sapiens.colonne -= 1;
                            } else {
                                sapiens.colonne += 1;
                            }
                            if (sensmob == 1 && sapiens.colonne < LIMITE_GAUCHE + 3)
                                || (sensmob == 0 && sapiens.colonne > LIMITE_DROITE + 3)
                            /*sortie !!!!!*/
                            {
                                if presence != 0 {
                                    sapiens.colonne = lastcol;
                                } else {
                                    self.sapglob.fin = 128;
                                    if sap_id == SapId::Sap3 {
                                        let sap1 = &mut self.sapglob.sap1;
                                        let sap3 = &self.sapglob.sap3;
                                        if sap1.vie == 0 {
                                            self.sapglob.fin = 129;
                                        }
                                        sap1.colonne = sap3.colonne;
                                        sap1.sens = sap3.sens;
                                    }
                                    return 0;
                                }
                            }
                        }
                        continue;
                    }
                    if tfr == 6 {
                        /*fin d'arme*/
                        let x = self.sapglob.anim_peek() as u8;
                        let sapiens = self.sapglob.sap_mut(sap_id);
                        sapiens.action &= x;
                        self.sapglob.anim_skip();
                        continue;
                    }
                    if tfr == 7 {
                        /*teste touche*/
                        let who = self.sapglob.who;
                        /* Si c'est le Joueur il peut toucher les 3 autres */
                        if who == 0 {
                            /* Frappe le copain ????*/
                            if self.sapglob.presence < 1 {
                                self.touche(sap_id, SapId::Sap3);
                            } else {
                                self.touche(sap_id, SapId::Sap2);
                                self.touche(sap_id, SapId::Sap4);
                            }
                        } else {
                            /*Si ce n'est pas le joueur il ne vise qu'une personne en particulier*/
                            let adversaire = self.sapglob.sap_mut(sap_id).adversaire;
                            self.touche(sap_id, adversaire);
                        }
                        self.sapglob.anim_skip();
                        continue;
                    }
                    if tfr == 8 {
                        /*fin sagaie*/
                        self.sapglob.pogne(sap_id);
                        continue;
                    }
                }
            } /*ok*/
            if ok == 1
            /*ok on y va sinon anim suiv*/
            {
                let (ligne, sens, animal) = {
                    let sapiens = self.sapglob.sap(sap_id);
                    (sapiens.ligne, sapiens.sens, sapiens.animal)
                };
                /*Le caractere lu a ete traite soit comme numero de block
                soit comme commande*/
                let mut linmob: i16 = ligne as i16 + self.sapglob.anim_read() as i16;
                tfr = self.sapglob.anim_read();
                if sens != 0 {
                    tfr = (-(tfr as i16) + 6) as i8;
                    /* teste l'ours*/
                    if animal == 4 {
                        tfr += 8;
                    }
                }

                let ptranim = self.sapglob.ptranim;
                let who = self.sapglob.profond[self.sapglob.who as usize];
                let sapiens = self.sapglob.sap_mut(sap_id);
                sapiens.sapanim[registerb as usize] = ptranim; /*on memorise la nlle position*/
                let colmob: i16 = tfr as i16 + sapiens.colonne;
                if colmob > 127 {
                    return 0;
                }
                if nummob > 10 {
                    linmob += sapiens.deltah as i16;
                }
                let mob =
                    nummob + sapiens.cross[ptrcross as usize] as i16 + MOB_CORPS!(self.sapwind);
                let inverse = sapiens.sens as i16;
                self.Affmob(
                    mob,
                    (linmob as i8) as i16 + who as i8 as i16 + 32,
                    4 * colmob,
                    inverse,
                );
            }
            registerb += 2; /*morceau suivant*/
        } /*infini*/
    } /*fin de procedure*/

    /***********************************************************************
                            DECVIT:Diminue vitesse
    ***********************************************************************/
    pub(crate) fn decvit(&mut self, sap_id: SapId) {
        let sapiens = self.sapglob.sap_mut(sap_id);
        let tfrvit = sapiens.vitesse.wrapping_sub(1);
        if tfrvit != 0 {
            sapiens.vitesse = tfrvit;
            self.posvit(sap_id);
        }
    }

    /***********************************************************************
                            INCVIT:Augmente vitesse
    ***********************************************************************/
    pub(crate) fn incvit(&mut self, sap_id: SapId) {
        let sapiens = self.sapglob.sap_mut(sap_id);
        let tfrvit = sapiens.vitesse.wrapping_add(1);
        if tfrvit < 5
        /* varie de 1 a 4 */
        {
            sapiens.vitesse = tfrvit;
            self.posvit(sap_id);
        }
    }

    /***********************************************************************
                    CLAV:Clavier durant combat
    ***********************************************************************/
    pub(crate) fn clav(&mut self, sap_id: SapId, caract: i16) {
        if caract == 0 {
            return;
        }
        if (caract & 4) != 0 {
            self.decvit(sap_id);
        } else if (caract & 8) != 0 {
            self.incvit(sap_id);
        }
        if (caract & 1) != 0 {
            self.saut(sap_id);
        } else if (caract & 2) != 0 {
            self.baisse(sap_id);
        }
        if ((caract & 18) == 18 || (caract & 64) != 0) && self.sapglob.presence != -1 {
            self.sapglob.fin = 129;
        }
        if (caract & 16) != 0 {
            self.sapglob.pogne(sap_id);
            while (self.getkey(0) & 16) != 0 {}
        }
        if (caract & 32) != 0 {
            self.actbra(sap_id);
        }
    }

    /***********************************************************************
                         GERCOP:Gere l'ami humain ou non
    ***********************************************************************/
    /* sap 1 c'est le joueur sap3 le copain*/
    pub(crate) fn gercop(&mut self, scop_id: SapId, type_: i16) {
        let colonne = self.sapglob.sap_mut(scop_id).colonne;
        {
            let sap1 = &self.sapglob.sap1;
            self.sapglob.distsp = sap1.colonne - colonne; /*distance entre saps*/
        }
        let sens = if self.sapglob.distsp < 0 {
            /*valeur absolue*/
            self.sapglob.distsp = -self.sapglob.distsp;
            0
        } else {
            1
        };

        /*dans le cas ou on gere le sap3 seul avec nous*/
        /*si le sap1 est mort alors il va tendre a sortir*/
        if self.sapglob.distsp > 3 {
            /*encore loin*/
            self.sapglob.sap_mut(scop_id).sens = sens; /*vers le joueur*/
            if type_ != 0 {
                /* c'est un humain*/
                if self.sapglob.sap(scop_id).hauteur != 0 {
                    self.saut(scop_id); /*si baisse alors monte*/
                } else {
                    self.avasap(scop_id); /*il marche*/
                }
            } else {
                /*c'est une bestiole*/
                if (self.sapglob.present[self.sapglob.who as usize] as i16) > 6 {
                    let scop = self.sapglob.sap_mut(scop_id);
                    scop.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlap2));
                } else {
                    let scop = self.sapglob.sap_mut(scop_id);
                    scop.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlou2));
                }
            }
        } else {
            /* deja present*/
            if type_ == 0 {
                /*si bestiole*/
                let sens = self.sapglob.sap1.sens;
                {
                    let scop = self.sapglob.sap_mut(scop_id);
                    scop.sens = sens;
                }
                /*arrete*/
                if (self.sapglob.present[self.sapglob.who as usize] as i16) > 6 {
                    let scop = self.sapglob.sap_mut(scop_id);
                    scop.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlap1));
                    scop.sapanim[0] = Some(AnimPtr::new(AnimTable::Anlap1));
                } else {
                    let scop = self.sapglob.sap_mut(scop_id);
                    scop.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlou1));
                    scop.sapanim[0] = Some(AnimPtr::new(AnimTable::Anlou1));
                }
                let scop = self.sapglob.sap_mut(scop_id);
                scop.coeur = DEBCRANE!(self.sapwind) + 8;
                scop.cptrsouffle = 1;
                return;
            }
            {
                let scop = self.sapglob.sap_mut(scop_id);
                /*si humain*/
                if scop.objet != 0 {
                    self.sapglob.pogne(scop_id); /* plus d'objet a la main*/
                }
            }
            if self.sapglob.sap(scop_id).hauteur != 0 {
                self.saut(scop_id); /*tjs debout*/
            } else {
                let vitesse = self.sapglob.sap1.vitesse;
                let scop = self.sapglob.sap_mut(scop_id);
                scop.vitesse = vitesse; /*meme sens*/
                self.sapglob.arret(scop_id); /*arrete :vitesse*/
                self.posvit(scop_id); /*il va s'arreter,positionner le sens et l'anim*/
            }
        }
    }

    /***********************************************************************
                    AUTONHOM:
    ***********************************************************************/
    pub(crate) fn autonhom(&mut self, sap_id2: SapId, sap_id1: SapId) {
        /*
        test specifique au copain alors que je suis seul a me battre contre un
        adversaire, Sapiens etant un jeu eminement MORAL, mon ami peut refuser
        de frapper le meme adversaire que moi...(2 Sept 87)
        */
        if sap_id2 == SapId::Sap3
        /* si c'est mon copain */
        {
            /* si je suis en vie et qu'il n'y a pas d'adversaire pour
               mon ami et que le Rnd est favorable... Il bouge tout de meme
               un peu pour donner le change...
            Mais qu'est ce qu'il me prends de commenter aussi ABONDAMENT ?
            alors que cette trapsure est au moins aussi geniale et
            infaillible que le reste du programme.
               */
            let sap1 = &self.sapglob.sap1;
            let sap4 = &self.sapglob.sap4;
            if sap1.vie != 0 && sap4.vie == 0 && self.random.Rnd() > 60 {
                return; /* puissant non ???*/
            }
        }

        self.sapglob.distsp =
            self.sapglob.sap(sap_id1).colonne - self.sapglob.sap_mut(sap_id2).colonne; /*distance entre saps*/
        let sens = if self.sapglob.distsp < 0 {
            /*valeur absolue*/
            self.sapglob.distsp = -self.sapglob.distsp;
            1
        } else {
            0
        };

        let hauteur = self.sapglob.sap(sap_id1).hauteur;
        let (hauteur2, sens2) = {
            let s2 = self.sapglob.sap_mut(sap_id2);
            if s2.vie < 20 || self.random.Rnd() > 160 {
                if hauteur > 1 && self.random.Rnd() > 128 {
                    s2.fuite = 4;
                } else if s2.fuite != 0 && self.random.Rnd() > 200 {
                    s2.fuite = self.random.Rnd() / 13;
                }
            }
            (s2.sens, s2.hauteur)
        };
        if sens == sens2 {
            if hauteur2 != 0 {
                self.saut(sap_id2); /*il se releve*/
            }
            self.sapglob.sap_mut(sap_id2).sens ^= 1; /*il se retourne*/
            return;
        }
        /* La distance entre les 2 saps est inferieure a 8*/
        if self.sapglob.distsp > 8
        /*peut tirer*/
        {
            let s2 = self.sapglob.sap_mut(sap_id2);
            if s2.objet == 2
            /*sagaie*/
            {
                if self.random.Rnd() < s2.agressivite as i16 {
                    self.actbra(sap_id2); /*il frappe*/
                }
            } else
            /*si pas sagaie en main il va la prendre*/
            {
                if self.random.Rnd() < 20 && s2.nbresag != 0 {
                    self.sapglob.pogne(sap_id2); /*il essai*/
                    return;
                }
            }
            if self.random.Rnd() < 60 {
                /* test si copain et lapin en face */
                if sap_id2 != SapId::Sap3 || (self.sapglob.present[1] as i16) < 7 {
                    self.avasap(sap_id2);
                }
            } else {
                self.saut(sap_id2);
            }
        } else {
            /*assez pres*/
            let (fuite2, hauteur2) = {
                let s2 = self.sapglob.sap_mut(sap_id2);
                (s2.fuite, s2.hauteur)
            };
            if self.sapglob.distsp < 2 || fuite2 != 0
            /*beaucoup trop pres*/
            {
                /*il tend a se mettre accroupis puis a reculer*/
                if hauteur2 > 1 {
                    self.saut(sap_id2);
                    return;
                }
                let s2 = self.sapglob.sap_mut(sap_id2);
                if s2.hauteur == 0 {
                    self.baisse(sap_id2);
                    return;
                }
                if s2.fuite != 0 {
                    s2.fuite -= 1;
                }
                self.recusap(sap_id2);
                return;
            }
            {
                let s2 = self.sapglob.sap_mut(sap_id2);
                if s2.hauteur < 2 && self.random.Rnd() > 180 && (s2.action & 128) == 0 {
                    self.baisse(sap_id2);
                }
            }
            if self.sapglob.distsp < 6 {
                /*il peut frapper*/
                let chance = {
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    (s2.souffle as i32 * s2.agressivite as i32) / 512 + 128
                };
                if (self.random.Rnd() as i32) < chance {
                    self.actbra(sap_id2); /*il frappe*/
                    return;
                } else {
                    /*ne frappe pas:alors bouge*/
                    let distsp = self.sapglob.distsp;
                    let action = self.sapglob.sap(sap_id1).action;
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    if (action != 0 || distsp < 4) && (s2.action & 128) == 0 {
                        if s2.hauteur < 2 {
                            self.recusap(sap_id2);
                        } else if self.random.Rnd() > 160 {
                            self.baisse(sap_id2);
                        } else {
                            self.saut(sap_id2);
                        }
                    } else {
                        self.baisse(sap_id2);
                    }
                }
            } else
            /*proche mais pas assez pour frapper*/
            {
                let s2 = self.sapglob.sap(sap_id2);
                if s2.hauteur >= 2 {
                    self.saut(sap_id2); /*il se releve*/
                    return;
                }
                if s2.objet != 1 && self.random.Rnd() > 70 {
                    self.sapglob.pogne(sap_id2); /*prends sa hache*/
                }
                if self.random.Rnd() > 80 {
                    self.avasap(sap_id2);
                }
                return;
            }
            let (action, hauteur) = {
                let s1 = self.sapglob.sap(sap_id1);
                (s1.action, s1.hauteur)
            };
            if (action & 3) != 0 {
                if hauteur >= 2 && self.random.Rnd() < 10 {
                    self.baisse(sap_id2);
                } else {
                    self.saut(sap_id2);
                }
                return;
            }
            let s2 = self.sapglob.sap_mut(sap_id2);
            if self.random.Rnd() > s2.agressivite as i16 && s2.hauteur >= 2 {
                self.recusap(sap_id2);
            }
        }
    }

    /***********************************************************************
                            BESTIO:Gere les Bestioles
    ***********************************************************************/
    pub(crate) fn bestio(&mut self, sap_id2: SapId, sap_id1: SapId) {
        self.sapglob.distsp = {
            let s1 = self.sapglob.sap(sap_id1);
            let s2 = self.sapglob.sap(sap_id2);
            s1.colonne - s2.colonne
        };
        let sens = if self.sapglob.distsp < 0 {
            self.sapglob.distsp = -self.sapglob.distsp;
            1
        } else {
            0
        };

        let animal2 = self.sapglob.sap(sap_id2).animal;
        /*1=Lapin,2=Lagopede,3=Loup,4=Ours*/
        if animal2 >= 3 {
            /*loup-Ours*/
            /* si cote a gauche ou a droite*/
            let colonne = self.sapglob.sap(sap_id2).colonne;
            if colonne < LIMITE_GAUCHE - 3 {
                let s2 = self.sapglob.sap_mut(sap_id2);
                s2.sens = 1;
                s2.colsag = 10 + self.random.Rnd() / 20;
                s2.fuite = 2;
            }
            if colonne > LIMITE_DROITE + 3 {
                let s2 = self.sapglob.sap_mut(sap_id2);
                s2.sens = 0;
                s2.colsag = 10 + self.random.Rnd() / 20;
                s2.fuite = 2;
            }
            /*si blesse ou avec une faible prob retrait*/
            let vie = self.sapglob.sap_mut(sap_id2).vie;
            if (vie < 40 && self.random.Rnd() > 180) || self.random.Rnd() < 10 {
                let fuite = self.sapglob.sap(sap_id2).fuite;
                if fuite == 0 {
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    s2.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlou2));
                    s2.fuite = 1;
                    s2.colsag = self.random.Rnd() / 12;
                }
            }
            if self.sapglob.sap(sap_id2).colsag != 0 {
                /*fuite:1 fuite reelle 2 éloigne du bord*/
                let s2 = self.sapglob.sap_mut(sap_id2);
                if s2.fuite == 1 {
                    s2.sens = sens;
                }
                s2.colsag -= 1;
            } else if self.sapglob.distsp > 4 && sens == self.sapglob.sap(sap_id2).sens {
                let s2 = self.sapglob.sap_mut(sap_id2);
                /*pas trop pres et tourne le dos*/
                s2.sens ^= 1; /*le sens s'inverse*/
            }
            if self.random.Rnd() < self.sapglob.sap(sap_id2).agressivite as i16 {
                if self.random.Rnd() > 100 {
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    s2.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlou1));
                } else {
                    let s2 = self.sapglob.sap_mut(sap_id2);
                    s2.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlou2));
                }
            }
            let s2 = self.sapglob.sap_mut(sap_id2);
            s2.souffle = 128;
            s2.coeur = if s2.coeur < 0 {
                -(DEBCRANE!(self.sapwind) + 8)
            } else {
                DEBCRANE!(self.sapwind) + 8
            };
        } else {
            /*P'TIT Lapin*/
            let fuite2 = {
                let s2 = self.sapglob.sap_mut(sap_id2);
                s2.souffle = 1; /*ca bat vite un p'tit coeur*/
                s2.coeur = if s2.coeur < 0 {
                    -(DEBCRANE!(self.sapwind) + 8)
                } else {
                    DEBCRANE!(self.sapwind) + 8
                };
                s2.fuite
            };

            if fuite2 == 0 && self.random.Rnd() < 60
            /* rien c'est con un lapin*/
            {
                /* plus l'adversaire est proche plus il s'agite*/
                if self.random.Rnd() / 8 > self.sapglob.distsp {
                    self.sapglob.sap_mut(sap_id2).sapanim[0] =
                        Some(AnimPtr::new(AnimTable::Anlap3)); /*lapin observe*/
                    let (vitesse, snssag, action, sens) = {
                        let s1 = self.sapglob.sap_mut(sap_id1);
                        (s1.vitesse, s1.snssag, s1.action, s1.sens)
                    };
                    if (self.sapglob.distsp < 10 && self.random.Rnd() > 60)
                        || (vitesse & 2) == 0
                        || snssag != 255
                        || action != 0
                    {
                        /*si proche et bouge ou si sagaie en l'air*/
                        let s2 = self.sapglob.sap_mut(sap_id2);
                        s2.sapanim[1] = Some(AnimPtr::new(AnimTable::Anlap2)); /*fuite*/
                        s2.sens = sens;
                        s2.fuite = 1;
                    }
                } else {
                    let agressivite = self.sapglob.sap(sap_id2).agressivite;
                    if self.random.Rnd() < agressivite as i16 {
                        /*ici la peur*/
                        let distsp = self.sapglob.distsp;
                        let s2 = self.sapglob.sap_mut(sap_id2);
                        s2.sapanim[0] = Some(AnimPtr::new(AnimTable::Anlap2));
                        if self.random.Rnd() < 25 && distsp < 45 {
                            /*Reglage sortie Lapin*/
                            s2.sens = (self.random.Rnd() & 1) as u8; /*sens au hasard*/
                        }
                    }
                }
            }
            /*c'est con il fait rien*/
            let s2 = self.sapglob.sap_mut(sap_id2);
            if s2.colonne < LIMITE_GAUCHE + 3 || s2.colonne > LIMITE_DROITE - 3 {
                /*il sort de l'ecran : on le degenere*/
                s2.sapanim[0] = None;
                s2.colonne = 200;
                s2.vie = 0; /* a pu lapin*/
                self.sapglob.present[self.sapglob.who as usize] = 0;
                self.generique(VRAI);
                if self.sapglob.presence != self.sapglob.juge() {
                    self.sapglob.fin = (self.sapglob.juge() + 1) as u8;
                }
            }
        }
    }

    /***********************************************************************
                    MOBS:Deplace les Sapiens
    ***********************************************************************/
    fn mobs(&mut self, TesteClavier: Boolean) {
        /* Pour les 4 sapiens */
        for surqui in SapId::iter().rev() {
            self.sapglob.who = surqui as i8;
            let (surqui_flg_mort, surqui_vie) = {
                let surqui_sap = self.sapglob.peuple(surqui);
                (surqui_sap.flg_mort, surqui_sap.vie)
            };
            /* teste si animation mort */
            if surqui_flg_mort != 0 {
                self.ger_mort(surqui);
                while self.mvtsap(surqui) != 0 {}
            } else if surqui_vie != 0
            /*  si defini alors*/
            {
                {
                    let surqui_sap = self.sapglob.peuple_mut(surqui);
                    incsou(surqui_sap, 1); /*il se repose*/
                }
                while self.mvtsap(surqui) != 0 {} /*il s'anime*/
                if surqui == SapId::Sap1 {
                    /*Le sap1 agit*/
                    let tfragil = self.sapglob.cptagil.wrapping_add(self.sapglob.agilite);
                    if tfragil < self.sapglob.cptagil && TesteClavier {
                        let touche = self.getkey(0);
                        self.clav(surqui, touche);
                    }
                    self.sapglob.cptagil = tfragil;
                } else if (self.sapglob.present[surqui as usize] as i16) < 5 {
                    /* Gestion du copain:Il y a-t-il une gestion de sap alors qu'il n'y a plus d'adversaire?*/
                    /*  homme*/
                    if self.sapglob.sap3clav != 0 && surqui as usize == 2 {
                        /*au clavier*/
                        if TesteClavier {
                            let keyboard_input = self.getkey(1);
                            self.clav(surqui, keyboard_input);
                        }
                    } else if self.sapglob.presence == 0 {
                        self.gercop(
                            surqui,
                            ((self.sapglob.present[surqui as usize] as i16) < 5) as i16,
                        );
                    } else {
                        self.autonhom(surqui, self.sapglob.peuple(surqui).adversaire);
                    }
                } else {
                    /*bestiole*/
                    if self.sapglob.presence == 0 {
                        self.gercop(
                            surqui,
                            ((self.sapglob.present[surqui as usize] as i16) < 5) as i16,
                        );
                    } else {
                        self.bestio(surqui, self.sapglob.peuple(surqui).adversaire);
                        /* si c'est un lapin alors il agit aussi sur
                        mon ami*/
                        if (self.sapglob.present[surqui as usize] as i16) > 6
                            && self.sapglob.present[2] != 0
                        {
                            self.bestio(surqui, SapId::Sap3);
                        }
                    }
                }
                self.gcoeur(surqui); /*affichage du coeur*/
            }
            self.gersag(surqui);
        }

        /*tout les impacts devant*/
        for who_c in SapId::iter() {
            self.sapglob.who = who_c as i8;
            self.impak(who_c);
        }
        /*Affiche les Herbes au 1er plan*/
        for i in 0..self.sapcomb.NbHerbeAvPlan {
            if self.sapcomb.herbe[i as usize].type_ != 0 {
                self.Affmob(
                    self.sapcomb.herbe[i as usize].type_ as i16,
                    self.sapcomb.herbe[i as usize].lin as i16,
                    self.sapcomb.herbe[i as usize].col as i16,
                    self.sapcomb.herbe[i as usize].sens as i16,
                );
            }
        }
    }

    /***********************************************************************
                            CalculeMobHerbe
    Calcule un numero de mob pour l'herbe
    ***********************************************************************/
    fn CalculeMobHerbe(&mut self) -> i16 {
        if self.random.myRandom(0, 1000) > 950 {
            (self.sapwind.mobAvplanSpec as i64
                + 1
                + self.random.myRandom(0, self.sapcomb.nbAvplanSpec as i64)) as i16
        } else {
            (MOB_HERBE!(self.sapwind) as i64
                + self.random.myRandom(0, self.sapcomb.nbMobHerbes as i64)) as i16
        }
    }
}

/***********************************************************************
                        AVPLAN:Calcule avant-plan
***********************************************************************/
const LIGNE_CHEMIN: i16 = 68;
const GRANULOSITE: i16 = 10;

impl Sapiens {
    fn avplan(&mut self) {
        let mut num: i16;
        let mut nb: i64;
        /* Premier passage */
        if self.sapcomb.TabIndexHerbeFond.is_none() {
            let _g = self.sapwind.ports.on(PortId::Ecran1);
            self.sapcomb.TabIndexHerbeFond =
                Some(self.CalculePaletteAttenuee(MOB_HERBE!(self.sapwind), 0.75, 999));
        }
        /*on fixe le Rnd*/
        self.random.initrnd(
            (self.sapglob.xj / 256) as i16,
            (self.sapglob.yj / 256) as i16,
            ((self.sapglob.xj & 224) | ((self.sapglob.yj & 224) / 8)) as i16,
        );
        if self.sapcomb.nbHerbes <= 20 {
            self.sapcomb.NbHerbeAvPlan = self.random.myRandom(0, self.sapcomb.nbHerbes) as i16;
        } else {
            self.sapcomb.NbHerbeAvPlan = (self
                .random
                .myRandom(0, self.sapcomb.nbHerbes / GRANULOSITE as i64)
                * GRANULOSITE as i64) as i16;
        }
        /*on affiche le chemin*/
        for i in 0..8 {
            let mob = MOB_CHEMIN!(self.sapwind) + (self.random.rnd0() & 3);
            self.Affmob(mob, LIGNE_CHEMIN, i * 8 * 4, 0);
        }

        /*on affiche les arbres*/
        nb = self
            .random
            .myRandom(NB_ARBRES_LATERAL as i64, NB_ARBRES_LATERAL_MAX as i64);
        for i in 0..nb {
            //arbre(78,1+i*63+rnd0()/4);
            let y = 68.0 + 10.0 * (i as f32) / (nb as f32); // de 68 a 78
            let colonne = self.random.myRandom(0, 250) as i16;
            self.arbre(y as i16, colonne);
        }
        // herbes arrieres plan
        nb = self.random.myRandom(
            (NB_HERBE_ARR_PLAN as i16) as i64 / GRANULOSITE as i64,
            (NB_MAX_HERBE_AR_PLAN as i16) as i64 / GRANULOSITE as i64,
        ) * GRANULOSITE as i64;
        /*affichage des herbes a l'arriere plan */
        for _ in 0..nb {
            num = self.CalculeMobHerbe();
            let r = self.CalculeAireMob(num, 0, 0);
            //AffmobPalette(num,LIGNE_CHEMIN+12-(r.bottom/2)+(i/3),myRandom(0,256),rnd0()&1,TabIndexHerbeFond);
            let y = (LIGNE_CHEMIN as i64 + 12 - (r.bottom / 2) as i64 + self.random.myRandom(0, 3))
                as i16;
            let x = self.random.myRandom(0, 256) as i16;
            let inverse = self.random.rnd0() & 1;
            let _g = self.sapwind.ports.on(PortId::Ecran1);
            self.sapwbrt.Affmobbrt(
                self.sapwind.ports.current_mut(),
                num,
                x,
                y,
                inverse,
                Some(
                    self.sapcomb
                        .TabIndexHerbeFond
                        .as_deref()
                        .expect("TabIndexHerbeFond must be Something when calling avplan"),
                ),
            );
        }
        /*on remplit le tableau des herbes avplan*/
        for i in 0..self.sapcomb.NbHerbeAvPlan {
            num = self.CalculeMobHerbe();
            self.sapcomb.herbe[i as usize].type_ = num as u16;
            let r = self.CalculeAireMob(num, 0, 0);
            self.sapcomb.herbe[i as usize].lin = (LIGNE_MAX_HERBE_AV_PLAN as i32
                - (self.sapcomb.NbHerbeAvPlan as i32 - i as i32)
                    * (LIGNE_MAX_HERBE_AV_PLAN as i32 - LIGNE_BASE_HERBE_AV_PLAN as i32)
                    / self.sapcomb.NbHerbeAvPlan as i32
                - (r.bottom / 2) as i32
                + 32) as u16;
            /*166*/
            // herbe[i].col=myRandom(0,220);
            self.sapcomb.herbe[i as usize].col = self.random.myRandom(0, 240) as u16;
            self.sapcomb.herbe[i as usize].sens = (self.random.rnd0() & 1) as u16;
            if self.sapcomb.herbe[i as usize].type_ != 0 {
                self.Affmob(
                    self.sapcomb.herbe[i as usize].type_ as i16,
                    self.sapcomb.herbe[i as usize].lin as i16, /*98*/
                    self.sapcomb.herbe[i as usize].col as i16,
                    self.sapcomb.herbe[i as usize].sens as i16,
                );
            }
        }
        /* L'avplan est recopie sur l'ecran 2*/
        self.CopyEcran1Sur2();
    }

    /***********************************************************************
                            LATERB
    ***********************************************************************/
    pub(crate) fn laterb(&mut self, TesteClavier: Boolean) {
        let t = self.sapwclav.sapiens_timer.load(Ordering::Relaxed);

        self.effdemi();
        self.mobs(TesteClavier);
        self.sapwind.ports.tfrdemi();
        self.GereInterface();
        while t > self
            .sapwclav
            .sapiens_timer
            .load(Ordering::Relaxed)
            .wrapping_sub(6)
        {}
    }

    /***********************************************************************
                            LATERA
    ***********************************************************************/
    pub(crate) fn latera(&mut self) {
        /*affiche noms*/
        self.sapglob.EffaceGenerique();
        self.generique(VRAI);

        self.sapglob.dismin = 50;
        self.affich(((self.sapglob.dzeta as i32 + 256) & 0x3ff) as u16);
        /*  Mise en place 3d fn angle
        .... avplan */
        self.avplan();
        self.sapwind.ports.tfrima();
        /* (c) God 7 Aout 87 Inits Souris */
        (
            self.sapglob.bouton,
            self.sapglob.dsposx,
            self.sapglob.dsposy,
        ) = self.vq_mouse();
        self.sapwind.reset_clav();
        self.laterb(FAUX);
        self.tfrvirt();
    }

    /***********************************************************************
                            INARME:Init des armes
    ***********************************************************************/
    pub(crate) fn inarme(&mut self) {
        let sap1 = &mut self.sapglob.sap1;
        self.sapglob.entvie = sap1.vie;
        sap1.forcehache = 0;
        sap1.cross[FormeVue::ARME as usize] = 0; /*hache par defaut*/
        sap1.nbresag = 0;

        /* Initialise l'agilite active*/

        self.sapglob.cptagil = 1;
        let mut i = self.sapglob.liste1.taille;
        while i != 0 {
            i -= 1;
            if sap1.forcehache == 0 {
                if self.sapglob.liste1.elements[i as usize] == 22
                /*massue*/
                {
                    sap1.forcehache = (10 + (self.sapglob.force as u16 * 10) / 256) as u8;
                    sap1.cross[FormeVue::ARME as usize] = 1;
                }
                if self.sapglob.liste1.elements[i as usize] == 21
                /*hache*/
                {
                    sap1.forcehache = (6 + (self.sapglob.force as u16 * 6) / 256) as u8;
                }
            }
            if self.sapglob.liste1.elements[i as usize] == 23 {
                sap1.nbresag += 1; /*sagaie*/
            }
        }
    }

    /***********************************************************************
                            COMBAT:Gere les combats
                    On traite la sortie de FIN sur LATERB.
    ***********************************************************************/
    pub(crate) fn combat(&mut self) {
        let mut tfr: u8;
        let mut quelqun: Boolean = false;

        /* Inits Souris */
        self.reset_mouse();
        if self.sapglob.dervue != Vue::LATERALE
        /* egalement si presence humaine ??? ou aberration:Collapse!!*/
        {
            /* Maquillage et habillage des 'Bons'*/
            self.prm(SapId::Sap1, -1);
            self.prm(SapId::Sap3, self.sapglob.present[2] as i16);
            /* les machinistes montent le decor*/
            self.latera();
        }
        (
            self.sapglob.bouton,
            self.sapglob.dsposx,
            self.sapglob.dsposy,
        ) = self.vq_mouse();
        self.sapglob.dervue = Vue::LATERALE;
        /* Sapiens une premiere ,Clap !*/
        self.clrbadi();

        loop {
            self.sapglob.sap3clav =
                if self.sapcomb.ModeJoueur2 != 0 && (self.sapglob.present[2] as i16) < 5 {
                    1
                } else {
                    0
                };
            self.inarme(); /* initialise les armes sap1,silence sur le plateau*/
            self.sapglob.presence = self.sapglob.juge(); /* on initialise les adversaires reciproques*/
            /* Si le joueur est seul alors affichage du commentaire */
            let sap2 = &self.sapglob.sap2;
            let sap4 = &self.sapglob.sap4;
            if sap2.vie == 0 && sap4.vie == 0 {
                if quelqun {
                    self.sapglob.presence = 0;
                    self.inimen(0);
                    self.laterb(VRAI);
                    /* On ouvre par defaut le menu'prends'*/
                    self.InverseOption(0, 2);
                    self.sapglob.s_action = 3;
                    self.inimen(4);
                    return;
                }
                //comment(132);
            } else {
                quelqun = true;
            }
            self.sapglob.fin = 0; /* Moteurs !!!!*/
            // ScrollComb(sens);
            while self.sapglob.fin == 0 {
                self.laterb(VRAI); /*  ACTION !!!*/
            }
            /* c'est parfait coupez */

            self.sapglob.tfrene(); /*on transfere l'enervement*/
            self.outarm(); /*armes en sortie*/
            self.sapglob.flcomb = 0;
            self.sapglob.flefba = 0;
            self.sapglob.fdialo = 0;
            if self.sapglob.presence == -1 {
                self.sapglob.present[2] = 0;
            }
            if self.sapglob.fin == 2
            /*clan 1 perdu*/
            {
                self.CopyEcran1Sur2();
                self.sapglob.dervue = Vue::AUCUNE;
                /* combat ultime */
                if self.sapglob.scenar == 5 {
                    self.sapglob.mort = 0;
                } else {
                    tfr = self.sapglob.bless.wrapping_sub(100);
                    if tfr > self.sapglob.bless {
                        self.sapglob.mort -= 1;
                        tfr = 20;
                    }
                    self.sapglob.bless = tfr;
                }
                self.afstat();
                /* on a perdu */
                if self.sapglob.mort == 0 {
                    self.sapglob.scenar = 7; // Perdu Mort
                    self.sceveu(7);
                    self.sapglob.presence = 0;
                    self.inimen(0);
                    return;
                }
                self.comment(81);
                /*c'est un homme qui a gagne*/
                if (self.sapglob.present[1] as i16) < 5 && self.sapglob.present[1] != 0 {
                    self.vole();
                }
                self.prm(SapId::Sap2, 0);
                self.prm(SapId::Sap4, 0);
                self.sapglob.presence = 0;
                attends_bruit();
                self.inimen(0);
                return;
            }
            if self.sapglob.fin == 1
            /*clan 2 perdu*/
            {
                self.sapglob.presence = 0;
                self.prm(SapId::Sap2, 0);
                self.prm(SapId::Sap4, 0);
                /* On a eu le chef des Hyene folle*/
                if self.sapglob.scenar == 4 && self.sapglob.chef != 0 && self.sapglob.tribu == 4 {
                    self.sapglob.scenpr = 0;
                    self.sceveu(9);
                }
                /* Scenario ultime */
                if self.sapglob.scenar == 5 {
                    self.sapglob.mort = 0; /*thats all folks*/
                    self.sapglob.scenar = 6;
                    self.sapglob.jour_dsk += 1;
                    self.sceveu(6); /*bravo tu as gagne*/
                    self.sapglob.presence = 0;
                    self.inimen(0);
                    return;
                }
            }
            /*si esc */
            if (self.sapglob.fin == 129 && self.sapglob.presence == 0) || self.sapglob.mort == 0 {
                let sap1 = &self.sapglob.sap1;
                if sap1.vie == 0 {
                    // le C ecrivait 255 ici, valeur hors enum egalee a AUCUNE
                    // par tous les tests (==/!= VUE_LATERALE, VUE_PAYSAGE)
                    self.sapglob.dervue = Vue::AUCUNE;
                }
                self.sapglob.presence = 0;
                self.inimen(0);
                return;
            }
            /*Si sortie de l'ecran (nous sommes donc seuls)*/
            if self.sapglob.fin == 128 {
                self.clrbadi();
                self.deplat();
                self.zoninf();
                if self.sapglob.presence != 0 && self.sapglob.presence < 5 {
                    self.prm(SapId::Sap1, self.sapglob.present[0] as i16);
                    self.prm(SapId::Sap3, self.sapglob.present[2] as i16);
                    self.latera();
                    self.sapglob.dervue = Vue::LATERALE;
                    self.inimen(0);
                    return;
                }
                self.fixcolon(SapId::Sap1);
                self.fixcolon(SapId::Sap3);
                self.prm(SapId::Sap3, self.sapglob.present[2] as i16);
                self.latera();
            }
        } /*infini .... et un peu plus*/
    }
}

/// Équivalent de `sprintf(format, nom)` : remplace le premier `%s` par `nom`
/// (tableau d'octets terminé par un NUL).
fn fmt_nom(format: &str, nom: &[u8]) -> String {
    let end = nom.iter().position(|&b| b == 0).unwrap_or(nom.len());
    format.replacen("%s", &String::from_utf8_lossy(&nom[..end]), 1)
}
