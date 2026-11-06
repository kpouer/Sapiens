//! Séquenceur des musiques originales (`muse1`/`muse2`).
//!
//! Port de `JoueVoie`/`Kmuse` (`Sources/SAPWMUS.C`, chemin `OLD_VERSION`) :
//! trois voies, un tick de 50 Hz, les répétitions de `dvoie*` et les
//! changements d'instrument. Les timbres General MIDI sont synthétisés.

use std::sync::atomic::Ordering;

use crate::sapiens::muse::{dvoie1, dvoie2, dvoie3, dvoie21, dvoie22, dvoie23, muse1, muse2};

use super::MUSIC_FINISHED;

const TICK_HZ: f32 = 50.0;
const NB_VOIES: usize = 3;
const GAIN_MUSIQUE: f32 = 0.26;

#[derive(Clone, Copy)]
struct Instr {
    program: u8,
    decal_note: i8,
}

impl Instr {
    const fn new(program: u8, decal_note: i8) -> Self {
        Self {
            program,
            decal_note,
        }
    }
}

const INS1: Instr = Instr::new(33, 0);
const INS2: Instr = Instr::new(116, 12);
const INS4: Instr = Instr::new(116, 12);
const INS6: Instr = Instr::new(12, 0);
const INS8: Instr = Instr::new(75, 12);
const INS21: Instr = Instr::new(7, 0);
const INS22: Instr = Instr::new(115, 0);
const INS24: Instr = Instr::new(37, 0);
const INS25: Instr = Instr::new(119, 0);
const INS27: Instr = Instr::new(117, 0);
const INS28: Instr = Instr::new(118, 0);
const INS29: Instr = Instr::new(87, 0);
const INS2A: Instr = Instr::new(110, 0);
const INS2B: Instr = Instr::new(113, 0);
const INS2C: Instr = Instr::new(32, 0);
const INS2D: Instr = Instr::new(30, 12);
const INS2E: Instr = Instr::new(28, 0);
const INS2F: Instr = Instr::new(118, 0);

static TABINS1: [Instr; 16] = [
    INS1, INS1, INS2, INS1, INS4, INS1, INS6, INS1, INS8, INS1, INS1, INS1, INS1, INS1, INS1, INS1,
];

static TABINS2: [Instr; 16] = [
    INS1, INS21, INS22, INS21, INS24, INS25, INS2C, INS27, INS28, INS29, INS2A, INS2B, INS2C,
    INS2D, INS2E, INS2F,
];

static DEFS1: [&[u16]; NB_VOIES] = [&dvoie1, &dvoie2, &dvoie3];
static DEFS2: [&[u16]; NB_VOIES] = [&dvoie21, &dvoie22, &dvoie23];

#[derive(Clone, Copy)]
struct SeqVoice {
    posvoie: usize,
    posseq: usize,
    debseq: usize,
    nbrseq: i32,
    base: i32,
    insdef: usize,
    instr: usize,
    maxtime: u16,
    time: u16,
    effet: u8,
    fini: bool,
}

impl SeqVoice {
    fn new(def: &[u16]) -> Self {
        let mut v = Self {
            posvoie: 4,
            posseq: 0,
            debseq: 0,
            nbrseq: 1,
            base: 0,
            insdef: 0,
            instr: 0,
            maxtime: 0,
            time: 0,
            effet: 0,
            fini: false,
        };
        if def.len() < 4 || def[0] == 0 {
            v.fini = true;
            return v;
        }
        v.nbrseq = def[0] as i32;
        v.debseq = def[1] as usize;
        v.posseq = v.debseq;
        v.base = def[2] as i32;
        v.insdef = (def[3] as usize).min(15);
        v.instr = v.insdef;
        v
    }

    fn octet(&mut self, muse: &[u8]) -> Option<u8> {
        if self.posseq < muse.len() {
            let b = muse[self.posseq];
            self.posseq += 1;
            Some(b)
        } else {
            self.fini = true;
            None
        }
    }

    fn tick(&mut self, def: &[u16], muse: &[u8], tabins: &'static [Instr; 16]) -> Option<Note> {
        if self.fini {
            return None;
        }
        self.time = self.time.saturating_add(1);
        if self.time < self.maxtime {
            return None;
        }

        loop {
            let car = self.octet(muse)?;
            if car >= 128 {
                if car == 255 {
                    self.effet = 0;
                    self.nbrseq -= 1;
                    if self.nbrseq <= 0 {
                        if self.posvoie + 3 >= def.len() {
                            self.fini = true;
                            return None;
                        }
                        let rep = def[self.posvoie];
                        let offset = def[self.posvoie + 1] as usize;
                        let base = def[self.posvoie + 2] as i32;
                        let insdef = (def[self.posvoie + 3] as usize).min(15);
                        self.posvoie += 4;
                        if rep == 0 {
                            self.fini = true;
                            return None;
                        }
                        self.nbrseq = rep as i32;
                        self.debseq = offset;
                        self.posseq = offset;
                        self.base = base;
                        self.insdef = insdef;
                    } else {
                        self.posseq = self.debseq;
                    }
                } else {
                    let c = car & 127;
                    if c < 16 {
                        self.instr = if c == 0 { self.insdef } else { c as usize };
                    } else {
                        let effet = c & 15;
                        if effet >= 3 {
                            self.effet = effet;
                        }
                        let _param = self.octet(muse)?;
                    }
                }
            } else {
                let midi = if car == 0 {
                    0
                } else {
                    car as i32 + self.base + i32::from(tabins[self.instr].decal_note)
                };
                let ticks = self.octet(muse)?;
                self.maxtime = u16::from(ticks).max(1);
                self.time = 0;
                return Some(Note {
                    midi,
                    ticks: self.maxtime,
                    instr: tabins[self.instr],
                });
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Note {
    midi: i32,
    ticks: u16,
    instr: Instr,
}

#[derive(Clone, Copy, Default)]
enum Timbre {
    #[default]
    Pad,
    Bass,
    SlapBass,
    Taiko,
    Tom,
    Drum,
    Marimba,
    Flute,
    Clav,
    Guitar,
    Fiddle,
    Bell,
    Agogo,
}

fn timbre(program: u8) -> Timbre {
    match program {
        32 | 33 => Timbre::Bass,
        37 => Timbre::SlapBass,
        116 => Timbre::Taiko,
        117 => Timbre::Tom,
        115 | 118 | 119 => Timbre::Drum,
        12 => Timbre::Marimba,
        75 => Timbre::Flute,
        7 => Timbre::Clav,
        28..=31 => Timbre::Guitar,
        87 => Timbre::Pad,
        110 | 111 => Timbre::Fiddle,
        112 => Timbre::Bell,
        113 => Timbre::Agogo,
        _ => Timbre::Pad,
    }
}

#[derive(Clone, Copy)]
struct SynthVoice {
    active: bool,
    kind: Timbre,
    freq: f32,
    phase: f32,
    vib_phase: f32,
    pos: u64,
    stop_at: u64,
    release_at: u64,
    release: f32,
    attack: f32,
    gain: f32,
    noise: u32,
    lp: f32,
}

impl Default for SynthVoice {
    fn default() -> Self {
        Self {
            active: false,
            kind: Timbre::Pad,
            freq: 440.0,
            phase: 0.0,
            vib_phase: 0.0,
            pos: 0,
            stop_at: 0,
            release_at: 0,
            release: 0.04,
            attack: 0.01,
            gain: 0.0,
            noise: 0x1234_5678,
            lp: 0.0,
        }
    }
}

impl SynthVoice {
    fn stop(&mut self) {
        self.active = false;
    }

    fn note_on(&mut self, note: Note, sr: f32) {
        if note.midi <= 0 {
            self.stop();
            return;
        }
        let midi = note.midi.clamp(21, 108) as f32;
        let kind = timbre(note.instr.program);
        let freq = match kind {
            Timbre::Taiko => 88.0 + (midi as i32 % 12) as f32 * 4.0,
            Timbre::Tom => 130.0 + (midi as i32 % 12) as f32 * 7.0,
            Timbre::Drum => 155.0 + (midi as i32 % 12) as f32 * 9.0,
            _ => 440.0 * 2.0f32.powf((midi - 69.0) / 12.0),
        };
        let seconds = note.ticks as f32 / TICK_HZ;
        let release = 0.04;
        self.kind = kind;
        self.freq = freq;
        self.phase = 0.0;
        self.vib_phase = 0.0;
        self.pos = 0;
        self.release_at = (seconds * sr).max(1.0) as u64;
        self.release = release;
        self.stop_at = self.release_at + (release * sr).max(1.0) as u64;
        self.attack = match kind {
            Timbre::Pad | Timbre::Fiddle => 0.035,
            Timbre::Drum | Timbre::Taiko | Timbre::Tom | Timbre::Agogo | Timbre::Bell => 0.002,
            _ => 0.008,
        };
        self.gain = match kind {
            Timbre::Drum | Timbre::Taiko | Timbre::Tom => 0.34,
            Timbre::Bass | Timbre::SlapBass => 0.28,
            _ => 0.22,
        };
        self.active = true;
    }

    fn noise(&mut self) -> f32 {
        self.noise ^= self.noise << 13;
        self.noise ^= self.noise >> 17;
        self.noise ^= self.noise << 5;
        (self.noise >> 8) as f32 / 8_388_608.0 - 1.0
    }

    fn sample(&mut self, sr: f32) -> f32 {
        if !self.active {
            return 0.0;
        }
        if self.pos >= self.stop_at {
            self.active = false;
            return 0.0;
        }
        let t = self.pos as f32 / sr;
        let rel_start = self.release_at as f32 / sr;
        let rel_end = self.stop_at as f32 / sr;
        let env = if t < self.attack {
            t / self.attack
        } else if t < rel_start {
            1.0
        } else {
            ((rel_end - t) / self.release).clamp(0.0, 1.0)
        };

        let mut vib_freq = self.freq;
        if matches!(self.kind, Timbre::Flute | Timbre::Fiddle) {
            self.vib_phase += std::f32::consts::TAU * 5.2 / sr;
            vib_freq *= 1.0 + 0.008 * self.vib_phase.sin();
        }
        self.phase += std::f32::consts::TAU * vib_freq / sr;
        if self.phase >= std::f32::consts::TAU {
            self.phase -= std::f32::consts::TAU;
        }

        let raw = match self.kind {
            Timbre::Bass => {
                self.phase.sin() + 0.32 * (2.0 * self.phase).sin() + 0.12 * (3.0 * self.phase).sin()
            }
            Timbre::SlapBass => {
                (self.phase.sin() + 0.45 * (2.0 * self.phase).sin()) * (-t * 2.0).exp()
            }
            Timbre::Marimba => {
                let e = (-t * 8.0).exp();
                (self.phase.sin() + 0.45 * (4.0 * self.phase).sin()) * e
            }
            Timbre::Flute => self.phase.sin() + 0.18 * (2.0 * self.phase).sin(),
            Timbre::Clav => {
                let e = (-t * 5.0).exp();
                (self.phase.sin() + 0.35 * (3.0 * self.phase).sin()) * e
            }
            Timbre::Guitar => {
                let e = (-t * 3.2).exp();
                (self.phase.sin()
                    + 0.30 * (2.0 * self.phase).sin()
                    + 0.18 * (3.0 * self.phase).sin())
                    * e
            }
            Timbre::Fiddle => {
                self.phase.sin() + 0.30 * (2.0 * self.phase).sin() + 0.18 * (3.0 * self.phase).sin()
            }
            Timbre::Pad => self.phase.sin() + 0.25 * (2.0 * self.phase).sin(),
            Timbre::Bell => {
                let e = (-t * 3.0).exp();
                (self.phase.sin()
                    + 0.45 * (2.76 * self.phase).sin()
                    + 0.20 * (5.40 * self.phase).sin())
                    * e
            }
            Timbre::Agogo => {
                let e = (-t * 7.0).exp();
                (self.phase.sin() + 0.50 * (2.2 * self.phase).sin()) * e
            }
            Timbre::Taiko => {
                let n = self.noise();
                let e = (-t * 11.0).exp();
                (self.phase.sin() * 0.90 + n * 0.28 * (-t * 30.0).exp()) * e
            }
            Timbre::Tom => {
                let n = self.noise();
                let e = (-t * 14.0).exp();
                (self.phase.sin() * 0.70 + n * 0.45 * (-t * 25.0).exp()) * e
            }
            Timbre::Drum => {
                let n = self.noise();
                self.lp += 0.35 * (n - self.lp);
                let e = (-t * 17.0).exp();
                (self.lp * 0.85 + self.phase.sin() * 0.25) * e
            }
        };

        self.pos += 1;
        if self.pos >= self.stop_at {
            self.active = false;
        }
        raw * env * self.gain
    }
}

pub(super) struct MusicPlayer {
    sample_rate: f32,
    active: bool,
    finished: bool,
    muse: &'static [u8],
    tabins: &'static [Instr; 16],
    defs: [&'static [u16]; NB_VOIES],
    voices: [SeqVoice; NB_VOIES],
    synths: [SynthVoice; NB_VOIES],
    tick_acc: f32,
}

impl MusicPlayer {
    pub(super) fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            active: false,
            finished: false,
            muse: &[],
            tabins: &TABINS1,
            defs: [&[], &[], &[]],
            voices: [SeqVoice::new(&[]); NB_VOIES],
            synths: [SynthVoice::default(); NB_VOIES],
            tick_acc: 0.0,
        }
    }

    pub(super) fn start(&mut self, song: u8) {
        let (muse, tabins, defs) = if song == 1 {
            (&muse1[..], &TABINS1, DEFS1)
        } else {
            (&muse2[..], &TABINS2, DEFS2)
        };
        self.muse = muse;
        self.tabins = tabins;
        self.defs = defs;
        for (i, def) in defs.iter().enumerate().take(NB_VOIES) {
            self.voices[i] = SeqVoice::new(def);
            self.synths[i].stop();
        }
        self.active = true;
        self.finished = false;
        self.tick_acc = 0.0;
        MUSIC_FINISHED.store(false, Ordering::Release);
    }

    pub(super) fn stop(&mut self) {
        self.active = false;
        self.finished = false;
        for synth in &mut self.synths {
            synth.stop();
        }
        MUSIC_FINISHED.store(false, Ordering::Release);
    }

    pub(super) fn sample(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }
        self.tick_acc += TICK_HZ / self.sample_rate;
        while self.tick_acc >= 1.0 {
            self.tick_acc -= 1.0;
            self.tick();
        }
        let mut out = 0.0;
        for synth in &mut self.synths {
            out += synth.sample(self.sample_rate);
        }
        out * GAIN_MUSIQUE
    }

    fn tick(&mut self) {
        let mut toutes_finies = true;
        for i in 0..NB_VOIES {
            if self.voices[i].fini {
                self.synths[i].stop();
                continue;
            }
            let note = self.voices[i].tick(self.defs[i], self.muse, self.tabins);
            match note {
                Some(note) => self.synths[i].note_on(note, self.sample_rate),
                None if self.voices[i].fini => self.synths[i].stop(),
                None => {}
            }
            if !self.voices[i].fini {
                toutes_finies = false;
            }
        }
        if toutes_finies && !self.finished {
            self.finished = true;
            self.active = false;
            MUSIC_FINISHED.store(true, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_deux_musiques_se_terminent() {
        for (song, limite) in [(1u8, 25_000usize), (2, 35_000)] {
            let mut player = MusicPlayer::new(48_000.0);
            player.start(song);
            for _ in 0..limite {
                player.tick();
                if player.finished {
                    break;
                }
            }
            assert!(player.finished, "musique {song} non terminée");
        }
    }

    #[test]
    fn rendu_fini_et_audible() {
        let mut player = MusicPlayer::new(48_000.0);
        player.start(1);
        let mut peak = 0.0f32;
        for _ in 0..48_000 {
            let v = player.sample();
            assert!(v.is_finite());
            peak = peak.max(v.abs());
        }
        assert!(peak > 0.001, "musique silencieuse : {peak}");
        assert!(peak <= 1.0, "musique saturée : {peak}");
    }
}
