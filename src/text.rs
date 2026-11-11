//! Moteur de texte QuickDraw : implémente les fonctions de dessin de texte
//! (TextFont, TextSize, TextFace, DrawChar, DrawString, StringWidth,
//! CharWidth, TextBox, GetFontInfo, GetStdFont) en traçant dans le tampon
//! 8 bits du port courant.
//!
//! Les polices sont des TTF embarquées (Liberation Sans / Serif / Mono, licence
//! SIL OFL 1.1), métric-compatibles Helvetica / Times / Courier du Mac
//! d'origine. Aucun accès aux polices système : le module est portable
//! (y compris wasm32). Les octets >= 0x80 des chaînes sont interprétés en
//! MacRoman (le code C convertit déjà ses placeholders via `CvtCarLissage`).

#![allow(dead_code)]
#![allow(non_snake_case)]

use ab_glyph::{Font as _, FontRef, GlyphId, PxScale, ScaleFont};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use crate::acam::plot;
use crate::model::Rect;
use crate::port::Ports;
use crate::shared::*;

// ---------------------------------------------------------------------------
// Polices embarquées (data/fonts/)
// ---------------------------------------------------------------------------

const SANS_REGULAR: &[u8] = include_bytes!("../data/fonts/LiberationSans-Regular.ttf");
const SANS_BOLD: &[u8] = include_bytes!("../data/fonts/LiberationSans-Bold.ttf");
const SERIF_REGULAR: &[u8] = include_bytes!("../data/fonts/LiberationSerif-Regular.ttf");
const SERIF_BOLD: &[u8] = include_bytes!("../data/fonts/LiberationSerif-Bold.ttf");
const MONO_REGULAR: &[u8] = include_bytes!("../data/fonts/LiberationMono-Regular.ttf");
const MONO_BOLD: &[u8] = include_bytes!("../data/fonts/LiberationMono-Bold.ttf");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Fam {
    Serif,
    Sans,
    Mono,
}

#[inline]
fn fdt_bytes(fam: Fam, bold: bool) -> &'static [u8] {
    match (fam, bold) {
        (Fam::Sans, false) => SANS_REGULAR,
        (Fam::Sans, true) => SANS_BOLD,
        (Fam::Serif, false) => SERIF_REGULAR,
        (Fam::Serif, true) => SERIF_BOLD,
        (Fam::Mono, false) => MONO_REGULAR,
        (Fam::Mono, true) => MONO_BOLD,
    }
}

#[inline]
fn fdt_idx(fam: Fam, bold: bool) -> usize {
    (match fam {
        Fam::Serif => 0,
        Fam::Sans => 1,
        Fam::Mono => 2,
    }) * 2
        + bold as usize
}

/// Polices analysées une seule fois (6 combinaisons famille × graisse).
static FONTS: LazyLock<Mutex<Vec<Option<Arc<FontRef<'static>>>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

fn cached_font(fam: Fam, bold: bool) -> Arc<FontRef<'static>> {
    let mut v = FONTS.lock().unwrap();
    if v.is_empty() {
        v.resize(6, None);
    }
    let idx = fdt_idx(fam, bold);
    if v[idx].is_none() {
        let f = Arc::new(
            FontRef::try_from_slice(fdt_bytes(fam, bold))
                .expect("police Liberation embarquée invalide"),
        );
        v[idx] = Some(f);
    }
    Arc::clone(v[idx].as_ref().unwrap())
}

// ---------------------------------------------------------------------------
// Correspondance des familles QuickDraw
// ---------------------------------------------------------------------------

/// Correspondance des numéros de police QuickDraw vers les familles embarquées :
/// `times`/`geneva` (0) → Serif, `courier` (1) → Mono, `helvetica` (2) → Sans.
#[inline]
fn fdt_family(font: i16) -> Fam {
    match font {
        1 => Fam::Mono,
        2 => Fam::Sans,
        _ => Fam::Serif,
    }
}

/// Taille effective en pixels (QuickDraw : négatif = pixels, 0 = défaut).
#[inline]
fn fdt_size(size: i16) -> f32 {
    let s = if size < 0 {
        -size
    } else if size == 0 {
        12
    } else {
        size
    };
    (s as f32).clamp(4.0, 400.0)
}

fn scale_for(size: i16) -> PxScale {
    let s = fdt_size(size);
    PxScale { x: s, y: s }
}

// ---------------------------------------------------------------------------
// MacRoman → Unicode
// ---------------------------------------------------------------------------

const MAC_ROMAN: [char; 128] = mac_roman_table();

const fn mac_roman_table() -> [char; 128] {
    // Codes 0x80..0xFF de l'encodage Mac OS Roman.
    let mut t = ['\u{0}'; 128];
    let mut i = 0;
    while i < 128 {
        t[i] = mac_roman_char(0x80 + i as u8);
        i += 1;
    }
    t
}

const fn mac_roman_char(b: u8) -> char {
    // NB: pas de boucle possible ici, table explicite pour les octets utiles.
    match b {
        0x80 => '\u{00C4}', // Ä
        0x81 => '\u{00C5}', // Å
        0x82 => '\u{00C7}', // Ç
        0x83 => '\u{00C9}', // É
        0x84 => '\u{00D1}', // Ñ
        0x85 => '\u{00D6}', // Ö
        0x86 => '\u{00DC}', // Ü
        0x87 => '\u{00E1}', // á
        0x88 => '\u{00E0}', // à
        0x89 => '\u{00E2}', // â
        0x8A => '\u{00E4}', // ä
        0x8B => '\u{00E3}', // ã
        0x8C => '\u{00E5}', // å
        0x8D => '\u{00E7}', // ç
        0x8E => '\u{00E9}', // é
        0x8F => '\u{00E8}', // è
        0x90 => '\u{00EA}', // ê
        0x91 => '\u{00EB}', // ë
        0x92 => '\u{00ED}', // í
        0x93 => '\u{00EC}', // ì
        0x94 => '\u{00EE}', // î
        0x95 => '\u{00EF}', // ï
        0x96 => '\u{00F1}', // ñ
        0x97 => '\u{00F3}', // ó
        0x98 => '\u{00F2}', // ò
        0x99 => '\u{00F4}', // ô
        0x9A => '\u{00F6}', // ö
        0x9B => '\u{00F5}', // õ
        0x9C => '\u{00FA}', // ú
        0x9D => '\u{00F9}', // ù
        0x9E => '\u{00FB}', // û
        0x9F => '\u{00FC}', // ü
        0xA0 => '\u{2020}',
        0xA1 => '\u{00B0}', // °
        0xA2 => '\u{00A2}', // ¢
        0xA3 => '\u{00A3}', // £
        0xA4 => '\u{00A7}', // §
        0xA5 => '\u{2022}', // •
        0xA6 => '\u{00B6}', // ¶
        0xA7 => '\u{00DF}', // ß
        0xA8 => '\u{00AE}', // ®
        0xA9 => '\u{00A9}', // ©
        0xAA => '\u{2122}', // ™
        0xAB => '\u{00B4}', // ´
        0xAC => '\u{00A8}', // ¨
        0xAD => '\u{2260}',
        0xAE => '\u{00C6}', // Æ
        0xAF => '\u{00D8}', // Ø
        0xB0 => '\u{221E}',
        0xB1 => '\u{00B1}', // ±
        0xB2 => '\u{2264}',
        0xB3 => '\u{2265}',
        0xB4 => '\u{00A5}', // ¥
        0xB5 => '\u{00B5}', // µ
        0xB6 => '\u{2202}',
        0xB7 => '\u{2211}',
        0xB8 => '\u{220F}',
        0xB9 => '\u{03C0}', // π
        0xBA => '\u{222B}',
        0xBB => '\u{00AA}', // ª
        0xBC => '\u{00BA}', // º
        0xBD => '\u{03A9}', // Ω
        0xBE => '\u{00E6}', // æ
        0xBF => '\u{00F8}', // ø
        0xC0 => '\u{00BF}', // ¿
        0xC1 => '\u{00A1}', // ¡
        0xC2 => '\u{00AC}', // ¬
        0xC3 => '\u{221A}',
        0xC4 => '\u{0192}', // ƒ
        0xC5 => '\u{2248}',
        0xC6 => '\u{2206}',
        0xC7 => '\u{00AB}', // «
        0xC8 => '\u{00BB}', // »
        0xC9 => '\u{2026}', // …
        0xCA => '\u{00A0}', // insécable
        0xCB => '\u{00C0}', // À
        0xCC => '\u{00C3}', // Ã
        0xCD => '\u{00D5}', // Õ
        0xCE => '\u{0152}', // Œ
        0xCF => '\u{0153}', // œ
        0xD0 => '\u{2013}', // –
        0xD1 => '\u{2014}', // —
        0xD2 => '\u{201C}', // “
        0xD3 => '\u{201D}', // ”
        0xD4 => '\u{2018}', // ‘
        0xD5 => '\u{2019}', // ’
        0xD6 => '\u{00F7}', // ÷
        0xD7 => '\u{25CA}',
        0xD8 => '\u{00FF}', // ÿ
        0xD9 => '\u{0178}', // Ÿ
        0xDA => '\u{2044}',
        0xDB => '\u{20AC}', // €
        0xDC => '\u{2039}', // ‹
        0xDD => '\u{203A}', // ›
        0xDE => '\u{FB01}', // ﬁ
        0xDF => '\u{FB02}', // ﬂ
        0xE0 => '\u{2021}', // ‡
        0xE1 => '\u{00B7}', // ·
        0xE2 => '\u{201A}', // ‚
        0xE3 => '\u{201E}', // „
        0xE4 => '\u{2030}', // ‰
        0xE5 => '\u{00C2}', // Â
        0xE6 => '\u{00CA}', // Ê
        0xE7 => '\u{00C1}', // Á
        0xE8 => '\u{00CB}', // Ë
        0xE9 => '\u{00C8}', // È
        0xEA => '\u{00CD}', // Í
        0xEB => '\u{00CE}', // Î
        0xEC => '\u{00CF}', // Ï
        0xED => '\u{00CC}', // Ì
        0xEE => '\u{00D3}', // Ó
        0xEF => '\u{00D4}', // Ô
        0xF0 => '\u{F8FF}', // logo Apple
        0xF1 => '\u{00D2}', // Ò
        0xF2 => '\u{00DA}', // Ú
        0xF3 => '\u{00DB}', // Û
        0xF4 => '\u{00D9}', // Ù
        0xF5 => '\u{0131}', // ı
        0xF6 => '\u{02C6}', // ˆ
        0xF7 => '\u{02DC}', // ˜
        0xF8 => '\u{00AF}', // ¯
        0xF9 => '\u{02D8}',
        0xFA => '\u{02D9}',
        0xFB => '\u{02DA}',
        0xFC => '\u{00B8}', // ¸
        0xFD => '\u{02DD}',
        0xFE => '\u{02DB}',
        0xFF => '\u{02C7}',
        _ => '?',
    }
}

#[inline]
fn decode(b: u8) -> char {
    if b < 0x80 {
        char::from(b)
    } else {
        MAC_ROMAN[(b - 0x80) as usize]
    }
}

/// Renvoie `c` s'il est représentable en MacRoman, `'?'` sinon.
#[inline]
fn to_mac_char(c: char) -> char {
    if c.is_ascii() || MAC_ROMAN.contains(&c) {
        c
    } else {
        '?'
    }
}

/// Convertit un caractère Unicode en octet MacRoman. `None` si le caractère
/// n'existe pas dans l'encodage historique (utilisé pour la saisie clavier).
pub(crate) fn unicode_to_mac_roman(c: char) -> Option<u8> {
    if c.is_ascii() {
        return Some(c as u8);
    }
    MAC_ROMAN
        .iter()
        .position(|&m| m == c)
        .map(|i| 0x80 + i as u8)
}

// ---------------------------------------------------------------------------
// Métriques de police (cache par famille × graisse × taille)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct FMetrics {
    ascent: f32,
    descent: f32,
    leading: f32,
    widening: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct FKey {
    fam: Fam,
    bold: bool,
    size: i16,
}

static METRIC: LazyLock<Mutex<HashMap<FKey, FMetrics>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn font_metrics(fam: Fam, bold: bool, size: i16) -> FMetrics {
    let key = FKey { fam, bold, size };
    if let Some(m) = METRIC.lock().unwrap().get(&key).copied() {
        return m;
    }
    let font = cached_font(fam, bold);
    let scale = scale_for(size);
    let scaled = font.as_scaled(scale);
    let mut m = FMetrics {
        ascent: scaled.ascent(),
        descent: -scaled.descent(),
        leading: scaled.line_gap(),
        widening: 0.0,
    };
    for b in 0x20u8..=0x7E {
        let gid = font.glyph_id(char::from(b));
        let aw = scaled.h_advance(gid);
        if aw > m.widening {
            m.widening = aw;
        }
    }
    METRIC.lock().unwrap().insert(key, m);
    m
}

// ---------------------------------------------------------------------------
// Rastérisation des glyphes (cache bitmap couverture)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GKey {
    fam: Fam,
    bold: bool,
    size: i16,
    ch: char,
}

struct GlyphDraw {
    advance: i16,
    ox: i32,
    oy: i32,
    w: i32,
    h: i32,
    px: Vec<u8>,
}

static GLYPHS: LazyLock<Mutex<HashMap<GKey, Arc<GlyphDraw>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn get_glyph(fam: Fam, bold: bool, size: i16, ch: char) -> Arc<GlyphDraw> {
    let key = GKey {
        fam,
        bold,
        size,
        ch,
    };
    if let Some(g) = GLYPHS.lock().unwrap().get(&key).cloned() {
        return g;
    }
    let g = Arc::new(rasterize(fam, bold, size, ch));
    let mut map = GLYPHS.lock().unwrap();
    if map.len() > 8192 {
        map.clear();
    }
    map.insert(key, g.clone());
    g
}

fn rasterize(fam: Fam, bold: bool, size: i16, ch: char) -> GlyphDraw {
    let font = cached_font(fam, bold);
    let scale = scale_for(size);
    let gid = font.glyph_id(ch);
    let advance = font.as_scaled(scale).h_advance(gid);
    // Caractère absent de la police (le glyphe 0 est obligatoirement `.notdef`) :
    // on le saute (aucun tracé).
    if gid == GlyphId(0) {
        return GlyphDraw {
            advance: advance.round() as i16,
            ox: 0,
            oy: 0,
            w: 0,
            h: 0,
            px: Vec::new(),
        };
    }
    match font.outline_glyph(gid.with_scale(scale)) {
        Some(og) => {
            let b = og.px_bounds();
            let ox = b.min.x.floor() as i32;
            let oy = b.min.y.floor() as i32;
            let x1 = b.max.x.ceil() as i32;
            let y1 = b.max.y.ceil() as i32;
            let w = (x1 - ox).max(0);
            let h = (y1 - oy).max(0);
            let mut px = vec![0u8; (w as usize).saturating_mul(h as usize)];
            let wu = w as u32;
            let hu = h as u32;
            og.draw(|x, y, cov| {
                // `x`/`y` sont déjà relatifs au coin `px_bounds.min`.
                if x < wu && y < hu {
                    let a = (cov * 255.0).round().clamp(0.0, 255.0) as u8;
                    px[(y as usize) * wu as usize + x as usize] = a;
                }
            });
            GlyphDraw {
                advance: advance.round() as i16,
                ox,
                oy,
                w,
                h,
                px,
            }
        }
        None => GlyphDraw {
            advance: advance.round() as i16,
            ox: 0,
            oy: 0,
            w: 0,
            h: 0,
            px: Vec::new(),
        },
    }
}

/// Trace un glyphe dont la baseline est en (bx, by), pixel par pixel (clip et
/// pen_mode gérés par `plot`). La couverture est seuillée pour rester net.
fn draw_glyph(p: &mut SapPort, bx: i16, by: i16, g: &GlyphDraw) {
    if g.w <= 0 || g.h <= 0 {
        return;
    }
    let col = p.fore_index;
    let basex = bx as i32 + g.ox;
    let basey = by as i32 + g.oy;
    let w = g.w as usize;
    for (row, line) in g.px.chunks_exact(w).enumerate() {
        for (colp, &a) in line.iter().enumerate() {
            if a > 127 {
                plot(
                    p,
                    (basex + colp as i32) as i16,
                    (basey + row as i32) as i16,
                    col,
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// État texte du port courant
// ---------------------------------------------------------------------------

#[inline]
fn fdt_state(p: &SapPort) -> (Fam, bool, i16) {
    let fam = fdt_family(p.tx_font);
    let bold = p.tx_face & 1 != 0;
    let size = p.tx_size;
    (fam, bold, size)
}

// ---------------------------------------------------------------------------
// API QuickDraw
// ---------------------------------------------------------------------------
impl Ports {
    pub(crate) fn TextFont(&mut self, font: i16) {
        let p = self.current_mut();
        p.tx_font = font;
    }

    pub(crate) fn TextSize(&mut self, size: i16) {
        let p = self.current_mut();
        p.tx_size = size;
    }

    pub(crate) fn TextFace(&mut self, face: i16) {
        let p = self.current_mut();
        p.tx_face = face;
    }
}

pub(crate) fn GetStdFont() -> i16 {
    0 // geneva
}

impl Ports {
    pub(crate) fn GetFontInfo(&mut self) -> FontInfo {
        let p = self.current_mut();
        let (fam, bold, size) = fdt_state(p);
        let m = font_metrics(fam, bold, size);
        FontInfo {
            ascent: m.ascent.round() as i16,
            descent: m.descent.round() as i16,
            widening: m.widening.round() as i16,
            leading: m.leading.round() as i16,
        }
    }

    /// Largeur (en pixels) d'un caractère dans la fonte courante du port.
    pub(crate) fn CharWidth(&mut self, c: i8) -> i16 {
        let p = self.current_mut();
        let (fam, bold, size) = fdt_state(p);
        get_glyph(fam, bold, size, decode(c as u8)).advance
    }

    /// Dessine un caractère à la position courante (baseline = pen_loc).
    pub(crate) fn DrawChar(&mut self, c: i8) {
        let p = self.current_mut();
        let (fam, bold, size) = fdt_state(p);
        let g = get_glyph(fam, bold, size, decode(c as u8));
        let loc = p.pen_loc;
        draw_glyph(p, loc.h, loc.v, &g);
    }

    /// Dessine une chaîne (convertie en MacRoman, `?` si non représentable), avance le crayon.
    pub(crate) fn DrawString(&mut self, s: &str) {
        let p = self.current_mut();

        let (fam, bold, size) = fdt_state(p);
        let by = p.pen_loc.v;
        let mut h = p.pen_loc.h;
        for c in s.chars() {
            let g = get_glyph(fam, bold, size, to_mac_char(c));
            draw_glyph(p, h, by, &g);
            h = h.wrapping_add(g.advance);
        }
        p.pen_loc.h = h;
    }

    /// Largeur d'une chaîne (convertie en MacRoman) dans la fonte courante.
    pub(crate) fn StringWidth(&mut self, s: &str) -> i16 {
        let p = self.current();

        let (fam, bold, size) = fdt_state(p);
        let mut w = 0;
        for c in s.chars() {
            w += get_glyph(fam, bold, size, to_mac_char(c)).advance as i32;
        }
        w.clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }

    /// Trace un texte (octets C, longueur explicite) dans un rectangle, justifié.
    /// `just` : 0 = gauche (teJustLeft), 1 = centré (teJustCenter), 2 = droite
    /// (teJustRight). Le texte est centré verticalement dans le rectangle, clippé
    /// par pixel au port (via `plot`).
    pub(crate) fn TextBox(&mut self, text: &str, boxr: &Rect, just: i16) {
        if boxr.is_empty() {
            return;
        }
        let p = self.current_mut();

        let (fam, bold, size) = fdt_state(p);
        // Largeur totale (i32, le texte peut déborder d'un i16).
        let mut total = 0;
        let text = text.as_bytes();
        for &c in text {
            total += get_glyph(fam, bold, size, decode(c)).advance as i32;
        }
        let m = font_metrics(fam, bold, size);
        let fh = (m.ascent + m.descent) as i32;
        let bh = boxr.bottom as i32 - boxr.top as i32;
        let baseline = boxr.top as i32 + (bh - fh).max(0) / 2 + m.ascent.round() as i32;
        let start = match just {
            j if j >= 2 => boxr.right as i32 - total, // droite
            1 => ((boxr.left as i32 + boxr.right as i32) - total) / 2, // centré
            _ => boxr.left as i32,                    // gauche
        };
        let mut acc = 0;
        for &c in text {
            let g = get_glyph(fam, bold, size, decode(c));
            let x = (start + acc) as i16;
            let y = baseline as i16;
            draw_glyph(p, x, y, &g);
            acc += g.advance as i32;
            if start + acc > boxr.right as i32 {
                break;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Point;
    use crate::port::PortId;

    struct TestPort {
        ports: Ports,
    }

    impl TestPort {
        fn new(w: usize, h: usize) -> Self {
            let mut ports = Ports::default();
            ports.mainWindow = SapPort {
                bounds: Rect {
                    top: 0,
                    left: 0,
                    bottom: h as i16,
                    right: w as i16,
                },
                row_bytes: w as i16,
                depth: 8,
                pixels: vec![0u8; w * h],
                pen_loc: Point { v: 0, h: 0 },
                pen_size: Point { v: 1, h: 1 },
                pen_mode: PAT_COPY,
                fore_index: 255,
                back_index: 0,
                clip: Rect {
                    top: 0,
                    left: 0,
                    bottom: h as i16,
                    right: w as i16,
                },
                open_rgn: 0,
                open_acc: Rect::default(),
                tx_font: 0,
                tx_size: 12,
                tx_face: 0,
                zoom: 1.0,
            };
            ports.set(PortId::MainWindow);
            TestPort { ports }
        }

        fn port(&self) -> &SapPort {
            &self.ports.mainWindow
        }

        fn count(&self) -> usize {
            self.port().pixels.iter().filter(|&&v| v != 0).count()
        }
    }

    /// Sérialise les tests (partage du `CURRENT_PORT` global).
    static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn with_lock() -> std::sync::MutexGuard<'static, ()> {
        LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn metrics_et_taille() {
        let _g = with_lock();
        let mut t = TestPort::new(160, 40);
        t.ports.TextFont(2);
        t.ports.TextSize(12);
        t.ports.TextFace(0);
        let info = t.ports.GetFontInfo();
        assert!(info.ascent > 0, "ascent nul : {info:?}");
        assert!(info.descent > 0);
        assert!(info.widening > 0);

        t.ports.TextFace(1);
        let bold = t.ports.GetFontInfo();
        // Le gras reste dans le même ordre de grandeurs en hauteur.
        let di = (bold.ascent - info.ascent).abs();
        assert!(di <= 4, "bold {bold:?} vs regular {info:?} (diff {di})");
    }

    #[test]
    fn largeurs_macro_romain() {
        let _g = with_lock();
        let mut t = TestPort::new(160, 40);
        t.ports.TextFont(2);
        t.ports.TextSize(12);
        t.ports.TextFace(0);
        // 'é' = 0x89 en MacRoman → largeur non nulle.
        assert!(t.ports.CharWidth(0x89u8 as i8) > 0);
        // Caractère absent de l'alphabet (logo Apple 0xF0) : avance réduite du
        // `.notdef` de la police, aucun glyphe tracé.
        let w_apple = t.ports.CharWidth(0xF0u8 as i8);
        assert!(w_apple < 12, "avance .notdef surprenante : {w_apple}");
        let pen = Point { v: 10, h: 10 };
        t.ports.mainWindow.pen_loc = pen;
        t.ports.DrawChar(0xF0u8 as i8);
        assert_eq!(t.count(), 0, "glyphe absent tracé quand même");
    }

    #[test]
    fn dessine_et_progresse() {
        let _g = with_lock();
        let mut t = TestPort::new(160, 40);
        t.ports.mainWindow.pen_loc = Point { v: 15, h: 10 };
        t.ports.TextFont(1);
        t.ports.TextSize(12);
        t.ports.TextFace(1);
        let s = "WXYZ";
        t.ports.DrawString(s);
        let w = t.ports.StringWidth(s);
        assert!(w > 16, "largeur étrange : {w}");
        let n = t.count();
        assert!(n > 60, "trop peu de pixels tracés : {n}");
        // Le crayon a avancé de la largeur totale.
        assert_eq!(t.port().pen_loc.h, 10 + w);
    }

    #[test]
    fn textbox_centre() {
        let _g = with_lock();
        let mut t = TestPort::new(200, 60);
        t.ports.TextFont(2);
        t.ports.TextSize(14);
        t.ports.TextFace(0);
        let text = "THE END";
        let boxr = Rect {
            top: 10,
            left: 20,
            bottom: 50,
            right: 180,
        };
        t.ports.TextBox(text, &boxr, 1); // teJustCenter
        let n = t.count();
        assert!(n > 100, "trop peu de pixels tracés : {n}");
        // Le texte est centré verticalement : présence de pixels dans la
        // moitié basse du rectangle.
        let w = 200usize;
        let mut half = 0;
        let mut all = 0;
        for (i, &v) in t.port().pixels.iter().enumerate() {
            let y = i / w;
            if y >= 30 {
                half += (v != 0) as usize;
            }
            all += (v != 0) as usize;
        }
        assert!(
            half > all / 4,
            "texte pas centré verticalement : {half}/{all}"
        );
    }

    #[test]
    fn conversion_unicode_vers_mac_roman() {
        // ASCII inchangé.
        assert_eq!(unicode_to_mac_roman('A'), Some(0x41));
        assert_eq!(unicode_to_mac_roman(' '), Some(0x20));
        // Accents MacRoman.
        assert_eq!(unicode_to_mac_roman('é'), Some(0x8E));
        assert_eq!(unicode_to_mac_roman('É'), Some(0x83));
        assert_eq!(unicode_to_mac_roman('€'), Some(0xDB));
        // Hors du répertoire historique : la saisie est ignorée.
        assert_eq!(unicode_to_mac_roman('中'), None);
    }
}
