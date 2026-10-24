#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
/***********************************************************************
                SAPREL : Module relationnel (SAPREL.C porté en Rust)
                    (c) god Le 7 Novembre 1986
***********************************************************************/
use std::os::raw::c_void;
use std::ptr::{addr_of, addr_of_mut};
use std::rc::Rc;
use std::sync::atomic::Ordering;

use crate::Sapiens;
use crate::acam::{
    BackColor, BlockMove, CloseRgn, DisposeRgn, EraseRgn, FAUX, FrameRgn, FrameRoundRect, GetClip,
    LineTo, NewRgn, OpenRgn, SetClip, TickCount, VRAI,
};
use crate::cbridge::{strcat, strcpy, whiteColor};
use crate::port;
use crate::sapiens::sapglob::{ListeId, SapId, Sapglob, tabprf, tenerv, tobje};
use crate::sapiens::sapw3d::distance;
use crate::sapiens::sapwbrt::{CopyOffscreen, DoMoveTo, DoPenSize};
use crate::sapiens::sapwmus::attends_bruit;
use crate::sapiens::{NBREVTDESTINE, NBREVTHEURE, Vue, evt};
use crate::shared::{Ptr, Rect, RgnHandle};

const ECHELLE: i16 = 2;

pub(crate) struct Saprel {
    LastObsv: i16,
    rBulleJoueur: Rect,
    rBulleAdversaire: Rect,
}

impl Default for Saprel {
    fn default() -> Self {
        Self {
            LastObsv: 0,
            rBulleJoueur: Rect {
                top: 0,
                left: 0,
                bottom: 0,
                right: 0,
            },
            rBulleAdversaire: Rect {
                top: 0,
                left: 0,
                bottom: 0,
                right: 0,
            },
        }
    }
}

const X_BULLE_DROITE: i16 = 640 - 64 - 16;
const X_BULLE_GAUCHE: i16 = 80;
const Y_BULLE: i16 = 58;
const TAILLE_QUEUE: i16 = 20;

impl Sapiens {
    /***********************************************************************
                            CLRBADI:efface bande si inutilisée
    ***********************************************************************/
    pub(crate) fn clrbadi(&mut self) {
        if self.sapglob.flagbadi > 0 {
            self.efbadi();
            self.sapglob.flagbadi = 0;
        }
    }

    /***********************************************************************
                    AFSTAT:affiche menu status
    ***********************************************************************/
    pub(crate) fn afstat(&mut self) {
        let refer3 = self.sap_lang.GetRefer3();
        unsafe {
            (*refer3.wrapping_add(7)).nomref[0] = ((self.sapglob.faim / 64) + 0x91) as i8;
            (*refer3.wrapping_add(8)).nomref[0] = ((self.sapglob.soif / 64) + 0x91) as i8;
            (*refer3.wrapping_add(9)).nomref[1] = ((self.sapglob.fatig / 64) + 0x95) as i8;
            (*refer3.wrapping_add(10)).nomref[1] = ((self.sapglob.bless / 64) + 0x9A) as i8;
            (*refer3.wrapping_add(11)).nomref[1] = (self.sapglob.mort + 0xA7) as i8;
            self.inimen(7);
            self.sapglob.s_action = 2;
        }
    }

    /*************************************************************************
                        TailleChaine
    Affiche un caractere sur le port en entrée
    => Chaine a traiter
    @= taille maximale en colonne x, nbre de lignes t
    **************************************************************************/
    fn TailleChaine(&self, mut debut: *mut i8) -> (i16, i16) {
        let mut x = 0;
        let mut taillex = 0;
        let mut y = 0;

        let mut ty = 0;
        unsafe {
            while *debut != 0 {
                if *debut == b'\r' as i8 {
                    ty += 1;
                    x = 0;
                    debut = debut.add(1);
                } else {
                    (x, y) = self.TailleCarMess(&mut debut, x, y);
                    if x > taillex {
                        taillex = x;
                    }
                    ty += y;
                }
            }
        }
        (taillex, ty)
    }

    fn TailleCarMess(&self, debut: *mut *mut i8, mut taillex: i16, mut tailley: i16) -> (i16, i16) {
        unsafe {
            let p = *debut;
            let car = *p as u8 as i16;
            *debut = p.add(1);
            if car > 128 {
                let chaine = self.GetStrBkMot(car);
                let (tx, ty) = self.TailleChaine(chaine);
                taillex += tx;
                tailley += ty;
            } else {
                taillex += self.TailleCar(car, 10);
            }
        }
        (taillex, tailley)
    }

    /***********************************************************************
                            EFBADI
    ***********************************************************************/
    pub(crate) fn efbadi(&mut self) {
        self.Gotoxy(3, 15);
        self.efbadass();
    }

    /***********************************************************************
                            scrollBADI
    ***********************************************************************/
    pub(crate) fn scrollbadi(&mut self) {
        self.Gotoxy(3, 15);
        self.scrollbadass();
    }

    /***********************************************************************
                            SCEVEU:
    Affiche le message correspondant à la phase du scenario
    Le message s'affiche sur l'aire 3d
    ***********************************************************************/
    pub(crate) fn sceveu(&mut self, quoi: u16) {
        unsafe {
            let scenme = self.sap_lang.GetScenme();

            self.sapglob.dervue = Vue::AUCUNE;
            if quoi == 6 {
                self.gemouvre(6, 32, 1, 2, 1, 8, 8, 38);
            } else {
                self.gemouvre(6, 32, 1, 2, 1, 8, 5, 38);
            }
            /* on efface la zone */
            self.effvirt(0x007);

            self.Gotoxy(12, 1);
            let mut p = *scenme.add((quoi - 1) as usize);
            let mut car = *p as u8;
            p = p.add(1);
            /* On affiche sur debecr */
            while car != 0 {
                self.car2(car as i16);
                loop {
                    car = *p as u8;
                    p = p.add(1);
                    if car == 13 {
                        self.Gotoxy(12, self.sapglob.linaff + 1);
                    }
                    if car != 13 {
                        break;
                    }
                }
            }
            if quoi == 6 {
                for _i in 0..3 {
                    self.bruit(6);
                    attends_bruit();
                }
            }
            /* Transfert debecr vers reel */
            self.TfrEcr(None);
            if quoi == 6 {
                /* Bravo tu as gagné */
                let rdest = Rect::new(0, 0, (255 * ECHELLE) as i16, (100 * ECHELLE) as i16);
                let rsourc = Rect::new(
                    (32 * ECHELLE) as i16,
                    ECHELLE as i16,
                    ((32 + 255) * ECHELLE) as i16,
                    ((1 + 100) * ECHELLE) as i16,
                );

                CopyOffscreen(self.sapwind.debecr, self.sapwind.Ecran1, &rsourc, &rdest);
                self.delai(15 * 60);
                self.Final();
            }
        }
    }

    /***********************************************************************
                            SceveuBulle:
    Affiche le message correspondant à la phase du scenario
    Le message s'affiche sur l'aire 3d
    ***********************************************************************/
    pub(crate) fn SceveuBulle(&mut self, quoi: u16) {
        let scenme = self.sap_lang.GetScenme();
        self.effaceBulles();
        unsafe {
            self.bulle(*scenme.add(quoi as usize) as Ptr, 1);
        }
    }

    /***********************************************************************
                            GAINSBOUR:Dégradation Physique
    ***********************************************************************/
    pub(crate) fn gainsbour(&mut self) {
        self.sapglob.bless = self
            .sapglob
            .bless
            .wrapping_sub((6 - 6 * self.sapglob.resistance as i16 / 256) as u8);
        if self.sapglob.bless > 251 {
            if self.sapglob.mort != 0 {
                /*pas mort encore*/
                self.sapglob.mort -= 1;
                self.sapglob.bless = 20;
            } else {
                self.sapglob.presence = 0;
                self.sapglob.fdialo = 0;
            }
        }
    }

    /***********************************************************************
                            PASTMP:Le temps Passe
    ***********************************************************************/
    pub(crate) fn pastmp(&mut self) {
        let mut i = self.sapglob.liste1.taille;
        while i != 0 {
            i -= 1;
            /*la viande se corromps*/
            if self.sapglob.liste1.elements[i as usize] >= 37
                && self.sapglob.liste1.elements[i as usize] < 41
            {
                self.sapglob.liste1.elements[i as usize] += 1;
            }
        }
    }
}

impl Sapglob {
    /***********************************************************************
                            TFRENE:
    ***********************************************************************/
    pub(crate) fn tfrene(&mut self) {
        if self.presence < 5 && self.presence != 0 && self.fpress != 0 {
            self.enetoi[self.tribu as usize - 1] = self.enerv;
            if self.chef != 0 || self.tribu != 1 {
                self.tagress[self.tribu as usize - 1] = self.agress;
            }
        }
    }
}

impl Sapiens {
    /**************************************************************************
                    COMMENT:affiche un commentaire
    *************************************************************************
    => rang du commentaire
    */
    pub(crate) fn comment(&mut self, rang: u16) {
        let mut p: *mut u8;

        if self.sapglob.flefba == 0 {
            self.sapglob.flefba += 1;
            self.scrollbadi();
        }
        self.sapglob.flagbadi = 1;
        let mescon = self.sap_lang.GetMesCon();
        p = mescon[(rang - 1) as usize] as *mut u8;
        unsafe {
            while *p != 0 {
                self.affcarmess(*p as i16);
                p = p.add(1);
            }
        }
        self.TfrBadAss();
    }

    /**************************************************************************
                    EffaceBulles
    *************************************************************************/
    pub(crate) fn effaceBulles(&mut self) {
        self.tfrvirt();
        if !self.saprel.rBulleJoueur.is_empty() {
            CopyOffscreen(
                self.sapwind.debecr,
                self.sapwind.mainWindow,
                &self.saprel.rBulleJoueur,
                &self.saprel.rBulleJoueur,
            );
        }
        self.saprel.rBulleJoueur = Rect::default();
        if !self.saprel.rBulleAdversaire.is_empty() {
            CopyOffscreen(
                self.sapwind.debecr,
                self.sapwind.mainWindow,
                &self.saprel.rBulleAdversaire,
                &self.saprel.rBulleAdversaire,
            );
        }
        self.saprel.rBulleAdversaire = Rect::default();
    }

    /**************************************************************************
                    Bulle:affiche une bulle avec un texte
    => message a afficher, qui le dit (0)== joueur (1)== adversaire
    *************************************************************************/
    pub(crate) fn bulle(&mut self, debut: Ptr, qui: i16) {
        unsafe {
            let mut rbase: Rect;
            let rgnclip: RgnHandle;
            let rgn: RgnHandle;
            let mut x: i16;
            let mut y: i16;
            let mut p: Ptr = debut;

            let _g = port::on(self.sapwind.debecr);
            /* Calcul de la taille de la chaine en pixel */
            let (mut tx, ty) = self.TailleChaine(p as *mut i8);
            if tx < 64 {
                tx = 64;
            }
            let sap1 = &self.sapglob.sap1;
            let mut r = if (qui ^ sap1.sens as i16) != 0 {
                Rect::new(
                    X_BULLE_GAUCHE,
                    Y_BULLE - (ty + 2) * 12,
                    X_BULLE_GAUCHE + tx + 16,
                    Y_BULLE,
                )
            } else {
                Rect::new(
                    X_BULLE_DROITE - tx - 16,
                    Y_BULLE - (ty + 2) * 12,
                    X_BULLE_DROITE,
                    Y_BULLE,
                )
            };
            r.bottom += TAILLE_QUEUE;
            rbase = r;
            /* On efface les bulles en cours */
            if qui == 0 {
                /* C'est le joueur */
                self.effaceBulles();
                self.saprel.rBulleJoueur = r;
            } else {
                /* C'est un adversaire ... */
                /* deja un message dit ??? */
                if !self.saprel.rBulleAdversaire.is_empty() {
                    r.OffsetRect(
                        0,
                        self.saprel.rBulleAdversaire.bottom
                            - self.saprel.rBulleAdversaire.top
                            - TAILLE_QUEUE
                            - 4,
                    );
                    self.saprel.rBulleAdversaire = self.saprel.rBulleAdversaire.union(&r);
                    rbase = self.saprel.rBulleAdversaire;
                } else {
                    self.saprel.rBulleAdversaire = r;
                }
            }
            r.bottom -= TAILLE_QUEUE;
            rgn = NewRgn();
            rgnclip = NewRgn();
            OpenRgn();
            FrameRoundRect(&r, 16, 16);
            /* Bulle a gauche */
            let sap1 = &self.sapglob.sap1;
            if (qui ^ sap1.sens as i16) != 0 {
                x = r.left + 48;
                y = r.bottom;
                DoMoveTo(x, y);
                LineTo(x - TAILLE_QUEUE, y + TAILLE_QUEUE);
                LineTo(x - 8, y);
                LineTo(x, y);
            } else {
                x = r.right - 48;
                y = r.bottom;
                DoMoveTo(x, y);
                LineTo(x + TAILLE_QUEUE, y + TAILLE_QUEUE);
                LineTo(x + 8, y);
                LineTo(x, y);
            }
            CloseRgn(rgn);
            BackColor(whiteColor as i16);
            EraseRgn(rgn);
            DoPenSize(1, 1);
            FrameRgn(rgn);
            GetClip(rgnclip);
            SetClip(rgn);
            x = r.left + 8;
            y = r.top + 6;
            self.sapwcar.GotoxyPixel(x, y);
            while *p == b' ' {
                p = p.add(1);
            }
            while *p != 0 {
                if *p == b'\r' {
                    y += 14;
                    self.sapwcar.GotoxyPixel(x, y);
                    loop {
                        p = p.add(1);
                        if *p != b' ' {
                            break;
                        }
                    }
                } else {
                    self.affcar10(*p as i16);
                    p = p.add(1);
                }
            }
            SetClip(rgnclip);
            DisposeRgn(rgn);
            DisposeRgn(rgnclip);
            self.TfrEcr(Some(&rbase));
            DoPenSize(1, 1);
            return;
        }
    }

    /***********************************************************************
                            CHRONO:Effectue Chronologie
    Heure varie de [0 à 1023]
    Matin :              0-90
    Apres midi :         426-512
    Nuit :               512-602
    Fin de nuit          938-1023

    ***********************************************************************/
    /*duree du jour 4*90=360 heures*/
    /*quoi 0:normal,1:ni param en baisse ni message,2 baisse mais pas mess*/
    pub(crate) fn chrono(&mut self, quoi: i16) {
        let mut avemess: u8;

        if self.sapglob.minute != 0 {
            self.sapglob.minute -= 1;
            return;
        }
        self.sapglob.minute = 5;
        self.sapglob.heure += 1;
        if (self.sapglob.heure & 255) == 90 {
            self.sapglob.heure += 336;
        }
        self.sapglob.heure &= 0x03ff;
        if (self.sapglob.heure & 0xFF) == 0 {
            self.pastmp();
        }
        /*mise en place des evts couleurs*/
        while self.sapglob.heure >= self.sapglob.tabheur[self.sapglob.pos_evt as usize].h as u16 {
            /* dois t'on modifier le degrad couleur */
            if self.sapglob.tabheur[self.sapglob.pos_evt as usize].q != 0 {
                /* Changement du degrade du ciel avec les huits couleurs ST en entrée*/
                self.sapglob.ciel_degr =
                    Rc::clone(&self.sapglob.tabheur[self.sapglob.pos_evt as usize].pal);
                self.sapw3d.FixeDegradesFonctionHeure(
                    self.sapglob.tabheur[self.sapglob.pos_evt as usize].clone(),
                );
            } else {
                /* On fixe la pal haute */
                self.sapglob.palhaute =
                    Rc::clone(&self.sapglob.tabheur[self.sapglob.pos_evt as usize].pal);
            }
            self.sapglob.pos_evt += 1;
            if self.sapglob.pos_evt >= NBREVTHEURE as i16 {
                self.sapglob.pos_evt = 0;
                self.sapglob.heure = 0;
            }
        }
        if self.sapglob.heure == 0 {
            self.sapglob.flefba = 0;
            self.comment(114); /* Un nouveau jour ...*/
            self.sapglob.jour_act += 1;
        }
        /* Position du meteore */
        if self.sapglob.heure & 0x100 != 0 {
            self.sapglob.metpos = (self.sapglob.heure & 0xFF).wrapping_sub(180);
            self.sapglob.angsol = 256;
        } else {
            self.sapglob.metpos = 75u16.wrapping_sub(self.sapglob.heure & 0xFF);
            self.sapglob.angsol = 768;
        }
        if quoi == 1 {
            return;
        }
        self.sapglob.finjou = 1; /*deperissement de la viande*/
        self.sapglob.faim = self.sapglob.faim.wrapping_sub(2);
        avemess = 0;
        if self.sapglob.faim > 253 {
            self.sapglob.faim = 5;
            self.gainsbour();
            avemess = 93;
        }
        self.sapglob.soif = self.sapglob.soif.wrapping_sub(4);
        if self.sapglob.soif > 251 {
            self.sapglob.soif = 5;
            self.gainsbour();
            avemess = 94;
        }
        self.sapglob.fatig = self.sapglob.fatig.wrapping_sub(2);
        if self.sapglob.fatig > 253 {
            self.sapglob.fatig = 5;
            self.gainsbour();
            avemess = 95;
        }
        if avemess != 0 && quoi == 0 {
            self.efbadi();
            self.comment(avemess as u16);
        }
    }

    /**************************************************************************
                    CAUSE: Sort à l'ecran une phrase correspondant
                                 à un contexte donné
    *************************************************************************
    => Contexte de 1 à ....
    */
    unsafe fn cause(&mut self, context: u16, mess_add: *const i8) {
        let mut chaine = [0; 255];
        unsafe {
            /*y cause ti ?*/
            if self.sapglob.fdialo != 0 {
                /*y cause */
                let rang = ((context - 1) * 4 + (self.random.Rnd() & 3) as u16) as i16;
                let mescon = self.sap_lang.GetMesCon();
                strcpy(chaine.as_mut_ptr(), mescon[rang as usize]);
                if !mess_add.is_null() {
                    strcat(chaine.as_mut_ptr(), mess_add);
                }
                self.bulle(chaine.as_mut_ptr() as Ptr, 1); /*affiche le commentaire*/
            }
        }
    }

    /***********************************************************************
                            FUITE:
    ***********************************************************************/
    pub(crate) fn fuite(&mut self) {
        self.sapglob.flefba = 0;
        if self.sapglob.agress < 245 {
            self.sapglob.agress = self.sapglob.agress.wrapping_add(10); /*l'agressivite augmente*/
        }
        self.comment(80); /*il s'enfuit*/
        self.quitte();
    }

    /***********************************************************************
                    INTRSQ:recherche valeur d'un objet
    ***********************************************************************
    => Objet a rechercher.La recherche se fait sur refer1
    <= valeur de l'objet f(presilection)
    */
    pub(crate) fn intrsq(&self, objet: u16) -> u16 {
        let refer1 = self.sap_lang.GetRefer1();
        unsafe {
            let mut valeur = (*refer1.add((objet - 1) as usize)).param1 & 0xF;
            if objet == self.sapglob.predil as u16 {
                valeur *= 2;
            }
            valeur as u16
        }
    }

    /***************************************************************************
                     RAPIAS:receherche objet interessant ds liste
    **************************************************************************
    =>tribu
    <= valeur et rang de la valeur
    */
    pub(crate) fn rapias(&self) -> u16 {
        let mut max: u8 = 0;
        let mut objet: u8 = 0;
        let mut i: i16 = 0;
        while (i as u8) < self.sapglob.liste1.taille {
            let valeur = self.intrsq(self.sapglob.liste1.elements[i as usize] as u16) as u8;
            if valeur > max {
                max = valeur;
                objet = self.sapglob.liste1.elements[i as usize];
            }
            i += 1;
        }
        objet as u16
    }

    /***********************************************************************
                    EXCITE:Augmente self.sapglob.enervement
    ************************************************************************/
    pub(crate) fn excite(&mut self, valeur: u16) {
        unsafe {
            let tfr = self.sapglob.enerv.wrapping_add(valeur);
            if self.sapglob.enerv <= tfr {
                self.sapglob.enerv = tfr;
            } else {
                self.sapglob.enerv = 65535;
            }

            if self.sapglob.flefba != 0 {
                /*flefba=0;
                Sappause(2000);*/
            }
            if self.sapglob.enerv < 51 * 256
            /*tres calme*/
            {
                if self.sapglob.fpress != 0 {
                    self.cause(7, core::ptr::null());
                } else {
                    self.cause(2, core::ptr::null());
                }
                return;
            }
            if self.sapglob.enerv < 102 * 256
            /*peu self.sapglob.enerve*/
            {
                self.cause(8, core::ptr::null());
                return;
            }
            if (self.random.Rnd() as u16) < self.sapglob.enerv / 512
            /*assez self.sapglob.enerve*/
            {
                self.inimen(0);
                self.sapglob.flcomb = 1; /*combat lancé*/
                return;
            }
            if self.sapglob.enerv < 153 * 256 {
                self.cause(9, core::ptr::null());
                return;
            }
            if self.sapglob.enerv < 204 * 256 {
                self.cause(10, core::ptr::null());
                return;
            }
            self.cause(11, core::ptr::null());
        }
    }

    /***********************************************************************
                          PRENDS:On prends l'objet de rang Rega
                          dans la liste des objets du sap1 et on le transfere
                          dans la liste 10
    ***********************************************************************/
    pub(crate) fn prends(&mut self) {
        let erreur =
            self.sapglob
                .enlevl(ListeId::Liste1, ListeId::Liste10, self.sapglob.rega as u16);
        if erreur == 1 && self.sapglob.lpos > 1 {
            self.sapglob.lpos -= 1; /*elt est le dernier*/
        }
        self.inimen(self.sapglob.s_menu as u16);
    }

    /**************************************************************************
                    TRDONN:donne
    unsigned short rangb;simule registre B [0,eltmax[
    **************************************************************************/
    pub(crate) fn trdonn(&mut self, rangb: u16) {
        let objet: u8;
        let tfr: u8;
        let mut valeur: u16;

        if self.sapglob.fdialo == 0 {
            return; /*pas de dialogue alors rien*/
        }
        objet = self.sapglob.liste1.elements[rangb as usize]; /*on prendsl'objet ds la liste*/
        self.sapglob.flefba = 0;
        unsafe {
            if self.sapglob.chef != 0 && self.sapglob.tribu == 1 {
                /*on donne à son self.sapglob.chef*/
                if self.sapglob.scenar != 1 {
                    /* on compare l'objet
                    l'objet desiré*/
                    if objet != (self.sapglob.scenar as u8) + 40
                    /*objet du scenario*/
                    {
                        self.SceveuBulle((self.sapglob.scenar - 1) as u16); /*on raffiche l'objet desire*/
                        return;
                    }
                } else {
                    /*scenario = viande*/
                    if objet < 35 || objet > 39 {
                        /*viande pourrie ou autre*/
                        self.SceveuBulle((self.sapglob.scenar - 1) as u16);
                        return;
                    }
                    self.sapglob.scenpr = ((40 - objet) as i32 - 1) as i8; /*on enleve au total*/
                    if self.sapglob.scenpr > 0 {
                        /* pas fini !!!*/
                        self.prends(); /* enleve viande*/
                        self.sapglob.list10.taille = 0; /* mise à zéro taille*/
                        self.effaceBulles();
                        self.cause(6, core::ptr::null()); /* donne encore*/
                        return;
                    }
                }
                if !self.IsEnreg() {
                    self.SceveuBulle(9);
                    return;
                }
                self.prends(); /*on prends la chose*/
                /* Remerciement du self.sapglob.chef */
                self.effaceBulles();
                let mescon = self.sap_lang.GetMesCon();
                match self.sapglob.scenar {
                    1 => self.bulle(mescon[81] as Ptr, 1),
                    2 => self.bulle(mescon[82] as Ptr, 1),
                    3 => self.bulle(mescon[83] as Ptr, 1),
                    4 => self.bulle(mescon[83] as Ptr, 1),
                    _ => {}
                }
                self.sapglob.scenar += 1;
                if self.sapglob.scenar == 5 {
                    /*combat ultime*/
                    self.sapglob.flcomb = 1;
                    self.SceveuBulle((self.sapglob.scenar - 1) as u16);
                    /* Delay de 5 seconde */
                    self.delai(5 * 60);
                    return;
                }
                if self.sapglob.scenar == 3 {
                    /*bras de roc 'achement self.sapglob.enerves et on doit lui voler sa hache*/
                    self.sapglob.enetoi[2] = 60000;
                }
                /*comment(80 + scenar);*/
                /*affiche commentaire selon scenar*/
                return; /*fin partie donne->scenario*/
            }
            /*-----------------------*/
            /* on donne hors scenario  */
            /*-----------------------*/

            /*cherche valeur de l'objet*/
            self.effaceBulles();
            valeur = self.sapglob.enerv.wrapping_sub(
                512 * self.intrsq(self.sapglob.liste1.elements[self.sapglob.rega as usize] as u16),
            );
            /*compense l'enervement*/
            if valeur > self.sapglob.enerv {
                /*bien compense*/
                valeur = 0;
            }
            if self.sapglob.fpress == 0 {
                /*pas presente => cado */
                if self.sapglob.list10.taille <= self.sapglob.list10.maxtaille {
                    self.prends(); /*y a de la place : on prends*/
                    if self.sapglob.enerv < 102 * 256 {
                        self.cause(1, core::ptr::null()); /*merci*/
                        self.cause(12, core::ptr::null()); /*je suis ...*/
                    }
                    if self.sapglob.agress > 0 {
                        self.sapglob.agress -= 1;
                    }
                    /*l'agressivite descend*/
                    /*Sappause(2000);*/
                    self.sapglob.enerv = valeur; /*l'enervement diminue*/
                    self.excite(0); /*on affiche l'etat d'enerv*/
                }
                return;
            }
            /*---------*/
            /* self.sapglob.echange */
            /*---------*/
            if self.sapglob.enerv < 102 * 256 {
                valeur =
                    self.intrsq(self.sapglob.liste1.elements[self.sapglob.rega as usize] as u16);
                if self.sapglob.chef != 0 && self.sapglob.scenar as u16 == self.sapglob.tribu as u16
                {
                    if self.sapglob.liste1.elements[self.sapglob.rega as usize]
                        != self.sapglob.predil
                    {
                        valeur = 0;
                    } else {
                        valeur = 200;
                    }
                }
                if valeur > self.sapglob.vlchan as u16
                /*valeur superieure*/
                {
                    /*ok il prends*/
                    self.sapglob.vlchan = valeur as u8;
                    self.cause(4, core::ptr::null()); /*marché conclu*/
                    if self.sapglob.echan == 1 {
                        /*protection*/
                        self.sapglob.enetoi[self.sapglob.tribu as usize - 1] = 0; /*enerv =0*/
                        self.sapglob.echan = 25; /*onguent*/
                    } else {
                        /*on permute les deux*/
                        tfr = self.sapglob.liste1.elements[rangb as usize];
                        self.sapglob.liste1.elements[rangb as usize] = self.sapglob.echan;
                        self.sapglob.echan = tfr;
                        self.inimen(self.sapglob.s_menu as u16);
                    }
                } else {
                    /*insuffisant*/
                    self.cause(3, core::ptr::null()); /*c'est pas assez glandu !!!*/
                    self.excite(50);
                }
            }
            /*si donne et pas trop self.sapglob.enerve*/
            else {
                /*enerv<e*/
                if self.sapglob.list10.taille < self.sapglob.list10.maxtaille {
                    /*il ya de la place*/
                    self.prends();
                    self.sapglob.enerv = valeur;
                    self.cause(5, core::ptr::null()); /*merci du cado*/
                    /*..
                    suivre...*/
                }
            }
        }
    }

    /***********************************************************************
                    IDIALO:Initialise l'adversaire en fn de la presence
    ***********************************************************************/
    pub(crate) fn idialo(&mut self) {
        let mut i: i16;
        let mut pos: u8;
        self.sapglob.fdialo = 0;
        self.sapglob.flcomb = 0;
        self.sapglob.fpress = 0;
        self.sapglob.flgcharism = 0;
        self.sapglob.dquest = 0;
        if self.sapglob.presence == 0 {
            return; /*si personne */
        }
        if self.sapglob.presence > 4 {
            /*bestiole*/
            self.sapglob.echan = 0;
            self.sapglob.flcomb += 1; /*on se bat*/
            return;
        }
        self.sapglob.fdialo += 1; /*on discute*/
        self.sapglob.enerv = tenerv[self.sapglob.tribu as usize - 1];
        self.sapglob.agress = self.sapglob.tagress[self.sapglob.tribu as usize - 1];
        self.sapglob.predil = tabprf[self.sapglob.tribu as usize - 1];
        /*objet
        self.sapglob.echanger*/
        self.sapglob.echan =
            tobje[self.sapglob.tribu as usize - 1][(self.random.Rnd() & 3) as usize];

        /*objet portes*/
        for i in 0..2 {
            self.sapglob.list10.elements[i] =
                tobje[self.sapglob.tribu as usize - 1][(self.random.Rnd() & 3) as usize];
        }
        self.sapglob.list10.taille = 2;
        /*bkmots[3][1]=(self.sapglob.tribu+0x85);*/
        /*tribu*/
        self.saptxt.nomtribu[1] = (self.sapglob.tribu + 0x85) as i8;
        self.saptxt.nomtribu[2] = 0;

        if self.sapglob.chef != 0 {
            /*c'est un self.sapglob.chef*/
            self.saptxt.nomtribu[0] = 0x85u8 as i8; /*Chef*/
            self.saptxt.nomadversaire[0] = (self.sapglob.tribu + 0x89) as i8;
            self.saptxt.nomadversaire[1] = 0;
            /*   bkmots[2][0]=(self.sapglob.tribu+0x89);*/
            /*nom du self.sapglob.chef*/
            /* bkmots[2][1]=0;*/
            /*fin*/
            /*  bkmots[3][0]=0x85;*/
            /*'chef'*/
            if self.sapglob.tribu == 1 {
                self.sapglob.fpress = 1;
                self.sapglob.enerv = self.sapglob.enetoi[self.sapglob.tribu as usize - 1];
            }
            if self.sapglob.scenar == self.sapglob.tribu && self.sapglob.tribu != 4 {
                self.sapglob.echan = self.sapglob.tribu + 40; /*objet volé par la self.sapglob.tribu*/
            } else {
                self.sapglob.echan = 1; /*ma protection*/
            }
        } else {
            /*bkmots[3][0]=32 ;*/
            self.saptxt.nomtribu[0] = 0x20;
            i = 0;
            while i < 6 {
                /*determine le nom*/
                pos = (48 * self.random.Rnd() as i16 / 256) as u8;
                self.saptxt.nomadversaire[i as usize] = self.saptxt.maced[pos as usize];
                i += 1;
                pos += 1;
                self.saptxt.nomadversaire[i as usize] = self.saptxt.maced[pos as usize];
                i += 1;
                pos += 1;
                self.saptxt.nomadversaire[i as usize] = self.saptxt.maced[pos as usize];
                i += 1;
            }
            self.saptxt.nomadversaire[i as usize] = 0;
        }
        /*tire nom secondaire*/
        i = 0;
        while i < 6 {
            /*determine le nom*/
            pos = (48 * self.random.Rnd() as i16 / 256) as u8;
            self.saptxt.nomsecondaire[i as usize] = self.saptxt.maced[pos as usize];
            i += 1;
            pos += 1;
            self.saptxt.nomsecondaire[i as usize] = self.saptxt.maced[pos as usize];
            i += 1;
            pos += 1;
            self.saptxt.nomsecondaire[i as usize] = self.saptxt.maced[pos as usize];
            i += 1;
        }

        self.saptxt.nomsecondaire[i as usize] = 0;
        self.sapglob.vlchan = self.intrsq(self.sapglob.echan as u16) as u8;
        self.sapglob.smenu[1] = 111; /* idee d'yves */

        self.inimen(0);
    }

    /***********************************************************************
                            QUITTE:On lui fait peur :il s'enfuit
    ***********************************************************************/
    pub(crate) fn quitte(&mut self) {
        self.sapglob.tfrene();
        self.sapglob.flcomb = 0;
        self.sapglob.presence = 0;
        self.idialo();
        self.sapglob.present[1] = 0;
        self.sapglob.present[3] = 0;
        self.prm(SapId::Sap2, 0);
        self.prm(SapId::Sap4, 0);
        /* inits souris */
        self.sapglob.dsposx = self.sapglob.sposx;
        self.sapglob.dsposy = self.sapglob.sposy;
        self.reset_mouse();
        if self.sapglob.dervue == Vue::LATERALE {
            self.laterb(false);
        } else {
            self.latera();
        }
        self.generique(true);
        self.inimen(0);
    }

    /***********************************************************************
                            CREECOP:Crée un copain
    ***********************************************************************/
    pub(crate) fn creecop(&mut self, qui: u16) {
        /*c'est ds la poche*/
        /*on recopie sap2 sur sap3*/
        BlockMove(
            addr_of!(self.sapglob.sap2.cross) as *const c_void,
            addr_of_mut!(self.sapglob.sap3.cross) as *mut c_void,
            8,
        ); /*parametres*/
        unsafe {
            if self.sapglob.presence < 5 {
                strcpy(
                    addr_of!(self.saptxt.nomcopain) as *mut i8,
                    addr_of!(self.saptxt.nomadversaire) as *const i8,
                ); /*nom*/
            } else {
                self.saptxt.nomcopain[0] = (128 + 45) as i8; /*Ton ami*/
                self.saptxt.nomcopain[1] = 0;
            }
            self.prm(SapId::Sap3, qui as i16);
            let sap2 = &self.sapglob.sap2;
            let sap3 = &mut self.sapglob.sap3;
            sap3.colonne = sap2.colonne;
            sap3.sens = sap2.sens;
            sap3.coeur = sap2.coeur;
            sap3.cptrsouffle = 1;
            self.prm(SapId::Sap2, 0);
            if self.sapglob.dervue != Vue::PAYSAGE
            /* on est en vue laterale*/
            {
                self.sapglob.presence = 0;
                self.trobsv(1);
            }
            self.sapglob.echan = 0;
            let liste_id = self.sapglob.peuple_mut(SapId::Sap3).possession;
            self.sapglob.liste_mut(liste_id).taille = 0;
        }
        self.generique(true);
    }

    /***********************************************************************
                            BONNECHASS
    ***********************************************************************/
    pub(crate) fn bonnechass(&mut self) {
        let mescon = self.sap_lang.GetMesCon();
        unsafe {
            if self.sapglob.enerv > 102 * 256 {
                /*énervé!!!*/
                self.excite(300);
            } else {
                if self.sapglob.fpress == 0
                /*pas présenté*/
                {
                    self.cause(2, core::ptr::null()); /*qui est tu ???*/
                } else {
                    /*il s'en va*/
                    /*bonne chasse*/
                    self.bulle(mescon[72] as Ptr, 1);
                    self.quitte();
                    self.inimen(0);
                    self.effaceBulles();
                }
            }
        }
    }

    /***********************************************************************
                            VIENSAVEC
    ***********************************************************************/
    pub(crate) fn viensavec(&mut self) {
        if self.sapglob.fpress != 0
            && self.sapglob.presence != 0
            && self.sapglob.presence < 5
            && self.sapglob.present[2] == 0
            && self.sapglob.present[3] == 0
            && self.sapglob.chef == 0
            && self.sapglob.flgcharism == 0
        {
            if (self.sapglob.charisme as i32 * self.random.Rnd() as i32) > self.sapglob.enerv as i32
                && self.random.Rnd() < 70
            {
                self.creecop(1);
                self.sapglob.viecopain = 3;
                let sap3 = &mut self.sapglob.sap3;
                sap3.vie = 200;
                /*je suis avec toi*/
                let mescon = self.sap_lang.GetMesCon();
                self.bulle(mescon[99] as Ptr, 1);
                self.sapglob.flagbadi = 0;
            } else {
                self.excite(1500);
            }
        } else {
            /*je ne suis pas celle que vous croyez*/
            let mescon = self.sap_lang.GetMesCon();
            self.bulle(mescon[98] as Ptr, 1);
            self.excite(2000);
        }
        self.sapglob.flgcharism = 1;
    }

    /***********************************************************************
                            TUVEUQUOI
    ***********************************************************************/
    pub(crate) fn tuveuquoi(&mut self) {
        let valeur: u8;
        let refer1 = self.sap_lang.GetRefer1();
        let mescon = self.sap_lang.GetMesCon();
        let scenme = self.sap_lang.GetScenme();
        unsafe {
            if self.sapglob.enerv > 102 * 256 {
                self.cause(13, core::ptr::null()); /*ta vie !!*/
            } else {
                if self.sapglob.chef != 0 && self.sapglob.tribu == 1 {
                    self.sapglob.dquest = 0;
                    self.bulle(*scenme.add((self.sapglob.scenar as usize) - 1) as Ptr, 1);
                } else {
                    if self.sapglob.tribu == self.sapglob.scenar && self.sapglob.chef != 0 {
                        self.cause(
                            14,
                            addr_of_mut!(
                                (*refer1.add((self.sapglob.predil.wrapping_sub(1)) as usize))
                                    .nomref[0]
                            ),
                        );
                        return;
                    }
                    valeur = self.intrsq(self.rapias()) as u8;
                    if valeur > self.sapglob.vlchan {
                        self.cause(
                            14,
                            addr_of_mut!((*refer1.add((self.rapias() - 1) as usize)).nomref[0]),
                        ); /*je veux*/
                    } else {
                        if self.sapglob.chef != 0 {
                            self.cause(
                                14,
                                addr_of_mut!(
                                    (*refer1.add((self.sapglob.predil.wrapping_sub(1)) as usize))
                                        .nomref[0]
                                ),
                            );
                        } else {
                            /*rien !!!*/
                            self.bulle(mescon[65] as Ptr, 1);
                            self.excite(0);
                        }
                    }
                }
            }
        }
    }

    /***********************************************************************
                            TRDIRE:Célakonköse
    ***********************************************************************
    => DIQuoi : Le numéro de la phrase ds la liste des phrases
    */
    pub(crate) fn trdire(&mut self, diquoi: u16) {
        let tfr: i16;
        let mut enerv2: u16;
        let refer1 = self.sap_lang.GetRefer1();
        if self.sapglob.fdialo == 0 {
            return; /*si pas de dialogue alors rien*/
        }
        self.sapglob.flefba = 1; /* pas d'effacement au prochain*/
        unsafe {
            if diquoi == self.sapglob.dquest as u16 {
                self.excite(1000);
                return;
            }
            self.sapglob.dquest = diquoi as u8; /*derniere question*/

            if diquoi == 5
            /*je suis tedehur*/
            {
                if self.sapglob.fpress == 0
                /*pas encore presente*/
                {
                    self.sapglob.fpress += 1; /* on est présenté*/
                    /*calcul de la reponse en fn de l'enervement*/
                    tfr = (self.sapglob.enerv as i32
                        - tenerv[self.sapglob.tribu as usize - 1] as i32)
                        as i16;
                    self.sapglob.enerv = self.sapglob.enetoi[self.sapglob.tribu as usize - 1];
                    enerv2 = (self.sapglob.enerv as i32).wrapping_add(tfr as i32) as u16;
                    if tfr > 0 && enerv2 < self.sapglob.enerv {
                        enerv2 = 65535;
                    }
                    if tfr < 0 && enerv2 > self.sapglob.enerv {
                        enerv2 = 0;
                    }
                    self.sapglob.enerv = enerv2;
                    let mescon = self.sap_lang.GetMesCon();
                    if self.sapglob.enerv > 0x3400 {
                        /*c'est donc toi*/
                        self.bulle(mescon[71] as Ptr, 1);
                        self.excite(0);
                    } else {
                        /*bonjour*/
                        self.bulle(mescon[70] as Ptr, 1);
                    }
                } else {
                    /*deja présenté*/
                    self.excite(600); /*on s'enerve*/
                }
                return;
            }

            if diquoi == 6 {
                self.bonnechass();
                return;
            }
            if diquoi == 7 || diquoi == 8
            /*viens avec moi*/
            {
                self.viensavec();
                return;
            }

            /*la phrase n'est ni je suis ni au revoir */
            if self.sapglob.stock1 == 0 {
                if diquoi == 1
                /*qui est tu ?*/
                {
                    if self.sapglob.enerv > 102 * 256 {
                        self.excite(100);
                    } else {
                        self.cause(12, self.GetStrBkMot(129 + 3)); /*'je suis'*/
                        if self.sapglob.fpress == 0 && self.sapglob.chef == 0 {
                            /*et toi??*/
                            let mescon = self.sap_lang.GetMesCon();
                            self.bulle(mescon[64] as Ptr, 1);
                            self.excite(200);
                        }
                    }
                    return;
                }
                if diquoi == 2
                /*que veux tu*/
                {
                    self.tuveuquoi();
                    return;
                }
                if diquoi == 3
                /*que m'offres tu ?*/
                {
                    if self.sapglob.enerv > 256 * 102
                    /*trop énervé pour négocier*/
                    {
                        /*rien...*/
                        let mescon = self.sap_lang.GetMesCon();
                        self.bulle(mescon[65] as Ptr, 1);
                        tfr = (self.sapglob.enerv as i32).wrapping_add(200) as i16;
                        if (self.sapglob.enerv as i32) < (tfr as i32) {
                            self.sapglob.enerv = tfr as u16;
                        }
                    } else {
                        if self.sapglob.fpress == 0 {
                            self.cause(2, core::ptr::null()); /*qui est tu ???*/
                        } else {
                            self.cause(
                                15,
                                addr_of_mut!(
                                    (*refer1.add(self.sapglob.echan.wrapping_sub(1) as usize))
                                        .nomref[0]
                                ),
                            ); /*moi offrir*/
                        }
                    }
                    return;
                }
                /*ou est ton self.sapglob.chef*/
                if self.sapglob.chef != 0
                /*c'est lui !!!*/
                {
                    self.excite(32700); /*il s'excite*/
                } else {
                    if self.sapglob.enerv > 51 * 256 {
                        self.cause(16, core::ptr::null()); /*je dirais rien*/
                        tfr = (self.sapglob.enerv as i32).wrapping_add(256) as i16;
                        if (self.sapglob.enerv as i32) < (tfr as i32) {
                            self.sapglob.enerv = tfr as u16;
                        }
                    } else {
                        if self.sapglob.fpress == 0 {
                            self.cause(2, core::ptr::null()); /*qui est tu*/
                        } else {
                            self.saptxt.poschef[0] = 32;
                            self.saptxt.poschef[2] = 32;
                            /*determination de la position du self.sapglob.chef*/
                            if self.sapglob.xj / 256
                                < self.sapglob.destine[self.sapglob.tribu as usize - 1].posx as u16
                            {
                                self.saptxt.poschef[0] = (128 + 31) as i8;
                            }
                            if self.sapglob.xj / 256
                                > self.sapglob.destine[self.sapglob.tribu as usize - 1].posx as u16
                            {
                                self.saptxt.poschef[0] = (128 + 32) as i8;
                            }
                            if self.sapglob.yj / 256
                                > self.sapglob.destine[self.sapglob.tribu as usize - 1].posy as u16
                            {
                                self.saptxt.poschef[2] = (128 + 33) as i8;
                            }
                            if self.sapglob.yj / 256
                                < self.sapglob.destine[self.sapglob.tribu as usize - 1].posy as u16
                            {
                                self.saptxt.poschef[2] = (128 + 34) as i8;
                            }
                            self.bulle(addr_of!(self.saptxt.poschef) as *mut i8 as Ptr, 1); /*affiche le commentaire*/
                        }
                    }
                }
            } else {
                /*stock1 <> 0*/
                /*flagornerie ou insulte*/
                match self.sapglob.stock1 {
                    1 => {
                        /*insulte*/
                        self.excite((200 * self.sapglob.stock2) as u16);
                    }
                    2 => {
                        /*flagornerie*/
                        if self.sapglob.agress < 200 {
                            self.sapglob.agress += 1;
                            /*enervement diminue*/
                            tfr = (self.sapglob.enerv as i32 - 100 * self.sapglob.stock2 as i32)
                                as i16;
                            if (tfr as i32) < (self.sapglob.enerv as i32) {
                                self.sapglob.enerv = tfr as u16;
                            }
                            self.excite(0);
                        } else {
                            /*vantardise*/
                            if self.sapglob.agress < 200 && self.sapglob.chef == 0
                            /*si trop self.sapglob.agressif ça marche pas*/
                            {
                                self.sapglob.agress += 1;
                                if self.random.Rnd() > (self.sapglob.enerv / 200) as i16 {
                                    self.fuite();
                                } else {
                                    /*enervement diminue*/
                                    tfr = (self.sapglob.enerv as i32
                                        - 100 * self.sapglob.stock2 as i32)
                                        as i16;
                                    if (tfr as i32) < (self.sapglob.enerv as i32) {
                                        self.sapglob.enerv = tfr as u16;
                                    }
                                    self.excite(0);
                                }
                            } else {
                                /*il le prend pour une insulte*/
                                self.excite((200 * self.sapglob.stock2) as u16);
                            }
                        }
                    }
                    _ => {
                        /*vantardise par defaut*/
                        if self.sapglob.agress < 200 && self.sapglob.chef == 0
                        /*si trop self.sapglob.agressif ça marche pas*/
                        {
                            self.sapglob.agress += 1;
                            if self.random.Rnd() > (self.sapglob.enerv / 200) as i16 {
                                self.fuite();
                            } else {
                                /*enervement diminue*/
                                tfr = (self.sapglob.enerv as i32 - 100 * self.sapglob.stock2 as i32)
                                    as i16;
                                if (tfr as i32) < (self.sapglob.enerv as i32) {
                                    self.sapglob.enerv = tfr as u16;
                                }
                                self.excite(0);
                            }
                        } else {
                            /*il le prend pour une insulte*/
                            self.excite((200 * self.sapglob.stock2) as u16);
                        }
                    }
                } /*fin du switch*/
            }
            return;
        }
    }

    /***********************************************************************
                            VOLE:On a perdu: il nous pique tout
    ***********************************************************************/
    pub(crate) fn vole(&mut self) {
        let mut i: i16;
        self.sapglob.liste4.taille = 0; /*plus rien à terre*/
        i = self.sapglob.liste1.taille as i16;
        while i != 0 {
            i -= 1;
            /* si la valeur est suffisante que ce n'est pas un objet scenario
            et avec un leger Rnd
             */
            if self.intrsq(self.sapglob.liste1.elements[i as usize] as u16) > 5
                && self.sapglob.liste1.elements[i as usize] < 40
                && self.random.Rnd() > 128
            {
                self.sapglob
                    .enlevl(ListeId::Liste1, ListeId::Listbidon, i as u16);
            }
        }
        self.sapglob.tlpos[1] = 1;
        self.sapglob.tlpos[4] = 1;
    }

    /***********************************************************************
                            OUTARM :Armes en sorties
    ***********************************************************************/
    pub(crate) fn outarm(&mut self) {
        let tfr: u8;

        self.sapglob.tlpos[1] = 1;
        self.sapglob.tlpos[4] = 1;

        let mut i = self.sapglob.liste1.taille;
        /*on enleve les sagaies de la liste en fonction de ce qui reste en sortie
        bien sur comme la taille va bouger on remet les positions ds les listes

        zéro*/
        while i != 0 {
            i -= 1;
            if self.sapglob.liste1.elements[i as usize] == 23 {
                let sap1 = &mut self.sapglob.sap1;
                if sap1.nbresag != 0 {
                    sap1.nbresag -= 1;
                } else {
                    self.sapglob
                        .enlevl(ListeId::Liste1, ListeId::Listbidon, i as u16);
                }
            }
        }

        let forcehache = self.sapglob.sap1.forcehache;
        if forcehache == 0 {
            /*pas de hache*/
            i = self.sapglob.liste1.taille;
            while i != 0 {
                i -= 1;
                if self.sapglob.liste1.elements[i as usize] == 21
                    || self.sapglob.liste1.elements[i as usize] == 22
                {
                    self.sapglob
                        .enlevl(ListeId::Liste1, ListeId::Listbidon, i as u16);
                    i = 0;
                }
            }
        }
        let sap1 = &self.sapglob.sap1;
        if sap1.vie != self.sapglob.entvie {
            /*il a encaissé*/
            tfr = self
                .sapglob
                .fatig
                .wrapping_sub(self.sapglob.entvie.wrapping_sub(sap1.vie) / 2);
            self.sapglob.fatig = if tfr < self.sapglob.fatig { tfr } else { 0 };
        }
    }

    /***********************************************************************
                            TRDORS:
    ***********************************************************************/
    pub(crate) fn trdors(&mut self) {
        let mut tfr: u8;
        if self.sapglob.fatig > 192 {
            self.sapglob.flefba = 0;
            self.comment(108);
            return;
        }

        self.sapglob.liste4.taille = 0; /*plus rien a terre*/
        self.sapglob.tlpos[4] = 1;
        tfr = self.sapglob.fatig.wrapping_add(self.sapglob.stock2);

        if tfr < self.sapglob.fatig {
            tfr = 255;
        }
        self.sapglob.fatig = 255;
        self.sapglob.dervue = Vue::PAYSAGE;
        /*affiche ecran n fois */
        /*avec chronos*/
        self.sapglob.stock2 = self.sapglob.stock2 / 7;
        while self.sapglob.stock2 != 0 {
            self.GereInterface();
            for _i in 0..9 {
                self.sapglob.minute = 0;
                self.chrono(1);
            }
            let t = TickCount();
            while TickCount() < t + 10 { /* vide */ }
            self.affich(self.sapglob.dzeta as u16);
            self.tfrima();
            self.sapglob.minute = 0;
            self.chrono(2);
            self.sapglob.stock2 -= 1;
        }
        self.sapglob.fatig = tfr;
        self.chrono(0);
        self.afstat();

        self.inimen(0);
    }

    /**************************************************************************
                        InitVueLaterale
    **************************************************************************/
    unsafe fn InitVueLaterale(&mut self) {
        if self.sapglob.dervue != Vue::LATERALE {
            self.sapglob.ldzeta = self.sapglob.dzeta;
        }
        self.prm(SapId::Sap2, self.sapglob.present[1] as i16);
        self.prm(SapId::Sap4, self.sapglob.present[3] as i16);
        self.sapglob.flcomb = 1;
        if self.sapglob.presence != 0 {
            if self.sapglob.presence < 5 {
                self.sapglob.flcomb = 0;
            }
            self.sapglob.presence &= 127; /*elimine bit 7 pour bestiole amie*/
            self.prm(SapId::Sap1, self.sapglob.present[0] as i16);
            self.prm(SapId::Sap3, self.sapglob.present[2] as i16);
            if self.sapglob.dervue != Vue::LATERALE {
                self.latera(); /* un affichage et dialogue*/
            }
            self.sapglob.dervue = Vue::LATERALE;
        }
        self.pos0_mouse();
    }

    /***********************************************************************
                            TROBSV:Traite l'observation
    => 0 vue 3d
       1 vue laterale
       2 panorama
       3 carte
    ***********************************************************************/
    pub(crate) fn trobsv(&mut self, regb: u16) {
        let dzetdep: u16;
        let mut regb = regb;
        let mut posTimer: u64;

        if self.sapglob.presence != 0 {
            return; /* si il ya quelun alors on est poli*/
        }
        unsafe {
            if regb == 0 {
                /* vue 4 D*/
                self.saprel.LastObsv = 0;
                self.sapglob.dismin = 0;
                self.sapglob.lastmen = 255;
                self.clrbadi();
                self.sapglob.dervue = Vue::PAYSAGE;
                /* gestion de la vue 3d */
                self.MenuFenetres(FAUX as i16);
                self.AffichePictAide(1);
                self.clav3d();
                self.MenuFenetres(VRAI as i16);
                self.pos0_mouse();
                /* Une presence a ete rencontrée : on initialise la
                vue laterale */
                if self.sapglob.presence != 0 {
                    /* On initialise la vue laterale */
                    regb = 1;
                }
            }
            if regb == 1 {
                /*vue laterale*/
                self.saprel.LastObsv = 1;
                self.InitVueLaterale();
                return;
            }
            if regb == 2 {
                /*panaroma*/
                while self.getkey(0) != 0 { /* vide */ }
                self.sapglob.dervue = Vue::PAYSAGE;
                dzetdep = self.sapglob.dzeta as u16;
                loop {
                    posTimer = self.sapwclav.sapiens_timer.load(Ordering::Relaxed);
                    self.affich(self.sapglob.dzeta as u16);
                    self.tfrima();
                    self.sapglob.dzeta = ((self.sapglob.dzeta as i32 + 16) & 0x3FF) as i16;
                    while posTimer
                        > self
                            .sapwclav
                            .sapiens_timer
                            .load(Ordering::Relaxed)
                            .wrapping_sub(8)
                    { /* vide */ }
                    if dzetdep as i32 == self.sapglob.dzeta as i32 {
                        break;
                    }
                    if self.getkey(0) != 0 {
                        break;
                    }
                }
            }
            if regb == 3 {
                /*carte*/
                self.sapglob.dervue = Vue::PAYSAGE;
                self.gemouvre(
                    self.sapglob.clin,
                    self.sapglob.ccol,
                    self.sapglob.cnlin,
                    self.sapglob.cncol,
                    15,
                    30,
                    8,
                    15,
                );
                self.zoocar();
                self.sapglob.lastmen = 255;
            }
        }
        self.inimen(0);
        self.pos0_mouse();
        self.efbadass();
        self.TfrBadAss();
    }

    /***********************************************************************
            ZONEVT:Determine un evt en fonction de la position
    ***********************************************************************/
    pub(crate) fn zonevt(&mut self, okmess: u16) -> i16 {
        let mut dieu: *mut evt;
        let mut nuit: bool;
        let mut qui: u8;
        let mut dist: i16;
        unsafe {
            /*on balaye tout les evts*/
            for i in 0..NBREVTDESTINE as i16 {
                dieu = &raw mut self.sapglob.destine[i as usize];
                dist = distance(
                    (*dieu).posx as i64 * 256 - self.sapglob.xj as i64,
                    (*dieu).posy as i64 * 256 - self.sapglob.yj as i64,
                );
                dist /= 256;
                /*dist est mis a jour*/
                if dist > (*dieu).distmax as i16
                /*dans la zone d'influence*/
                {
                    (*dieu).type_ &= 127; /*on revalide la zone en clearant le bit 128*/
                } else
                /*ok on est ds la zone*/
                {
                    /*la zone est-elle valide ??,sommes mous assez central*/
                    if (*dieu).type_ & 128 == 0
                        && (dist as i32 * 12 + 10) < (self.random.Rnd() as u16) as i32
                    {
                        /*on compare
                        la densité probabiliste de la zone*/
                        if self.random.Rnd() >= (*dieu).rnd as i16 {
                            /* on affiche un message (l'evt n'a???*/
                            if self.random.Rnd() > 240 {
                                if okmess != 0 {
                                    self.efbadi();
                                    self.comment((*dieu).message as u16);
                                    return 0;
                                }
                            }
                        } else {
                            if (*dieu).evt == 0
                            /*eau ??*/
                            {
                                self.sapglob.source += 1;
                                self.sapglob.liste4.taille = 1;
                                self.sapglob.tlpos[4] = 1;
                                self.sapglob.liste4.elements[0] = 2;
                                self.efbadi();
                                self.comment(76); /*tu entends une source*/
                                return 0;
                            } else {
                                /* est ce un self.sapglob.chef ???*/
                                self.sapglob.chef = if (*dieu).evt > 127 { 1 } else { 0 };
                                qui = (*dieu).evt & 127;
                                /* one shot !!!*/
                                if (*dieu).type_ != 0 {
                                    (*dieu).type_ |= 128;
                                }
                                nuit = self.sapglob.heure > 512;
                                /* test spécifiques a la presence et au moment de la journée*/
                                if self.sapglob.chef != 0
                                    || (nuit && qui != 5 && self.random.Rnd() > 222)
                                    || (!nuit && qui == 5 && self.random.Rnd() > 222)
                                    || (nuit && qui == 5)
                                    || (!nuit && qui != 5)
                                {
                                    return qui as i16;
                                }
                            } /*evt =eau ou vivant*/
                        } /*Rnd correct*/
                    } /*zone valide*/
                } /*dans la zone*/
            } /*pour tout les elements*/
            0
        }
    }

    /***********************************************************************
                            ZONINF:Gestion des evts selon la position
    ***********************************************************************/
    /* on travaille sur la position du joueur,en sortie les flags de presence
    sont mis à jour
    en sortie 0 aucune presence generee sinon type de la presence de 1 à 8
     */
    pub(crate) fn zoninf(&mut self) {
        let tfr: u8;
        let mut i: u8;

        /*si dej
        present alors fin*/
        if self.sapglob.presence != 0 {
            return;
        }
        /*la position est-elle differente*/
        if self.sapglob.derxj == (self.sapglob.xj & 0xFFE0)
            && self.sapglob.deryj == (self.sapglob.yj & 0xFFE0)
        {
            return;
        }
        /*on mémorise la derniere position*/
        self.sapglob.derxj = self.sapglob.xj & 0xFFE0;
        self.sapglob.deryj = self.sapglob.yj & 0xFFE0;
        self.sapglob.chef = 0;
        self.sapglob.flcomb = 0;
        self.sapglob.source = 0;
        self.sapglob.present[1] = self.zonevt(1) as i8; /*avec message possible*/
        if self.sapglob.present[1] != 0
        /*il ya un evt vivant*/
        {
            /* s'il ya un self.sapglob.chef alors il est automatiquement accompagné*/
            /*sinon on retire un evt pour le deuxieme*/
            if self.sapglob.chef == 0 {
                self.sapglob.present[3] = self.zonevt(0) as i8; /*sans message possible*/
            } else {
                self.sapglob.present[3] = self.sapglob.present[1];
            }
            /* si il y un homme il est en premier*/
            if (self.sapglob.present[1] as u8) > 4
                && (self.sapglob.present[3] as u8) < 5
                && self.sapglob.present[3] != 0
            {
                /*on swape*/
                tfr = self.sapglob.present[1] as u8;
                self.sapglob.present[1] = self.sapglob.present[3];
                self.sapglob.present[3] = tfr as i8;
            }
            /*ds tout les cas la presence est fixée a celle du prioritaire*/
            self.sapglob.presence = self.sapglob.present[1] as i16;
            self.sapglob.tribu = self.sapglob.presence as u8;
            if self.sapglob.dervue != Vue::LATERALE {
                let sap1 = &mut self.sapglob.sap1;
                sap1.colonne = 3;
            }
            /* On calcule les positions des adversaires */
            self.prm(SapId::Sap2, self.sapglob.present[1] as i16);
            self.prm(SapId::Sap4, self.sapglob.present[3] as i16);
            self.idialo();
            /* on teste  le charisme immediat sur animal*/
            /* si on est seduisant,qu'il y a une seule bestiole et avec un peu de chance*/
            if self.sapglob.presence > 4
                && (self.sapglob.charisme as i16) > self.random.Rnd()
                && self.random.Rnd() < 40
                && self.sapglob.present[2] == 0
            {
                self.sapglob.flefba = 0;
                self.comment((97 + self.sapglob.presence) as u16); /*premier mess 102*/
                self.creecop(self.sapglob.presence as u16);
                self.sapglob.viecopain = 1; /*pour les bestioles*/
                self.sapglob.flefba = 0;
                if self.sapglob.dervue == Vue::LATERALE {
                    self.sapglob.flagbadi = 0;
                }
                self.sapglob.presence = 128; /*bit 7 et presence 0*/
                return;
            }
        }
        if self.sapglob.source == 0 {
            /*aucun evt n'est valide */
            if self.sapglob.scenar == 4 && self.sapglob.scenpr == 0 {
                /* A la bonne position on trouve la Hache Scrée des Hyenes folles */
                if ((if self.sapglob.derxj > 0xABE0 {
                    self.sapglob.derxj - 0xABE0
                } else {
                    0xABE0 - self.sapglob.derxj
                }) < 500)
                    && ((if self.sapglob.deryj > 0xB2E0 {
                        self.sapglob.deryj - 0xB2E0
                    } else {
                        0xB2E0 - self.sapglob.deryj
                    }) < 500)
                {
                    self.sapglob.liste4.taille = 1;
                    self.sapglob.tlpos[4] = 1;
                    self.sapglob.liste4.elements[0] = 44;
                    return;
                }
            }
            self.sapglob.liste4.taille = self.random.Rnd() as u8 & 7; /*jusqu'a 7 elements à terre*/
            i = self.sapglob.liste4.taille;
            self.sapglob.tlpos[4] = 1; /* debut de liste */
            /*on génère les objets a terre*/
            while i != 0 {
                i -= 1;
                self.sapglob.liste4.elements[i as usize] =
                    (3 + (17 * self.random.Rnd()) / 256) as u8;
            }
        }
    }
}

/***********************************************************************
                        s_action:pour afstat
***********************************************************************/
