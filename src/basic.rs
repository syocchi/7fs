use nom::{number::complete::be_u32, Parser};

pub fn data(input: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    let (input, length) = be_u32(input)?;
    let (input, data) = nom::bytes::streaming::take(length as usize)(input)?;
    Ok((input, Vec::from(data)))
}

pub fn string(input: &[u8]) -> nom::IResult<&[u8], String> {
    let (input, data) = nom::bytes::streaming::take_until("\0")
        .map(Vec::from)
        .parse(input)?;
    let (input, _) = nom::bytes::streaming::take(1usize)(input)?;
    Ok((input, String::from_utf8(data).unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data() {
        let bytes = vec![
            0x00, 0x00, 0x00, 0x04, // length
            0x00, 0x00, 0x00, 0x00, // data
        ];

        assert_eq!(data(&bytes), Ok((&[][..], vec![0x00, 0x00, 0x00, 0x00])));
    }
    #[test]
    fn test_string() {
        let bytes = vec!['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0u8];

        assert_eq!(string(&bytes), Ok((&[][..], "Hello".to_string())));
    }
}
