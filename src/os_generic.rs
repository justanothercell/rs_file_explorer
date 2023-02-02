#[cfg(windows)]
pub(crate) use std::os::windows::fs::MetadataExt;
#[cfg(unix)]
pub(crate) use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

#[cfg(windows)]
pub(crate) fn config_dir() -> String {
    format!("{}/cc", std::env::var("APPDATA").expect("no %APPDATA%found"))
}

#[cfg(unix)]
pub(crate) fn config_dir() -> String {
    String::from("/etc/cc")
}

#[cfg(windows)]
pub(crate) fn fmt_canonical_path(path: &PathBuf) -> String {
    path.to_str().unwrap().split_at(4).1.replace("\\", "/")
}

#[cfg(unix)]
pub(crate) fn fmt_canonical_path(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string()
}