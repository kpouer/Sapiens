//! Sortie audio web : `AudioContext` + `ScriptProcessorNode`.
//!
//! Le nœud tourne sur le thread principal du navigateur et appelle le même
//! `Mixer` que la version native. Celui-ci est construit dans un worker
//! (la synthèse des sons prend un moment) et partagé via un verrou à attente
//! active, jamais attendu par le callback audio (`try_lock`).
//!
//! Les navigateurs interdisent de démarrer l'audio sans geste de
//! l'utilisateur : le contexte est repris au premier clic/touche/toucher. Tant
//! qu'il ne tourne pas, `READY` reste faux et le jeu reste silencieux.

use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioContextState, AudioProcessingEvent};

use super::{Cmd, Mixer, READY};

const TAILLE_BLOC: u32 = 2048;

static MIXER: spin::Mutex<Option<Mixer>> = spin::Mutex::new(None);

pub(super) fn start(rx: Receiver<Cmd>) {
    let ctx = match AudioContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            log::warn!("audio: AudioContext indisponible ({e:?}), jeu silencieux");
            return;
        }
    };
    let sample_rate = ctx.sample_rate();

    crate::platform::spawn("sapiens_audio_init", 1024 * 1024, move || {
        *MIXER.lock() = Some(Mixer::new(sample_rate, rx));
    });

    let node = match ctx.create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        TAILLE_BLOC,
        1,
        2,
    ) {
        Ok(node) => node,
        Err(e) => {
            log::warn!("audio: ScriptProcessorNode indisponible ({e:?})");
            return;
        }
    };

    let mut scratch = vec![0.0f32; TAILLE_BLOC as usize];
    let on_process =
        Closure::<dyn FnMut(AudioProcessingEvent)>::new(move |ev: AudioProcessingEvent| {
            let Ok(out) = ev.output_buffer() else {
                return;
            };
            let len = out.length() as usize;
            if scratch.len() != len {
                scratch.resize(len, 0.0);
            }
            scratch.fill(0.0);
            if let Some(mut guard) = MIXER.try_lock()
                && let Some(mixer) = guard.as_mut()
            {
                mixer.render(&mut scratch, 1);
                // Le callback n'est appelé que contexte en marche.
                if !READY.swap(true, Ordering::Release) {
                    log::info!("audio: sortie démarrée");
                }
            }
            // La mémoire wasm est un SharedArrayBuffer : `copyToChannel` refuse
            // ces vues. On passe par un Float32Array JS non partagé.
            let tmp = js_sys::Float32Array::new_with_length(scratch.len() as u32);
            tmp.copy_from(&scratch);
            for ch in 0..out.number_of_channels() {
                if let Err(e) = out.copy_to_channel_with_f32_array(&tmp, ch as i32) {
                    log::warn!("audio: copyToChannel : {e:?}");
                }
            }
        });
    node.set_onaudioprocess(Some(on_process.as_ref().unchecked_ref()));
    on_process.forget();

    if node.connect_with_audio_node(&ctx.destination()).is_err() {
        log::warn!("audio: connexion à la sortie impossible");
        return;
    }

    // Reprise du contexte au premier geste de l'utilisateur.
    if let Some(window) = web_sys::window() {
        let resume = Closure::<dyn FnMut()>::new({
            let ctx = ctx.clone();
            move || {
                if let Ok(p) = ctx.resume() {
                    drop(p);
                }
            }
        });
        // Phase de capture : egui peut arrêter la propagation sur le canvas.
        for ev in [
            "pointerdown",
            "pointerup",
            "mousedown",
            "click",
            "keydown",
            "keyup",
            "touchstart",
            "touchend",
        ] {
            let _ = window.add_event_listener_with_callback_and_bool(
                ev,
                resume.as_ref().unchecked_ref(),
                true,
            );
        }
        resume.forget();
    }

    // Si le contexte est suspendu (onglet, politique d'autoplay), on coupe
    // `READY` pour que le jeu n'attende pas des effets qui ne finiront pas.
    let on_state = Closure::<dyn FnMut()>::new({
        let ctx = ctx.clone();
        move || {
            let state = ctx.state();
            log::info!("audio: état du contexte {state:?}");
            if state != AudioContextState::Running {
                READY.store(false, Ordering::Release);
            }
        }
    });
    ctx.set_onstatechange(Some(on_state.as_ref().unchecked_ref()));
    on_state.forget();
    log::info!(
        "audio: contexte créé ({:?}, {} Hz)",
        ctx.state(),
        sample_rate
    );
    // Le contexte et le nœud vivent aussi longtemps que la page.
    std::mem::forget(node);
    std::mem::forget(ctx);
}
