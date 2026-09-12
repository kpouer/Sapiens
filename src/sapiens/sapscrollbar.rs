#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
use crate::acam::{SetRect, TickCount};
use crate::cbridge::{Collapse, DialogPtr, OSErr, WindowPtr};
use crate::sapiens::sapwbrt::RetraceItem;
use crate::sapiens::sapwind::GetRectItem;
use crate::shared::{ControlHandle, Rect, StringPtr};
use crate::stubs::{
    CarboniseDisposeControlActionProc, CarboniseFindControlDialog, CarboniseGetDialogFromControl,
    CarboniseGetDialogWindow, CarboniseNewControlActionProc, CarboniseSetNewControl,
    CarboniseTrackControl, GetCtlMax, GetCtlValue, NewControl, SetControlValue, ShowControl,
};
use std::os::raw::c_char;
use std::ptr;

static mut whatItem: i16 = 0;
static mut WhatControl: ControlHandle = ptr::null_mut();
static mut ESTR: u8 = 0;

const inUpButton: i16 = 20;
const inDownButton: i16 = 21;
const inPageUp: i16 = 22;
const inPageDown: i16 = 23;
const inThumb: i16 = 5;

/******************************************************************************************
                                CommonAction
Change la valeur de l'ascenseur du control demande.
=> Control, delta a appliquer
<= Valeur reellement appliquee
******************************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CommonAction(control: ControlHandle, amount: i16) -> i16 {
    unsafe {
        let value = GetCtlValue(control); /* get current value */
        let max = GetCtlMax(control); /* and maximum value */
        let mut amount = value + amount;
        if amount < 0 {
            amount = 0;
        } else if amount > max {
            amount = max;
        }
        SetControlValue(control, amount);
        amount - value
    }
}

/******************************************************************************************
                               ActionInfo
******************************************************************************************/
unsafe extern "C" fn ActionInfo(control: ControlHandle, part: i16) {
    let mut amount: i16 = 0;
    if part != 0 {
        match part {
            inUpButton | inDownButton | inPageUp | inPageDown => amount = 1,
            _ => (),
        }
        if part == inDownButton || part == inPageDown {
            amount = -amount;
        }
        if amount != 0 {
            amount = CommonAction(control, -amount);
            let t = TickCount();
            if whatItem > 0 {
                RetraceItem(CarboniseGetDialogFromControl(control), whatItem);
            }
            let mut t2 = TickCount();
            while t2 < t + 10 {
                t2 = TickCount();
            }
        }
    }
}

/*********************************************************************
                        ClickInAscenseur
=> Dialogue, item a retracer en fonction de l'asenseur
Un anti repeat est gere sur l'ascenseur
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn ClickInAscenseur(dialog: DialogPtr, item: i16, pt: crate::shared::Point) {
    unsafe {
        /* Dans le control ? */
        whatItem = item;
        /* On determine le control qui a ete clique */
        let mut control: ControlHandle = ptr::null_mut();
        let part = CarboniseFindControlDialog(pt, dialog, &mut control);
        WhatControl = control;
        /* En fonction de l'endroit du click, on le gere */
        match part {
            /* Nulle part d'actif */
            0 => {}
            /* Dans l'onglet de l'ascenseur */
            inThumb => {
                CarboniseTrackControl(control, pt, None);
                if whatItem > 0 {
                    RetraceItem(CarboniseGetDialogFromControl(control), whatItem);
                }
            }
            /* Ailleur que dans l'onglet scroll bar */
            _ => {
                let action_upp = CarboniseNewControlActionProc(Some(
                    ActionInfo as unsafe extern "C" fn(ControlHandle, i16),
                ));
                CarboniseTrackControl(control, pt, action_upp);
                CarboniseDisposeControlActionProc(action_upp);
            }
        }
    }
}

/*****************************************************************************
                                CarboniseNewScrollBarStd
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CarboniseNewScrollBarStd(
    dialog: DialogPtr,
    item: i16,
    value: i16,
    min: i16,
    max: i16,
) -> ControlHandle {
    unsafe {
        /* On saisie les valeurs actuelles*/
        let mut aireTot: Rect = std::mem::zeroed();
        GetRectItem(dialog, item, &mut aireTot);
        let control = NewControl(
            CarboniseGetDialogWindow(dialog),
            &aireTot,
            (&raw const ESTR) as StringPtr,
            true,       /* Visible*/
            value,      /* Valeur initiale */
            min,        /* Minimum*/
            max,        /* Maximum */
            16,         /* scrollBarProc */
            0x5363726F, /* 'Scro' Reference*/
        );
        let err: OSErr = CarboniseSetNewControl(dialog, item, control);
        if err != 0 {
            Collapse(
                c"CarboniseSetNewControl %d".as_ptr() as *mut c_char,
                err as i32,
            );
        }
        /* On le montre */
        ShowControl(control);
        control
    }
}

/*****************************************************************************
                                CarboniseNewScrollBarWindow
Ajoute un scroll une fenetre
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CarboniseNewScrollBarWindow(window: WindowPtr) -> ControlHandle {
    unsafe {
        let mut aireTot: Rect = std::mem::zeroed();
        SetRect(&mut aireTot, 0, 0, 16, 16);
        let control = NewControl(
            window,
            &aireTot,
            (&raw const ESTR) as StringPtr,
            true,       /* Visible*/
            0,          /* Valeur initiale */
            0,          /* Minimum*/
            0,          /* Maximum */
            16,         /* scrollBarProc */
            0x5363726F, /* 'Scro' Reference*/
        );
        /* On le montre */
        ShowControl(control);
        control
    }
}
