extern crate fontlib;

use fontlib::FontType;

#[test]
fn font_type_ttf1() {
    match fontlib::font_type("tests/TTF/Monaco_Linux.ttf").ok() {
        Some(FontType::TrueType) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf2() {
    match fontlib::font_type("tests/TTF/ClearSans-Regular.ttf").ok() {
        Some(FontType::TrueType) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_otf() {
    match fontlib::font_type("tests/OTF/Kleymissky_0283.otf").ok() {
        Some(FontType::OpenType) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_woff() {
    match fontlib::font_type("tests/WOFF/ClearSans-Regular.woff").ok() {
        Some(FontType::WOFF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_eot() {
    match fontlib::font_type("tests/EOT/ClearSans-Regular.eot").ok() {
        Some(FontType::EOT) => (),
        _ => panic!(),
    }
}
