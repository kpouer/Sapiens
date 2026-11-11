//! Couche de plate-forme : tout ce qui diffère entre le natif
//! (macOS/Windows/Linux) et le navigateur (`wasm32-unknown-unknown`).
//!
//! Sur le web, le jeu tourne dans un Web Worker à mémoire partagée
//! (`wasm_thread`) et egui sur le thread principal du navigateur. Ce dernier
//! n'a pas le droit de bloquer (`Atomics.wait` y est interdit) : les verrous
//! partagés avec l'UI sont donc des verrous à attente active (`sync::Mutex`),
//! et tout ce qui touche au DOM (`localStorage`, téléchargements) est confié à
//! l'UI via la file `web::ACTIONS`.

pub(crate) use web_time::Instant;

// ---------------------------------------------------------------------------
// Threads
// ---------------------------------------------------------------------------

/// Lance un thread nommé avec la pile demandée (Web Worker sur wasm).
pub(crate) fn spawn<F>(name: &str, stack_size: usize, f: F)
where
    F: FnOnce() + Send + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    let builder = std::thread::Builder::new();
    #[cfg(target_arch = "wasm32")]
    let builder = wasm_thread::Builder::new();
    builder
        .name(name.to_string())
        .stack_size(stack_size)
        .spawn(f)
        .unwrap_or_else(|e| panic!("échec du thread {name} : {e}"));
}

/// Fin du programme (`SapiensExit`). Sur le web il n'y a pas de processus à
/// quitter : on signale la fin à l'UI et on endort le thread du jeu.
pub(crate) fn quit(code: i32) -> ! {
    #[cfg(not(target_arch = "wasm32"))]
    std::process::exit(code);
    #[cfg(target_arch = "wasm32")]
    {
        log::info!("fin du jeu (code {code})");
        web::QUIT.store(true, std::sync::atomic::Ordering::SeqCst);
        loop {
            std::thread::park();
        }
    }
}

// ---------------------------------------------------------------------------
// Verrous partagés avec l'UI
// ---------------------------------------------------------------------------

pub(crate) mod sync {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use std::sync::Mutex;

    /// Verrou à attente active, même API que `std::sync::Mutex` (jamais
    /// empoisonné). Le thread principal du navigateur ne peut pas s'endormir
    /// sur un verrou contendu ; les sections critiques sont toutes courtes.
    #[cfg(target_arch = "wasm32")]
    pub(crate) struct Mutex<T: ?Sized>(spin::Mutex<T>);

    #[cfg(target_arch = "wasm32")]
    impl<T> Mutex<T> {
        pub(crate) const fn new(value: T) -> Self {
            Self(spin::Mutex::new(value))
        }
    }

    #[cfg(target_arch = "wasm32")]
    impl<T: ?Sized> Mutex<T> {
        pub(crate) fn lock(&self) -> std::sync::LockResult<spin::MutexGuard<'_, T>> {
            Ok(self.0.lock())
        }
    }
}

// ---------------------------------------------------------------------------
// Stockage clé/valeur persistant (préférences, langue)
// ---------------------------------------------------------------------------

pub(crate) mod storage {
    /// Lit une valeur persistée.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn read(key: &str) -> Option<Vec<u8>> {
        std::fs::read(crate::langue::config_dir().join(key)).ok()
    }

    /// Persiste une valeur.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn write(key: &str, bytes: &[u8]) -> std::io::Result<()> {
        let dir = crate::langue::config_dir();
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join(key), bytes)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn read(key: &str) -> Option<Vec<u8>> {
        super::web::STORE.lock().unwrap().get(key).cloned()
    }

    /// Mise à jour immédiate de la copie en mémoire ; l'écriture dans
    /// `localStorage` est faite par l'UI (inaccessible depuis un worker).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn write(key: &str, bytes: &[u8]) -> std::io::Result<()> {
        super::web::STORE
            .lock()
            .unwrap()
            .insert(key.to_string(), bytes.to_vec());
        super::web::push_action(super::web::WebAction::Persist(
            key.to_string(),
            bytes.to_vec(),
        ));
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Fichiers d'aventure (.sap)
// ---------------------------------------------------------------------------

pub(crate) mod fs {
    use std::path::Path;

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) type Reader = std::fs::File;
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) type Writer = std::fs::File;

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open(path: &Path) -> std::io::Result<Reader> {
        std::fs::File::open(path)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn create(path: &Path) -> std::io::Result<Writer> {
        std::fs::File::create(path)
    }

    /// Sur le web, les fichiers n'existent qu'en mémoire : un fichier chargé
    /// par l'utilisateur y est déposé par l'UI (`insert`), un fichier sauvé y
    /// est conservé et proposé au téléchargement.
    #[cfg(target_arch = "wasm32")]
    pub(crate) type Reader = std::io::Cursor<Vec<u8>>;

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn open(path: &Path) -> std::io::Result<Reader> {
        super::web::FILES
            .lock()
            .unwrap()
            .get(path)
            .cloned()
            .map(std::io::Cursor::new)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn insert(path: &Path, bytes: Vec<u8>) {
        super::web::FILES
            .lock()
            .unwrap()
            .insert(path.to_path_buf(), bytes);
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn create(path: &Path) -> std::io::Result<Writer> {
        Ok(Writer {
            path: path.to_path_buf(),
            buf: Vec::new(),
        })
    }

    /// Fichier en cours d'écriture : à la fermeture (`drop`), il est rangé en
    /// mémoire et téléchargé par le navigateur.
    #[cfg(target_arch = "wasm32")]
    pub(crate) struct Writer {
        path: std::path::PathBuf,
        buf: Vec<u8>,
    }

    #[cfg(target_arch = "wasm32")]
    impl std::io::Write for Writer {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.buf.extend_from_slice(data);
            Ok(data.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            insert(&self.path, self.buf.clone());
            Ok(())
        }
    }

    #[cfg(target_arch = "wasm32")]
    impl Drop for Writer {
        fn drop(&mut self) {
            let bytes = std::mem::take(&mut self.buf);
            insert(&self.path, bytes.clone());
            let name = self
                .path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Sapiens.sap".to_string());
            super::web::push_action(super::web::WebAction::Download(name, bytes));
        }
    }
}

// ---------------------------------------------------------------------------
// Navigateur : état partagé et actions DOM exécutées par l'UI
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
pub(crate) mod web {
    use super::sync::Mutex;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::LazyLock;
    use std::sync::atomic::AtomicBool;

    const PREFIX: &str = "sapiens:";

    /// Copie en mémoire du `localStorage` (clés sans préfixe).
    pub(crate) static STORE: LazyLock<Mutex<HashMap<String, Vec<u8>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    /// Système de fichiers en mémoire pour les aventures.
    pub(crate) static FILES: LazyLock<Mutex<HashMap<PathBuf, Vec<u8>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    /// Posé par `quit` : le jeu est terminé.
    pub(crate) static QUIT: AtomicBool = AtomicBool::new(false);

    /// Opérations DOM demandées par le thread du jeu.
    pub(crate) enum WebAction {
        Persist(String, Vec<u8>),
        Download(String, Vec<u8>),
        OpenUrl(String),
    }

    static ACTIONS: Mutex<Vec<WebAction>> = Mutex::new(Vec::new());

    pub(crate) fn push_action(action: WebAction) {
        ACTIONS.lock().unwrap().push(action);
    }

    fn local_storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok()?
    }

    /// Charge le `localStorage` en mémoire. À appeler sur le thread principal
    /// avant de lancer le jeu.
    pub(crate) fn preload_storage() {
        let Some(storage) = local_storage() else {
            return;
        };
        let mut store = STORE.lock().unwrap();
        let len = storage.length().unwrap_or(0);
        for i in 0..len {
            let Some(key) = storage.key(i).ok().flatten() else {
                continue;
            };
            let Some(name) = key.strip_prefix(PREFIX) else {
                continue;
            };
            if let Some(bytes) = storage
                .get_item(&key)
                .ok()
                .flatten()
                .and_then(|v| from_hex(&v))
            {
                store.insert(name.to_string(), bytes);
            }
        }
    }

    /// Exécute les actions en attente (thread principal, chaque frame).
    pub(crate) fn run_actions() {
        let actions = std::mem::take(&mut *ACTIONS.lock().unwrap());
        for action in actions {
            match action {
                WebAction::Persist(key, bytes) => {
                    if let Some(storage) = local_storage() {
                        let _ = storage.set_item(&format!("{PREFIX}{key}"), &to_hex(&bytes));
                    }
                }
                WebAction::Download(name, bytes) => {
                    if let Err(e) = download(&name, &bytes) {
                        log::warn!("téléchargement de {name} impossible : {e:?}");
                    }
                }
                WebAction::OpenUrl(url) => {
                    if let Some(window) = web_sys::window() {
                        let _ = window.open_with_url_and_target(&url, "_blank");
                    }
                }
            }
        }
    }

    fn download(name: &str, bytes: &[u8]) -> Result<(), wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        // Copie hors de la mémoire partagée : `Blob` refuse un
        // `SharedArrayBuffer`.
        let array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
        array.copy_from(bytes);
        let parts = js_sys::Array::of1(&array);
        let options = web_sys::BlobPropertyBag::new();
        options.set_type("application/octet-stream");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options)?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        let document = web_sys::window()
            .and_then(|w| w.document())
            .ok_or("pas de document")?;
        let a: web_sys::HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
        a.set_href(&url);
        a.set_download(name);
        a.click();
        web_sys::Url::revoke_object_url(&url)?;
        Ok(())
    }

    /// Paramètre de la query string de la page (`?lang=fr`).
    pub(crate) fn query_param(name: &str) -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        web_sys::UrlSearchParams::new_with_str(&search)
            .ok()?
            .get(name)
    }

    fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn from_hex(s: &str) -> Option<Vec<u8>> {
        if !s.len().is_multiple_of(2) {
            return None;
        }
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
            .collect()
    }
}
