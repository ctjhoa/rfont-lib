extern crate fontlib;

#[test]
fn font_name_ttf_ind_bn_1_001() {
    match fontlib::font_name("tests/TTF/ind_bn_1_001.ttf").ok() {
        Some(font_name) => {
            println!("{:?}", font_name);
        },
        _ => panic!(),
    }
}

