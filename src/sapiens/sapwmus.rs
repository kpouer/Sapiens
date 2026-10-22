#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::Sapiens;
use crate::acam::{CarboniseSetCursorArrow, FAUX, GetCursor, SetCursor};
use crate::audio;
use crate::cbridge::watchCursor;
use crate::sapiens::TypeMuse;
use crate::shared::Boolean;

pub(crate) struct Sapwmus {
    /*************** Variables globales exposees au C (SAPWMUS.C) ***************/
    pub(crate) quelle_musique: i16,
    pub(crate) IsBruitages: Boolean,
    MusiqueEnCours: Boolean,
}

impl Default for Sapwmus {
    fn default() -> Self {
        Self {
            quelle_musique: 1,
            IsBruitages: true,
            MusiqueEnCours: false,
        }
    }
}

impl Sapiens {
    pub(crate) fn bruit(&mut self, num: i16) {
        if self.sapwmus.IsBruitages {
            audio::play_sfx(num as u8);
            #[cfg(target_os = "windows")]
            if (num == 11 || num == 12 || num == 6) {
                self.delai(30);
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
pub(crate) extern "C" fn attends_bruit() {
    audio::wait_sfx();
}

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
            audio::stop_music();
            self.sapwmus.MusiqueEnCours = FAUX;
            return;
        }
        SetCursor(GetCursor(watchCursor as i16));
        self.sapwmus.quelle_musique = quelle;
        // Le séquenceur `muse1`/`muse2` du chemin OLD_VERSION est porté dans
        // src/audio/music.rs ; les fichiers .mux ne sont plus utilisés.
        audio::play_music(if quelle == 1 { 1 } else { 2 });
        self.sapwmus.MusiqueEnCours = true;
        CarboniseSetCursorArrow();
    }

    /*************************************************************************
                    CheckChangeMusique
    En mode player sur librairie on teste la fin de la musique
    ***************************************************************************/
    pub(crate) fn CheckChangeMusique(&mut self) {
        if audio::is_music_finished() {
            if self.sapwmus.quelle_musique == 1 {
                self.MusicInit(2);
            } else if self.sapwmus.quelle_musique == 2 {
                self.MusicInit(1);
            }
        }
    }
}
