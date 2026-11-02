//! Stubs restants de l'API "Acam" et du Toolbox Mac que le code C appelle.
//!
//! Les symboles rendus fonctionnels vivent dans `src/acam.rs` ; tout ce qui
//! est encore un no-op sûr reste ici. Chaque `#[no_mangle]` de ce fichier est
//! un futur portage possible (entrées souris/clavier, textes, son, ...).

#![allow(non_snake_case)]

use crate::cbridge::{MenuHandle, OSErr, SndChannelPtr, WindowPtr};
use crate::exchange::{INPUT, MOUSE_EVENT_SCALE};
use crate::port;
use crate::port::{PortId, Ports};
use crate::shared::*;
use std::ffi::c_void;

// ---------------------------------------------------------------------------
// Gestion des fenetres / evenements
// ---------------------------------------------------------------------------
pub(crate) fn GetKeys() -> KeyMap {
    INPUT.lock().unwrap().keymap
}

pub(crate) fn Button() -> Boolean {
    INPUT.lock().unwrap().button as Boolean
}

impl Ports {
    /// Convertit le `where_` d'un événement souris (position normalisée de
    /// 0 à `MOUSE_EVENT_SCALE`, posée par egui) dans le repère de la fenêtre
    /// principale, comme `GetMouse`.
    pub(crate) fn event_point(&self, pt: Point) -> Point {
        let b = self.mainWindow.bounds;
        let w = f64::from(b.right - b.left).max(1.0);
        let h = f64::from(b.bottom - b.top).max(1.0);
        let scale = f64::from(MOUSE_EVENT_SCALE);
        Point {
            h: (f64::from(pt.h) / scale * w) as i16,
            v: (f64::from(pt.v) / scale * h) as i16,
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn GetMouse(&self) -> Point {
        let st = INPUT.lock().unwrap();
        let p = &self.mainWindow;
        let b = (*p).bounds;
        let w = f64::from(b.right - b.left).max(1.0);
        let h = f64::from(b.bottom - b.top).max(1.0);
        Point {
            h: (f64::from(st.mouse_h) * w) as i16,
            v: (f64::from(st.mouse_v) * h) as i16,
        }
    }
}

pub(crate) extern "C" fn FrontWindow() -> WindowPtr {
    std::ptr::null_mut()
}
pub(crate) extern "C" fn SelectWindow(_w: WindowPtr) {}
/// Un seul « bureau » : tout clic est dans le contenu de la fenêtre de jeu
/// (la barre de menu est gérée par egui, qui ne pousse pas d'événement).
pub(crate) extern "C" fn FindWindow(_pt: Point, _window: *mut WindowPtr) -> i16 {
    crate::cbridge::inContent as i16
}
pub(crate) extern "C" fn SystemClick(_event: *const EventRecord, _w: WindowPtr) {}
pub(crate) extern "C" fn SystemTask() {}
pub(crate) extern "C" fn DragWindow(_w: WindowPtr, _pt: Point, _bounds: *const Rect) {}

impl Ports {
    pub(crate) fn SizeWindow(
        &mut self,
        port_id: PortId,
        wWidth: i16,
        wHeight: i16,
        _update: Boolean,
    ) {
        /* Un `WindowPtr` est ici un `SapPort` : redimensionner la fenêtre revient à
        reallouer son tampon (menu « Échelle », `FixeTailleMainWindow`). */
        let old = self.port(port_id).bounds;
        port::resize(
            self.port_mut(port_id),
            Rect {
                top: old.top,
                left: old.left,
                bottom: old.top + wHeight,
                right: old.left + wWidth,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// Menus
// ---------------------------------------------------------------------------

pub(crate) fn MenuSelect(_pt: Point) -> i32 {
    0
}
pub(crate) fn MenuKey(_ch: i16) -> i64 {
    0
}
pub(crate) fn AddResMenu(_menu: MenuHandle, _type: i64) {}
pub(crate) fn HiliteMenu(_menuID: i16) {}
pub(crate) fn OpenDeskAcc(_name: StringPtr) {}

// ---------------------------------------------------------------------------
// Controles (ascenseurs)
// ---------------------------------------------------------------------------

pub(crate) fn CarboniseSetPatBlack() {}
pub(crate) fn CarboniseSetPatGray() {}

// ---------------------------------------------------------------------------
// Ressources / memoire (restants)
// ---------------------------------------------------------------------------

pub(crate) fn ReleaseResource(_h: Handle) {}
pub(crate) fn DisposePtr(_p: Ptr) {}
pub(crate) fn GetPicture(resID: i16) -> PicHandle {
    crate::pict::get_picture(resID)
}
pub(crate) fn GetRectFilePng(_name: &str, _r: &mut Rect) -> Boolean {
    false
}

// ---------------------------------------------------------------------------
// Timers / chemins temporaires
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn PushPath() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PopPath() {}

// ---------------------------------------------------------------------------
// Son
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn SndNewChannel(
    _chan: *mut SndChannelPtr,
    _synth: i16,
    _init: i16,
    _userRoutine: *mut c_void,
) -> OSErr {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SndPlay(_chan: SndChannelPtr, _snd: SndListHandle, _async: Boolean) {}

// ---------------------------------------------------------------------------
// Images offscreen (restants)
// ---------------------------------------------------------------------------

pub(crate) fn ConvertPNGFileToOffscreen(_fileName: &str) -> Option<SapPort> {
    None
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseInvalRect(_r: *const Rect) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseValideUpdate() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseGetMenuBarHeight() -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseNewUserItemProc(_proc: *mut c_void) -> Handle {
    std::ptr::null_mut()
}

// ---------------------------------------------------------------------------
// Divers systeme
// ---------------------------------------------------------------------------

pub(crate) extern "C" fn InitDefautConfigSortieNumerique() {}

// ---------------------------------------------------------------------------
// Volumes / chemins
// ---------------------------------------------------------------------------

pub(crate) extern "C" fn SaisitChemin(
    _volRefNum: *mut i16,
    _libPath: *mut i8,
    _numPath: *mut i64,
) -> Boolean {
    false
}
pub(crate) extern "C" fn SetPathApplication() {}
pub(crate) extern "C" fn FixeCheminRelatif(_volRef: i16, _pathBase: i64, _path: *mut i8) -> OSErr {
    0
}
