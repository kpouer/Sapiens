use log::{error, info};
use std::fmt::Display;
use std::sync::atomic::{AtomicI32, AtomicU8, Ordering};

pub enum Cursor {
    Hidden = 0,
    Default = 1,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Cursor::Hidden => "Hidden",
            Cursor::Default => "Default",
        };
        write!(f, "{}", str)
    }
}

impl From<u8> for Cursor {
    fn from(value: u8) -> Self {
        match value {
            0 => Cursor::Hidden,
            1 => Cursor::Default,
            _ => {
                error!("Unexpected cursor value {value}");
                Cursor::Hidden
            }
        }
    }
}

static CURSOR: AtomicU8 = AtomicU8::new(Cursor::Hidden as u8);

pub(crate) fn set_cursor(cursor: Cursor) {
    info!("set_cursor({cursor})");
    CURSOR.store(cursor as u8, Ordering::SeqCst);
}

pub(crate) fn get_cursor() -> Cursor {
    CURSOR.load(Ordering::SeqCst).into()
}

/* Commande de menu en attente : posee par le thread egui (src/main.rs, barre
de menus et raccourcis Cmd), reprise par le thread C dans GereInterface, car
le `&mut Sapiens` du jeu n'existe que sur le thread C. 0 = rien en attente
(MenuSelect/MenuKey renvoient 0 quand le clic n'est pas dans un menu). */
static MENU_COMMAND: AtomicI32 = AtomicI32::new(0);

/// Depose une commande de menu depuis le thread egui (barre de menus, raccourcis
/// Cmd). Elle sera executee par `GereInterface`, sur le thread C.
pub(crate) fn set_menu_command(menu_result: i32) {
    MENU_COMMAND.store(menu_result, Ordering::SeqCst);
}

/// Reprise et remise a zero de la commande en attente (cote thread C).
pub(crate) fn take_menu_command() -> Option<i32> {
    MENU_COMMAND.swap(0, Ordering::SeqCst).into()
}
