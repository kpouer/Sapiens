//! Barre de menus native macOS (`NSMenu` via `muda`), alimentée par le même
//! modèle que la barre egui (`acam::menus_snapshot`). Sur les autres
//! plateformes, `main.rs` garde la barre egui.
//!
//! Le menu est reconstruit quand la langue change (libellés et IDs runtime) ;
//! à chaque frame on ne fait que recopier l'état activé/coché posé par le C
//! (`AdjustMenus` → `DisableItem`/`EnableItem`/`CheckItem`).

use std::collections::HashMap;

use muda::accelerator::Accelerator;
use muda::{CheckMenuItem, Menu, MenuEvent, MenuId, PredefinedMenuItem, Submenu};

use crate::acam::{self, MenuSnapshot};
use crate::langue::{LangMode, Langue, langue};

/// Action déclenchée par un item natif.
#[derive(Clone, Copy)]
pub(crate) enum MenuAction {
    /// `menuID<<16|item` à passer à `DoMenuCommand`.
    Command(i32),
    /// Choix « Fichier → Langue ».
    Lang(LangMode),
}

struct NativeMenu {
    id: i16,
    submenu: Submenu,
    /// Un élément par item du modèle (`None` pour les séparateurs).
    items: Vec<Option<CheckMenuItem>>,
}

pub(crate) struct NativeMenuBar {
    /// Garde la barre vivante : `NSApp` ne la retient pas pour nous.
    _menu: Menu,
    lang: Langue,
    menus: Vec<NativeMenu>,
    lang_items: Vec<(LangMode, CheckMenuItem)>,
    actions: HashMap<MenuId, MenuAction>,
}

/// Raccourci Cmd affiché dans le menu (équivalents `MenuKey`, cf.
/// `handle_shortcuts` côté egui).
fn accelerator(base_id: i16, item: i16) -> Option<Accelerator> {
    let key = match (base_id, item) {
        (129, 1) => "N",
        (129, 2) => "O",
        (129, 3) => "S",
        (129, 7) => "Q",
        (128, 2) => "/",
        _ => return None,
    };
    format!("Cmd+{key}").parse().ok()
}

fn lang_title(lang: Langue) -> &'static str {
    match lang {
        Langue::FR => "Langue",
        Langue::ES => "Idioma",
        _ => "Language",
    }
}

impl NativeMenuBar {
    /// Construit la barre et l'installe comme menu principal de l'application.
    /// À appeler depuis le thread principal, une fois `NSApp` créé (premier
    /// `update` d'eframe).
    pub(crate) fn new(snapshot: &[MenuSnapshot], lang_mode: LangMode) -> Self {
        let lang = langue();
        let menu = Menu::new();
        let mut menus = Vec::with_capacity(snapshot.len());
        let mut lang_items = Vec::new();
        let mut actions = HashMap::new();

        for m in snapshot {
            let submenu = Submenu::new(&m.title, m.enabled);
            let mut items = Vec::with_capacity(m.items.len());
            for item in &m.items {
                if item.sep {
                    // Dans le menu Fichier, le choix de langue s'insère entre
                    // « Préférences… » et « Quitter » (comme la barre egui).
                    if m.base_id == 129 && item.index == 6 {
                        let _ = submenu.append(&PredefinedMenuItem::separator());
                        let langs = Submenu::new(lang_title(lang), true);
                        for opt in [LangMode::Auto, LangMode::Fr, LangMode::En, LangMode::Es] {
                            let it =
                                CheckMenuItem::new(opt.label(lang), true, opt == lang_mode, None);
                            actions.insert(it.id().clone(), MenuAction::Lang(opt));
                            let _ = langs.append(&it);
                            lang_items.push((opt, it));
                        }
                        let _ = submenu.append(&langs);
                    }
                    let _ = submenu.append(&PredefinedMenuItem::separator());
                    items.push(None);
                } else {
                    let it = CheckMenuItem::new(
                        &item.text,
                        item.enabled,
                        item.checked,
                        accelerator(m.base_id, item.index),
                    );
                    actions.insert(
                        it.id().clone(),
                        MenuAction::Command(crate::menu_result(m.id, item.index)),
                    );
                    let _ = submenu.append(&it);
                    items.push(Some(it));
                }
            }
            let _ = menu.append(&submenu);
            menus.push(NativeMenu {
                id: m.id,
                submenu,
                items,
            });
        }
        menu.init_for_nsapp();

        Self {
            _menu: menu,
            lang,
            menus,
            lang_items,
            actions,
        }
    }

    /// Met la barre native en phase avec le modèle et renvoie l'action
    /// éventuellement choisie par l'utilisateur depuis la frame précédente.
    pub(crate) fn sync(slot: &mut Option<Self>, lang_mode: LangMode) -> Option<MenuAction> {
        let snapshot = acam::menus_snapshot();
        let stale = match slot {
            Some(bar) => bar.lang != langue() || bar.menus.len() != snapshot.len(),
            None => !snapshot.is_empty(),
        };
        if stale {
            // Les anciens items sont détruits : leurs événements en attente
            // portent des IDs inconnus, on les jette.
            while MenuEvent::receiver().try_recv().is_ok() {}
            *slot = Some(Self::new(&snapshot, lang_mode));
        }
        let bar = slot.as_mut()?;
        bar.update_state(&snapshot, lang_mode);

        let mut action = None;
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if let Some(a) = bar.actions.get(&event.id) {
                action = Some(*a);
            }
        }
        // Un `CheckMenuItem` bascule tout seul au clic : on réimpose l'état du
        // modèle pour que la coche reste celle décidée par le C.
        if action.is_some() {
            bar.update_state(&snapshot, lang_mode);
        }
        action
    }

    fn update_state(&self, snapshot: &[MenuSnapshot], lang_mode: LangMode) {
        for (native, m) in self.menus.iter().zip(snapshot) {
            if native.id != m.id {
                continue;
            }
            native.submenu.set_enabled(m.enabled);
            for (it, item) in native.items.iter().zip(&m.items) {
                if let Some(it) = it {
                    if it.is_enabled() != item.enabled {
                        it.set_enabled(item.enabled);
                    }
                    if it.is_checked() != item.checked {
                        it.set_checked(item.checked);
                    }
                }
            }
        }
        for (opt, it) in &self.lang_items {
            let checked = *opt == lang_mode;
            if it.is_checked() != checked {
                it.set_checked(checked);
            }
        }
    }
}
