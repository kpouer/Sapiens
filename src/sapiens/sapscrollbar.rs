#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
use crate::Sapiens;
use crate::acam::{SetRect, TickCount};
use crate::cbridge::{Collapse, DialogPtr, OSErr, WindowPtr};
use crate::sapiens::sapwbrt::RetraceItem;
use crate::sapiens::sapwind::GetRectItem;
use crate::shared::{ControlHandle, Point, Rect, StringPtr};
use crate::stubs::{
    CarboniseDisposeControlActionProc, CarboniseFindControlDialog, CarboniseGetDialogFromControl,
    CarboniseGetDialogWindow, CarboniseNewControlActionProc, CarboniseSetNewControl,
    CarboniseTrackControl, GetCtlMax, GetCtlValue, NewControl, SetControlValue, ShowControl,
};
use std::os::raw::c_char;
use std::ptr;

/* Etat mutable de SAPSCROLLBAR.C, ex-`static mut`, regroupe dans cette
structure, champ de `Sapiens`. */
pub(crate) struct Sapscrollbar {
    /* ex-`whatItem` */
    pub(crate) what_item: i16,
    /* ex-`WhatControl` */
    pub(crate) what_control: ControlHandle,
    /* ex-`ESTR` : Pascal string vide passee a NewControl */
    pub(crate) estr: u8,
}

impl Default for Sapscrollbar {
    fn default() -> Self {
        Self {
            what_item: 0,
            what_control: ptr::null_mut(),
            estr: 0,
        }
    }
}

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
pub(crate) extern "C" fn CommonAction(control: ControlHandle, amount: i16) -> i16 {
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

impl Sapiens {
    /******************************************************************************************
                                   ActionInfo
    ******************************************************************************************/
    /* Anti-repeat de l'ascenseur. Conserve comme callback `pascal` d'origine : le
    port ne l'utilise plus (`CarboniseTrackControl` est un stub), mais la logique
    reste fidele au C. */
    #[allow(dead_code)]
    unsafe extern "C" fn ActionInfo(&self, control: ControlHandle, part: i16) {
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
                if self.sapscrollbar.what_item > 0 {
                    RetraceItem(
                        CarboniseGetDialogFromControl(control),
                        self.sapscrollbar.what_item,
                    );
                }
                let mut t2 = TickCount();
                while t2 < t + 10 {
                    t2 = TickCount();
                }
            }
        }
    }
}

impl Sapiens {
    /*********************************************************************
                            ClickInAscenseur
    => Dialogue, item a retracer en fonction de l'asenseur
    Un anti repeat est gere sur l'ascenseur
    ********************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn ClickInAscenseur(&mut self, dialog: DialogPtr, item: i16, pt: Point) {
        /* Dans le control ? */
        self.sapscrollbar.what_item = item;
        /* On determine le control qui a ete clique */
        let mut control: ControlHandle = ptr::null_mut();
        let part = CarboniseFindControlDialog(pt, dialog, &mut control);
        self.sapscrollbar.what_control = control;
        /* En fonction de l'endroit du click, on le gere */
        match part {
            /* Nulle part d'actif */
            0 => {}
            /* Dans l'onglet de l'ascenseur */
            inThumb => {
                CarboniseTrackControl(control, pt, None);
                if self.sapscrollbar.what_item > 0 {
                    RetraceItem(
                        CarboniseGetDialogFromControl(control),
                        self.sapscrollbar.what_item,
                    );
                }
            }
            /* Ailleur que dans l'onglet scroll bar */
            _ => {
                let action_upp = CarboniseNewControlActionProc(None);
                CarboniseTrackControl(control, pt, action_upp);
                CarboniseDisposeControlActionProc(action_upp);
            }
        }
    }

    /*****************************************************************************
                                    CarboniseNewScrollBarStd
    *****************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn CarboniseNewScrollBarStd(
        &mut self,
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
                (&raw const self.sapscrollbar.estr) as StringPtr,
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
    pub(crate) extern "C" fn CarboniseNewScrollBarWindow(
        &mut self,
        window: WindowPtr,
    ) -> ControlHandle {
        unsafe {
            let mut aireTot: Rect = std::mem::zeroed();
            SetRect(&mut aireTot, 0, 0, 16, 16);
            let control = NewControl(
                window,
                &aireTot,
                (&raw const self.sapscrollbar.estr) as StringPtr,
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
}
