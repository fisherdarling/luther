pub mod alphabet_translator {
    #[inline(always)]
    pub fn hex_to_char(hex: &str) -> char {
        let numeric_code = u8::from_str_radix(&hex, 16).unwrap();
        numeric_code as char
    }

    pub fn char_to_hex(c: char) -> String {
        let mut temp = String::new();
        let n = c as u8;
        let mut hex_code = format!("{:X}", n);
        if hex_code.len() == 1 {
            hex_code.insert(0, '0');
        }
        temp.push('x');
        temp.push_str(&hex_code);
        temp
    }
}

#[cfg(test)]
mod test {
    use crate::alphabet_translator::alphabet_translator;

    #[test]
    fn a_char() {
        assert_eq!("x61", alphabet_translator::char_to_hex('a'));
    }

    #[test]
    fn space_char() {
        assert_eq!("x20", alphabet_translator::char_to_hex(' '));
    }

    #[test]
    fn newline_char() {
        assert_eq!("x0A", alphabet_translator::char_to_hex('\n'));
    }

    #[test]
    fn wack_char() {
        assert_eq!("x5C", alphabet_translator::char_to_hex('\\'));
    }
    #[test]
    fn a_hex() {
        assert_eq!('a', alphabet_translator::hex_to_char("61"));
    }

    #[test]
    fn space_hex() {
        assert_eq!(' ', alphabet_translator::hex_to_char("20"));
    }

    #[test]
    fn newline_hex() {
        assert_eq!('\n', alphabet_translator::hex_to_char("0A"));
    }

    #[test]
    fn wack_hex() {
        assert_eq!('\\', alphabet_translator::hex_to_char("5C"));
    }
}
