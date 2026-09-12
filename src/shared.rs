//! Types et état partagés entre le code C (via FFI) et le moteur d'affichage Rust.
//!
//! Regroupe : les types QuickDraw/`repr(C)` (Rect, Point, RGBColor, ...), la
//! structure de port de dessin `SapPort`, les pointeurs globaux de port
//! (courant et fenêtre), la LUT 256 couleurs (http:// `Index2Color` /
//! `Color2Index` / rendu egui), le registre des allocations (pour libérer
//! proprement les `Ptr`/`Handle`), et l'état des entrées (souris/clavier)
//! alimenté par egui et consommé par les stubs `GetMouse`/`Button`/`GetKeys`.

#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::cbridge::TEText;
use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::atomic::AtomicPtr;
// ---------------------------------------------------------------------------
// Types repr(C)
// ---------------------------------------------------------------------------

pub type Boolean = bool;
pub type Handle = *mut *mut c_void;
pub type Ptr = *mut u8;
pub type CGrafPtr = *mut c_void;
pub type DialogPtr = *mut c_void;
pub type RgnHandle = *mut c_void;
pub type ControlHandle = *mut c_void;
pub type Cursor = *mut c_void;
pub type SndListHandle = *mut *mut c_void;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Pic {
    pub picSize: ::std::os::raw::c_short,
    pub picFrame: Rect,
}
pub type PicHandle = *mut *mut Pic;
pub type TEHandle = *mut *mut TEText;

pub type StringPtr = *mut u8;
pub type KeyMap = *mut u64;
pub type ControlActionProcPtr = ::std::option::Option<
    unsafe extern "C" fn(control: ControlHandle, part: ::std::os::raw::c_short),
>;

pub type ControlActionUPP = ControlActionProcPtr;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Point {
    pub v: i16,
    pub h: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Rect {
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16,
}

impl Rect {
    pub fn is_empty(&self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub fn intersect(&self, other: &Rect) -> Rect {
        Rect {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }
        Rect {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct RGBColor {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct EventRecord {
    pub what: i16,
    pub message: i64,
    pub when: i64,
    pub where_: Point,
    pub modifiers: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct FontInfo {
    pub ascent: i16,
    pub descent: i16,
    pub widening: i16,
    pub leading: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SFReply {
    pub good: Boolean,
    pub vRefNum: i16,
    pub fType: i64,
    pub fName: [i8; 64],
}

#[repr(C)]
pub struct QDGlobals {
    pub screenBits: QDGlobalsScreenBits,
}

#[repr(C)]
pub struct QDGlobalsScreenBits {
    pub bounds: Rect,
}

// ---------------------------------------------------------------------------
// Port de dessin
// ---------------------------------------------------------------------------

/// Port de dessin : fenêtre ou offscreen. C'est l'objet pointé par
/// `CGrafPtr`/`WindowPtr` côté C (`AcamGetInfoOffscreen` y donne accès).
/// Le tampon contient les indices 0..255 de la LUT système.
#[repr(C)]
pub struct SapPort {
    pub bounds: Rect, // top, left, bottom, right
    pub row_bytes: i16,
    pub depth: i16,
    pub pixels: *mut u8,
    pub pixel_len: usize,
    pub pen_loc: Point,  // v, h : position du crayon
    pub pen_size: Point, // h = largeur, v = hauteur
    pub pen_mode: i16,
    pub fore_index: u8,
    pub back_index: u8,
    pub clip: Rect,
    pub open_rgn: u8,
    pub open_acc: Rect,
    // État texte (QuickDraw) : champ poubelle côté C (`BitMap` opaque).
    pub tx_font: i16,
    pub tx_size: i16,
    pub tx_face: i16,
}

/// Port graphique courant (ce que dessine le C).
pub static CURRENT_PORT: AtomicPtr<SapPort> = AtomicPtr::new(std::ptr::null_mut());

/// Port de la fenêtre principale (`mainWindow`).
pub static WINDOW_PORT: AtomicPtr<SapPort> = AtomicPtr::new(std::ptr::null_mut());

// ---------------------------------------------------------------------------
// Registre des allocations (Ptr/Handle) pour pouvoir les libérer
// ---------------------------------------------------------------------------

pub static ALLOCS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());

pub fn alloc_pixels(len: usize) -> *mut u8 {
    let mut v: Vec<u8> = vec![0u8; len.max(1)];
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    ALLOCS.lock().unwrap().push((p as usize, len.max(1)));
    p
}

pub fn free_pixels(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    let addr = ptr as usize;
    let mut guard = ALLOCS.lock().unwrap();
    if let Some(pos) = guard.iter().position(|(a, _)| *a == addr) {
        let (_, len) = guard.swap_remove(pos);
        unsafe {
            drop(Vec::from_raw_parts(ptr, 0, len.max(1)));
        }
    }
    // Libre. On retente une fois avec un nouvel emplacement de mémoire.
}

// ---------------------------------------------------------------------------
// LUT 256 couleurs
// ---------------------------------------------------------------------------

/// Cube 6-6-6 (216 couleurs) sur les pas 0,51,102,153,204,255, suivi de
/// 40 gris (12,18,...,246). L'index 0 est noir, un gris bien distinct est
/// l'index 255 si besoin. Utilisée par `Color2Index`, `Index2Color` et le
/// rendu egui.
pub const LUT: [[u8; 3]; 256] = build_lut();

const fn build_lut() -> [[u8; 3]; 256] {
    let mut l = [[0u8; 3]; 256];
    let mut i = 0;
    while i < 216 {
        let r = i / 36;
        let g = (i / 6) % 6;
        let b = i % 6;
        l[i] = [step6(r), step6(g), step6(b)];
        i += 1;
    }
    let mut k = 0usize;
    while k < 40 {
        let v = gray(k as u32);
        l[216 + k] = [v, v, v];
        k += 1;
    }
    l
}

const fn step6(v: usize) -> u8 {
    (v as u8) * 51
}

const fn gray(k: u32) -> u8 {
    12 + (k as u8) * 6
}

/// Indice de la LUT le plus proche d'une couleur 8 bits par composante.
pub fn rgb_to_index(r: u8, g: u8, b: u8) -> u8 {
    let mut best = 0u8;
    let mut best_d = i32::MAX;
    for (i, c) in LUT.iter().enumerate() {
        let dr = r as i32 - c[0] as i32;
        let dg = g as i32 - c[1] as i32;
        let db = b as i32 - c[2] as i32;
        let d = dr * dr + dg * dg + db * db;
        if d < best_d {
            best_d = d;
            best = i as u8;
        }
    }
    best
}

#[inline]
pub fn lut_rgb(index: u8) -> (u8, u8, u8) {
    (
        LUT[index as usize][0],
        LUT[index as usize][1],
        LUT[index as usize][2],
    )
}

// ---------------------------------------------------------------------------
// Constantes QuickDraw
// ---------------------------------------------------------------------------

pub const BLACK_COLOR: i16 = 33;
pub const WHITE_COLOR: i16 = 30;
pub const PAT_COPY: i16 = 8;
pub const PAT_OR: i16 = 9;
pub const PAT_XOR: i16 = 10;
pub const PAT_BIC: i16 = 11;

// ---------------------------------------------------------------------------
// Entrées
// ---------------------------------------------------------------------------

/// État des entrées posé par egui (thread principal) et lu par le C.
#[derive(Clone, Copy, Default)]
pub struct InputState {
    /// Position souris normalisée (0..1) dans la fenêtre egui.
    pub mouse_h: f32,
    pub mouse_v: f32,
    pub button: bool,
    /// État clavier aligné sur `KeyMap` du C (`unsigned long[4]`, soit 4 mots
    /// de 64 bits). Pour un keycode Mac `k` (0..127), le bit `((k^24)&31)`
    /// du mot `k>>5` est posé, conformément à `KeyPressed` (SAPWCLAV.C).
    pub keymap: [u64; 4],
}

pub static INPUT: Mutex<InputState> = Mutex::new(InputState {
    mouse_h: 0.5,
    mouse_v: 0.5,
    button: false,
    keymap: [0; 4],
});
