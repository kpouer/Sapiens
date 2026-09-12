// ---------------------------------------------------------------------------
// Langue du jeu (menu « Fichier → Langue »)
// ---------------------------------------------------------------------------
// Le choix de l'utilisateur est persisté côté Rust (`langue` C est ensuite
// imposé à chaque frame) : Auto = langue du système (Français/Espagnol si le
// système est fr/es, sinon Anglais). Les trois langues du jeu (0/1/2) sont
// conservées.

use sys_locale::get_locale;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LangMode {
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
}

#[derive(Copy, Clone, PartialEq)]
pub enum Langue {
    FR = 0,
    GB = 1,
    ES = 2,
}

impl From<i16> for Langue {
    fn from(value: i16) -> Self {
        match value {
            0 => Langue::FR,
            1 => Langue::ES,
            _ => Langue::GB,
        }
    }
}

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
pub fn effective_lang(mode: LangMode) -> Langue {
    match mode {
        LangMode::Fr => Langue::FR,
        LangMode::En => Langue::GB,
        LangMode::Es => Langue::ES,
        LangMode::Auto => system_lang(),
    }
}

fn lang_path() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let base = if cfg!(target_os = "macos") {
        std::path::Path::new(&home).join("Library/Application Support/Sapiens")
    } else {
        std::path::Path::new(&home).join(".config/sapiens")
    };
    base.join("lang")
}

pub fn load_lang_mode() -> LangMode {
    match std::fs::read_to_string(lang_path()) {
        Ok(s) => match s.trim() {
            "fr" => LangMode::Fr,
            "en" => LangMode::En,
            "es" => LangMode::Es,
            _ => LangMode::Auto,
        },
        Err(_) => LangMode::Auto,
    }
}

pub fn save_lang_mode(mode: LangMode) {
    let p = lang_path();
    let s = match mode {
        LangMode::Auto => "auto",
        LangMode::Fr => "fr",
        LangMode::En => "en",
        LangMode::Es => "es",
    };
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(&p, s);
}
