// ---------------------------------------------------------------------------
// Langue du jeu (menu « Fichier → Langue »)
// ---------------------------------------------------------------------------
// Deux notions distinctes :
//   - `Langue`   : le global C historique (0 = fr, 1 = gb, 2 = es) lu par le
//                  code porté pour choisir textes et ressources.
//   - `LangMode` : le choix de l'utilisateur côté Rust (Auto/Fr/En/Es), qui
//                  décide de la `Langue` effective (Auto = langue du système).
//
// L'initialisation vient d'un paramètre de ligne de commande (`--lang`, traité
// dans `main.rs`) ; à défaut elle est relue du fichier `lang` persisté, puis
// retombe sur `Auto`. Le menu egui persiste le choix via `save_lang_mode`.

use std::sync::atomic::{AtomicI16, Ordering};
use sys_locale::get_locale;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Sélection de l'utilisateur : Auto = langue du système.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum LangMode {
    Auto,
    Fr,
    En,
    Es,
}

impl LangMode {
    /// Libellé du mode dans la langue courante du jeu.
    pub(crate) fn label(&self, lang: Langue) -> &'static str {
        match (lang, self) {
            (Langue::FR, LangMode::Auto) => "Auto",
            (Langue::FR, LangMode::Fr) => "Français",
            (Langue::FR, LangMode::En) => "Anglais",
            (Langue::FR, LangMode::Es) => "Espagnol",
            (Langue::ES, LangMode::Auto) => "Auto",
            (Langue::ES, LangMode::Fr) => "Francés",
            (Langue::ES, LangMode::En) => "Inglés",
            (Langue::ES, LangMode::Es) => "Español",
            (_, LangMode::Auto) => "Auto",
            (_, LangMode::Fr) => "French",
            (_, LangMode::En) => "English",
            (_, LangMode::Es) => "Spanish",
        }
    }

    /// Mode correspondant à une valeur de `--lang` (`auto`/`fr`/`en`/`es`).
    pub(crate) fn from_arg(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "auto" => Some(LangMode::Auto),
            "fr" | "fr-fr" | "francais" | "français" => Some(LangMode::Fr),
            "en" | "en-us" | "english" | "anglais" => Some(LangMode::En),
            "es" | "es-es" | "spanish" | "espagnol" => Some(LangMode::Es),
            _ => None,
        }
    }

    /// Représentation persistée dans le fichier `lang`.
    fn as_str(&self) -> &'static str {
        match self {
            LangMode::Auto => "auto",
            LangMode::Fr => "fr",
            LangMode::En => "en",
            LangMode::Es => "es",
        }
    }
}

/// Langue C effective (discriminants attendus par le code porté et les
/// ressources : 0 = fr, 1 = gb, 2 = es).
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Langue {
    FR = 0,
    GB = 1,
    ES = 2,
}

impl From<i16> for Langue {
    fn from(value: i16) -> Self {
        match value {
            0 => Langue::FR,
            2 => Langue::ES,
            _ => Langue::GB,
        }
    }
}

// ---------------------------------------------------------------------------
// Global C `langue`
// ---------------------------------------------------------------------------
// `langue` est partagé entre le thread graphique egui et le thread jeu (et lu
// par des fonctions libres). Encapsulé derrière `langue()` / `set_langue()` ;
// la statique `LANGUE` reste privée.

static LANGUE: AtomicI16 = AtomicI16::new(-1);

#[inline]
pub(crate) fn langue_i16() -> i16 {
    LANGUE.load(Ordering::Relaxed)
}

#[inline]
pub(crate) fn set_langue(l: Langue) {
    LANGUE.store(l as i16, Ordering::Relaxed);
}

/// Valeur brute du global C `langue` (peut valoir -1 tant que rien ne l'a
/// fixée ; l'écran egui la corrige avec `set_game_language`).
pub(crate) fn langue() -> Langue {
    langue_i16().into()
}

// ---------------------------------------------------------------------------
// Détection système / mapping mode → langue
// ---------------------------------------------------------------------------

/// Langue du système : préfixe `fr` → Français, `es` → Espagnol, sinon
/// (anglais ou autre) → Anglais.
fn system_lang() -> Langue {
    let locale = get_locale()
        .unwrap_or_else(|| String::from("en-US"))
        .to_ascii_lowercase();
    if locale.starts_with("fr") {
        return Langue::FR;
    }
    if locale.starts_with("es") {
        return Langue::ES;
    }
    Langue::GB
}

/// Langue C effective (« langue » du jeu) pour le mode choisi.
pub(crate) fn effective_lang(mode: LangMode) -> Langue {
    match mode {
        LangMode::Fr => Langue::FR,
        LangMode::En => Langue::GB,
        LangMode::Es => Langue::ES,
        LangMode::Auto => system_lang(),
    }
}

// ---------------------------------------------------------------------------
// Persistance (fichier `lang` sous le dossier de configuration utilisateur)
// ---------------------------------------------------------------------------

/// Dossier de configuration utilisateur (préférences, langue).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn config_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    if cfg!(target_os = "macos") {
        std::path::Path::new(&home).join("Library/Application Support/Sapiens")
    } else {
        std::path::Path::new(&home).join(".config/sapiens")
    }
}

/// Clé de stockage (fichier `lang` sous `config_dir` en natif).
const LANG_KEY: &str = "lang";

pub(crate) fn load_lang_mode() -> LangMode {
    crate::platform::storage::read(LANG_KEY)
        .and_then(|bytes| LangMode::from_arg(String::from_utf8_lossy(&bytes).trim()))
        .unwrap_or(LangMode::Auto)
}

pub(crate) fn save_lang_mode(mode: LangMode) {
    let _ = crate::platform::storage::write(LANG_KEY, mode.as_str().as_bytes());
}

// ---------------------------------------------------------------------------
// Initialisation haut niveau
// ---------------------------------------------------------------------------

/// Langue du jeu : la sélection (Auto/Fr/En/Es) est imposée côté Rust à
/// chaque frame ; cet appel écrit le global C `langue` et reconstruit la barre
/// de menus (IDs langue). Les touches `HOMME`/`FEMME`/`OUI` sont recalculées
/// côté jeu par `Sapglob::init_and_check_texts` (defsap / init_graph).
pub(crate) fn set_game_language(l: Langue) {
    set_langue(l);
    *crate::acam::MENU_MODEL.lock().unwrap() = Some(crate::acam::build_model());
}
