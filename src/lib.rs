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

pub struct Font {
    reader: BufReader<File>,
    offset_table: OffsetTable,
    tables: Vec<TableRecord>,

    font_family_name: Option<String>,
}

impl Font {
    pub fn new(source: &str) -> Result<Font, io::Error> {
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

        Ok(Font {
            reader: reader,
            offset_table: offset_table,
            tables: tables,

            font_family_name: None,
        })
    }

    pub fn font_family_name(&mut self) -> Result<Option<String>, io::Error> {
        match self.font_family_name {
            Some(ref font_family_name) => {
                Ok(Some(font_family_name.clone()))
            }
            None => {
                if let Some(table) = self.tables.iter().find(|&t| &t.tag == b"name") {
                    try!(self.reader.seek(SeekFrom::Start(table.offset as u64)));
                    let mut naming_table = NamingTable {
                        format: try!(self.reader.read_u16::<BigEndian>()),
                        count: try!(self.reader.read_u16::<BigEndian>()),
                        string_offset: try!(self.reader.read_u16::<BigEndian>()),
                        name_records: Vec::new(),
                        lang_tag_count: 0,
                        lang_tag_records: Vec::new(),
                    };
                    // TODO: Handle lang tag records
                    for _ in 0u16..naming_table.count {
                        naming_table.name_records.push(NameRecord {
                            platform_id: try!(self.reader.read_u16::<BigEndian>()),
                            encoding_id: try!(self.reader.read_u16::<BigEndian>()),
                            language_id: try!(self.reader.read_u16::<BigEndian>()),
                            name_id: try!(self.reader.read_u16::<BigEndian>()),
                            length: try!(self.reader.read_u16::<BigEndian>()),
                            offset: try!(self.reader.read_u16::<BigEndian>()),
                        });
                    }

                    if let Some(rec) = naming_table.name_records.iter().find(|&r| r.name_id == 1 && r.encoding_id == 1) {
                        try!(self.reader.seek(SeekFrom::Start(table.offset as u64 + naming_table.string_offset as u64 + rec.offset as u64)));
                        let mut buf = Vec::new();
                        buf.resize(rec.length as usize, 0);
                        try!(self.reader.read(&mut buf));
                        if let Ok(font_family_name) = String::from_utf8(buf) {
                            self.font_family_name = Some(font_family_name);

                            Ok(self.font_family_name.clone())
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(io::Error::new(ErrorKind::Other, "no name table found"))
                }
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
