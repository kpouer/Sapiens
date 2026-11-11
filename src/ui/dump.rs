use crate::exchange::WindowFrame;
use crate::shared::lut_rgb;
use std::time::Instant;

/// Débogage : si `SAPIENS_DUMP` est défini, écrit régulièrement le tampon de la
/// fenêtre sous forme d'image PPM (lisible ensuite avec `sips`/`ffmpeg`).
pub(super) struct PortDumper {
    out: String,
    then: Instant,
}

impl PortDumper {
    pub(super) fn from_env() -> Option<Self> {
        std::env::var("SAPIENS_DUMP").ok().map(|out| PortDumper {
            out,
            then: Instant::now(),
        })
    }

    pub(super) fn tick(&mut self, f: &WindowFrame) {
        const INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);
        if self.then.elapsed() < INTERVAL {
            return;
        }
        self.then = Instant::now();
        let (w, h) = (f.w, f.h);
        let mut ppm = Vec::with_capacity(12 + w * h * 3);
        ppm.extend_from_slice(format!("P6\n{w} {h}\n255\n").as_bytes());
        for &idx in &f.pixels {
            let (r, g, b) = lut_rgb(idx);
            ppm.extend_from_slice(&[r, g, b]);
        }
        let path = self.out.clone();
        match std::fs::write(&path, &ppm) {
            Ok(()) => eprintln!("dump -> {path} ({w}x{h})"),
            Err(e) => eprintln!("dump échec {path}: {e}"),
        }
    }
}
