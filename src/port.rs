//! Port de dessin : la fenêtre affichée ou un tampon hors-écran.
//!
//! Remplace les shims `Acam*` / `Carbonise*` du portage d'origine, qui
//! n'existaient que parce qu'un `WindowPtr` Mac et son `GrafPort` sont deux
//! objets distincts. Ici le port *est* la fenêtre : un seul pointeur suffit.
//!
//! Règle unique, appliquée par les primitives : chaque port porte son facteur
//! d'échelle. Il vaut 1.0 partout sauf sur la fenêtre principale, où il suit
//! le menu « Échelle » (le global `SapZoom`). Les dessins à l'écran sont donc
//! mis à l'échelle par le port courant, ceux vers un hors-écran jamais.

use crate::shared::free_pixels;
use crate::shared::{CGrafPtr, CURRENT_PORT, Ptr, Rect, SapPort, WINDOW_PORT, alloc_pixels};
use std::sync::atomic::Ordering;

/// Aire de l'écran, telle que l game's la voit (la fenêtre fait 640x400).
pub(crate) const SCREEN: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 400,
    right: 640,
};

// ---------------------------------------------------------------------------
// Port courant
// ---------------------------------------------------------------------------

/// Rend `p` le port de dessin courant (le `SetPort` de QuickDraw).
pub(crate) fn set(p: CGrafPtr) {
    CURRENT_PORT.store(p as *mut SapPort, Ordering::SeqCst);
}

/// Port de dessin courant.
pub(crate) fn current() -> CGrafPtr {
    CURRENT_PORT.load(Ordering::SeqCst) as CGrafPtr
}

/// Port de la fenêtre principale, ou `null` avant `NewCWindow`.
pub(crate) fn window() -> CGrafPtr {
    WINDOW_PORT.load(Ordering::SeqCst) as CGrafPtr
}

/// `p` est-il le port de la fenêtre principale ?
pub(crate) fn is_window(p: CGrafPtr) -> bool {
    !p.is_null() && p == window()
}

/// Le port courant est-il la fenêtre affichée ?
pub(crate) fn on_screen() -> bool {
    is_window(current())
}

/// Garde de restoration du port courant.
pub(crate) struct Guard(CGrafPtr);

impl Drop for Guard {
    fn drop(&mut self) {
        set(self.0);
    }
}

/// Dessine sur `p` jusqu'à la fin de la portée, puis restaure le port
/// précédent — y compris en cas de `return` anticipé.
///
/// ```ignore
/// let _g = port::on(debecr);
/// // ... dessin dans debecr ...
/// ```
pub(crate) fn on(p: CGrafPtr) -> Guard {
    let g = Guard(current());
    set(p);
    g
}

/// Ne change pas le port, mais le restaure à la sortie de la portée : utile
/// autour d'un appel qui change le port courant en douce (boîte de dialogue,
/// impressions).
pub(crate) fn preserve() -> Guard {
    Guard(current())
}

// ---------------------------------------------------------------------------
// Création / destruction
// ---------------------------------------------------------------------------

/// Alloue un port de `bounds`, profondeur `depth`, échelle 1.0.
pub(crate) fn create(bounds: Rect, depth: i16) -> CGrafPtr {
    let w = (bounds.right - bounds.left).max(0) as usize;
    let h = (bounds.bottom - bounds.top).max(0) as usize;
    let port = Box::new(SapPort {
        bounds,
        row_bytes: w as i16,
        depth,
        pixels: alloc_pixels(w * h),
        pixel_len: w * h,
        pen_loc: crate::shared::Point { v: 0, h: 0 },
        pen_size: crate::shared::Point { v: 1, h: 1 },
        pen_mode: crate::shared::PAT_COPY,
        fore_index: 0,
        back_index: 255,
        clip: bounds,
        open_rgn: 0,
        open_acc: Rect::default(),
        tx_font: 0,
        tx_size: 12,
        tx_face: 0,
        zoom: 1.0,
    });
    Box::into_raw(port) as CGrafPtr
}

/// Libère un port hors-écran. Le port de la fenêtre est laissé en place.
pub(crate) fn close(p: CGrafPtr) {
    if p.is_null() || is_window(p) {
        return;
    }
    unsafe {
        let pp = p as *mut SapPort;
        free_pixels((*pp).pixels);
        drop(Box::from_raw(pp));
    }
}

/// Redimensionne le tampon de `p` sur `bounds`, en préservant le contenu
/// commun aux deux aires (l'aire en haut à gauche). Le clip suit l'aire.
pub(crate) fn resize(p: CGrafPtr, bounds: Rect) {
    let Some(info) = info(p) else { return };
    let w = (bounds.right - bounds.left).max(0) as usize;
    let h = (bounds.bottom - bounds.top).max(0) as usize;
    if w == 0 || h == 0 {
        return;
    }
    let pixels = alloc_pixels(w * h);
    unsafe {
        let pp = &mut *(p as *mut SapPort);
        let copy_w = (bounds.right - info.bounds.left).min(info.bounds.right - info.bounds.left);
        let copy_h = (bounds.bottom - info.bounds.top).min(info.bounds.bottom - info.bounds.top);
        for y in 0..copy_h.max(0) as usize {
            let src = info.pixels.add(y as usize * info.row_bytes.max(0) as usize);
            let dst = pixels.add(y * w);
            std::ptr::copy_nonoverlapping(src, dst, copy_w.max(0) as usize);
        }
        free_pixels(pp.pixels);
        pp.pixels = pixels;
        pp.pixel_len = w * h;
        pp.bounds = bounds;
        pp.row_bytes = w as i16;
        pp.clip = bounds;
    }
}

// ---------------------------------------------------------------------------
// Contenu d'un port
// ---------------------------------------------------------------------------

/// Ce qu'il faut pour lire ou écrire dans le tampon d'un port.
#[derive(Clone, Copy)]
pub(crate) struct PortInfo {
    /// Premier pixel, en indices 0..255 de la LUT système.
    pub(crate) pixels: Ptr,
    /// Aire du port.
    pub(crate) bounds: Rect,
    /// Octets par ligne (`row_bytes`).
    pub(crate) row_bytes: i16,
    pub(crate) depth: i16,
}

/// Description du tampon de `p`, ou `None` si le port n'existe pas.
pub(crate) fn info(p: CGrafPtr) -> Option<PortInfo> {
    if p.is_null() {
        return None;
    }
    unsafe {
        let pp = &*(p as *const SapPort);
        Some(PortInfo {
            pixels: pp.pixels,
            bounds: pp.bounds,
            row_bytes: pp.row_bytes,
            depth: pp.depth,
        })
    }
}

/// Aire du port `p`, ou un rect vide.
pub(crate) fn bounds(p: CGrafPtr) -> Rect {
    info(p).map(|i| i.bounds).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Échelle
// ---------------------------------------------------------------------------

/// Facteur d'échelle du port `p` : 1.0 sauf pour la fenêtre.
pub(crate) fn zoom_of(p: CGrafPtr) -> f32 {
    if p.is_null() {
        1.0
    } else {
        unsafe { (*(p as *const SapPort)).zoom }
    }
}

/// Facteur d'échelle du port courant.
pub(crate) fn zoom() -> f32 {
    zoom_of(current())
}

/// Facteur d'échelle de la fenêtre : c'est lui qu'il faut appliquer aux
/// coordonnées venant de l'écran physique (souris, souris graphique), quel que
/// soit le port courant au moment de la lecture.
pub(crate) fn window_zoom() -> f32 {
    zoom_of(window())
}

/// Fixe l'échelle de la fenêtre (menu « Échelle »). Sans effet avant sa création.
pub(crate) fn set_window_zoom(z: f32) {
    let w = window();
    if !w.is_null() {
        unsafe {
            (*(w as *mut SapPort)).zoom = z;
        }
    }
}

/// Met un rectangle à l'échelle du port `p`.
pub(crate) fn zoomed(p: CGrafPtr, r: &Rect) -> Rect {
    let z = zoom_of(p);
    if z == 1.0 {
        return *r;
    }
    let f = |v: i16| (v as f32 * z) as i16;
    Rect {
        top: f(r.top),
        left: f(r.left),
        bottom: f(r.bottom),
        right: f(r.right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hors_ecran_toujours_echelle_1() {
        let off = create(
            Rect {
                top: 0,
                left: 0,
                bottom: 200,
                right: 320,
            },
            8,
        );
        let r = Rect {
            top: 10,
            left: 20,
            bottom: 30,
            right: 40,
        };
        assert_eq!(zoom_of(off), 1.0);
        assert_eq!(zoomed(off, &r), r);
        close(off);
    }

    #[test]
    fn fenetre_mise_a_echelle_par_le_menu() {
        let r = Rect {
            top: 10,
            left: 20,
            bottom: 30,
            right: 40,
        };
        let w = create(SCREEN, 8);
        WINDOW_PORT.store(w as *mut SapPort, Ordering::SeqCst);
        set_window_zoom(2.0);
        assert_eq!(window_zoom(), 2.0);
        assert_eq!(
            zoomed(w, &r),
            Rect {
                top: 20,
                left: 40,
                bottom: 60,
                right: 80,
            }
        );
        /* Un hors-écran n'est pas concerné par l'échelle de la fenêtre. */
        let off = create(SCREEN, 8);
        assert_eq!(zoomed(off, &r), r);
        /* close() laisse le port de la fenêtre en place. */
        close(w);
        assert_eq!(window(), w);
        close(off);
        WINDOW_PORT.store(core::ptr::null_mut(), Ordering::SeqCst);
    }

    #[test]
    fn zoom_de_null_vaut_1() {
        assert_eq!(zoom_of(core::ptr::null_mut()), 1.0);
    }

    #[test]
    fn resize_preserve_le_contenu_commun() {
        let p = create(
            Rect {
                top: 0,
                left: 0,
                bottom: 4,
                right: 4,
            },
            8,
        );
        unsafe {
            let pp = &mut *(p as *mut SapPort);
            *pp.pixels.add(5) = 42; // (x=1, y=1)
        }
        resize(
            p,
            Rect {
                top: 0,
                left: 0,
                bottom: 8,
                right: 8,
            },
        );
        let i = info(p).unwrap();
        assert_eq!(i.bounds.bottom, 8);
        assert_eq!(i.row_bytes, 8);
        unsafe {
            assert_eq!(*i.pixels.add(8 + 1), 42);
            // Le coin nouveau est à zéro.
            assert_eq!(*i.pixels.add(7 * 8 + 7), 0);
        }
        close(p);
    }
}
