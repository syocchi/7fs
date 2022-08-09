use nom::{branch::permutation, number::streaming::be_u64, IResult, Parser};

use crate::basic::string;

use super::item_data::{item_data, ItemData};

#[derive(PartialEq, Debug)]
pub struct Item {
    pub utime: u64,
    pub name: String,
    pub data: ItemData,
}

impl Item {
    pub fn new(utime: u64, name: String, data: ItemData) -> Item {
        Item { utime, name, data }
    }
}

pub fn item(input: &[u8]) -> IResult<&[u8], Item> {
    permutation((be_u64, string, item_data))
        .map(|(utime, name, data)| Item::new(utime, name, data))
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_item() {
        let bytes = &[
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // time
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0u8, // name
            1u8, // kind
            0u8, 0u8, 0u8, 4u8, // data
            0u8, 0u8, 0u8, 4u8, // data
        ];
        assert_eq!(
            item(bytes),
            Ok((
                &[][..],
                Item {
                    utime: 0,
                    name: "Hello".to_string(),
                    data: ItemData::File(vec![0u8, 0u8, 0u8, 4u8]),
                }
            ))
        )
    }
}
