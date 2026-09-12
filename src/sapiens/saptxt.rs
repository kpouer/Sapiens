#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
use crate::langue::Langue::{ES, FR};
use crate::sapiens::sapglob::{FEMME, HOMME, OUI};
use crate::sapiens::sapwind::Collapse;
use crate::sapiens::{TypeReference, reference};
use std::os::raw::{c_char, c_int, c_short};
use std::ptr::{addr_of_mut, null_mut};

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

#[unsafe(no_mangle)]
pub static mut langue: c_short = -1;

#[unsafe(no_mangle)]
pub static mut nomjoueur: [c_char; 9] = [0x54, 0x6F, 0x72, 0x67, 0x61, 0x6E, 0, 0, 0];

#[unsafe(no_mangle)]
pub static mut nomcopain: [c_char; 7] = [0x47, 0x41, 0x42, 0x55, 0x2E, 0x2E, 0];

#[unsafe(no_mangle)]
pub static mut nomadversaire: [c_char; 255] = [0; 255];

#[unsafe(no_mangle)]
pub static mut nomtribu: [c_char; 255] = [0; 255];

#[unsafe(no_mangle)]
pub static mut nomsecondaire: [c_char; 255] = [0; 255];

#[unsafe(no_mangle)]
pub static mut poschef: [c_char; 4] = [0x2E, 0x20, 0x2E, 0];

#[unsafe(no_mangle)]
pub static mut maced: [c_char; 52] = [
    0x48, 0x41, 0x52, 0x47, 0x49, 0x4F, 0x4E, 0x47, 0x49, 0x4B, 0x52, 0x55, 0x4E, 0x48, 0x4F, 0x47,
    0x52, 0x4F, 0x4B, 0x48, 0x4F, 0x55, 0x4C, 0x41, 0x4E, 0x49, 0x52, 0x4B, 0x48, 0x55, 0x47, 0x4F,
    0x54, 0x48, 0x4F, 0x52, 0x4E, 0x49, 0x55, 0x50, 0x41, 0x49, 0x53, 0x4E, 0x49, 0x50, 0x48, 0x59,
    0x47, 0x4C, 0x4F, 0x00,
];

#[unsafe(no_mangle)]
pub static mut traps: [*mut c_char; 7] = [
    c"CHAUVET".as_ptr() as *mut c_char,
    c"GROTTE".as_ptr() as *mut c_char,
    c"MUSICS".as_ptr() as *mut c_char,
    c"DETAIL".as_ptr() as *mut c_char,
    c"AUROCH".as_ptr() as *mut c_char,
    c"MYRIAD".as_ptr() as *mut c_char,
    c"CAT   ".as_ptr() as *mut c_char,
];

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRefer0() -> *mut reference {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::refer0_fr) as *mut reference
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::refer0_es) as *mut reference
    } else {
        addr_of_mut!(super::saptxt_gb::refer0_gb) as *mut reference
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRefer1() -> *mut reference {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::refer1_fr) as *mut reference
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::refer1_es) as *mut reference
    } else {
        addr_of_mut!(super::saptxt_gb::refer1_gb) as *mut reference
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRefer2() -> *mut reference {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::refer2_fr) as *mut reference
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::refer2_es) as *mut reference
    } else {
        addr_of_mut!(super::saptxt_gb::refer2_gb) as *mut reference
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRefer3() -> *mut reference {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::refer3_fr) as *mut reference
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::refer3_es) as *mut reference
    } else {
        addr_of_mut!(super::saptxt_gb::refer3_gb) as *mut reference
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRefer(t: TypeReference) -> *mut reference {
    match t {
        TypeReference_REFER0 => GetRefer0(),
        _TypeReference_REFER1 => GetRefer1(),
        _TypeReference_REFER2 => GetRefer2(),
        _TypeReference_REFER3 => GetRefer3(),
        _ => {
            Collapse(c"GetRefer bad type".as_ptr() as *mut c_char);
            null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetReferLangue(l: c_short, t: TypeReference) -> *mut reference {
    let lbak = langue;
    langue = l;
    let refp = GetRefer(t);
    langue = lbak;
    refp
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMesCon() -> *mut *mut c_char {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::mescon_fr) as *mut *mut c_char
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::mescon_es) as *mut *mut c_char
    } else {
        addr_of_mut!(super::saptxt_gb::mescon_gb) as *mut *mut c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetApp() -> *mut *mut c_char {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::app_fr) as *mut *mut c_char
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::app_es) as *mut *mut c_char
    } else {
        addr_of_mut!(super::saptxt_gb::app_gb) as *mut *mut c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBkMot() -> *mut *mut c_char {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::bkmots_fr) as *mut *mut c_char
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::bkmots_es) as *mut *mut c_char
    } else {
        addr_of_mut!(super::saptxt_gb::bkmots_gb) as *mut *mut c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetScenme() -> *mut *mut c_char {
    if langue == FR as c_short {
        addr_of_mut!(super::saptxt_fr::scenme_fr) as *mut *mut c_char
    } else if langue == ES as c_short {
        addr_of_mut!(super::saptxt_es::scenme_es) as *mut *mut c_char
    } else {
        addr_of_mut!(super::saptxt_gb::scenme_gb) as *mut *mut c_char
    }
}

// Convertit un numero de ressource en numero de ressource langue pour la langue donnee
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CvtLangRess(l: c_short, id: c_short) -> c_short {
    (id as c_int + DELTA_LANGUE as c_int * l as c_int) as c_short
}

// Convertit un numero de ressource en numero de ressource langue pour la langue defaut
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CvtDefaultLangRess(id: c_short) -> c_short {
    CvtLangRess(langue, id)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAndCheckTexts() {
    match langue {
        x if x == FR as c_short => {
            HOMME = b'H';
            FEMME = b'F';
            OUI = b'O';
        }
        x if x == ES as c_short => {
            HOMME = b'H';
            FEMME = b'M';
            OUI = b'S';
        }
        _ => {
            HOMME = b'M';
            FEMME = b'F';
            OUI = b'Y';
        }
    }
}

/*************************************************************************
                    CvtCar
Conversion d'un caractere special en caractere encode Windows selon la langue
**************************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CvtCar(car: c_short) -> c_short {
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
pub unsafe extern "C" fn CvtCarLissage(car: c_short) -> c_short {
    // Conversion
    if langue == FR as c_short {
        match car {
            C_POURCENT => 0x90, /* ê */
            C_ET => 0x8E,       /* é */
            C_DOLLAR => 0x88,   /* à */
            C_DIESE => 0x8D,    /* ç */
            _ => car,
        }
    } else if langue == ES as c_short {
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
