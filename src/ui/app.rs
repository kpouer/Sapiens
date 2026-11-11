//! Application egui : affiche l'image publiée par le thread du jeu, lui
//! transmet les entrées et les commandes de menu, et sert ses requêtes
//! bloquantes (alertes, fichiers, préférences).

use crate::exchange::{
    AlertRequest, Cursor, UiRequest, UiResponse, WindowFrame, cursor_hidden, cursor_shape,
    set_menu_command, take_title, take_ui_request, take_window_frame,
};
use crate::langue::{LangMode, effective_lang, langue, set_game_language};
use crate::shared::lut_rgb;
use crate::ui::dialogs::{self, PreferencesState};
use crate::ui::dump::PortDumper;
use crate::ui::input::record_input;
use crate::ui::menu;
#[cfg(target_os = "macos")]
use crate::ui::native_menu;
use eframe::egui;
use egui::CursorIcon;
use std::sync::mpsc::Sender;

/// Démarrage du jeu, appelé une seule fois avec la taille du moniteur
/// (`None` si la plateforme ne la fournit pas).
pub(crate) type StartGame = Box<dyn FnOnce(Option<egui::Vec2>)>;

pub(crate) struct SapiensApp {
    tex: Option<egui::TextureHandle>,
    dump: Option<PortDumper>,
    lang_mode: LangMode,
    last_save_dir: Option<std::path::PathBuf>,
    active_dialog: Option<ActiveDialog>,
    start_game: Option<StartGame>,
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
    pub(crate) fn new(lang_mode: LangMode, start_game: StartGame) -> Self {
        Self {
            tex: None,
            dump: PortDumper::from_env(),
            lang_mode,
            last_save_dir: None,
            active_dialog: None,
            start_game: Some(start_game),
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

    #[cfg(target_os = "macos")]
    fn render_menu_bar(&mut self, _ctx: &egui::Context) -> Option<i32> {
        let dialog_open = self.active_dialog.is_some();
        menu::native_menu_bar(&mut self.native_menu, &mut self.lang_mode, dialog_open)
    }

    #[cfg(not(target_os = "macos"))]
    fn render_menu_bar(&mut self, ctx: &egui::Context) -> Option<i32> {
        let dialog_open = self.active_dialog.is_some();
        let (command, menu_h) = menu::egui_menu_bar(ctx, &mut self.lang_mode, dialog_open);
        self.menu_h = menu_h;
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
                                    menu::apply_lang_mode(&mut self.lang_mode, result.lang_mode);
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
        if self.start_game.is_some() {
            let monitor = ctx.input(|i| i.viewport().monitor_size);
            self.monitor_wait += 1;
            if (monitor.is_some() || self.monitor_wait >= 50)
                && let Some(start) = self.start_game.take()
            {
                start(monitor);
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
