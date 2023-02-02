#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
#[cfg(not(windows))]
use std::os::unix::prelude::MetadataExt;

#[cfg(windows)]
use std::os::windows::fs::FileTypeExt;
#[cfg(not(windows))]
use std::os::unix::prelude::FileTypeExt;

use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{Local, Offset, TimeZone};

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

#[cfg(windows)]
pub(crate) fn get_meta_info(path: &PathBuf) -> (u64, u64, u64, u64, bool) {
    let meta = path.metadata().unwrap();
    let s_1601_to_1970 = 11644473600;
    let tz = Local.timestamp_opt(0, 0).unwrap().offset().fix().local_minus_utc() as u64;
    (meta.creation_time() / 10000000 - s_1601_to_1970 + tz,
     meta.last_access_time() / 10000000 - s_1601_to_1970 + tz,
     meta.last_write_time() / 10000000 - s_1601_to_1970 + tz,
     meta.file_size(),
     meta.permissions().readonly())
}
#[cfg(not(windows))]
pub(crate) fn get_meta_info(path: &PathBuf) -> (u64, u64, u64, u64, bool) {
    let meta = path.metadata().unwrap();
    (meta.created().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
     meta.atime() as u64,
     meta.mtime() as u64,
     meta.size() as u64,
     meta.mode() & 0o200 > 0)
}

#[cfg(windows)]
pub(crate) fn fmt_path_save(path: &PathBuf) -> String {
    path.to_str().unwrap().split_at(4).1.to_string()
}
#[cfg(not(windows))]
pub(crate) fn fmt_path_save(path: &PathBuf) -> String {
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