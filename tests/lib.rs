extern crate fontlib;

#[test]
fn font_type_test() {
    match fontlib::font_type("tests/TrueType/Monaco_Linux.ttf").ok() {
        Some(fontlib::FontType::TrueType) => (),
        _ => panic!(),
    }
}
