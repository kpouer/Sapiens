//! Ressources `PICT` (QuickDraw) : décodage et dessin.
//!
//! Seul le sous-ensemble utilisé par Sapiens est géré : PICT version 2 dont
//! l'image est un `PackBitsRect` (opcode 0x98) en 8 bits indexés avec table
//! de couleurs. Les ressources viennent de `c_code/Rsrc/sapiens.rsrc` ; elles
//! sont extraites dans `data/pict/` et embarquées dans l'exécutable.

use crate::port::Ports;
use crate::shared::{Pic, Rect, rgb_to_index};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// Image décodée : un indice de la LUT globale par pixel.
struct Picture {
    w: usize,
    h: usize,
    idx: Vec<u8>,
}

/// Octets bruts de la ressource `PICT` `id`.
fn resource_bytes(id: i16) -> Option<&'static [u8]> {
    match id {
        128 => Some(include_bytes!("../data/pict/pict_128.bin")),
        134 => Some(include_bytes!("../data/pict/pict_134.bin")),
        _ => None,
    }
}

struct Reader<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.b.get(self.p..self.p.checked_add(n)?)?;
        self.p += n;
        Some(s)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        let s = self.take(2)?;
        Some(u16::from_be_bytes([s[0], s[1]]))
    }
    fn i16(&mut self) -> Option<i16> {
        Some(self.u16()? as i16)
    }
    fn rect(&mut self) -> Option<[i16; 4]> {
        Some([self.i16()?, self.i16()?, self.i16()?, self.i16()?])
    }
}

/// Décompresse une ligne PackBits en exactement `len` octets (complétée de 0).
fn unpack_bits(src: &[u8], len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    let mut i = 0;
    while i < src.len() && out.len() < len {
        let f = src[i];
        i += 1;
        if f < 128 {
            let n = f as usize + 1;
            let end = (i + n).min(src.len());
            out.extend_from_slice(&src[i..end]);
            i = end;
        } else if f > 128 {
            let n = 257 - f as usize;
            if let Some(&v) = src.get(i) {
                out.resize(out.len() + n, v);
            }
            i += 1;
        }
    }
    out.resize(len, 0);
    out
}

fn decode(data: &[u8]) -> Option<Picture> {
    let mut r = Reader { b: data, p: 10 }; // picSize (2) + picFrame (8)
    if r.take(4)? != [0x00, 0x11, 0x02, 0xff] {
        return None; // pas une PICT version 2
    }
    loop {
        match r.u16()? {
            0x0c00 => {
                r.take(24)?; // HeaderOp
            }
            0x0001 => {
                let n = r.u16()? as usize; // Clip : taille de la région
                r.take(n.checked_sub(2)?)?;
            }
            0x001e => {} // DefHilite : sans donnée
            0x00a0 => {
                r.take(2)?; // ShortComment
            }
            0x0098 => return decode_packbits_rect(&mut r),
            _ => return None,
        }
    }
}

fn decode_packbits_rect(r: &mut Reader) -> Option<Picture> {
    let row_bytes = (r.u16()? & 0x3fff) as usize;
    let [top, left, bottom, _right] = r.rect()?;
    r.take(2 + 2 + 4 + 4 + 4 + 2)?; // pmVersion .. vRes, pixelType
    let pixel_size = r.u16()?;
    if pixel_size != 8 {
        return None;
    }
    r.take(2 + 2 + 4 + 4 + 4)?; // cmpCount .. pmReserved
    // Table de couleurs : l'indice est la position (le champ « value » est
    // sans importance pour une table de périphérique).
    r.take(4 + 2)?; // ctSeed, ctFlags
    let n = r.i16()? as i32 + 1;
    let mut pal = Vec::with_capacity(n.max(0) as usize);
    for _ in 0..n {
        r.u16()?; // value
        let (rr, gg, bb) = (r.u16()?, r.u16()?, r.u16()?);
        pal.push(rgb_to_index(
            (rr >> 8) as u8,
            (gg >> 8) as u8,
            (bb >> 8) as u8,
        ));
    }
    let [s_top, s_left, s_bottom, s_right] = r.rect()?;
    r.rect()?; // dstRect
    r.u16()?; // mode

    let h_total = (bottom - top) as usize;
    let mut rows: Vec<Vec<u8>> = Vec::with_capacity(h_total);
    for _ in 0..h_total {
        let count = if row_bytes > 250 {
            r.u16()? as usize
        } else {
            r.u8()? as usize
        };
        rows.push(unpack_bits(r.take(count)?, row_bytes));
    }
    // On ne garde que srcRect (les lignes d'arrondi à droite sont écartées).
    let w = (s_right - s_left) as usize;
    let h = (s_bottom - s_top) as usize;
    let x0 = (s_left - left) as usize;
    let y0 = (s_top - top) as usize;
    let mut idx = Vec::with_capacity(w * h);
    for y in 0..h {
        let row = rows.get(y0 + y)?;
        for x in 0..w {
            let c = *row.get(x0 + x)? as usize;
            idx.push(*pal.get(c).unwrap_or(&0));
        }
    }
    Some(Picture { w, h, idx })
}

#[derive(Default)]
pub(crate) struct Pict {
    pictures: HashMap<i16, Pic>,
}

impl Pict {
    #[allow(non_snake_case)]
    pub(crate) fn GetPicture(&mut self, id: i16) -> Option<&Pic> {
        if let Entry::Vacant(e) = self.pictures.entry(id) {
            let pic = resource_bytes(id).and_then(decode)?;
            let frame = Rect::new(0, 0, pic.w as i16, pic.h as i16);
            let pic: &'static Picture = Box::leak(Box::new(pic));
            let master = Pic {
                picSize: 0,
                picFrame: frame,
                pict: pic as *const Picture as usize,
            };
            e.insert(master);
        }
        self.pictures.get(&id)
    }
}

impl Ports {
    #[allow(non_snake_case)]
    /// `DrawPicture` : copie (srcCopy) l'image dans `dst` du port courant, à
    /// l'échelle du rectangle, avec clip.
    pub(crate) fn DrawPicture(&mut self, p: &Pic, dst: &Rect) {
        let pict = p.pict as *const Picture;
        if pict.is_null() {
            return;
        }
        let pict = unsafe { &*pict };
        let dw = (dst.right - dst.left) as i32;
        let dh = (dst.bottom - dst.top) as i32;
        if dw <= 0 || dh <= 0 || pict.w == 0 || pict.h == 0 {
            return;
        }
        let p = self.current_mut();
        let row_bytes = p.row_bytes as usize;
        for dy in 0..dh {
            let y = dst.top as i32 + dy;
            if y < p.clip.top as i32 || y >= p.clip.bottom as i32 {
                continue;
            }
            let sy = (dy as usize * pict.h) / dh as usize;
            let src_row = &pict.idx[sy * pict.w..(sy + 1) * pict.w];
            for dx in 0..dw {
                let x = dst.left as i32 + dx;
                if x < p.clip.left as i32 || x >= p.clip.right as i32 {
                    continue;
                }
                let sx = (dx as usize * pict.w) / dw as usize;
                if let Some(d) = p.pixels.get_mut(y as usize * row_bytes + x as usize) {
                    *d = src_row[sx];
                }
            }
        }
    }
}
