#[cfg(windows)]
pub(crate) use std::os::windows::fs::MetadataExt;
#[cfg(unix)]
pub(crate) use std::os::unix::fs::MetadataExt;