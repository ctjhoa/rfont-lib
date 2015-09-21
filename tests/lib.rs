extern crate fontlib;

use fontlib::FontType;

#[test]
fn font_type_ttf_ind_bn_1_001() {
    match fontlib::font_type("tests/TTF/ind_bn_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_gu_1_001() {
    match fontlib::font_type("tests/TTF/ind_gu_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_hi_1_001() {
    match fontlib::font_type("tests/TTF/ind_hi_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_kn_1_001() {
    match fontlib::font_type("tests/TTF/ind_kn_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_ml_1_001() {
    match fontlib::font_type("tests/TTF/ind_ml_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_or_1_001() {
    match fontlib::font_type("tests/TTF/ind_or_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_pa_1_001() {
    match fontlib::font_type("tests/TTF/ind_pa_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_ta_1_001() {
    match fontlib::font_type("tests/TTF/ind_ta_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_ind_te_1_001() {
    match fontlib::font_type("tests/TTF/ind_te_1_001.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_norasi_boldoblique() {
    match fontlib::font_type("tests/TTF/Norasi-BoldOblique.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_norasi_bold() {
    match fontlib::font_type("tests/TTF/Norasi-Bold.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_norasi_oblique() {
    match fontlib::font_type("tests/TTF/Norasi-Oblique.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_norasi() {
    match fontlib::font_type("tests/TTF/Norasi.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_ttf_clearsans_regular() {
    match fontlib::font_type("tests/TTF/ClearSans-Regular.ttf").ok() {
        Some(FontType::TTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_otf_kleymissky_0283() {
    match fontlib::font_type("tests/OTF/Kleymissky_0283.otf").ok() {
        Some(FontType::OTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_otf_file() {
    match fontlib::font_type("tests/OTF/File.otf").ok() {
        Some(FontType::OTF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_woff_clearsans_regular() {
    match fontlib::font_type("tests/WOFF/ClearSans-Regular.woff").ok() {
        Some(FontType::WOFF) => (),
        _ => panic!(),
    }
}

#[test]
fn font_type_eot_clearsans_regular() {
    match fontlib::font_type("tests/EOT/ClearSans-Regular.eot").ok() {
        Some(FontType::EOT) => (),
        _ => panic!(),
    }
}
