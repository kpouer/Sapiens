#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::Sapiens;
use crate::acam::{FAUX, GetCursor};
use crate::cbridge::{SndChannelPtr, initMono, sampledSynth, watchCursor};
use crate::sapiens::TypeMuse;
use crate::sapiens::sapwbrt::SetPathInData;
use crate::shared::{Boolean, Handle, SndListHandle};
use crate::stubs::{
    ArreteMusique, CarboniseSetCursorArrow, GetResource, IsFinDeMusique, JoueMusiqueHarmony,
    PopPath, PushPath, SetCursor, SndNewChannel, SndPlay,
};

pub(crate) struct Sapwmus {
    /*************** Variables globales exposees au C (SAPWMUS.C) ***************/
    pub(crate) quelle_musique: i16,
    pub(crate) IsBruitages: Boolean,
    MusiqueEnCours: Boolean,
    TabSons: [Handle; 25],
    canalSnd: SndChannelPtr,
}

impl Default for Sapwmus {
    fn default() -> Self {
        Self {
            quelle_musique: 1,
            IsBruitages: true,
            MusiqueEnCours: false,
            TabSons: [core::ptr::null_mut(); 25],
            canalSnd: core::ptr::null_mut(),
        }
    }
}

impl Sapiens {
    pub(crate) fn bruit(&mut self, num: i16) {
        if self.sapwmus.IsBruitages {
            if self.sapwmus.canalSnd.is_null() {
                SndNewChannel(
                    &raw mut self.sapwmus.canalSnd,
                    sampledSynth as i16,
                    (0x20 + initMono) as i16,
                    std::ptr::null_mut(),
                );
            }
            if !self.sapwmus.TabSons[num as usize].is_null() {
                let type_snd: i64 = u32::from_be_bytes(*b"snd ") as i64; // = 0x736E6420
                self.sapwmus.TabSons[num as usize] = GetResource(type_snd, num + 1000);
            }
            if !self.sapwmus.TabSons[num as usize].is_null() {
                if !self.sapwmus.canalSnd.is_null() {
                    SndPlay(
                        self.sapwmus.canalSnd,
                        self.sapwmus.TabSons[num as usize] as SndListHandle,
                        true,
                    );
                } else {
                    SndPlay(
                        std::ptr::null_mut(),
                        self.sapwmus.TabSons[num as usize] as SndListHandle,
                        true,
                    );
                }
            }
            #[cfg(target_os = "windows")]
            if (num == 11 || num == 12 || num == 6) {
                delai(30);
            }
        } else {
            self.delai(10);
        }
    }
}

/*************************************************************************
                attends_bruit
***************************************************************************/
#[unsafe(no_mangle)]
pub(crate) extern "C" fn attends_bruit() {}

/*************************************************************************
                quel_synthe
***************************************************************************/
#[unsafe(no_mangle)]
pub(crate) extern "C" fn quel_synthe(_n: i16) {}

/*************************************************************************
                reset_synthe
***************************************************************************/
#[unsafe(no_mangle)]
pub(crate) extern "C" fn reset_synthe() {}

impl Sapiens {
    /*************************************************************************
                    MusicInit
    Lance la musique désirée : 1 musique 1, 2 musique 2
    ***************************************************************************/
    pub(crate) fn MusicInit(&mut self, quelle: i16) {
        if self.sapglob.okmuse == TypeMuse::NO_MUSIQUE {
            ArreteMusique(true);
            self.sapwmus.MusiqueEnCours = FAUX;
            return;
        }
        SetCursor(GetCursor(watchCursor as i16));
        // S300 l'arret de la musique se fait dans SapiensExit
        //if(!IsExitMuse) atexit(ExitMuse);
        //IsExitMuse=VRAI;
        self.sapwmus.quelle_musique = quelle as i16;
        PushPath();
        SetPathInData(&self.sapgest, c"musics".as_ptr() as *mut i8);
        let nom = if quelle == 1 {
            c"Sapiens1.mux".as_ptr() as *mut i8
        } else {
            c"Sapiens2.mux".as_ptr() as *mut i8
        };
        JoueMusiqueHarmony(
            nom,
            std::ptr::null_mut(),
            c"".as_ptr() as *mut i8,
            0,
            c"DIGIT_QUALITE".as_ptr() as *mut i8,
            -1,
            0,
            0,
            false,
        );
        CarboniseSetCursorArrow();
        PopPath();

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
    }

    /*************************************************************************
                    CheckChangeMusique
    En mode player sur librairie on teste la fin de la musique
    ***************************************************************************/
    pub(crate) fn CheckChangeMusique(&mut self) {
        if IsFinDeMusique() {
            if self.sapwmus.quelle_musique == 1 {
                self.MusicInit(2);
            } else if self.sapwmus.quelle_musique == 2 {
                self.MusicInit(1);
            }
        }
    }
}
