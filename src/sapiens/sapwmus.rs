#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::acam::{FAUX, GetCursor};
use crate::cbridge::{SndChannelPtr, initMono, sampledSynth, watchCursor};
use crate::sapiens::TypeMuse_NO_MUSIQUE;
use crate::sapiens::sapglob::okmuse;
use crate::sapiens::sapwbrt::SetPathInData;
use crate::sapiens::sapwind::delai;
use crate::shared::{Boolean, Handle, SndListHandle};
use crate::stubs::{
    ArreteMusique, CarboniseSetCursorArrow, GetResource, IsFinDeMusique, JoueMusiqueHarmony,
    PopPath, PushPath, SetCursor, SndNewChannel, SndPlay,
};
use std::os::raw::{c_char, c_short};
/*************** Variables globales exposees au C (SAPWMUS.C) ***************/
#[unsafe(no_mangle)]
pub static mut quelle_musique: c_short = 1;
#[unsafe(no_mangle)]
pub static mut IsBruitages: Boolean = true;
#[unsafe(no_mangle)]
pub static mut MusiqueEnCours: Boolean = false;
#[unsafe(no_mangle)]
pub static mut TabSons: [Handle; 25] = [core::ptr::null_mut(); 25];
#[unsafe(no_mangle)]
pub static mut canalSnd: SndChannelPtr = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn bruit(num: c_short) {
    unsafe {
        if IsBruitages {
            if canalSnd.is_null() {
                SndNewChannel(
                    &raw mut canalSnd,
                    sampledSynth as i16,
                    (0x20 + initMono) as i16,
                    std::ptr::null_mut(),
                );
            }
            if !TabSons[num as usize].is_null() {
                let type_snd: i64 = u32::from_be_bytes(*b"snd ") as i64; // = 0x736E6420
                TabSons[num as usize] = GetResource(type_snd, num + 1000);
            }
            if !TabSons[num as usize].is_null() {
                if !canalSnd.is_null() {
                    SndPlay(canalSnd, TabSons[num as usize] as SndListHandle, true);
                } else {
                    SndPlay(
                        std::ptr::null_mut(),
                        TabSons[num as usize] as SndListHandle,
                        true,
                    );
                }
            }
            #[cfg(target_os = "windows")]
            if (num == 11 || num == 12 || num == 6) {
                delai(30);
            }
        } else {
            delai(10);
        }
    }
}

/*************************************************************************
                attends_bruit
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn attends_bruit() {}

/*************************************************************************
                quel_synthe
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn quel_synthe(_n: c_short) {}

/*************************************************************************
                reset_synthe
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn reset_synthe() {}

/*************************************************************************
                MusicInit
Lance la musique désirée : 1 musique 1, 2 musique 2
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn MusicInit(quelle: c_short) {
    let nom;

    unsafe {
        if okmuse == TypeMuse_NO_MUSIQUE {
            ArreteMusique(true);
            MusiqueEnCours = FAUX;
            return;
        }
        SetCursor(GetCursor(watchCursor as i16));
        // S300 l'arret de la musique se fait dans SapiensExit
        //if(!IsExitMuse) atexit(ExitMuse);
        //IsExitMuse=VRAI;
        quelle_musique = quelle as c_short;
        PushPath();
        SetPathInData(c"musics".as_ptr() as *mut c_char);
        if quelle == 1 {
            nom = c"Sapiens1.mux".as_ptr() as *mut c_char;
        } else {
            nom = c"Sapiens2.mux".as_ptr() as *mut c_char;
        }
        JoueMusiqueHarmony(
            nom,
            std::ptr::null_mut(),
            c"".as_ptr() as *mut c_char,
            0,
            c"DIGIT_QUALITE".as_ptr() as *mut c_char,
            -1,
            0,
            0,
            false,
        );
        CarboniseSetCursorArrow();
        PopPath();
    }

    // #ifdef OLD_VERSION
    // else
    //     {
    //         if (quelle == 1) {
    //             debmus = muse1;
    //             tabins = tabins1;
    //             init_voie(&voie1, dvoie1, 0);
    //             init_voie(&voie2, dvoie2, 0);
    //             init_voie(&voie3, dvoie3, 0);
    //         } else {
    //             debmus = muse2;
    //             tabins = tabins2;
    //             init_voie(&voie1, dvoie21, 16);
    //             init_voie(&voie2, dvoie22, 16);
    //             init_voie(&voie3, dvoie23, 16);
    //         }
    //         for (i = 0; i < 16; i++) {
    //             InitInstrument(tabins[i]);
    //         }
    //         if (task.tmAddr == 0) {
    //             InstallTask();
    //         }
    //     }
    // #endif
    return;
}

/*************************************************************************
                CheckChangeMusique
En mode player sur librairie on teste la fin de la musique
***************************************************************************/
pub fn CheckChangeMusique() {
    if IsFinDeMusique() {
        unsafe {
            if quelle_musique == 1 {
                MusicInit(2);
            } else if quelle_musique == 2 {
                MusicInit(1);
            }
        }
    }
}
