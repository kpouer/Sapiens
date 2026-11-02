//! Implémentations réelles de l'API Acam / QuickDraw.
//!
//! Retire du jeu de stubs les symboles qui peuvent être rendus fonctionnels :
//! ports de dessin (fenêtre + offscreen), primitives QuickDraw, régions
//! (approximées par des rectangles), mémoire/`Handle`, chaînes Pascal,
//! timer, menus, curseur. Les fonctions qui restent des no-op sûrs sont dans
//! `stubs.rs` ; celles qui sont codées ici sont à ENLEVER de `stubs.rs` pour
//! éviter les doublons de symbole au link.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::cbridge::{MenuHandle, watchCursor};
use crate::exchange::{CursorShape, request_alert, set_cursor_shape, set_title};
use crate::langue::{Langue, langue};
use crate::port;
use crate::port::{PortId, Ports};
use crate::shared::*;
use std::ffi::c_void;
use std::sync::Mutex;
use std::time::Instant;

pub(crate) const VRAI: bool = true;
pub(crate) const FAUX: bool = false;

// ---------------------------------------------------------------------------
// Timer / aléa
// ---------------------------------------------------------------------------

static TICK_START: Mutex<Option<Instant>> = Mutex::new(None);

/// Ticks de 1/60 s depuis le premier appel, comme le `TickCount` du Mac.
pub(crate) fn TickCount() -> i64 {
    let mut g = TICK_START.lock().unwrap();
    let start = *g.get_or_insert_with(Instant::now);
    (start.elapsed().as_millis() * 60 / 1000) as i64
}

pub(crate) struct Random {
    RAND_SEED: u64,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            RAND_SEED: 0x1234567,
        }
    }
}

impl Random {
    fn next_rand(&mut self) -> u64 {
        let mut x = self.RAND_SEED;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.RAND_SEED = x;
        x
    }

    pub(crate) fn Random(&mut self) -> i64 {
        ((self.next_rand() >> 16) & 0x7FFF) as i64
    }

    pub(crate) fn CarboniseGetRandSeed(&self) -> i64 {
        self.RAND_SEED as i64
    }

    pub(crate) fn CarboniseSetRandSeed(&mut self, seed: i64) {
        self.RAND_SEED = seed as u64;
    }
}

// ---------------------------------------------------------------------------
// Faux handles
// ---------------------------------------------------------------------------

/// Huit octets alignes sur 8, servant de support a un faux `Handle`.
///
/// `#[repr(align)]]` n'est pas autorise sur un `static`, il faut donc passer par
/// ce type. L'alignement compte : ces adresses sont dereferencees en `void**`
/// par `DisposHandle` et le C (`*GetCursor`). Un `[u8; 8]` nu n'a que
/// l'alignement 1, le linker le pose alors n'importe ou (on l'a vu a
/// 0x...456) et le dereferencement leve
/// `panic_misaligned_pointer_dereference`, qui tue le jeu au demarrage.
#[repr(align(8))]
#[derive(Clone, Copy)]
struct Slot8([u8; 8]);

// ---------------------------------------------------------------------------
// Curseur
// ---------------------------------------------------------------------------

/// Nombre d'emplacements de curseur (les ids classiques du Toolbox :
/// flèche/sablier/`ibeam`...). Un seul id (`watchCursor`) est défini par
/// `cbridge.rs`, les autres tombent sur la flèche.
const NB_CURSOR_SLOTS: usize = 4;

/// Emplacements stables des « handles » de curseur : `GetCursor(id)` renvoie
/// celui de l'id. L'id y est gravé **+1** en little-endian (2 octets) :
/// - `*GetCursor(id)` (8 octets lus en `void*` par le C) reste non-nul,
/// - `SetCursor` relit l'id par rapport d'adresse pour choisir la forme.
static mut CURSOR_SLOTS: [Slot8; NB_CURSOR_SLOTS] = [Slot8([0; 8]); NB_CURSOR_SLOTS];

pub(crate) fn GetCursor(id: i16) -> Cursor {
    let idx = (id.max(0) as usize) % NB_CURSOR_SLOTS;
    unsafe {
        let slot = core::ptr::addr_of_mut!(CURSOR_SLOTS)
            .cast::<Slot8>()
            .add(idx);
        let stored = id.wrapping_add(1);
        (*slot).0[0] = stored as u8;
        (*slot).0[1] = (stored >> 8) as u8;
        slot as *mut c_void
    }
}

/// Relit l'id gravé par `GetCursor` dans un handle ; renvoie `None` si le
/// pointeur n'est pas un des emplacements (handle nul ou octets déréférencés).
fn cursor_id(c: Cursor) -> Option<i16> {
    if c.is_null() {
        return None;
    }
    unsafe {
        let base = core::ptr::addr_of!(CURSOR_SLOTS) as usize;
        let addr = c as usize;
        let align = core::mem::size_of::<Slot8>();
        if addr >= base && addr < base + NB_CURSOR_SLOTS * align && (addr - base) % align == 0 {
            Some(core::ptr::read_unaligned(c as *const i16).wrapping_sub(1))
        } else {
            None
        }
    }
}

/// `SetCursor` du C : pose la forme demandée (sablier pendant les temps de
/// chargement, flèche sinon) via le pont `exchange.rs` ; c'est le thread egui
/// qui l'applique réellement à l'écran.
pub(crate) fn SetCursor(c: Cursor) {
    let shape = match cursor_id(c) {
        Some(id) if id == watchCursor as i16 => CursorShape::Watch,
        _ => CursorShape::Arrow,
    };
    set_cursor_shape(shape);
}

/// `CarboniseSetCursorArrow` : restaure la flèche après une attente
/// (SAPWIND.C, SAP3D.C, SAPWMUS.C, SAPWCLAV.C).
pub(crate) fn CarboniseSetCursorArrow() {
    set_cursor_shape(CursorShape::Arrow);
}

// ---------------------------------------------------------------------------
// Mémoire
// ---------------------------------------------------------------------------

#[deprecated]
pub(crate) fn NewPtr(size: i64) -> Ptr {
    alloc_pixels(size.max(0) as usize)
}

pub(crate) fn DisposHandle(h: Handle) {
    if h.is_null() {
        return;
    }
    unsafe {
        let inner = *h;
        if !inner.is_null() {
            free_pixels(inner as *mut u8);
        }
        free_pixels(h as *mut u8);
    }
}

// ---------------------------------------------------------------------------
// Chaînes Pascal
// ---------------------------------------------------------------------------

pub(crate) fn PtoCstr(str: StringPtr) -> *mut i8 {
    if str.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let len = *str;
        let p = str.add(1);
        std::ptr::copy(p, str, len as usize);
        *str.add(len as usize) = 0;
        str as *mut i8
    }
}

pub(crate) fn CtoPstr(str: *mut i8) -> *mut u8 {
    if str.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let mut len: usize = 0;
        while *str.add(len) != 0 && len < 255 {
            len += 1;
        }
        let p = str as *mut u8;
        std::ptr::copy(p, p.add(1), len);
        *p = len as u8;
        p
    }
}

/// Ressources `STR#` extraites de `c_code/Rsrc/sapiens.rsrc`.
/// Les identifiants suivent `CvtLangRess` : base + 1000 * langue
/// (0 = français, 1000 = anglais, 2000 = espagnol).
/// Les index absents (ou chaînes vides) donnent une chaîne vide.
const STR_FR: &[&str] = &[
    "Aucune",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Des fichiers essentiels sont absents, réinstallez le logiciel.",
    "",
    "",
    "",
    "C'est toi.Ton nom est %s.",
    "C'est ton ami.Son nom est %s.",
    "C'est ton adversaire.",
    "C'est l'ami de ton adversaire.",
    "Ce n'est personne.",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "C'est l'indicateur de ton souffle et de ton état.",
    "C'est l'indicateur de souffle et d'état de tes adversaires.",
    "C'est l'indicateur du souffle et de l'état de ton ami.",
];
const STR_GB: &[&str] = &[
    "None",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Important files are missing. Please reinstall this software.",
    "",
    "",
    "",
    "It's you. Your name is %s.",
    "It's your friend, whose name is %s.",
    "It's your opponent.",
    "It's your opponent's friend.",
    "Nobody's here.",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "It's your breath and condition indicator.",
    "It's your opponent's breath and condition indicator.",
    "It's your friend's breath and condition indicator.",
];
const STR_ES: &[&str] = &[
    "Ninguno",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Faltan archivos importantes. Por favor reinstalar el programa.",
    "",
    "",
    "",
    "Eres tú. Tu nombre es %s.",
    "Es tu amigo, su nombre es %s.",
    "Es tu oponente.",
    "Es el amigo de tu oponente.",
    "No hay nadie aquí.",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Es tu indicador de respiración y condición.",
    "Es el indicador de respiración y condición de tu oponente.",
    "Es el indicado de respiración y condición de tu amigo.",
];

fn str_resource(id: i16) -> &'static [&'static str] {
    match id {
        0 => STR_FR,
        1000 => STR_GB,
        2000 => STR_ES,
        1 => &["Edition spéciale 30ème anniversaire"],
        1001 => &["30th anniversary special edition"],
        2001 => &["Edición especial 30 aniversario "],
        128 => &["Français", "English", "Castellano"],
        _ => &[],
    }
}

/// Écrit dans `s` (chaîne Pascal, MacRoman) la chaîne n° `index` (base 1)
/// de la ressource `STR#` `id`. Chaîne vide si elle n'existe pas.
pub(crate) fn GetIndString(s: StringPtr, id: i16, index: i16) {
    if s.is_null() {
        return;
    }
    let text = usize::try_from(index - 1)
        .ok()
        .and_then(|i| str_resource(id).get(i))
        .copied()
        .unwrap_or("");
    unsafe {
        let mut len = 0usize;
        for c in text.chars() {
            if len == 255 {
                break;
            }
            let b = crate::text::unicode_to_mac_roman(c).unwrap_or(b'?');
            *s.add(1 + len) = b;
            len += 1;
        }
        *s = len as u8;
    }
}

// ---------------------------------------------------------------------------
// Alertes / ParamText
// ---------------------------------------------------------------------------

static PARAM_TEXT: Mutex<[String; 4]> =
    Mutex::new([String::new(), String::new(), String::new(), String::new()]);

pub(crate) fn c_string(p: StringPtr) -> String {
    if p.is_null() {
        return String::new();
    }
    unsafe {
        std::ffi::CStr::from_ptr(p as *const i8)
            .to_string_lossy()
            .into_owned()
    }
}

/// Substitution des quatre zones `^0`..`^3` des alertes (Toolbox Mac).
pub(crate) fn ParamText(p1: String, p2: String, p3: String, p4: String) {
    let mut g = PARAM_TEXT.lock().unwrap();
    g[0] = p1;
    g[1] = p2;
    g[2] = p3;
    g[3] = p4;
}

/// Affiche une alerte via egui et attend l'item choisi. Bloquant côté C.
pub(crate) fn Alert(alert_id: i16, _filter: *mut c_void) -> i16 {
    let params = PARAM_TEXT.lock().unwrap().clone();
    request_alert(alert_id, params)
}

// ---------------------------------------------------------------------------
// Fenêtre principale
// ---------------------------------------------------------------------------

pub(crate) fn NewCWindow(b: &Rect) -> SapPort {
    /* Le positionnement global de la fenêtre est géré par egui : le port de
    dessin travaille en coordonnées locales (0,0)-(largeur,hauteur). */
    let local = Rect::new(0, 0, b.right - b.left, b.bottom - b.top);
    port::create(local, 8)
}

/// `SetWTitle` : le titre (chaîne Pascal MacRoman) part par le pont
/// `exchange.rs` ; c'est le thread egui qui l'applique à la fenêtre native
/// (`ViewportCommand::Title`), une seule fenêtre étant créée par `NewCWindow`.
pub(crate) fn SetWTitle(_w: &SapPort, title: String) {
    set_title(title);
}

// ---------------------------------------------------------------------------
// Menus
// ---------------------------------------------------------------------------
//
// La barre de menus Mac (ressources MBAR/MENU, strippées du portage) est
// reconstituée ici : titres et items se déduisent du code C (SAPWIND.C) et
// des libellés du jeu (« Fichier »/« Archivo »/« File », SAPGEST.C). Les
// actions, elles, restent en C : `DoMenuCommand(menuID<<16|item)`, appelée
// par egui (src/main.rs). `DisableItem`/`EnableItem`/`CheckItem` posent
// l'état (depuis `AdjustMenus`, thread C) ; egui relit cet état via
// `menus_snapshot` à chaque frame.

/// Décalage d'ID appliqué par `CvtDefaultLangRess` selon la langue (SAPIENS.H).
const DELTA_LANGUE: i16 = 1000;

/// Textes d'un titre ou item dans les trois langues du jeu.
#[derive(Clone, Copy)]
struct L10n {
    fr: &'static str,
    gb: &'static str,
    es: &'static str,
}

const fn l10n(fr: &'static str, gb: &'static str, es: &'static str) -> L10n {
    L10n { fr, gb, es }
}

impl L10n {
    fn label(&self, lang: Langue) -> &'static str {
        match lang {
            Langue::FR => self.fr,
            Langue::ES => self.es,
            Langue::GB => self.gb,
        }
    }
}

/// Un item de menu : textes, séparateur et état (activé/coché).
struct MenuItem {
    l: L10n,
    sep: bool,
    enabled: bool,
    checked: bool,
}

/// Un menu : id courant (avec l'offset langue), titre et items.
struct Menu {
    id: i16,
    title: L10n,
    enabled: bool,
    items: Vec<MenuItem>,
}

/// Modèle complet de la barre de menus. Les `Menu` sont dans des `Box` dont
/// l'adresse (le `MenuHandle`) est stable tant que la langue ne change pas.
pub(crate) struct MenuModel {
    /// Langue avec laquelle le modèle a été construit (0/1/2).
    lang: Langue,
    menus: Vec<Menu>,
}

pub(crate) static MENU_MODEL: Mutex<Option<MenuModel>> = Mutex::new(None);

/// Emplacement stable du « handle » de barre de menus : `GetNewMBar` renvoie
/// son adresse (FixeMenu passera ce pointeur à `DisposHandle`, no-op sur un
/// entier de mémoire statique, d'où `Slot8` pour que le `*h` ne pante pas).
static mut MENU_BAR_HANDLE: Slot8 = Slot8([0; 8]);

fn fresh_menu(id_base: i16, title: L10n, lang: Langue, items: &[(L10n, bool)]) -> Menu {
    Menu {
        id: id_base + DELTA_LANGUE * lang as i16,
        title,
        enabled: true,
        items: items
            .iter()
            .map(|&(l, sep)| MenuItem {
                l,
                sep,
                enabled: true,
                checked: false,
            })
            .collect(),
    }
}

pub(crate) fn build_model() -> MenuModel {
    let lang = langue();
    // menu 128 (pomme) : itemInformation=1, itemAide=2.
    let pomme = [
        (
            l10n(
                "À propos de Sapiens…",
                "About Sapiens…",
                "Acerca de Sapiens…",
            ),
            false,
        ),
        (l10n("Aide", "Help", "Ayuda"), false),
    ];
    // menu 129 (fichier) : Nouveau, Charger, Sauver, sép., Préférences, sép., Quitter.
    let fichier = [
        (l10n("Nouveau", "New", "Nuevo"), false),
        (l10n("Charger…", "Load…", "Cargar…"), false),
        (l10n("Sauver…", "Save…", "Guardar…"), false),
        (l10n("", "", ""), true),
        (l10n("Préférences…", "Preferences…", "Preferencias…"), false),
        (l10n("", "", ""), true),
        (l10n("Quitter", "Quit", "Salir"), false),
    ];
    // menu 130 (voir) : iPaysage, iLaterale, iPanorama, iCarte.
    let voir = [
        (l10n("Paysage", "Landscape", "Paisaje"), false),
        (l10n("Latérale", "Side view", "Vista lateral"), false),
        (l10n("Panorama", "Panorama", "Panorama"), false),
        (l10n("Carte", "Map", "Mapa"), false),
    ];
    // menu 131 (fenêtre) : iEchelle100..400.
    let fenetre = [
        (l10n("Échelle 100 %", "Zoom 100 %", "Escala 100 %"), false),
        (l10n("Échelle 200 %", "Zoom 200 %", "Escala 200 %"), false),
        (l10n("Échelle 300 %", "Zoom 300 %", "Escala 300 %"), false),
        (l10n("Échelle 400 %", "Zoom 400 %", "Escala 400 %"), false),
    ];
    MenuModel {
        lang,
        menus: vec![
            fresh_menu(128, l10n("SAPIENS", "SAPIENS", "SAPIENS"), lang, &pomme),
            fresh_menu(129, l10n("Fichier", "File", "Archivo"), lang, &fichier),
            fresh_menu(130, l10n("Voir", "View", "Ver"), lang, &voir),
            fresh_menu(131, l10n("Fenêtre", "Window", "Ventana"), lang, &fenetre),
        ],
    }
}

fn menu_handle_ptr(menu: &Menu) -> MenuHandle {
    (menu as *const Menu) as *mut c_void
}

pub(crate) fn GetNewMBar(_menuBarID: i16) -> Handle {
    *MENU_MODEL.lock().unwrap() = Some(build_model());
    unsafe { core::ptr::addr_of_mut!(MENU_BAR_HANDLE) as *mut _ }
}

pub(crate) fn GetMHandle(menuID: i16) -> MenuHandle {
    let g = MENU_MODEL.lock().unwrap();
    match &*g {
        Some(m) => m
            .menus
            .iter()
            .find(|menu| menu.id == menuID)
            .map(menu_handle_ptr)
            .unwrap_or(std::ptr::null_mut()),
        None => std::ptr::null_mut(),
    }
}

pub(crate) fn SetMenuBar(_menuBar: Handle) {}

pub(crate) fn DrawMenuBar() {}

fn find_menu(menus: &mut [Menu], handle: MenuHandle) -> Option<&mut Menu> {
    if handle.is_null() {
        return None;
    }
    menus
        .iter_mut()
        .find(|menu| menu_handle_ptr(menu) == handle)
        .map(|menu| &mut *menu)
}

fn menu_at(menus: &[Menu], handle: MenuHandle) -> Option<&Menu> {
    if handle.is_null() {
        return None;
    }
    menus
        .iter()
        .find(|menu| menu_handle_ptr(menu) == handle)
        .map(|menu| &*menu)
}

fn handle_is_known(menus: &[Menu], handle: MenuHandle) -> bool {
    menu_at(menus, handle).is_some()
}

fn label_for(menus: &[Menu], handle: MenuHandle, item: i16) -> Option<&'static str> {
    let lang = langue();
    let menu = menu_at(menus, handle)?;
    if item <= 0 {
        Some(menu.title.label(lang))
    } else {
        menu.items
            .get((item - 1) as usize)
            .map(|it| it.l.label(lang))
    }
}

pub(crate) fn DisableItem(menu: MenuHandle, item: i16) {
    if let Some(m) = &mut *MENU_MODEL.lock().unwrap()
        && let Some(menu) = find_menu(&mut m.menus, menu)
    {
        if item == 0 {
            menu.enabled = false;
        } else if let Some(it) = menu.items.get_mut((item - 1) as usize) {
            it.enabled = false;
        }
    }
}

pub(crate) fn EnableItem(menu: MenuHandle, item: i16) {
    if let Some(m) = &mut *MENU_MODEL.lock().unwrap()
        && let Some(menu) = find_menu(&mut m.menus, menu)
    {
        if item == 0 {
            menu.enabled = true;
        } else if let Some(it) = menu.items.get_mut((item - 1) as usize) {
            it.enabled = true;
        }
    }
}

pub(crate) fn CheckItem(menu: MenuHandle, item: i16, checked: Boolean) {
    if let Some(m) = &mut *MENU_MODEL.lock().unwrap()
        && let Some(menu) = find_menu(&mut m.menus, menu)
        && let Some(it) = menu.items.get_mut((item - 1) as usize)
    {
        it.checked = checked;
    }
}

/// `GetItem` remplit le tampon Pascal avec le libellé courant de l'item
/// (`SAPWIND.C` l'utilise pour les noms de desk accessory de l'Apple menu).
pub(crate) fn GetItem(menu: MenuHandle, item: i16, text: StringPtr) {
    let blank = |text: StringPtr| unsafe {
        *text = 0;
    };
    if text.is_null() {
        return;
    }
    let g = MENU_MODEL.lock().unwrap();
    let Some(m) = &*g else {
        blank(text);
        return;
    };
    let label = if handle_is_known(&m.menus, menu) {
        self::label_for(&m.menus, menu, item)
    } else {
        None
    };
    drop(g);
    match label {
        Some(s) => {
            let bytes = s.as_bytes();
            let n = bytes.len().min(255);
            unsafe {
                *text = n as u8;
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), text.add(1), n);
            }
        }
        None => blank(text),
    }
}

// ---------------------------------------------------------------------------
// Menus — exploration pour egui
// ---------------------------------------------------------------------------

pub(crate) struct ItemSnapshot {
    pub(crate) index: i16,
    pub(crate) text: String,
    pub(crate) sep: bool,
    pub(crate) enabled: bool,
    pub(crate) checked: bool,
}

pub(crate) struct MenuSnapshot {
    pub(crate) base_id: i16,
    pub(crate) id: i16,
    pub(crate) title: String,
    pub(crate) enabled: bool,
    pub(crate) items: Vec<ItemSnapshot>,
}

/// Recopie l'état courant de la barre de menus (libellés et état) pour egui.
/// `id` est l'ID de menu « runtime » (avec l'offset langue) dont on a besoin
/// pour construire `menuID<<16|item` ; `base_id` sert à retrouver un menu
/// indépendamment de la langue (raccourcis clavier).
pub(crate) fn menus_snapshot() -> Vec<MenuSnapshot> {
    let lang = langue();
    let mut g = MENU_MODEL.lock().unwrap();
    // Si la langue C a changé ailleurs (ChargePref/Alert), on reconstruit le
    // modèle pour garder IDs de menus et libellés cohérents.
    if let Some(m) = g.as_mut()
        && m.lang != lang
    {
        *m = build_model();
    }
    let Some(m) = &*g else { return Vec::new() };
    m.menus
        .iter()
        .map(|menu| {
            let base_id = menu.id - DELTA_LANGUE * (menu.id / DELTA_LANGUE);
            MenuSnapshot {
                base_id,
                id: menu.id,
                title: menu.title.label(lang).to_string(),
                enabled: menu.enabled,
                items: menu
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, it)| ItemSnapshot {
                        index: (i + 1) as i16,
                        text: it.l.label(lang).to_string(),
                        sep: it.sep,
                        enabled: it.enabled,
                        checked: it.checked,
                    })
                    .collect(),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Primitives graphiques
// ---------------------------------------------------------------------------

/// Trace un pixel plein (`mode` tenu en compte), avec clip.
#[inline]
pub(crate) fn plot(p: &mut SapPort, x: i16, y: i16, c: u8) {
    if x < p.clip.left || x >= p.clip.right || y < p.clip.top || y >= p.clip.bottom {
        return;
    }
    let w = p.row_bytes as usize;
    let off = (y as usize) * w + (x as usize);
    let mode = p.pen_mode;
    let Some(d) = p.pixels.get_mut(off) else {
        return;
    };
    *d = match mode {
        PAT_OR => *d | c,
        PAT_XOR => *d ^ c,
        PAT_BIC => *d & !c,
        _ => c,
    };
}

/// Remplit un rectangle (respecte clip), et accumule dans la région ouverte.
fn fill_rect(p: &mut SapPort, r: &Rect, color: u8) {
    p.open_acc = p.open_acc.union(r);
    let c = r.intersect(&p.clip);
    if c.is_empty() {
        return;
    }
    let w = p.row_bytes as usize;
    let mode = p.pen_mode;
    let (l, r) = (c.left as usize, c.right as usize);
    for y in c.top..c.bottom {
        let start = (y as usize) * w;
        let row = &mut p.pixels[start + l..start + r];
        match mode {
            PAT_OR => row.iter_mut().for_each(|d| *d |= color),
            PAT_XOR => row.iter_mut().for_each(|d| *d ^= color),
            PAT_BIC => row.iter_mut().for_each(|d| *d &= !color),
            _ => row.fill(color),
        }
    }
}

/// Trace un segment de droite épais (taille du crayon), avec clip.
fn draw_line(port: *mut SapPort, x0: i16, y0: i16, x1: i16, y1: i16) {
    unsafe {
        let p = &mut *port;
        p.open_acc = p.open_acc.union(&Rect {
            top: y0.min(y1),
            left: x0.min(x1),
            bottom: (y0.max(y1) as i32 + 1).min(32767) as i16,
            right: (x0.max(x1) as i32 + 1).min(32767) as i16,
        });
        let pw = p.pen_size.h.max(1) as i32;
        let ph = p.pen_size.v.max(1) as i32;
        let c = p.fore_index;
        let x0 = x0 as i32;
        let y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let (mut x, mut y) = (x0, y0);

        loop {
            for py in y..y + ph {
                for px in x..x + pw {
                    plot(p, px as i16, py as i16, c);
                }
            }
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl Ports {
    pub(crate) fn MoveTo(&mut self, h: i16, v: i16) {
        self.current_mut().pen_loc = Point { v, h };
    }

    pub(crate) fn LineTo(&mut self, h: i16, v: i16) {
        let p = self.current_mut();
        let loc = p.pen_loc;
        draw_line(p, loc.h, loc.v, h, v);
        p.pen_loc = Point { v, h };
    }

    pub(crate) fn Line(&mut self, dh: i16, dv: i16) {
        let p = self.current_mut();
        let loc = p.pen_loc;
        let h = loc.h.wrapping_add(dh);
        let v = loc.v.wrapping_add(dv);
        draw_line(p, loc.h, loc.v, h, v);
        p.pen_loc = Point { v, h };
    }

    pub(crate) fn PenSize(&mut self, width: i16, height: i16) {
        let p = self.current_mut();
        p.pen_size = Point {
            h: width.max(1),
            v: height.max(1),
        }
    }

    pub(crate) fn PenMode(&mut self, mode: i16) {
        let p = self.current_mut();
        p.pen_mode = mode;
    }

    pub(crate) fn PenNormal(&mut self) {
        let p = self.current_mut();
        p.pen_size = Point { v: 1, h: 1 };
        p.pen_mode = PAT_COPY;
    }
}

fn color_num_to_index(color: i16) -> u8 {
    match color {
        BLACK_COLOR => 0,
        WHITE_COLOR => 215,
        205 => rgb_to_index(200, 40, 40),
        341 => rgb_to_index(40, 160, 40),
        409 => rgb_to_index(40, 60, 200),
        9 => 3,
        _ => 0,
    }
}

impl Ports {
    pub(crate) fn ForeColor(&mut self, color: i16) {
        let p = self.current_mut();
        p.fore_index = color_num_to_index(color);
    }

    pub(crate) fn BackColor(&mut self, color: i16) {
        let p = self.current_mut();
        p.back_index = color_num_to_index(color);
    }

    pub(crate) fn RGBForeColor(&mut self, c: &RGBColor) {
        let p = self.current_mut();
        p.fore_index = rgb_to_index(
            (c.red >> 8) as u8,
            (c.green >> 8) as u8,
            (c.blue >> 8) as u8,
        )
    }
}

pub(crate) fn Index2Color(index: i16) -> RGBColor {
    let i = index.clamp(0, 255) as usize;
    let (r, g, b) = lut_rgb(i as u8);
    RGBColor {
        red: ((r as u16) << 8) | r as u16,
        green: ((g as u16) << 8) | g as u16,
        blue: ((b as u16) << 8) | b as u16,
    }
}

pub(crate) fn Color2Index(c: &RGBColor) -> i16 {
    rgb_to_index(
        (c.red >> 8) as u8,
        (c.green >> 8) as u8,
        (c.blue >> 8) as u8,
    ) as i16
}

// ---------------------------------------------------------------------------
// Rectangles
// ---------------------------------------------------------------------------
pub(crate) fn SectRect(src1: &Rect, src2: &Rect) -> Option<Rect> {
    Some(src1.intersect(src2)).filter(|r| !r.is_empty())
}

// ---------------------------------------------------------------------------
// Remplissage / contour
// ---------------------------------------------------------------------------

impl Ports {
    pub(crate) fn PaintRect(&mut self, r: &Rect) {
        let p = self.current_mut();
        let fore = p.fore_index;
        fill_rect(p, r, fore);
    }

    pub(crate) fn EraseRect(&mut self, r: &Rect) {
        let p = self.current_mut();
        let back = p.back_index;
        fill_rect(p, r, back);
    }
}

fn frame_rect(p: &mut SapPort, r: &Rect) {
    p.open_acc = p.open_acc.union(r);
    if r.is_empty() {
        return;
    }
    let c = p.fore_index;
    let right = r.right.saturating_sub(1);
    let bottom = r.bottom.saturating_sub(1);
    for x in r.left..r.right {
        if x > clip_overflow_bound() {
            break;
        }
        plot(p, x, r.top, c);
        if bottom >= r.top {
            plot(p, x, bottom, c);
        }
    }
    for y in r.top..r.bottom {
        plot(p, r.left, y, c);
        if right >= r.left {
            plot(p, right, y, c);
        }
    }
}

#[inline]
fn clip_overflow_bound() -> i16 {
    32760
}

impl Ports {
    pub(crate) fn FrameRect(&mut self, r: &Rect) {
        let p = self.current_mut();
        frame_rect(p, r);
    }

    pub(crate) fn FrameRoundRect(&mut self, r: &Rect, _ovalWidth: i16, _ovalHeight: i16) {
        let p = self.current_mut();
        frame_rect(p, r);
    }

    pub(crate) fn ClipRect(&mut self, r: &Rect) {
        let p = self.current_mut();
        let b = p.bounds;
        p.clip = r.intersect(&b);
    }
}

// ---------------------------------------------------------------------------
// CopyBits / ScrollRect
// ---------------------------------------------------------------------------
pub(crate) enum PortKind {
    PortId(PortId),
    Raw(SapPort),
}

impl From<PortId> for PortKind {
    fn from(id: PortId) -> Self {
        Self::PortId(id)
    }
}

impl From<SapPort> for PortKind {
    fn from(sap_port: SapPort) -> Self {
        Self::Raw(sap_port)
    }
}

impl PortKind {
    pub fn unwrap_or<'a>(&'a self, ports: &'a Ports) -> &'a SapPort {
        match self {
            PortKind::Raw(x) => x,
            PortKind::PortId(id) => ports.port(*id),
        }
    }

    pub fn unwrap_mut_or<'a>(&'a mut self, ports: &'a mut Ports) -> &'a mut SapPort {
        match self {
            PortKind::Raw(x) => x,
            PortKind::PortId(id) => ports.port_mut(*id),
        }
    }
}

impl Ports {
    pub(crate) fn CopyBits<S: Into<PortKind>, D: Into<PortKind>>(
        &mut self,
        src_id: S,
        dst_id: D,
        sr: &Rect,
        dr: &Rect,
        mode: i16,
    ) {
        let src_id = src_id.into();
        let mut dst_id = dst_id.into();
        let dclip = {
            let dp = dst_id.unwrap_mut_or(self);
            dp.open_acc = dp.open_acc.union(dr);
            let dclip = dr.intersect(&dp.clip);
            if sr.is_empty() || dclip.is_empty() {
                return;
            }
            dclip
        };
        let sw = sr.right - sr.left;
        let sh = sr.bottom - sr.top;
        let dw = dr.right - dr.left;
        let dh = dr.bottom - dr.top;
        let wd = dst_id.unwrap_or(self).row_bytes as usize;
        let ws = src_id.unwrap_or(self).row_bytes as usize;
        let ob = dst_id.unwrap_or(self).bounds;
        let sb = src_id.unwrap_or(self).bounds;
        if src_id.unwrap_or(self).pixels.is_empty() || dst_id.unwrap_or(self).pixels.is_empty() {
            return;
        }
        /* La source peut être la destination (ScrollRect) : chaque ligne source
        est d'abord recopiée dans un tampon. */
        let mut tmp: Vec<u8> = Vec::new();
        let combine = |d: &mut u8, v: u8| {
            *d = match mode {
                10 => *d ^ v,
                9 => *d | v,
                11 => *d & !v,
                _ => v,
            }
        };

        // Cas identiques : copie par lignes.
        if sw == dw && sh == dh {
            for dy in dclip.top..dclip.bottom {
                let sy = sr.top + (dy - dr.top);
                if sy < sr.top || sy >= sr.bottom {
                    continue;
                }
                if sy < sb.top || sy >= sb.bottom {
                    continue;
                }
                let start = dclip.left.max(dr.left);
                let end = dclip.right.min(dr.right);
                if start >= end {
                    continue;
                }
                let srcx = sr.left + (start - dr.left);
                if srcx < sb.left || srcx + (end - start) > sb.right {
                    continue;
                }
                let dsty = dy - ob.top;
                let srcy = sy - sb.top;
                if dsty < 0 {
                    continue;
                }
                let n = (end - start) as usize;
                let so = (srcy as usize) * ws + (srcx - sb.left) as usize;
                tmp.clear();
                tmp.extend_from_slice(&src_id.unwrap_or(self).pixels[so..so + n]);
                let doff = (dsty as usize) * wd + (start - ob.left) as usize;
                let dstrow = &mut dst_id.unwrap_mut_or(self).pixels[doff..doff + n];
                if mode == 10 || mode == 9 || mode == 11 {
                    dstrow
                        .iter_mut()
                        .zip(&tmp)
                        .for_each(|(d, &v)| combine(d, v));
                } else {
                    dstrow.copy_from_slice(&tmp);
                }
            }
            return;
        }

        // Mise à l'échelle (nearest).
        for dy in dclip.top..dclip.bottom {
            let sy = sr.top + (((dy as i32 - dr.top as i32) * sh as i32) / dh.max(1) as i32) as i16;
            if sy < sr.top || sy >= sr.bottom {
                continue;
            }
            if sy < sb.top || sy >= sb.bottom {
                continue;
            }
            let dsty = dy - ob.top;
            let srcy = sy - sb.top;
            if dsty < 0 || dsty as i32 >= (ob.bottom - ob.top) as i32 {
                continue;
            }
            let so = (srcy as usize) * ws;
            tmp.clear();
            tmp.extend_from_slice(&src_id.unwrap_or(self).pixels[so..so + ws]);
            let doff = (dsty as usize) * wd;
            let dstrow = &mut dst_id.unwrap_mut_or(self).pixels[doff..doff + wd];
            for dx in dclip.left..dclip.right {
                let sx = sr.left
                    + (((dx as i32 - dr.left as i32) * sw as i32) / dw.max(1) as i32) as i16;
                if sx < sr.left || sx >= sr.right {
                    continue;
                }
                if sx < sb.left || sx >= sb.right {
                    continue;
                }
                let v = tmp[(sx - sb.left) as usize];
                combine(&mut dstrow[(dx - ob.left) as usize], v);
            }
        }
    }

    pub(crate) fn ScrollRect(&mut self, r: &Rect, dh: i16, dv: i16, updateRgn: RgnHandle) {
        let p = self.current_id();
        unsafe {
            let src = *r;
            if src.is_empty() {
                return;
            }
            let mut dst = src;
            dst.left = dst.left.wrapping_add(dh);
            dst.right = dst.right.wrapping_add(dh);
            dst.top = dst.top.wrapping_add(dv);
            dst.bottom = dst.bottom.wrapping_add(dv);
            let clipped = src.intersect(&dst);
            if !clipped.is_empty() {
                self.CopyBits(p, p, &clipped, &clipped, 0);
            }
            if !updateRgn.is_null() {
                let union_ = src.union(&dst);
                *(updateRgn as *mut Rect) = union_;
                // Région non vide
            }
        }
    }

    // ---------------------------------------------------------------------------
    // Régions (approximation rectangulaire)
    // ---------------------------------------------------------------------------
    pub(crate) fn OpenRgn(&mut self) {
        let p = self.current_mut();
        p.open_rgn = 1;
        p.open_acc = Rect::default();
    }

    pub(crate) fn CloseRgn(&mut self) -> Rect {
        let p = self.current_mut();
        let acc = p.open_acc;
        p.open_rgn = 0;
        p.open_acc = Rect::default();
        acc
    }

    pub(crate) fn GetClip(&mut self) -> Rect {
        let p = self.current();
        p.clip
    }

    pub(crate) fn SetClip(&mut self, rgn: &Rect) {
        let p = self.current_mut();
        let b = p.bounds;
        p.clip = rgn.intersect(&b);
    }

    pub(crate) fn EraseRgn(&mut self, rgn: &Rect) {
        let p = self.current_mut();
        let back = p.back_index;
        fill_rect(p, rgn, back);
    }

    pub(crate) fn FrameRgn(&mut self, rgn: &Rect) {
        let p = self.current_mut();
        frame_rect(p, rgn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copybits_agrandit_sans_perdre_la_derniere_ligne() {
        let mut src = port::create(Rect::new(0, 0, 640, 400), 8);
        let mut dst = port::create(Rect::new(0, 0, 1280, 800), 8);
        for y in 0..400usize {
            for x in 0..640usize {
                src.pixels[y * 640 + x] = ((x * 7 + y * 13) & 0xFF) as u8;
            }
        }
        dst.pixels.fill(0xAA);
        // Les ports passés par valeur seraient libérés par CopyBits : on les
        // range dans `Ports` pour relire la destination ensuite.
        let mut ports = Ports::default();
        ports.mainWindow = src;
        ports.debecr = dst;
        ports.CopyBits(
            PortId::MainWindow,
            PortId::Debecr,
            &Rect::new(0, 0, 640, 400),
            &Rect::new(0, 0, 1280, 800),
            0,
        );

        for y in 0..800usize {
            for x in 0..1280usize {
                let expected = (((x / 2) * 7 + (y / 2) * 13) & 0xFF) as u8;
                assert_eq!(
                    ports.debecr.pixels[y * 1280 + x],
                    expected,
                    "pixel ({x}, {y}) mal mis à l'échelle"
                );
            }
        }
    }
}
