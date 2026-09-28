#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/******************************************************************************************
                            sapbonc.rs
                        Impression d'un bon de commande ressources TEXT 0
                        Dg le 16 aout 1995
****************************************************************************************/
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_short, c_void};

use crate::acam::{
    BlockMove, DisposHandle, FAUX, FrameRect, GetHandleSize, GetIndString, NewHandleClear, PtoCstr,
    SetHandleSize, VRAI,
};
use crate::cbridge::{
    DateTimeRec, DialogPtr, ModalFilterUPP, RSRC_TEXT_CARTE_ENREG, THPrint, TPPrPort, TPrStatus,
    TPrint, autoKey, charCodeMask, courier, keyDown, noErr, sprintf, strcat, strlen, strncmp,
    version,
};
use crate::langue::Langue::FR;
use crate::sapiens::sapwind::{
    DefAfficheItem, GetRectItem, GetValBouton, GetValItem, GetVersionSapiens, OuvreDialogue,
    SetValBouton,
};
use crate::sapiens::{DLOG_COORDONNEES, iPrAbort};
use crate::shared::{Boolean, EventRecord, Handle, Ptr, Rect, TEHandle};
use crate::stubs::{
    CarboniseGetDialogKeyboardFocusItem, CarboniseSetCursorArrow, CarboniseSetPatBlack,
    CarboniseShowDialog, CurResFile, DetachResource, DisposeRoutineDescriptor, DlgCut, GetResource,
    GetTime, HLock, HUnlock, ModalDialog, PrClose, PrCloseDoc, PrClosePage, PrError, PrJobDialog,
    PrOpen, PrOpenDoc, PrOpenPage, PrPicFile, PrSetError, PrStlDialog, PrintDefault, SelIText,
    TEInsert, TENew, TEUpdate, UseResFile,
};
use crate::text::{TextFont, TextSize};
use crate::{Sapiens, port};

const TAILLE_STR: usize = 128;
const TAILLE_STR_REM: usize = 255;

#[repr(C)]
struct Coordonnees {
    version: [c_char; TAILLE_STR],
    date: [c_char; TAILLE_STR],
    nom: [c_char; TAILLE_STR],
    adresse: [c_char; TAILLE_STR],
    codePostal: [c_char; TAILLE_STR],
    ville: [c_char; TAILLE_STR],
    pays: [c_char; TAILLE_STR],
    eMail: [c_char; TAILLE_STR],
    isProchaineVersion: Boolean,
    isLiquide: Boolean,
    trouve: [c_char; TAILLE_STR],
    remarque: [c_char; TAILLE_STR_REM],
}

const i_imprimer: c_short = 1;
const i_annuler: c_short = 2;
const i_nom: c_short = 3;
const i_adresse: c_short = 4;
const i_codePostal: c_short = 5;
const i_Ville: c_short = 6;
const i_Pays: c_short = 7;
const i_Email: c_short = 8;
const i_normal: c_short = 9;
const i_disque: c_short = 10;
const i_liquide: c_short = 11;
const i_cheque: c_short = 12;
const i_trouve: c_short = 13;
const i_remarque: c_short = 14;
const i_rect1: c_short = 15;
const i_rect2: c_short = 16;

const STR_COMMENTAIRE: c_short = 100;

pub(crate) struct Sapbonc {
    commentaire: [c_char; 255],
}

impl Default for Sapbonc {
    fn default() -> Self {
        Self {
            commentaire: [0; 255],
        }
    }
}

impl Sapbonc {
    /*******************************************************************
                            GetCommentaire
    ********************************************************************/
    unsafe fn GetCommentaire(&mut self, index: c_short) -> *mut c_char {
        let p = &raw mut self.commentaire as *mut c_char;
        GetIndString(p as *mut u8, STR_COMMENTAIRE, index - 44);
        PtoCstr(p as *mut u8);
        p
    }
}

/******************************************************************************************
                                VectFrameRect
Encadre l'item
******************************************************************************************/
unsafe extern "C" fn VectFrameRect(window: DialogPtr, itemid: c_short) {
    unsafe {
        let mut r: Rect = Rect::default();

        GetRectItem(window, itemid, &raw mut r);
        CarboniseSetPatBlack();
        FrameRect(&r);
    }
}

/***********************************************************************************
                                                dlgFilter
***********************************************************************************/
unsafe extern "C" fn dlgFilter(
    dlg: DialogPtr,
    event: *mut EventRecord,
    _item: *mut c_short,
) -> Boolean {
    unsafe {
        let mut car: u8 = 0;
        let mut result: Boolean = FAUX;

        let _g = port::on(dlg);
        let ev = *event;
        if ev.what as u32 == keyDown || ev.what as u32 == autoKey {
            car = (ev.message & charCodeMask as i64) as u8;
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
        }
        result
    }
}

impl Sapiens {
    /******************************************************************************************
                                            GetCoordonnees
    ******************************************************************************************/
    unsafe fn GetCoordonnees(&self, c: *mut Coordonnees) -> Boolean {
        unsafe {
            let mut dialog: DialogPtr = core::ptr::null_mut();
            let mut item: c_short = 0;
            let mut chaine: [c_char; 255] = [0; 255];
            let mut d: DateTimeRec = DateTimeRec::default();
            let mut modalUPP: ModalFilterUPP = core::ptr::null_mut();

            let _g = port::preserve();
            dialog = OuvreDialogue(DLOG_COORDONNEES as c_short);
            if dialog.is_null() {
                return FAUX;
            }
            SetValBouton(dialog, i_normal, VRAI);
            SetValBouton(dialog, i_liquide, VRAI);
            DefAfficheItem(dialog, i_rect1, Some(VectFrameRect));
            DefAfficheItem(dialog, i_rect2, Some(VectFrameRect));
            CarboniseSetCursorArrow();
            modalUPP = dlgFilter as *const () as *mut c_void;
            CarboniseShowDialog(dialog);
            loop {
                ModalDialog(modalUPP, &raw mut item);
                match item {
                    i_normal | i_disque => {
                        SetValBouton(dialog, i_normal, item == i_normal);
                        SetValBouton(dialog, i_disque, item == i_disque);
                    }
                    i_liquide | i_cheque => {
                        SetValBouton(dialog, i_liquide, item == i_liquide);
                        SetValBouton(dialog, i_cheque, item == i_cheque);
                    }
                    _ => {}
                }
                if item == i_imprimer || item == i_annuler {
                    break;
                }
            }
            if item == i_imprimer {
                core::ptr::write_bytes(c as *mut Coordonnees, 0, 1);
                GetVersionSapiens(chaine.as_mut_ptr());
                sprintf(
                    (*c).version.as_mut_ptr(),
                    b"%s \0".as_ptr() as *const c_char,
                    chaine.as_ptr(),
                );
                /* Branche MAC : sur PC on ajoute "PC" */
                strcat(
                    (*c).version.as_mut_ptr(),
                    b"Mac\0".as_ptr() as *const c_char,
                );
                GetTime(&raw mut d);
                if version == FR as i16 {
                    sprintf(
                        (*c).date.as_mut_ptr(),
                        b"%d/%d/%d\0".as_ptr() as *const c_char,
                        d.day as c_int,
                        d.month as c_int,
                        d.year as c_int,
                    );
                } else {
                    sprintf(
                        (*c).date.as_mut_ptr(),
                        b"%d/%d/%d\0".as_ptr() as *const c_char,
                        d.month as c_int,
                        d.day as c_int,
                        d.year as c_int,
                    );
                }
                GetValItem(dialog, i_nom, (*c).nom.as_mut_ptr());
                GetValItem(dialog, i_adresse, (*c).adresse.as_mut_ptr());
                GetValItem(dialog, i_codePostal, (*c).codePostal.as_mut_ptr());
                GetValItem(dialog, i_Ville, (*c).ville.as_mut_ptr());
                GetValItem(dialog, i_Pays, (*c).pays.as_mut_ptr());
                GetValItem(dialog, i_Email, (*c).eMail.as_mut_ptr());
                (*c).isProchaineVersion = GetValBouton(dialog, i_disque) != 0;
                (*c).isLiquide = GetValBouton(dialog, i_liquide) != 0;
                GetValItem(dialog, i_trouve, (*c).trouve.as_mut_ptr());
                GetValItem(dialog, i_remarque, (*c).remarque.as_mut_ptr());
            }
            DisposeRoutineDescriptor(modalUPP as *mut c_void);
            item == i_imprimer
        }
    }
}

/******************************************************************************************
                                        Subst
 Dans le handle h substitue la chaine str1 par la chaine str2
******************************************************************************************/
unsafe fn Subst(h: Handle, str1: Ptr, str2: Ptr) {
    unsafe {
        let mut taille: c_long = GetHandleSize(h);
        let mut p: Ptr = *h as Ptr;
        let len1 = strlen(str1 as *const c_char);
        let len2 = strlen(str2 as *const c_char);
        let mut i: c_long = 0;

        while i < taille {
            if strncmp(
                p.add(i as usize) as *const c_char,
                str1 as *const c_char,
                len1,
            ) == 0
            {
                BlockMove(
                    p.add((i + len1 as c_long) as usize) as *const c_void,
                    p.add(i as usize) as *mut c_void,
                    taille - (i + len1 as c_long),
                );
                SetHandleSize(h, taille + len2 as c_long - len1 as c_long);
                taille = GetHandleSize(h);
                p = *h as Ptr;
                BlockMove(
                    p.add(i as usize) as *const c_void,
                    p.add((i + len2 as c_long) as usize) as *mut c_void,
                    taille - i - len2 as c_long,
                );
                BlockMove(
                    str2 as *const c_void,
                    p.add(i as usize) as *mut c_void,
                    len2 as c_long,
                );
            }
            i += 1;
        }
    }
}

impl Sapiens {
    /******************************************************************************************
                                            DoPrintBonCommande
    ******************************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn DoPrintBonCommande(&mut self) {
        unsafe {
            let mut PrtPort: TPPrPort = core::ptr::null_mut();
            let mut rpage: Rect = Rect::default();
            let mut prStatus: TPrStatus = TPrStatus::default();
            let mut hprint: THPrint = core::ptr::null_mut();
            let mut h: Handle = core::ptr::null_mut();
            let mut texte: TEHandle = core::ptr::null_mut();
            let mut appResFile: c_short = 0;
            let mut printResFile: c_short = 0;
            let mut c: Coordonnees = core::mem::zeroed();

            if !self.GetCoordonnees(&raw mut c) {
                return;
            }
            let _g = port::preserve();
            appResFile = CurResFile();
            /* On Initialise le driver imprimante */
            PrSetError(noErr as c_short);
            PrOpen();
            if PrError() == noErr as c_short {
                printResFile = CurResFile();
                hprint = NewHandleClear(size_of::<TPrint>() as c_long) as THPrint;
                if !hprint.is_null() {
                    PrintDefault(hprint);
                    if PrStlDialog(hprint) {
                        /* On demande de les modifier et de confirmer l'impression*/
                        if PrJobDialog(hprint) {
                            UseResFile(printResFile);
                            PrtPort =
                                PrOpenDoc(hprint, core::ptr::null_mut(), core::ptr::null_mut());
                            if PrError() == noErr as c_short {
                                /* On la ouvert */
                                PrOpenPage(PrtPort, core::ptr::null_mut());
                                if PrError() == noErr as c_short {
                                    UseResFile(appResFile);
                                    rpage = (**hprint).prInfo.rPage;
                                    h = GetResource(
                                        0x54455854, /* 'TEXT' */
                                        RSRC_TEXT_CARTE_ENREG as c_short,
                                    );
                                    UseResFile(printResFile);
                                    if !h.is_null() {
                                        DetachResource(h);
                                        TextFont(courier as c_short);
                                        TextSize(12);
                                        texte = TENew(&raw const rpage, &raw const rpage);
                                        if !texte.is_null() {
                                            Subst(
                                                h,
                                                b"$1\0".as_ptr() as Ptr,
                                                c.version.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$2\0".as_ptr() as Ptr,
                                                c.nom.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$3\0".as_ptr() as Ptr,
                                                c.date.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$4\0".as_ptr() as Ptr,
                                                c.adresse.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$5\0".as_ptr() as Ptr,
                                                c.codePostal.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$6\0".as_ptr() as Ptr,
                                                c.ville.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$7\0".as_ptr() as Ptr,
                                                c.pays.as_mut_ptr() as Ptr,
                                            );
                                            if strlen(c.eMail.as_ptr()) > 0 {
                                                Subst(
                                                    h,
                                                    b"$8\0".as_ptr() as Ptr,
                                                    c.eMail.as_mut_ptr() as Ptr,
                                                );
                                            } else {
                                                Subst(
                                                    h,
                                                    b"$8\0".as_ptr() as Ptr,
                                                    self.sapbonc.GetCommentaire(46) as Ptr,
                                                ); /* Non */
                                            }
                                            if c.isLiquide {
                                                Subst(
                                                    h,
                                                    b"$9\0".as_ptr() as Ptr,
                                                    self.sapbonc.GetCommentaire(49) as Ptr,
                                                );
                                            } else {
                                                Subst(
                                                    h,
                                                    b"$9\0".as_ptr() as Ptr,
                                                    self.sapbonc.GetCommentaire(50) as Ptr,
                                                );
                                            }
                                            if c.isProchaineVersion {
                                                Subst(
                                                    h,
                                                    b"$A\0".as_ptr() as Ptr,
                                                    self.sapbonc.GetCommentaire(48) as Ptr,
                                                );
                                            } else {
                                                Subst(
                                                    h,
                                                    b"$A\0".as_ptr() as Ptr,
                                                    self.sapbonc.GetCommentaire(47) as Ptr,
                                                );
                                            }
                                            Subst(
                                                h,
                                                b"$B\0".as_ptr() as Ptr,
                                                c.trouve.as_mut_ptr() as Ptr,
                                            );
                                            Subst(
                                                h,
                                                b"$C\0".as_ptr() as Ptr,
                                                c.remarque.as_mut_ptr() as Ptr,
                                            );
                                            HLock(h);
                                            TEInsert(*h as *const c_void, GetHandleSize(h), texte);
                                            HUnlock(h);
                                            /* On affiche la page */
                                            TEUpdate(&raw const rpage, texte);
                                        }
                                        DisposHandle(h);
                                    }
                                }
                                PrClosePage(PrtPort); /* Impression de la page */
                            }
                            PrCloseDoc(PrtPort);
                        }
                        /* Si selection papier JOB OK */
                        else {
                            PrSetError(iPrAbort as c_short);
                        }
                        if PrError() == noErr as c_short && (**hprint).prJob.bJDocLoop == 1 {
                            PrPicFile(
                                hprint,
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                                &raw mut prStatus,
                            );
                        }
                    } else {
                        PrSetError(iPrAbort as c_short);
                    }
                    /* Liberation du hprint */
                    DisposHandle(hprint as Handle);
                }
            }
            UseResFile(appResFile);
            PrClose();
        }
    }
}

/*EOF*/
