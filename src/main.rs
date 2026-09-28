//! Lanceur du portage : ouvre la fenêtre egui qui affiche le port de dessin
//! de la fenêtre Sapiens, démarre le code C (`sapiens_main`) dans un thread
//! et un thread qui égrène `sapiensTimer` toutes les 11 ms (comme le timer
//! du Mac d'origine).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) mod acam;
pub(crate) mod cbridge;
mod exchange;
pub(crate) mod langue;
pub(crate) mod port;
pub(crate) mod sapiens;
pub(crate) mod shared;
pub(crate) mod stubs;
pub(crate) mod text;

use crate::acam::Random;
use crate::exchange::{Cursor, get_cursor, set_menu_command};
use crate::langue::{LangMode, Langue, effective_lang, load_lang_mode, save_lang_mode};
use crate::sapiens::sap3d::Sap3d;
use crate::sapiens::sapbonc::Sapbonc;
use crate::sapiens::sapcomb::Sapcomb;
use crate::sapiens::sapenreg::Sapenreg;
use crate::sapiens::sapgest::Sapgest;
use crate::sapiens::sapglob::Sapglob;
use crate::sapiens::saplist::Saplist;
use crate::sapiens::saprel::Saprel;
use crate::sapiens::sapscrollbar::Sapscrollbar;
use crate::sapiens::saptxt::Saptxt;
use crate::sapiens::saptxt_es::Saptxt_es;
use crate::sapiens::saptxt_fr::Saptxt_fr;
use crate::sapiens::saptxt_gb::Saptxt_gb;
use crate::sapiens::sapw3d::Sapw3d;
use crate::sapiens::sapwarb::Sapwarb;
use crate::sapiens::sapwbrt::Sapwbrt;
use crate::sapiens::sapwcar::Sapwcar;
use crate::sapiens::sapwcart::Sapwcart;
use crate::sapiens::sapwclav::{Sapwclav, Souris};
use crate::sapiens::sapwcvt::Sapwcvt;
use crate::sapiens::sapwind::SapWind;
use crate::sapiens::sapwmess::Sapwmess;
use crate::sapiens::sapwmus::Sapwmus;
use eframe::egui;
use egui::CursorIcon;
use shared::{INPUT, SapPort, WINDOW_PORT, lut_rgb};

fn main() {
    // Le code C ouvre les fichiers relatifs (data/mobs/*.BRT) dans le répertoire
    // courant ; on se place donc sous data/mobs.
    if let Ok(md) = std::env::var("CARGO_MANIFEST_DIR") {
        let data = std::path::Path::new(&md).join("data").join("mobs");
        if std::env::set_current_dir(&data).is_err() {
            eprintln!(
                "AVERTISSEMENT: impossible de changer de répertoire vers {:?}",
                data
            );
        }
    } else {
        eprintln!("AVERTISSEMENT: CARGO_MANIFEST_DIR absent, chemins des données non résolus");
    }

    // Langue : mode choisi (Auto = langue du système), imposé au C avant que
    // le thread jeu ne choisisse via son Alert (SAPWIND.C).
    let lang_mode = load_lang_mode();
    acam::set_game_language(effective_lang(lang_mode));

    // Compteur 11 ms pour sapiensTimer (SAPWCLAV.C).
    let sapiens_timer = Arc::new(AtomicU64::new(0));
    std::thread::spawn({
        let t = sapiens_timer.clone();
        move || timer_loop(t)
    });

    // Le code C (boucle infinie du jeu original).
    std::thread::Builder::new()
        .name("sapiens_c".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| unsafe {
            // Zoom natif de la version 1996 : la fenêtre du jeu fait alors
            // exactement 640x400 et son contenu occupe tout le buffer. Avec
            // SapZoom=2 l'écran était 1280x800 mais le champ du jeu restait
            // en 640x400, d'où un rendu décalé.
            let mut sapwind = SapWind::default();
            sapwind.set_zoom(1.0);
            // Pointeurs auto-reférents de SAPGLOB.C (debban, peuple, soma...).
            let mut sapiens = Sapiens {
                sapwclav: Sapwclav::with_sapiens_timer(sapiens_timer),
                sapwind,
                ..Default::default()
            };
            sapiens.init_glob();
            sapiens.sapiens_main();
        })
        .expect("échec du thread C");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 422.0])
            .with_title("Sapiens"),
        ..Default::default()
    };
    let result = eframe::run_native(
        "Sapiens",
        native_options,
        Box::new(move |_cc| Ok(Box::new(SapiensApp::new(lang_mode)))),
    );
    if let Err(e) = result {
        eprintln!("eframe: {e}");
    }
    // La fenêtre est fermée : on quitte (le thread C est une boucle infinie).
    std::process::exit(0);
}

fn timer_loop(t: Arc<AtomicU64>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(11));
        t.fetch_add(1, Ordering::Relaxed);
    }
}

/// Sous-menu « Fichier → Langue » : Auto / Français / Anglais / Espagnol.
fn render_language_menu(ui: &mut egui::Ui, mode: &mut LangMode) {
    let lang = acam::game_lang_raw();
    let title = match lang {
        Langue::FR => "Langue",
        Langue::ES => "Idioma",
        _ => "Language",
    };
    egui::menu::menu_button(ui, title, |ui| {
        for opt in [LangMode::Auto, LangMode::Fr, LangMode::En, LangMode::Es] {
            if ui.selectable_label(*mode == opt, opt.label(lang)).clicked() {
                *mode = opt;
                acam::set_game_language(effective_lang(opt));
                save_lang_mode(opt);
                ui.close_menu();
            }
        }
    });
}

#[derive(Default)]
struct Sapiens {
    souris: Souris,
    sapwclav: Sapwclav,
    sapwind: SapWind,
    sapcomb: Sapcomb,
    sapglob: Sapglob,
    sap3d: Sap3d,
    sapw3d: Sapw3d,
    sapwbrt: Sapwbrt,
    random: Random,
    sapbonc: Sapbonc,
    saplist: Saplist,
    sapwmus: Sapwmus,
    sapwarb: Sapwarb,
    sapwcar: Sapwcar,
    sapwcart: Sapwcart,
    saprel: Saprel,
    sapscrollbar: Sapscrollbar,
    sapwmess: Sapwmess,
    sapgest: Sapgest,
    sapenreg: Sapenreg,
    sapwcvt: Sapwcvt,
    saptxt: Saptxt,
    saptxt_fr: Saptxt_fr,
    saptxt_gb: Saptxt_gb,
    saptxt_es: Saptxt_es,
}

struct SapiensApp {
    tex: Option<egui::TextureHandle>,
    dump: Option<PortDumper>,
    lang_mode: LangMode,
}

impl SapiensApp {
    fn new(lang_mode: LangMode) -> Self {
        Self {
            tex: None,
            dump: PortDumper::from_env(),
            lang_mode,
        }
    }
}

impl From<Cursor> for CursorIcon {
    fn from(value: Cursor) -> Self {
        match value {
            Cursor::Hidden => CursorIcon::None,
            Cursor::Default => CursorIcon::Default,
        }
    }
}

impl eframe::App for SapiensApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_cursor_icon(get_cursor().into());
        // Le Rust est autoritaire sur la langue : on réimpose la sélection
        // (un `Sapiens.cfg` chargé par ChargePref pourrait la remettre en
        // cause au démarrage).
        let effective = effective_lang(self.lang_mode);
        if acam::game_lang_raw() != effective {
            acam::set_game_language(effective);
        }

        // Raccourcis clavier (équivalents MenuKey) avant la barre de menus.
        handle_shortcuts(ctx);

        // Barre de menus Mac reconstituée (modèle → src/acam.rs).
        let menus = acam::menus_snapshot();
        let mut command: Option<i32> = None;
        egui::TopBottomPanel::top("menus").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                for menu in &menus {
                    if menu.enabled {
                        egui::menu::menu_button(ui, &menu.title, |ui| {
                            for item in &menu.items {
                                if item.sep {
                                    // Dans le menu Fichier, insère le choix de
                                    // langue entre « Préférences… » et « Quitter ».
                                    if menu.base_id == 129 && item.index == 6 {
                                        ui.separator();
                                        render_language_menu(ui, &mut self.lang_mode);
                                        ui.separator();
                                    } else {
                                        ui.separator();
                                    }
                                } else {
                                    let label = if item.checked {
                                        format!("✓ {}", item.text)
                                    } else {
                                        item.text.clone()
                                    };
                                    if ui
                                        .add_enabled(item.enabled, egui::Button::new(label))
                                        .clicked()
                                    {
                                        command = Some(menu_result(menu.id, item.index));
                                        ui.close_menu();
                                    }
                                }
                            }
                        });
                    } else {
                        ui.add_enabled(false, egui::Button::new(&menu.title));
                    }
                }
            });
        });
        if let Some(r) = command {
            set_menu_command(r);
        }

        record_input(ctx);

        let window = WINDOW_PORT.load(Ordering::SeqCst);
        if window.is_null() {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
                .show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            egui::RichText::new("Initialisation de Sapiens…")
                                .color(egui::Color32::WHITE),
                        );
                    });
                });
            ctx.request_repaint_after(std::time::Duration::from_millis(10));
            return;
        }

        let img = port_image(window);
        if let Some(d) = &mut self.dump {
            d.tick(window);
        }
        // Ré-upload à chaque frame (remplace la texture du même nom).
        self.tex = Some(ctx.load_texture("sapiens", img, egui::TextureOptions::NEAREST));

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(self.tex.as_ref().unwrap()));
                });
            });
        // Rafraîchissement permanent (boucle de jeu C).
        ctx.request_repaint();
    }
}

/// Convertit l'intégralité du tampon d'indices du port fenêtre (1280×800) en
/// image egui (via la LUT), réduite dans la fenêtre affichée 640×400.
fn port_image(p: *mut SapPort) -> egui::ColorImage {
    unsafe {
        let pp = &*p;
        let w = (pp.bounds.right - pp.bounds.left).max(0) as usize;
        let h = (pp.bounds.bottom - pp.bounds.top).max(0) as usize;
        let row = pp.row_bytes.max(0) as usize;
        const CW: usize = 640;
        const CH: usize = 400;
        let mut pixels = Vec::with_capacity(CW * CH);
        for y in 0..CH {
            let sy = (y * h / CH).min(h.saturating_sub(1));
            let rowp = pp.pixels.add(sy * row);
            for x in 0..CW {
                let sx = (x * w / CW).min(w.saturating_sub(1));
                let idx = *rowp.add(sx);
                let (r, g, b) = lut_rgb(idx);
                pixels.push(egui::Color32::from_rgb(r, g, b));
            }
        }
        egui::ColorImage {
            size: [CW, CH],
            pixels,
        }
    }
}

/// Alimente l'état d'entrée partagé (utilisé par les stubs GetMouse/Button au
/// moment voulu). Les coordonnées sont normalisées dans la zone de jeu (sous la
/// barre de menus), pas dans toute la fenêtre.
fn record_input(ctx: &egui::Context) {
    let area = ctx.available_rect();
    let (mh, mv) = ctx
        .pointer_interact_pos()
        .map(|pos| {
            (
                ((pos.x - area.left()) / area.width().max(1.0)).clamp(0.0, 1.0),
                ((pos.y - area.top()) / area.height().max(1.0)).clamp(0.0, 1.0),
            )
        })
        .unwrap_or((0.5, 0.5));
    let (button, keymap) = ctx.input(|i| {
        let mut km = [0u64; 4];
        for &k in &i.keys_down {
            if let Some(code) = mac_keycode(k) {
                km_set_key(&mut km, code);
            }
        }
        if i.modifiers.shift {
            km_set_key(&mut km, 0x38);
        }
        if i.modifiers.ctrl {
            km_set_key(&mut km, 0x3B);
        }
        if i.modifiers.alt {
            km_set_key(&mut km, 0x3A);
        }
        if i.modifiers.command {
            km_set_key(&mut km, 0x37);
        }
        (i.pointer.primary_down(), km)
    });
    {
        let mut st = INPUT.lock().unwrap();
        st.mouse_h = mh;
        st.mouse_v = mv;
        st.button = button;
        st.keymap = keymap;
    }
}

/// Résultat de menu attendu par `DoMenuCommand` : `menuID<<16|item`,
/// menuID étant l'ID runtime (avec l'offset langue, cf. `CvtDefaultLangRess`).
fn menu_result(menu_id: i16, item: i16) -> i32 {
    ((menu_id as i32) << 16) | (item as i32 & 0xFFFF)
}

/// Raccourcis clavier Cmd du Mac (équivalents `MenuKey`) : Cmd+N (Nouveau),
/// Cmd+O (Charger), Cmd+S (Sauver), Cmd+Q (Quitter), Cmd+/ (Aide).
fn handle_shortcuts(ctx: &egui::Context) {
    let (command, keys) = ctx.input(|i| {
        (
            i.modifiers.command,
            i.keys_down.iter().copied().collect::<Vec<_>>(),
        )
    });
    if !command || keys.is_empty() {
        return;
    }
    let menus = acam::menus_snapshot();
    // N° d'item par menu (base) : identiques à SAPWIND.C (`enum` + séparateurs).
    let mut trigger: Option<i32> = None;
    'keys: for k in keys {
        let (base, item) = match k {
            egui::Key::N => (129, 1),     // Nouveau
            egui::Key::O => (129, 2),     // Charger…
            egui::Key::S => (129, 3),     // Sauver…
            egui::Key::Q => (129, 7),     // Quitter
            egui::Key::Slash => (128, 2), // Aide
            _ => continue,
        };
        for menu in &menus {
            if menu.base_id == base {
                trigger = Some(menu_result(menu.id, item));
                break 'keys;
            }
        }
    }
    if let Some(r) = trigger {
        set_menu_command(r);
    }
}

/// Pose le bit du keycode Mac `code` dans les 4 mots de 64 bits alignés sur
/// `KeyMap` du C (bit `((code^24)&31)` du mot `code>>5`, cf. `KeyPressed`).
fn km_set_key(keymap: &mut [u64; 4], code: u8) {
    if code >= 128 {
        return;
    }
    let bit = (code ^ 24) & 31;
    keymap[(code >> 5) as usize] |= 1u64 << bit;
}

/// Traduit une touche logique egui vers son keycode virtuel Mac.
fn mac_keycode(k: egui::Key) -> Option<u8> {
    use egui::Key::*;
    Some(match k {
        // Commandes de jeu (SAPWCLAV.C).
        ArrowUp => 0x7E,
        ArrowDown => 0x7D,
        ArrowLeft => 0x7B,
        ArrowRight => 0x7C,
        Space => 0x31,
        Enter => 0x24,
        Escape => 0x35,
        C => 0x08,
        M => 0x2E,
        // Lettres.
        A => 0x00,
        B => 0x0B,
        D => 0x02,
        E => 0x0E,
        F => 0x03,
        G => 0x05,
        H => 0x04,
        I => 0x22,
        J => 0x26,
        K => 0x28,
        L => 0x25,
        N => 0x2D,
        O => 0x1F,
        P => 0x23,
        Q => 0x0C,
        R => 0x0F,
        S => 0x01,
        T => 0x11,
        U => 0x20,
        V => 0x09,
        W => 0x0D,
        X => 0x07,
        Y => 0x10,
        Z => 0x06,
        // Chiffres (rangée alphabétique).
        Num0 => 0x1D,
        Num1 => 0x12,
        Num2 => 0x13,
        Num3 => 0x14,
        Num4 => 0x15,
        Num5 => 0x17,
        Num6 => 0x16,
        Num7 => 0x1A,
        Num8 => 0x1C,
        Num9 => 0x19,
        // Ponctuation / divers.
        Period => 0x2F,
        Comma => 0x2B,
        Slash => 0x2C,
        Semicolon => 0x29,
        Minus => 0x1B,
        Equals => 0x18,
        Backtick => 0x32,
        OpenBracket => 0x21,
        CloseBracket => 0x1E,
        Backslash => 0x2A,
        Tab => 0x30,
        Backspace => 0x33,
        Delete => 0x75,
        Home => 0x73,
        End => 0x77,
        PageUp => 0x74,
        PageDown => 0x79,
        _ => return None,
    })
}

/// Débogage : si `SAPIENS_DUMP` est défini, écrit régulièrement le tampon de la
/// fenêtre sous forme d'image PPM (lisible ensuite avec `sips`/`ffmpeg`).
struct PortDumper {
    out: String,
    then: std::time::Instant,
}

impl PortDumper {
    fn from_env() -> Option<Self> {
        std::env::var("SAPIENS_DUMP").ok().map(|out| PortDumper {
            out,
            then: std::time::Instant::now(),
        })
    }

    fn tick(&mut self, p: *mut SapPort) {
        const INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);
        if self.then.elapsed() < INTERVAL {
            return;
        }
        self.then = std::time::Instant::now();
        unsafe {
            let pp = &*p;
            let w = (pp.bounds.right - pp.bounds.left).max(0) as usize;
            let h = (pp.bounds.bottom - pp.bounds.top).max(0) as usize;
            let row = pp.row_bytes.max(0) as usize;
            let mut ppm = Vec::with_capacity(12 + w * h * 3);
            ppm.extend_from_slice(format!("P6\n{w} {h}\n255\n").as_bytes());
            for y in 0..h {
                let rowp = pp.pixels.add(y * row);
                for x in 0..w {
                    let (r, g, b) = lut_rgb(*rowp.add(x));
                    ppm.push(r);
                    ppm.push(g);
                    ppm.push(b);
                }
            }
            let path = self.out.clone();
            match std::fs::write(&path, &ppm) {
                Ok(()) => eprintln!("dump -> {path} ({w}x{h})"),
                Err(e) => eprintln!("dump échec {path}: {e}"),
            }
        }
    }
}
