use crate::Sapiens;
use crate::cbridge::Ecran;
use crate::sapiens::sapwbrt::{GetIndexPixBrt, PlotIndexPixBrt};
use std::os::raw::c_short;

impl Sapiens {
    /**********************************************************************
                        lit_sil
    Lit un point couleur dans le silex
    => ligne,colonne ,couleur
    ***********************************************************************/
    pub(crate) extern "C" fn lit_sil(&self, y: c_short, x: c_short) -> c_short {
        unsafe { (GetIndexPixBrt(Ecran, x + 53, y) != *self.sapglob.palhaute.add(16)) as c_short }
    }

    /**********************************************************************
                        ecrit_sil
    Ecrit un point couleur dans le silex
    => ligne (0-200) ,colonne (0-320),couleur
    ***********************************************************************/
    pub(crate) extern "C" fn ecrit_sil(&self, y: c_short, x: c_short, couleur: c_short) {
        unsafe {
            PlotIndexPixBrt(
                Ecran,
                x + 53,
                y,
                *self.sapglob.palhaute.add((16 + couleur) as usize),
            )
        }
    }
}
