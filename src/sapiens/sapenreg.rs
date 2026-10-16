#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::Sapiens;
use crate::acam::FAUX;
use crate::cbridge::strlen;
use crate::shared::Boolean;
use log::warn;
use std::ffi::CStr;

const TabCvt1: &[u8; 16] = b"AEUTNZPFSRDGJKLM";
const TabCvt2: &[u8; 16] = b"123456789-XWZTNH";

fn Code(base: u16) -> u16 {
    let mut d0 = base ^ 0x2867;
    let mut d1 = d0;
    d0 = ((d0 & 15) << 12) + (d0 >> 4);
    d0 ^= 6;
    d0 ^= d1;
    d1 = d0;
    d0 = ((d0 & 15) << 12) + (d0 >> 4);
    d0 ^= 9;
    d0 ^= d1;
    d1 = d0;
    d0 = ((d0 & 15) << 12) + (d0 >> 4);
    d0 ^= 11;
    d0 ^= d1;
    d0
}

pub(crate) extern "C" fn VerifNumSerie(str: *mut i8) -> Boolean {
    let str = unsafe { CStr::from_ptr(str).to_bytes() };
    if str.len() < 9 {
        warn!("Invalid serial number length {}", str.len());
        return false;
    }
    let mut clef: u16 = 0;
    let mut base: u16 = 0;

    for i in (0..8usize).step_by(2) {
        let n = str[i];
        let j = match TabCvt1.iter().position(|x| *x == n) {
            None => return false,
            Some(j) => j,
        };
        clef = (clef << 4) + j as u16;
        let n = str[i + 1];
        let j = match TabCvt2.iter().position(|x| *x == n) {
            None => return false,
            Some(j) => j,
        };
        base = (base << 4) + j as u16;
    }
    let n = clef;
    clef = Code(base);
    if clef != n {
        return false;
    }
    if str[8] != 'S' as u8 {
        return false;
    }
    true
}

pub(crate) struct Sapenreg {
    pub(crate) numSerie: [i8; 80],
}

impl Default for Sapenreg {
    fn default() -> Self {
        Self { numSerie: [0; 80] }
    }
}

/**********************************************************************************
                                        IsEnreg
Teste si l'exemplaire est enregistré
<= VRAI si l'exemplaire est enregistré
***********************************************************************************/
impl Sapiens {
    pub(crate) fn IsEnreg(&mut self) -> Boolean {
        /* On extrait le nom et le numero de serie */
        unsafe {
            if strlen(&raw const self.saptxt.nomjoueur as *const _) == 0
                || strlen(&raw const self.sapenreg.numSerie as *const _) == 0
            {
                return FAUX;
            }
            VerifNumSerie(&raw mut self.sapenreg.numSerie as *mut _)
        }
    }
}
