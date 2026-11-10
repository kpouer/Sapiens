//! Lanceur du portage : ouvre la fenêtre egui qui affiche le port de dessin
//! de la fenêtre Sapiens, démarre le code C (`sapiens_main`) dans un thread
//! et un thread qui égrène `sapiensTimer` toutes les 11 ms (comme le timer
//! du Mac d'origine).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) mod acam;
mod audio;
pub(crate) mod cbridge;
pub mod data;
mod dialogs;
pub(crate) mod exchange;
pub(crate) mod langue;
#[cfg(target_os = "macos")]
mod native_menu;
pub(crate) mod pict;
pub(crate) mod port;
pub(crate) mod sapiens;
pub(crate) mod shared;
pub(crate) mod stubs;
pub(crate) mod text;

use crate::acam::Random;
use crate::cbridge::{cmdKey, controlKey, keyDown, mouseDown, mouseUp, optionKey, shiftKey};
use crate::dialogs::PreferencesState;
use crate::exchange::{
    AlertRequest, Cursor, INPUT, MOUSE_EVENT_SCALE, UiRequest, UiResponse, WindowFrame,
    clear_events, cursor_hidden, cursor_shape, push_event, set_menu_command, take_title,
    take_ui_request, take_window_frame,
};
#[cfg(not(target_os = "macos"))]
use crate::langue::Langue;
use crate::langue::{
    LangMode, effective_lang, langue, load_lang_mode, save_lang_mode, set_game_language,
};
use crate::pict::Pict;
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
use shared::{EventRecord, Point, lut_rgb};
use std::sync::mpsc::Sender;
use std::time::Instant;

fn main() {
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

/// Période du timer du jeu (`sapiensTimer`, SAPWCLAV.C).
const TIMER_PERIOD: std::time::Duration = std::time::Duration::from_millis(11);

/// Nombre de périodes `TIMER_PERIOD` entières contenues dans `elapsed`.
fn timer_ticks(elapsed: std::time::Duration) -> u64 {
    (elapsed.as_nanos() / TIMER_PERIOD.as_nanos()) as u64
}

/// Égrène `sapiensTimer`. La valeur est déduite du temps réel écoulé depuis le
/// départ, et non du nombre de réveils : un `sleep` dure toujours un peu plus
/// que demandé, et compter les réveils ferait prendre du retard au jeu.
fn timer_loop(t: Arc<AtomicU64>) {
    let start = Instant::now();
    loop {
        let ticks = timer_ticks(start.elapsed());
        t.store(ticks, Ordering::Relaxed);
        // Dort jusqu'à la prochaine échéance (pas une période fixe).
        let next =
            start + std::time::Duration::from_nanos((ticks + 1) * TIMER_PERIOD.as_nanos() as u64);
        std::thread::sleep(next.saturating_duration_since(Instant::now()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn timer_ticks_compte_les_periodes_entieres() {
        assert_eq!(timer_ticks(Duration::ZERO), 0);
        assert_eq!(timer_ticks(Duration::from_millis(10)), 0);
        assert_eq!(timer_ticks(Duration::from_millis(11)), 1);
        assert_eq!(timer_ticks(Duration::from_millis(1100)), 100);
        // Une heure : pas de dérive, 3 600 000 / 11 périodes.
        assert_eq!(timer_ticks(Duration::from_secs(3600)), 327_272);
    }
}

/// Marge de sécurité (en points egui) pour que la fenêtre tienne à l'écran,
/// titre, barre de menus du système et dock compris.
const STARTUP_MARGIN: f32 = 48.0;
/// Hauteur approximative de la barre de menus interne avant la première frame
/// (nulle sur macOS : la barre est celle du système, hors de la fenêtre).
const MENU_H_ESTIMATE: f32 = if cfg!(target_os = "macos") { 0.0 } else { 24.0 };

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
        .spawn(move || {
            let mut sapwind = SapWind::default();
            sapwind.set_zoom(initial_zoom);
            // Pointeurs auto-référents de SAPGLOB.C (debban, peuple...).
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
#[cfg(not(target_os = "macos"))]
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
    pict: Pict,
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
    /// Hauteur de la barre de menus egui (0 avec la barre native macOS).
    menu_h: f32,
    /// Dernière image du port fenêtre publiée par le thread C.
    frame: Option<WindowFrame>,
    /// Barre de menus native, créée à la première frame (`NSApp` prêt).
    #[cfg(target_os = "macos")]
    native_menu: Option<native_menu::NativeMenuBar>,
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
            menu_h: 0.0,
            frame: None,
            #[cfg(target_os = "macos")]
            native_menu: None,
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

    /// Barre de menus native macOS : les raccourcis Cmd sont des équivalents
    /// clavier du `NSMenu`, qui les intercepte avant egui.
    #[cfg(target_os = "macos")]
    fn render_menu_bar(&mut self, _ctx: &egui::Context) -> Option<i32> {
        match native_menu::NativeMenuBar::sync(&mut self.native_menu, self.lang_mode)? {
            native_menu::MenuAction::Command(r) => Some(r),
            native_menu::MenuAction::Lang(opt) => {
                if self.active_dialog.is_none() {
                    self.lang_mode = opt;
                    set_game_language(effective_lang(opt));
                    save_lang_mode(opt);
                }
                None
            }
        }
    }

    /// Barre de menus egui (Windows/Linux) et raccourcis clavier associés.
    #[cfg(not(target_os = "macos"))]
    fn render_menu_bar(&mut self, ctx: &egui::Context) -> Option<i32> {
        // Raccourcis clavier (équivalents MenuKey) avant la barre de menus.
        if self.active_dialog.is_none() {
            handle_shortcuts(ctx);
        }

        // Barre de menus Mac reconstituée (modèle → src/acam.rs).
        let menus = acam::menus_snapshot();
        let mut command: Option<i32> = None;
        let panel = egui::TopBottomPanel::top("menus").show(ctx, |ui| {
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
        self.menu_h = panel.response.rect.height();
        command
    }

    fn render_dialogs(&mut self, ctx: &egui::Context) {
        let lang = langue();
        let mut response: Option<UiResponse> = None;

        if let Some(dialog) = self.active_dialog.as_mut() {
            match dialog {
                ActiveDialog::Alert { request, .. } => {
                    let spec = dialogs::alert_spec(request, lang);
                    let modal =
                        egui::Modal::new(egui::Id::unique("sapiens_alert")).show(ctx, |ui| {
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
                        egui::Modal::new(egui::Id::unique("sapiens_preferences")).show(ctx, |ui| {
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

impl From<Cursor> for CursorIcon {
    fn from(value: Cursor) -> Self {
        match value {
            Cursor::Arrow => CursorIcon::Default,
            Cursor::Watch => CursorIcon::Progress,
        }
    }
}

impl eframe::App for SapiensApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Le thread C a besoin de la taille du moniteur pour choisir le zoom
        // initial ; on le lance dès que l'information est disponible (repli sur
        // 100 % si la plateforme ne la fournit pas).
        let ctx = ui.ctx();
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

        let command = self.render_menu_bar(ctx);
        if self.active_dialog.is_none()
            && let Some(r) = command
        {
            set_menu_command(r);
        }

        // Le thread du jeu est bloqué quand un modal egui est ouvert :
        // `record_input` vide alors la file pour ne pas rejouer les frappes
        // accumulées à la fermeture.
        // Zone de jeu : toute la zone de contenu, sous la barre de menus.
        let mut game_area = ctx.content_rect();
        game_area.min.y += self.menu_h;
        record_input(ctx, game_area, self.active_dialog.is_none());
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
                let menu_h = self.menu_h;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    w as f32,
                    h as f32 + menu_h,
                )));
            }
        }

        match window {
            None => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
                    .show(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(
                                egui::RichText::new("Initialisation de Sapiens…")
                                    .color(egui::Color32::WHITE),
                            );
                        });
                    });
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(10));
            }
            Some(window) => {
                if let Some(d) = &mut self.dump {
                    d.tick(window);
                }

                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
                    .show(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.add(egui::Image::new(self.tex.as_ref().unwrap()));
                        });
                    });
            }
        }

        let ctx = ui.ctx();
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
    egui::ColorImage::new([f.w, f.h], pixels)
}

/// Alimente l'état d'entrée partagé (utilisé par les stubs GetMouse/Button au
/// moment voulu). Les coordonnées sont normalisées dans la zone de jeu (sous la
/// barre de menus), pas dans toute la fenêtre.
///
/// Traduit aussi les événements clavier egui en `keyDown` Mac pour la file
/// consommée par `GetNextEvent`. `accept_events` est faux quand un dialogue
/// modal est ouvert : le thread du jeu est alors bloqué et ne doit pas recevoir
/// les frappes destinées à la boîte de dialogue.
fn record_input(ctx: &egui::Context, area: egui::Rect, accept_events: bool) {
    let (mh, mv) = ctx
        .pointer_interact_pos()
        .map(|pos| {
            (
                ((pos.x - area.left()) / area.width().max(1.0)).clamp(0.0, 1.0),
                ((pos.y - area.top()) / area.height().max(1.0)).clamp(0.0, 1.0),
            )
        })
        .unwrap_or((0.5, 0.5));
    let (button, keymap, events) = ctx.input(|i| {
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
        (i.pointer.primary_down(), km, i.events.clone())
    });
    {
        let mut st = INPUT.lock().unwrap();
        st.mouse_h = mh;
        st.mouse_v = mv;
        st.button = button;
        st.keymap = keymap;
    }

    if !accept_events {
        clear_events();
        return;
    }

    let where_ = Point {
        h: (mh * area.width()) as i16,
        v: (mv * area.height()) as i16,
    };
    // Un `keyDown` Mac porte le code touche dans l'octet haut et le caractère
    // dans l'octet bas. egui envoie `Key` puis `Text` : on garde le `keyDown`
    // en attente pour y greffer le caractère réellement saisi (disposition
    // clavier, Maj, AltGr...).
    let mut pending: Option<EventRecord> = None;
    for event in &events {
        match event {
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } => {
                if let Some(ev) = pending.take() {
                    push_event(ev);
                }
                // Les raccourcis `command` sont déjà traités par `handle_shortcuts`
                // (Cmd sur macOS, Ctrl ailleurs) : ne pas les doubler ici.
                if !modifiers.command
                    && let Some(ev) = mac_key_event(*key, modifiers, where_)
                {
                    pending = Some(ev);
                }
            }
            egui::Event::Text(text) => {
                if let Some(mut ev) = pending.take() {
                    if let Some(c) = text.chars().next()
                        && let Some(b) = crate::text::unicode_to_mac_roman(c)
                    {
                        ev.message = (ev.message & !0xFF) | i64::from(b);
                    }
                    push_event(ev);
                }
            }
            egui::Event::PointerButton {
                button: egui::PointerButton::Primary,
                pressed,
                pos,
                modifiers,
            } => {
                if let Some(ev) = pending.take() {
                    push_event(ev);
                }
                // Clics hors de la zone de jeu (barre de menu egui) ignorés.
                if area.contains(*pos) {
                    let scale = f32::from(MOUSE_EVENT_SCALE);
                    push_event(EventRecord {
                        what: if *pressed { mouseDown } else { mouseUp } as i16,
                        message: 0,
                        when: 0,
                        where_: Point {
                            h: (((pos.x - area.left()) / area.width().max(1.0)).clamp(0.0, 1.0)
                                * scale) as i16,
                            v: (((pos.y - area.top()) / area.height().max(1.0)).clamp(0.0, 1.0)
                                * scale) as i16,
                        },
                        modifiers: mac_modifiers(modifiers),
                    });
                }
            }
            egui::Event::WindowFocused(false) => {
                // Les frappes en attente n'ont plus de sens : on perd le focus.
                pending = None;
                clear_events();
            }
            _ => {
                if let Some(ev) = pending.take() {
                    push_event(ev);
                }
            }
        }
    }
    if let Some(ev) = pending.take() {
        push_event(ev);
    }
}

/// Construit un `keyDown` Mac depuis une touche egui. Le caractère est un
/// repli ; `Event::Text` l'écrase quand il est disponible.
fn mac_key_event(
    key: egui::Key,
    modifiers: &egui::Modifiers,
    where_: Point,
) -> Option<EventRecord> {
    let keycode = mac_keycode(key)?;
    Some(EventRecord {
        what: keyDown as i16,
        message: (i64::from(keycode) << 8) | i64::from(fallback_char(key, modifiers.shift)),
        // Aucun consommateur du jeu ne lit `when` : le laisser à zéro évite de
        // dépendre d'une horloge (utile pour wasm plus tard).
        when: 0,
        where_,
        modifiers: mac_modifiers(modifiers),
    })
}

/// Modificateurs Mac, dérivés des modificateurs egui portables :
/// `command` = Cmd sur macOS, Ctrl sur Windows/Linux.
fn mac_modifiers(modifiers: &egui::Modifiers) -> i16 {
    let mut mac = 0i16;
    if modifiers.command {
        mac |= cmdKey as i16;
    }
    if modifiers.shift {
        mac |= shiftKey as i16;
    }
    if modifiers.alt {
        mac |= optionKey as i16;
    }
    if modifiers.ctrl && !modifiers.command {
        mac |= controlKey as i16;
    }
    mac
}

/// Caractère de repli quand egui n'envoie pas d'`Event::Text` (touches de
/// contrôle, fonction, flèches...). Les codes sont ceux du Mac.
fn fallback_char(key: egui::Key, shift: bool) -> i16 {
    use egui::Key::*;
    match key {
        Enter => 13,
        Tab => 9,
        Escape => 27,
        Backspace => 8,
        Space => 32,
        ArrowLeft => 0x1C,
        ArrowRight => 0x1D,
        ArrowUp => 0x1E,
        ArrowDown => 0x1F,
        F1 => 0x10,
        F2 => 0x11,
        F3 => 0x12,
        F4 => 0x13,
        F5 => 0x14,
        F6 => 0x15,
        F7 => 0x16,
        F8 => 0x17,
        F9 => 0x18,
        F10 => 0x19,
        F11 => 0x1A,
        F12 => 0x1B,
        _ => ascii_key(key, shift).map_or(0, i16::from),
    }
}

/// Repli ASCII pour les touches imprimables (le vrai caractère vient de
/// `Event::Text`, qui respecte la disposition clavier).
fn ascii_key(key: egui::Key, shift: bool) -> Option<u8> {
    use egui::Key::*;
    let c = match key {
        A => b'a',
        B => b'b',
        C => b'c',
        D => b'd',
        E => b'e',
        F => b'f',
        G => b'g',
        H => b'h',
        I => b'i',
        J => b'j',
        K => b'k',
        L => b'l',
        M => b'm',
        N => b'n',
        O => b'o',
        P => b'p',
        Q => b'q',
        R => b'r',
        S => b's',
        T => b't',
        U => b'u',
        V => b'v',
        W => b'w',
        X => b'x',
        Y => b'y',
        Z => b'z',
        Num0 => b'0',
        Num1 => b'1',
        Num2 => b'2',
        Num3 => b'3',
        Num4 => b'4',
        Num5 => b'5',
        Num6 => b'6',
        Num7 => b'7',
        Num8 => b'8',
        Num9 => b'9',
        Period => b'.',
        Comma => b',',
        Slash => b'/',
        Semicolon => b';',
        Minus => b'-',
        Equals => b'=',
        Backtick => b'`',
        OpenBracket => b'[',
        CloseBracket => b']',
        Backslash => b'\\',
        _ => return None,
    };
    Some(if shift { c.to_ascii_uppercase() } else { c })
}

/// Résultat de menu attendu par `DoMenuCommand` : `menuID<<16|item`,
/// menuID étant l'ID runtime (avec l'offset langue, cf. `CvtDefaultLangRess`).
fn menu_result(menu_id: i16, item: i16) -> i32 {
    ((menu_id as i32) << 16) | (item as i32 & 0xFFFF)
}

/// Raccourcis clavier Cmd du Mac (équivalents `MenuKey`) : Cmd+N (Nouveau),
/// Cmd+O (Charger), Cmd+S (Sauver), Cmd+Q (Quitter), Cmd+/ (Aide).
#[cfg(not(target_os = "macos"))]
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
        // Touches de fonction.
        F1 => 0x7A,
        F2 => 0x78,
        F3 => 0x63,
        F4 => 0x76,
        F5 => 0x60,
        F6 => 0x61,
        F7 => 0x62,
        F8 => 0x64,
        F9 => 0x65,
        F10 => 0x6D,
        F11 => 0x67,
        F12 => 0x6F,
        _ => return None,
    })
}

/// Débogage : si `SAPIENS_DUMP` est défini, écrit régulièrement le tampon de la
/// fenêtre sous forme d'image PPM (lisible ensuite avec `sips`/`ffmpeg`).
struct PortDumper {
    out: String,
    then: Instant,
}

impl PortDumper {
    fn from_env() -> Option<Self> {
        std::env::var("SAPIENS_DUMP").ok().map(|out| PortDumper {
            out,
            then: Instant::now(),
        })
    }

    fn tick(&mut self, f: &WindowFrame) {
        const INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);
        if self.then.elapsed() < INTERVAL {
            return;
        }
        self.then = Instant::now();
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
