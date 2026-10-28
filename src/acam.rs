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

use crate::cbridge::{MenuHandle, WindowPtr, watchCursor};
use crate::exchange::{CursorShape, request_alert, set_cursor_shape, set_title};
use crate::langue::{Langue, langue};
use crate::port;
use crate::shared::*;
use crate::text::with_port;
use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub(crate) const VRAI: bool = true;
pub(crate) const FAUX: bool = false;

// ---------------------------------------------------------------------------
// Timer / aléa
// ---------------------------------------------------------------------------

static TICK_START: Mutex<Option<Instant>> = Mutex::new(None);

pub(crate) fn TickCount() -> i64 {
    let mut g = TICK_START.lock().unwrap();
    let start = *g.get_or_insert_with(Instant::now);
    start.elapsed().as_millis() as i64
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

#[unsafe(no_mangle)]
pub(crate) extern "C" fn GetCursor(id: i16) -> Cursor {
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

pub(crate) fn GetIndString(s: StringPtr, _id: i16, _index: i16) {
    if s.is_null() {
        return;
    }
    unsafe {
        *s = 0;
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

pub(crate) fn NewCWindow(
    _storage: *mut c_void,
    bounds: *const Rect,
    _title: StringPtr,
    _visible: Boolean,
    _procID: i16,
    _behind: WindowPtr,
    _goAwayFlag: Boolean,
    _refCon: i64,
) -> WindowPtr {
    let existing = port::window();
    if !existing.is_null() {
        return existing;
    }
    let b = if bounds.is_null() {
        port::SCREEN
    } else {
        unsafe { *bounds }
    };
    /* Le positionnement global de la fenêtre est géré par egui : le port de
    dessin travaille en coordonnées locales (0,0)-(largeur,hauteur). */
    let local = Rect::new(0, 0, b.right - b.left, b.bottom - b.top);
    let p = port::create(local, 8);
    WINDOW_PORT.store(p as *mut SapPort, Ordering::SeqCst);
    p
}

/// `SetWTitle` : le titre (chaîne Pascal MacRoman) part par le pont
/// `exchange.rs` ; c'est le thread egui qui l'applique à la fenêtre native
/// (`ViewportCommand::Title`), une seule fenêtre étant créée par `NewCWindow`.
pub(crate) fn SetWTitle(_w: WindowPtr, title: String) {
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

fn find_menu<'a>(menus: &'a mut [Menu], handle: MenuHandle) -> Option<&'a mut Menu> {
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

pub(crate) fn EnableItem(menu: MenuHandle, item: i16) {
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

pub(crate) fn CheckItem(menu: MenuHandle, item: i16, checked: Boolean) {
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
// Primitives graphiques
// ---------------------------------------------------------------------------

#[inline]
pub(crate) fn cur() -> *mut SapPort {
    port::current() as *mut SapPort
}

/// Trace un pixel plein (`mode` tenu en compte), avec clip.
#[inline]
pub(crate) fn plot(p: &mut SapPort, x: i16, y: i16, c: u8) {
    unsafe {
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
}

/// Remplit un rectangle (respecte clip), et accumule dans la région ouverte.
fn fill_rect(p: &mut SapPort, r: &Rect, color: u8) {
    p.open_acc = p.open_acc.union(&r);
    let c = r.intersect(&p.clip);
    if c.is_empty() {
        return;
    }
    let w = p.row_bytes as usize;
    let base = p.pixels;
    if p.pen_mode == PAT_COPY {
        let wpx = (c.right - c.left) as usize;
        unsafe {
            for y in c.top..c.bottom {
                let row = base.add((y as usize) * w + (c.left as usize));
                std::ptr::write_bytes(row, color, wpx);
            }
        }
    } else {
        for y in c.top..c.bottom {
            unsafe {
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

pub(crate) fn MoveTo(h: i16, v: i16) {
    with_port(|p| p.pen_loc = Point { v, h });
}

pub(crate) fn LineTo(h: i16, v: i16) {
    with_port(|p| {
        let loc = p.pen_loc;
        draw_line(p, loc.h, loc.v, h, v);
        p.pen_loc = Point { v, h };
    });
}

pub(crate) fn Line(dh: i16, dv: i16) {
    with_port(|p| {
        let loc = p.pen_loc;
        let h = loc.h.wrapping_add(dh);
        let v = loc.v.wrapping_add(dv);
        draw_line(p, loc.h, loc.v, h, v);
        p.pen_loc = Point { v, h };
    });
}

pub(crate) fn PenSize(width: i16, height: i16) {
    with_port(|p| {
        p.pen_size = Point {
            h: width.max(1),
            v: height.max(1),
        }
    });
}

pub(crate) fn PenMode(mode: i16) {
    with_port(|p| p.pen_mode = mode);
}

pub(crate) fn PenNormal() {
    with_port(|p| {
        p.pen_size = Point { v: 1, h: 1 };
        p.pen_mode = PAT_COPY;
    });
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

pub(crate) fn ForeColor(color: i16) {
    with_port(|p| p.fore_index = color_num_to_index(color));
}

pub(crate) fn BackColor(color: i16) {
    with_port(|p| p.back_index = color_num_to_index(color));
}

pub(crate) fn RGBForeColor(c: &RGBColor) {
    with_port(|p| {
        p.fore_index = rgb_to_index(
            (c.red >> 8) as u8,
            (c.green >> 8) as u8,
            (c.blue >> 8) as u8,
        )
    });
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

pub(crate) fn PaintRect(r: &Rect) {
    with_port(|p| {
        let fore = p.fore_index;
        fill_rect(&mut *p, r, fore);
    });
}

pub(crate) fn EraseRect(r: &Rect) {
    with_port(|p| {
        let back = p.back_index;
        fill_rect(p, r, back);
    });
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

pub(crate) fn FrameRect(r: &Rect) {
    with_port(|p| frame_rect(p, r));
}

pub(crate) fn FrameRoundRect(r: &Rect, _ovalWidth: i16, _ovalHeight: i16) {
    with_port(|p| frame_rect(p, r));
}

pub(crate) fn ClipRect(r: &Rect) {
    with_port(|p| {
        let b = p.bounds;
        p.clip = r.intersect(&b);
    });
}

// ---------------------------------------------------------------------------
// CopyBits / ScrollRect
// ---------------------------------------------------------------------------

pub(crate) fn CopyBits(
    srcBits: *const c_void,
    dstBits: *const c_void,
    srcRect: *const Rect,
    dstRect: *const Rect,
    mode: i16,
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

pub(crate) fn ScrollRect(r: &Rect, dh: i16, dv: i16, updateRgn: RgnHandle) {
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
pub(crate) fn OpenRgn() {
    with_port(|p| {
        p.open_rgn = 1;
        p.open_acc = Rect::default();
    });
}

pub(crate) fn CloseRgn() -> Option<Rect> {
    with_port(|p| {
        let acc = p.open_acc;
        p.open_rgn = 0;
        p.open_acc = Rect::default();
        acc
    })
}

pub(crate) fn GetClip() -> Option<Rect> {
    with_port(|p| p.clip)
}

pub(crate) fn SetClip(rgn: &Rect) {
    with_port(|p| {
        let b = p.bounds;
        p.clip = rgn.intersect(&b);
    });
}

pub(crate) fn EraseRgn(rgn: &Rect) {
    with_port(|p| {
        let back = p.back_index;
        fill_rect(p, rgn, back);
    });
}

pub(crate) fn FrameRgn(rgn: &Rect) {
    with_port(|p| frame_rect(p, rgn));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copybits_agrandit_sans_perdre_la_derniere_ligne() {
        let src = port::create(Rect::new(0, 0, 640, 400), 8);
        let dst = port::create(Rect::new(0, 0, 1280, 800), 8);
        let si = port::info(src).unwrap();
        let di = port::info(dst).unwrap();
        unsafe {
            for y in 0..400usize {
                for x in 0..640usize {
                    *si.pixels.add(y * 640 + x) = ((x * 7 + y * 13) & 0xFF) as u8;
                }
            }
            std::ptr::write_bytes(di.pixels, 0xAA, 1280 * 800);
        }

        CopyBits(
            src,
            dst,
            &Rect::new(0, 0, 640, 400),
            &Rect::new(0, 0, 1280, 800),
            0,
        );

        unsafe {
            for y in 0..800usize {
                for x in 0..1280usize {
                    let expected = (((x / 2) * 7 + (y / 2) * 13) & 0xFF) as u8;
                    assert_eq!(
                        *di.pixels.add(y * 1280 + x),
                        expected,
                        "pixel ({x}, {y}) mal mis à l'échelle"
                    );
                }
            }
        }

        port::close(src);
        port::close(dst);
    }
}
