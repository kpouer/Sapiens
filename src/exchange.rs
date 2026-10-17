//! Pont d'échange entre le thread C du jeu et le thread egui.
//!
//! Un static par sens et par primitive (jamais un verrou unique pour tout) :
//!
//! - **egui → thread C** : commandes de menu (boîte 0-swap) et état des
//!   entrées (souris/clavier), alimenté chaque frame par egui (`src/main.rs`)
//!   et consommé par les stubs `GetMouse`/`Button`/`GetKeys`.
//! - **thread C → egui** : curseur (forme/caché) et titre de fenêtre ; egui
//!   les relit en tête de `update()`. Les modèles Mac mutables (barre de
//!   menus) restent chez l'émulation concernée et sont exposés par snapshot
//!   (`acam.rs`).

use log::{error, info};
use std::fmt::Display;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering};

// ---------------------------------------------------------------------------
// Curseur (thread C → egui)
// ---------------------------------------------------------------------------
//
// Deux axes séparés, comme sur le Mac : `SetCursor` / `CarboniseSetCursorArrow`
// posent la *forme*, `HideCursor` / `ShowCursor` posent la *visibilité*. Le
// thread egui applique : caché → curseur invisible, sinon la forme, en tête de
// `update()` (src/main.rs).

/// Forme de curseur demandée par le code C.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CursorShape {
    Arrow = 0,
    Watch = 1,
}

impl Display for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CursorShape::Arrow => "Arrow",
            CursorShape::Watch => "Watch",
        };
        write!(f, "{}", str)
    }
}

impl From<u8> for CursorShape {
    fn from(value: u8) -> Self {
        match value {
            0 => CursorShape::Arrow,
            1 => CursorShape::Watch,
            _ => {
                error!("Unexpected cursor shape {value}");
                CursorShape::Arrow
            }
        }
    }
}

static CURSOR_SHAPE: AtomicU8 = AtomicU8::new(CursorShape::Arrow as u8);
static CURSOR_HIDDEN: AtomicBool = AtomicBool::new(false);

/// Pose la forme du curseur (thread C). Log seulement si elle change : le
/// code jeu la reposait à chaque opération.
pub(crate) fn set_cursor_shape(shape: CursorShape) {
    if CURSOR_SHAPE.swap(shape as u8, Ordering::SeqCst) != shape as u8 {
        info!("curseur forme = {shape}");
    }
}

pub(crate) fn cursor_shape() -> CursorShape {
    CURSOR_SHAPE.load(Ordering::SeqCst).into()
}

/// Masque (`true`) ou réaffiche (`false`) le curseur, indépendamment de la
/// forme en cours (`HideCursor` / `ShowCursor`, SAPWCLAV.C).
pub(crate) fn set_cursor_hidden(hidden: bool) {
    if CURSOR_HIDDEN.swap(hidden, Ordering::SeqCst) != hidden {
        info!("curseur caché = {hidden}");
    }
}

pub(crate) fn cursor_hidden() -> bool {
    CURSOR_HIDDEN.load(Ordering::SeqCst)
}

// ---------------------------------------------------------------------------
// Titre de fenêtre (thread C → egui)
// ---------------------------------------------------------------------------

/// Dernier titre posé par `SetWTitle` (écriture rare : écrasement du précédent
/// plutôt qu'une file, on ne veut que le titre courant).
static TITLE: Mutex<Option<String>> = Mutex::new(None);

/// Depose le titre de la fenêtre (thread C). Repris par egui en tête de
/// `update()`, qui le passe à `ViewportCommand::Title`.
pub(crate) fn set_title(title: String) {
    let mut g = TITLE.lock().unwrap();
    if g.as_deref() != Some(title.as_str()) {
        info!("titre fenêtre = {title:?}");
    }
    *g = Some(title);
}

/// Reprise du titre en attente, s'il a changé (cote thread egui).
pub(crate) fn take_title() -> Option<String> {
    TITLE.lock().unwrap().take()
}

/* Commande de menu en attente : posee par le thread egui (src/main.rs, barre
de menus et raccourcis Cmd), reprise par le thread C dans GereInterface, car
le `&mut Sapiens` du jeu n'existe que sur le thread C. 0 = rien en attente
(MenuSelect/MenuKey renvoient 0 quand le clic n'est pas dans un menu). */
static MENU_COMMAND: AtomicI32 = AtomicI32::new(0);

/// Depose une commande de menu depuis le thread egui (barre de menus, raccourcis
/// Cmd). Elle sera executee par `GereInterface`, sur le thread C.
pub(crate) fn set_menu_command(menu_result: i32) {
    MENU_COMMAND.store(menu_result, Ordering::SeqCst);
}

/// Reprise et remise a zero de la commande en attente (cote thread C).
pub(crate) fn take_menu_command() -> Option<i32> {
    MENU_COMMAND.swap(0, Ordering::SeqCst).into()
}

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
