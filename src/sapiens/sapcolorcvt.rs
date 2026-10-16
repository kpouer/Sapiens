/*************************************************************
                SapColorCvt.c
Conversion de couleurs
Ajoute le 24/05/05 pour Windows pour compenser l'imprecision de
la conversion HSL sur la palette standard 8 bits.
Repasse en comparaison simple precedente sur le delta rvb
*****************************************************************/
use crate::acam::Color2Index;
use crate::shared::RGBColor;

pub(crate) fn SapColor2Index(rgb: &RGBColor) -> i16 {
    Color2Index(rgb)
}
