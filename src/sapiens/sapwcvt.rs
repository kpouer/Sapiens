#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::acam::Index2Color;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapwind::SapWind;
use crate::shared::RGBColor;

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
        let tabcvt = &mut self.TabCvtCoul;
        for r in 0..8u16 {
            for v in 0..8 {
                for b in 0..8 {
                    let rgb = RGBColor {
                        red: r * 9362,
                        green: v * 9362,
                        blue: b * 9362,
                    };
                    tabcvt[((r << 8) + (v << 4) + b) as usize] = SapColor2Index(&rgb) as u8;
                }
            }
        }
        let tabcvtombre = &mut self.TabCvtCoulOmbre;
        for i in 0..256 {
            let mut rgb = Index2Color(i);
            rgb >>= 1;
            tabcvtombre[i as usize] = SapColor2Index(&rgb) as u8;
        }
    }
}

impl RGBColor {
    /*************************************************************************
                    CvtCouleurStEnRgb
    *************************************************************************/
    pub(crate) const fn CvtCouleurStEnRgb(val: i16) -> Self {
        Self {
            red: (((val & 0x700) >> 8) as u16) * 9362,
            green: (((val & 0x070) >> 4) as u16) * 9362,
            blue: ((val & 0x007) as u16) * 9362,
        }
    }
}
