#[cfg(windows)]
pub(crate) use std::os::windows::fs::MetadataExt;
#[cfg(not(windows))]
pub(crate) use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;

#[cfg(windows)]
pub(crate) fn config_dir() -> String {
    format!("{}/cc", std::env::var("APPDATA").expect("no %APPDATA%found"))
}

#[cfg(not(windows))]
pub(crate) fn config_dir() -> String {
    String::from("/etc/cc")
}

#[cfg(windows)]
pub(crate) fn fmt_canonical_path(path: &PathBuf) -> String {
    path.to_str().unwrap().split_at(4).1.replace("\\", "/")
}

#[cfg(not(windows))]
pub(crate) fn fmt_canonical_path(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

pub(crate) fn enable_virtual_terminal_processing() {
    #[cfg(windows)]
    {
        use winapi_util::console::Console;

        if let Ok(mut term) = Console::stdout() {
            let _ = term.set_virtual_terminal_processing(true);
        }
        if let Ok(mut term) = Console::stderr() {
            let _ = term.set_virtual_terminal_processing(true);
        }
    }
}