#![feature(slice_patterns)]

use std::fs::File;
use std::io::{self, BufReader, Read};

pub enum FontType {
    EOT,
    Glyph,
    OpenType,
    TrueType,
    WOFF
}

pub fn font_type(source: &str) -> Result<FontType, io::Error> {
    let mut reader = BufReader::new(try!(File::open(source)));
    let mut buf = [0u8; 4];

    try!(reader.read(&mut buf));
    match buf {
        [0, 1, 0, 0] => Ok(FontType::TrueType),
        _ => Ok(FontType::WOFF)
    }
    
}

