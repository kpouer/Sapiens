#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
use crate::langue::Langue::{ES, FR};
use crate::langue::{langue, langue_i16};
use crate::sapiens::sapglob::Sapglob;
use crate::sapiens::saptxt_es::Saptxt_es;
use crate::sapiens::saptxt_fr::Saptxt_fr;
use crate::sapiens::saptxt_gb::Saptxt_gb;
use crate::sapiens::{TypeReference, reference};
use std::ptr::addr_of;

const DELTA_LANGUE: i16 = 1000;

const C_POURCENT: i16 = b'%' as i16;
const C_ET: i16 = b'&' as i16;
const C_DOLLAR: i16 = b'$' as i16;
const C_DIESE: i16 = b'#' as i16;
const C_CROCHET_G: i16 = b'[' as i16;
const C_CROCHET_D: i16 = b']' as i16;
const C_CHAPEAU: i16 = b'^' as i16;
const C_BARRE: i16 = b'|' as i16;
const C_TRAIT_BAS: i16 = b'_' as i16;

pub(crate) struct Saptxt {
    pub(crate) nomjoueur: [i8; 9],
    pub(crate) nomcopain: [i8; 7],
    pub(crate) nomadversaire: [i8; 255],
    pub(crate) nomtribu: [i8; 255],
    pub(crate) nomsecondaire: [i8; 255],
    pub(crate) poschef: [i8; 4],
    pub(crate) maced: [i8; 52],
    #[allow(dead_code)]
    pub(crate) traps: [*mut i8; 7],
}

impl Default for Saptxt {
    fn default() -> Self {
        Self {
            nomjoueur: [0x54, 0x6F, 0x72, 0x67, 0x61, 0x6E, 0, 0, 0],
            nomcopain: [0x47, 0x41, 0x42, 0x55, 0x2E, 0x2E, 0],
            nomadversaire: [0; 255],
            nomtribu: [0; 255],
            nomsecondaire: [0; 255],
            poschef: [0x2E, 0x20, 0x2E, 0],
            maced: [
                0x48, 0x41, 0x52, 0x47, 0x49, 0x4F, 0x4E, 0x47, 0x49, 0x4B, 0x52, 0x55, 0x4E, 0x48,
                0x4F, 0x47, 0x52, 0x4F, 0x4B, 0x48, 0x4F, 0x55, 0x4C, 0x41, 0x4E, 0x49, 0x52, 0x4B,
                0x48, 0x55, 0x47, 0x4F, 0x54, 0x48, 0x4F, 0x52, 0x4E, 0x49, 0x55, 0x50, 0x41, 0x49,
                0x53, 0x4E, 0x49, 0x50, 0x48, 0x59, 0x47, 0x4C, 0x4F, 0x00,
            ],
            traps: [
                c"CHAUVET".as_ptr() as *mut i8,
                c"GROTTE".as_ptr() as *mut i8,
                c"MUSICS".as_ptr() as *mut i8,
                c"DETAIL".as_ptr() as *mut i8,
                c"AUROCH".as_ptr() as *mut i8,
                c"MYRIAD".as_ptr() as *mut i8,
                c"CAT   ".as_ptr() as *mut i8,
            ],
        }
    }
}

/**********************************************************************
                Textes des trois langues
Les 24 tables (8 par langue) sont portees en Rust dans
saptxt_fr.rs / saptxt_gb.rs / saptxt_es.rs, anciens SAPTXT_xx.C.
Elles ne sont accessibles que par les GetRefer* / Get* ci-dessous,
qui choisissent la table selon `langue`.
***********************************************************************/

/* Remplit le char nomref[64] de `struct reference` depuis une chaine
MacRoman, exactement comme le fait le C : zéro jusqu'au bout. */
pub(crate) const fn nomref(s: &[u8]) -> [i8; 64] {
    let mut out = [0; 64];
    let mut i = 0;
    while i < s.len() && i < 64 {
        out[i] = s[i] as i8;
        i += 1;
    }
    out
}

/* Chaine MacRoman statique : equivalent du litteral C "..." dans un tableau
`char *`. Un litteral c"..." est inutilisable ici, il refuse les octets
MacRoman hauts (\x82, \x9C...).

Le litteral DOIT garder son \0 final : en C il fait partie de la donnee, et
le code fait strlen() dessus. Sans lui la lecture deborderait sur la suite
du .rodata. Pour une chaine vide, ecrire b"\0" et surtout pas b"" : as_ptr()
rendrait 0x1, un pointeur pendant, et le dereferencement planterait. */
pub(crate) const fn sc(s: &[u8]) -> *mut i8 {
    s.as_ptr() as *mut i8
}

#[derive(Default)]
pub(crate) struct SapLang {
    saptxt_fr: Saptxt_fr,
    saptxt_gb: Saptxt_gb,
    saptxt_es: Saptxt_es,
}

impl SapLang {
    pub(crate) fn GetRefer0(&self) -> *mut reference {
        if langue() == FR {
            addr_of!(self.saptxt_fr.refer0) as *mut reference
        } else if langue() == ES {
            addr_of!(self.saptxt_es.refer0) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer0) as *mut reference
        }
    }

    pub(crate) fn GetRefer1(&self) -> *mut reference {
        if langue() == FR {
            addr_of!(self.saptxt_fr.refer1) as *mut reference
        } else if langue() == ES {
            addr_of!(self.saptxt_es.refer1) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer1) as *mut reference
        }
    }

    pub(crate) fn GetRefer2(&self) -> *mut reference {
        if langue() == FR {
            addr_of!(self.saptxt_fr.refer2) as *mut reference
        } else if langue() == ES {
            addr_of!(self.saptxt_es.refer2) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer2) as *mut reference
        }
    }

    pub(crate) fn GetRefer3(&self) -> *mut reference {
        if langue() == FR {
            addr_of!(self.saptxt_fr.refer3) as *mut reference
        } else if langue() == ES {
            addr_of!(self.saptxt_es.refer3) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer3) as *mut reference
        }
    }
    pub(crate) fn GetRefer(&self, t: TypeReference) -> *mut reference {
        match t {
            TypeReference::REFER0 => self.GetRefer0(),
            TypeReference::REFER1 => self.GetRefer1(),
            TypeReference::REFER2 => self.GetRefer2(),
            TypeReference::REFER3 => self.GetRefer3(),
        }
    }

    pub(crate) fn GetMesCon(&self) -> &[*mut i8] {
        if langue() == FR {
            &self.saptxt_fr.mescon
        } else if langue() == ES {
            &self.saptxt_es.mescon
        } else {
            &self.saptxt_gb.mescon
        }
    }

    pub(crate) fn GetApp(&self) -> &[*mut i8] {
        if langue() == FR {
            &self.saptxt_fr.app
        } else if langue() == ES {
            &self.saptxt_es.app
        } else {
            &self.saptxt_gb.app
        }
    }

    pub(crate) fn GetBkMot(&self) -> *mut *mut i8 {
        if langue() == FR {
            addr_of!(self.saptxt_fr.bkmots) as *mut *mut i8
        } else if langue() == ES {
            addr_of!(self.saptxt_es.bkmots) as *mut *mut i8
        } else {
            addr_of!(self.saptxt_gb.bkmots) as *mut *mut i8
        }
    }

    pub(crate) fn GetScenme(&self) -> *mut *mut i8 {
        if langue() == FR {
            addr_of!(self.saptxt_fr.scenme) as *mut *mut i8
        } else if langue() == ES {
            addr_of!(self.saptxt_es.scenme) as *mut *mut i8
        } else {
            addr_of!(self.saptxt_gb.scenme) as *mut *mut i8
        }
    }
}

// Convertit un numero de ressource en numero de ressource langue pour la langue donnee
pub(crate) const fn CvtLangRess(l: i16, id: i16) -> i16 {
    (id as i32 + DELTA_LANGUE as i32 * l as i32) as i16
}

// Convertit un numero de ressource en numero de ressource langue pour la langue defaut
pub(crate) fn CvtDefaultLangRess(id: i16) -> i16 {
    CvtLangRess(langue_i16(), id)
}

impl Sapglob {
    // Choisit les touches « homme », « femme », « oui » selon la langue courante.
    pub(crate) fn init_and_check_texts(&mut self) {
        match langue() {
            FR => {
                self.homme = b'H';
                self.femme = b'F';
                self.oui = b'O';
            }
            ES => {
                self.homme = b'H';
                self.femme = b'M';
                self.oui = b'S';
            }
            _ => {
                self.homme = b'M';
                self.femme = b'F';
                self.oui = b'Y';
            }
        }
    }
}

/*************************************************************************
                    CvtCar
Conversion d'un caractere special en caractere encode Windows selon la langue
**************************************************************************/
pub(crate) fn CvtCar(car: i16) -> i16 {
    match car {
        C_POURCENT => 0x8a, /* ê */
        C_ET => 0x82,       /* é */
        C_DOLLAR => 0x85,   /* à */
        C_DIESE => 0x87,    /* ç */
        _ => car,
    }
}

/*************************************************************************
                    CvtCarLissage
Conversion d'un caractere special en caractere encode mac selon la langue
pour le lissage
**************************************************************************/
pub(crate) fn CvtCarLissage(car: i16) -> i16 {
    // Conversion
    if langue() == FR {
        match car {
            C_POURCENT => 0x90, /* ê */
            C_ET => 0x8E,       /* é */
            C_DOLLAR => 0x88,   /* à */
            C_DIESE => 0x8D,    /* ç */
            _ => car,
        }
    } else if langue() == ES {
        match car {
            C_CROCHET_G => 0xC0, /* Point d'interrogation inverse */
            C_CROCHET_D => 0xC1, /* Point d'excalamtion inverse */
            C_POURCENT => 0x97,  /* O accent grave */
            C_CHAPEAU => 0x96,   /* n tilde */
            C_ET => 0x87,        /* a accent grave */
            C_BARRE => 0x92,     /* i accent grave */
            C_DIESE => 0x8E,     /* e accent aigu */
            C_TRAIT_BAS => 0x9C, /* u accent grave */
            _ => car,
        }
    } else {
        car
    }
}
