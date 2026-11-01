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

use crate::acam::NewCWindow;
use crate::sapiens::ECHELLE;
use crate::sapiens::sapwbrt::SapiensCreeOffscreen;
use crate::sapiens::sapwcart::Sapwcart;
use crate::shared::{Rect, SapPort};
use std::cell::Cell;
use std::rc::Rc;
use strum::Display;

/// Aire de l'écran, telle que l game's la voit (la fenêtre fait 640x400).
pub(crate) const SCREEN: Rect = Rect {
    top: 0,
    left: 0,
    bottom: 400,
    right: 640,
};

pub(crate) struct Ports {
    /// Port graphique courant (ce que dessine le C).
    pub(crate) current: Rc<Cell<PortId>>,
    pub(crate) mainWindow: SapPort,
    pub(crate) Ecran1: SapPort,
    pub(crate) Ecran2: SapPort,
    pub(crate) debecr: SapPort,
    pub(crate) BuffMouse: SapPort,
    pub(crate) sapwcart: Sapwcart,
}

impl Default for Ports {
    fn default() -> Self {
        Self {
            current: Rc::new(Cell::new(PortId::MainWindow)),
            mainWindow: NewCWindow(&SCREEN),
            Ecran1: SapiensCreeOffscreen(256 * ECHELLE as i16, 100 * ECHELLE as i16),
            Ecran2: SapiensCreeOffscreen(256 * ECHELLE as i16, 100 * ECHELLE as i16),
            debecr: SapiensCreeOffscreen(320 * ECHELLE as i16, 200 * ECHELLE as i16),
            BuffMouse: SapiensCreeOffscreen(64 * ECHELLE as i16, 64 * ECHELLE as i16),
            sapwcart: Sapwcart::default(),
        }
    }
}

impl Ports {
    pub(crate) fn port(&self, port_id: PortId) -> &SapPort {
        match port_id {
            PortId::MainWindow => &self.mainWindow,
            PortId::Ecran1 => &self.Ecran1,
            PortId::Ecran2 => &self.Ecran2,
            PortId::debecr => &self.debecr,
            PortId::BuffMouse => &self.BuffMouse,
            PortId::ecr_carte => &self.sapwcart.ecr_carte,
        }
    }

    pub(crate) fn port_mut(&mut self, port_id: PortId) -> &mut SapPort {
        match port_id {
            PortId::MainWindow => &mut self.mainWindow,
            PortId::Ecran1 => &mut self.Ecran1,
            PortId::Ecran2 => &mut self.Ecran2,
            PortId::debecr => &mut self.debecr,
            PortId::BuffMouse => &mut self.BuffMouse,
            PortId::ecr_carte => &mut self.sapwcart.ecr_carte,
        }
    }

    /// Port de dessin courant.
    pub(crate) fn current(&self) -> &SapPort {
        self.port(self.current_id())
    }

    pub(crate) fn current_mut(&mut self) -> &mut SapPort {
        self.port_mut(self.current_id())
    }

    /// Identifiant du port de dessin courant.
    pub(crate) fn current_id(&self) -> PortId {
        self.current.get()
    }

    /// Rend `p` le port de dessin courant (le `SetPort` de QuickDraw).
    pub(crate) fn set(&self, p: PortId) {
        self.current.set(p);
    }

    /// Dessine sur `p` jusqu'à la fin de la portée, puis restaure le port
    /// précédent — y compris en cas de `return` anticipé.
    ///
    /// ```ignore
    /// let _g = self.sapwind.ports.on(PortId::Debecr);
    /// // ... dessin dans debecr ...
    /// ```
    pub(crate) fn on(&self, p: PortId) -> Guard {
        let g = self.preserve();
        self.set(p);
        g
    }

    /// Ne change pas le port, mais le restaure à la sortie de la portée : utile
    /// autour d'un appel qui change le port courant en douce (boîte de dialogue,
    /// impressions).
    pub(crate) fn preserve(&self) -> Guard {
        Guard {
            current: Rc::clone(&self.current),
            prev: self.current_id(),
        }
    }

    /// Port de la fenêtre principale.
    pub(crate) fn window(&self) -> &SapPort {
        &self.mainWindow
    }

    /// `p` est-il le port de la fenêtre principale ?
    pub(crate) fn is_window(&self, p: PortId) -> bool {
        p == PortId::MainWindow
    }
}

// ---------------------------------------------------------------------------
// Port courant
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Display)]
pub(crate) enum PortId {
    #[default]
    MainWindow,
    Ecran1,
    Ecran2,
    debecr,
    BuffMouse,
    ecr_carte,
}

/// Garde de restoration du port courant (voir [`Ports::on`]).
///
/// N'emprunte pas `Ports` : la portée garde l'accès libre à `self`.
#[must_use]
pub(crate) struct Guard {
    current: Rc<Cell<PortId>>,
    prev: PortId,
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.current.set(self.prev);
    }
}

// ---------------------------------------------------------------------------
// Création / destruction
// ---------------------------------------------------------------------------

/// Alloue un port de `bounds`, profondeur `depth`, échelle 1.0.
pub(crate) fn create(bounds: Rect, depth: i16) -> SapPort {
    let w = (bounds.right - bounds.left).max(0) as usize;
    let h = (bounds.bottom - bounds.top).max(0) as usize;
    SapPort {
        bounds,
        row_bytes: w as i16,
        depth,
        pixels: vec![0; w * h],
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
    }
}

/// Redimensionne le tampon de `p` sur `bounds`, en préservant le contenu
/// commun aux deux aires (l'aire en haut à gauche). Le clip suit l'aire.
pub(crate) fn resize(p: &mut SapPort, bounds: Rect) {
    let w = (bounds.right - bounds.left).max(0) as usize;
    let h = (bounds.bottom - bounds.top).max(0) as usize;
    if w == 0 || h == 0 {
        return;
    }
    let mut pixels = vec![0; w * h];
    let old = p.bounds;
    let old_row = p.row_bytes.max(0) as usize;
    let copy_w = (bounds.right - old.left).min(old.right - old.left).max(0) as usize;
    let copy_h = (bounds.bottom - old.top).min(old.bottom - old.top).max(0) as usize;
    for y in 0..copy_h {
        let src = &p.pixels[y * old_row..y * old_row + copy_w];
        pixels[y * w..y * w + copy_w].copy_from_slice(src);
    }
    p.pixels = pixels;
    p.bounds = bounds;
    p.row_bytes = w as i16;
    p.clip = bounds;
}

impl Ports {
    /// Facteur d'échelle du port courant.
    pub(crate) fn zoom(&self) -> f32 {
        self.current().zoom
    }

    /// Facteur d'échelle de la fenêtre : c'est lui qu'il faut appliquer aux
    /// coordonnées venant de l'écran physique (souris, souris graphique), quel que
    /// soit le port courant au moment de la lecture.
    pub(crate) fn window_zoom(&self) -> f32 {
        self.window().zoom
    }

    /// Fixe l'échelle de la fenêtre (menu « Échelle »). Sans effet avant sa création.
    pub(crate) fn set_window_zoom(&mut self, z: f32) {
        self.mainWindow.zoom = z;
    }
}

impl SapPort {
    /// Pixel à l'offset linéaire `idx` du tampon, `None` hors tampon.
    #[inline]
    pub(crate) fn px(&self, idx: isize) -> Option<u8> {
        usize::try_from(idx)
            .ok()
            .and_then(|i| self.pixels.get(i).copied())
    }

    /// Pixel mutable à l'offset linéaire `idx` du tampon, `None` hors tampon.
    #[inline]
    pub(crate) fn px_mut(&mut self, idx: isize) -> Option<&mut u8> {
        usize::try_from(idx)
            .ok()
            .and_then(|i| self.pixels.get_mut(i))
    }

    /// Écrit `v` à l'offset linéaire `idx`, sans effet hors tampon.
    #[inline]
    pub(crate) fn put(&mut self, idx: isize, v: u8) {
        if let Some(d) = self.px_mut(idx) {
            *d = v;
        }
    }

    /// Met un rectangle à l'échelle du port `p`.
    pub(crate) fn zoomed(&self, r: &Rect) -> Rect {
        let z = self.zoom;
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
        assert_eq!(off.zoom, 1.0);
        assert_eq!(off.zoomed(&r), r);
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
        let mut ports = Ports::default();
        ports.mainWindow = w;
        ports.set_window_zoom(2.0);
        assert_eq!(ports.window_zoom(), 2.0);
        assert_eq!(
            ports.mainWindow.zoomed(&r),
            Rect {
                top: 20,
                left: 40,
                bottom: 60,
                right: 80,
            }
        );
        /* Un hors-écran n'est pas concerné par l'échelle de la fenêtre. */
        let off = create(SCREEN, 8);
        assert_eq!(off.zoomed(&r), r);
    }

    #[test]
    fn guard_restaure_le_port_precedent() {
        let ports = Ports::default();
        ports.set(PortId::Ecran1);
        {
            let _g = ports.on(PortId::debecr);
            assert!(ports.current_id() == PortId::debecr);
            {
                let _g2 = ports.preserve();
                ports.set(PortId::MainWindow);
            }
            assert!(ports.current_id() == PortId::debecr);
        }
        assert!(ports.current_id() == PortId::Ecran1);
    }

    #[test]
    fn resize_preserve_le_contenu_commun() {
        let mut p = create(
            Rect {
                top: 0,
                left: 0,
                bottom: 4,
                right: 4,
            },
            8,
        );
        p.pixels[5] = 42; // (x=1, y=1)
        resize(
            &mut p,
            Rect {
                top: 0,
                left: 0,
                bottom: 8,
                right: 8,
            },
        );
        assert_eq!(p.bounds.bottom, 8);
        assert_eq!(p.row_bytes, 8);
        assert_eq!(p.pixels[8 + 1], 42);
        // Le coin nouveau est à zéro.
        assert_eq!(p.pixels[7 * 8 + 7], 0);
    }
}
