#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::Sapiens;
use crate::langue::Langue::{ES, FR};
use crate::sapiens::sapglob::Sapglob;
use crate::sapiens::{TypeReference, reference};
use std::os::raw::{c_char, c_int, c_short};
use std::ptr::{addr_of, null_mut};
use std::sync::atomic::{AtomicI16, Ordering};

const DELTA_LANGUE: c_short = 1000;

const C_POURCENT: c_short = b'%' as c_short;
const C_ET: c_short = b'&' as c_short;
const C_DOLLAR: c_short = b'$' as c_short;
const C_DIESE: c_short = b'#' as c_short;
const C_CROCHET_G: c_short = b'[' as c_short;
const C_CROCHET_D: c_short = b']' as c_short;
const C_CHAPEAU: c_short = b'^' as c_short;
const C_BARRE: c_short = b'|' as c_short;
const C_TRAIT_BAS: c_short = b'_' as c_short;

/* Etat mutable de SAPTXT.C. `langue` est un global partage entre le thread
graphique egui et le thread jeu (et lu par des fonctions libres). Encapsule
derriere `langue()` / `set_langue()` ; la statique `LANGUE` reste privee.
Le reste est regroupe dans cette structure, champ de `Sapiens`. */
static LANGUE: AtomicI16 = AtomicI16::new(-1);

#[inline]
pub(crate) fn langue() -> c_short {
    LANGUE.load(Ordering::Relaxed)
}

#[inline]
pub(crate) fn set_langue(l: c_short) {
    LANGUE.store(l, Ordering::Relaxed);
}

pub(crate) struct Saptxt {
    pub(crate) nomjoueur: [c_char; 9],
    pub(crate) nomcopain: [c_char; 7],
    pub(crate) nomadversaire: [c_char; 255],
    pub(crate) nomtribu: [c_char; 255],
    pub(crate) nomsecondaire: [c_char; 255],
    pub(crate) poschef: [c_char; 4],
    pub(crate) maced: [c_char; 52],
    #[allow(dead_code)]
    pub(crate) traps: [*mut c_char; 7],
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
                c"CHAUVET".as_ptr() as *mut c_char,
                c"GROTTE".as_ptr() as *mut c_char,
                c"MUSICS".as_ptr() as *mut c_char,
                c"DETAIL".as_ptr() as *mut c_char,
                c"AUROCH".as_ptr() as *mut c_char,
                c"MYRIAD".as_ptr() as *mut c_char,
                c"CAT   ".as_ptr() as *mut c_char,
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
pub(crate) const fn nomref(s: &[u8]) -> [c_char; 64] {
    let mut out = [0 as c_char; 64];
    let mut i = 0;
    while i < s.len() && i < 64 {
        out[i] = s[i] as c_char;
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
pub(crate) const fn sc(s: &[u8]) -> *mut c_char {
    s.as_ptr() as *mut c_char
}

impl Sapiens {
    pub(crate) fn GetRefer0(&self) -> *mut reference {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.refer0) as *mut reference
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.refer0) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer0) as *mut reference
        }
    }

    pub(crate) fn GetRefer1(&self) -> *mut reference {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.refer1) as *mut reference
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.refer1) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer1) as *mut reference
        }
    }

    pub(crate) fn GetRefer2(&self) -> *mut reference {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.refer2) as *mut reference
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.refer2) as *mut reference
        } else {
            addr_of!(self.saptxt_gb.refer2) as *mut reference
        }
    }

    pub(crate) fn GetRefer3(&self) -> *mut reference {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.refer3) as *mut reference
        } else if langue() == ES as c_short {
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

    pub(crate) fn GetMesCon(&self) -> *mut *mut c_char {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.mescon) as *mut *mut c_char
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.mescon) as *mut *mut c_char
        } else {
            addr_of!(self.saptxt_gb.mescon) as *mut *mut c_char
        }
    }

    pub(crate) fn GetApp(&self) -> *mut *mut c_char {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.app) as *mut *mut c_char
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.app) as *mut *mut c_char
        } else {
            addr_of!(self.saptxt_gb.app) as *mut *mut c_char
        }
    }

    pub(crate) fn GetBkMot(&self) -> *mut *mut c_char {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.bkmots) as *mut *mut c_char
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.bkmots) as *mut *mut c_char
        } else {
            addr_of!(self.saptxt_gb.bkmots) as *mut *mut c_char
        }
    }

    pub(crate) fn GetScenme(&self) -> *mut *mut c_char {
        if langue() == FR as c_short {
            addr_of!(self.saptxt_fr.scenme) as *mut *mut c_char
        } else if langue() == ES as c_short {
            addr_of!(self.saptxt_es.scenme) as *mut *mut c_char
        } else {
            addr_of!(self.saptxt_gb.scenme) as *mut *mut c_char
        }
    }
}

// Convertit un numero de ressource en numero de ressource langue pour la langue donnee
pub(crate) const fn CvtLangRess(l: c_short, id: c_short) -> c_short {
    (id as c_int + DELTA_LANGUE as c_int * l as c_int) as c_short
}

// Convertit un numero de ressource en numero de ressource langue pour la langue defaut
pub(crate) fn CvtDefaultLangRess(id: c_short) -> c_short {
    CvtLangRess(langue(), id)
}

impl Sapglob {
    // Choisit les touches « homme », « femme », « oui » selon la langue courante.
    pub(crate) unsafe fn init_and_check_texts(&mut self) {
        match langue() {
            x if x == FR as c_short => {
                self.homme = b'H';
                self.femme = b'F';
                self.oui = b'O';
            }
            x if x == ES as c_short => {
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
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn CvtCar(car: c_short) -> c_short {
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
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn CvtCarLissage(car: c_short) -> c_short {
    // Conversion
    if langue() == FR as c_short {
        match car {
            C_POURCENT => 0x90, /* ê */
            C_ET => 0x8E,       /* é */
            C_DOLLAR => 0x88,   /* à */
            C_DIESE => 0x8D,    /* ç */
            _ => car,
        }
    } else if langue() == ES as c_short {
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
