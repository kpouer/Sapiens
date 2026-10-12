#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/*;-----------------------------------------------------------------
;               HIRES;Passe en resolution et detourne Erreurs Disques
;-----------------------------------------------------------------------
*/
use std::ffi::c_void;
use std::os::raw::{c_char, c_long, c_short, c_ushort};
use std::process::exit;

use crate::Sapiens;
use crate::acam::{
    BackColor, BlockMove, CheckItem, CtoPstr, DisableItem, DisposHandle, DrawMenuBar, EnableItem,
    FAUX, ForeColor, GetCursor, GetIndString, GetItem, GetMHandle, GetNewMBar, MaxMem, NewCWindow,
    PenNormal, PtoCstr, Random, SetMenuBar, SetRect, TickCount, VRAI,
};
use crate::cbridge::{
    DRVR, DialogPtr, MenuHandle, WindowPtr, blackColor, bold, charCodeMask, cmdKey, ctrlItem,
    editText, everyEvent, geneva, helvetica, itemDisable, optionKey, radCtrl, statText, strcat,
    strcpy, strlen, userItem, watchCursor, whiteColor,
};
use crate::exchange::take_menu_command;
use crate::port;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::saptxt::{CvtDefaultLangRess, langue, set_langue};
use crate::sapiens::sapwbrt::{
    DoEraseRect, DoFrameRect, DoLine, DoMoveTo, DoPenSize, DoTextSize, DoZoomRect,
    SapiensCreeOffscreen, SetPathInData,
};
use crate::sapiens::sapwcvt::Sapwcvt;
use crate::sapiens::{ECHELLE, STR_VRAC, TAILLE_STR_MESS_FORMATE, Vue};
use crate::shared::{
    Boolean, CGrafPtr, ControlHandle, EventRecord, Handle, PicHandle, RGBColor, Rect, StringPtr,
    TEHandle,
};
use crate::stubs::{
    AddResMenu, Alert, ArreteMusique, BeginUpdate, CarboniseAdapteStaticText,
    CarboniseGetDialogTextEditHandle, CarboniseGetMenuBarHeight, CarboniseInvalRect,
    CarboniseNewUserItemProc, CarboniseSetCursorArrow, CarboniseSetPortDialog,
    CarboniseValideUpdate, DisposeRoutineDescriptor, DragWindow, DrawPicture, EndUpdate,
    FindWindow, FrontWindow, GetCtlValue, GetDItem, GetDialogItem, GetIText, GetNewDialog,
    GetNextEvent, GetPicture, GetResource, HLock, HUnlock, HiliteControl, HiliteMenu,
    InitDefautConfigSortieNumerique, MenuKey, MenuSelect, OpenDeskAcc, ParamText, PopPath,
    PushPath, ReleaseResource, SelectWindow, SetCTitle, SetCtlValue, SetCursor, SetDItem,
    SetDialogDefaultItem, SetDialogItem, SetIText, SetWTitle, SizeWindow, SystemClick, SystemTask,
};
use crate::text::{DrawString, GetFontInfo, GetStdFont, StringWidth, TextFace, TextFont, TextSize};

/******************** Constantes (SAPIENS.H / acam.h) *******************/
const VERS: c_long = 0x76657273; /* 'vers' */
const ALRT_VERSION: c_short = 133;
const noGrowDocProc: c_short = 0;

/******************* Constantes menus (SAPWIND.C) *******************/
const itemInformation: c_short = 1;
const itemAide: c_short = 2;

const itemNouveau: c_short = 1;
const itemCharger: c_short = 2;
const itemSauver: c_short = 3;
const itemPreferences: c_short = 5;
const itemQuitter: c_short = 7;

const iPaysage: c_short = 1;
const iLaterale: c_short = 2;
const iPanorama: c_short = 3;
const iCarte: c_short = 4;

const iEchelle100: c_short = 1;
const iEchelle200: c_short = 2;
const iEchelle300: c_short = 3;
const iEchelle400: c_short = 4;

/*************** Variables globales exposees au C (SAPWIND.C) ***************/

pub(crate) struct SapWind {
    pub(crate) SapZoom: f32,

    MenuReduit: bool,
    pub(crate) mainWindow: WindowPtr,
    pub(crate) Ecran1: CGrafPtr,
    pub(crate) Ecran2: CGrafPtr,
    pub(crate) debecr: CGrafPtr,
    pub(crate) BuffMouse: CGrafPtr,

    pub(crate) toucheClav: c_short,
    pub(crate) mobBambou: c_short,
    pub(crate) mobFace: c_short,
    pub(crate) mobFeuille: c_short,
    pub(crate) mobFleches: c_short,
    pub(crate) mobSapmob: c_short,
    pub(crate) mobAvplan: c_short,
    pub(crate) mobAvplanSpec: c_short,
    pub(crate) mobSilex: c_short,
    pub(crate) mob3D: c_short,
    pub(crate) Fonte14: c_short,
    pub(crate) Fonte10: c_short,
    OneString: [c_char; 200],
}

impl Default for SapWind {
    fn default() -> Self {
        Self {
            SapZoom: 1.0,
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
    fn aire_rect(&self, r: *mut Rect) {
        SetRect(
            r,
            0,
            0,
            (320.0 * ECHELLE as f32 * self.SapZoom) as c_short,
            (200.0 * ECHELLE as f32 * self.SapZoom) as c_short,
        );
    }
}

/******************************************************************************************
                          GetVersionSapiens
Affiche la version du logiciel dans l'user item en entrée. Le texte est centré
Structure des 'vers'
2 octets de version en hexa
3 octets de release
1 octet de pays
taille de la chaine courte suivi de la chaine courte
taille de la chaine longue suivi de la chaine longue
******************************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn GetVersionSapiens(chaine: *mut c_char) -> Boolean {
    unsafe {
        chaine.write(0);
        let ress: Handle = GetResource(VERS, 1);
        if ress.is_null() {
            return FAUX;
        }
        HLock(ress);
        let p: *mut u8 = *ress as *mut u8;
        /* On recherche le pointeur sur la chaine longue de la version */
        let p = p.add(6); /* On saute jusqu'a la short version string */
        BlockMove(p.add(1) as *const c_void, chaine as *mut c_void, *p as i64);
        chaine.add(*p as usize).write(0);
        HUnlock(ress);
        ReleaseResource(ress);
        VRAI
    }
}

/** Remplace `vsprintf` (sans varargs) : copie `s` dans le tampon C. */
fn copie_chaine(buf: &mut [c_char], s: &str) {
    let n = s.len().min(buf.len() - 1);
    for (i, b) in s.as_bytes()[..n].iter().enumerate() {
        buf[i] = *b as c_char;
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
        let mut chaine: [c_char; TAILLE_STR_MESS_FORMATE as usize] = [0; 512];
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

    /***********************************************************************************
                     Confirme
    Affichage d'un message d'erreur formaté
        RMQ: Cette fonction supporte un affichage formaté identique au printf du C.
    => Message à afficher sous la forme "format", arguments
    <= VRAI si confirmation
    ***********************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn Confirme(&self, format: *mut c_char) -> Boolean {
        unsafe {
            let mut chaine: [c_char; TAILLE_STR_MESS_FORMATE as usize] = [0; 512];
            let mut etat: c_short = 0;

            /* On fixe la zone à la zone de secours de l'interface */
            if !format.is_null() {
                let src = std::ffi::CStr::from_ptr(format);
                copie_chaine(&mut chaine, &src.to_string_lossy());
                CarboniseSetCursorArrow();
                CtoPstr(chaine.as_mut_ptr());
                ParamText(
                    &raw mut chaine as *mut u8,
                    TRANS_VIDE.as_ptr() as *mut u8,
                    TRANS_VIDE.as_ptr() as *mut u8,
                    TRANS_VIDE.as_ptr() as *mut u8,
                );
                etat = self.DoAlertSapiens(CvtDefaultLangRess(135)); /* ALRT_CONFIRMATION */
            }
            etat == 1
        }
    }
}

/***********************************************************************************
                        MessInit
Affichage d'un message durant l'initialisation
***********************************************************************************/
pub(crate) fn MessInit(_format: *mut c_char) {}

/**********************************************************************************
                                SetValItem
=> Fixe une valeur type chainedans un item
<= VRAI si la valeur est bonne, FAUX si la valeur a été tronquée
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn SetValItem(
    dialog: DialogPtr,
    itemHit: c_short,
    c: *mut c_char,
) -> c_short {
    unsafe {
        let mut itemType: c_short = 0;
        let mut control: Handle = core::ptr::null_mut();
        let mut box_: Rect = Rect::default();
        let mut chaine: [c_char; 255] = [0; 255];

        strcpy(&raw mut chaine as *mut c_char, c);
        /* C'est un texte ??? */
        GetDItem(
            dialog,
            itemHit,
            &raw mut itemType,
            &raw mut control,
            &raw mut box_,
        );
        /* On fixe la chaine */
        CtoPstr(chaine.as_mut_ptr());
        if (itemType & editText as c_short) != 0 || (itemType & statText as c_short) != 0 {
            SetIText(control, &raw mut chaine as *mut u8);
        }
        if (itemType & ctrlItem as c_short) != 0 {
            SetCTitle(control, &raw mut chaine as *mut u8);
        }
        VRAI as c_short
    }
}

/***********************************************************************************
                          SetShortItem
 Fixe une valeur de type short dans un item en respectant le formatage
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn SetShortItem(
    dialog: DialogPtr,
    itemHit: c_short,
    val: c_short,
) -> c_short {
    unsafe {
        let mut chaine: [c_char; 80] = [0; 80];

        copie_chaine(&mut chaine, &format!("{val}"));
        SetValItem(dialog, itemHit, &raw mut chaine as *mut c_char)
    }
}

/***********************************************************************************
                                SetValBouton
=> Fixe une valeur VRAI ou FAUX dans un bouton
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetValBouton(
    dialog: DialogPtr,
    itemHit: c_short,
    value: Boolean,
) -> c_short {
    let mut itemType: c_short = 0;
    let mut control: Handle = core::ptr::null_mut();
    let mut box_: Rect = Rect::default();

    /* C'est un texte ??? */
    GetDItem(
        dialog,
        itemHit,
        &raw mut itemType,
        &raw mut control,
        &raw mut box_,
    );
    if (itemType & ctrlItem as c_short) != 0 || (itemType & radCtrl as c_short) != 0 {
        GetDItem(
            dialog,
            itemHit,
            &raw mut itemType,
            &raw mut control,
            &raw mut box_,
        );
        SetCtlValue(control as ControlHandle, if value { 1 } else { 0 });
        VRAI as c_short
    } else {
        FAUX as c_short
    }
}

/***********************************************************************************
                                GetValBouton
=>lit une valeur VRAI ou FAUX dans un bouton
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn GetValBouton(dialog: DialogPtr, itemHit: c_short) -> c_short {
    let mut itemType: c_short = 0;
    let mut control: Handle = core::ptr::null_mut();
    let mut box_: Rect = Rect::default();
    let value: c_short;

    /* C'est un bouton ??? */
    GetDItem(
        dialog,
        itemHit,
        &raw mut itemType,
        &raw mut control,
        &raw mut box_,
    );
    if (itemType & ctrlItem as c_short) != 0 || (itemType & radCtrl as c_short) != 0 {
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
    FAUX as c_short
}

/***********************************************************************************
                                SetActiveItem
=> Fixe un Item à actif ou inactif si c'est un control
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn SetActiveItem(dialog: DialogPtr, itemHit: c_short, value: Boolean) {
    let mut itemType: c_short = 0;
    let mut control: Handle = core::ptr::null_mut();
    let mut box_: Rect = Rect::default();

    GetDItem(
        dialog,
        itemHit,
        &raw mut itemType,
        &raw mut control,
        &raw mut box_,
    );
    if value {
        itemType &= (0xFFFF - itemDisable) as c_short;
    } else {
        itemType |= itemDisable as c_short;
    }
    if (itemType & editText as c_short) != 0 && !value {
        itemType = statText as c_short;
    }
    if (itemType & statText as c_short) != 0 && value {
        itemType = editText as c_short;
    }
    SetDItem(dialog, itemHit, itemType, control, &raw mut box_);

    if (itemType & ctrlItem as c_short) != 0 {
        HiliteControl(control, if value { 0 } else { 255 });
    }
}

/***********************************************************************************
                                GetValItem
Lit  une valeur type chaine dans un item texte editable
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn GetValItem(
    dialog: DialogPtr,
    itemHit: c_short,
    chaine: *mut c_char,
) -> c_short {
    let mut itemType: c_short = 0;
    let mut handle: Handle = core::ptr::null_mut();
    let mut box_: Rect = Rect::default();

    /* C'est un texte ??? */
    GetDItem(
        dialog,
        itemHit,
        &raw mut itemType,
        &raw mut handle,
        &raw mut box_,
    );
    if (itemType & editText as c_short) != 0 || (itemType & statText as c_short) != 0 {
        GetIText(handle, chaine as *mut u8);
        PtoCstr(chaine as *mut u8);
        VRAI as c_short
    } else {
        FAUX as c_short
    }
}

/***********************************************************************************
                                GetShortItem
 Lit une valeur de type short dans un item en respectant le formatage
**********************************************************************************/
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn GetShortItem(
    dialog: DialogPtr,
    itemHit: c_short,
    val: *mut c_short,
) -> c_short {
    unsafe {
        let mut chaine: [c_char; 255] = [0; 255];

        if GetValItem(dialog, itemHit, &raw mut chaine as *mut c_char) != 0 {
            let cstr = std::ffi::CStr::from_ptr(&raw const chaine as *const c_char);
            if let Ok(v) = cstr.to_string_lossy().trim().parse::<i16>() {
                *val = v;
            }
        }
        VRAI as c_short
    }
}

/******************************************************************************************
                                    DefAfficheItem
 Défini le vecteur d'affichage d'un item de Type User
=> Dialog, N° de l'item, vecteur
******************************************************************************************/
pub(crate) fn DefAfficheItem(
    dialog: DialogPtr,
    item: c_short,
    vect_affiche: Option<unsafe extern "C" fn(window: DialogPtr, itemid: c_short)>,
) {
    let mut handle: Handle = core::ptr::null_mut();
    let mut rect: Rect = Rect::default();
    let mut itemType: c_short = 0;

    GetDialogItem(
        dialog,
        item,
        &raw mut itemType,
        &raw mut handle,
        &raw mut rect,
    );
    if (itemType & 127) == userItem as c_short {
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
    pub(crate) fn DoAlertSapiens(&self, num: c_short) -> c_short {
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
pub(crate) fn GetRectItem(dialog: DialogPtr, item: c_short, box_: *mut Rect) {
    let mut itemType: c_short = 0;
    let mut handle: Handle = core::ptr::null_mut();

    /* On récupère l'item coorespondant à l'aire (seule l'aire nous intéresse) */
    GetDItem(dialog, item, &raw mut itemType, &raw mut handle, box_);
}

/*****************************************************************************
                       DrawArrowPopUp
****************************************************************************/
pub(crate) fn DrawArrowPopUp(r: *mut Rect) {
    let mut i: c_short = 0;
    unsafe {
        let y = ((*r).bottom + (*r).top) / 2 + 3;
        let mut box_: Rect = Rect::default();
        SetRect(
            &raw mut box_,
            (*r).right - 15,
            (*r).top + 1,
            (*r).right - 1,
            (*r).bottom - 1,
        );
        DoEraseRect(&box_);
        while i < 6 {
            DoMoveTo((*r).right - 10 - i, y - i);
            DoLine(i * 2, 0);
            i += 1;
        }
    }
}

impl SapWind {
    /******************************************************************************************
                                        GetStr
    Recupere une chaine indexée dans une locale et la passe au format C
    <= Ptr sur la locale
    ******************************************************************************************/
    pub(crate) fn GetStr(&mut self, index: c_short) -> *mut c_char {
        let s = &raw mut self.OneString as *mut c_char;
        GetIndString(
            s as *mut c_void as StringPtr,
            CvtDefaultLangRess(STR_VRAC as c_short),
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
    pub(crate) fn initrnd(&mut self, sem1: c_short, sem2: c_short, sem3: c_short) {
        self.CarboniseSetRandSeed(
            ((sem1 as c_long) << 16) + ((sem2 as c_long) << 8) + sem3 as c_long,
        );
    }

    /**********************************************************************
                        rnd0
    ***********************************************************************/
    pub(crate) fn rnd0(&mut self) -> c_short {
        (self.Random() & 0xFF) as c_short
    }

    /**********************************************************************
                        Rnd
    ***********************************************************************/
    pub(crate) fn Rnd(&mut self) -> c_short {
        (self.Random() & 0xFF) as c_short
    }

    /**********************************************************************
                        myRandom
    Sort une valeur entre deux bornes
    ***********************************************************************/
    pub(crate) fn myRandom(&mut self, min: c_long, max: c_long) -> c_long {
        let mut rdm = (self.Rnd() as c_long) as u16;
        rdm = rdm.wrapping_add(((self.Rnd() as c_long) * 256) as u16);
        let range = max - min;
        let t = ((rdm as c_long) * range) / 65536;
        min + t
    }
}
/**********************************************************************
                    bande
Affiche une bande sur les cotés
***********************************************************************/
/*;********* BANDE_: 0= bande jeu, 1= bande silex *******************/
pub(crate) fn bande(_quelle: c_short) {}

impl Sapiens {
    /*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                            SAPDISK.S
    +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    /****************************************************************************
    Boxf: trace une boite sur le port spécifié
    Les  coordonnées sont données en ecran ST 320x200
    (C) Ligne dep,Col dep,Ligne fin,col.fin,index couleur ST
    ***************************************************************************/
    pub(crate) fn boxf(
        &self,
        lignedep: c_short,
        coldep: c_short,
        lignefin: c_short,
        colfin: c_short,
        index: c_short,
    ) {
        let mut aire = Rect::new(
            coldep * ECHELLE as c_short,
            lignedep * ECHELLE as c_short,
            colfin * ECHELLE as c_short,
            lignefin * ECHELLE as c_short,
        );
        unsafe {
            self.BoxFill(
                port::current(),
                &raw mut aire,
                *self.sapglob.palhaute.offset(index as isize),
            );
        }
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
pub(crate) fn cadreg(_ligne: c_short, _col: c_short, _nligne: c_short, _ncol: c_short) {}

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
        CheckItem(menu, iEchelle100 as c_short, self.sapwind.SapZoom == 1.0);
        CheckItem(menu, iEchelle200 as c_short, self.sapwind.SapZoom == 2.0);
        CheckItem(menu, iEchelle300 as c_short, self.sapwind.SapZoom == 3.0);
        CheckItem(menu, iEchelle400 as c_short, self.sapwind.SapZoom == 4.0);
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
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn DoInfo(&self) {
        let mut str_: [c_char; 255] = [0; 255];
        unsafe {
            GetVersionSapiens(&raw mut str_ as *mut c_char);
        }
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
        let menuID: c_short;
        let menuItem: c_short;
        let mut daName: [u8; 256] = [0; 256];

        self.AdjustMenus();
        self.souris.MontreSouris();
        menuID = ((menuResult >> 16) & 0xFFFF) as c_short; /* HiWord */
        menuItem = (menuResult & 0xFFFF) as c_short; /* LoWord */
        unsafe {
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
                    itemQuitter => {
                        if self.ConfirmePerteAventure() {
                            SapiensExit(0);
                        }
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
            }
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
    pub(crate) fn DoEvtClavier(&mut self, event: *const EventRecord) {
        unsafe {
            let ev = &*event;
            let mut t: c_long;

            if ev.modifiers & cmdKey as c_short != 0 {
                let _car = (ev.message & 0xFF) as c_short;
                self.AdjustMenus();
                self.DoMenuCommand(MenuKey((ev.message & charCodeMask as i64) as c_short) as i32);
            } else if ev.modifiers & optionKey as c_short != 0 {
                let mut i: c_short;
                let car: c_short = ((ev.message & 0xFF00) >> 8) as c_short;
                if car == 0x11
                /*T*/
                {
                    /* Transfert Ecran */
                    t = TickCount();
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
                    t = TickCount();
                    i = 0;
                    while i < 50 {
                        self.affich(self.sapglob.dzeta as c_ushort);
                        i += 1;
                    }
                    self.Collapse(&format!("{i} images 3d : {} ticks", TickCount() - t));
                }
                if car == 0x23
                /*'P'*/
                {
                    self.AffPalette();
                }
                if car == 0x29
                /*'M'*/
                {
                    if self.sapwmus.quelle_musique == 1 {
                        self.MusicInit(2);
                    } else {
                        self.MusicInit(1);
                    }
                }
                if car == 0x06
                /*'T'*/
                {
                    i = 0;
                    while i < 64 {
                        self.chrono(1);
                        i += 1;
                    }
                }
                if car == 0x0f
                /*R*/
                {
                    self.InitMobs();
                }
                if car == 0x01
                /*S*/
                {
                    t = 0;
                    MaxMem(&raw mut t);
                }
            } else {
                let car: c_short = (ev.message & charCodeMask as i64) as c_short;
                /* Touches de fonction */
                if car == 0x10 {
                    self.DoAide();
                }
            }
        }
    }
}

/*******************************************************************
                        CalcIndexPalette
Calcule la deuxieme section de la palette ST en entrée :
index de la couleur dans la palette systeme
********************************************************************/
unsafe fn CalcIndexPalette(palette: *mut c_short) {
    unsafe {
        let mut rgb: RGBColor = RGBColor::default();

        let mut i: c_short = 0;
        while i < 16 {
            Sapwcvt::CvtCouleurEnRgb(palette, i, &mut rgb);
            *palette.offset((16 + i) as isize) = SapColor2Index(&rgb);
            i += 1;
        }
    }
}

/*******************************************************************
                        FixeMenu
=> Numero de la menu Bar à installer, Numero du menu Pomme de cette menuBar
********************************************************************/
pub(crate) fn FixeMenu(n: c_short) -> Boolean {
    let menuBar: Handle = GetNewMBar(n); /* read menus into menu bar */
    if menuBar.is_null() {
        return FAUX;
    }
    SetMenuBar(menuBar); /* install menus */
    DisposHandle(menuBar);
    let menu: MenuHandle = GetMHandle(CvtDefaultLangRess(128)); /* menuPomme */
    if !menu.is_null() {
        AddResMenu(menu, DRVR as c_long); /* add DA names to Apple menu */
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
        let mut nb: c_short;

        let _g = port::on(self.sapwind.Ecran1);

        self.LibereDataMobs();
        PushPath();
        SetPathInData(&self.sapgest, c"Mobs".as_ptr() as *mut c_char);
        self.sapwind.mobBambou = 0;
        nb = self.ChargeFichierBrt(
            c"WBAMBOU.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobBambou + 1,
            26,
            0,
        ); /*0, 21 Mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobFace = self.sapwind.mobBambou + nb;
        nb = self.ChargeFichierBrt(
            c"WFACE.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobFace + 1,
            31,
            0,
        ); /* Base 21,29 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobFeuille = self.sapwind.mobFace + nb;
        nb = self.ChargeFichierBrt(
            c"WFEUILLE.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobFeuille + 1,
            36,
            11,
        ); /* Base 50 36 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobFleches = self.sapwind.mobFeuille + nb;
        nb = self.ChargeFichierBrt(
            c"WFLECHES.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobFleches + 1,
            20,
            0,
        ); /* Base 8618 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobSapmob = self.sapwind.mobFleches + nb;
        nb = self.ChargeFichierBrt(
            c"WSAPMOB.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobSapmob + 1,
            141,
            0,
        ); /*Base 120+1,151 mobs */
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobAvplan = self.sapwind.mobSapmob + nb;
        nb = self.ChargeFichierBrt(
            c"WAVPLAN.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobAvplan + 1,
            9,
            13,
        );
        self.sapcomb.nbMobHerbes = nb;
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobAvplanSpec = self.sapwind.mobAvplan + nb;
        nb = self.ChargeFichierBrt(
            c"WAVSPEC.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobAvplanSpec + 1,
            4,
            0,
        );
        self.sapcomb.nbAvplanSpec = nb;
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mobSilex = self.sapwind.mobAvplanSpec + nb;
        nb = self.ChargeFichierBrt(
            c"WSILEX.BRT".as_ptr() as *mut c_char,
            self.sapwind.mobSilex + 1,
            15,
            0,
        );
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.mob3D = self.sapwind.mobSilex + nb;
        nb = self.ChargeFichierBrt(
            c"W3D.BRT".as_ptr() as *mut c_char,
            self.sapwind.mob3D + 1,
            38,
            0,
        );
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.Fonte14 = self.sapwind.mob3D + nb;
        nb = self.ChargeFichierBrt(
            c"Geneva14.BRT".as_ptr() as *mut c_char,
            self.sapwind.Fonte14 + 1,
            120,
            0,
        );
        if nb < 0 {
            return FAUX;
        }
        self.sapwind.Fonte10 = self.sapwind.Fonte14 + nb;
        nb = self.ChargeFichierBrt(
            c"Geneva10.BRT".as_ptr() as *mut c_char,
            self.sapwind.Fonte10 + 1,
            120,
            0,
        );
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
    let mut r: Rect = Rect::default();
    let _g = port::on(gfx);
    let portRect = port::bounds(gfx);
    /* Le fond au dessus de l'awale */
    let picMotif: PicHandle = GetPicture(128) as PicHandle;
    let mut y: c_short = 0;
    while y < portRect.bottom {
        let mut x: c_short = 0;
        while x < portRect.right + 16 {
            BackColor(whiteColor as c_short);
            ForeColor(blackColor as c_short);
            SetRect(&raw mut r, x, y, x + 64, y + 64);
            DrawPicture(picMotif, &raw const r as *const Rect);
            x += 64;
        }
        y += 64;
    }
    ReleaseResource(picMotif as Handle);
}

impl Sapiens {
    /***********************************************************************************
                                     FixeTitreMainWindow
    ***********************************************************************************/
    pub(crate) fn FixeTitreMainWindow(&self) {
        unsafe {
            let mut chaine: [c_char; 255] = [0; 255];
            let mut str_: [c_char; 255] = [0; 255];

            strcpy(
                &raw mut chaine as *mut c_char,
                c"Sapiens ".as_ptr() as *const c_char,
            );
            GetVersionSapiens(&raw mut str_ as *mut c_char);
            strcat(
                &raw mut chaine as *mut c_char,
                &raw const str_ as *const c_char,
            );
            CtoPstr(chaine.as_mut_ptr());
            port::set(self.sapwind.mainWindow);
            SetWTitle(self.sapwind.mainWindow, &raw mut chaine as *mut u8);
        }
    }

    /****************************************************************************
                Encadre l'aire 3d
    => etat 1 actif
    *****************************************************************************/
    pub(crate) fn Frame3d(&self, etat: c_short) {
        let _g = port::on(self.sapwind.debecr);
        let r = Rect::new(62, 0, 640 - 64, 204);
        if etat != 0 {
            ForeColor(blackColor as c_short);
        } else {
            ForeColor(whiteColor as c_short);
        }
        DoPenSize(2, 2);
        DoFrameRect(&r);
        ForeColor(blackColor as c_short);
        PenNormal();
        port::set(self.sapwind.mainWindow);
        if etat != 0 {
            ForeColor(blackColor as c_short);
        } else {
            ForeColor(whiteColor as c_short);
        }
        DoPenSize(2, 2);
        DoFrameRect(&r);
        PenNormal();
        ForeColor(blackColor as c_short);
    }

    /**********************************************************************************
                        MenuFenetres
    Entre ou sort du menu fenetre actif
    => etat =1 si actif
    ***********************************************************************************/
    pub(crate) fn MenuFenetres(&mut self, etat: c_short) {
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
    unsafe fn AffStr(
        &self,
        chaine: *mut c_char,
        fonte: c_short,
        size: c_short,
        face: c_short,
        x_in: c_short,
        y: c_short,
    ) {
        let mut r: Rect = Rect::default();
        let mut x: c_short = x_in;

        CtoPstr(chaine);
        TextFont(fonte);
        TextSize(size);
        TextFace(face);
        if x < 0 {
            x = (640 - StringWidth(chaine as *mut u8)) / 2;
        }
        ForeColor(blackColor as c_short);
        DoTextSize(size);
        DoMoveTo(x + 1, y + 1);
        DrawString(chaine as *mut u8);
        ForeColor(whiteColor as c_short);
        DoMoveTo(x, y);
        DrawString(chaine as *mut u8);
        DoMoveTo(x - 1, y);
        DrawString(chaine as *mut u8);
        ForeColor(blackColor as c_short);
        PtoCstr(chaine as *mut u8);
        self.aire_rect(&raw mut r);
    }
}

/******************************************************************************************
                                SetFontDialog
Fixe la fonte du dialogue en entrée
******************************************************************************************/
unsafe fn SetFontDialog(dialog: DialogPtr, font: c_short, taille: c_short) {
    unsafe {
        let mut info: crate::shared::FontInfo = crate::shared::FontInfo::default();

        CarboniseSetPortDialog(dialog);
        TextFont(font);
        TextSize(taille);
        GetFontInfo(&raw mut info);
        let te: TEHandle = CarboniseGetDialogTextEditHandle(dialog);
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
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn OuvreDialogue(numDitl: c_short) -> DialogPtr {
    unsafe {
        let dialog: DialogPtr = GetNewDialog(numDitl, core::ptr::null_mut(), -1isize as WindowPtr);
        if !dialog.is_null() {
            SetFontDialog(dialog, geneva as c_short, 9);
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
        let largeur = (320.0 * ECHELLE as f32 * self.SapZoom) as c_long;
        let hauteur = (200.0 * ECHELLE as f32 * self.SapZoom) as c_long;

        let aireDevice = port::SCREEN;
        /* Creation de la fenetre principale */
        /* Aire de la fenetre */
        let mut rect = Rect::new(
            0,
            0,
            (320.0 * ECHELLE as f32 * self.SapZoom) as c_short,
            (200.0 * ECHELLE as f32 * self.SapZoom) as c_short,
        );
        /* On la centre sur l'aire generale de l'ecran */
        rect.OffsetRect(
            aireDevice.left + (aireDevice.right - aireDevice.left - largeur as c_short) / 2,
            aireDevice.top
                + (CarboniseGetMenuBarHeight() + 8)
                + (aireDevice.bottom
                    - aireDevice.top
                    - CarboniseGetMenuBarHeight()
                    - hauteur as c_short)
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
        let mut i: c_short;
        let mut y: c_short;
        let mut r: Rect = Rect::default();
        let mut chaine: [c_char; 255] = [0; 255];

        /* Init des chemins standards */
        self.InitCheminsSapiens();
        SetCursor(GetCursor(watchCursor as c_short));
        self.FixeAireNoMouseAireBasse();
        /* On charge les preferences */
        let firstInit = self.ChargePref();
        InitDefautConfigSortieNumerique();
        self.MusicInit(1); /* Lance musique 1 */
        /* Choix de la langue*/
        if langue() == -1 {
            CarboniseSetCursorArrow();
            set_langue(Alert(ALRT_VERSION, core::ptr::null_mut()) - 1);
            SetCursor(GetCursor(watchCursor as c_short));
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
                    ((320 * ECHELLE as c_short as c_long - (r.right - r.left) as c_long) / 2)
                        as c_short,
                    60,
                );
                let rdest = DoZoomRect(&r);
                DrawPicture(pict, &raw const rdest);
                ReleaseResource(pict as Handle);
            }
            /* Affichage de la version de sapiens */
            GetVersionSapiens(&raw mut chaine as *mut c_char);
            self.sapwind.AffStr(
                &raw mut chaine as *mut c_char,
                helvetica as c_short,
                14,
                bold as c_short,
                -1,
                r.bottom + 16,
            );
            /* Affichage des autres commentaires */
            strcpy(
                &raw mut chaine as *mut c_char,
                c"Copyright Didier & Olivier Guillion, 1986,1996,2005,2017".as_ptr()
                    as *const c_char,
            );
            self.sapwind.AffStr(
                &raw mut chaine as *mut c_char,
                helvetica as c_short,
                16,
                bold as c_short,
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
                if strlen(&raw const chaine as *const c_char) != 0 {
                    self.sapwind.AffStr(
                        &raw mut chaine as *mut c_char,
                        helvetica as c_short,
                        16,
                        bold as c_short,
                        -1,
                        y,
                    );
                }
                y += 18;
                if strlen(&raw const chaine as *const c_char) == 0 {
                    break;
                }
            }
            let timeBase = TickCount() + 5 * 60;

            self.sapwind.aire_rect(&raw mut r);
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
            CalcIndexPalette(&raw mut self.saplist.pal1 as *mut c_short);
            CalcIndexPalette(&raw mut self.saplist.pal2 as *mut c_short);
            self.sapwind.Ecran1 =
                SapiensCreeOffscreen(256 * ECHELLE as c_short, 100 * ECHELLE as c_short);
            if self.sapwind.Ecran1.is_null() {
                self.Collapse("[SapWind.init_graph.1] Unable to create offscreen");
                SapiensExit(-2);
            }
            self.sapwind.Ecran2 =
                SapiensCreeOffscreen(256 * ECHELLE as c_short, 100 * ECHELLE as c_short);
            port::set(self.sapwind.Ecran1);
            MessInit(self.sapwind.GetStr(21)); /* Initialsations des palettes */
            /* Fabrique les luts */
            self.sapwcvt.FabriqueLuts();
            self.sapwind.debecr =
                SapiensCreeOffscreen(320 * ECHELLE as c_short, 200 * ECHELLE as c_short);
            port::set(self.sapwind.debecr);
            AfficheFond(self.sapwind.debecr);
            /* Buffer de la souris*/
            self.sapwind.BuffMouse =
                SapiensCreeOffscreen(64 * ECHELLE as c_short, 64 * ECHELLE as c_short);
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
            /* Installation du timer de comptage de frame */
            self.InstalleTimeTask();
            /* Transfert de l'ecran */
            CarboniseSetCursorArrow();
            firstInit
        }
    }

    /*******************************************************************
                            GereInterface
    Gestion générale de l'interface
    ********************************************************************/
    pub(crate) extern "C" fn GereInterface(&mut self) {
        let mut part;
        let mut windowResult = WindowPtr::default();
        let mut event = EventRecord::default();
        let mut bounds = Rect::default();

        /* Commande de menu venue de la barre egui : elle s'execute ici, sur le
        thread C, la ou vit le `&mut Sapiens`. Le swap a ete fait, donc un modal
        qui repompe GereInterface ne la relira pas. */
        if let Some(cmd) = take_menu_command() {
            self.DoMenuCommand(cmd);
        }

        let _g = port::on(self.sapwind.mainWindow);
        let window = FrontWindow();
        self.CheckChangeMusique();
        unsafe {
            while GetNextEvent(everyEvent as i16, &mut event) {
                match event.what as u32 {
                    _keyDown => {
                        self.sapwind.toucheClav =
                            (event.message as c_short & charCodeMask as c_short) as c_short;
                        self.DoEvtClavier(&raw const event as *const _);
                    }
                    _mouseDown => {
                        /* On cherche quelle fenetre a été cliquée */
                        part = FindWindow(event.where_, &mut windowResult);
                        /* Où a-t'on cliqué ??? */
                        match part as u32 {
                            /* Dans la barre de menu , alors on le traite */
                            _inMenuBar => {
                                self.DoMenuCommand(MenuSelect(event.where_));
                            }
                            /* Sur le bureau, alors le bureau se débrouille ... sauf si c'est un pseudo document */
                            _inSysWindow => {
                                SystemClick(&event, windowResult);
                            }
                            /* Dans le deplacement */
                            _inDrag => {
                                SelectWindow(windowResult);
                                bounds = port::SCREEN;
                                DragWindow(windowResult, event.where_, &bounds);
                            }
                            /* Dans une fenetre : alors on le traite */
                            _inContent => {
                                if windowResult != window {
                                    SelectWindow(windowResult);
                                }

                                //if(window==mainWindow)
                                //       DoClick(event.where);
                            }
                            _ => {}
                        }
                    }
                    _updateEvt => {
                        if port::is_window(window) {
                            //                    AcamGetPortOffscreen(&gbak);
                            port::set(window);
                            BeginUpdate(window);
                            self.TfrEcr(None);
                            self.tfrima();
                            if self.sapglob.dervue == Vue::LATERALE {
                                self.tfrdemi();
                            }
                            EndUpdate(window);
                            //                    AcamSetPortOffscreen(gbak);
                        }
                    }
                    _activateEvt => {
                        if (event.message & 1) > 0 {
                            /* on reprends le traitement */
                        } else {
                            /* On est arrêté */
                            self.souris.MontreSouris();
                        }
                    }
                    _app4Evt => {
                        if (event.message >> 24) == 1 {
                            if (event.message & 1) > 0 {
                                /* on reprends le traitement */
                                port::set(self.sapwind.mainWindow);
                                self.TfrEcr(None);
                                self.tfrima();
                                if self.sapglob.dervue == Vue::LATERALE {
                                    self.tfrdemi();
                                }
                            } else {
                                /* On est arrêté */
                                self.souris.MontreSouris();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        SystemTask();
    }

    /*******************************************************************
                            delai
    => Nbre de 60eme de seocndes a atteindre
    ********************************************************************/
    pub(crate) fn delai(&mut self, duree: c_long) {
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
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn FixeTailleMainWindow(&self) {
        let mut rect = Rect::default();

        unsafe {
            let _g = port::on(self.mainWindow);
            SetRect(
                &mut rect,
                0,
                0,
                (320.0 * ECHELLE as f32 * self.SapZoom) as i16,
                (200.0 * ECHELLE as f32 * self.SapZoom) as i16,
            );
            SizeWindow(self.mainWindow, rect.right, rect.bottom, true);
        }
    }
}

/*******************************************************************
                        SapiensExit
********************************************************************/
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SapiensExit(code: c_short) {
    ArreteMusique(true);
    exit(code as i32);
}

/*EOF*/
