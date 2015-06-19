extern crate fontlib;

use fontlib::FontType;

#[cfg(test)]
mod test {

    #[test]
    fn font_type_test() {
        assert_eq!(fontlib::font_type(), Ok(FontType::TrueType));
    }
}
