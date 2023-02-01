use crate::os_generic::MetadataExt;

#[derive(Debug)]
pub(crate) struct Item {
    ty: ItemType,
    name: String,
    readonly: bool
}

#[derive(Debug)]
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
            let perm = meta.permissions();
            Item {
                ty: if meta.is_file() {
                    ItemType::File(meta.file_size())
                } else if meta.is_dir() {
                    ItemType::Dir
                } else if meta.is_symlink() {
                    ItemType::Link(std::fs::read_link(entry.path()).expect("unable to read symlink").to_str().unwrap().to_string())
                } else { unreachable!() },
                name: entry.file_name().to_str().unwrap().to_string(),
                readonly: perm.readonly()
            }
        }).collect()
}