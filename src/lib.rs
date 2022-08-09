mod basic;
mod v0101;

#[cfg(test)]
mod test {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_7fs() {
        let mut file = std::fs::File::open("test.7fs").unwrap();
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf).unwrap();
        let buf = &buf[16..];
        println!("{:#?}", v0101::item(buf).unwrap().1);
    }
}
