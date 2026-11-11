//! Pont d'échange entre le thread C du jeu et le thread egui.
//!
//! Un static par sens et par primitive (jamais un verrou unique pour tout) :
//!
//! - **egui → thread C** : commandes de menu (boîte 0-swap), état des
//!   entrées (souris/clavier) et file d'événements Mac (`keyDown`, cf.
//!   `event_queue.rs`), alimentés chaque frame par egui (`src/main.rs`) et
//!   consommés par les stubs `GetMouse`/`Button`/`GetKeys` puis
//!   `GetNextEvent`.
//! - **thread C → egui** : curseur (forme/caché) et titre de fenêtre ; egui
//!   les relit en tête de `update()`. Les modèles Mac mutables (barre de
//!   menus) restent chez l'émulation concernée et sont exposés par snapshot
//!   (`acam.rs`).

pub(crate) mod event_queue;

pub(crate) use event_queue::{clear_events, push_event};

use crate::langue::LangMode;
use crate::platform::Instant;
use crate::platform::sync::Mutex;
use crate::shared::{KeyMap, SapPort};
use log::{error, info};
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;
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
pub(crate) enum Cursor {
    Arrow = 0,
    Watch = 1,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Cursor::Arrow => "Arrow",
            Cursor::Watch => "Watch",
        };
        write!(f, "{}", str)
    }
}

impl From<u8> for Cursor {
    fn from(value: u8) -> Self {
        match value {
            0 => Cursor::Arrow,
            1 => Cursor::Watch,
            _ => {
                error!("Unexpected cursor shape {value}");
                Cursor::Arrow
            }
        }
    }
}

static CURSOR_SHAPE: AtomicU8 = AtomicU8::new(Cursor::Arrow as u8);
static CURSOR_HIDDEN: AtomicBool = AtomicBool::new(false);

/// Pose la forme du curseur (thread C). Log seulement si elle change : le
/// code jeu la reposait à chaque opération.
pub(crate) fn set_cursor_shape(shape: Cursor) {
    if CURSOR_SHAPE.swap(shape as u8, Ordering::SeqCst) != shape as u8 {
        info!("curseur forme = {shape}");
    }
}

pub(crate) fn cursor_shape() -> Cursor {
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
le `&mut Sapiens` du jeu n'existe que sur le thread C. 0 = rien en attente. */
static MENU_COMMAND: AtomicI32 = AtomicI32::new(0);

/// Depose une commande de menu depuis le thread egui (barre de menus, raccourcis
/// Cmd). Elle sera executee par `GereInterface`, sur le thread C.
pub(crate) fn set_menu_command(menu_result: i32) {
    MENU_COMMAND.store(menu_result, Ordering::SeqCst);
}

/// Reprise et remise a zero de la commande en attente (cote thread C).
pub(crate) fn take_menu_command() -> Option<i32> {
    let command = MENU_COMMAND.swap(0, Ordering::SeqCst);
    (command != 0).then_some(command)
}

// ---------------------------------------------------------------------------
// Image de la fenêtre (thread C → egui)
// ---------------------------------------------------------------------------

/// Copie du tampon d'indices du port fenêtre, sans padding de lignes.
pub(crate) struct WindowFrame {
    pub(crate) w: usize,
    pub(crate) h: usize,
    /// `w * h` indices de la LUT système.
    pub(crate) pixels: Vec<u8>,
}

/// Dernière image publiée, pas encore reprise par egui (« dernière valeur » :
/// une image non lue est simplement écrasée). Le verrou n'est tenu que le temps
/// d'échanger l'`Option`.
static WINDOW_FRAME: Mutex<Option<WindowFrame>> = Mutex::new(None);
static LAST_PUBLISH: Mutex<Option<Instant>> = Mutex::new(None);
const PUBLISH_INTERVAL: Duration = Duration::from_millis(15);

/// Publie une copie du port fenêtre (thread C), au plus une fois par
/// `PUBLISH_INTERVAL` sauf si `force`.
pub(crate) fn publish_window_frame(port: &SapPort, force: bool) {
    {
        let mut last = LAST_PUBLISH.lock().unwrap();
        if !force && last.is_some_and(|t| t.elapsed() < PUBLISH_INTERVAL) {
            return;
        }
        *last = Some(Instant::now());
    }
    let w = (port.bounds.right - port.bounds.left).max(0) as usize;
    let h = (port.bounds.bottom - port.bounds.top).max(0) as usize;
    let row = port.row_bytes.max(0) as usize;
    if w == 0 || h == 0 || row < w || port.pixels.len() < (h - 1) * row + w {
        return;
    }
    let mut pixels = Vec::with_capacity(w * h);
    for y in 0..h {
        pixels.extend_from_slice(&port.pixels[y * row..y * row + w]);
    }
    *WINDOW_FRAME.lock().unwrap() = Some(WindowFrame { w, h, pixels });
}

/// Reprise de l'image publiée depuis la dernière lecture (côté thread egui).
pub(crate) fn take_window_frame() -> Option<WindowFrame> {
    WINDOW_FRAME.lock().unwrap().take()
}

// ---------------------------------------------------------------------------
// Entrées
// ---------------------------------------------------------------------------

/// Les `where_` des événements souris sont normalisés de 0 à cette valeur ;
/// le thread C les convertit en coordonnées fenêtre (`Ports::event_point`).
pub(crate) const MOUSE_EVENT_SCALE: i16 = 32767;

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
    pub(crate) keymap: KeyMap,
}

pub(crate) static INPUT: Mutex<InputState> = Mutex::new(InputState {
    mouse_h: 0.5,
    mouse_v: 0.5,
    button: false,
    keymap: [0; 4],
});

// ---------------------------------------------------------------------------
// Dialogues modaux (thread C → egui → thread C)
// ---------------------------------------------------------------------------
//
// Les appels Toolbox `Alert`, les préférences et les boîtes de choix de
// fichiers sont bloquants dans le code d'origine. Le thread C dépose donc une
// requête, puis attend la réponse ; egui (seul à pouvoir ouvrir `rfd` et une
// fenêtre modale) la traite dans `update()` et renvoie le résultat.

#[derive(Clone)]
pub(crate) struct AlertRequest {
    pub(crate) alert_id: i16,
    pub(crate) params: [String; 4],
}

#[derive(Clone, Copy)]
pub(crate) struct PreferencesRequest {
    pub(crate) is_bruitages: bool,
    pub(crate) music: bool,
    pub(crate) lissage_police: bool,
    pub(crate) mode_joueur2: i16,
}

#[derive(Clone, Copy)]
pub(crate) struct PreferencesResult {
    pub(crate) is_bruitages: bool,
    pub(crate) music: bool,
    pub(crate) lissage_police: bool,
    pub(crate) mode_joueur2: i16,
    pub(crate) lang_mode: LangMode,
}

#[derive(Clone)]
pub(crate) struct FileRequest {
    pub(crate) extension: String,
    pub(crate) default_name: String,
}

pub(crate) enum UiRequest {
    Alert(AlertRequest),
    Preferences(PreferencesRequest),
    OpenFile(FileRequest),
    SaveFile(FileRequest),
}

pub(crate) enum UiResponse {
    Alert(i16),
    Preferences(Option<PreferencesResult>),
    File(Option<PathBuf>),
}

struct PendingRequest {
    request: UiRequest,
    reply: Sender<UiResponse>,
}

static PENDING_REQUEST: Mutex<Option<PendingRequest>> = Mutex::new(None);

fn request(request: UiRequest) -> Receiver<UiResponse> {
    let (tx, rx) = channel();
    let mut pending = PENDING_REQUEST.lock().unwrap();
    if pending.is_some() {
        error!("une requête d'interface était déjà en attente");
    }
    *pending = Some(PendingRequest { request, reply: tx });
    rx
}

/// Reprise de la requête en attente par egui. La réponse doit être envoyée sur
/// le canal retourné, sinon le thread C reste bloqué.
pub(crate) fn take_ui_request() -> Option<(UiRequest, Sender<UiResponse>)> {
    PENDING_REQUEST
        .lock()
        .unwrap()
        .take()
        .map(|p| (p.request, p.reply))
}

/// Affiche une alerte et attend l'item de bouton choisi (thread C).
pub(crate) fn request_alert(alert_id: i16, params: [String; 4]) -> i16 {
    match request(UiRequest::Alert(AlertRequest { alert_id, params }))
        .recv()
        .unwrap_or(UiResponse::Alert(0))
    {
        UiResponse::Alert(item) => item,
        _ => 0,
    }
}

/// Affiche la boîte de préférences (thread C). `None` = annulation.
pub(crate) fn request_preferences(initial: PreferencesRequest) -> Option<PreferencesResult> {
    match request(UiRequest::Preferences(initial))
        .recv()
        .unwrap_or(UiResponse::Preferences(None))
    {
        UiResponse::Preferences(result) => result,
        _ => None,
    }
}

/// Demande un fichier à charger (thread C).
#[cfg(not(test))]
pub(crate) fn request_open_file(extension: &str, default_name: &str) -> Option<PathBuf> {
    let req = FileRequest {
        extension: extension.to_string(),
        default_name: default_name.to_string(),
    };
    match request(UiRequest::OpenFile(req))
        .recv()
        .unwrap_or(UiResponse::File(None))
    {
        UiResponse::File(path) => path,
        _ => None,
    }
}

/// Demande un nom de fichier pour sauvegarder (thread C).
#[cfg(not(test))]
pub(crate) fn request_save_file(extension: &str, default_name: &str) -> Option<PathBuf> {
    let req = FileRequest {
        extension: extension.to_string(),
        default_name: default_name.to_string(),
    };
    match request(UiRequest::SaveFile(req))
        .recv()
        .unwrap_or(UiResponse::File(None))
    {
        UiResponse::File(path) => path,
        _ => None,
    }
}

#[cfg(test)]
thread_local! {
    pub(crate) static TEST_FILE: std::cell::RefCell<Option<PathBuf>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn request_open_file(_extension: &str, _default_name: &str) -> Option<PathBuf> {
    TEST_FILE.with(|p| p.borrow().clone())
}

#[cfg(test)]
pub(crate) fn request_save_file(_extension: &str, _default_name: &str) -> Option<PathBuf> {
    TEST_FILE.with(|p| p.borrow().clone())
}
