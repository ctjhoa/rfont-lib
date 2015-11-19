extern crate fontlib;

#[test]
fn font_family_name_ttf_ind_bn_1_001() {
    if let Some(mut font) = fontlib::Font::new("tests/TTF/ind_bn_1_001.ttf").ok() {
        match font.font_family_name() {
            Ok(Some(font_family_name)) => {
                assert_eq!(font_family_name, "ind_bn_1_001");
            },
            _ => panic!(),
        }
    } else {
        panic!()
    }
}

