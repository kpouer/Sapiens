//! Interface egui. Ne connaît pas le jeu : elle échange avec lui uniquement
//! via `exchange` (image, entrées, commandes de menu, requêtes bloquantes).

mod app;
mod dialogs;
mod dump;
mod input;
mod menu;
#[cfg(target_os = "macos")]
mod native_menu;

pub(crate) use app::SapiensApp;
