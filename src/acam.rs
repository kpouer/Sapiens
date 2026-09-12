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

use crate::cbridge::{MenuHandle, OSErr, WindowPtr};
use crate::langue::Langue;
use crate::sapiens::saptxt::{InitAndCheckTexts, langue};
use crate::shared::*;
use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::time::Instant;
// ---------------------------------------------------------------------------
// Timer global C (SAPWCLAV.C)
// ---------------------------------------------------------------------------

unsafe extern "C" {
    /// `volatile unsigned long sapiensTimer;` (64 bits sur macOS).
    static mut sapiensTimer: u64;
}

pub const VRAI: bool = true;
pub const FAUX: bool = false;

#[macro_export]
macro_rules! nomJoueur {
    () => {
        crate::sapiens::saptxt::nomjoueur
    };
}

/// Incrémente `sapiensTimer` (appelé toutes les 11 ms par le thread Rust).
#[inline]
pub fn bump_sapiens_timer() {
    unsafe {
        let cur = core::ptr::read(&raw const sapiensTimer);
        core::ptr::write_volatile(&raw mut sapiensTimer, cur.wrapping_add(1));
    }
}

// ---------------------------------------------------------------------------
// Timer / aléa
// ---------------------------------------------------------------------------

static TICK_START: Mutex<Option<Instant>> = Mutex::new(None);

#[unsafe(no_mangle)]
pub extern "C" fn TickCount() -> i64 {
    let mut g = TICK_START.lock().unwrap();
    let start = *g.get_or_insert_with(Instant::now);
    start.elapsed().as_millis() as i64
}

static mut RAND_SEED: u64 = 0x1234567;

fn next_rand() -> u64 {
    unsafe {
        let mut x = RAND_SEED;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        RAND_SEED = x;
        x
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Random() -> i64 {
    ((next_rand() >> 16) & 0x7FFF) as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseGetRandSeed() -> i64 {
    unsafe { RAND_SEED as i64 }
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseSetRandSeed(seed: i64) {
    unsafe {
        RAND_SEED = seed as u64;
    }
}

// ---------------------------------------------------------------------------
// HiWord / LoWord
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn HiWord(x: i64) -> i16 {
    (x >> 16) as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn LoWord(x: i64) -> i16 {
    x as i16
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

/// Emplacement stable du « handle » de curseur : `GetCursor` renverra son
/// adresse, et `*GetCursor(id)` (8 octets) sera non-null pour `SetCursor`.
static mut CURSOR_HANDLE: Slot8 = Slot8([0; 8]);

#[unsafe(no_mangle)]
pub extern "C" fn GetCursor(_id: i16) -> Cursor {
    unsafe { core::ptr::addr_of_mut!(CURSOR_HANDLE) as *mut c_void }
}

// ---------------------------------------------------------------------------
// Mémoire
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn NewPtr(size: i64) -> Ptr {
    alloc_pixels(size.max(0) as usize)
}

#[unsafe(no_mangle)]
pub extern "C" fn NewPtrClear(size: i64) -> Ptr {
    alloc_pixels(size.max(0) as usize)
}

#[unsafe(no_mangle)]
pub extern "C" fn DisposPtr(p: Ptr) {
    free_pixels(p);
}

#[unsafe(no_mangle)]
pub extern "C" fn NewHandle(size: i64) -> Handle {
    let data: *mut c_void = if size > 0 {
        alloc_pixels(size as usize) as *mut c_void
    } else {
        std::ptr::null_mut()
    };
    let outer = alloc_pixels(8) as *mut *mut c_void;
    unsafe {
        *outer = data;
    }
    outer
}

#[unsafe(no_mangle)]
pub extern "C" fn NewHandleClear(size: i64) -> Handle {
    let h = NewHandle(size);
    if !h.is_null() {
        unsafe {
            let inner = *h;
            if !inner.is_null() {
                let len = GetHandleSize(h);
                std::ptr::write_bytes(inner as *mut u8, 0, len.max(0) as usize);
            }
        }
    }
    h
}

#[unsafe(no_mangle)]
pub extern "C" fn SetHandleSize(h: Handle, size: i64) {
    if h.is_null() {
        return;
    }
    unsafe {
        let old = *h;
        if size <= 0 {
            if !old.is_null() {
                free_pixels(old as *mut u8);
            }
            *h = std::ptr::null_mut();
            return;
        }
        let nlen = size as usize;
        let new_ = alloc_pixels(nlen) as *mut c_void;
        if let Some((_, old_len)) = ALLOCS
            .lock()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == old as usize)
        {
            let copy = (*old_len).min(nlen);
            std::ptr::copy_nonoverlapping(old as *const u8, new_ as *mut u8, copy);
        }
        if !old.is_null() {
            free_pixels(old as *mut u8);
        }
        *h = new_;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DisposHandle(h: Handle) {
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

#[unsafe(no_mangle)]
pub extern "C" fn GetHandleSize(h: Handle) -> i64 {
    if h.is_null() {
        return 0;
    }
    unsafe {
        let inner = *h;
        if inner.is_null() {
            return 0;
        }
        let allocs = ALLOCS.lock().unwrap();
        for (a, l) in allocs.iter() {
            if *a == inner as usize {
                return *l as i64;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn MemError() -> OSErr {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn MaxMem(grow: *mut i64) -> i64 {
    if !grow.is_null() {
        unsafe { *grow = 0 };
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn BlockMove(src: *const c_void, dest: *mut c_void, len: i64) {
    if src.is_null() || dest.is_null() || len <= 0 {
        return;
    }
    unsafe {
        std::ptr::copy(src as *const u8, dest as *mut u8, len as usize);
    }
}

// ---------------------------------------------------------------------------
// Chaînes Pascal
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn PtoCstr(str: StringPtr) -> *mut i8 {
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

#[unsafe(no_mangle)]
pub extern "C" fn CtoPstr(str: *mut i8) -> *mut u8 {
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

#[unsafe(no_mangle)]
pub extern "C" fn GetIndString(s: StringPtr, _id: i16, _index: i16) {
    if s.is_null() {
        return;
    }
    unsafe {
        *s = 0;
    }
}

// ---------------------------------------------------------------------------
// Fenêtre principale
// ---------------------------------------------------------------------------

fn create_port(bounds: Rect, depth: i16) -> *mut SapPort {
    let w = (bounds.right - bounds.left).max(0) as usize;
    let h = (bounds.bottom - bounds.top).max(0) as usize;
    let row_bytes = w as i16;
    let pixels = alloc_pixels(w * h);
    let port = Box::new(SapPort {
        bounds,
        row_bytes,
        depth,
        pixels,
        pixel_len: w * h,
        pen_loc: Point { v: 0, h: 0 },
        pen_size: Point { v: 1, h: 1 },
        pen_mode: PAT_COPY,
        fore_index: 0,
        back_index: 255,
        clip: bounds,
        open_rgn: 0,
        open_acc: Rect::default(),
        tx_font: 0,
        tx_size: 12,
        tx_face: 0,
    });
    Box::into_raw(port)
}

#[unsafe(no_mangle)]
pub extern "C" fn NewCWindow(
    _storage: *mut c_void,
    bounds: *const Rect,
    _title: StringPtr,
    _visible: Boolean,
    _procID: i16,
    _behind: WindowPtr,
    _goAwayFlag: Boolean,
    _refCon: i64,
) -> WindowPtr {
    let existing = WINDOW_PORT.load(Ordering::SeqCst);
    if !existing.is_null() {
        return existing as *mut c_void;
    }
    let b = if bounds.is_null() {
        Rect {
            top: 0,
            left: 0,
            bottom: 400,
            right: 640,
        }
    } else {
        unsafe { *bounds }
    };
    let port = create_port(b, 8);
    WINDOW_PORT.store(port, Ordering::SeqCst);
    port as *mut c_void
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

/// Langue du jeu : la sélection (Auto/Fr/En/Es) est imposée côté Rust à
/// chaque frame ; cet appel écrit le global C `langue`, relance
/// `InitAndCheckTexts` et reconstruit la barre de menus (IDs langue).
pub fn set_game_language(l: Langue) {
    unsafe {
        langue = l as i16;
        InitAndCheckTexts();
    }
    *MENU_MODEL.lock().unwrap() = Some(build_model());
}

/// Valeur brute du global C `langue` (peut valoir -1 tant que le C n'a pas
/// choisi ; l'écran egui la corrige avec `set_game_language`).
pub fn game_lang_raw() -> Langue {
    unsafe { core::ptr::addr_of!(langue).read().into() }
}

fn current_lang() -> i16 {
    unsafe {
        let l = core::ptr::addr_of!(langue).read();
        if l == 1 || l == 2 { l } else { 0 }
    }
}

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
    fn label(&self, lang: i16) -> &'static str {
        match lang {
            0 => self.fr,
            2 => self.es,
            _ => self.gb,
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
struct MenuModel {
    /// Langue avec laquelle le modèle a été construit (0/1/2).
    lang: i16,
    menus: Vec<Box<Menu>>,
}

static MENU_MODEL: Mutex<Option<MenuModel>> = Mutex::new(None);

/// Emplacement stable du « handle » de barre de menus : `GetNewMBar` renvoie
/// son adresse (FixeMenu passera ce pointeur à `DisposHandle`, no-op sur un
/// entier de mémoire statique, d'où `Slot8` pour que le `*h` ne pante pas).
static mut MENU_BAR_HANDLE: Slot8 = Slot8([0; 8]);

fn fresh_menu(id_base: i16, title: L10n, lang: i16, items: &[(L10n, bool)]) -> Box<Menu> {
    Box::new(Menu {
        id: id_base + DELTA_LANGUE * lang,
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
    })
}

fn build_model() -> MenuModel {
    let lang = current_lang();
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

#[unsafe(no_mangle)]
pub extern "C" fn GetNewMBar(_menuBarID: i16) -> Handle {
    *MENU_MODEL.lock().unwrap() = Some(build_model());
    unsafe { core::ptr::addr_of_mut!(MENU_BAR_HANDLE) as *mut _ }
}

#[unsafe(no_mangle)]
pub extern "C" fn GetMHandle(menuID: i16) -> MenuHandle {
    let g = MENU_MODEL.lock().unwrap();
    match &*g {
        Some(m) => m
            .menus
            .iter()
            .find(|menu| menu.id == menuID)
            .map(|menu| menu_handle_ptr(menu))
            .unwrap_or(std::ptr::null_mut()),
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn SetMenuBar(_menuBar: Handle) {}

#[unsafe(no_mangle)]
pub extern "C" fn DrawMenuBar() {}

fn find_menu<'a>(menus: &'a mut [Box<Menu>], handle: MenuHandle) -> Option<&'a mut Menu> {
    if handle.is_null() {
        return None;
    }
    menus
        .iter_mut()
        .find(|menu| menu_handle_ptr(menu) == handle)
        .map(|menu| &mut **menu)
}

fn menu_at(menus: &[Box<Menu>], handle: MenuHandle) -> Option<&Menu> {
    if handle.is_null() {
        return None;
    }
    menus
        .iter()
        .find(|menu| menu_handle_ptr(menu) == handle)
        .map(|menu| &**menu)
}

fn handle_is_known(menus: &[Box<Menu>], handle: MenuHandle) -> bool {
    menu_at(menus, handle).is_some()
}

fn label_for(menus: &[Box<Menu>], handle: MenuHandle, item: i16) -> Option<&'static str> {
    let lang = current_lang();
    let menu = menu_at(menus, handle)?;
    if item <= 0 {
        Some(menu.title.label(lang))
    } else {
        menu.items
            .get((item - 1) as usize)
            .map(|it| it.l.label(lang))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DisableItem(menu: MenuHandle, item: i16) {
    if let Some(m) = &mut *MENU_MODEL.lock().unwrap() {
        if let Some(menu) = find_menu(&mut m.menus, menu) {
            if item == 0 {
                menu.enabled = false;
            } else if let Some(it) = menu.items.get_mut((item - 1) as usize) {
                it.enabled = false;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn EnableItem(menu: MenuHandle, item: i16) {
    if let Some(m) = &mut *MENU_MODEL.lock().unwrap() {
        if let Some(menu) = find_menu(&mut m.menus, menu) {
            if item == 0 {
                menu.enabled = true;
            } else if let Some(it) = menu.items.get_mut((item - 1) as usize) {
                it.enabled = true;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn CheckItem(menu: MenuHandle, item: i16, checked: Boolean) {
    if let Some(m) = &mut *MENU_MODEL.lock().unwrap() {
        if let Some(menu) = find_menu(&mut m.menus, menu) {
            if let Some(it) = menu.items.get_mut((item - 1) as usize) {
                it.checked = checked;
            }
        }
    }
}

/// `GetItem` remplit le tampon Pascal avec le libellé courant de l'item
/// (`SAPWIND.C` l'utilise pour les noms de desk accessory de l'Apple menu).
#[unsafe(no_mangle)]
pub extern "C" fn GetItem(menu: MenuHandle, item: i16, text: StringPtr) {
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

pub struct ItemSnapshot {
    pub index: i16,
    pub text: String,
    pub sep: bool,
    pub enabled: bool,
    pub checked: bool,
}

pub struct MenuSnapshot {
    pub base_id: i16,
    pub id: i16,
    pub title: String,
    pub enabled: bool,
    pub items: Vec<ItemSnapshot>,
}

/// Recopie l'état courant de la barre de menus (libellés et état) pour egui.
/// `id` est l'ID de menu « runtime » (avec l'offset langue) dont on a besoin
/// pour construire `menuID<<16|item` ; `base_id` sert à retrouver un menu
/// indépendamment de la langue (raccourcis clavier).
pub fn menus_snapshot() -> Vec<MenuSnapshot> {
    let lang = current_lang();
    let mut g = MENU_MODEL.lock().unwrap();
    // Si la langue C a changé ailleurs (ChargePref/Alert), on reconstruit le
    // modèle pour garder IDs de menus et libellés cohérents.
    if let Some(m) = g.as_mut() {
        if m.lang != lang {
            *m = build_model();
        }
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
// Ports offscreen / fenêtre
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn AcamCreatePortOffscreen(taillex: i16, tailley: i16, prof: i16) -> CGrafPtr {
    let b = Rect {
        top: 0,
        left: 0,
        bottom: tailley,
        right: taillex,
    };
    create_port(b, prof) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn AcamClosePortOffscreen(port: CGrafPtr) {
    if port.is_null() {
        return;
    }
    let pp = port as *mut SapPort;
    if pp == WINDOW_PORT.load(Ordering::SeqCst) {
        return;
    }
    unsafe {
        let p = &mut *pp;
        free_pixels(p.pixels);
        drop(Box::from_raw(pp));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn AcamGetPortOffscreen(port: *mut CGrafPtr) {
    if !port.is_null() {
        unsafe {
            *port = CURRENT_PORT.load(Ordering::SeqCst) as *mut c_void;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn AcamSetPortOffscreen(port: CGrafPtr) {
    CURRENT_PORT.store(port as *mut SapPort, Ordering::SeqCst);
}

#[unsafe(no_mangle)]
pub extern "C" fn AcamInitOffscreenSystem(r: *mut Rect) -> i16 {
    if !r.is_null() {
        unsafe {
            *r = Rect {
                top: 0,
                left: 0,
                bottom: 400,
                right: 640,
            };
        }
    }
    640
}

#[unsafe(no_mangle)]
pub extern "C" fn AcamGetInfoOffscreen(
    port: CGrafPtr,
    r: *mut Rect,
    rowBytes: *mut i16,
    depth: *mut i16,
) -> *mut c_void {
    if port.is_null() {
        return std::ptr::null_mut();
    }
    let p = port as *mut SapPort;
    unsafe {
        let pp = &*p;
        if !r.is_null() {
            *r = pp.bounds;
        }
        if !rowBytes.is_null() {
            *rowBytes = pp.row_bytes;
        }
        if !depth.is_null() {
            *depth = pp.depth;
        }
        pp.pixels as *mut c_void
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseGetPortBitMapForCopyBits(port: CGrafPtr) -> *mut c_void {
    port
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseGetCurrentPort() -> CGrafPtr {
    CURRENT_PORT.load(Ordering::SeqCst) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseSetPortWindow(w: WindowPtr) {
    CURRENT_PORT.store(w as *mut SapPort, Ordering::SeqCst);
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseGetWindowPort(w: WindowPtr) -> WindowPtr {
    w
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseGetRectPort(port: CGrafPtr, r: *mut Rect) {
    if port.is_null() || r.is_null() {
        return;
    }
    unsafe {
        *r = (*(port as *mut SapPort)).bounds;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseGetQDRectScreenBits(r: *mut Rect) {
    if !r.is_null() {
        unsafe {
            *r = Rect {
                top: 0,
                left: 0,
                bottom: 400,
                right: 640,
            };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn CarboniseInitQuickdDraw() {}

// ---------------------------------------------------------------------------
// Primitives graphiques
// ---------------------------------------------------------------------------

#[inline]
pub(crate) fn cur() -> *mut SapPort {
    CURRENT_PORT.load(Ordering::SeqCst)
}

/// Trace un pixel plein (`mode` tenu en compte), avec clip.
#[inline]
pub(crate) unsafe fn plot(port: *mut SapPort, x: i16, y: i16, c: u8) {
    let p = &mut *port;
    if x < p.clip.left || x >= p.clip.right || y < p.clip.top || y >= p.clip.bottom {
        return;
    }
    let w = p.row_bytes as usize;
    let off = (y as usize) * w + (x as usize);
    if off >= p.pixel_len {
        return;
    }
    let d = p.pixels.add(off);
    *d = match p.pen_mode {
        PAT_OR => *d | c,
        PAT_XOR => *d ^ c,
        PAT_BIC => *d & !c,
        _ => c,
    };
}

/// Remplit un rectangle (respecte clip), et accumule dans la région ouverte.
unsafe fn fill_rect(port: *mut SapPort, r: Rect, color: u8) {
    let p = &mut *port;
    p.open_acc = p.open_acc.union(&r);
    let c = r.intersect(&p.clip);
    if c.is_empty() {
        return;
    }
    let w = p.row_bytes as usize;
    let base = p.pixels;
    if p.pen_mode == PAT_COPY {
        let wpx = (c.right - c.left) as usize;
        for y in c.top..c.bottom {
            let row = base.add((y as usize) * w + (c.left as usize));
            std::ptr::write_bytes(row, color, wpx);
        }
    } else {
        for y in c.top..c.bottom {
            let row = base.add((y as usize) * w);
            for x in c.left..c.right {
                let d = row.add((x - c.left) as usize);
                let v = match p.pen_mode {
                    PAT_OR => *d | color,
                    PAT_XOR => *d ^ color,
                    PAT_BIC => *d & !color,
                    _ => color,
                };
                *d = v;
            }
        }
    }
}

/// Trace un segment de droite épais (taille du crayon), avec clip.
unsafe fn draw_line(port: *mut SapPort, x0: i16, y0: i16, x1: i16, y1: i16) {
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
                plot(port, px as i16, py as i16, c);
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

#[unsafe(no_mangle)]
pub extern "C" fn MoveTo(h: i16, v: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).pen_loc = Point { v, h };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn LineTo(h: i16, v: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        let loc = (*p).pen_loc;
        draw_line(p, loc.h, loc.v, h, v);
        (*p).pen_loc = Point { v, h };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Line(dh: i16, dv: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        let loc = (*p).pen_loc;
        let h = loc.h.wrapping_add(dh);
        let v = loc.v.wrapping_add(dv);
        draw_line(p, loc.h, loc.v, h, v);
        (*p).pen_loc = Point { v, h };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn PenSize(width: i16, height: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).pen_size = Point {
            h: width.max(1),
            v: height.max(1),
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn PenMode(mode: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).pen_mode = mode;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn PenNormal() {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).pen_size = Point { v: 1, h: 1 };
        (*p).pen_mode = PAT_COPY;
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

#[unsafe(no_mangle)]
pub extern "C" fn ForeColor(color: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).fore_index = color_num_to_index(color);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn BackColor(color: i16) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).back_index = color_num_to_index(color);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RGBForeColor(c: *const RGBColor) {
    let p = cur();
    if p.is_null() || c.is_null() {
        return;
    }
    unsafe {
        let cc = &*c;
        (*p).fore_index = rgb_to_index(
            (cc.red >> 8) as u8,
            (cc.green >> 8) as u8,
            (cc.blue >> 8) as u8,
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Index2Color(index: i16, color: *mut RGBColor) {
    if color.is_null() {
        return;
    }
    let i = index.clamp(0, 255) as usize;
    let (r, g, b) = lut_rgb(i as u8);
    unsafe {
        (*color).red = ((r as u16) << 8) | r as u16;
        (*color).green = ((g as u16) << 8) | g as u16;
        (*color).blue = ((b as u16) << 8) | b as u16;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Color2Index(c: *const RGBColor) -> i16 {
    if c.is_null() {
        return 0;
    }
    unsafe {
        let cc = &*c;
        rgb_to_index(
            (cc.red >> 8) as u8,
            (cc.green >> 8) as u8,
            (cc.blue >> 8) as u8,
        ) as i16
    }
}

// ---------------------------------------------------------------------------
// Rectangles
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn SetRect(r: *mut Rect, left: i16, top: i16, right: i16, bottom: i16) {
    if r.is_null() {
        return;
    }
    unsafe {
        *r = Rect {
            top,
            left,
            bottom,
            right,
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn OffsetRect(r: *mut Rect, dh: i16, dv: i16) {
    if r.is_null() {
        return;
    }
    unsafe {
        (*r).left = (*r).left.wrapping_add(dh);
        (*r).right = (*r).right.wrapping_add(dh);
        (*r).top = (*r).top.wrapping_add(dv);
        (*r).bottom = (*r).bottom.wrapping_add(dv);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn InsetRect(r: *mut Rect, dh: i16, dv: i16) {
    if r.is_null() {
        return;
    }
    unsafe {
        (*r).left = (*r).left.wrapping_add(dh);
        (*r).right = (*r).right.wrapping_sub(dh);
        (*r).top = (*r).top.wrapping_add(dv);
        (*r).bottom = (*r).bottom.wrapping_sub(dv);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn UnionRect(src1: *const Rect, src2: *const Rect, dst: *mut Rect) {
    if src1.is_null() || src2.is_null() || dst.is_null() {
        return;
    }
    unsafe {
        *dst = (*src1).union(&*src2);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn SectRect(src1: *const Rect, src2: *const Rect, dst: *mut Rect) -> Boolean {
    if src1.is_null() || src2.is_null() || dst.is_null() {
        return false;
    }
    unsafe {
        *dst = (*src1).intersect(&*src2);
        if dst.is_null() {
            false
        } else {
            !(*dst).is_empty() as Boolean
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn PtInRect(pt: Point, r: *const Rect) -> Boolean {
    if r.is_null() {
        return false;
    }
    unsafe {
        let rr = &*r;
        (pt.h >= rr.left && pt.h < rr.right && pt.v >= rr.top && pt.v < rr.bottom) as Boolean
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn EmptyRect(r: *const Rect) -> Boolean {
    if r.is_null() {
        return true;
    }
    unsafe { (*r).is_empty() as Boolean }
}

#[unsafe(no_mangle)]
pub extern "C" fn SetPt(pt: *mut Point, h: i16, v: i16) {
    if pt.is_null() {
        return;
    }
    unsafe {
        *pt = Point { v, h };
    }
}

// ---------------------------------------------------------------------------
// Remplissage / contour
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn PaintRect(r: *const Rect) {
    let p = cur();
    if p.is_null() || r.is_null() {
        return;
    }
    unsafe {
        let fore = (*p).fore_index;
        fill_rect(p, *r, fore);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn EraseRect(r: *const Rect) {
    let p = cur();
    if p.is_null() || r.is_null() {
        return;
    }
    unsafe {
        let back = (*p).back_index;
        fill_rect(p, *r, back);
    }
}

unsafe fn frame_rect(port: *mut SapPort, r: &Rect) {
    let p = &mut *port;
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
        plot(port, x, r.top, c);
        if bottom >= r.top {
            plot(port, x, bottom, c);
        }
    }
    for y in r.top..r.bottom {
        plot(port, r.left, y, c);
        if right >= r.left {
            plot(port, right, y, c);
        }
    }
}

#[inline]
fn clip_overflow_bound() -> i16 {
    32760
}

#[unsafe(no_mangle)]
pub extern "C" fn FrameRect(r: *mut Rect) {
    let p = cur();
    if p.is_null() || r.is_null() {
        return;
    }
    unsafe {
        frame_rect(p, &*r);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn FrameRoundRect(r: *const Rect, _ovalWidth: i16, _ovalHeight: i16) {
    let p = cur();
    if p.is_null() || r.is_null() {
        return;
    }
    unsafe {
        frame_rect(p, &*r);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ClipRect(r: *const Rect) {
    let p = cur();
    if p.is_null() || r.is_null() {
        return;
    }
    unsafe {
        let b = (*p).bounds;
        (*p).clip = (*r).intersect(&b);
    }
}

// ---------------------------------------------------------------------------
// CopyBits / ScrollRect
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn CopyBits(
    srcBits: *const c_void,
    dstBits: *const c_void,
    srcRect: *const Rect,
    dstRect: *const Rect,
    mode: i16,
    _maskRgn: RgnHandle,
) {
    if srcBits.is_null() || dstBits.is_null() || srcRect.is_null() || dstRect.is_null() {
        return;
    }
    let src = srcBits as *mut SapPort;
    let dst = dstBits as *mut SapPort;
    unsafe {
        let sr = &*srcRect;
        let dr = &*dstRect;
        let dp = &mut *dst;
        dp.open_acc = dp.open_acc.union(dr);
        let dclip = dr.intersect(&dp.clip);
        if sr.is_empty() || dclip.is_empty() {
            return;
        }
        let sw = sr.right - sr.left;
        let sh = sr.bottom - sr.top;
        let dw = dr.right - dr.left;
        let dh = dr.bottom - dr.top;
        let sp = &*src;
        let wd = dp.row_bytes as usize;
        let ws = sp.row_bytes as usize;
        let ob = dp.bounds;
        let sb = sp.bounds;
        if sp.pixels.is_null() || dp.pixels.is_null() {
            return;
        }

        // Cas identiques : copie par lignes (pointer overlap-safe).
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
                let srcrow = sp
                    .pixels
                    .add((srcy as usize) * ws + (srcx - sb.left) as usize);
                let dstrow = dp
                    .pixels
                    .add((dsty as usize) * wd + (start - ob.left) as usize);
                if mode == 10 {
                    for k in 0..n {
                        *dstrow.add(k) ^= *srcrow.add(k);
                    }
                } else if mode == 9 {
                    for k in 0..n {
                        *dstrow.add(k) |= *srcrow.add(k);
                    }
                } else if mode == 11 {
                    for k in 0..n {
                        *dstrow.add(k) &= !*srcrow.add(k);
                    }
                } else {
                    std::ptr::copy(srcrow, dstrow, n);
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
            let srcrow = sp.pixels.add((srcy as usize) * ws);
            let dstrow = dp.pixels.add((dsty as usize) * wd);
            for dx in dclip.left..dclip.right {
                let sx = sr.left
                    + (((dx as i32 - dr.left as i32) * sw as i32) / dw.max(1) as i32) as i16;
                if sx < sr.left || sx >= sr.right {
                    continue;
                }
                if sx < sb.left || sx >= sb.right {
                    continue;
                }
                let v = *srcrow.add((sx - sb.left) as usize);
                let d = dstrow.add((dx - ob.left) as usize);
                *d = match mode {
                    10 => *d ^ v,
                    9 => *d | v,
                    11 => *d & !v,
                    _ => v,
                };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ScrollRect(r: *const Rect, dh: i16, dv: i16, updateRgn: RgnHandle) {
    if r.is_null() {
        return;
    }
    let p = cur();
    if p.is_null() {
        return;
    }
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
            CopyBits(
                p as *const c_void,
                p as *const c_void,
                &clipped,
                &clipped,
                0,
                std::ptr::null_mut(),
            );
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

#[unsafe(no_mangle)]
pub extern "C" fn NewRgn() -> RgnHandle {
    let r = Box::new(Rect::default());
    Box::into_raw(r) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn RectRgn(rgn: RgnHandle, r: *const Rect) {
    if rgn.is_null() {
        return;
    }
    unsafe {
        if r.is_null() {
            *(rgn as *mut Rect) = Rect::default();
        } else {
            *(rgn as *mut Rect) = *r;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn OpenRgn() {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        (*p).open_rgn = 1;
        (*p).open_acc = Rect::default();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn CloseRgn(rgn: RgnHandle) {
    let p = cur();
    if p.is_null() {
        return;
    }
    unsafe {
        let acc = (*p).open_acc;
        (*p).open_rgn = 0;
        (*p).open_acc = Rect::default();
        if !rgn.is_null() {
            *(rgn as *mut Rect) = acc;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DiffRgn(a: RgnHandle, b: RgnHandle, dst: RgnHandle) {
    if dst.is_null() {
        return;
    }
    unsafe {
        let ra = if a.is_null() {
            Rect::default()
        } else {
            *(a as *const Rect)
        };
        let rb = if b.is_null() {
            Rect::default()
        } else {
            *(b as *const Rect)
        };
        let out = diff_rect(&ra, &rb);
        *(dst as *mut Rect) = out;
    }
}

fn diff_rect(ra: &Rect, rb: &Rect) -> Rect {
    if rb.is_empty() {
        return *ra;
    }
    if ra.is_empty() {
        return Rect::default();
    }
    if rb.left <= ra.left && rb.right >= ra.right {
        // Bande horizontale pleine largeur.
        if rb.top <= ra.top {
            if rb.bottom >= ra.bottom {
                return Rect::default();
            }
            return Rect {
                top: rb.bottom,
                left: ra.left,
                bottom: ra.bottom,
                right: ra.right,
            };
        }
        return Rect {
            top: ra.top,
            left: ra.left,
            bottom: rb.top,
            right: ra.right,
        };
    }
    if rb.top <= ra.top && rb.bottom >= ra.bottom {
        // Bande verticale pleine hauteur.
        if rb.left <= ra.left {
            if rb.right >= ra.right {
                return Rect::default();
            }
            return Rect {
                top: ra.top,
                left: rb.right,
                bottom: ra.bottom,
                right: ra.right,
            };
        }
        return Rect {
            top: ra.top,
            left: ra.left,
            bottom: ra.bottom,
            right: rb.left,
        };
    }
    // Cas général : on garde la partie haute.
    *ra
}

#[unsafe(no_mangle)]
pub extern "C" fn GetClip(rgn: RgnHandle) {
    let p = cur();
    if p.is_null() || rgn.is_null() {
        return;
    }
    unsafe {
        *(rgn as *mut Rect) = (*p).clip;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn SetClip(rgn: RgnHandle) {
    let p = cur();
    if p.is_null() || rgn.is_null() {
        return;
    }
    unsafe {
        let b = (*p).bounds;
        (*p).clip = (*(rgn as *const Rect)).intersect(&b);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn EraseRgn(rgn: RgnHandle) {
    let p = cur();
    if p.is_null() || rgn.is_null() {
        return;
    }
    unsafe {
        let back = (*p).back_index;
        fill_rect(p, *(rgn as *const Rect), back);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn FrameRgn(rgn: RgnHandle) {
    let p = cur();
    if p.is_null() || rgn.is_null() {
        return;
    }
    unsafe {
        frame_rect(p, &*(rgn as *const Rect));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DisposeRgn(rgn: RgnHandle) {
    if rgn.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(rgn as *mut Rect));
    }
}
