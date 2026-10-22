#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
/***********************************************************************
                        SauvePref / ChargePref
***********************************************************************/
use crate::Sapiens;
use crate::exchange::{PreferencesRequest, request_preferences};
use crate::langue::{config_dir, effective_lang, langue_i16, set_langue};
use crate::sapiens::sapwind::FixeMenu;
use crate::sapiens::{TypeMuse, saptxt::CvtDefaultLangRess};
use crate::shared::Boolean;
use std::io::Read;

const VERSION_PREF: &[u8] = b"Sapiens Preferences 2.0.0\0";

pub(crate) const JOUEUR2_AUTONOME: i16 = 0;
const JOUEUR2_CLAVIER: i16 = 1;

fn pref_path() -> std::path::PathBuf {
    config_dir().join("Sapiens.cfg")
}

fn read_i16(r: &mut impl Read) -> Option<i16> {
    let mut b = [0u8; 2];
    r.read_exact(&mut b).ok()?;
    Some(i16::from_le_bytes(b))
}

impl Sapiens {
    pub(crate) fn SauvePref(&mut self) {
        let mut out = Vec::with_capacity(VERSION_PREF.len() + 10);
        out.extend_from_slice(VERSION_PREF);
        out.extend_from_slice(&i16::from(self.sapwmus.IsBruitages).to_le_bytes());
        out.extend_from_slice(&self.sapcomb.ModeJoueur2.to_le_bytes());
        out.extend_from_slice(&(self.sapglob.okmuse as i16).to_le_bytes());
        out.extend_from_slice(&langue_i16().to_le_bytes());
        // Format historique : seuls les deux octets bas de `SapZoom` sont écrits.
        let zoom_bits = self.sapwind.SapZoom.to_bits();
        out.extend_from_slice(&(zoom_bits as u16).to_le_bytes());

        let path = pref_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Err(e) = std::fs::write(&path, &out) {
            self.Collapse(&format!("Error writing preferences: {e}"));
        }
    }

    /***********************************************************************
                            ChargePref
    <= VRAI si first init
    ***********************************************************************/
    pub(crate) fn ChargePref(&mut self) -> Boolean {
        let Ok(bytes) = std::fs::read(pref_path()) else {
            return true;
        };
        let mut cur = std::io::Cursor::new(bytes);
        let mut version = vec![0u8; VERSION_PREF.len()];
        if cur.read_exact(&mut version).is_err() || version != VERSION_PREF {
            return true;
        }

        let Some(is_bruitages) = read_i16(&mut cur) else {
            return true;
        };
        let Some(mode_joueur2) = read_i16(&mut cur) else {
            return true;
        };
        let Some(okmuse) = read_i16(&mut cur) else {
            return true;
        };
        let Some(l_pref) = read_i16(&mut cur) else {
            return true;
        };
        let Some(zoom_low) = read_i16(&mut cur) else {
            return true;
        };

        self.sapwmus.IsBruitages = is_bruitages != 0;
        self.sapcomb.ModeJoueur2 = mode_joueur2;
        self.sapglob.okmuse = okmuse.into();
        set_langue(l_pref.into());
        let zoom_bytes = (zoom_low as u16).to_le_bytes();
        self.sapwind
            .set_zoom(f32::from_le_bytes([zoom_bytes[0], zoom_bytes[1], 0, 0]));
        false
    }

    /***********************************************************************
                            DoPreferences
    ***********************************************************************/
    pub(crate) fn DoPreferences(&mut self) {
        self.souris.MontreSouris();
        let request = PreferencesRequest {
            is_bruitages: self.sapwmus.IsBruitages,
            music: self.sapglob.okmuse != TypeMuse::NO_MUSIQUE,
            lissage_police: self.sapwcar.lissage_police,
            mode_joueur2: self.sapcomb.ModeJoueur2,
        };
        let Some(result) = request_preferences(request) else {
            self.MusicInit(1);
            return;
        };

        set_langue(effective_lang(result.lang_mode));
        self.sapcomb.ModeJoueur2 = result.mode_joueur2;
        if result.mode_joueur2 == JOUEUR2_CLAVIER {
            self.DoAlertSapiens(CvtDefaultLangRess(136));
        }
        self.sapwmus.IsBruitages = result.is_bruitages;
        self.sapglob.okmuse = if result.music {
            TypeMuse::MUSIQUE_HARMONY_QUALITE
        } else {
            TypeMuse::NO_MUSIQUE
        };
        self.sapwcar.lissage_police = result.lissage_police;

        self.inimen(8);
        self.SauvePref();
        FixeMenu(CvtDefaultLangRess(0));
        self.sapwind.FixeTailleMainWindow();
        self.inimen(0);
        self.MusicInit(1);
    }
}
