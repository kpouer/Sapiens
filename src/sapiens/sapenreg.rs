#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::acam::{AcamGetPortOffscreen, AcamSetPortOffscreen, FAUX, VRAI};
use crate::cbridge::{
    Confirme, DialogPtr, ModalFilterUPP, autoKey, charCodeMask, keyDown, memset, remove, strcat,
    strcpy, strlen,
};
use crate::nomJoueur;
use crate::sapiens::sapbonc::DoPrintBonCommande;
use crate::sapiens::saptxt::nomjoueur;
use crate::sapiens::sapwclav::MontreSouris;
use crate::sapiens::sapwind::{
    CloseDialogue, Collapse, DoAlertSapiens, FixeTitreMainWindow, GetStr, GetValItem, OuvreDialogue,
};
use crate::sapiens::sapwpref::{SauvePref, SetPathPreferences};
use crate::sapiens::{
    ALRT_ENREG, ALRT_NO_FONCTION, ALRT_OK_ENREG, DLOG_NUM_SERIE, NOM_PREF, TAILLE_MAX_NOM,
    TAILLE_MAX_NOM_USER,
};
use crate::shared::{Boolean, CGrafPtr, EventRecord};
use crate::stubs::{
    CarboniseGetDialogKeyboardFocusItem, CarboniseSetCursorArrow, CarboniseSetPortDialog,
    DisposeRoutineDescriptor, DlgCut, ModalDialog, PreservePath, RestorePath, SelIText, numSerie,
};
use log::warn;
use std::ffi::CStr;
use std::os::raw::{c_char, c_short, c_ulong, c_ushort, c_void};

const TabCvt1: &[u8; 16] = b"AEUTNZPFSRDGJKLM";
const TabCvt2: &[u8; 16] = b"123456789-XWZTNH";

fn Code(base: u16) -> u16 {
    let mut d0 = base ^ 0x2867;
    let mut d1 = d0;
    d0 = ((d0 & 15) << 12) + (d0 >> 4);
    d0 ^= 6;
    d0 ^= d1;
    d1 = d0;
    d0 = ((d0 & 15) << 12) + (d0 >> 4);
    d0 ^= 9;
    d0 ^= d1;
    d1 = d0;
    d0 = ((d0 & 15) << 12) + (d0 >> 4);
    d0 ^= 11;
    d0 ^= d1;
    d0
}

pub extern "C" fn VerifNumSerie(str: *mut ::std::os::raw::c_char) -> Boolean {
    let str = unsafe { CStr::from_ptr(str).to_bytes() };
    if str.len() < 9 {
        warn!("Invalid serial number length {}", str.len());
        return false;
    }
    let mut clef: c_ushort = 0;
    let mut base: c_ushort = 0;

    for i in (0..8usize).step_by(2) {
        let n = str[i];
        let j = match TabCvt1.iter().position(|x| *x == n) {
            None => return false,
            Some(j) => j,
        };
        clef = (clef << 4) + j as c_ushort;
        let n = str[i + 1];
        let j = match TabCvt2.iter().position(|x| *x == n) {
            None => return false,
            Some(j) => j,
        };
        base = (base << 4) + j as c_ushort;
    }
    let n = clef;
    clef = Code(base);
    if clef != n {
        return false;
    }
    if str[8] != 'S' as u8 {
        return false;
    }
    true
}

/**********************************************************************************
                                        IsEnreg
Teste si l'exemplaire est enregistr�
<= VRAI si l'exemplaire est enregistr�
***********************************************************************************/
pub extern "C" fn IsEnreg() -> Boolean {
    /* On extrait le nom et le numero de serie */
    unsafe {
        if strlen(&raw const nomJoueur!() as *const _) == 0
            || strlen(&raw const numSerie as *const _) == 0
        {
            return FAUX;
        }
        VerifNumSerie(&raw mut numSerie as *mut _)
    }
}

const I_OK: c_short = 1;
const I_ANNULER: c_short = 2;
const I_NOM: c_short = 3;
const I_SERIE: c_short = 4;

/***********************************************************************************
                                                dlgFilter
***********************************************************************************/
unsafe extern "C" fn dlgFilter(
    dlg: DialogPtr,
    event: *mut EventRecord,
    _item: *mut c_short,
) -> Boolean {
    let mut gbak: CGrafPtr = core::ptr::null_mut();
    let mut car: u8;
    let mut result: Boolean = FAUX;

    AcamGetPortOffscreen(&raw mut gbak);
    CarboniseSetPortDialog(dlg);
    unsafe {
        let ev = *event;
        if ev.what as u32 == keyDown || ev.what as u32 == autoKey {
            car = (ev.message & charCodeMask as i64) as u8;
            let mut message = ev.message;
            if CarboniseGetDialogKeyboardFocusItem(dlg) + 1 == I_SERIE {
                /* Passe le numero de serie en majuscule */
                if car >= b'a' {
                    car &= 223;
                    message = (message & 0xFFFFFF00) | (car as i64);
                }
            }
            /* On elimine les retour chariot */
            if car == 3 || car == 13 {
                result = VRAI;
            }
            /* ESC efface le champs */
            if car == 27 {
                SelIText(dlg, CarboniseGetDialogKeyboardFocusItem(dlg) + 1, 0, -32768);
                DlgCut(dlg);
                result = VRAI;
            }
            (*event).message = message;
        }
    }
    AcamSetPortOffscreen(gbak);
    result
}

/**********************************************************************************
                                        SaisieEnregistrement
Demande à l'utilisateur de saisir son numero d'enregistrement et son nom
***********************************************************************************/
pub extern "C" fn SaisieEnregistrement() -> Boolean {
    unsafe {
        let dialog: DialogPtr;
        let mut item: c_short = 0;
        let mut str_buf: [c_char; 255] = [0; 255];
        let mut chaine: [c_char; 255] = [0; 255];
        let mut gbak: CGrafPtr = core::ptr::null_mut();
        let modalUPP: ModalFilterUPP;

        AcamGetPortOffscreen(&raw mut gbak);
        dialog = OuvreDialogue(DLOG_NUM_SERIE as c_short);
        if dialog.is_null() {
            AcamSetPortOffscreen(gbak);
            return FAUX;
        }
        modalUPP = dlgFilter as *const () as *mut c_void;
        loop {
            ModalDialog(modalUPP, &raw mut item);
            if item == I_OK {
                GetValItem(dialog, I_SERIE, &raw mut str_buf as *mut c_char);
                let n = strlen(&raw const str_buf as *const c_char) as usize;
                for i in 0..n {
                    if str_buf[i] >= b'a' as i8 {
                        str_buf[i] &= -33;
                    }
                }
                if !VerifNumSerie(&raw mut str_buf as *mut c_char) {
                    SelIText(dialog, I_SERIE, 0, 32767);
                    Collapse(GetStr(4)); /* "Numéro d'enregistrement invalide" */
                    item = 0;
                } else {
                    GetValItem(dialog, I_NOM, &raw mut str_buf as *mut c_char);
                    if strlen(&raw const str_buf as *const c_char) == 0 {
                        item = 0;
                        Collapse(GetStr(5)); /* "Vous devez entrer un nom" */
                    } else {
                        if strlen(&raw const str_buf as *const c_char)
                            > (TAILLE_MAX_NOM - 1) as c_ulong
                        {
                            item = 0;
                            Collapse(GetStr(6)); /* "Le nom est trop long" */
                        }
                    }
                    SelIText(dialog, I_NOM, 0, 32767);
                }
                if item != 0 {
                    strcpy(&raw mut chaine as *mut c_char, GetStr(7)); /* Confirmez-vous l'enregistrement sous le nom " */
                    GetValItem(dialog, I_NOM, &raw mut str_buf as *mut c_char);
                    strcat(
                        &raw mut chaine as *mut c_char,
                        &raw mut str_buf as *mut c_char,
                    );
                    strcat(
                        &raw mut chaine as *mut c_char,
                        c"' ?".as_ptr() as *mut c_char,
                    );
                    if !Confirme(&raw mut chaine as *mut c_char) {
                        item = 0;
                    }
                }
            }
            if item == I_OK || item == I_ANNULER {
                break;
            }
        }
        DisposeRoutineDescriptor(modalUPP);
        if item == I_OK {
            GetValItem(dialog, I_SERIE, &raw mut numSerie as *mut c_char);
            let ps = &raw mut numSerie as *mut c_char;
            let mut k = strlen(ps) as usize;
            for i in 0..k {
                if *ps.add(i) >= b'a' as i8 {
                    *ps.add(i) &= -33;
                }
            }
            GetValItem(dialog, I_NOM, &raw mut nomjoueur as *mut c_char);
            *ps.add((TAILLE_MAX_NOM_USER - 1) as usize) = 0; /* On limite le nom à 40 caractères */
            let pj = &raw mut nomjoueur as *mut c_char;
            *pj.add((TAILLE_MAX_NOM_USER - 1) as usize) = 0; /* On limite le nom à 40 caractères */
            k = strlen(pj) as usize;
            for i in 0..k {
                if *pj.add(i) == b' ' as i8 {
                    *pj.add(i) = b'_' as i8;
                }
            }
            SauvePref();
            FixeTitreMainWindow();
            DoAlertSapiens(ALRT_OK_ENREG as c_short);
        }
        CloseDialogue(dialog, DLOG_NUM_SERIE as c_short);
        AcamSetPortOffscreen(gbak);
        item == I_OK
    }
}

/******************************************************************************************
                                Desenregistre
******************************************************************************************/
pub extern "C" fn Desenregistre() {
    unsafe {
        if Confirme(GetStr(14)) {
            PreservePath();
            SetPathPreferences();
            remove(NOM_PREF.as_ptr() as *const c_char);
            memset(
                &raw mut numSerie as *mut c_char as *mut c_void,
                0,
                TAILLE_MAX_NOM_USER as c_ulong,
            );
            memset(
                &raw mut nomjoueur as *mut c_char as *mut c_void,
                0,
                TAILLE_MAX_NOM_USER as c_ulong,
            );
            FixeTitreMainWindow();
            RestorePath();
        }
    }
}

/******************************************************************************************
                                NoEnreg
******************************************************************************************/
pub extern "C" fn NoEnreg() {
    unsafe {
        if DoAlertSapiens(ALRT_NO_FONCTION as c_short) == 2 {
            CheckEnreg();
        }
    }
}

/******************************************************************************************
                                CheckEnreg
******************************************************************************************/
pub extern "C" fn CheckEnreg() {
    let mut cont: Boolean = VRAI;

    unsafe {
        MontreSouris();
        CarboniseSetCursorArrow();
        while cont {
            match DoAlertSapiens(ALRT_ENREG as c_short) {
                1 => {
                    /* Pas encore */
                    cont = FAUX;
                }
                2 => {
                    if SaisieEnregistrement() {
                        cont = FAUX;
                    }
                }
                3 => DoPrintBonCommande(),
                _ => {}
            }
        }
    }
}

/*EOF*/
