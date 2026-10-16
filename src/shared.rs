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

pub(crate) type Boolean = bool;
pub(crate) type Handle = *mut *mut c_void;
pub(crate) type Ptr = *mut u8;
pub(crate) type CGrafPtr = *mut c_void;
pub(crate) type DialogPtr = *mut c_void;
pub(crate) type RgnHandle = *mut c_void;
pub(crate) type ControlHandle = *mut c_void;
pub(crate) type Cursor = *mut c_void;
pub(crate) type SndListHandle = *mut *mut c_void;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Pic {
    pub(crate) picSize: i16,
    pub(crate) picFrame: Rect,
}
pub(crate) type PicHandle = *mut *mut Pic;
pub(crate) type TEHandle = *mut *mut TEText;

pub(crate) type StringPtr = *mut u8;
pub(crate) type KeyMap = *mut u64;
pub(crate) type ControlActionProcPtr =
    ::std::option::Option<unsafe extern "C" fn(control: ControlHandle, part: i16)>;

pub(crate) type ControlActionUPP = ControlActionProcPtr;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct Point {
    pub(crate) h: i16,
    pub(crate) v: i16,
}

impl Point {
    pub(crate) const fn new(h: i16, v: i16) -> Self {
        Self { h, v }
    }

    pub(crate) const fn zoom(mut self, zoom: f32) -> Self {
        self.v = (self.v as f32 / zoom) as i16;
        self.h = (self.h as f32 / zoom) as i16;
        self
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct Rect {
    pub(crate) top: i16,
    pub(crate) left: i16,
    pub(crate) bottom: i16,
    pub(crate) right: i16,
}

impl Rect {
    pub(crate) const fn new(left: i16, top: i16, right: i16, bottom: i16) -> Self {
        Rect {
            top,
            left,
            bottom,
            right,
        }
    }

    pub(crate) const fn OffsetRect(&mut self, dh: i16, dv: i16) {
        self.left = self.left.wrapping_add(dh);
        self.right = self.right.wrapping_add(dh);
        self.top = self.top.wrapping_add(dv);
        self.bottom = self.bottom.wrapping_add(dv);
    }

    pub(crate) const fn InsetRect(&mut self, dh: i16, dv: i16) {
        self.left = self.left.wrapping_add(dh);
        self.right = self.right.wrapping_sub(dh);
        self.top = self.top.wrapping_add(dv);
        self.bottom = self.bottom.wrapping_sub(dv);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub(crate) fn intersect(&self, other: &Rect) -> Rect {
        Rect {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub(crate) fn union(&self, other: &Self) -> Self {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }
        Self {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }

    pub(crate) const fn pt_in_rect(&self, pt: Point) -> bool {
        pt.h >= self.left && pt.h < self.right && pt.v >= self.top && pt.v < self.bottom
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct RGBColor {
    pub(crate) red: u16,
    pub(crate) green: u16,
    pub(crate) blue: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct EventRecord {
    pub(crate) what: i16,
    pub(crate) message: i64,
    pub(crate) when: i64,
    pub(crate) where_: Point,
    pub(crate) modifiers: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct FontInfo {
    pub(crate) ascent: i16,
    pub(crate) descent: i16,
    pub(crate) widening: i16,
    pub(crate) leading: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct SFReply {
    pub(crate) good: Boolean,
    pub(crate) vRefNum: i16,
    pub(crate) fType: i64,
    pub(crate) fName: [i8; 64],
}

#[repr(C)]
pub(crate) struct QDGlobals {
    pub(crate) screenBits: QDGlobalsScreenBits,
}

#[repr(C)]
pub(crate) struct QDGlobalsScreenBits {
    pub(crate) bounds: Rect,
}

// ---------------------------------------------------------------------------
// Port de dessin
// ---------------------------------------------------------------------------

/// Port de dessin : fenêtre ou offscreen. C'est l'objet pointé par
/// `CGrafPtr`/`WindowPtr` côté C ; `crate::port` en est le gestionnaire.
/// Le tampon contient les indices 0..255 de la LUT système.
#[repr(C)]
pub(crate) struct SapPort {
    pub(crate) bounds: Rect, // top, left, bottom, right
    pub(crate) row_bytes: i16,
    pub(crate) depth: i16,
    pub(crate) pixels: *mut u8,
    pub(crate) pixel_len: usize,
    pub(crate) pen_loc: Point,  // v, h : position du crayon
    pub(crate) pen_size: Point, // h = largeur, v = hauteur
    pub(crate) pen_mode: i16,
    pub(crate) fore_index: u8,
    pub(crate) back_index: u8,
    pub(crate) clip: Rect,
    pub(crate) open_rgn: u8,
    pub(crate) open_acc: Rect,
    // État texte (QuickDraw) : champ poubelle côté C (`BitMap` opaque).
    pub(crate) tx_font: i16,
    pub(crate) tx_size: i16,
    pub(crate) tx_face: i16,
    /// Facteur d'échelle de ce port : 1.0 sauf pour la fenêtre principale,
    /// qui suit le menu « Échelle » (`SapZoom`).
    pub(crate) zoom: f32,
}

/// Port graphique courant (ce que dessine le C).
pub(crate) static CURRENT_PORT: AtomicPtr<SapPort> = AtomicPtr::new(std::ptr::null_mut());

/// Port de la fenêtre principale (`mainWindow`).
pub(crate) static WINDOW_PORT: AtomicPtr<SapPort> = AtomicPtr::new(std::ptr::null_mut());

// ---------------------------------------------------------------------------
// Registre des allocations (Ptr/Handle) pour pouvoir les libérer
// ---------------------------------------------------------------------------

#[deprecated]
pub(crate) static ALLOCS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());

#[deprecated]
pub(crate) fn alloc_pixels(len: usize) -> *mut u8 {
    let mut v: Vec<u8> = vec![0u8; len.max(1)];
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    ALLOCS.lock().unwrap().push((p as usize, len.max(1)));
    p
}

#[deprecated]
pub(crate) fn free_pixels(ptr: *mut u8) {
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
pub(crate) const LUT: [[u8; 3]; 256] = build_lut();

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
pub(crate) fn rgb_to_index(r: u8, g: u8, b: u8) -> u8 {
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
pub(crate) fn lut_rgb(index: u8) -> (u8, u8, u8) {
    (
        LUT[index as usize][0],
        LUT[index as usize][1],
        LUT[index as usize][2],
    )
}

// ---------------------------------------------------------------------------
// Constantes QuickDraw
// ---------------------------------------------------------------------------

pub(crate) const BLACK_COLOR: i16 = 33;
pub(crate) const WHITE_COLOR: i16 = 30;
pub(crate) const PAT_COPY: i16 = 8;
pub(crate) const PAT_OR: i16 = 9;
pub(crate) const PAT_XOR: i16 = 10;
pub(crate) const PAT_BIC: i16 = 11;

// ---------------------------------------------------------------------------
// Entrées
// ---------------------------------------------------------------------------

/// État des entrées posé par egui (thread principal) et lu par le C.
#[derive(Clone, Copy, Default)]
pub(crate) struct InputState {
    /// Position souris normalisée (0..1) dans la fenêtre egui.
    pub(crate) mouse_h: f32,
    pub(crate) mouse_v: f32,
    pub(crate) button: bool,
    /// État clavier aligné sur `KeyMap` du C (`unsigned long[4]`, soit 4 mots
    /// de 64 bits). Pour un keycode Mac `k` (0..127), le bit `((k^24)&31)`
    /// du mot `k>>5` est posé, conformément à `KeyPressed` (SAPWCLAV.C).
    pub(crate) keymap: [u64; 4],
}

pub(crate) static INPUT: Mutex<InputState> = Mutex::new(InputState {
    mouse_h: 0.5,
    mouse_v: 0.5,
    button: false,
    keymap: [0; 4],
});
