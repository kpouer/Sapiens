#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/***********************************************************************
                        SauvePref
***********************************************************************/
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, BackColor, CtoPstr, FrameRect, GetIndString,
    InsetRect, PtoCstr,
};
use crate::cbridge::{
    DialogPtr, MenuHandle, OSErr, fclose, fopen, fread, fwrite, gestaltFindFolderAttr, strcmp,
    strlen, times, whiteColor,
};
use crate::sapiens::NOM_PREF;
use crate::sapiens::sapcomb::ModeJoueur2;
use crate::sapiens::sapglob::okmuse;
use crate::sapiens::saplist::inimen;
use crate::sapiens::saptxt::{CvtDefaultLangRess, langue};
use crate::sapiens::sapwbrt::{DoFrameRect, DoTextBox};
use crate::sapiens::sapwcar::LissagePolice;
use crate::sapiens::sapwclav::MontreSouris;
use crate::sapiens::sapwind::{
    CloseDialogue, Collapse, DefAfficheItem, DoAlertSapiens, DrawArrowPopUp, FixeMenu,
    FixeTailleMainWindow, GetRectItem, GetValBouton, OuvreDialogue, SapZoom, SetValBouton,
};
use crate::sapiens::sapwmus::{IsBruitages, MusicInit};
use crate::shared::{Boolean, Point, Rect, StringPtr};
use crate::stubs::{
    CarboniseSetPortDialog, DeleteMenu, DisposeMenu, FindFolder, FixeVolPath, Gestalt, GetMouse,
    InsMenuItem, InsertMenu, LocalToGlobal, ModalDialog, NewMenu, PopUpMenuSelect, PreservePath,
    RestorePath,
};
use crate::text::{TextFace, TextFont, TextSize};
use std::os::raw::{c_char, c_float, c_long, c_short, c_uint, c_ulong, c_void};

const VERSION_PREF: &[u8] = b"Sapiens Preferences 2.0.0\0";

const ITEM_OK: c_short = 1;
const ITEM_ANNULER: c_short = 2;
const ITEM_LANGUE: c_short = 3;
const ITEM_BRUITAGES: c_short = 4;
const ITEM_MUSIQUES: c_short = 5;
const ITEM_POLICES_LISSEES: c_short = 6;
const ITEM_2_AUTO: c_short = 7;
const ITEM_2_CLAVIER: c_short = 8;
const ITEM_RECT1: c_short = 9;

pub const JOUEUR2_AUTONOME: c_short = 0;
const JOUEUR2_CLAVIER: c_short = 1;
const SAP_NB_LANGUAGES: c_short = 3;
const ID_MENU: c_short = 1;

#[unsafe(no_mangle)]
pub extern "C" fn SauvePref() {
    PreservePath();
    unsafe {
        SetPathPreferences();
        let f = fopen(NOM_PREF.as_ptr() as *const c_char, c"wb".as_ptr());
        RestorePath();
        if f.is_null() {
            return;
        }
        if fwrite(
            VERSION_PREF.as_ptr() as *const c_void,
            std::mem::size_of::<c_char>() as c_ulong,
            strlen(VERSION_PREF.as_ptr() as _) + 1,
            f,
        ) != strlen(VERSION_PREF.as_ptr() as _) + 1
        {
            Collapse("Error writing preferences.".as_ptr() as _);
            fclose(f);
            return;
        }
        let is_bruitages: c_short = if IsBruitages { 1 } else { 0 };
        fwrite(
            &is_bruitages as *const c_short as *const c_void,
            size_of::<c_short>() as c_ulong,
            1,
            f,
        );
        fwrite(
            &raw const ModeJoueur2 as *const c_short as *const c_void,
            size_of::<c_short>() as c_ulong,
            1,
            f,
        );
        fwrite(
            &raw const okmuse as *const c_uint as *const c_void,
            size_of::<c_short>() as c_ulong,
            1,
            f,
        );
        fwrite(
            &raw const langue as *const c_short as *const c_void,
            size_of::<c_short>() as c_ulong,
            1,
            f,
        );
        fwrite(
            &raw const SapZoom as *const c_float as *const c_void,
            size_of::<c_short>() as c_ulong,
            1,
            f,
        );
        fclose(f);
        return;
    }
}

/*******************************************************************
                        SetPathPreferences
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn SetPathPreferences() {
    let mut err: OSErr = 0;
    let mut result: c_long = 0;
    let mut volRefNum: c_short = 0;
    let mut dirId: c_long = 0;

    err = Gestalt(gestaltFindFolderAttr as c_long, &raw mut result);
    if err == 0 && (result & 1) != 0 {
        err = FindFolder(
            crate::cbridge::kOnSystemDisk as c_short,
            crate::cbridge::kPreferencesFolderType as c_long,
            crate::cbridge::kCreateFolder != 0,
            &raw mut volRefNum,
            &raw mut dirId,
        );
        if err == 0 {
            FixeVolPath(volRefNum, dirId);
        }
    }
}

/***********************************************************************
                        ChargePref
<= VRAi si first init
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn ChargePref() -> Boolean {
    let mut chaine = [0 as c_char; 255];
    PreservePath();
    unsafe {
        SetPathPreferences();
        let f = fopen(NOM_PREF.as_ptr() as *const c_char, c"rb".as_ptr());
        RestorePath();
        if !f.is_null() {
            fread(
                chaine.as_mut_ptr() as *mut c_void,
                std::mem::size_of::<c_char>() as c_ulong,
                strlen(VERSION_PREF.as_ptr() as _) + 1,
                f,
            );
            if strcmp(chaine.as_ptr(), VERSION_PREF.as_ptr() as *const c_char) == 0 {
                let mut tmp: c_short = 0;
                fread(
                    &raw mut tmp as *mut c_void,
                    size_of::<c_short>() as c_ulong,
                    1,
                    f,
                );
                IsBruitages = tmp != 0;
                fread(
                    &raw mut ModeJoueur2 as *mut c_void,
                    size_of::<c_short>() as c_ulong,
                    1,
                    f,
                );
                fread(
                    &raw mut tmp as *mut c_void,
                    size_of::<c_short>() as c_ulong,
                    1,
                    f,
                );
                okmuse = tmp as c_uint;
                fread(
                    &raw mut langue as *mut c_void,
                    size_of::<c_short>() as c_ulong,
                    1,
                    f,
                );
                let mut loi = [0u8; 2];
                fread(
                    loi.as_mut_ptr() as *mut c_void,
                    size_of::<c_short>() as c_ulong,
                    1,
                    f,
                );
                SapZoom = f32::from_le_bytes([loi[0], loi[1], 0, 0]);
            }
            fclose(f);
        }
        f.is_null()
    }
}

/*********************************************************************
                        GetLibLangue
********************************************************************/
fn GetLibLangue(n: c_short, str_: *mut c_char) {
    GetIndString(str_ as *mut u8 as StringPtr, 128, n + 1);
    PtoCstr(str_ as *mut u8);
}

/*********************************************************************
                        VectAfficheMenuLangue
********************************************************************/
unsafe extern "C" fn VectAfficheMenuLangue(window: DialogPtr, itemid: c_short) {
    unsafe {
        let mut gbak: *mut c_void = core::ptr::null_mut();
        let mut r: Rect = Rect::default();
        let mut chaine = [0 as c_char; 255];

        AcamGetPortOffscreen(&raw mut gbak);
        CarboniseSetPortDialog(window);
        GetRectItem(window, itemid, &raw mut r);
        GetLibLangue(langue, chaine.as_mut_ptr());
        BackColor(whiteColor as c_short);
        r.right -= 14;
        TextFont(times as c_short);
        TextSize(16);
        TextFace(0);
        DoTextBox(
            chaine.as_mut_ptr(),
            strlen(chaine.as_ptr()) as c_short,
            &raw mut r,
            crate::cbridge::teJustLeft as c_short,
        );
        r.right += 14;
        InsetRect(&raw mut r, -1, -1);
        DrawArrowPopUp(&raw mut r);
        FrameRect(&raw mut r);
        AcamSetPortOffscreen(gbak);
    }
}

/******************************************************************************************
                                CreeMenuLangue
=> Menu a remplir
******************************************************************************************/
fn CreeMenuLangue(menu: MenuHandle) {
    let mut nom = [0 as c_char; 255];

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
unsafe extern "C" fn VectFrameBouton(window: DialogPtr, item: c_short) {
    let mut r: Rect = Rect::default();

    unsafe {
        GetRectItem(window, item, &raw mut r);
        DoFrameRect(&raw mut r);
    }
}

/***********************************************************************
                        DoPreferences
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoPreferences() {
    unsafe {
        let dialog: DialogPtr;
        let mut item: c_short = 0;
        let mut gbak: *mut c_void = core::ptr::null_mut();
        let mut pt: Point = Point::default();
        let mut result: c_long = 0;
        let mut menuLangue: MenuHandle;

        AcamGetPortOffscreen(&raw mut gbak);
        dialog = OuvreDialogue(CvtDefaultLangRess(137));
        if dialog.is_null() {
            AcamSetPortOffscreen(gbak);
            return;
        }
        CarboniseSetPortDialog(dialog);
        SetValBouton(dialog, ITEM_BRUITAGES, IsBruitages);
        SetValBouton(dialog, ITEM_MUSIQUES, okmuse != 0);
        SetValBouton(dialog, ITEM_POLICES_LISSEES, LissagePolice);
        DefAfficheItem(dialog, ITEM_RECT1, Some(VectFrameBouton));
        DefAfficheItem(dialog, ITEM_LANGUE, Some(VectAfficheMenuLangue));
        SetValBouton(dialog, ITEM_2_AUTO, ModeJoueur2 == JOUEUR2_AUTONOME);
        SetValBouton(dialog, ITEM_2_CLAVIER, ModeJoueur2 == JOUEUR2_CLAVIER);
        MontreSouris();
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
                    GetMouse(&raw mut pt);
                    LocalToGlobal(&raw mut pt);
                    result = PopUpMenuSelect(menuLangue, pt.v, pt.h, langue + 1);
                    DeleteMenu(ID_MENU);
                    DisposeMenu(menuLangue);
                    if result != 0 {
                        langue = (result - 1) as c_short;
                        VectAfficheMenuLangue(dialog, ITEM_LANGUE);
                    }
                }
                _ => {}
            }
            if item == ITEM_OK || item == ITEM_ANNULER {
                break;
            }
        }
        AcamSetPortOffscreen(gbak);
        if item == ITEM_OK {
            if GetValBouton(dialog, ITEM_2_AUTO) != 0 {
                ModeJoueur2 = JOUEUR2_AUTONOME;
            }
            if GetValBouton(dialog, ITEM_2_CLAVIER) != 0 {
                DoAlertSapiens(CvtDefaultLangRess(136));
                ModeJoueur2 = JOUEUR2_CLAVIER;
            }
            IsBruitages = GetValBouton(dialog, ITEM_BRUITAGES) != 0;
            okmuse = GetValBouton(dialog, ITEM_MUSIQUES) as c_uint;
            LissagePolice = GetValBouton(dialog, ITEM_POLICES_LISSEES) != 0;
            inimen(8);
            SauvePref();
            FixeMenu(CvtDefaultLangRess(0));
            FixeTailleMainWindow();
            inimen(0);
        }
        MusicInit(1);
        CloseDialogue(dialog, CvtDefaultLangRess(137));
    }
}
