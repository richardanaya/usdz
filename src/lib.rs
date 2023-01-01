mod zip;
use zip::*;

pub struct UsdzFile {
    pub zip_file: ZipFile,
}

pub fn parse_usdz_file(buffer: &[u8]) -> Result<UsdzFile, &'static str> {
    let zip_file = parse_zip_file(buffer)?;
    Ok(UsdzFile { zip_file })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buffer = include_bytes!("test_usdz/basic.usdz").to_vec();
        let zip_file = parse_zip_file(&buffer);
        assert_eq!(zip_file.is_ok(), true);
    }
}
