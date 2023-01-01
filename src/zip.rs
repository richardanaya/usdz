use nom::bytes::complete::tag;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;

// Define the structure of a local file header
#[derive(Debug, PartialEq)]
pub struct LocalFileHeader {
    signature: u32,
    version_needed: u16,
    flags: u16,
    compression_method: u16,
    last_modified_time: u16,
    last_modified_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    pub file_name: String,
    extra_field: Option<Vec<u8>>,
    pub uncompressed_data: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
pub struct CentralDirectoryHeader {
    signature: u32,
    version_made_by: u16,
    version_needed: u16,
    flags: u16,
    compression_method: u16,
    last_modified_time: u16,
    last_modified_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_comment_length: u16,
    disk_number_start: u16,
    internal_file_attributes: u16,
    external_file_attributes: u32,
    relative_offset_of_local_header: u32,
    file_name: String,
    extra_field: Option<Vec<u8>>,
    file_comment: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
pub struct EndOfCentralDirectoryRecord {
    signature: u32,
    number_of_this_disk: u16,
    number_of_the_disk_with_the_start_of_the_central_directory: u16,
    total_number_of_entries_in_the_central_directory_on_this_disk: u16,
    total_number_of_entries_in_the_central_directory: u16,
    size_of_the_central_directory: u32,
    offset_of_start_of_central_directory_with_respect_to_the_starting_disk_number: u32,
    zip_file_comment_length: u16,
    zip_file_comment: Option<Vec<u8>>,
}

fn parse_central_directory_header(input: &[u8]) -> nom::IResult<&[u8], CentralDirectoryHeader> {
    use nom::bytes::complete::take;

    let (
        input,
        (
            _signature,
            version_made_by,
            version_needed,
            flags,
            compression_method,
            last_modified_time,
            last_modified_date,
            crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_comment_length,
            disk_number_start,
            internal_file_attributes,
            external_file_attributes,
            relative_offset_of_local_header,
        ),
    ) = tuple((
        tag([0x50, 0x4B, 0x01, 0x02]), // signature
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u32,
        le_u32,
        le_u32,
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u32,
        le_u32,
    ))(input)?;

    // Extract the file name, extra field, and file comment from the input
    let (input, file_name) = take(file_name_length)(input)?;
    let file_name = std::str::from_utf8(file_name).unwrap().to_owned();
    let (input, extra_field) = if extra_field_length > 0 {
        let (input, extra_field) = take(extra_field_length)(input)?;
        (input, Some(extra_field.to_vec()))
    } else {
        (input, None)
    };
    let (input, file_comment) = if file_comment_length > 0 {
        let (input, file_comment) = take(file_comment_length)(input)?;
        (input, Some(file_comment.to_vec()))
    } else {
        (input, None)
    };

    Ok((
        input,
        CentralDirectoryHeader {
            signature: 0x504B0102,
            version_made_by,
            version_needed,
            flags,
            compression_method,
            last_modified_time,
            last_modified_date,
            crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_comment_length,
            disk_number_start,
            internal_file_attributes,
            external_file_attributes,
            relative_offset_of_local_header,
            file_name,
            extra_field,
            file_comment,
        },
    ))
}

fn parse_end_of_central_directory_record(
    input: &[u8],
) -> nom::IResult<&[u8], EndOfCentralDirectoryRecord> {
    use nom::bytes::complete::take;

    let (
        input,
        (
            _signature,
            number_of_this_disk,
            number_of_the_disk_with_the_start_of_the_central_directory,
            total_number_of_entries_in_the_central_directory_on_this_disk,
            total_number_of_entries_in_the_central_directory,
            size_of_the_central_directory,
            offset_of_start_of_central_directory_with_respect_to_the_starting_disk_number,
            zip_file_comment_length,
        ),
    ) = tuple((
        tag([0x50, 0x4B, 0x05, 0x06]), // signature
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u32,
        le_u32,
        le_u16,
    ))(input)?;

    // Extract the zip file comment from the input
    let (input, zip_file_comment) = if zip_file_comment_length > 0 {
        let (input, zip_file_comment) = take(zip_file_comment_length)(input)?;
        (input, Some(zip_file_comment.to_vec()))
    } else {
        (input, None)
    };

    Ok((
        input,
        EndOfCentralDirectoryRecord {
            signature: 0x504B0506,
            number_of_this_disk,
            number_of_the_disk_with_the_start_of_the_central_directory,
            total_number_of_entries_in_the_central_directory_on_this_disk,
            total_number_of_entries_in_the_central_directory,
            size_of_the_central_directory,
            offset_of_start_of_central_directory_with_respect_to_the_starting_disk_number,
            zip_file_comment_length,
            zip_file_comment,
        },
    ))
}

// Use nom to parse a local file header
fn parse_local_file_header(input: &[u8]) -> nom::IResult<&[u8], LocalFileHeader> {
    use nom::bytes::complete::take;

    let (
        input,
        (
            _signature,
            version_needed,
            flags,
            compression_method,
            last_modified_time,
            last_modified_date,
            crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
        ),
    ) = tuple((
        tag([0x50, 0x4B, 0x03, 0x04]), // signature
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u16,
        le_u32,
        le_u32,
        le_u32,
        le_u16,
        le_u16,
    ))(input)?;

    // Extract the file name and extra field from the input
    let (input, file_name) = take(file_name_length)(input)?;
    let file_name = std::str::from_utf8(file_name).unwrap().to_owned();
    let (input, extra_field) = if extra_field_length > 0 {
        let (input, extra_field) = take(extra_field_length)(input)?;
        (input, Some(extra_field.to_vec()))
    } else {
        (input, None)
    };

    // get uncompressed data
    let (input, uncompressed_data) = if compression_method == 0 {
        let (input, uncompressed_data) = take(uncompressed_size)(input)?;
        (input, Some(uncompressed_data.to_vec()))
    } else {
        (input, None)
    };

    Ok((
        input,
        LocalFileHeader {
            signature: 0x504B0304,
            version_needed,
            flags,
            compression_method,
            last_modified_time,
            last_modified_date,
            crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_name,
            extra_field,
            uncompressed_data,
        },
    ))
}

#[derive(Debug, PartialEq)]
pub enum ZipFilePart {
    LocalFileHeader(LocalFileHeader),
    CentralDirectoryHeader(CentralDirectoryHeader),
    EndOfCentralDirectoryRecord(EndOfCentralDirectoryRecord),
}

#[derive(Debug, PartialEq)]
pub struct ZipFile {
    pub parts: Vec<ZipFilePart>,
}

fn parse_zip_file_part(input: &[u8]) -> nom::IResult<&[u8], ZipFilePart> {
    let (_, signature) = le_u32(input)?;
    match signature {
        0x4034B50 => {
            let (input, local_file_header) = parse_local_file_header(input)?;
            Ok((input, ZipFilePart::LocalFileHeader(local_file_header)))
        }
        0x02014B50 => {
            let (input, central_directory_header) = parse_central_directory_header(input)?;
            Ok((
                input,
                ZipFilePart::CentralDirectoryHeader(central_directory_header),
            ))
        }
        0x06054B50 => {
            let (input, end_of_central_directory_record) =
                parse_end_of_central_directory_record(input)?;
            Ok((
                input,
                ZipFilePart::EndOfCentralDirectoryRecord(end_of_central_directory_record),
            ))
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

pub fn parse_zip_file(buffer: &[u8]) -> Result<ZipFile, &'static str> {
    let mut parts = Vec::new();
    let mut remaining_buffer = buffer;
    loop {
        match parse_zip_file_part(remaining_buffer) {
            Ok((new_remaining_buffer, part)) => {
                parts.push(part);
                remaining_buffer = new_remaining_buffer;
            }
            Err(_) => {
                print!("Error parsing zip file");
                return Err("Error parsing zip file");
            }
        }
        if remaining_buffer.len() == 0 {
            break;
        }
    }
    Ok(ZipFile { parts: parts })
}
