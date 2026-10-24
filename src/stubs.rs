//! Stubs restants de l'API "Acam" et du Toolbox Mac que le code C appelle.
//!
//! Les symboles rendus fonctionnels vivent dans `src/acam.rs` ; tout ce qui
//! est encore un no-op sûr reste ici. Chaque `#[no_mangle]` de ce fichier est
//! un futur portage possible (entrées souris/clavier, textes, son, ...).

#![allow(non_snake_case)]

use crate::cbridge::{
    DateTimeRec, MenuHandle, OSErr, SndChannelPtr, THPrint, TPPrPort, TPrStatus, WindowPtr,
};
use crate::exchange::INPUT;
use crate::port;
use crate::shared::*;
use std::ffi::c_void;
use std::sync::atomic::Ordering;
// ---------------------------------------------------------------------------
// Variables globales exposees par le C
// ---------------------------------------------------------------------------

pub(crate) static qd: QDGlobals = QDGlobals {
    screenBits: QDGlobalsScreenBits {
        bounds: Rect {
            top: 0,
            left: 0,
            bottom: 400,
            right: 640,
        },
    },
};

// ---------------------------------------------------------------------------
// Gestion des fenetres / evenements
// ---------------------------------------------------------------------------
pub(crate) fn GetKeys() -> KeyMap {
    INPUT.lock().unwrap().keymap
}

pub(crate) fn Button() -> Boolean {
    INPUT.lock().unwrap().button as Boolean
}

#[allow(non_snake_case)]
pub(crate) fn GetMouse() -> Point {
    let st = INPUT.lock().unwrap();
    let (w, h) = unsafe {
        let p = WINDOW_PORT.load(Ordering::SeqCst);
        if p.is_null() {
            (640.0_f64, 400.0_f64)
        } else {
            let b = (*p).bounds;
            (
                f64::from(b.right - b.left).max(1.0),
                f64::from(b.bottom - b.top).max(1.0),
            )
        }
    };
    Point {
        h: (f64::from(st.mouse_h) * w) as i16,
        v: (f64::from(st.mouse_v) * h) as i16,
    }
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn FrontWindow() -> WindowPtr {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SelectWindow(_w: WindowPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetNextEvent(_mask: i16, _event: *mut EventRecord) -> Boolean {
    false
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn FindWindow(_pt: Point, _window: *mut WindowPtr) -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SystemClick(_event: *const EventRecord, _w: WindowPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SystemTask() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn BeginUpdate(_w: WindowPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn EndUpdate(_w: WindowPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DragWindow(_w: WindowPtr, _pt: Point, _bounds: *const Rect) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SizeWindow(w: WindowPtr, wWidth: i16, wHeight: i16, _update: Boolean) {
    /* Un `WindowPtr` est ici un `SapPort` : redimensionner la fenêtre revient à
    reallouer son tampon (menu « Échelle », `FixeTailleMainWindow`). */
    let p = w as CGrafPtr;
    let Some(old) = port::info(p) else {
        return;
    };
    port::resize(
        p,
        Rect {
            top: old.bounds.top,
            left: old.bounds.left,
            bottom: old.bounds.top + wHeight,
            right: old.bounds.left + wWidth,
        },
    );
}
#[allow(non_snake_case)]
pub(crate) fn LocalToGlobal(_pt: &mut Point) {}

// ---------------------------------------------------------------------------
// Menus
// ---------------------------------------------------------------------------

pub(crate) fn MenuSelect(_pt: Point) -> i32 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn MenuKey(_ch: i16) -> i64 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn AddResMenu(_menu: MenuHandle, _type: i64) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn NewMenu(_menuID: i16, _title: StringPtr) -> MenuHandle {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn InsertMenu(_menu: MenuHandle, _beforeID: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DeleteMenu(_menuID: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DisposeMenu(_menu: MenuHandle) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn HiliteMenu(_menuID: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PopUpMenuSelect(
    _menu: MenuHandle,
    _top: i16,
    _left: i16,
    _item: i16,
) -> i64 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn OpenDeskAcc(_name: StringPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn InsMenuItem(_menu: MenuHandle, _itemName: StringPtr, _afterItem: i16) {}

// ---------------------------------------------------------------------------
// Dialogues
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn ModalDialog(_filter: *mut c_void, _item: *mut i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetNewDialog(
    _id: i16,
    _storage: *mut c_void,
    _behind: WindowPtr,
) -> DialogPtr {
    std::ptr::null_mut()
}
pub(crate) fn GetDItem(
    _dialog: DialogPtr,
    _itemNo: i16,
    _itemType: *mut i16,
    _item: *mut Handle,
    _box: *mut Rect,
) {
}
pub(crate) fn GetDialogItem(
    _dialog: DialogPtr,
    _itemNo: i16,
    _itemType: *mut i16,
    _item: *mut Handle,
    _box: *mut Rect,
) {
}
pub(crate) extern "C" fn SetDialogItem(
    _dialog: DialogPtr,
    _itemNo: i16,
    _itemType: i16,
    _item: Handle,
    _box: *const Rect,
) {
}
pub(crate) extern "C" fn SetDItem(
    _dialog: DialogPtr,
    _itemNo: i16,
    _itemType: i16,
    _item: Handle,
    _box: *mut Rect,
) {
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetDialogDefaultItem(_dialog: DialogPtr, _item: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseAdapteStaticText(
    _dialog: DialogPtr,
    _id: i16,
    _font: i16,
    _size: i16,
) {
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseSetPortDialog(_dialog: DialogPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseGetDialogTextEditHandle(_dialog: DialogPtr) -> TEHandle {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetIText(_handle: Handle, _text: StringPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetIText(_handle: Handle, _text: StringPtr) {}

// ---------------------------------------------------------------------------
// Controles (ascenseurs)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetCtlValue(_c: ControlHandle) -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetCtlMax(_c: ControlHandle) -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetControlValue(_c: ControlHandle, _value: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetCtlValue(_c: ControlHandle, _value: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetCTitle(_control: Handle, _text: StringPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn HiliteControl(_control: Handle, _value: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn NewControl(
    _owner: WindowPtr,
    _bounds: *const Rect,
    _title: StringPtr,
    _visible: Boolean,
    _value: i16,
    _min: i16,
    _max: i16,
    _procID: i16,
    _refCon: i64,
) -> ControlHandle {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn ShowControl(_c: ControlHandle) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseGetDialogWindow(_dialog: DialogPtr) -> WindowPtr {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseGetDialogFromControl(_c: ControlHandle) -> DialogPtr {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseFindControlDialog(
    _pt: Point,
    _dialog: DialogPtr,
    _outControl: *mut ControlHandle,
) -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseTrackControl(
    _c: ControlHandle,
    _pt: Point,
    _actionProc: ControlActionUPP,
) {
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseNewControlActionProc(
    _proc: ControlActionProcPtr,
) -> ControlActionUPP {
    None
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseDisposeControlActionProc(_upp: ControlActionUPP) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseSetNewControl(
    _d: DialogPtr,
    _item: i16,
    _c: ControlHandle,
) -> i16 {
    0
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseSetPatBlack() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseSetPatGray() {}

// ---------------------------------------------------------------------------
// Ressources / memoire (restants)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetResource(_type: i64, _id: i16) -> Handle {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn ReleaseResource(_h: Handle) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn HLock(_h: Handle) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn HUnlock(_h: Handle) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DisposePtr(_p: Ptr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DisposeRoutineDescriptor(_p: *mut c_void) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetPicture(_resID: i16) -> PicHandle {
    std::ptr::null_mut()
}
pub(crate) fn DrawPicture(_pic: PicHandle, _dstRect: &Rect) {}
pub(crate) fn GetRectFilePng(_name: *mut i8, _r: &mut Rect) -> Boolean {
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

#[unsafe(no_mangle)]
pub(crate) extern "C" fn ConvertPNGFileToOffscreen(_fileName: *mut i8) -> CGrafPtr {
    std::ptr::null_mut()
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

#[unsafe(no_mangle)]
pub(crate) extern "C" fn Gestalt(_selector: i64, _response: *mut i64) -> OSErr {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn FindFolder(
    _vRefNum: i16,
    _folderType: i64,
    _create: Boolean,
    _foundVRefNum: *mut i16,
    _foundDirID: *mut i64,
) -> OSErr {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetVol(_volName: StringPtr, _vRefNum: i16) {}
pub(crate) fn ArreteMusique(_b: Boolean) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn IsFinDeMusique() -> Boolean {
    false
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn JoueMusiqueHarmony(
    _nom: *mut i8,
    _donneesXML: Handle,
    _listeMesures: *mut i8,
    _indexAbc: i16,
    _organe: *mut i8,
    _volume: i16,
    _tempo: i16,
    _decalageDemiTon: i16,
    _isMetronome: Boolean,
) -> Boolean {
    false
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn InitDefautConfigSortieNumerique() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn stricmp(_c1: *const i8, _c2: *const i8) -> i32 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn OpenDocument(_path: *mut i8) -> i16 {
    0
}

// ---------------------------------------------------------------------------
// Volumes / chemins
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn PreservePath() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn RestorePath() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SaisitChemin(
    _volRefNum: *mut i16,
    _libPath: *mut i8,
    _numPath: *mut i64,
) -> Boolean {
    false
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn FixeVolPath(_volRefNum: i16, _path: i64) -> OSErr {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn SetPathApplication() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn FixeCheminRelatif(_volRef: i16, _pathBase: i64, _path: *mut i8) -> OSErr {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetNomCheminCourant(_path: *mut i8) {}

// ---------------------------------------------------------------------------
// Boites de dialogue fichiers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Saisie de texte (TextEdit) / Coeur dialogues
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn SelIText(_d: DialogPtr, _item: i16, _start: i16, _end: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DlgCut(_d: DialogPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseGetDialogKeyboardFocusItem(_d: DialogPtr) -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn CarboniseShowDialog(_d: DialogPtr) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetTime(_d: *mut DateTimeRec) {}

// ---------------------------------------------------------------------------
// Impression (Print Manager)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn CurResFile() -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrSetError(_err: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrOpen() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrError() -> i16 {
    0
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrintDefault(_hPrint: THPrint) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrStlDialog(_hPrint: THPrint) -> Boolean {
    false
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrJobDialog(_hPrint: THPrint) -> Boolean {
    false
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn UseResFile(_resFile: i16) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrOpenDoc(
    _hPrint: THPrint,
    _pPrPort: *mut c_void,
    _pIOBuf: *mut c_void,
) -> TPPrPort {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrOpenPage(_pPrPort: TPPrPort, _pPageFrame: *mut Rect) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrClosePage(_pPrPort: TPPrPort) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrCloseDoc(_pPrPort: TPPrPort) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrClose() {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn PrPicFile(
    _hPrint: THPrint,
    _pPrPort: *mut c_void,
    _pIOBuf: *mut c_void,
    _pDevBuf: *mut c_void,
    _pPrStatus: *mut TPrStatus,
) {
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn DetachResource(_h: Handle) {}

// ---------------------------------------------------------------------------
// TextEdit (TENew / TEInsert / TEUpdate)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub(crate) extern "C" fn TENew(_destRect: *const Rect, _viewRect: *const Rect) -> TEHandle {
    std::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn TEInsert(_text: *const c_void, _length: i64, _h: TEHandle) {}
#[unsafe(no_mangle)]
pub(crate) extern "C" fn TEUpdate(_updateRect: *const Rect, _hTE: TEHandle) {}
