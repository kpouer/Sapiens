//! Effets sonores procéduraux.
//!
//! Les ressources `snd ` du Mac d'origine ne sont plus présentes dans les
//! données du jeu : chaque bruit est donc synthétisé au démarrage, une fois
//! pour le sample rate du périphérique de sortie.

use std::sync::Arc;

const NB_SONS: usize = 13;

pub(super) fn generate_all(sample_rate: f32) -> [Arc<Vec<f32>>; NB_SONS] {
    std::array::from_fn(|i| Arc::new(generate(i as u8, sample_rate)))
}

fn generate(num: u8, sr: f32) -> Vec<f32> {
    let mut out = match num {
        0 => coup(0.16, 72.0, 0.85, sr),
        1 => impact(0.10, 1250.0, 0.55, sr),
        2 => metal(0.26, 780.0, 0.62, sr),
        3 => impact(0.14, 420.0, 0.62, sr),
        4 => craquement(0.30, sr),
        5 => coup(0.24, 52.0, 0.95, sr),
        6 => arpege(&[523.25, 659.25, 783.99, 1046.5], 0.11, 0.50, sr),
        7 => echec(0.48, sr),
        8 => cri(0.38, 420.0, 300.0, 0.55, sr),
        9 => double_clic(0.13, sr),
        10 => taille_silex(0.20, sr),
        11 => cri(0.34, 180.0, 105.0, 0.75, sr),
        12 => cri(0.30, 620.0, 380.0, 0.60, sr),
        _ => double_clic(0.13, sr),
    };
    normaliser(&mut out, 0.9);
    out
}

fn normaliser(buf: &mut [f32], max: f32) {
    let peak = buf.iter().fold(0.0f32, |p, v| p.max(v.abs()));
    if peak > max {
        let gain = max / peak;
        for v in buf {
            *v *= gain;
        }
    }
}

struct Rng(u32);

impl Rng {
    fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }

    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 >> 8) as f32 / 8_388_608.0 - 1.0
    }
}

fn len(sr: f32, seconds: f32) -> usize {
    (sr * seconds).max(1.0) as usize
}

fn fade_in_out(buf: &mut [f32], sr: f32) {
    let fade = (sr * 0.003).max(1.0) as usize;
    let n = buf.len();
    for i in 0..fade.min(n) {
        let g = i as f32 / fade as f32;
        buf[i] *= g;
        buf[n - 1 - i] *= g;
    }
}

fn coup(duration: f32, freq: f32, amp: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut rng = Rng::new(0x51F0_0001);
    let mut phase = 0.0f32;
    let mut lp = 0.0f32;
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        phase += std::f32::consts::TAU * freq * (1.0 - 0.35 * t / duration) / sr;
        let body = phase.sin() * (-t * 18.0).exp();
        lp += 0.18 * (rng.next() - lp);
        let noise = lp * (-t * 35.0).exp() * 0.55;
        *s = (body * 0.85 + noise) * amp;
    }
    fade_in_out(&mut out, sr);
    out
}

fn impact(duration: f32, freq: f32, amp: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut rng = Rng::new(0x51F0_0002 ^ freq as u32);
    let mut phase = 0.0f32;
    let mut hp = 0.0f32;
    let mut prev = 0.0f32;
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        phase += std::f32::consts::TAU * freq / sr;
        let click = phase.sin() * (-t * 55.0).exp();
        let raw = rng.next();
        hp = 0.55 * (hp + raw - prev);
        prev = raw;
        let noise = hp * (-t * 45.0).exp();
        *s = (click * 0.45 + noise * 0.65) * amp;
    }
    fade_in_out(&mut out, sr);
    out
}

fn metal(duration: f32, freq: f32, amp: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut rng = Rng::new(0x51F0_0003);
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        let env = (-t * 11.0).exp();
        let a = (std::f32::consts::TAU * freq * t).sin();
        let b = (std::f32::consts::TAU * freq * 1.517 * t).sin();
        let c = (std::f32::consts::TAU * freq * 2.243 * t).sin();
        let transient = rng.next() * (-t * 70.0).exp() * 0.35;
        *s = (a * 0.45 + b * 0.30 + c * 0.20) * env * amp + transient * amp;
    }
    fade_in_out(&mut out, sr);
    out
}

fn craquement(duration: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut rng = Rng::new(0x51F0_0004);
    let mut lp = 0.0f32;
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        let raw = rng.next();
        lp += 0.10 * (raw - lp);
        let burst = (lp * 1.8 + raw * 0.25) * (-t * 7.0).exp();
        let crackle = if rng.next() > 0.985 { raw * 1.5 } else { 0.0 };
        *s = (burst + crackle) * 0.75;
    }
    fade_in_out(&mut out, sr);
    out
}

fn arpege(notes: &[f32], step: f32, amp: f32, sr: f32) -> Vec<f32> {
    let duration = step * notes.len() as f32 + 0.22;
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    for (k, &freq) in notes.iter().enumerate() {
        let start = (sr * step * k as f32) as usize;
        for (i, item) in out.iter_mut().enumerate().take(n).skip(start) {
            let t = (i - start) as f32 / sr;
            if t > 0.42 {
                break;
            }
            let env = (-t * 5.0).exp() * (t * 160.0).min(1.0);
            let p = std::f32::consts::TAU * freq * t;
            *item += (p.sin() + 0.25 * (2.0 * p).sin()) * env * amp * 0.55;
        }
    }
    fade_in_out(&mut out, sr);
    out
}

fn echec(duration: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut phase = 0.0f32;
    let mut lp = 0.0f32;
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        let freq = 190.0 - 80.0 * (t / duration);
        phase += std::f32::consts::TAU * freq / sr;
        let raw = if phase.sin() >= 0.0 { 1.0 } else { -1.0 };
        lp += 0.08 * (raw - lp);
        let env = (t * 30.0).min(1.0) * (-t * 3.0).exp();
        *s = lp * env * 0.45;
    }
    fade_in_out(&mut out, sr);
    out
}

fn cri(duration: f32, f0: f32, f1: f32, amp: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut rng = Rng::new(0x51F0_0005 ^ f0 as u32);
    let mut phase = 0.0f32;
    let mut vib = 0.0f32;
    let mut lp = 0.0f32;
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        let u = (t / duration).min(1.0);
        let freq = f0 + (f1 - f0) * u;
        vib += std::f32::consts::TAU * 6.5 / sr;
        phase += std::f32::consts::TAU * freq * (1.0 + 0.018 * vib.sin()) / sr;
        let raw = phase.sin() + 0.32 * (2.0 * phase).sin() + 0.16 * (3.0 * phase).sin();
        lp += 0.25 * (rng.next() - lp);
        let env = (t * 18.0).min(1.0) * ((1.0 - u) * 8.0).exp();
        *s = (raw * 0.65 + lp * 0.25) * env * amp;
    }
    fade_in_out(&mut out, sr);
    out
}

fn double_clic(duration: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    for (k, &freq) in [880.0, 1320.0].iter().enumerate() {
        let start = (sr * 0.045 * k as f32) as usize;
        for (i, item) in out.iter_mut().enumerate().take(n).skip(start) {
            let t = (i - start) as f32 / sr;
            let env = (-t * 45.0).exp();
            let p = std::f32::consts::TAU * freq * t;
            *item += p.sin() * env * 0.40;
        }
    }
    fade_in_out(&mut out, sr);
    out
}

fn taille_silex(duration: f32, sr: f32) -> Vec<f32> {
    let n = len(sr, duration);
    let mut out = vec![0.0; n];
    let mut rng = Rng::new(0x51F0_0006);
    let mut hp = 0.0f32;
    let mut prev = 0.0f32;
    for (i, s) in out.iter_mut().enumerate() {
        let t = i as f32 / sr;
        let raw = rng.next();
        hp = 0.7 * (hp + raw - prev);
        prev = raw;
        let mut v = hp * 0.10;
        for k in 0..3 {
            let dt = t - 0.035 * k as f32;
            if dt >= 0.0 {
                v += (std::f32::consts::TAU * (1700.0 + 310.0 * k as f32) * dt).sin()
                    * (-dt * 90.0).exp()
                    * 0.45;
            }
        }
        *s = v * 0.75;
    }
    fade_in_out(&mut out, sr);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tous_les_bruits_sont_valides() {
        let sons = generate_all(44_100.0);
        for (i, son) in sons.iter().enumerate() {
            assert!(!son.is_empty(), "bruit {i} vide");
            assert!(son.len() < 44_100, "bruit {i} trop long");
            let mut peak = 0.0f32;
            for &v in son.iter() {
                assert!(v.is_finite(), "bruit {i} non fini");
                peak = peak.max(v.abs());
            }
            assert!(peak > 0.01, "bruit {i} silencieux");
            assert!(peak <= 1.0, "bruit {i} sature : {peak}");
        }
    }
}
