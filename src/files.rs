use std::convert::Into;
use std::ops::Add;
use std::os::windows::fs::FileTypeExt;
use chrono::{TimeZone, Utc, DateTime, Duration, LocalResult};
use crate::cli::truncate_str;
use crate::os_generic::MetadataExt;

const TIME_ZERO: DateTime<Utc> = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap();

#[derive(Debug)]
pub(crate) struct Item {
    ty: ItemType,
    name: String,
    readonly: bool,
    created: u64,
    last_used: u64,
    last_written: u64
}

impl Item {
    pub(crate) fn render(&self) -> String {
        let m: LocalResult<DateTime<Utc>> = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0);
        let c = m.unwrap();
        m.unwrap()
        format!("| {:20} | {:12} {} | {:12?} | {:12?} | {:12?} |",
                truncate_str(&(self.name.clone() + match self.ty {
                ItemType::Dir => "/",
                _  => " "
            }), 20), match self.ty {
                ItemType::File(b) => format!("{b:6} bytes"),
                ItemType::Dir => format!("       <dir>"),
                ItemType::Link(_, true) => format!("     => ... "),
                ItemType::Link(_, false) => format!("     => .../")
            },
            if self.readonly { " R" } else { "RW" },
            TIME_ZERO.add(Duration::seconds((self.created / 100000000) as i64)),
            Utc.timestamp_opt((self.last_used / 100000000) as i64, 0).unwrap(),
            Utc.timestamp_opt((self.last_written / 100000000) as i64, 0).unwrap()
        )
    }
}

#[derive(Debug)]
pub(crate) enum ItemType {
    File(u64),
    Dir,
    Link(String, bool)
}

pub(crate) fn collect_items(dir: &str) -> Vec<Item>{
    std::fs::read_dir(dir).expect("insufficient permission or does not exist")
        .map(|entry| {
            let entry = entry.expect("insufficient permission or does not exist");
            let meta = entry.metadata().unwrap();
            let perm = meta.permissions();
            Item {
                ty: if meta.is_file() {
                    ItemType::File(meta.file_size())
                } else if meta.is_dir() {
                    ItemType::Dir
                } else if meta.is_symlink() {
                    ItemType::Link(std::fs::read_link(entry.path()).unwrap().to_str().unwrap().to_string(), entry.file_type().unwrap().is_symlink_file())
                } else { unreachable!() },
                name: entry.file_name().to_str().unwrap().to_string(),
                readonly: perm.readonly(),
                created: meta.creation_time(),
                last_used: meta.last_access_time(),
                last_written: meta.last_write_time()
            }
        }).collect()
}