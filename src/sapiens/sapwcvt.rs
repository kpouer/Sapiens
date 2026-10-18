#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::Sapiens;
use crate::acam::Index2Color;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::shared::RGBColor;
use std::ptr::addr_of_mut;

pub(crate) struct Sapwcvt {
    pub(crate) TabCvtCoul: [u8; 0x778],
    pub(crate) TabCvtCoulOmbre: [u8; 256],
}

impl Default for Sapwcvt {
    fn default() -> Self {
        Self {
            TabCvtCoul: [0; 0x778],
            TabCvtCoulOmbre: [0; 256],
        }
    }
}

impl Sapwcvt {
    /*************************************************************************
                    FabriqueLuts
    *************************************************************************/
    pub(crate) fn FabriqueLuts(&mut self) {
        /* Calcul de la LUT st vers index */
        let tabcvt = addr_of_mut!(self.TabCvtCoul) as *mut u8;
        for r in 0..8i16 {
            for v in 0..8i16 {
                for b in 0..8i16 {
                    let rgb = RGBColor {
                        red: (r as u16) * 9362,
                        green: (v as u16) * 9362,
                        blue: (b as u16) * 9362,
                    };
                    unsafe {
                        *tabcvt.add(((r << 8) + (v << 4) + b) as usize) =
                            SapColor2Index(&rgb) as u8;
                    }
                }
            }
        }
        let tabcvtombre = addr_of_mut!(self.TabCvtCoulOmbre) as *mut u8;
        for i in 0..256i16 {
            let mut rgb = Index2Color(i);
            rgb.red >>= 1;
            rgb.green >>= 1;
            rgb.blue >>= 1;
            unsafe {
                *tabcvtombre.add(i as usize) = SapColor2Index(&rgb) as u8;
            }
        }
    }

    /*************************************************************************
                    CvtCouleurStEnRgb
    *************************************************************************/
    const fn CvtCouleurStEnRgb(val: i16, rgb: &mut RGBColor) {
        rgb.red = (((val & 0x700) >> 8) as u16) * 9362;
        rgb.green = (((val & 0x070) >> 4) as u16) * 9362;
        rgb.blue = ((val & 0x007) as u16) * 9362;
    }

    /*************************************************************************
                    CvtCouleurEnRgb
    *************************************************************************/
    pub(crate) fn CvtCouleurEnRgb(palette: *mut i16, index: i16, rgb: &mut RGBColor) {
        unsafe {
            Self::CvtCouleurStEnRgb(*palette.add(index as usize), rgb);
        }
    }
}
