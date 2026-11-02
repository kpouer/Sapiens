//! File d'événements Mac, indépendante de la plateforme hôte.
//!
//! Le contrat (`EventQueue`) ne parle que d'`EventRecord` : aucune API macOS,
//! Windows ou web n'y entre. Deux implémentations suffisent :
//!
//! - `NativeEventQueue` : `Mutex<VecDeque<_>>`, pour macOS/Windows/Linux où le
//!   jeu et egui vivent sur deux threads ;
//! - `LocalEventQueue` : `thread_local! + RefCell`, pour un wasm32 mono-thread.
//!
//! Le backend est choisi à la compilation par `PlatformEventQueue` : pas de
//! `dyn`, pas d'allocation ni de branche à l'exécution. Le jour où wasm passe
//! en multi-thread, seul le type alias est à revoir.

use crate::shared::EventRecord;
use std::collections::VecDeque;

/// Au-delà, on jette l'événement le plus ancien : quand le thread du jeu est
/// bloqué (dialogue modal), les frappes périmées ne doivent pas être rejouées.
const MAX_EVENTS: usize = 64;

pub(crate) trait EventQueue: Send + Sync {
    fn push(&self, event: EventRecord);
    fn pop(&self, mask: i16) -> Option<EventRecord>;
    fn clear(&self);
}

fn push_bounded(queue: &mut VecDeque<EventRecord>, event: EventRecord) {
    if queue.len() >= MAX_EVENTS {
        queue.pop_front();
    }
    queue.push_back(event);
}

fn pop_matching(queue: &mut VecDeque<EventRecord>, mask: i16) -> Option<EventRecord> {
    let index = queue
        .iter()
        .position(|event| event_matches(mask, event.what))?;
    queue.remove(index)
}

/// Masque Mac : `everyEvent` vaut `-1` en `i16`, sinon le bit `what` est posé
/// (`1 << keyDown`, etc.). Les types au-delà de 15 ne sont atteignables que par
/// `everyEvent`, ce qui suffit au jeu.
fn event_matches(mask: i16, what: i16) -> bool {
    let mask = mask as u16;
    if mask == u16::MAX {
        return true;
    }
    (0..16).contains(&what) && (mask & (1u16 << what as u32)) != 0
}

// ---------------------------------------------------------------------------
// Backend natif (macOS / Windows / Linux)
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct NativeEventQueue {
    queue: std::sync::Mutex<VecDeque<EventRecord>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeEventQueue {
    pub(crate) const fn new() -> Self {
        Self {
            queue: std::sync::Mutex::new(VecDeque::new()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl EventQueue for NativeEventQueue {
    fn push(&self, event: EventRecord) {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        push_bounded(&mut queue, event);
    }

    fn pop(&self, mask: i16) -> Option<EventRecord> {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        pop_matching(&mut queue, mask)
    }

    fn clear(&self) {
        self.queue.lock().unwrap_or_else(|e| e.into_inner()).clear();
    }
}

// ---------------------------------------------------------------------------
// Backend wasm mono-thread
// ---------------------------------------------------------------------------

/// Handle vide : le stockage réel est thread-local, donc pas besoin d'atomics
/// ni de verrou pour un wasm32 sans threads.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) struct LocalEventQueue;

#[cfg(any(target_arch = "wasm32", test))]
impl LocalEventQueue {
    pub(crate) const fn new() -> Self {
        Self
    }
}

#[cfg(any(target_arch = "wasm32", test))]
std::thread_local! {
    static LOCAL_QUEUE: std::cell::RefCell<VecDeque<EventRecord>> =
        const { std::cell::RefCell::new(VecDeque::new()) };
}

#[cfg(any(target_arch = "wasm32", test))]
impl EventQueue for LocalEventQueue {
    fn push(&self, event: EventRecord) {
        LOCAL_QUEUE.with(|queue| push_bounded(&mut queue.borrow_mut(), event));
    }

    fn pop(&self, mask: i16) -> Option<EventRecord> {
        LOCAL_QUEUE.with(|queue| pop_matching(&mut queue.borrow_mut(), mask))
    }

    fn clear(&self) {
        LOCAL_QUEUE.with(|queue| queue.borrow_mut().clear());
    }
}

// ---------------------------------------------------------------------------
// Façade utilisée par le reste du portage
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type PlatformEventQueue = NativeEventQueue;

#[cfg(target_arch = "wasm32")]
pub(crate) type PlatformEventQueue = LocalEventQueue;

static EVENTS: PlatformEventQueue = PlatformEventQueue::new();

/// Dépose un événement (thread egui).
pub(crate) fn push_event(event: EventRecord) {
    EVENTS.push(event);
}

/// Retire le premier événement compatible avec `mask` (thread du jeu).
#[allow(non_snake_case)]
pub(crate) fn GetNextEvent(mask: i16) -> Option<EventRecord> {
    EVENTS.pop(mask)
}

/// Vide les événements en attente (ouverture d'un modal, perte de focus).
pub(crate) fn clear_events() {
    EVENTS.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(what: i16, message: i64) -> EventRecord {
        EventRecord {
            what,
            message,
            ..Default::default()
        }
    }

    fn verifie_file<Q: EventQueue>(queue: &Q) {
        queue.push(event(3, 1));
        queue.push(event(1, 2));
        queue.push(event(3, 3));

        // FIFO et respect du masque : keyDown uniquement.
        assert_eq!(queue.pop(1 << 3), Some(event(3, 1)));
        assert_eq!(queue.pop(1 << 3), Some(event(3, 3)));

        // L'événement souris non demandé est resté dans la file.
        assert_eq!(queue.pop(1 << 1), Some(event(1, 2)));
        assert_eq!(queue.pop(i16::MIN), None);

        // clear vide réellement la file.
        queue.push(event(3, 4));
        queue.clear();
        assert_eq!(queue.pop(i16::MIN), None);
    }

    #[test]
    fn file_native() {
        verifie_file(&NativeEventQueue::new());
    }

    #[test]
    fn file_locale() {
        verifie_file(&LocalEventQueue::new());
    }

    #[test]
    fn souris_down_puis_up_dans_l_ordre() {
        let queue = NativeEventQueue::new();
        queue.push(event(1, 0));
        queue.push(event(2, 0));
        assert_eq!(queue.pop(1 << 2), Some(event(2, 0)));
        assert_eq!(queue.pop(1 << 1), Some(event(1, 0)));
    }

    #[test]
    fn every_event_prend_le_premier() {
        let queue = NativeEventQueue::new();
        queue.push(event(1, 1));
        queue.push(event(3, 2));
        assert_eq!(queue.pop(-1), Some(event(1, 1)));
        assert_eq!(queue.pop(-1), Some(event(3, 2)));
    }

    #[test]
    fn saturation_jette_les_plus_anciens() {
        let queue = NativeEventQueue::new();
        for i in 0..MAX_EVENTS as i64 + 2 {
            queue.push(event(3, i));
        }
        assert_eq!(queue.pop(-1), Some(event(3, 2)));
    }
}
