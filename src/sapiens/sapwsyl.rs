use crate::acam::{ForeColor, Line, MoveTo, RGBForeColor};
use crate::cbridge::{Ecran, blackColor};
use crate::sapiens::ECHELLE;
use crate::sapiens::sapglob::palhaute;
use crate::sapiens::sapwbrt::{GetIndexPixBrt, PlotIndexPixBrt};
use crate::shared::RGBColor;
use std::os::raw::c_short;

unsafe extern "C" {
    fn CvtCouleurEnRgb(palette: *mut c_short, index: c_short, rgb: *mut RGBColor);
}

/**********************************************************************
                    lit_sil
Lit un point couleur dans le silex
=> ligne,colonne ,couleur
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn lit_sil(y: c_short, x: c_short) -> c_short {
    unsafe { (GetIndexPixBrt(Ecran, x + 53, y) != *palhaute.add(16)) as c_short }
}

/**********************************************************************
                    ecrit_sil
Ecrit un point couleur dans le silex
=> ligne (0-200) ,colonne (0-320),couleur
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn ecrit_sil(y: c_short, x: c_short, couleur: c_short) {
    unsafe { PlotIndexPixBrt(Ecran, x + 53, y, *palhaute.add((16 + couleur) as usize)) }
}
