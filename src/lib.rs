#![feature(slice_patterns)]

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

pub enum FontType {
    EOT,
    Glyph,
    OTF,
    TTF,
    WOFF
}

pub fn font_type(source: &str) -> Result<FontType, io::Error> {
    let mut reader = BufReader::new(try!(File::open(source)));
    let mut buf4b = [0u8; 4];

    try!(reader.read(&mut buf4b));
    match &buf4b {
        &[0, 1, 0, 0] => Ok(FontType::TTF),
        _ => match &buf4b {
            b"true" | b"typ1" => Ok(FontType::TTF),
            b"OTTO" => Ok(FontType::OTF),
            b"wOFF" => Ok(FontType::WOFF),
            _ => {
                let mut buf2b = [0u8; 2];
                try!(reader.seek(SeekFrom::Start(34)));
                try!(reader.read(&mut buf2b));
                match &buf2b {
                    b"LP" => Ok(FontType::EOT),
                    _ => panic!("Font type unknown")
                }
            }
        }
    }
}

