/****************************************************************************
            Encadre et efface le message
*****************************************************************************/
use crate::acam::{
    AcamGetPortOffscreen, AcamSetPortOffscreen, CarboniseGetWindowPort, CarboniseSetPortWindow,
    InsetRect, SetRect,
};
use crate::sapiens::sapwbrt::{CopyOffscreen, DoEraseRect, DoFrameRect, DoPenSize, DoScrollRect};
use crate::sapiens::sapwclav::eff_sour;
use crate::sapiens::sapwind::{debecr, mainWindow};
use crate::shared::{CGrafPtr, Rect};
use crate::stubs::CarboniseInvalRect;
use std::os::raw::{c_short, c_void};

#[unsafe(no_mangle)]
pub static mut rMess: Rect = Rect {
    top: 208,
    left: 32,
    bottom: 224,
    right: 606,
};

pub fn framebadass() {
    let mut gbak = CGrafPtr::default();
    AcamGetPortOffscreen(&mut gbak as *mut _);

    unsafe {
        AcamSetPortOffscreen(debecr);
        let mut r = rMess;
        InsetRect(&mut r as *mut _ as *mut _, -1, -1);
        DoEraseRect(&mut r);
        DoFrameRect(&mut r);
        DoPenSize(1, 1);
        AcamSetPortOffscreen(gbak);
    }
}

/****************************************************************************
            TfrBadAss
Le message s'affiche sur debecr, on le transfere sur le reel
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn TfrBadAss() {
    eff_sour();
    unsafe {
        CopyOffscreen(
            debecr,
            CarboniseGetWindowPort(mainWindow),
            &raw mut rMess,
            &raw mut rMess,
        );
        CarboniseInvalRect(&raw const rMess);
    }
}

/****************************************************************************
            Effacement du message
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn efbadass() {
    unsafe {
        let mut gbak: *mut c_void = core::ptr::null_mut();

        AcamGetPortOffscreen(&raw mut gbak);
        AcamSetPortOffscreen(debecr);
        DoEraseRect(&raw mut rMess);
        AcamSetPortOffscreen(gbak);
    }
}

/****************************************************************************
            Scrolling du message
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn scrollbadass() {
    unsafe {
        let mut i: c_short;
        let mut gbak: *mut c_void = core::ptr::null_mut();
        let mut reff: Rect = Rect {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
        };

        eff_sour();
        AcamGetPortOffscreen(&raw mut gbak);
        CarboniseSetPortWindow(mainWindow);
        SetRect(
            &raw mut reff,
            rMess.left,
            rMess.top,
            rMess.left + 8,
            rMess.bottom,
        );
        i = 0;
        while i < rMess.right - rMess.left {
            DoScrollRect(&raw mut rMess, 8, 0, core::ptr::null_mut());
            DoEraseRect(&raw mut reff);
            i += 8;
        }
        efbadass();
        AcamSetPortOffscreen(gbak);
    }
}

/*EOF*/
