//! Sortie audio du portage : effets sonores et musiques originales.
//!
//! Un thread dédié possède le flux `cpal` (CoreAudio). Le thread du jeu et
//! egui ne font que déposer des commandes dans un canal, sans jamais toucher
//! au callback temps réel.
//!
//! Sur le web (wasm32), le même mixeur alimente un `ScriptProcessorNode`
//! (voir `web`) ; le son démarre au premier geste de l'utilisateur.

mod music;
mod sfx;
#[cfg(target_arch = "wasm32")]
mod web;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use crate::platform::Instant;
#[cfg(not(target_arch = "wasm32"))]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(not(target_arch = "wasm32"))]
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};

use music::MusicPlayer;

const NB_VOIX_SFX: usize = 8;
const ATTENTE_SFX_MAX: Duration = Duration::from_secs(3);

static TX: OnceLock<Sender<Cmd>> = OnceLock::new();
static STARTED: OnceLock<()> = OnceLock::new();
static READY: AtomicBool = AtomicBool::new(false);
static MUSIC_FINISHED: AtomicBool = AtomicBool::new(false);
static SFX_ACTIVE: AtomicUsize = AtomicUsize::new(0);

enum Cmd {
    Sfx(u8),
    Music(Option<u8>),
    StopMusic,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn init() {
    if STARTED.set(()).is_err() {
        return;
    }
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = TX.set(tx);
    web::start(rx);
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn init() {
    if STARTED.set(()).is_err() {
        return;
    }
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = TX.set(tx);
    if let Err(e) = std::thread::Builder::new()
        .name("sapiens_audio".into())
        .spawn(move || audio_thread(rx))
    {
        log::warn!("audio: impossible de démarrer le thread : {e}");
    }
}

pub(crate) fn play_sfx(num: u8) {
    if !READY.load(Ordering::Acquire) {
        return;
    }
    let Some(tx) = TX.get() else {
        return;
    };
    SFX_ACTIVE.fetch_add(1, Ordering::AcqRel);
    if tx.send(Cmd::Sfx(num)).is_err() {
        sfx_done();
    }
}

pub(crate) fn wait_sfx() {
    let deadline = Instant::now() + ATTENTE_SFX_MAX;
    while SFX_ACTIVE.load(Ordering::Acquire) > 0 && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(2));
    }
}

pub(crate) fn play_music(song: u8) {
    if let Some(tx) = TX.get() {
        let _ = tx.send(Cmd::Music(Some(song.clamp(1, 2))));
        MUSIC_FINISHED.store(false, Ordering::Release);
    }
}

pub(crate) fn stop_music() {
    if let Some(tx) = TX.get() {
        let _ = tx.send(Cmd::StopMusic);
        MUSIC_FINISHED.store(false, Ordering::Release);
    }
}

pub(crate) fn is_music_finished() -> bool {
    MUSIC_FINISHED.load(Ordering::Acquire)
}

#[cfg(not(target_arch = "wasm32"))]
fn audio_thread(rx: Receiver<Cmd>) {
    match build_stream(rx) {
        Ok(stream) => {
            if let Err(e) = stream.play() {
                log::warn!("audio: lecture impossible : {e}");
                return;
            }
            READY.store(true, Ordering::Release);
            loop {
                std::thread::park();
            }
        }
        Err(e) => log::warn!("audio: sortie indisponible ({e}), jeu silencieux"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn build_stream(rx: Receiver<Cmd>) -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| "aucun périphérique de sortie".to_string())?;
    let supported = device
        .default_output_config()
        .map_err(|e| format!("configuration par défaut : {e}"))?;
    let format = supported.sample_format();
    let config: StreamConfig = supported.config();
    let sample_rate = config.sample_rate as f32;
    let mixer = Mixer::new(sample_rate, rx);

    match format {
        SampleFormat::F32 => build::<f32>(&device, config, mixer),
        SampleFormat::I16 => build::<i16>(&device, config, mixer),
        SampleFormat::U16 => build::<u16>(&device, config, mixer),
        SampleFormat::I8 => build::<i8>(&device, config, mixer),
        SampleFormat::U8 => build::<u8>(&device, config, mixer),
        SampleFormat::I32 => build::<i32>(&device, config, mixer),
        SampleFormat::U32 => build::<u32>(&device, config, mixer),
        SampleFormat::F64 => build::<f64>(&device, config, mixer),
        autre => Err(format!("format d'échantillon non géré : {autre:?}")),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn build<T>(
    device: &cpal::Device,
    config: StreamConfig,
    mut mixer: Mixer,
) -> Result<cpal::Stream, String>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;
    let mut scratch = Vec::<f32>::new();
    device
        .build_output_stream(
            config,
            move |data: &mut [T], _| {
                if scratch.len() < data.len() {
                    scratch.resize(data.len(), 0.0);
                }
                let buf = &mut scratch[..data.len()];
                mixer.render(buf, channels);
                for (dst, src) in data.iter_mut().zip(buf.iter()) {
                    *dst = T::from_sample(*src);
                }
            },
            |err: cpal::Error| log::warn!("audio: {err}"),
            None,
        )
        .map_err(|e| e.to_string())
}

#[derive(Default)]
struct SfxVoice {
    buf: Option<Arc<Vec<f32>>>,
    pos: usize,
}

impl SfxVoice {
    fn sample(&mut self) -> f32 {
        let Some(buf) = self.buf.as_ref() else {
            return 0.0;
        };
        let v = buf[self.pos];
        self.pos += 1;
        if self.pos >= buf.len() {
            self.buf = None;
            sfx_done();
        }
        v
    }
}

struct Mixer {
    rx: Receiver<Cmd>,
    sons: [Arc<Vec<f32>>; 13],
    voix: [SfxVoice; NB_VOIX_SFX],
    music: MusicPlayer,
}

impl Mixer {
    fn new(sample_rate: f32, rx: Receiver<Cmd>) -> Self {
        Self {
            rx,
            sons: sfx::generate_all(sample_rate),
            voix: std::array::from_fn(|_| SfxVoice::default()),
            music: MusicPlayer::new(sample_rate),
        }
    }

    fn render(&mut self, out: &mut [f32], channels: usize) {
        self.drain();
        let channels = channels.max(1);
        for frame in out.chunks_mut(channels) {
            let mut s = self.music.sample();
            for voice in &mut self.voix {
                s += voice.sample();
            }
            let s = s.clamp(-0.95, 0.95);
            for v in frame {
                *v = s;
            }
        }
    }

    fn drain(&mut self) {
        while let Ok(cmd) = self.rx.try_recv() {
            match cmd {
                Cmd::Sfx(num) => self.trigger(num),
                Cmd::Music(Some(song)) => self.music.start(song),
                Cmd::Music(None) | Cmd::StopMusic => self.music.stop(),
            }
        }
    }

    fn trigger(&mut self, num: u8) {
        let Some(buf) = self.sons.get(num as usize).cloned() else {
            sfx_done();
            return;
        };
        let slot = self
            .voix
            .iter()
            .position(|v| v.buf.is_none())
            .unwrap_or_else(|| {
                sfx_done();
                0
            });
        self.voix[slot] = SfxVoice {
            buf: Some(buf),
            pos: 0,
        };
    }
}

fn sfx_done() {
    if SFX_ACTIVE.load(Ordering::Acquire) > 0 {
        SFX_ACTIVE.fetch_sub(1, Ordering::AcqRel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixage_sfx_et_musique() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut mixer = Mixer::new(48_000.0, rx);
        SFX_ACTIVE.store(1, Ordering::Release);
        tx.send(Cmd::Sfx(6)).unwrap();
        let mut buf = vec![0.0f32; 2048];
        mixer.render(&mut buf, 2);
        assert!(buf.iter().all(|v| v.is_finite()));
        assert!(buf.iter().any(|v| v.abs() > 0.001));

        tx.send(Cmd::Music(Some(1))).unwrap();
        let mut peak = 0.0f32;
        for _ in 0..20 {
            mixer.render(&mut buf, 2);
            for &v in &buf {
                assert!(v.is_finite());
                peak = peak.max(v.abs());
            }
        }
        assert!(peak > 0.001, "musique inaudible : {peak}");

        tx.send(Cmd::StopMusic).unwrap();
        mixer.render(&mut buf, 2);
        SFX_ACTIVE.store(0, Ordering::Release);
    }
}
