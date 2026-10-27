//! Dialogues natifs côté egui : sélecteurs de fichiers `rfd`, alertes et
//! préférences. Le thread C ne manipule jamais egui : il dépose une requête
//! (`exchange`) et attend la réponse.

use crate::exchange::{AlertRequest, FileRequest, PreferencesResult};
use crate::langue::{LangMode, Langue};

pub(crate) fn documents_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::Path::new(&home)
        .join("Documents")
        .join("sapiens")
}

fn absolute(path: std::path::PathBuf) -> std::path::PathBuf {
    if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(path)
    }
}

fn start_dir(last_dir: &Option<std::path::PathBuf>) -> std::path::PathBuf {
    if let Some(dir) = last_dir
        && dir.is_dir()
    {
        return dir.clone();
    }
    let dir = documents_dir();
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn with_extension(mut path: std::path::PathBuf, extension: &str) -> std::path::PathBuf {
    if !extension.is_empty() {
        path.set_extension(extension);
    }
    path
}

fn file_title(lang: Langue, save: bool) -> &'static str {
    match (lang, save) {
        (Langue::FR, false) => "Charger une aventure",
        (Langue::FR, true) => "Sauver l'aventure",
        (Langue::ES, false) => "Cargar una aventura",
        (Langue::ES, true) => "Guardar la aventura",
        (_, false) => "Load an adventure",
        (_, true) => "Save the adventure",
    }
}

/// Sélecteur de fichier pour `Charger…`. Le chemin retourné est absolu.
pub(crate) fn pick_open_file(
    last_dir: &mut Option<std::path::PathBuf>,
    request: &FileRequest,
    lang: Langue,
) -> Option<std::path::PathBuf> {
    let dir = start_dir(last_dir);
    let mut dialog = rfd::FileDialog::new()
        .set_title(file_title(lang, false))
        .set_directory(&dir);
    if !request.extension.is_empty() {
        dialog = dialog.add_filter("Sapiens", &[request.extension.as_str()]);
    }
    let path = dialog.pick_file().map(absolute)?;
    *last_dir = path.parent().map(std::path::Path::to_path_buf);
    Some(path)
}

/// Sélecteur de fichier pour `Sauver…`. Force l'extension `.sap`.
pub(crate) fn pick_save_file(
    last_dir: &mut Option<std::path::PathBuf>,
    request: &FileRequest,
    lang: Langue,
) -> Option<std::path::PathBuf> {
    let dir = start_dir(last_dir);
    let mut name = request.default_name.clone();
    if name.is_empty() {
        name = "Torgan".to_string();
    }
    if !request.extension.is_empty() && !name.ends_with(&format!(".{}", request.extension)) {
        name.push('.');
        name.push_str(&request.extension);
    }
    let mut dialog = rfd::FileDialog::new()
        .set_title(file_title(lang, true))
        .set_directory(&dir)
        .set_file_name(&name);
    if !request.extension.is_empty() {
        dialog = dialog.add_filter("Sapiens", &[request.extension.as_str()]);
    }
    let path = with_extension(absolute(dialog.save_file()?), &request.extension);
    *last_dir = path.parent().map(std::path::Path::to_path_buf);
    Some(path)
}

// ---------------------------------------------------------------------------
// Documentation
// ---------------------------------------------------------------------------

fn open_with_system(path: &std::path::Path) -> bool {
    let mut command = if cfg!(target_os = "macos") {
        let mut c = std::process::Command::new("open");
        c.arg(path);
        c
    } else if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", "start", ""]).arg(path);
        c
    } else {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(path);
        c
    };
    command
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Ouvre le manuel HTML dans le navigateur par défaut.
pub(crate) fn open_manual(lang: Langue) -> bool {
    let name = match lang {
        Langue::FR => "sapiens_fr.html",
        Langue::ES => "sapiens_es.html",
        _ => "sapiens_gb.html",
    };
    let mut candidates = Vec::new();
    if let Ok(root) = std::env::var("CARGO_MANIFEST_DIR") {
        candidates.push(std::path::Path::new(&root).join("data/docs").join(name));
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    candidates.push(cwd.join("data/docs").join(name));
    candidates.push(cwd.join("../docs").join(name));
    candidates.push(cwd.join("../../data/docs").join(name));
    candidates
        .iter()
        .find(|path| path.is_file())
        .is_some_and(|path| open_with_system(path))
}

// ---------------------------------------------------------------------------
// Alertes
// ---------------------------------------------------------------------------

pub(crate) struct AlertSpec {
    pub(crate) text: String,
    pub(crate) buttons: Vec<(String, i16)>,
    pub(crate) dismiss: i16,
}

fn tr(lang: Langue, fr: &'static str, en: &'static str, es: &'static str) -> &'static str {
    match lang {
        Langue::FR => fr,
        Langue::ES => es,
        _ => en,
    }
}

/// Contenu localisé d'une alerte. Les IDs sont ceux des ressources Mac
/// (`CvtDefaultLangRess` ajoute 1000 pour les langues non françaises).
pub(crate) fn alert_spec(request: &AlertRequest, lang: Langue) -> AlertSpec {
    let id = request.alert_id.rem_euclid(1000);
    match id {
        1 => AlertSpec {
            text: tr(
                lang,
                "Voulez-vous vraiment abandonner l'aventure en cours ?",
                "Do you really want to abandon the current adventure?",
                "¿Quiere realmente abandonar la aventura actual?",
            )
            .to_string(),
            buttons: vec![
                (tr(lang, "Oui", "Yes", "Sí").to_string(), 1),
                (tr(lang, "Non", "No", "No").to_string(), 2),
            ],
            dismiss: 2,
        },
        2 => {
            let version = request.params[0].trim();
            let text = if version.is_empty() {
                tr(lang, "Sapiens", "Sapiens", "Sapiens").to_string()
            } else {
                format!("Sapiens {version}")
            };
            AlertSpec {
                text: format!(
                    "{text}\n\n{}",
                    tr(
                        lang,
                        "Copyright Didier & Olivier Guillion, 1986,1996,2005,2017",
                        "Copyright Didier & Olivier Guillion, 1986,1996,2005,2017",
                        "Copyright Didier & Olivier Guillion, 1986,1996,2005,2017",
                    )
                ),
                buttons: vec![("OK".to_string(), 1)],
                dismiss: 1,
            }
        }
        133 => AlertSpec {
            text: tr(
                lang,
                "Choisissez la langue de Sapiens.",
                "Choose the language for Sapiens.",
                "Elija el idioma de Sapiens.",
            )
            .to_string(),
            buttons: vec![
                ("Français".to_string(), 1),
                ("English".to_string(), 2),
                ("Español".to_string(), 3),
            ],
            dismiss: 1,
        },
        134 => AlertSpec {
            text: request.params[0].clone(),
            buttons: vec![("OK".to_string(), 1)],
            dismiss: 1,
        },
        135 => AlertSpec {
            text: request.params[0].clone(),
            buttons: vec![
                (tr(lang, "Oui", "Yes", "Sí").to_string(), 1),
                (tr(lang, "Non", "No", "No").to_string(), 2),
            ],
            dismiss: 2,
        },
        136 => AlertSpec {
            text: tr(
                lang,
                "Le joueur 2 utilise le clavier. Les flèches déplacent le personnage et les touches du pavé numérique servent aux actions.",
                "Player 2 uses the keyboard. The arrow keys move the character and the numeric keypad keys perform actions.",
                "El jugador 2 usa el teclado. Las flechas mueven al personaje y las teclas del teclado numérico realizan las acciones.",
            )
            .to_string(),
            buttons: vec![("OK".to_string(), 1)],
            dismiss: 1,
        },
        _ => AlertSpec {
            text: if request.params[0].is_empty() {
                format!("Sapiens ({})", request.alert_id)
            } else {
                request.params[0].clone()
            },
            buttons: vec![("OK".to_string(), 1)],
            dismiss: 1,
        },
    }
}

/// Affiche le corps et les boutons d'une alerte. Retourne l'item choisi.
pub(crate) fn render_alert(ui: &mut egui::Ui, spec: &AlertSpec) -> Option<i16> {
    ui.label(&spec.text);
    ui.add_space(8.0);
    let mut hit = None;
    ui.horizontal(|ui| {
        for (label, item) in &spec.buttons {
            if ui.button(label).clicked() {
                hit = Some(*item);
            }
        }
    });
    hit
}

// ---------------------------------------------------------------------------
// Préférences
// ---------------------------------------------------------------------------

pub(crate) struct PreferencesState {
    pub(crate) is_bruitages: bool,
    pub(crate) music: bool,
    pub(crate) lissage_police: bool,
    pub(crate) mode_joueur2: i16,
    pub(crate) lang_mode: LangMode,
}

impl PreferencesState {
    pub(crate) fn from_request(
        request: crate::exchange::PreferencesRequest,
        lang_mode: LangMode,
    ) -> Self {
        Self {
            is_bruitages: request.is_bruitages,
            music: request.music,
            lissage_police: request.lissage_police,
            mode_joueur2: request.mode_joueur2,
            lang_mode,
        }
    }
}

pub(crate) enum PreferencesOutcome {
    Ok(PreferencesResult),
    Cancel,
}

/// Corps de la boîte de préférences. `None` = rester ouvert.
pub(crate) fn render_preferences(
    ui: &mut egui::Ui,
    state: &mut PreferencesState,
    lang: Langue,
) -> Option<PreferencesOutcome> {
    ui.heading(tr(lang, "Préférences", "Preferences", "Preferencias"));
    ui.add_space(8.0);

    ui.label(tr(lang, "Langue", "Language", "Idioma"));
    ui.horizontal(|ui| {
        for mode in [LangMode::Auto, LangMode::Fr, LangMode::En, LangMode::Es] {
            ui.radio_value(&mut state.lang_mode, mode, mode.label(lang));
        }
    });
    ui.add_space(6.0);
    ui.checkbox(
        &mut state.is_bruitages,
        tr(lang, "Bruitages", "Sound effects", "Efectos sonoros"),
    );
    ui.checkbox(&mut state.music, tr(lang, "Musiques", "Music", "Música"));
    ui.checkbox(
        &mut state.lissage_police,
        tr(
            lang,
            "Polices lissées",
            "Smooth fonts",
            "Fuentes suavizadas",
        ),
    );
    ui.add_space(6.0);
    ui.label(tr(lang, "Joueur 2", "Player 2", "Jugador 2"));
    ui.horizontal(|ui| {
        ui.radio_value(
            &mut state.mode_joueur2,
            0,
            tr(lang, "Autonome", "Automatic", "Autónomo"),
        );
        ui.radio_value(
            &mut state.mode_joueur2,
            1,
            tr(lang, "Clavier", "Keyboard", "Teclado"),
        );
    });
    ui.add_space(12.0);

    let mut outcome = None;
    ui.horizontal(|ui| {
        if ui.button("OK").clicked() {
            outcome = Some(PreferencesOutcome::Ok(PreferencesResult {
                is_bruitages: state.is_bruitages,
                music: state.music,
                lissage_police: state.lissage_police,
                mode_joueur2: state.mode_joueur2,
                lang_mode: state.lang_mode,
            }));
        }
        if ui
            .button(tr(lang, "Annuler", "Cancel", "Cancelar"))
            .clicked()
        {
            outcome = Some(PreferencesOutcome::Cancel);
        }
    });
    outcome
}
