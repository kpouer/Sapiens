//! Barre de menus : native sur macOS (`native_menu`), egui ailleurs, et
//! raccourcis clavier associés.

#[cfg(not(target_os = "macos"))]
use crate::acam;
#[cfg(not(target_os = "macos"))]
use crate::exchange::set_menu_command;
use crate::langue::{LangMode, effective_lang, save_lang_mode, set_game_language};
#[cfg(not(target_os = "macos"))]
use crate::langue::{Langue, langue};
#[cfg(target_os = "macos")]
use crate::ui::native_menu;
#[cfg(not(target_os = "macos"))]
use eframe::egui;

/// Résultat de menu attendu par `DoMenuCommand` : `menuID<<16|item`,
/// menuID étant l'ID runtime (avec l'offset langue, cf. `CvtDefaultLangRess`).
pub(super) fn menu_result(menu_id: i16, item: i16) -> i32 {
    ((menu_id as i32) << 16) | (item as i32 & 0xFFFF)
}

/// Applique un nouveau mode de langue : jeu + fichier persisté.
pub(super) fn apply_lang_mode(current: &mut LangMode, mode: LangMode) {
    *current = mode;
    set_game_language(effective_lang(mode));
    save_lang_mode(mode);
}

/// Barre de menus native macOS : les raccourcis Cmd sont des équivalents
/// clavier du `NSMenu`, qui les intercepte avant egui. Renvoie la commande
/// choisie ; un changement de langue n'est appliqué qu'hors dialogue modal.
#[cfg(target_os = "macos")]
pub(super) fn native_menu_bar(
    native: &mut Option<native_menu::NativeMenuBar>,
    lang_mode: &mut LangMode,
    dialog_open: bool,
) -> Option<i32> {
    match native_menu::NativeMenuBar::sync(native, *lang_mode)? {
        native_menu::MenuAction::Command(r) => Some(r),
        native_menu::MenuAction::Lang(opt) => {
            if !dialog_open {
                apply_lang_mode(lang_mode, opt);
            }
            None
        }
    }
}

/// Barre de menus egui (Windows/Linux) et raccourcis clavier associés.
/// Renvoie la commande choisie et la hauteur de la barre.
#[cfg(not(target_os = "macos"))]
pub(super) fn egui_menu_bar(
    ctx: &egui::Context,
    lang_mode: &mut LangMode,
    dialog_open: bool,
) -> (Option<i32>, f32) {
    // Raccourcis clavier (équivalents MenuKey) avant la barre de menus.
    if !dialog_open {
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
                                    render_language_menu(ui, lang_mode);
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
    (command, panel.response.rect.height())
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
                apply_lang_mode(mode, opt);
                ui.close_menu();
            }
        }
    });
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
