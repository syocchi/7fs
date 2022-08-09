use super::item::Item;

pub enum ItemData {
    File(Vec<u8>),
    Folder(Vec<Item>), // terminator: 0xff
    URLFile(String),
    Shortcut(String),
    ZipArchive(Vec<u8>),
    TarArchive(Vec<u8>),
}
