#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/***********************************************************************
                        SauvePref
***********************************************************************/
use crate::Sapiens;
use crate::acam::{BackColor, CtoPstr, FrameRect, GetIndString, PtoCstr};
use crate::cbridge::{
    DialogPtr, MenuHandle, OSErr, fclose, fopen, fread, fwrite, gestaltFindFolderAttr, strcmp,
    strlen, times, whiteColor,
};
use crate::langue::{langue_i16, set_langue};
use crate::port;
use crate::sapiens::saptxt::CvtDefaultLangRess;
use crate::sapiens::sapwbrt::{DoFrameRect, DoTextBox};
use crate::sapiens::sapwind::{
    DefAfficheItem, DrawArrowPopUp, FixeMenu, GetRectItem, GetValBouton, OuvreDialogue,
    SetValBouton,
};
use crate::sapiens::{NOM_PREF, TypeMuse};
use crate::shared::{Boolean, Rect, StringPtr};
use crate::stubs::{
    DeleteMenu, DisposeMenu, FindFolder, FixeVolPath, Gestalt, GetMouse, InsMenuItem, InsertMenu,
    LocalToGlobal, ModalDialog, NewMenu, PopUpMenuSelect, PreservePath, RestorePath,
};
use crate::text::{TextFace, TextFont, TextSize};
use std::os::raw::c_void;

const VERSION_PREF: &[u8] = b"Sapiens Preferences 2.0.0\0";

const ITEM_OK: i16 = 1;
const ITEM_ANNULER: i16 = 2;
const ITEM_LANGUE: i16 = 3;
const ITEM_BRUITAGES: i16 = 4;
const ITEM_MUSIQUES: i16 = 5;
const ITEM_POLICES_LISSEES: i16 = 6;
const ITEM_2_AUTO: i16 = 7;
const ITEM_2_CLAVIER: i16 = 8;
const ITEM_RECT1: i16 = 9;

pub(crate) const JOUEUR2_AUTONOME: i16 = 0;
const JOUEUR2_CLAVIER: i16 = 1;
const SAP_NB_LANGUAGES: i16 = 3;
const ID_MENU: i16 = 1;

impl Sapiens {
    pub(crate) fn SauvePref(&mut self) {
        PreservePath();
        SetPathPreferences();
        unsafe {
            let f = fopen(NOM_PREF.as_ptr() as *const i8, c"wb".as_ptr());
            RestorePath();
            if f.is_null() {
                return;
            }
            if fwrite(
                VERSION_PREF.as_ptr() as *const c_void,
                std::mem::size_of::<i8>() as u64,
                strlen(VERSION_PREF.as_ptr() as _) + 1,
                f,
            ) != strlen(VERSION_PREF.as_ptr() as _) + 1
            {
                self.Collapse("Error writing preferences.");
                fclose(f);
                return;
            }
            let is_bruitages: i16 = if self.sapwmus.IsBruitages { 1 } else { 0 };
            fwrite(
                &is_bruitages as *const i16 as *const c_void,
                size_of::<i16>() as u64,
                1,
                f,
            );
            fwrite(
                &raw const self.sapcomb.ModeJoueur2 as *const c_void,
                size_of::<i16>() as u64,
                1,
                f,
            );
            fwrite(
                &raw const self.sapglob.okmuse as *const u32 as *const c_void,
                size_of::<i16>() as u64,
                1,
                f,
            );
            let l_pref: i16 = langue_i16();
            fwrite(
                &l_pref as *const i16 as *const c_void,
                size_of::<i16>() as u64,
                1,
                f,
            );
            fwrite(
                &raw const self.sapwind.SapZoom as *const c_void,
                size_of::<i16>() as u64,
                1,
                f,
            );
            fclose(f);
        }
    }
}

/*******************************************************************
                        SetPathPreferences
********************************************************************/
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetPathPreferences() {
    let mut err: OSErr;
    let mut result: i64 = 0;
    let mut volRefNum: i16 = 0;
    let mut dirId: i64 = 0;

    err = Gestalt(gestaltFindFolderAttr as i64, &raw mut result);
    if err == 0 && (result & 1) != 0 {
        err = FindFolder(
            crate::cbridge::kOnSystemDisk as i16,
            crate::cbridge::kPreferencesFolderType as i64,
            crate::cbridge::kCreateFolder != 0,
            &raw mut volRefNum,
            &raw mut dirId,
        );
        if err == 0 {
            FixeVolPath(volRefNum, dirId);
        }
    }
}

impl Sapiens {
    /***********************************************************************
                            ChargePref
    <= VRAi si first init
    ***********************************************************************/
    pub(crate) fn ChargePref(&mut self) -> Boolean {
        let mut chaine = [0; 255];
        PreservePath();
        unsafe {
            SetPathPreferences();
            let f = fopen(NOM_PREF.as_ptr() as *const i8, c"rb".as_ptr());
            RestorePath();
            if !f.is_null() {
                fread(
                    chaine.as_mut_ptr() as *mut c_void,
                    std::mem::size_of::<i8>() as u64,
                    strlen(VERSION_PREF.as_ptr() as _) + 1,
                    f,
                );
                if strcmp(chaine.as_ptr(), VERSION_PREF.as_ptr() as *const i8) == 0 {
                    let mut tmp: i16 = 0;
                    fread(&raw mut tmp as *mut c_void, size_of::<i16>() as u64, 1, f);
                    self.sapwmus.IsBruitages = tmp != 0;
                    fread(
                        &raw mut self.sapcomb.ModeJoueur2 as *mut c_void,
                        size_of::<i16>() as u64,
                        1,
                        f,
                    );
                    fread(&raw mut tmp as *mut c_void, size_of::<i16>() as u64, 1, f);
                    self.sapglob.okmuse = tmp.into();
                    let mut l_pref: i16 = 0;
                    fread(
                        &raw mut l_pref as *mut c_void,
                        size_of::<i16>() as u64,
                        1,
                        f,
                    );
                    set_langue(l_pref.into());
                    let mut loi = [0u8; 2];
                    fread(
                        loi.as_mut_ptr() as *mut c_void,
                        size_of::<i16>() as u64,
                        1,
                        f,
                    );
                    self.sapwind
                        .set_zoom(f32::from_le_bytes([loi[0], loi[1], 0, 0]));
                }
                fclose(f);
            }
            f.is_null()
        }
    }
}

/*********************************************************************
                        GetLibLangue
********************************************************************/
fn GetLibLangue(n: i16, str_: *mut i8) {
    GetIndString(str_ as *mut u8 as StringPtr, 128, n + 1);
    PtoCstr(str_ as *mut u8);
}

/*********************************************************************
                        VectAfficheMenuLangue
********************************************************************/
unsafe extern "C" fn VectAfficheMenuLangue(window: DialogPtr, itemid: i16) {
    let mut r: Rect = Rect::default();
    let mut chaine = [0; 255];

    let _g = port::on(window);
    GetRectItem(window, itemid, &raw mut r);
    GetLibLangue(langue_i16(), chaine.as_mut_ptr());
    BackColor(whiteColor as i16);
    r.right -= 14;
    TextFont(times as i16);
    TextSize(16);
    TextFace(0);
    unsafe {
        DoTextBox(
            chaine.as_mut_ptr(),
            strlen(chaine.as_ptr()) as i16,
            &r,
            crate::cbridge::teJustLeft as i16,
        );
    }
    r.right += 14;
    r.InsetRect(-1, -1);
    DrawArrowPopUp(&mut r);
    FrameRect(&r);
}

/******************************************************************************************
                                CreeMenuLangue
=> Menu a remplir
******************************************************************************************/
fn CreeMenuLangue(menu: MenuHandle) {
    let mut nom = [0; 255];

    for i in 0..SAP_NB_LANGUAGES {
        GetLibLangue(i, nom.as_mut_ptr());
        CtoPstr(nom.as_mut_ptr());
        InsMenuItem(menu, nom.as_mut_ptr() as *mut u8, 9999);
    }
}

/******************************************************************************************
                                VectFrameBouton
Le bouton est enncadre
******************************************************************************************/
unsafe extern "C" fn VectFrameBouton(window: DialogPtr, item: i16) {
    let mut r: Rect = Rect::default();

    GetRectItem(window, item, &raw mut r);
    DoFrameRect(&r);
}

impl Sapiens {
    /***********************************************************************
                            DoPreferences
    ***********************************************************************/
    pub(crate) fn DoPreferences(&mut self) {
        let mut item: i16 = 0;
        let mut result: i64;
        let mut menuLangue: MenuHandle;
        unsafe {
            let dialog = OuvreDialogue(CvtDefaultLangRess(137));
            if dialog.is_null() {
                return;
            }
            let _g = port::on(dialog);
            SetValBouton(dialog, ITEM_BRUITAGES, self.sapwmus.IsBruitages);
            SetValBouton(
                dialog,
                ITEM_MUSIQUES,
                self.sapglob.okmuse != TypeMuse::NO_MUSIQUE,
            );
            SetValBouton(dialog, ITEM_POLICES_LISSEES, self.sapwcar.lissage_police);
            DefAfficheItem(dialog, ITEM_RECT1, Some(VectFrameBouton));
            DefAfficheItem(dialog, ITEM_LANGUE, Some(VectAfficheMenuLangue));
            SetValBouton(
                dialog,
                ITEM_2_AUTO,
                self.sapcomb.ModeJoueur2 == JOUEUR2_AUTONOME,
            );
            SetValBouton(
                dialog,
                ITEM_2_CLAVIER,
                self.sapcomb.ModeJoueur2 == JOUEUR2_CLAVIER,
            );
            self.souris.MontreSouris();
            loop {
                ModalDialog(core::ptr::null_mut(), &raw mut item);
                match item {
                    ITEM_2_AUTO => {
                        SetValBouton(dialog, ITEM_2_AUTO, true);
                        SetValBouton(dialog, ITEM_2_CLAVIER, false);
                    }
                    ITEM_2_CLAVIER => {
                        SetValBouton(dialog, ITEM_2_AUTO, false);
                        SetValBouton(dialog, ITEM_2_CLAVIER, true);
                    }
                    ITEM_BRUITAGES | ITEM_MUSIQUES | ITEM_POLICES_LISSEES => {
                        SetValBouton(dialog, item, GetValBouton(dialog, item) == 0);
                    }
                    ITEM_LANGUE => {
                        menuLangue = NewMenu(ID_MENU, b"\x00".as_ptr() as *mut c_void as StringPtr);
                        InsertMenu(menuLangue, -1);
                        CreeMenuLangue(menuLangue);
                        let mut pt = GetMouse();
                        LocalToGlobal(&mut pt);
                        result = PopUpMenuSelect(menuLangue, pt.v, pt.h, langue_i16() + 1);
                        DeleteMenu(ID_MENU);
                        DisposeMenu(menuLangue);
                        if result != 0 {
                            set_langue(((result - 1) as i16).into());
                            VectAfficheMenuLangue(dialog, ITEM_LANGUE);
                        }
                    }
                    _ => {}
                }
                if item == ITEM_OK || item == ITEM_ANNULER {
                    break;
                }
            }
            if item == ITEM_OK {
                if GetValBouton(dialog, ITEM_2_AUTO) != 0 {
                    self.sapcomb.ModeJoueur2 = JOUEUR2_AUTONOME;
                }
                if GetValBouton(dialog, ITEM_2_CLAVIER) != 0 {
                    self.DoAlertSapiens(CvtDefaultLangRess(136));
                    self.sapcomb.ModeJoueur2 = JOUEUR2_CLAVIER;
                }
                self.sapwmus.IsBruitages = GetValBouton(dialog, ITEM_BRUITAGES) != 0;
                self.sapglob.okmuse = GetValBouton(dialog, ITEM_MUSIQUES).into();
                self.sapwcar.lissage_police = GetValBouton(dialog, ITEM_POLICES_LISSEES) != 0;
                self.inimen(8);
                self.SauvePref();
                FixeMenu(CvtDefaultLangRess(0));
                self.sapwind.FixeTailleMainWindow();
                self.inimen(0);
            }
        }
        self.MusicInit(1);
        // CloseDialogue(dialog, CvtDefaultLangRess(137));
    }
}
