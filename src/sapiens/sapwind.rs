#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/*;-----------------------------------------------------------------
;               HIRES;Passe en resolution et detourne Erreurs Disques
;-----------------------------------------------------------------------
*/
use std::ffi::c_void;
use std::os::raw::{c_char, c_float, c_long, c_short, c_ushort};
use std::process::exit;

use crate::acam::{
    AcamGetPortOffscreen, AcamInitOffscreenSystem, AcamSetPortOffscreen, BackColor, BlockMove,
    CarboniseGetCurrentPort, CarboniseGetQDRectScreenBits, CarboniseGetRectPort,
    CarboniseGetWindowPort, CarboniseInitQuickdDraw, CarboniseSetPortWindow, CarboniseSetRandSeed,
    CheckItem, CtoPstr, DisableItem, DisposHandle, DrawMenuBar, EnableItem, FAUX, ForeColor,
    GetCursor, GetIndString, GetItem, GetMHandle, GetNewMBar, MaxMem, NewCWindow, OffsetRect,
    PenNormal, PtoCstr, Random, SetMenuBar, SetRect, TickCount, VRAI,
};
use crate::cbridge::{
    DITL, DLOG, DRVR, DialogPtr, MenuHandle, WindowPtr, blackColor, bold, charCodeMask, cmdKey,
    ctrlItem, editText, everyEvent, geneva, helvetica, itemDisable, optionKey, radCtrl, statText,
    strcat, strcpy, strlen, userItem, watchCursor, whiteColor,
};
use crate::sapiens::sap3d::affich;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapcomb::{nbAvplanSpec, nbMobHerbes};
use crate::sapiens::sapgest::{InitCheminsSapiens, trgest};
use crate::sapiens::sapglob::{dervue, dicone, dzeta, flcomb, mort, palhaute};
use crate::sapiens::saplist::{DoAide, TraiteVueLaterale, inimen, pal1, pal2};
use crate::sapiens::saprel::{chrono, trobsv};
use crate::sapiens::saptxt::{CvtDefaultLangRess, InitAndCheckTexts, langue};
use crate::sapiens::sapw3d::{CalculeEtoiles, CalculeToutDegrades};
use crate::sapiens::sapwbrt::{
    AffPalette, BoxFill, ChargeFichierBrt, DoEraseRect, DoFrameRect, DoLine, DoMoveTo, DoPenSize,
    DoTextSize, DoZoomRectSiEcranReel, LibereDataMobs, SapiensCreeOffscreen, SetPathInData, TfrEcr,
    tfrdemi, tfrima, tfrmenufenetre, tfrvirt,
};
use crate::sapiens::sapwcar::InitCouleurFontes;
use crate::sapiens::sapwclav::{FixeAireNoMouseAireBasse, InstalleTimeTask, MontreSouris};
use crate::sapiens::sapwcvt::{CvtCouleurEnRgb, FabriqueLuts};
use crate::sapiens::sapwmus::{CheckChangeMusique, MusicInit, quelle_musique};
use crate::sapiens::sapwpref::{ChargePref, DoPreferences};
use crate::sapiens::{ECHELLE, STR_VRAC, TAILLE_STR_MESS_FORMATE, VUE_LATERALE};
use crate::shared::{
    Boolean, CGrafPtr, ControlHandle, EventRecord, Handle, PicHandle, RGBColor, Rect, StringPtr,
    TEHandle,
};
use crate::stubs::{
    AddResMenu, Alert, ArreteMusique, BeginUpdate, CarboniseAdapteStaticText,
    CarboniseGetDialogTextEditHandle, CarboniseGetMenuBarHeight, CarboniseInvalRect,
    CarboniseNewUserItemProc, CarboniseSetCursorArrow, CarboniseSetPortDialog,
    CarboniseValideUpdate, DisposDialog, DisposeRoutineDescriptor, DragWindow, DrawPicture,
    EndUpdate, FindWindow, FrontWindow, GetCtlValue, GetDItem, GetDialogItem, GetIText,
    GetNewDialog, GetNextEvent, GetPicture, GetResource, HLock, HUnlock, HiliteControl, HiliteMenu,
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
#[unsafe(no_mangle)]
pub static mut SapZoom: c_float = 1.0;
#[unsafe(no_mangle)]
pub static mut mainWindow: WindowPtr = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut Ecran1: CGrafPtr = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut Ecran2: CGrafPtr = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut debecr: CGrafPtr = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut BuffMouse: CGrafPtr = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut toucheClav: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobBambou: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobFace: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobFeuille: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobFleches: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobSapmob: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobAvplan: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobAvplanSpec: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mobSilex: c_short = 0;
#[unsafe(no_mangle)]
pub static mut mob3D: c_short = 0;
#[unsafe(no_mangle)]
pub static mut Fonte14: c_short = 0;
#[unsafe(no_mangle)]
pub static mut Fonte10: c_short = 0;
/* Statiques d'usage interne au module (aucun lien C) */
static mut OneString: [c_char; 200] = [0; 200];
static mut MenuReduit: Boolean = false;

static TRANS_VIDE: [u8; 1] = [0];

/**************** HIRES: Passe en resolution *******************/
pub fn hires() -> i16 {
    1
}

/// Taille en pixels (ST 320x200) de l'aire de jeu multipliée par SapZoom.
unsafe fn aire_rect(r: *mut Rect) {
    unsafe {
        SetRect(
            r,
            0,
            0,
            (320.0 * ECHELLE as f32 * SapZoom) as c_short,
            (200.0 * ECHELLE as f32 * SapZoom) as c_short,
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
pub unsafe extern "C" fn GetVersionSapiens(chaine: *mut c_char) -> Boolean {
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
unsafe fn copie_chaine(buf: &mut [c_char], s: &str) {
    let n = s.len().min(buf.len() - 1);
    for (i, b) in s.as_bytes()[..n].iter().enumerate() {
        buf[i] = *b as c_char;
    }
    buf[n] = 0;
}

/***********************************************************************************
                 Collapse
Affichage d'un message d'erreur formaté
    RMQ: Cette fonction supporte un affichage formaté identique au printf du C.
=> Message à afficher sous la forme "format", arguments
<= rien
***********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Collapse(format: *mut c_char) {
    unsafe {
        let mut gbak: CGrafPtr = core::ptr::null_mut();

        AcamGetPortOffscreen(&raw mut gbak);
        if !mainWindow.is_null() {
            CarboniseSetPortWindow(mainWindow);
        }

        /* On fixe la zone à la zone de secours de l'interface */
        if !format.is_null() {
            let mut chaine: [c_char; TAILLE_STR_MESS_FORMATE as usize] = [0; 512];
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
            DoAlertSapiens(CvtDefaultLangRess(134)); /* ALRT_COLLAPSE */
        }
        AcamSetPortOffscreen(gbak);
    }
}

/***********************************************************************************
                 Confirme
Affichage d'un message d'erreur formaté
    RMQ: Cette fonction supporte un affichage formaté identique au printf du C.
=> Message à afficher sous la forme "format", arguments
<= VRAI si confirmation
***********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Confirme(format: *mut c_char) -> Boolean {
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
            etat = DoAlertSapiens(CvtDefaultLangRess(135)); /* ALRT_CONFIRMATION */
        }
        etat == 1
    }
}

/***********************************************************************************
                        MessInit
Affichage d'un message durant l'initialisation
***********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MessInit(_format: *mut c_char) {}

/**********************************************************************************
                                SetValItem
=> Fixe une valeur type chainedans un item
<= VRAI si la valeur est bonne, FAUX si la valeur a été tronquée
**********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetValItem(
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
pub unsafe extern "C" fn SetShortItem(
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
pub extern "C" fn SetValBouton(dialog: DialogPtr, itemHit: c_short, value: Boolean) -> c_short {
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
pub unsafe extern "C" fn GetValBouton(dialog: DialogPtr, itemHit: c_short) -> c_short {
    let mut itemType: c_short = 0;
    let mut control: Handle = core::ptr::null_mut();
    let mut box_: Rect = Rect::default();
    let mut value: c_short = 0;

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
pub unsafe extern "C" fn SetActiveItem(dialog: DialogPtr, itemHit: c_short, value: Boolean) {
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
pub unsafe extern "C" fn GetValItem(
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
pub unsafe extern "C" fn GetShortItem(
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefAfficheItem(
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

/***********************************************************************************
                        GetNbItemInDialog
Suppplée à une lacune du MACOS: Va chercher en fonction de l'id d'un dialogue,
le nombre d'item dans le DITL de ceui-ci
=> ID du dialog
<= Nombre d'item ou -1 si erreur
**********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNbItemInDialog(id_dialogue: c_short) -> c_short {
    unsafe {
        let mut handle: Handle;
        let mut id_ditl: c_short = 0;

        handle = GetResource(DLOG as c_long, id_dialogue);
        if !handle.is_null() {
            id_ditl = *((*handle as *mut u8).offset(18) as *const c_short);
            /* ON libére la ressource du DLOG */
            ReleaseResource(handle);
            handle = GetResource(DITL as c_long, id_ditl);
            if !handle.is_null() {
                id_ditl = *(*handle as *const c_short);
                /* ON libére la ressource du DITL */
                ReleaseResource(handle);
                return id_ditl + 1;
            }
        }
        Collapse(c"GetNbItemInDialog:Ressource non trouvee.".as_ptr() as *mut c_char);
        -1
    }
}

/******************************************************************************************
                                    DisposeUserItems
Sur power PC à chaque user item à été alloué un UniversalProcPtr, on doit les liberer
en fermture de la boite
=> dialog,id du ditl
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisposeUserItems(_dialog: DialogPtr, _idDITL: c_short) {
    /* #ifdef powerc : corps vide sur Mac 68K */
}

/******************************************************************************************
                                CloseDialogue
Ferme la fenetre, libere le dialogue libere les ressource DITL et DLOG associées
=> Pointeur sur la fenetre a ferme, ID des ressources a liberer
<= Ptr sur le dialogue
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseDialogue(dialog: DialogPtr, idDITL: c_short) {
    unsafe {
        DisposeUserItems(dialog, idDITL);
        DisposDialog(dialog);
    }
}

/******************************************************************************************
                                    DoAlertSapiens
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoAlertSapiens(num: c_short) -> c_short {
    let n = Alert(num, core::ptr::null_mut());
    tfrvirt();
    TfrEcr(core::ptr::null_mut());
    n
}

/******************************************************************************************
                                    GetRectItem
=> Recupere le cadre d'un item donné
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRectItem(dialog: DialogPtr, item: c_short, box_: *mut Rect) {
    let mut itemType: c_short = 0;
    let mut handle: Handle = core::ptr::null_mut();

    /* On recupere l'item coorespondant à l'aire (seule l' aire nous interesse) */
    GetDItem(dialog, item, &raw mut itemType, &raw mut handle, box_);
}

/*****************************************************************************
                       DrawArrowPopUp
****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DrawArrowPopUp(r: *mut Rect) {
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
        DoEraseRect(&raw mut box_);
        while i < 6 {
            DoMoveTo((*r).right - 10 - i, y - i);
            DoLine(i * 2, 0);
            i += 1;
        }
    }
}

/******************************************************************************************
                                    GetStr
Recupere une chaine indexée dans une locale et la passe au format C
<= Ptr sur la locale
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStr(index: c_short) -> *mut c_char {
    unsafe {
        let s = &raw mut OneString as *mut c_char;
        GetIndString(
            s as *mut c_void as StringPtr,
            CvtDefaultLangRess(STR_VRAC as c_short),
            index,
        );
        PtoCstr(s as *mut u8);
        s
    }
}

/**********************************************************************
                    initrnd
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initrnd(sem1: c_short, sem2: c_short, sem3: c_short) {
    CarboniseSetRandSeed(((sem1 as c_long) << 16) + ((sem2 as c_long) << 8) + sem3 as c_long);
}

/**********************************************************************
                    rnd0
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rnd0() -> c_short {
    (Random() & 0xFF) as c_short
}

/**********************************************************************
                    Rnd
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rnd() -> c_short {
    (Random() & 0xFF) as c_short
}

/**********************************************************************
                    myRandom
Sort une valeur entre deux bornes
***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn myRandom(min: c_long, max: c_long) -> c_long {
    unsafe {
        let mut rdm: u16;
        let range: c_long;
        let t: c_long;

        rdm = (Rnd() as c_long) as u16;
        rdm = rdm.wrapping_add(((Rnd() as c_long) * 256) as u16);
        range = max - min;
        t = ((rdm as c_long) * range) / 65536;
        min + t
    }
}

/**********************************************************************
                    bande
Affiche une bande sur les cotés
***********************************************************************/
/*;********* BANDE_: 0= bande jeu, 1= bande silex *******************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bande(_quelle: c_short) {}

/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                        SAPDISK.S
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
/****************************************************************************
Boxf: trace une boite sur le port spécifié
Les  coordonnées sont données en ecran ST 320x200
(C) Ligne dep,Col dep,Ligne fin,col.fin,index couleur ST
***************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boxf(
    lignedep: c_short,
    coldep: c_short,
    lignefin: c_short,
    colfin: c_short,
    index: c_short,
) {
    unsafe {
        let mut aire: Rect = Rect::default();

        SetRect(
            &raw mut aire,
            coldep * ECHELLE as c_short,
            lignedep * ECHELLE as c_short,
            colfin * ECHELLE as c_short,
            lignefin * ECHELLE as c_short,
        );
        BoxFill(
            CarboniseGetCurrentPort(),
            &raw mut aire,
            *palhaute.offset(index as isize),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn irqon() {}

/*+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
               ECRAN64.S
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
/*;------------------------------------------------------
;               CADREG:Trace un cadre graphique en pointille
;-------------------------------------------------------
;=> numero ligne pixel deut *2,col octet,n ligne/2,n col
;<= sur ecran cadre xor
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cadreg(_ligne: c_short, _col: c_short, _nligne: c_short, _ncol: c_short) {}

/******************************************************************************************
                                AdjustMenus
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdjustMenus() {
    unsafe {
        let mut menu: MenuHandle;
        let mut gbak: CGrafPtr = core::ptr::null_mut();

        AcamGetPortOffscreen(&raw mut gbak);
        CarboniseSetPortWindow(mainWindow);

        menu = GetMHandle(CvtDefaultLangRess(128)); /* menuPomme */
        menu = GetMHandle(CvtDefaultLangRess(129)); /* menuFichier */
        if MenuReduit {
            DisableItem(menu, 0);
        } else {
            EnableItem(menu, 0);
        }

        menu = GetMHandle(CvtDefaultLangRess(130)); /* menuVoir */
        if MenuReduit || mort == 0 {
            DisableItem(menu, 0);
        } else {
            EnableItem(menu, 0);
        }
        menu = GetMHandle(CvtDefaultLangRess(131)); /* menuFenetre */
        CheckItem(menu, iEchelle100 as c_short, SapZoom == 1.0);
        CheckItem(menu, iEchelle200 as c_short, SapZoom == 2.0);
        CheckItem(menu, iEchelle300 as c_short, SapZoom == 3.0);
        CheckItem(menu, iEchelle400 as c_short, SapZoom == 4.0);
        DrawMenuBar();
        AcamSetPortOffscreen(gbak);
    }
}

/******************************************************************************************
                                FixeMenusReduit
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixeMenusReduit() {
    unsafe {
        if !MenuReduit {
            MenuReduit = VRAI;
            AdjustMenus();
        }
    }
}

/******************************************************************************************
                                FixeMenusComplet
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixeMenusComplet() {
    unsafe {
        if MenuReduit {
            MenuReduit = FAUX;
            AdjustMenus();
        }
    }
}

/******************************************************************************************
                                ConfirmePerteAventure
******************************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn ConfirmePerteAventure() -> Boolean {
    MontreSouris();
    unsafe { DoAlertSapiens(CvtDefaultLangRess(1)) == 1 } /* ALRT_PERTE_PARTIE */
}

/******************************************************************************************
                                DoInfo
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoInfo() {
    unsafe {
        let mut str_: [c_char; 255] = [0; 255];

        GetVersionSapiens(&raw mut str_ as *mut c_char);
        CtoPstr(str_.as_mut_ptr());
        ParamText(
            &raw mut str_ as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
            TRANS_VIDE.as_ptr() as *mut u8,
        );
        DoAlertSapiens(CvtDefaultLangRess(2)); /* ALRT_INFO */
    }
}

/******************************************************************************************
                                DoMenuCommand
******************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMenuCommand(menuResult: c_long) {
    unsafe {
        let menuID: c_short;
        let menuItem: c_short;
        let mut daName: [u8; 256] = [0; 256];

        AdjustMenus();
        MontreSouris();
        menuID = ((menuResult >> 16) & 0xFFFF) as c_short; /* HiWord */
        menuItem = (menuResult & 0xFFFF) as c_short; /* LoWord */
        if menuID == CvtDefaultLangRess(128) {
            /* menuPomme */
            match menuItem {
                itemInformation => DoInfo(),
                itemAide => DoAide(),
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
                itemNouveau => trgest(3),
                itemCharger => {
                    trgest(1);
                    inimen(0);
                }
                itemSauver => trgest(2),
                itemPreferences => DoPreferences(),
                itemQuitter => {
                    if ConfirmePerteAventure() {
                        SapiensExit(0);
                    }
                }
                _ => {}
            }
        } else if menuID == CvtDefaultLangRess(130) {
            /* menuVoir */
            match menuItem {
                iPaysage => {
                    trobsv(0);
                    if flcomb != 0 {
                        TraiteVueLaterale();
                    }
                }
                iLaterale => {
                    trobsv(1);
                    TraiteVueLaterale();
                }
                iPanorama => trobsv(2),
                iCarte => trobsv(3),
                _ => {}
            }
        } else if menuID == CvtDefaultLangRess(131) {
            /* menuFenetre */
            match menuItem {
                iEchelle100 => SapZoom = 1.0,
                iEchelle200 => SapZoom = 2.0,
                iEchelle300 => SapZoom = 3.0,
                iEchelle400 => SapZoom = 4.0,
                _ => {}
            }
            FixeTailleMainWindow();
        }
        HiliteMenu(0);
        AdjustMenus();
        DrawMenuBar();
    }
}

/*******************************************************************
                        DoEvtClavier
Gestion du clavier
=> evenement a traiter
********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoEvtClavier(event: *const EventRecord) {
    unsafe {
        let ev = &*event;
        let mut chaine: [c_char; 255] = [0; 255];
        let mut t: c_long;

        if ev.modifiers & cmdKey as c_short != 0 {
            let _car = (ev.message & 0xFF) as c_short;
            AdjustMenus();
            DoMenuCommand(MenuKey((ev.message & charCodeMask as i64) as c_short));
        } else if ev.modifiers & optionKey as c_short != 0 {
            let mut i: c_short = 0;
            let car: c_short = ((ev.message & 0xFF00) >> 8) as c_short;
            if car == 0x11
            /*T*/
            {
                /* Transfert Ecran */
                t = TickCount();
                i = 0;
                while i < 200 {
                    TfrEcr(core::ptr::null_mut());
                    i += 1;
                }
                copie_chaine(
                    &mut chaine,
                    &format!(
                        "{i} transferts totaux de l'ecran : {} ticks",
                        TickCount() - t
                    ),
                );
                Collapse(&raw mut chaine as *mut c_char);
            }
            if car == 0x02
            /*'D'*/
            {
                /* 3d */
                t = TickCount();
                i = 0;
                while i < 50 {
                    affich(dzeta as c_ushort);
                    i += 1;
                }
                copie_chaine(
                    &mut chaine,
                    &format!("{i} images 3d : {} ticks", TickCount() - t),
                );
                Collapse(&raw mut chaine as *mut c_char);
            }
            if car == 0x23
            /*'P'*/
            {
                AffPalette();
            }
            if car == 0x29
            /*'M'*/
            {
                if quelle_musique == 1 {
                    MusicInit(2);
                } else {
                    MusicInit(1);
                }
            }
            if car == 0x06
            /*'T'*/
            {
                i = 0;
                while i < 64 {
                    chrono(1);
                    i += 1;
                }
            }
            if car == 0x0f
            /*R*/
            {
                InitMobs();
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
                DoAide();
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
            CvtCouleurEnRgb(palette, i, &raw mut rgb);
            *palette.offset((16 + i) as isize) = SapColor2Index(&raw mut rgb);
            i += 1;
        }
    }
}

/*******************************************************************
                        FixeMenu
=> Numero de la menu Bar à installer, Numero du menu Pomme de cette menuBar
********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixeMenu(n: c_short) -> Boolean {
    unsafe {
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
}

/******************************************************************
                    InitMobs
Charge ou recharge les mobs
******************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMobs() -> Boolean {
    unsafe {
        let mut gbak: CGrafPtr = core::ptr::null_mut();
        let mut nb: c_short;

        AcamGetPortOffscreen(&raw mut gbak);
        AcamSetPortOffscreen(Ecran1);

        LibereDataMobs();
        PushPath();
        SetPathInData(c"Mobs".as_ptr() as *mut c_char);
        mobBambou = 0;
        nb = ChargeFichierBrt(c"WBAMBOU.BRT".as_ptr() as *mut c_char, mobBambou + 1, 26, 0); /*0, 21 Mobs */
        if nb < 0 {
            return FAUX;
        }
        mobFace = mobBambou + nb;
        nb = ChargeFichierBrt(c"WFACE.BRT".as_ptr() as *mut c_char, mobFace + 1, 31, 0); /* Base 21,29 mobs */
        if nb < 0 {
            return FAUX;
        }
        mobFeuille = mobFace + nb;
        nb = ChargeFichierBrt(
            c"WFEUILLE.BRT".as_ptr() as *mut c_char,
            mobFeuille + 1,
            36,
            11,
        ); /* Base 50 36 mobs */
        if nb < 0 {
            return FAUX;
        }
        mobFleches = mobFeuille + nb;
        nb = ChargeFichierBrt(
            c"WFLECHES.BRT".as_ptr() as *mut c_char,
            mobFleches + 1,
            20,
            0,
        ); /* Base 8618 mobs */
        if nb < 0 {
            return FAUX;
        }
        mobSapmob = mobFleches + nb;
        nb = ChargeFichierBrt(
            c"WSAPMOB.BRT".as_ptr() as *mut c_char,
            mobSapmob + 1,
            141,
            0,
        ); /*Base 120+1,151 mobs */
        if nb < 0 {
            return FAUX;
        }
        mobAvplan = mobSapmob + nb;
        nb = ChargeFichierBrt(c"WAVPLAN.BRT".as_ptr() as *mut c_char, mobAvplan + 1, 9, 13);
        nbMobHerbes = nb;
        if nb < 0 {
            return FAUX;
        }
        mobAvplanSpec = mobAvplan + nb;
        nb = ChargeFichierBrt(
            c"WAVSPEC.BRT".as_ptr() as *mut c_char,
            mobAvplanSpec + 1,
            4,
            0,
        );
        nbAvplanSpec = nb;
        if nb < 0 {
            return FAUX;
        }
        mobSilex = mobAvplanSpec + nb;
        nb = ChargeFichierBrt(c"WSILEX.BRT".as_ptr() as *mut c_char, mobSilex + 1, 15, 0);
        if nb < 0 {
            return FAUX;
        }
        mob3D = mobSilex + nb;
        nb = ChargeFichierBrt(c"W3D.BRT".as_ptr() as *mut c_char, mob3D + 1, 38, 0);
        if nb < 0 {
            return FAUX;
        }
        Fonte14 = mob3D + nb;
        nb = ChargeFichierBrt(c"Geneva14.BRT".as_ptr() as *mut c_char, Fonte14 + 1, 120, 0);
        if nb < 0 {
            return FAUX;
        }
        Fonte10 = Fonte14 + nb;
        nb = ChargeFichierBrt(c"Geneva10.BRT".as_ptr() as *mut c_char, Fonte10 + 1, 120, 0);
        if nb < 0 {
            return FAUX;
        }
        AcamSetPortOffscreen(gbak);
        PopPath();
        VRAI
    }
}

/***********************************************************************************
                                 AfficheFond
Affiche le tramage de fond de la fenetre
***********************************************************************************/
unsafe fn AfficheFond(port: CGrafPtr) {
    let mut r: Rect = Rect::default();
    let mut gbak: CGrafPtr = core::ptr::null_mut();
    let mut portRect: Rect = Rect::default();

    AcamGetPortOffscreen(&raw mut gbak);
    AcamSetPortOffscreen(port);
    CarboniseGetRectPort(port, &raw mut portRect);
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
    AcamSetPortOffscreen(gbak);
}

/***********************************************************************************
                                 FixeTitreMainWindow
***********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixeTitreMainWindow() {
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
        CarboniseSetPortWindow(mainWindow);
        SetWTitle(mainWindow, &raw mut chaine as *mut u8);
    }
}

/****************************************************************************
            Encadre l'aire 3d
=> etat 1 actif
*****************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Frame3d(etat: c_short) {
    unsafe {
        let mut r: Rect = Rect::default();
        let mut gbak: CGrafPtr = core::ptr::null_mut();

        AcamGetPortOffscreen(&raw mut gbak);
        AcamSetPortOffscreen(debecr);
        SetRect(&raw mut r, 62, 0, 640 - 64, 204);
        if etat != 0 {
            ForeColor(blackColor as c_short);
        } else {
            ForeColor(whiteColor as c_short);
        }
        DoPenSize(2, 2);
        DoFrameRect(&raw mut r);
        ForeColor(blackColor as c_short);
        PenNormal();
        CarboniseSetPortWindow(mainWindow);
        if etat != 0 {
            ForeColor(blackColor as c_short);
        } else {
            ForeColor(whiteColor as c_short);
        }
        DoPenSize(2, 2);
        DoFrameRect(&raw mut r);
        PenNormal();
        ForeColor(blackColor as c_short);
        AcamSetPortOffscreen(gbak);
    }
}

/**********************************************************************************
                    MenuFenetres
Entre ou sort du menu fenetre actif
=> etat =1 si actif
***********************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MenuFenetres(etat: c_short) {
    unsafe {
        Frame3d(etat);
        tfrmenufenetre(etat);
        if etat != 0 {
            FixeMenusComplet();
        } else {
            FixeMenusReduit();
        }
        dicone = -1;
    }
}

/**********************************************************************************************
                                            ValideUpdate
***********************************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValideUpdate() {
    CarboniseValideUpdate();
}

/**********************************************************************************************
                                                                AffStr
***********************************************************************************************/
unsafe fn AffStr(
    chaine: *mut c_char,
    fonte: c_short,
    size: c_short,
    face: c_short,
    x_in: c_short,
    y: c_short,
) {
    unsafe {
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
        aire_rect(&raw mut r);
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
pub unsafe extern "C" fn OuvreDialogue(numDitl: c_short) -> DialogPtr {
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

/*******************************************************************
                        InitMainWindow
********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMainWindow() {
    unsafe {
        let mut rect: Rect = Rect::default();
        let mut aireDevice: Rect = Rect::default();
        let largeur = (320.0 * ECHELLE as f32 * SapZoom) as c_long;
        let hauteur = (200.0 * ECHELLE as f32 * SapZoom) as c_long;

        AcamInitOffscreenSystem(&raw mut aireDevice);
        /* Creation de la fenetre principale */
        /* Aire de la fenetre */
        SetRect(
            &raw mut rect,
            0,
            0,
            (320.0 * ECHELLE as f32 * SapZoom) as c_short,
            (200.0 * ECHELLE as f32 * SapZoom) as c_short,
        );
        /* On la centre sur l'aire generale de l'ecran */
        OffsetRect(
            &raw mut rect,
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
        mainWindow = NewCWindow(
            core::ptr::null_mut(),
            &raw const rect,
            core::ptr::null_mut(),
            VRAI,
            noGrowDocProc,
            -1isize as WindowPtr,
            FAUX,
            0,
        );
    }
}

/*******************************************************************
                        init_graph
********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_graph() -> Boolean {
    unsafe {
        let mut i: c_short;
        let mut y: c_short;
        let num: c_short;
        let mut pict: PicHandle = core::ptr::null_mut();
        let mut r: Rect = Rect::default();
        let mut rdest: Rect = Rect::default();
        let mut chaine: [c_char; 255] = [0; 255];
        let timeBase: c_long;
        let firstInit: Boolean;

        CarboniseInitQuickdDraw();
        /* Init des chemins standards */
        InitCheminsSapiens();
        SetCursor(GetCursor(watchCursor as c_short));
        FixeAireNoMouseAireBasse();
        /* On charge les preferences */
        firstInit = ChargePref();
        InitDefautConfigSortieNumerique();
        MusicInit(1); /* Lance musique 1 */
        /* Choix de la langue*/
        if langue == -1 {
            CarboniseSetCursorArrow();
            langue = Alert(ALRT_VERSION, core::ptr::null_mut()) - 1;
            SetCursor(GetCursor(watchCursor as c_short));
        }
        InitAndCheckTexts();
        InitMainWindow();
        FixeTitreMainWindow();
        CarboniseSetPortWindow(mainWindow);
        AfficheFond(CarboniseGetWindowPort(mainWindow));
        /* On affiche le logo centre */
        pict = GetPicture(134) as PicHandle;
        if !pict.is_null() {
            r = (**pict).picFrame;
            OffsetRect(
                &raw mut r,
                ((320 * ECHELLE as c_short as c_long - (r.right - r.left) as c_long) / 2)
                    as c_short,
                60,
            );
            DoZoomRectSiEcranReel(&raw mut r, &raw mut rdest);
            DrawPicture(pict, &raw const rdest as *const Rect);
            ReleaseResource(pict as Handle);
        }
        /* Affichage de la version de sapiens */
        GetVersionSapiens(&raw mut chaine as *mut c_char);
        AffStr(
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
            c"Copyright Didier & Olivier Guillion, 1986,1996,2005,2017".as_ptr() as *const c_char,
        );
        AffStr(
            &raw mut chaine as *mut c_char,
            helvetica as c_short,
            16,
            bold as c_short,
            -1,
            r.bottom + 32,
        );
        i = 1;
        y = 60 + 120 + 18 + 32;
        num = CvtDefaultLangRess(1);
        loop {
            GetIndString(
                &raw mut chaine as *mut u8 as *mut c_void as StringPtr,
                num,
                i,
            );
            i += 1;
            PtoCstr(&raw mut chaine as *mut u8);
            if strlen(&raw const chaine as *const c_char) != 0 {
                AffStr(
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
        timeBase = TickCount() + 5 * 60;

        aire_rect(&raw mut r);
        CarboniseInvalRect(&raw const r);
        ValideUpdate();
        /* On initialise le menu */
        MessInit(GetStr(26)); /* Initialisation du menu */
        if !FixeMenu(CvtDefaultLangRess(0)) {
            /* MENU_BAR */
            Collapse(GetStr(11) /*"Reinstallez le logiciel SVP"*/);
            SapiensExit(-1);
        }
        FixeMenusReduit();
        CarboniseSetPortWindow(mainWindow);
        MessInit(GetStr(20)); /* Initialsations des ecrans */
        CalcIndexPalette(&raw mut pal1 as *mut c_short);
        CalcIndexPalette(&raw mut pal2 as *mut c_short);
        Ecran1 = SapiensCreeOffscreen(256 * ECHELLE as c_short, 100 * ECHELLE as c_short);
        if Ecran1.is_null() {
            Collapse(c"[SapWind.init_graph.1] Unable to create offscreen".as_ptr() as *mut c_char);
            SapiensExit(-2);
        }
        Ecran2 = SapiensCreeOffscreen(256 * ECHELLE as c_short, 100 * ECHELLE as c_short);
        AcamSetPortOffscreen(Ecran1);
        MessInit(GetStr(21)); /* Initialsations des palettes */
        /* Fabrique les luts */
        FabriqueLuts();
        debecr = SapiensCreeOffscreen(320 * ECHELLE as c_short, 200 * ECHELLE as c_short);
        AcamSetPortOffscreen(debecr);
        AfficheFond(debecr);
        /* Buffer de la souris*/
        BuffMouse = SapiensCreeOffscreen(64 * ECHELLE as c_short, 64 * ECHELLE as c_short);
        MessInit(GetStr(22)); /*Chargement des graphismes */
        /* On charge les mobs */
        if !InitMobs() {
            Collapse(GetStr(11) /*"Reinstallez le logiciel SVP"*/);
            SapiensExit(-1);
        }
        InitCouleurFontes();
        MessInit(GetStr(23)); /*Chargement des dégradés */
        /* On calcule tout les degrades*/
        CalculeToutDegrades();
        MessInit(GetStr(24)); /*Chargement des décors trois 3 */
        CalculeEtoiles();
        MessInit(GetStr(25)); /* Initialisation de la musique */
        /* Initialisation de la musique */
        while timeBase > TickCount() {
            core::hint::spin_loop();
        }
        /* Installation du timer de comptage de frame */
        InstalleTimeTask();
        /* Transfert de l'ecran */
        CarboniseSetCursorArrow();
        firstInit
    }
}

/*******************************************************************
                        GereInterface
Gestion générale de l'interface
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn GereInterface() {
    let mut part;
    let mut windowResult = WindowPtr::default();
    let mut gbak = CGrafPtr::default();
    let mut event = EventRecord::default();
    let mut bounds = Rect::default();

    unsafe {
        AcamGetPortOffscreen(&mut gbak);
        CarboniseSetPortWindow(mainWindow);
        let window = FrontWindow();
        CheckChangeMusique();
        while GetNextEvent(everyEvent as i16, &mut event) {
            match event.what as u32 {
                keyDown => {
                    toucheClav = (event.message as c_short & charCodeMask as c_short) as c_short;
                    DoEvtClavier(&raw const event as *const _);
                }
                mouseDown => {
                    /* On cherche quelle fenetre a été cliquée */
                    part = FindWindow(event.where_, &mut windowResult);
                    /* Où a-t'on cliqué ??? */
                    match part as u32 {
                        /* Dans la barre de menu , alors on le traite */
                        inMenuBar => {
                            DoMenuCommand(MenuSelect(event.where_));
                        }
                        /* Sur le bureau, alors le bureau se débrouille ... sauf si c'est un pseudo document */
                        inSysWindow => {
                            SystemClick(&event, windowResult);
                        }
                        /* Dans le deplacement */
                        inDrag => {
                            SelectWindow(windowResult);
                            CarboniseGetQDRectScreenBits(&mut bounds);
                            DragWindow(windowResult, event.where_, &bounds);
                        }
                        /* Dans une fenetre : alors on le traite */
                        inContent => {
                            if windowResult != window {
                                SelectWindow(windowResult);
                            }

                            //if(window==mainWindow)
                            //       DoClick(event.where);
                        }
                        _ => {}
                    }
                }
                updateEvt => {
                    if window == mainWindow {
                        //                    AcamGetPortOffscreen(&gbak);
                        CarboniseSetPortWindow(window);
                        BeginUpdate(window);
                        TfrEcr(std::ptr::null_mut());
                        tfrima();
                        if dervue == VUE_LATERALE as u8 {
                            tfrdemi();
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
                        MontreSouris();
                    }
                }
                _app4Evt => {
                    if (event.message >> 24) == 1 {
                        if (event.message & 1) > 0 {
                            /* on reprends le traitement */
                            CarboniseSetPortWindow(mainWindow);
                            TfrEcr(std::ptr::null_mut());
                            tfrima();
                            if dervue == VUE_LATERALE as u8 {
                                tfrdemi();
                            }
                        } else {
                            /* On est arrêté */
                            MontreSouris();
                        }
                    }
                }
                _ => {}
            }
        }
    }
    SystemTask();
    AcamSetPortOffscreen(gbak);
}

/*******************************************************************
                        delai
=> Nbre de 60eme de seocndes a atteindre
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn delai(duree: c_long) {
    let t = TickCount() + duree;

    loop {
        GereInterface();
        if t <= TickCount() {
            break;
        }
    }
}

/*******************************************************************
                        FixeTailleMainWindow
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn FixeTailleMainWindow() {
    let mut gbak = CGrafPtr::default();
    let mut rect = Rect::default();

    AcamGetPortOffscreen(&mut gbak);
    unsafe {
        AcamSetPortOffscreen(CarboniseGetWindowPort(mainWindow));
        SetRect(
            &mut rect,
            0,
            0,
            (320.0 * ECHELLE as f32 * SapZoom) as i16,
            (200.0 * ECHELLE as f32 * SapZoom) as i16,
        );
        SizeWindow(mainWindow, rect.right, rect.bottom, true);
    }
    CarboniseInvalRect(&rect);
    AcamSetPortOffscreen(gbak);
}

/*******************************************************************
                        SapiensExit
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn SapiensExit(code: c_short) {
    ArreteMusique(true);
    exit(code as i32);
}

/*EOF*/
