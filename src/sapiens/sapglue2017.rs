/*************************************************************************
                    SapGlue2017.c
                (c) Dg le 18-03-2017
***************************************************************************/
#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::cbridge::{
    DialogPtr, ListHandle, NavEventProcPtr, NavEventUPP, NavObjectFilterProcPtr,
    NavObjectFilterUPP, WindowPtr, strcpy,
};
use crate::sapiens::sapwind::{Collapse, GetRectItem};
use crate::shared::{Boolean, Point, Ptr, Rect};
use std::os::raw::{c_char, c_short, c_void};

pub extern "C" fn ExtIsApplicationActive() -> Boolean {
    true
}

pub extern "C" fn ExtInvalideAireItem(_dialog: DialogPtr, _item: c_short) {}

pub extern "C" fn ExtDoModalDialog(
    _contexte: *mut c_void,
    _item: *mut c_short,
    _ptResult: *mut Point,
) {
}

pub extern "C" fn ExtOuvreDialogue(
    _id: c_short,
    _modePlacement: c_short,
    _isMenuEdition: Boolean,
    _isPrint: Boolean,
    _isModale: Boolean,
    _contexte: *mut c_void,
) -> DialogPtr {
    std::ptr::null_mut()
}

pub extern "C" fn ExtSetValItemUTF(
    _dialog: DialogPtr,
    _itemHit: c_short,
    _c: *mut c_char,
) -> c_short {
    1
}

pub unsafe extern "C" fn CallBackErreur(err: *mut c_char) {
    Collapse(err);
}

/*********************************************************************
                        ExtGetNomLogiciel
*******************************************************************/
pub unsafe extern "C" fn ExtGetNomLogiciel(str: *mut c_char) {
    strcpy(str, c"Sapiens".as_ptr());
}

/*********************************************************************
                        ExtGetStrVS
*******************************************************************/
pub extern "C" fn ExtGetStrVS(_index: c_short) -> *mut c_char {
    b"Virtual Singer...\0".as_ptr() as *mut c_char
}

pub extern "C" fn ExtSaisieTexte(
    _data: *mut c_char,
    _textStyle: *mut c_void,
    _backColor: *mut c_char,
) -> Ptr {
    std::ptr::null_mut()
}

pub extern "C" fn ExtQuelElementSelectionne(_liste: ListHandle) -> c_short {
    0
}

pub extern "C" fn ExtFermeDialogue(_contexte: *mut c_void) {}

pub extern "C" fn ExtGetRectItem(dialog: DialogPtr, item: c_short, box_: *mut Rect) {
    unsafe {
        GetRectItem(dialog, item, box_);
    }
}

pub extern "C" fn ExtSelectElementListe(_liste: ListHandle, _n: c_short) {}

pub unsafe extern "C" fn ExtCollapse(format: *mut c_char) {
    Collapse(format);
}

/******************************************************************************************
                                    ClavierNoteOn
=> numinstrument (-1 si toute)
******************************************************************************************/
pub extern "C" fn ClavierNoteOn(
    _numInstrument: c_short,
    _note: c_short,
    _velocite: c_short,
    _doigte: u8,
    _isStopNow: Boolean,
) {
}

/******************************************************************************************
                                    ClavierNoteOff
******************************************************************************************/
pub extern "C" fn ClavierNoteOff(_numInstrument: c_short, _note: c_short, _isImmediate: Boolean) {}

/******************************************************************************************
                                    IsClavierActive
******************************************************************************************/
pub extern "C" fn IsClavierActive() -> Boolean {
    false
}

pub unsafe extern "C" fn ScriptTraceLog(_format: *mut c_char) {}

pub extern "C" fn Confirmation(_window: WindowPtr, _isUTF: Boolean, _str: *mut c_char) -> Boolean {
    true
}

pub extern "C" fn SetPathTmp() {}

pub extern "C" fn ConfirmationVrac(_idStr: c_short) -> Boolean {
    true
}

/*********************************************************************
                       CarboniseNewNavObjectFilterProc
********************************************************************/
pub extern "C" fn CarboniseNewNavObjectFilterProc(
    userRoutine: NavObjectFilterProcPtr,
) -> NavObjectFilterUPP {
    userRoutine as NavObjectFilterUPP
}

/*********************************************************************
                       CarboniseDisposeNavObjectFilterProc
********************************************************************/
pub extern "C" fn CarboniseDisposeNavObjectFilterProc(_filterProc: NavObjectFilterUPP) {}

/*********************************************************************
                       CarboniseNewNavEventProc
********************************************************************/
pub extern "C" fn CarboniseNewNavEventProc(userRoutine: NavEventProcPtr) -> NavEventUPP {
    userRoutine as NavEventUPP
}

/*********************************************************************
                       CarboniseDisposeNavEventProc
********************************************************************/
pub extern "C" fn CarboniseDisposeNavEventProc(_eventProc: NavEventUPP) {}
