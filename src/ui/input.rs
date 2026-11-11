//! Traduction des entrées egui (souris, clavier) vers l'état et les
//! événements Mac consommés par le thread du jeu (`exchange`).

use crate::cbridge::{cmdKey, controlKey, keyDown, mouseDown, mouseUp, optionKey, shiftKey};
use crate::exchange::{INPUT, MOUSE_EVENT_SCALE, clear_events, push_event};
use crate::shared::{EventRecord, Point};
use eframe::egui;

/// Alimente l'état d'entrée partagé (utilisé par les stubs GetMouse/Button au
/// moment voulu). Les coordonnées sont normalisées dans la zone de jeu (sous la
/// barre de menus), pas dans toute la fenêtre.
///
/// Traduit aussi les événements clavier egui en `keyDown` Mac pour la file
/// consommée par `GetNextEvent`. `accept_events` est faux quand un dialogue
/// modal est ouvert : le thread du jeu est alors bloqué et ne doit pas recevoir
/// les frappes destinées à la boîte de dialogue.
pub(super) fn record_input(ctx: &egui::Context, area: egui::Rect, accept_events: bool) {
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
