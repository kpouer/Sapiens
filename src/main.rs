//! Lanceur du portage : ouvre la fenêtre egui qui affiche le port de dessin
//! de la fenêtre Sapiens, démarre le code C (`sapiens_main`) dans un thread
//! et un thread qui égrène `sapiensTimer` toutes les 11 ms (comme le timer
//! du Mac d'origine).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) mod acam;
mod audio;
pub(crate) mod cbridge;
mod dialogs;
mod exchange;
pub(crate) mod langue;
pub(crate) mod port;
pub(crate) mod sapiens;
pub(crate) mod shared;
pub(crate) mod stubs;
pub(crate) mod text;

use crate::acam::Random;
use crate::dialogs::PreferencesState;
use crate::exchange::{
    AlertRequest, CursorShape, INPUT, UiRequest, UiResponse, WindowFrame, cursor_hidden,
    cursor_shape, set_menu_command, take_title, take_ui_request, take_window_frame,
};
use crate::langue::{
    LangMode, Langue, effective_lang, langue, load_lang_mode, save_lang_mode, set_game_language,
};
use crate::sapiens::sap3d::Sap3d;
use crate::sapiens::sapcomb::Sapcomb;
use crate::sapiens::sapenreg::Sapenreg;
use crate::sapiens::sapgest::Sapgest;
use crate::sapiens::sapglob::Sapglob;
use crate::sapiens::saplist::Saplist;
use crate::sapiens::saprel::Saprel;
use crate::sapiens::saptxt::{SapLang, Saptxt};
use crate::sapiens::sapw3d::Sapw3d;
use crate::sapiens::sapwarb::Sapwarb;
use crate::sapiens::sapwbrt::Sapwbrt;
use crate::sapiens::sapwcar::Sapwcar;
use crate::sapiens::sapwclav::{Sapwclav, Souris};
use crate::sapiens::sapwcvt::Sapwcvt;
use crate::sapiens::sapwind::SapWind;
use crate::sapiens::sapwmess::Sapwmess;
use crate::sapiens::sapwmus::Sapwmus;
use eframe::egui;
use egui::CursorIcon;
use shared::lut_rgb;
use std::sync::mpsc::Sender;

fn main() {
    // Le code C ouvre les fichiers relatifs (data/mobs/*.BRT) dans le répertoire
    // courant ; on se place donc sous data/mobs.
    if let Ok(md) = std::env::var("CARGO_MANIFEST_DIR") {
        let data = std::path::Path::new(&md).join("data").join("mobs");
        if std::env::set_current_dir(&data).is_err() {
            eprintln!(
                "AVERTISSEMENT: impossible de changer de répertoire vers {:?}",
                data
            );
        }
    } else {
        eprintln!("AVERTISSEMENT: CARGO_MANIFEST_DIR absent, chemins des données non résolus");
    }

    // Langue : initialisée par `--lang` (sinon fichier persisté), puis imposée
    // au C avant que le thread jeu ne démarre.
    let lang_mode = initial_lang_mode();
    set_game_language(effective_lang(lang_mode));

    audio::init();

    // Compteur 11 ms pour sapiensTimer (SAPWCLAV.C).
    let sapiens_timer = Arc::new(AtomicU64::new(0));
    std::thread::spawn({
        let t = sapiens_timer.clone();
        move || timer_loop(t)
    });

    // Le thread C est lancé depuis `SapiensApp::update`, une fois la taille du
    // moniteur connue (choix du zoom initial).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 422.0])
            .with_title("Sapiens"),
        ..Default::default()
    };
    let result = eframe::run_native(
        "Sapiens",
        native_options,
        Box::new(move |_cc| Ok(Box::new(SapiensApp::new(lang_mode, sapiens_timer)))),
    );
    if let Err(e) = result {
        eprintln!("eframe: {e}");
    }
    // La fenêtre est fermée : on quitte (le thread C est une boucle infinie).
    std::process::exit(0);
}

/// Mode de langue initial, déterminé par `--lang <auto|fr|en|es>` (ou
/// `--lang=<...>`). Sans argument, on relit le fichier persisté, sinon `Auto`.
/// En cas de valeur invalide, on signale l'usage et on retombe sur le fichier.
fn initial_lang_mode() -> LangMode {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = if arg == "--lang" {
            args.next()
        } else {
            arg.strip_prefix("--lang=").map(str::to_string)
        };
        if let Some(v) = value {
            match LangMode::from_arg(&v) {
                Some(mode) => return mode,
                None => eprintln!("--lang attendu : auto|fr|en|es (reçu « {v} »)"),
            }
        }
    }
    load_lang_mode()
}

fn timer_loop(t: Arc<AtomicU64>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(11));
        t.fetch_add(1, Ordering::Relaxed);
    }
}

/// Marge de sécurité (en points egui) pour que la fenêtre tienne à l'écran,
/// titre, barre de menus du système et dock compris.
const STARTUP_MARGIN: f32 = 48.0;
/// Hauteur approximative de la barre de menus interne avant la première frame.
const MENU_H_ESTIMATE: f32 = 24.0;

/// Zoom initial : 200 % si le moniteur peut contenir la fenêtre (1280×800 plus
/// la barre de menus), sinon 100 %. Une préférence sauvegardée (`Sapiens.cfg`)
/// reste prioritaire : `ChargePref` appelle `set_zoom` après coup.
fn startup_zoom(monitor: egui::Vec2) -> f32 {
    let base_w = 320.0 * crate::sapiens::ECHELLE as f32;
    let base_h = 200.0 * crate::sapiens::ECHELLE as f32;
    let need_w = base_w * 2.0 + STARTUP_MARGIN;
    let need_h = base_h * 2.0 + MENU_H_ESTIMATE + STARTUP_MARGIN;
    if monitor.x >= need_w && monitor.y >= need_h {
        2.0
    } else {
        1.0
    }
}

/// Lance le code C (boucle infinie du jeu original) avec le zoom initial choisi.
fn spawn_game_thread(sapiens_timer: Arc<AtomicU64>, initial_zoom: f32) {
    std::thread::Builder::new()
        .name("sapiens_c".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || unsafe {
            let mut sapwind = SapWind::default();
            sapwind.set_zoom(initial_zoom);
            // Pointeurs auto-référents de SAPGLOB.C (debban, peuple, soma...).
            let mut sapiens = Sapiens {
                sapwclav: Sapwclav::with_sapiens_timer(sapiens_timer),
                sapwind,
                ..Default::default()
            };
            sapiens.init_glob();
            sapiens.sapiens_main();
        })
        .expect("échec du thread C");
}

/// Sous-menu « Fichier → Langue » : Auto / Français / Anglais / Espagnol.
fn render_language_menu(ui: &mut egui::Ui, mode: &mut LangMode) {
    let lang = langue();
    let title = match lang {
        Langue::FR => "Langue",
        Langue::ES => "Idioma",
        _ => "Language",
    };
    egui::menu::menu_button(ui, title, |ui| {
        for opt in [LangMode::Auto, LangMode::Fr, LangMode::En, LangMode::Es] {
            if ui.selectable_label(*mode == opt, opt.label(lang)).clicked() {
                *mode = opt;
                set_game_language(effective_lang(opt));
                save_lang_mode(opt);
                ui.close_menu();
            }
        }
    });
}

#[derive(Default)]
struct Sapiens {
    souris: Souris,
    sapwclav: Sapwclav,
    sapwind: SapWind,
    sapcomb: Sapcomb,
    sapglob: Sapglob,
    sap3d: Sap3d,
    sapw3d: Sapw3d,
    sapwbrt: Sapwbrt,
    random: Random,
    saplist: Saplist,
    sapwmus: Sapwmus,
    sapwarb: Sapwarb,
    sapwcar: Sapwcar,
    saprel: Saprel,
    sapwmess: Sapwmess,
    sapgest: Sapgest,
    sapenreg: Sapenreg,
    sapwcvt: Sapwcvt,
    saptxt: Saptxt,
    sap_lang: SapLang,
}

struct SapiensApp {
    tex: Option<egui::TextureHandle>,
    dump: Option<PortDumper>,
    lang_mode: LangMode,
    last_save_dir: Option<std::path::PathBuf>,
    active_dialog: Option<ActiveDialog>,
    sapiens_timer: Arc<AtomicU64>,
    game_started: bool,
    monitor_wait: u32,
    last_port_size: Option<(usize, usize)>,
    /// Dernière image du port fenêtre publiée par le thread C.
    frame: Option<WindowFrame>,
}

enum ActiveDialog {
    Alert {
        request: AlertRequest,
        reply: Option<Sender<UiResponse>>,
    },
    Preferences {
        state: PreferencesState,
        reply: Option<Sender<UiResponse>>,
    },
}

impl SapiensApp {
    fn new(lang_mode: LangMode, sapiens_timer: Arc<AtomicU64>) -> Self {
        Self {
            tex: None,
            dump: PortDumper::from_env(),
            lang_mode,
            last_save_dir: None,
            active_dialog: None,
            sapiens_timer,
            game_started: false,
            monitor_wait: 0,
            last_port_size: None,
            frame: None,
        }
    }

    /// Traite une requête déposée par le thread C. Les boîtes de fichiers
    /// (`rfd`) sont ouvertes tout de suite ; alertes et préférences passent par
    /// une fenêtre egui modale rendue en fin de frame.
    fn dispatch_ui_request(&mut self) {
        if self.active_dialog.is_some() {
            return;
        }
        let Some((request, reply)) = take_ui_request() else {
            return;
        };
        let lang = langue();
        match request {
            UiRequest::OpenFile(request) => {
                let path = dialogs::pick_open_file(&mut self.last_save_dir, &request, lang);
                let _ = reply.send(UiResponse::File(path));
            }
            UiRequest::SaveFile(request) => {
                let path = dialogs::pick_save_file(&mut self.last_save_dir, &request, lang);
                let _ = reply.send(UiResponse::File(path));
            }
            UiRequest::Alert(request) => {
                self.active_dialog = Some(ActiveDialog::Alert {
                    request,
                    reply: Some(reply),
                });
            }
            UiRequest::Preferences(request) => {
                self.active_dialog = Some(ActiveDialog::Preferences {
                    state: PreferencesState::from_request(request, self.lang_mode),
                    reply: Some(reply),
                });
            }
        }
    }

    fn render_dialogs(&mut self, ctx: &egui::Context) {
        let lang = langue();
        let mut response: Option<UiResponse> = None;

        if let Some(dialog) = self.active_dialog.as_mut() {
            match dialog {
                ActiveDialog::Alert { request, .. } => {
                    let spec = dialogs::alert_spec(request, lang);
                    let modal = egui::Modal::new(egui::Id::new("sapiens_alert")).show(ctx, |ui| {
                        ui.set_min_width(360.0);
                        dialogs::render_alert(ui, &spec)
                    });
                    if let Some(item) = modal
                        .inner
                        .or_else(|| modal.should_close().then_some(spec.dismiss))
                    {
                        response = Some(UiResponse::Alert(item));
                    }
                }
                ActiveDialog::Preferences { state, .. } => {
                    let modal =
                        egui::Modal::new(egui::Id::new("sapiens_preferences")).show(ctx, |ui| {
                            ui.set_min_width(420.0);
                            dialogs::render_preferences(ui, state, lang)
                        });
                    let should_close = modal.should_close();
                    if let Some(outcome) = modal
                        .inner
                        .or_else(|| should_close.then_some(dialogs::PreferencesOutcome::Cancel))
                    {
                        match outcome {
                            dialogs::PreferencesOutcome::Ok(result) => {
                                if result.lang_mode != self.lang_mode {
                                    self.lang_mode = result.lang_mode;
                                    set_game_language(effective_lang(result.lang_mode));
                                    save_lang_mode(result.lang_mode);
                                }
                                response = Some(UiResponse::Preferences(Some(result)));
                            }
                            dialogs::PreferencesOutcome::Cancel => {
                                response = Some(UiResponse::Preferences(None));
                            }
                        }
                    }
                }
            }
        }

        if let Some(response) = response
            && let Some(mut dialog) = self.active_dialog.take()
        {
            let reply = match &mut dialog {
                ActiveDialog::Alert { reply, .. } => reply,
                ActiveDialog::Preferences { reply, .. } => reply,
            };
            if let Some(reply) = reply.take() {
                let _ = reply.send(response);
            }
        }
    }
}

impl From<CursorShape> for CursorIcon {
    fn from(value: CursorShape) -> Self {
        match value {
            CursorShape::Arrow => CursorIcon::Default,
            CursorShape::Watch => CursorIcon::Progress,
        }
    }
}

impl eframe::App for SapiensApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Le thread C a besoin de la taille du moniteur pour choisir le zoom
        // initial ; on le lance dès que l'information est disponible (repli sur
        // 100 % si la plateforme ne la fournit pas).
        if !self.game_started {
            if let Some(monitor) = ctx.input(|i| i.viewport().monitor_size) {
                spawn_game_thread(self.sapiens_timer.clone(), startup_zoom(monitor));
                self.game_started = true;
            } else {
                self.monitor_wait += 1;
                if self.monitor_wait >= 50 {
                    spawn_game_thread(self.sapiens_timer.clone(), 1.0);
                    self.game_started = true;
                }
            }
        }

        // Curseur posé par le thread C (forme + visibilité, cf. exchange.rs) :
        // la visibilité prime, comme sur le Mac (`HideCursor` masque quoi qu'il
        // en soit de la forme).
        ctx.set_cursor_icon(if cursor_hidden() {
            CursorIcon::None
        } else {
            cursor_shape().into()
        });
        // Titre posé par `SetWTitle` (thread C), appliqué une seule fois.
        if let Some(title) = take_title() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));
        }
        // Requête bloquante du thread C (alerte, fichier, préférences).
        self.dispatch_ui_request();
        // Le Rust est autoritaire sur la langue : on réimpose la sélection
        // (un `Sapiens.cfg` chargé par ChargePref pourrait la remettre en
        // cause au démarrage).
        let effective = effective_lang(self.lang_mode);
        if langue() != effective {
            set_game_language(effective);
        }

        // Raccourcis clavier (équivalents MenuKey) avant la barre de menus.
        if self.active_dialog.is_none() {
            handle_shortcuts(ctx);
        }

        // Barre de menus Mac reconstituée (modèle → src/acam.rs).
        let menus = acam::menus_snapshot();
        let mut command: Option<i32> = None;
        egui::TopBottomPanel::top("menus").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                for menu in &menus {
                    if menu.enabled {
                        egui::menu::menu_button(ui, &menu.title, |ui| {
                            for item in &menu.items {
                                if item.sep {
                                    // Dans le menu Fichier, insère le choix de
                                    // langue entre « Préférences… » et « Quitter ».
                                    if menu.base_id == 129 && item.index == 6 {
                                        ui.separator();
                                        render_language_menu(ui, &mut self.lang_mode);
                                        ui.separator();
                                    } else {
                                        ui.separator();
                                    }
                                } else {
                                    let label = if item.checked {
                                        format!("✓ {}", item.text)
                                    } else {
                                        item.text.clone()
                                    };
                                    if ui
                                        .add_enabled(item.enabled, egui::Button::new(label))
                                        .clicked()
                                    {
                                        command = Some(menu_result(menu.id, item.index));
                                        ui.close_menu();
                                    }
                                }
                            }
                        });
                    } else {
                        ui.add_enabled(false, egui::Button::new(&menu.title));
                    }
                }
            });
        });
        if self.active_dialog.is_none()
            && let Some(r) = command
        {
            set_menu_command(r);
        }

        record_input(ctx);

        let new_frame = take_window_frame();
        if let Some(f) = new_frame {
            self.tex =
                Some(ctx.load_texture("sapiens", frame_image(&f), egui::TextureOptions::NEAREST));
            self.frame = Some(f);
        }
        let window = self.frame.as_ref();
        if let Some(f) = window {
            // La fenêtre egui suit la taille réelle du port (menu « Échelle »).
            let (w, h) = (f.w, f.h);
            if w > 0 && h > 0 && self.last_port_size != Some((w, h)) {
                self.last_port_size = Some((w, h));
                let menu_h = (ctx.available_rect().top() - ctx.screen_rect().top()).max(0.0);
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    w as f32,
                    h as f32 + menu_h,
                )));
            }
        }
        if window.is_none() {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
                .show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            egui::RichText::new("Initialisation de Sapiens…")
                                .color(egui::Color32::WHITE),
                        );
                    });
                });
            ctx.request_repaint_after(std::time::Duration::from_millis(10));
        } else {
            if let Some(d) = &mut self.dump {
                d.tick(window.unwrap());
            }

            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
                .show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.add(egui::Image::new(self.tex.as_ref().unwrap()));
                    });
                });
        }

        self.render_dialogs(ctx);
        // Rafraîchissement permanent (boucle de jeu C).
        ctx.request_repaint();
    }
}

/// Convertit l'image d'indices du port fenêtre en image egui (via la LUT), à
/// sa taille réelle : le zoom est ainsi réellement visible.
fn frame_image(f: &WindowFrame) -> egui::ColorImage {
    let pixels = f
        .pixels
        .iter()
        .map(|&idx| {
            let (r, g, b) = lut_rgb(idx);
            egui::Color32::from_rgb(r, g, b)
        })
        .collect();
    egui::ColorImage {
        size: [f.w, f.h],
        pixels,
    }
}

/// Alimente l'état d'entrée partagé (utilisé par les stubs GetMouse/Button au
/// moment voulu). Les coordonnées sont normalisées dans la zone de jeu (sous la
/// barre de menus), pas dans toute la fenêtre.
fn record_input(ctx: &egui::Context) {
    let area = ctx.available_rect();
    let (mh, mv) = ctx
        .pointer_interact_pos()
        .map(|pos| {
            (
                ((pos.x - area.left()) / area.width().max(1.0)).clamp(0.0, 1.0),
                ((pos.y - area.top()) / area.height().max(1.0)).clamp(0.0, 1.0),
            )
        })
        .unwrap_or((0.5, 0.5));
    let (button, keymap) = ctx.input(|i| {
        let mut km = [0u64; 4];
        for &k in &i.keys_down {
            if let Some(code) = mac_keycode(k) {
                km_set_key(&mut km, code);
            }
        }
        if i.modifiers.shift {
            km_set_key(&mut km, 0x38);
        }
        if i.modifiers.ctrl {
            km_set_key(&mut km, 0x3B);
        }
        if i.modifiers.alt {
            km_set_key(&mut km, 0x3A);
        }
        if i.modifiers.command {
            km_set_key(&mut km, 0x37);
        }
        (i.pointer.primary_down(), km)
    });
    {
        let mut st = INPUT.lock().unwrap();
        st.mouse_h = mh;
        st.mouse_v = mv;
        st.button = button;
        st.keymap = keymap;
    }
}

/// Résultat de menu attendu par `DoMenuCommand` : `menuID<<16|item`,
/// menuID étant l'ID runtime (avec l'offset langue, cf. `CvtDefaultLangRess`).
fn menu_result(menu_id: i16, item: i16) -> i32 {
    ((menu_id as i32) << 16) | (item as i32 & 0xFFFF)
}

/// Raccourcis clavier Cmd du Mac (équivalents `MenuKey`) : Cmd+N (Nouveau),
/// Cmd+O (Charger), Cmd+S (Sauver), Cmd+Q (Quitter), Cmd+/ (Aide).
fn handle_shortcuts(ctx: &egui::Context) {
    let (command, keys) = ctx.input(|i| {
        (
            i.modifiers.command,
            i.keys_down.iter().copied().collect::<Vec<_>>(),
        )
    });
    if !command || keys.is_empty() {
        return;
    }
    let menus = acam::menus_snapshot();
    // N° d'item par menu (base) : identiques à SAPWIND.C (`enum` + séparateurs).
    let mut trigger: Option<i32> = None;
    'keys: for k in keys {
        let (base, item) = match k {
            egui::Key::N => (129, 1),     // Nouveau
            egui::Key::O => (129, 2),     // Charger…
            egui::Key::S => (129, 3),     // Sauver…
            egui::Key::Q => (129, 7),     // Quitter
            egui::Key::Slash => (128, 2), // Aide
            _ => continue,
        };
        for menu in &menus {
            if menu.base_id == base {
                trigger = Some(menu_result(menu.id, item));
                break 'keys;
            }
        }
    }
    if let Some(r) = trigger {
        set_menu_command(r);
    }
}

/// Pose le bit du keycode Mac `code` dans les 4 mots de 64 bits alignés sur
/// `KeyMap` du C (bit `((code^24)&31)` du mot `code>>5`, cf. `KeyPressed`).
fn km_set_key(keymap: &mut [u64; 4], code: u8) {
    if code >= 128 {
        return;
    }
    let bit = (code ^ 24) & 31;
    keymap[(code >> 5) as usize] |= 1u64 << bit;
}

/// Traduit une touche logique egui vers son keycode virtuel Mac.
fn mac_keycode(k: egui::Key) -> Option<u8> {
    use egui::Key::*;
    Some(match k {
        // Commandes de jeu (SAPWCLAV.C).
        ArrowUp => 0x7E,
        ArrowDown => 0x7D,
        ArrowLeft => 0x7B,
        ArrowRight => 0x7C,
        Space => 0x31,
        Enter => 0x24,
        Escape => 0x35,
        C => 0x08,
        M => 0x2E,
        // Lettres.
        A => 0x00,
        B => 0x0B,
        D => 0x02,
        E => 0x0E,
        F => 0x03,
        G => 0x05,
        H => 0x04,
        I => 0x22,
        J => 0x26,
        K => 0x28,
        L => 0x25,
        N => 0x2D,
        O => 0x1F,
        P => 0x23,
        Q => 0x0C,
        R => 0x0F,
        S => 0x01,
        T => 0x11,
        U => 0x20,
        V => 0x09,
        W => 0x0D,
        X => 0x07,
        Y => 0x10,
        Z => 0x06,
        // Chiffres (rangée alphabétique).
        Num0 => 0x1D,
        Num1 => 0x12,
        Num2 => 0x13,
        Num3 => 0x14,
        Num4 => 0x15,
        Num5 => 0x17,
        Num6 => 0x16,
        Num7 => 0x1A,
        Num8 => 0x1C,
        Num9 => 0x19,
        // Ponctuation / divers.
        Period => 0x2F,
        Comma => 0x2B,
        Slash => 0x2C,
        Semicolon => 0x29,
        Minus => 0x1B,
        Equals => 0x18,
        Backtick => 0x32,
        OpenBracket => 0x21,
        CloseBracket => 0x1E,
        Backslash => 0x2A,
        Tab => 0x30,
        Backspace => 0x33,
        Delete => 0x75,
        Home => 0x73,
        End => 0x77,
        PageUp => 0x74,
        PageDown => 0x79,
        _ => return None,
    })
}

/// Débogage : si `SAPIENS_DUMP` est défini, écrit régulièrement le tampon de la
/// fenêtre sous forme d'image PPM (lisible ensuite avec `sips`/`ffmpeg`).
struct PortDumper {
    out: String,
    then: std::time::Instant,
}

impl PortDumper {
    fn from_env() -> Option<Self> {
        std::env::var("SAPIENS_DUMP").ok().map(|out| PortDumper {
            out,
            then: std::time::Instant::now(),
        })
    }

    fn tick(&mut self, f: &WindowFrame) {
        const INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);
        if self.then.elapsed() < INTERVAL {
            return;
        }
        self.then = std::time::Instant::now();
        let (w, h) = (f.w, f.h);
        let mut ppm = Vec::with_capacity(12 + w * h * 3);
        ppm.extend_from_slice(format!("P6\n{w} {h}\n255\n").as_bytes());
        for &idx in &f.pixels {
            let (r, g, b) = lut_rgb(idx);
            ppm.extend_from_slice(&[r, g, b]);
        }
        let path = self.out.clone();
        match std::fs::write(&path, &ppm) {
            Ok(()) => eprintln!("dump -> {path} ({w}x{h})"),
            Err(e) => eprintln!("dump échec {path}: {e}"),
        }
    }
}
