use strum::Display;

static GENEVA10: &[u8] = include_bytes!("../../data/mobs/GENEVA10.BRT");
static GENEVA14: &[u8] = include_bytes!("../../data/mobs/GENEVA14.BRT");
static W3D: &[u8] = include_bytes!("../../data/mobs/W3D.BRT");
static WAVPLAN: &[u8] = include_bytes!("../../data/mobs/WAVPLAN.BRT");
static WAVSPEC: &[u8] = include_bytes!("../../data/mobs/WAVSPEC.BRT");
static WBAMBOU: &[u8] = include_bytes!("../../data/mobs/WBAMBOU.BRT");
static WFACE: &[u8] = include_bytes!("../../data/mobs/WFACE.BRT");
static WFEUILLE: &[u8] = include_bytes!("../../data/mobs/WFEUILLE.BRT");
static WFLECHES: &[u8] = include_bytes!("../../data/mobs/WFLECHES.BRT");
static WSAPMOB: &[u8] = include_bytes!("../../data/mobs/WSAPMOB.BRT");
static WSILEX: &[u8] = include_bytes!("../../data/mobs/WSILEX.BRT");

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Display)]
pub(crate) enum Brt {
    GENEVA10,
    GENEVA14,
    W3D,
    WAVPLAN,
    WAVSPEC,
    WBAMBOU,
    WFACE,
    WFEUILLE,
    WFLECHES,
    WSAPMOB,
    WSILEX,
}

impl Brt {
    pub(crate) fn get(&self) -> &[u8] {
        match self {
            Brt::GENEVA10 => GENEVA10,
            Brt::GENEVA14 => GENEVA14,
            Brt::W3D => W3D,
            Brt::WAVPLAN => WAVPLAN,
            Brt::WAVSPEC => WAVSPEC,
            Brt::WBAMBOU => WBAMBOU,
            Brt::WFACE => WFACE,
            Brt::WFEUILLE => WFEUILLE,
            Brt::WFLECHES => WFLECHES,
            Brt::WSAPMOB => WSAPMOB,
            Brt::WSILEX => WSILEX,
        }
    }
}
