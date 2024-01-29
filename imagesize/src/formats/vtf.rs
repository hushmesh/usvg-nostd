use crate::util::*;
use crate::{ImageResult, ImageSize};

use no_std_io::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(16))?;

    Ok(ImageSize {
        width: read_u16(reader, &Endian::Little)? as usize,
        height: read_u16(reader, &Endian::Little)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"VTF\0")
}
