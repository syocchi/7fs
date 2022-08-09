use crate::basic::*;
use nom::{
    branch::permutation,
    combinator::verify,
    multi::many_till,
    number::complete::{be_u32, be_u8},
    Parser,
};
use std::u8;

use super::item::{item, Item};

#[derive(PartialEq, Debug)]
pub enum ItemData {
    File(Vec<u8>),
    Folder(Vec<Item>), // terminator: 0xff
    URLFile(String),
    Shortcut(String),
    ZipArchive(Vec<u8>),
    TarArchive(Vec<u8>),
}

pub fn item_data(input: &[u8]) -> nom::IResult<&[u8], ItemData> {
    let (input, kind) = be_u8(input)?;
    match kind {
        1 => data.map(ItemData::File).parse(input),
        2 => many_till(item, verify(be_u8, |&x| x == 0xff))
            .map(|x| x.0)
            .map(ItemData::Folder)
            .parse(input),
        3 => string.map(ItemData::URLFile).parse(input),
        4 => string.map(ItemData::Shortcut).parse(input),
        5 => data.map(ItemData::ZipArchive).parse(input),
        6 => data.map(ItemData::TarArchive).parse(input),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file() {
        let data = vec![
            0x01, // kind
            0x00, 0x00, 0x00, 0x04, // length
            0x00, 0x00, 0x00, 0x00, // data
        ];
        assert_eq!(
            item_data(&data),
            Ok((&[][..], ItemData::File(vec![0x00, 0x00, 0x00, 0x00]))),
        );
    }

    #[test]
    fn test_folder() {
        let data = vec![
            0x02, // kind
            //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0u8,  // name
            0x01, // kind
            0x00, 0x00, 0x00, 0x04, // length
            0x00, 0x00, 0x00, 0x00, // data
            //
            0xff, // terminator
        ];
        assert_eq!(
            item_data(&data),
            Ok((
                &[][..],
                ItemData::Folder(vec![Item {
                    utime: 0,
                    name: "Hello".to_string(),
                    data: ItemData::File(vec![0u8, 0u8, 0u8, 0u8]),
                }])
            )),
        );
    }

    #[test]
    fn url_file() {
        let data = vec![
            0x03, // kind
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0u8, // name
        ];
        assert_eq!(
            item_data(&data),
            Ok((&[][..], ItemData::URLFile("Hello".to_string()))),
        );
    }

    #[test]
    fn shortcut() {
        let data = vec![
            0x04, // kind
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0u8, // name
        ];
        assert_eq!(
            item_data(&data),
            Ok((&[][..], ItemData::Shortcut("Hello".to_string()))),
        );
    }

    #[test]
    fn zip_archive() {
        let data = vec![
            0x05, // kind
            0x00, 0x00, 0x00, 0x04, // length
            0x00, 0x00, 0x00, 0x00, // data
        ];
        assert_eq!(
            item_data(&data),
            Ok((&[][..], ItemData::ZipArchive(vec![0x00, 0x00, 0x00, 0x00]))),
        );
    }

    #[test]
    fn tar_archive() {
        let data = vec![
            0x06, // kind
            0x00, 0x00, 0x00, 0x04, // length
            0x00, 0x00, 0x00, 0x00, // data
        ];
        assert_eq!(
            item_data(&data),
            Ok((&[][..], ItemData::TarArchive(vec![0x00, 0x00, 0x00, 0x00]))),
        );
    }
}
