use super::item_data::ItemData;

pub struct Item {
    utime: u64,
    name: String,
    kind: u8,
    data: ItemData,
}
