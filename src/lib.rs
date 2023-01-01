mod zip;
use zip::*;
mod usd;
pub use usd::*;

pub struct UsdzFile {
    pub zip_file: ZipFile,
}

impl UsdzFile {
    pub fn parse(buffer: &[u8]) -> Result<UsdzFile, &'static str> {
        let zip_file = parse_zip_file(buffer)?;
        Ok(UsdzFile { zip_file })
    }

    pub fn get_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        for part in &self.zip_file.parts {
            match part {
                ZipFilePart::LocalFileHeader(local_file_header) => {
                    files.push(local_file_header.file_name.clone());
                }
                _ => {}
            }
        }
        files
    }

    pub fn get_file_data(&self, file_name: &str) -> Option<Vec<u8>> {
        for part in &self.zip_file.parts {
            match part {
                ZipFilePart::LocalFileHeader(local_file_header) => {
                    if local_file_header.file_name == file_name {
                        return local_file_header.uncompressed_data.clone();
                    }
                }
                _ => {}
            }
        }
        None
    }
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

    #[test]
    fn parse_usd() {
        let buffer = include_bytes!("test_usdz/basic.usdz").to_vec();
        let zip_file = UsdzFile::parse(&buffer).unwrap();
        let files = zip_file.get_files();
        assert_eq!(files.len(), 1);
        let first_file = files.get(0).unwrap();
        assert_eq!(first_file, "basic/basic.usd");
        let file_data = zip_file.get_file_data(first_file).unwrap();
        let usd = Usd::parse(&file_data).unwrap();
        assert_eq!(usd.nodes.len(), 1);
    }
}
