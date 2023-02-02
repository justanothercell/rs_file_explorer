use std::ops::{Add};
use chrono::{TimeZone, Utc, Duration, Local, Offset, DateTime};
use crate::cli::truncate_str;
use crate::os_generic::{get_meta_info};

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub(crate) ty: ItemType,
    pub(crate) name: String,
    pub(crate) readonly: bool,
    pub(crate) created: u64,
    pub(crate) last_accessed: u64,
    pub(crate) last_written: u64
}

impl Item {
    pub(crate) fn render(&self) -> String {
        let mut time_zero = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap();

        let created = time_zero.add(Duration::seconds(self.created as i64));
        let accessed = time_zero.add(Duration::seconds(self.last_accessed as i64));
        let written = time_zero.add(Duration::seconds(self.last_written as i64));
        format!("| {:32} | {} {} | {} | {:11} | {:11} |",
                truncate_str(&(self.name.clone() + match self.ty {
                ItemType::Dir => "/",
                _  => " "
            }), 32), match self.ty {
                ItemType::File(b) => format!("{b:10} bytes"),
                ItemType::Dir => format!("           <dir>"),
                ItemType::Link(_) => format!("         => ... "),
            },
            if self.readonly { " R" } else { "RW" },
            created.format("%d.%m.%Y %H:%M:%S"),
            fmt_est_time_passed(&accessed),
            fmt_est_time_passed(&written)
        )
    }
}

fn fmt_est_time_passed(date: &DateTime<Utc>) -> String {
    let mut now = Utc::now();
    // account for timezone
    now += Duration::seconds(Local.timestamp_opt(0, 0).unwrap().offset().fix().local_minus_utc() as i64);

    let d = now.signed_duration_since(date.clone());
    if d.num_minutes() == 0 {
        format!("    {:2}s ago", d.num_seconds() % 60)
    } else if d.num_hours() == 0 {
        format!("{:2}m {:2}s ago", d.num_minutes(), d.num_seconds() % 60)
    } else if d.num_days() == 0 {
        format!("{:2}h {:2}m ago", d.num_hours(), d.num_minutes() % 60)
    } else if d.num_weeks() == 0 {
        format!("{:2}d {:2}m ago", d.num_days(), d.num_hours() % 60)
    } else if d.num_weeks() < 4 {
        format!("{:2}w {:2}d ago", d.num_weeks(), d.num_days() % 7)
    } else {
        date.format("   %d.%m.%y").to_string()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ItemType {
    File(u64),
    Dir,
    Link(String)
}

pub(crate) fn collect_items(dir: &str) -> Vec<Item>{
    std::fs::read_dir(dir).expect("insufficient permission or does not exist")
        .map(|entry| {
            let entry = entry.expect("insufficient permission or does not exist");
            let meta = entry.metadata().unwrap();
            let (created, last_accessed, last_written, file_size, readonly) = get_meta_info(&entry.path());
            Item {
                ty: if meta.is_file() {
                    ItemType::File(file_size)
                } else if meta.is_dir() {
                    ItemType::Dir
                } else if meta.is_symlink() {
                    ItemType::Link(std::fs::read_link(entry.path()).unwrap().to_str().unwrap().to_string())
                } else { unreachable!() },
                name: entry.file_name().to_str().unwrap().to_string(),
                readonly,
                created,
                last_accessed,
                last_written
            }
        }).collect()
}