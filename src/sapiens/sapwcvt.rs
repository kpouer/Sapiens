#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::Sapiens;
use crate::acam::Index2Color;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::shared::RGBColor;
use std::os::raw::{c_short, c_uchar};
use std::ptr::addr_of_mut;

pub(crate) struct Sapwcvt {
    pub(crate) TabCvtCoul: [c_uchar; 0x778],
    pub(crate) TabCvtCoulOmbre: [c_uchar; 256],
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
    pub(crate) unsafe extern "C" fn FabriqueLuts(&mut self) {
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
                    *tabcvt.add(((r << 8) + (v << 4) + b) as usize) = SapColor2Index(&rgb) as u8;
                }
            }
        }
        let tabcvtombre = addr_of_mut!(self.TabCvtCoulOmbre) as *mut u8;
        for i in 0..256i16 {
            let mut rgb = RGBColor::default();
            Index2Color(i, &mut rgb);
            rgb.red >>= 1;
            rgb.green >>= 1;
            rgb.blue >>= 1;
            *tabcvtombre.add(i as usize) = SapColor2Index(&rgb) as u8;
        }
    }

    /*************************************************************************
                    CvtCouleurStEnRgb
    *************************************************************************/
    const fn CvtCouleurStEnRgb(val: c_short, rgb: &mut RGBColor) {
        rgb.red = (((val & 0x700) >> 8) as u16) * 9362;
        rgb.green = (((val & 0x070) >> 4) as u16) * 9362;
        rgb.blue = ((val & 0x007) as u16) * 9362;
    }

    /*************************************************************************
                    CvtCouleurEnRgb
    *************************************************************************/
    pub(crate) fn CvtCouleurEnRgb(palette: *mut c_short, index: c_short, rgb: &mut RGBColor) {
        unsafe {
            Self::CvtCouleurStEnRgb(*palette.add(index as usize), rgb);
        }
    }

    /*************************************************************************
                    CvtRgbEnCouleur
    Recherche dans la palette st en entrée l'index de la couleur
    la plus proche en rvb
    *************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn CvtRgbEnCouleur(
        palette: *mut c_short,
        rgb: *mut RGBColor,
    ) -> c_short {
        let mut imin: c_short = 0;
        let mut delmin: u32 = u32::MAX;
        for i in 0..16i16 {
            let mut refx = RGBColor::default();
            Self::CvtCouleurEnRgb(palette, i, &mut refx);
            let delta = ((refx.green as i32 - (*rgb).green as i32).abs()
                + (refx.red as i32 - (*rgb).red as i32).abs()
                + (refx.blue as i32 - (*rgb).blue as i32).abs()) as u32;
            if delta < delmin {
                if delta == 0 {
                    return i;
                }
                delmin = delta;
                imin = i;
            }
        }
        imin
    }
}

impl Sapiens {
    /*************************************************************************
                    CvtIndexEnCouleur
    Convertit un index de palette systeme en index de palette ST
    en utilisant la deuxieme section de la palette
    *************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) unsafe extern "C" fn CvtIndexEnCouleur(
        &self,
        palette: *mut c_short,
        index: c_short,
    ) -> c_short {
        for i in 0..16i16 {
            if *palette.add(i as usize + 16) == index {
                return i;
            }
        }
        self.Collapse("CvtIndexEnCouleur: Index non trouve");
        -1
    }
}
