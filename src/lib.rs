#![feature(slice_patterns)]

extern crate byteorder;

mod table;

use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom, ErrorKind};
use byteorder::{BigEndian, ReadBytesExt};
use table::name::{NamingTable, NameRecord};

pub enum FontType {
    EOT,
    Glyph,
    OTF,
    TTF,
    WOFF
}

pub struct OffsetTable {
    /// Font type.
    pub sfnt_version: u32,
    /// Number of tables.
    pub num_tables: u16,
    /// (Maximum power of 2 <= numTables) x 16.
    pub search_range: u16,
    /// Log2(maximum power of 2 <= numTables).
    pub entry_selector: u16,
    /// NumTables x 16-searchRange.
    pub range_shift: u16,
}

pub struct TableRecord {
    /// 4 -byte identifier.
    pub tag: [u8; 4],
    /// Checksum for this table.
    pub checksum: u32,
    /// Offset from beginning of TrueType font file.
    pub offset: u32,
    /// Length of this table.
    pub length: u32,
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

pub fn font_name(source: &str) -> Result<NamingTable, io::Error> {
    let mut reader = BufReader::new(try!(File::open(source)));
    let mut buf4b = [0u8; 4];

    let offset_table = OffsetTable {
        sfnt_version: try!(reader.read_u32::<BigEndian>()),
        num_tables: try!(reader.read_u16::<BigEndian>()),
        search_range: try!(reader.read_u16::<BigEndian>()),
        entry_selector: try!(reader.read_u16::<BigEndian>()),
        range_shift: try!(reader.read_u16::<BigEndian>()),
    };


    let mut tables = Vec::new();
    for _ in 0u16..offset_table.num_tables {
        try!(reader.read(&mut buf4b));
        tables.push(TableRecord {
            tag: buf4b,
            checksum: try!(reader.read_u32::<BigEndian>()),
            offset: try!(reader.read_u32::<BigEndian>()),
            length: try!(reader.read_u32::<BigEndian>()),
        });
    }

    if let Some(table) = tables.iter().find(|&t| &t.tag == b"name") {
        try!(reader.seek(SeekFrom::Start(table.offset as u64)));
        let mut naming_table = NamingTable {
            format: try!(reader.read_u16::<BigEndian>()),
            count: try!(reader.read_u16::<BigEndian>()),
            string_offset: try!(reader.read_u16::<BigEndian>()),
            name_records: Vec::new(),
            lang_tag_count: 0,
            lang_tag_records: Vec::new(),
        };
        for _ in 0u16..naming_table.count {
            naming_table.name_records.push(NameRecord {
                platform_id: try!(reader.read_u16::<BigEndian>()),
                encoding_id: try!(reader.read_u16::<BigEndian>()),
                language_id: try!(reader.read_u16::<BigEndian>()),
                name_id: try!(reader.read_u16::<BigEndian>()),
                length: try!(reader.read_u16::<BigEndian>()),
                offset: try!(reader.read_u16::<BigEndian>()),
            });
        }
        Ok(naming_table)
    } else {
        Err(io::Error::new(ErrorKind::Other, "no name table found"))
    }
}
