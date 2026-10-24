#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/*;-----------------------------------------------------------------
;               HIRES;Passe en resolution et detourne Erreurs Disques
;-----------------------------------------------------------------------
*/
use crate::acam::{
    Alert, BackColor, CarboniseSetCursorArrow, CheckItem, CtoPstr, DisableItem, DisposHandle,
    DrawMenuBar, EnableItem, FAUX, ForeColor, GetCursor, GetIndString, GetItem, GetMHandle,
    GetNewMBar, NewCWindow, ParamText, PenNormal, PtoCstr, Random, SetCursor, SetMenuBar,
    SetWTitle, TickCount, VRAI,
};
use crate::audio;
use crate::cbridge::{
    DRVR, DialogPtr, MenuHandle, WindowPtr, blackColor, bold, charCodeMask, cmdKey, ctrlItem,
    everyEvent, geneva, helvetica, optionKey, radCtrl, strcpy, strlen, userItem, watchCursor,
    whiteColor,
};
use crate::exchange::take_menu_command;
use crate::langue::{langue_i16, set_langue};
use crate::port;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::saptxt::CvtDefaultLangRess;
use crate::sapiens::sapwbrt::{
    DoEraseRect, DoFrameRect, DoLine, DoMoveTo, DoPenSize, DoTextSize, DoZoomRect,
    SapiensCreeOffscreen, SetPathInData,
};
use crate::sapiens::{ECHELLE, STR_VRAC, TAILLE_STR_MESS_FORMATE, Vue};
use crate::shared::{
    Boolean, CGrafPtr, ControlHandle, EventRecord, Handle, PicHandle, RGBColor, Rect, StringPtr,
    TEHandle,
};
use crate::stubs::{
    AddResMenu, BeginUpdate, CarboniseAdapteStaticText, CarboniseGetDialogTextEditHandle,
    CarboniseGetMenuBarHeight, CarboniseInvalRect, CarboniseNewUserItemProc,
    CarboniseSetPortDialog, CarboniseValideUpdate, DisposeRoutineDescriptor, DragWindow,
    DrawPicture, EndUpdate, FindWindow, FrontWindow, GetCtlValue, GetDItem, GetDialogItem,
    GetNewDialog, GetNextEvent, GetPicture, GetResource, HLock, HUnlock, HiliteMenu,
    InitDefautConfigSortieNumerique, MenuKey, MenuSelect, OpenDeskAcc, PopPath, PushPath,
    ReleaseResource, SelectWindow, SetCtlValue, SetDialogDefaultItem, SetDialogItem, SizeWindow,
    SystemClick, SystemTask,
};
use crate::text::{
    DrawString, GetFontInfo, GetStdFont, StringWidth, TextFace, TextFont, TextSize, pstr_to_string,
};
use crate::{Sapiens, cbridge};
use cbridge::*;
use std::ffi::c_void;
use std::process::exit;

/******************** Constantes (SAPIENS.H / acam.h) *******************/
const VERS: i64 = 0x76657273; /* 'vers' */
const ALRT_VERSION: i16 = 133;
const noGrowDocProc: i16 = 0;

/******************* Constantes menus (SAPWIND.C) *******************/
const itemInformation: i16 = 1;
const itemAide: i16 = 2;

const itemNouveau: i16 = 1;
const itemCharger: i16 = 2;
const itemSauver: i16 = 3;
const itemPreferences: i16 = 5;
const itemQuitter: i16 = 7;

const iPaysage: i16 = 1;
const iLaterale: i16 = 2;
const iPanorama: i16 = 3;
const iCarte: i16 = 4;

const iEchelle100: i16 = 1;
const iEchelle200: i16 = 2;
const iEchelle300: i16 = 3;
const iEchelle400: i16 = 4;

/*************** Variables globales exposees au C (SAPWIND.C) ***************/

pub(crate) struct SapWind {
    pub(crate) SapZoom: f32,

    MenuReduit: bool,
    pub(crate) mainWindow: WindowPtr,
    pub(crate) Ecran1: CGrafPtr,
    pub(crate) Ecran2: CGrafPtr,
    pub(crate) debecr: CGrafPtr,
    pub(crate) BuffMouse: CGrafPtr,

    pub(crate) toucheClav: i16,
    pub(crate) mobBambou: i16,
    pub(crate) mobFace: i16,
    pub(crate) mobFeuille: i16,
    pub(crate) mobFleches: i16,
    pub(crate) mobSapmob: i16,
    pub(crate) mobAvplan: i16,
    pub(crate) mobAvplanSpec: i16,
    pub(crate) mobSilex: i16,
    pub(crate) mob3D: i16,
    pub(crate) Fonte14: i16,
    pub(crate) Fonte10: i16,
    OneString: [i8; 200],
}

impl Default for SapWind {
    fn default() -> Self {
        Self {
            SapZoom: 2.0,
            MenuReduit: false,
            mainWindow: core::ptr::null_mut(),
            Ecran1: core::ptr::null_mut(),
            Ecran2: core::ptr::null_mut(),
            debecr: core::ptr::null_mut(),
            BuffMouse: core::ptr::null_mut(),
            toucheClav: 0,
            mobBambou: 0,
            mobFace: 0,
            mobFeuille: 0,
            mobFleches: 0,
            mobSapmob: 0,
            mobAvplan: 0,
            mobAvplanSpec: 0,
            mobSilex: 0,
            mob3D: 0,
            Fonte14: 0,
            Fonte10: 0,
            OneString: [0; 200],
        }
    }
}

static TRANS_VIDE: [u8; 1] = [0];

/**************** HIRES: Passe en resolution *******************/
pub(crate) fn hires() -> i16 {
    1
}

impl SapWind {
    /// Change l'échelle d'affichage (menu « Échelle »).
    ///
    /// `SapZoom` reste le global C d'origine, mais c'est le port de la fenêtre qui
    /// porte l'échelle utilisée par le dessin : les deux sont mis à jour ensemble.
    pub(crate) fn set_zoom(&mut self, z: f32) {
        self.SapZoom = z;
        port::set_window_zoom(z);
    }

    /// Taille en pixels (ST 320x200) de l'aire de jeu multipliée par SapZoom.
    const fn aire_rect(&self) -> Rect {
        Rect::new(
            0,
            0,
            (320.0 * ECHELLE as f32 * self.SapZoom) as i16,
            (200.0 * ECHELLE as f32 * self.SapZoom) as i16,
        )
    }
}

/******************************************************************************************
                          GetVersionSapiens
Retourne la version du logiciel, ou une chaine vide si la ressource 'vers' est
absente. Structure des 'vers'
2 octets de version en hexa
3 octets de release
1 octet de pays
taille de la chaine courte suivi de la chaine courte
taille de la chaine longue suivi de la chaine longue
******************************************************************************************/
pub(crate) fn GetVersionSapiens() -> String {
    let ress: Handle = GetResource(VERS, 1);
    if ress.is_null() {
        return String::new();
    }
    HLock(ress);
    unsafe {
        let p: *mut u8 = *ress as *mut u8;
        /* On saute jusqu'à la short version string (chaine Pascal) */
        let s = pstr_to_string(p.add(6) as StringPtr);
        HUnlock(ress);
        ReleaseResource(ress);
        s
    }
}

/** Remplace `vsprintf` (sans varargs) : copie `s` dans le tampon C. */
fn copie_chaine(buf: &mut [i8], s: &str) {
    let n = s.len().min(buf.len() - 1);
    for (i, b) in s.as_bytes()[..n].iter().enumerate() {
        buf[i] = *b as i8;
    }
    buf[n] = 0;
}

impl Sapiens {
    /***********************************************************************************
                     Collapse
    Affichage d'un message d'erreur
    => Message à afficher
    <= rien
    ***********************************************************************************/
    pub(crate) fn Collapse(&self, message: &str) {
        let _g = port::preserve();
        if !self.sapwind.mainWindow.is_null() {
            port::set(self.sapwind.mainWindow);
        }

        /* On fixe la zone à la zone de secours de l'interface */
        let mut chaine: [i8; TAILLE_STR_MESS_FORMATE as usize] = [0; 512];
        copie_chaine(&mut chaine, message);
        CarboniseSetCursorArrow();
        CtoPstr(chaine.as_mut_ptr());
        ParamText(
            &raw mut chaine as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
        );
        self.DoAlertSapiens(CvtDefaultLangRess(134)); /* ALRT_COLLAPSE */
    }
}

/***********************************************************************************
                        MessInit
Affichage d'un message durant l'initialisation
***********************************************************************************/
pub(crate) fn MessInit(_format: *mut i8) {}

/***********************************************************************************
                                SetValBouton
=> Fixe une valeur VRAI ou FAUX dans un bouton
**********************************************************************************/
pub(crate) fn SetValBouton(dialog: DialogPtr, itemHit: i16, value: Boolean) -> i16 {
    let mut itemType = 0;
    let mut control: Handle = core::ptr::null_mut();
    let mut box_ = Rect::default();

    /* C'est un texte ??? */
    GetDItem(
        dialog,
        itemHit,
        &raw mut itemType,
        &raw mut control,
        &raw mut box_,
    );
    if (itemType & ctrlItem as i16) != 0 || (itemType & radCtrl as i16) != 0 {
        GetDItem(
            dialog,
            itemHit,
            &raw mut itemType,
            &raw mut control,
            &raw mut box_,
        );
        SetCtlValue(control as ControlHandle, if value { 1 } else { 0 });
        VRAI as i16
    } else {
        FAUX as i16
    }
}

/***********************************************************************************
                                GetValBouton
=>lit une valeur VRAI ou FAUX dans un bouton
**********************************************************************************/
pub(crate) fn GetValBouton(dialog: DialogPtr, itemHit: i16) -> i16 {
    let mut itemType: i16 = 0;
    let mut control: Handle = core::ptr::null_mut();
    let mut box_: Rect = Rect::default();
    let value: i16;

    /* C'est un bouton ??? */
    GetDItem(
        dialog,
        itemHit,
        &raw mut itemType,
        &raw mut control,
        &raw mut box_,
    );
    if (itemType & ctrlItem as i16) != 0 || (itemType & radCtrl as i16) != 0 {
        GetDItem(
            dialog,
            itemHit,
            &raw mut itemType,
            &raw mut control,
            &raw mut box_,
        );
        value = GetCtlValue(control as ControlHandle);
        return value;
    }
    FAUX as i16
}

/******************************************************************************************
                                    DefAfficheItem
 Défini le vecteur d'affichage d'un item de Type User
=> Dialog, N° de l'item, vecteur
******************************************************************************************/
pub(crate) fn DefAfficheItem(
    dialog: DialogPtr,
    item: i16,
    vect_affiche: Option<unsafe extern "C" fn(window: DialogPtr, itemid: i16)>,
) {
    let mut handle: Handle = core::ptr::null_mut();
    let mut rect: Rect = Rect::default();
    let mut itemType: i16 = 0;

    GetDialogItem(
        dialog,
        item,
        &raw mut itemType,
        &raw mut handle,
        &raw mut rect,
    );
    if (itemType & 127) == userItem as i16 {
        /* Deja un vecteur pour cet item ? */
        if !handle.is_null() {
            DisposeRoutineDescriptor(handle as *mut c_void);
        }
        let proc: *mut c_void = match vect_affiche {
            Some(f) => f as usize as *mut c_void,
            None => core::ptr::null_mut(),
        };
        SetDialogItem(
            dialog,
            item,
            itemType,
            CarboniseNewUserItemProc(proc) as Handle,
            &raw mut rect,
        );
    }
}

impl Sapiens {
    /******************************************************************************************
                                        DoAlertSapiens
    ******************************************************************************************/
    pub(crate) fn DoAlertSapiens(&self, num: i16) -> i16 {
        let n = Alert(num, core::ptr::null_mut());
        self.tfrvirt();
        self.TfrEcr(None);
        n
    }
}

/******************************************************************************************
                                    GetRectItem
=> Recupere le cadre d'un item donné
******************************************************************************************/
pub(crate) fn GetRectItem(dialog: DialogPtr, item: i16, box_: *mut Rect) {
    let mut itemType: i16 = 0;
    let mut handle: Handle = core::ptr::null_mut();

    /* On récupère l'item coorespondant à l'aire (seule l'aire nous intéresse) */
    GetDItem(dialog, item, &raw mut itemType, &raw mut handle, box_);
}

/*****************************************************************************
                       DrawArrowPopUp
****************************************************************************/
pub(crate) fn DrawArrowPopUp(r: &mut Rect) {
    let y = (r.bottom + r.top) / 2 + 3;
    let box_ = Rect::new(r.right - 15, r.top + 1, r.right - 1, r.bottom - 1);
    DoEraseRect(&box_);
    for i in 0..6 {
        DoMoveTo(r.right - 10 - i, y - i);
        DoLine(i * 2, 0);
    }
}

impl SapWind {
    /******************************************************************************************
                                        GetStr
    Recupere une chaine indexée dans une locale et la passe au format C
    <= Ptr sur la locale
    ******************************************************************************************/
    pub(crate) fn GetStr(&mut self, index: i16) -> *mut i8 {
        let s = &raw mut self.OneString as *mut i8;
        GetIndString(
            s as *mut c_void as StringPtr,
            CvtDefaultLangRess(STR_VRAC as i16),
            index,
        );
        PtoCstr(s as *mut u8);
        s
    }
}

impl Random {
    /**********************************************************************
                        initrnd
    ***********************************************************************/
    pub(crate) fn initrnd(&mut self, sem1: i16, sem2: i16, sem3: i16) {
        self.CarboniseSetRandSeed(((sem1 as i64) << 16) + ((sem2 as i64) << 8) + sem3 as i64);
    }

    /**********************************************************************
                        rnd0
    ***********************************************************************/
    pub(crate) fn rnd0(&mut self) -> i16 {
        (self.Random() & 0xFF) as i16
    }

    /**********************************************************************
                        Rnd
    ***********************************************************************/
    pub(crate) fn Rnd(&mut self) -> i16 {
        (self.Random() & 0xFF) as i16
    }

    /**********************************************************************
                        myRandom
    Sort une valeur entre deux bornes
    ***********************************************************************/
    pub(crate) fn myRandom(&mut self, min: i64, max: i64) -> i64 {
        let mut rdm = (self.Rnd() as i64) as u16;
        rdm = rdm.wrapping_add(((self.Rnd() as i64) * 256) as u16);
        let range = max - min;
        let t = ((rdm as i64) * range) / 65536;
        min + t
    }
}
/**********************************************************************
                    bande
Affiche une bande sur les cotés
***********************************************************************/
/*;********* BANDE_: 0= bande jeu, 1= bande silex *******************/
pub(crate) fn bande(_quelle: i16) {}

impl Sapiens {
    /*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                            SAPDISK.S
    +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    /****************************************************************************
    Boxf: trace une boite sur le port spécifié
    Les  coordonnées sont données en ecran ST 320x200
    (C) Ligne dep,Col dep,Ligne fin,col.fin,index couleur ST
    ***************************************************************************/
    pub(crate) fn boxf(&self, lignedep: i16, coldep: i16, lignefin: i16, colfin: i16, index: i16) {
        let mut aire = Rect::new(
            coldep * ECHELLE as i16,
            lignedep * ECHELLE as i16,
            colfin * ECHELLE as i16,
            lignefin * ECHELLE as i16,
        );
        self.BoxFill(
            port::current(),
            &mut aire,
            self.sapglob.palhaute.borrow()[index as usize],
        );
    }
}

/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
               ECRAN64.S
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
/*;------------------------------------------------------
;               CADREG:Trace un cadre graphique en pointille
;-------------------------------------------------------
;=> numero ligne pixel deut *2,col octet,n ligne/2,n col
;<= sur ecran cadre xor
*/
pub(crate) fn cadreg(_ligne: i16, _col: i16, _nligne: i16, _ncol: i16) {}

impl Sapiens {
    /******************************************************************************************
                                    AdjustMenus
    ******************************************************************************************/
    pub(crate) fn AdjustMenus(&self) {
        let mut menu: MenuHandle;

        let _g = port::on(self.sapwind.mainWindow);

        menu = GetMHandle(CvtDefaultLangRess(128)); /* menuPomme */
        menu = GetMHandle(CvtDefaultLangRess(129)); /* menuFichier */
        if self.sapwind.MenuReduit {
            DisableItem(menu, 0);
        } else {
            EnableItem(menu, 0);
        }

        menu = GetMHandle(CvtDefaultLangRess(130)); /* menuVoir */
        if self.sapwind.MenuReduit || self.sapglob.mort == 0 {
            DisableItem(menu, 0);
        } else {
            EnableItem(menu, 0);
        }
        menu = GetMHandle(CvtDefaultLangRess(131)); /* menuFenetre */
        CheckItem(menu, iEchelle100, self.sapwind.SapZoom == 1.0);
        CheckItem(menu, iEchelle200, self.sapwind.SapZoom == 2.0);
        CheckItem(menu, iEchelle300, self.sapwind.SapZoom == 3.0);
        CheckItem(menu, iEchelle400, self.sapwind.SapZoom == 4.0);
        DrawMenuBar();
    }

    /******************************************************************************************
                                    FixeMenusReduit
    ******************************************************************************************/
    pub(crate) fn FixeMenusReduit(&mut self) {
        if !self.sapwind.MenuReduit {
            self.sapwind.MenuReduit = VRAI;
            self.AdjustMenus();
        }
    }

    /******************************************************************************************
                                    FixeMenusComplet
    ******************************************************************************************/
    pub(crate) fn FixeMenusComplet(&mut self) {
        if self.sapwind.MenuReduit {
            self.sapwind.MenuReduit = FAUX;
            self.AdjustMenus();
        }
    }

    /******************************************************************************************
                                    ConfirmePerteAventure
    ******************************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn ConfirmePerteAventure(&mut self) -> Boolean {
        self.souris.MontreSouris();
        self.DoAlertSapiens(CvtDefaultLangRess(1)) == 1 /* ALRT_PERTE_PARTIE */
    }

    /******************************************************************************************
                                    DoInfo
    ******************************************************************************************/
    pub(crate) fn DoInfo(&self) {
        let mut str_: [i8; 255] = [0; 255];
        copie_chaine(&mut str_, &GetVersionSapiens());
        CtoPstr(str_.as_mut_ptr());
        ParamText(
            &raw mut str_ as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
        );
        self.DoAlertSapiens(CvtDefaultLangRess(2)); /* ALRT_INFO */
    }

    /******************************************************************************************
                                    DoMenuCommand
    *******************************************************************************************/

    pub(crate) fn DoMenuCommand(&mut self, menuResult: i32) {
        let mut daName: [u8; 256] = [0; 256];

        self.AdjustMenus();
        self.souris.MontreSouris();
        let menuID = ((menuResult >> 16) & 0xFFFF) as i16; /* HiWord */
        let menuItem = (menuResult & 0xFFFF) as i16; /* LoWord */
        if menuID == CvtDefaultLangRess(128) {
            /* menuPomme */
            match menuItem {
                itemInformation => self.DoInfo(),
                itemAide => self.DoAide(),
                _ => {
                    GetItem(
                        GetMHandle(CvtDefaultLangRess(128)),
                        menuItem,
                        &raw mut daName as *mut u8,
                    );
                    OpenDeskAcc(&raw mut daName as *mut c_void as StringPtr);
                }
            }
        } else if menuID == CvtDefaultLangRess(129) {
            /* menuFichier */
            match menuItem {
                itemNouveau => self.trgest(3),
                itemCharger => {
                    self.trgest(1);
                    self.inimen(0);
                }
                itemSauver => self.trgest(2),
                itemPreferences => self.DoPreferences(),
                itemQuitter if self.ConfirmePerteAventure() => {
                    SapiensExit(0);
                }
                _ => {}
            }
        } else if menuID == CvtDefaultLangRess(130) {
            /* menuVoir */
            match menuItem {
                iPaysage => {
                    self.trobsv(0);
                    if self.sapglob.flcomb != 0 {
                        self.TraiteVueLaterale();
                    }
                }
                iLaterale => {
                    self.trobsv(1);
                    self.TraiteVueLaterale();
                }
                iPanorama => self.trobsv(2),
                iCarte => self.trobsv(3),
                _ => {}
            }
        } else if menuID == CvtDefaultLangRess(131) {
            /* menuFenetre */
            match menuItem {
                iEchelle100 => self.sapwind.set_zoom(1.0),
                iEchelle200 => self.sapwind.set_zoom(2.0),
                iEchelle300 => self.sapwind.set_zoom(3.0),
                iEchelle400 => self.sapwind.set_zoom(4.0),
                _ => {}
            }
            self.sapwind.FixeTailleMainWindow();
            self.RedessineFenetre();
        }
        HiliteMenu(0);
        self.AdjustMenus();
        DrawMenuBar();
    }

    /*******************************************************************
                            DoEvtClavier
    Gestion du clavier
    => evenement a traiter
    ********************************************************************/
    pub(crate) fn DoEvtClavier(&mut self, ev: &EventRecord) {
        if ev.modifiers & cmdKey as i16 != 0 {
            let _car = (ev.message & 0xFF) as i16;
            self.AdjustMenus();
            self.DoMenuCommand(MenuKey((ev.message & charCodeMask as i64) as i16) as i32);
        } else if ev.modifiers & optionKey as i16 != 0 {
            let mut i: i16;
            let car: i16 = ((ev.message & 0xFF00) >> 8) as i16;
            if car == 0x11
            /*T*/
            {
                /* Transfert Ecran */
                let t = TickCount();
                i = 0;
                while i < 200 {
                    self.TfrEcr(None);
                    i += 1;
                }
                self.Collapse(&format!(
                    "{i} transferts totaux de l'ecran : {} ticks",
                    TickCount() - t
                ));
            }
            if car == 0x02
            /*'D'*/
            {
                /* 3d */
                let t = TickCount();
                i = 0;
                while i < 50 {
                    self.affich(self.sapglob.dzeta as u16);
                    i += 1;
                }
                self.Collapse(&format!("{i} images 3d : {} ticks", TickCount() - t));
            }
            if car == 0x23 {
                /*'P'*/
                self.AffPalette();
            }
            if car == 0x29 {
                /*'M'*/
                if self.sapwmus.quelle_musique == 1 {
                    self.MusicInit(2);
                } else {
                    self.MusicInit(1);
                }
            }
            if car == 0x06 {
                /*'T'*/
                i = 0;
                while i < 64 {
                    self.chrono(1);
                    i += 1;
                }
            }
            if car == 0x0f {
                /*R*/
                self.InitMobs();
            }
        } else {
            let car: i16 = (ev.message & charCodeMask as i64) as i16;
            /* Touches de fonction */
            if car == 0x10 {
                self.DoAide();
            }
        }
    }
}

/*******************************************************************
                        CalcIndexPalette
Calcule la deuxieme section de la palette ST en entrée :
index de la couleur dans la palette systeme
********************************************************************/
fn CalcIndexPalette(palette: &mut [i16]) {
    for i in 0..16 {
        let rgb = RGBColor::CvtCouleurStEnRgb(palette[i]);
        palette[16 + i] = SapColor2Index(&rgb);
    }
}

/*******************************************************************
                        FixeMenu
=> Numero de la menu Bar à installer, Numero du menu Pomme de cette menuBar
********************************************************************/
pub(crate) fn FixeMenu(n: i16) -> Boolean {
    let menuBar: Handle = GetNewMBar(n); /* read menus into menu bar */
    if menuBar.is_null() {
        return FAUX;
    }
    SetMenuBar(menuBar); /* install menus */
    DisposHandle(menuBar);
    let menu: MenuHandle = GetMHandle(CvtDefaultLangRess(128)); /* menuPomme */
    if !menu.is_null() {
        AddResMenu(menu, DRVR as i64); /* add DA names to Apple menu */
    }
    DrawMenuBar();
    VRAI
}

impl Sapiens {
    /******************************************************************
                        InitMobs
    Charge ou recharge les mobs
    ******************************************************************/
    pub(crate) fn InitMobs(&mut self) -> Boolean {
        let mut nb: i16;

        let _g = port::on(self.sapwind.Ecran1);

        self.LibereDataMobs();
        PushPath();
        SetPathInData(&self.sapgest, c"Mobs".as_ptr() as *mut i8);
        self.sapwind.mobBambou = 0;
        nb = self.ChargeFichierBrt("WBAMBOU.BRT", self.sapwind.mobBambou + 1, 26, 0); /*0, 21 Mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobFace = self.sapwind.mobBambou + nb;
        nb = self.ChargeFichierBrt("WFACE.BRT", self.sapwind.mobFace + 1, 31, 0); /* Base 21,29 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobFeuille = self.sapwind.mobFace + nb;
        nb = self.ChargeFichierBrt("WFEUILLE.BRT", self.sapwind.mobFeuille + 1, 36, 11); /* Base 50 36 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobFleches = self.sapwind.mobFeuille + nb;
        nb = self.ChargeFichierBrt("WFLECHES.BRT", self.sapwind.mobFleches + 1, 20, 0); /* Base 8618 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobSapmob = self.sapwind.mobFleches + nb;
        nb = self.ChargeFichierBrt("WSAPMOB.BRT", self.sapwind.mobSapmob + 1, 141, 0); /*Base 120+1,151 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobAvplan = self.sapwind.mobSapmob + nb;
        nb = self.ChargeFichierBrt("WAVPLAN.BRT", self.sapwind.mobAvplan + 1, 9, 13);
        self.sapcomb.nbMobHerbes = nb;
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobAvplanSpec = self.sapwind.mobAvplan + nb;
        nb = self.ChargeFichierBrt("WAVSPEC.BRT", self.sapwind.mobAvplanSpec + 1, 4, 0);
        self.sapcomb.nbAvplanSpec = nb;
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobSilex = self.sapwind.mobAvplanSpec + nb;
        nb = self.ChargeFichierBrt("WSILEX.BRT", self.sapwind.mobSilex + 1, 15, 0);
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mob3D = self.sapwind.mobSilex + nb;
        nb = self.ChargeFichierBrt("W3D.BRT", self.sapwind.mob3D + 1, 38, 0);
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.Fonte14 = self.sapwind.mob3D + nb;
        nb = self.ChargeFichierBrt("Geneva14.BRT", self.sapwind.Fonte14 + 1, 120, 0);
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.Fonte10 = self.sapwind.Fonte14 + nb;
        nb = self.ChargeFichierBrt("Geneva10.BRT", self.sapwind.Fonte10 + 1, 120, 0);
        if nb < 0 {
            return FAUX;
        }
        PopPath();
        VRAI
    }
}

/***********************************************************************************
                                 AfficheFond
Affiche le tramage de fond de la fenetre
***********************************************************************************/
unsafe fn AfficheFond(gfx: CGrafPtr) {
    let _g = port::on(gfx);
    let portRect = port::bounds(gfx);
    /* Le fond au dessus de l'awale */
    let picMotif: PicHandle = GetPicture(128) as PicHandle;
    let mut y: i16 = 0;
    while y < portRect.bottom {
        let mut x: i16 = 0;
        while x < portRect.right + 16 {
            BackColor(whiteColor as i16);
            ForeColor(blackColor as i16);
            let r = Rect::new(x, y, x + 64, y + 64);
            DrawPicture(picMotif, &r);
            x += 64;
        }
        y += 64;
    }
    ReleaseResource(picMotif as Handle);
}

impl Sapiens {
    /***********************************************************************************
                            RedessineFenetre
    Repeint toute la fenêtre principale depuis le décor (debecr) et, en vue
    latérale, depuis l'écran virtuel (Ecran1). Utilisé quand la zone visible
    change de taille (menu « Échelle ») ou après une mise à jour système.
    ***********************************************************************************/
    pub(crate) fn RedessineFenetre(&self) {
        port::set(self.sapwind.mainWindow);
        self.TfrEcr(None);
        self.tfrima();
        if self.sapglob.dervue == Vue::LATERALE {
            self.tfrdemi();
        }
    }

    /***********************************************************************************
                                     FixeTitreMainWindow
    ***********************************************************************************/
    pub(crate) fn FixeTitreMainWindow(&self) {
        port::set(self.sapwind.mainWindow);
        SetWTitle(
            self.sapwind.mainWindow,
            format!("Sapiens {}", GetVersionSapiens()),
        );
    }

    /****************************************************************************
                Encadre l'aire 3d
    => etat 1 actif
    *****************************************************************************/
    pub(crate) fn Frame3d(&self, etat: i16) {
        let _g = port::on(self.sapwind.debecr);
        let r = Rect::new(62, 0, 640 - 64, 204);
        if etat != 0 {
            ForeColor(blackColor as i16);
        } else {
            ForeColor(whiteColor as i16);
        }
        DoPenSize(2, 2);
        DoFrameRect(&r);
        ForeColor(blackColor as i16);
        PenNormal();
        port::set(self.sapwind.mainWindow);
        if etat != 0 {
            ForeColor(blackColor as i16);
        } else {
            ForeColor(whiteColor as i16);
        }
        DoPenSize(2, 2);
        DoFrameRect(&r);
        PenNormal();
        ForeColor(blackColor as i16);
    }

    /**********************************************************************************
                        MenuFenetres
    Entre ou sort du menu fenetre actif
    => etat =1 si actif
    ***********************************************************************************/
    pub(crate) fn MenuFenetres(&mut self, etat: i16) {
        self.Frame3d(etat);
        self.tfrmenufenetre(etat);
        if etat != 0 {
            self.FixeMenusComplet();
        } else {
            self.FixeMenusReduit();
        }
        self.sapglob.dicone = -1;
    }
}

/**********************************************************************************************
                                            ValideUpdate
***********************************************************************************************/
pub(crate) fn ValideUpdate() {
    CarboniseValideUpdate();
}

impl SapWind {
    /**********************************************************************************************
                                                                    AffStr
    ***********************************************************************************************/
    unsafe fn AffStr(&self, chaine: *mut i8, fonte: i16, size: i16, face: i16, x_in: i16, y: i16) {
        let mut x: i16 = x_in;

        CtoPstr(chaine);
        TextFont(fonte);
        TextSize(size);
        TextFace(face);
        if x < 0 {
            x = (640 - StringWidth(chaine as *mut u8)) / 2;
        }
        ForeColor(blackColor as i16);
        DoTextSize(size);
        DoMoveTo(x + 1, y + 1);
        DrawString(chaine as *mut u8);
        ForeColor(whiteColor as i16);
        DoMoveTo(x, y);
        DrawString(chaine as *mut u8);
        DoMoveTo(x - 1, y);
        DrawString(chaine as *mut u8);
        ForeColor(blackColor as i16);
        PtoCstr(chaine as *mut u8);
    }
}

/******************************************************************************************
                                SetFontDialog
Fixe la fonte du dialogue en entrée
******************************************************************************************/
unsafe fn SetFontDialog(dialog: DialogPtr, font: i16, taille: i16) {
    CarboniseSetPortDialog(dialog);
    TextFont(font);
    TextSize(taille);
    let info = GetFontInfo().unwrap_or_default();
    let te: TEHandle = CarboniseGetDialogTextEditHandle(dialog);
    unsafe {
        if !te.is_null() {
            (**te).txFont = font;
            (**te).txSize = taille;
            (**te).lineHeight = info.descent + info.ascent + info.leading;
            (**te).fontAscent = info.ascent;
        }
    }
}

/*******************************************************************
                        OuvreDialogue
********************************************************************/
pub(crate) fn OuvreDialogue(numDitl: i16) -> DialogPtr {
    unsafe {
        let dialog: DialogPtr = GetNewDialog(numDitl, core::ptr::null_mut(), -1isize as WindowPtr);
        if !dialog.is_null() {
            SetFontDialog(dialog, geneva as i16, 9);
            CarboniseAdapteStaticText(dialog, numDitl, GetStdFont(), 9);
            SetDialogDefaultItem(dialog, 1);
        }
        dialog
    }
}

impl SapWind {
    /*******************************************************************
                            InitMainWindow
    ********************************************************************/
    pub(crate) fn InitMainWindow(&mut self) {
        let largeur = (320.0 * ECHELLE as f32 * self.SapZoom) as i64;
        let hauteur = (200.0 * ECHELLE as f32 * self.SapZoom) as i64;

        let aireDevice = port::SCREEN;
        /* Creation de la fenetre principale */
        /* Aire de la fenetre */
        let mut rect = Rect::new(
            0,
            0,
            (320.0 * ECHELLE as f32 * self.SapZoom) as i16,
            (200.0 * ECHELLE as f32 * self.SapZoom) as i16,
        );
        /* On la centre sur l'aire generale de l'ecran */
        rect.OffsetRect(
            aireDevice.left + (aireDevice.right - aireDevice.left - largeur as i16) / 2,
            aireDevice.top
                + (CarboniseGetMenuBarHeight() + 8)
                + (aireDevice.bottom
                    - aireDevice.top
                    - CarboniseGetMenuBarHeight()
                    - hauteur as i16)
                    / 2,
        );
        /* On crée la fenetre principale */
        self.mainWindow = NewCWindow(
            core::ptr::null_mut(),
            &raw const rect,
            core::ptr::null_mut(),
            VRAI,
            noGrowDocProc,
            -1isize as WindowPtr,
            FAUX,
            0,
        );
        /* La fenêtre porte l'échelle courante (menu « Échelle »). */
        port::set_window_zoom(self.SapZoom);
    }
}

impl Sapiens {
    /*******************************************************************
                            init_graph
    ********************************************************************/
    pub(crate) fn init_graph(&mut self) -> Boolean {
        let mut i: i16;
        let mut y: i16;
        let mut r: Rect = Rect::default();
        let mut chaine: [i8; 255] = [0; 255];

        /* Init des chemins standards */
        self.InitCheminsSapiens();
        SetCursor(GetCursor(watchCursor as i16));
        self.FixeAireNoMouseAireBasse();
        /* On charge les preferences */
        let firstInit = self.ChargePref();
        InitDefautConfigSortieNumerique();
        self.MusicInit(1); /* Lance musique 1 */
        /* Choix de la langue*/
        if langue_i16() == -1 {
            CarboniseSetCursorArrow();
            set_langue((Alert(ALRT_VERSION, core::ptr::null_mut()) - 1).into());
            SetCursor(GetCursor(watchCursor as i16));
        }
        unsafe {
            self.sapglob.init_and_check_texts();
            self.sapwind.InitMainWindow();
            self.FixeTitreMainWindow();
            port::set(self.sapwind.mainWindow);
            AfficheFond(self.sapwind.mainWindow);
            /* On affiche le logo centre */
            let pict = GetPicture(134) as PicHandle;
            if !pict.is_null() {
                r = (**pict).picFrame;
                r.OffsetRect(
                    ((320 * ECHELLE as i16 as i64 - (r.right - r.left) as i64) / 2) as i16,
                    60,
                );
                let rdest = DoZoomRect(&r);
                DrawPicture(pict, &rdest);
                ReleaseResource(pict as Handle);
            }
            /* Affichage de la version de sapiens */
            copie_chaine(&mut chaine, &GetVersionSapiens());
            self.sapwind.AffStr(
                &raw mut chaine as *mut i8,
                helvetica as i16,
                14,
                bold as i16,
                -1,
                r.bottom + 16,
            );
            /* Affichage des autres commentaires */
            strcpy(
                &raw mut chaine as *mut i8,
                c"Copyright Didier & Olivier Guillion, 1986,1996,2005,2017".as_ptr() as *const i8,
            );
            self.sapwind.AffStr(
                &raw mut chaine as *mut i8,
                helvetica as i16,
                16,
                bold as i16,
                -1,
                r.bottom + 32,
            );
            i = 1;
            y = 60 + 120 + 18 + 32;
            let num = CvtDefaultLangRess(1);
            loop {
                GetIndString(
                    &raw mut chaine as *mut u8 as *mut c_void as StringPtr,
                    num,
                    i,
                );
                i += 1;
                PtoCstr(&raw mut chaine as *mut u8);
                if strlen(&raw const chaine as *const i8) != 0 {
                    self.sapwind.AffStr(
                        &raw mut chaine as *mut i8,
                        helvetica as i16,
                        16,
                        bold as i16,
                        -1,
                        y,
                    );
                }
                y += 18;
                if strlen(&raw const chaine as *const i8) == 0 {
                    break;
                }
            }
            let timeBase = TickCount() + 5 * 60;

            r = self.sapwind.aire_rect();
            CarboniseInvalRect(&raw const r);
            ValideUpdate();
            /* On initialise le menu */
            MessInit(self.sapwind.GetStr(26)); /* Initialisation du menu */
            if !FixeMenu(CvtDefaultLangRess(0)) {
                /* MENU_BAR */
                let x = self.sapwind.GetStr(11);
                self.Collapse(&std::ffi::CStr::from_ptr(x).to_string_lossy());
                SapiensExit(-1);
            }
            self.FixeMenusReduit();
            port::set(self.sapwind.mainWindow);
            MessInit(self.sapwind.GetStr(20)); /* Initialsations des ecrans */
            CalcIndexPalette(self.saplist.pal1.borrow_mut().as_mut());
            CalcIndexPalette(self.saplist.pal2.borrow_mut().as_mut());
            self.sapwind.Ecran1 = SapiensCreeOffscreen(256 * ECHELLE as i16, 100 * ECHELLE as i16);
            if self.sapwind.Ecran1.is_null() {
                self.Collapse("[SapWind.init_graph.1] Unable to create offscreen");
                SapiensExit(-2);
            }
            self.sapwind.Ecran2 = SapiensCreeOffscreen(256 * ECHELLE as i16, 100 * ECHELLE as i16);
            port::set(self.sapwind.Ecran1);
            MessInit(self.sapwind.GetStr(21)); /* Initialsations des palettes */
            /* Fabrique les luts */
            self.sapwcvt.FabriqueLuts();
            self.sapwind.debecr = SapiensCreeOffscreen(320 * ECHELLE as i16, 200 * ECHELLE as i16);
            port::set(self.sapwind.debecr);
            AfficheFond(self.sapwind.debecr);
            /* Buffer de la souris*/
            self.sapwind.BuffMouse = SapiensCreeOffscreen(64 * ECHELLE as i16, 64 * ECHELLE as i16);
            MessInit(self.sapwind.GetStr(22)); /*Chargement des graphismes */
            /* On charge les mobs */
            if !self.InitMobs() {
                let x = self.sapwind.GetStr(11);
                self.Collapse(&std::ffi::CStr::from_ptr(x).to_string_lossy());
                SapiensExit(-1);
            }
            self.InitCouleurFontes();
            MessInit(self.sapwind.GetStr(23)); /*Chargement des dégradés */
            /* On calcule tout les degrades*/
            self.CalculeToutDegrades();
            MessInit(self.sapwind.GetStr(24)); /*Chargement des décors trois 3 */
            self.CalculeEtoiles();
            MessInit(self.sapwind.GetStr(25)); /* Initialisation de la musique */
            /* Initialisation de la musique */
            while timeBase > TickCount() {
                core::hint::spin_loop();
            }
            /* Transfert de l'ecran */
            CarboniseSetCursorArrow();
            firstInit
        }
    }

    /*******************************************************************
                            GereInterface
    Gestion générale de l'interface
    ********************************************************************/
    pub(crate) fn GereInterface(&mut self) {
        let mut part;
        let mut windowResult = WindowPtr::default();
        let mut event = EventRecord::default();

        /* Commande de menu venue de la barre egui : elle s'execute ici, sur le
        thread C, la ou vit le `&mut Sapiens`. Le swap a ete fait, donc un modal
        qui repompe GereInterface ne la relira pas. */
        if let Some(cmd) = take_menu_command() {
            self.DoMenuCommand(cmd);
        }

        let _g = port::on(self.sapwind.mainWindow);
        let window = FrontWindow();
        self.CheckChangeMusique();
        while GetNextEvent(everyEvent as i16, &mut event) {
            match event.what as u32 {
                keyDown => {
                    self.sapwind.toucheClav = event.message as i16 & charCodeMask as i16;
                    self.DoEvtClavier(&event);
                }
                mouseDown => {
                    /* On cherche quelle fenetre a été cliquée */
                    part = FindWindow(event.where_, &mut windowResult);
                    /* Où a-t'on cliqué ??? */
                    match part as u32 {
                        /* Dans la barre de menu , alors on le traite */
                        inMenuBar => {
                            self.DoMenuCommand(MenuSelect(event.where_));
                        }
                        /* Sur le bureau, alors le bureau se débrouille ... sauf si c'est un pseudo document */
                        inSysWindow => {
                            SystemClick(&event, windowResult);
                        }
                        /* Dans le deplacement */
                        inDrag => {
                            SelectWindow(windowResult);
                            let bounds = port::SCREEN;
                            DragWindow(windowResult, event.where_, &bounds);
                        }
                        /* Dans une fenetre : alors on le traite */
                        inContent if windowResult != window => {
                            SelectWindow(windowResult);

                            //if(window==mainWindow)
                            //       DoClick(event.where);
                        }
                        _ => {}
                    }
                }
                updateEvt => {
                    if port::is_window(window) {
                        //                    AcamGetPortOffscreen(&gbak);
                        port::set(window);
                        BeginUpdate(window);
                        self.RedessineFenetre();
                        EndUpdate(window);
                        //                    AcamSetPortOffscreen(gbak);
                    }
                }
                activateEvt => {
                    if (event.message & 1) > 0 {
                        /* on reprends le traitement */
                    } else {
                        /* On est arrêté */
                        self.souris.MontreSouris();
                    }
                }
                app4Evt if (event.message >> 24) == 1 => {
                    if (event.message & 1) > 0 {
                        /* on reprends le traitement */
                        self.RedessineFenetre();
                    } else {
                        /* On est arrêté */
                        self.souris.MontreSouris();
                    }
                }
                _ => {}
            }
        }
        SystemTask();
    }

    /*******************************************************************
                            delai
    => Nbre de 60eme de seocndes a atteindre
    ********************************************************************/
    pub(crate) fn delai(&mut self, duree: i64) {
        let t = TickCount() + duree;

        loop {
            self.GereInterface();
            if t <= TickCount() {
                break;
            }
        }
    }
}

impl SapWind {
    /*******************************************************************
                            FixeTailleMainWindow
    ********************************************************************/
    pub(crate) fn FixeTailleMainWindow(&self) {
        let _g = port::on(self.mainWindow);
        let right = (320.0 * ECHELLE as f32 * self.SapZoom) as i16;
        let bottom = (200.0 * ECHELLE as f32 * self.SapZoom) as i16;
        SizeWindow(self.mainWindow, right, bottom, true);
    }
}

/*******************************************************************
                        SapiensExit
********************************************************************/
pub(crate) fn SapiensExit(code: i32) {
    audio::stop_music();
    exit(code);
}
