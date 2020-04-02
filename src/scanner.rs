use crate::dfa::*;
use std::collections::BTreeMap;
// example scan.u file
// x0ax20x5C x6fpqrx73
// wiki/noto.tt           pqrs
// wiki/nots.tt           opqr
// wiki/endsq.tt          endsq
// wiki/twosmallwords.tt  twosmallwords
// wiki/whackamole.tt     whack         x5cooox5cx20x5cooox5c
// wiki/anyone.tt         IGNORE

type Alphabet = BTreeMap<char, usize>;
type State = usize;

/// Each line of the sanner definition file looks like
/// wiki/noto.tt           pqrs         replace_with
/// tt is the dfa created by the file name
/// id is the name in the middle
/// replace_with is the optional replace with value
pub struct TransitionTables {
    tt: DFA,
    id: String,
    replace_with: Option<String>,
}

impl TransitionTables {
    pub fn new(tt: DFA, id: String, replace_with: Option<String>) -> Self {
        Self {
            tt,
            id,
            replace_with,
        }
    }
    pub fn from_str_custom(input: &str) -> Result<Self, ()> {
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            [file_name, id] => {
                // if replace_with.
                Ok(TransitionTables::new(
                    DFA::from_file(file_name).unwrap(),
                    id.to_string(),
                    None,
                ))
            }
            [file_name, id, replace_with] => {
                // if replace_with.
                Ok(TransitionTables::new(
                    DFA::from_file(file_name).unwrap(),
                    id.to_string(),
                    Some(replace_with.to_string()),
                ))
            }
            _ => Err(()),
        }
    }
}

/// Main struct for a scan definition file.
pub struct Scanner {
    Alpha: Alphabet,
    dfa: DFA,
}

impl Scanner {
    /// Alphabet comes in with xHH for control chars, we need
    /// to turn it into real chars
    fn alphabet_build(input: &str) -> Alphabet {
        let mut alpha = Alphabet::new();
        let mut clean_in = String::from(input);
        clean_in.retain(|c| !c.is_whitespace());
        let in_chars: Vec<char> = clean_in.chars().collect();
        let mut char_index = 0;
        let mut i = 0;
        while i < in_chars.len() {
            if in_chars[i] == 'x' {
                let hex_1 = in_chars[i + 1];
                let hex_2 = in_chars[i + 2];
                let mut hex_str = hex_1.to_string();
                hex_str.push(hex_2);
                i += 2;
                alpha.insert(Scanner::hex_to_char(&hex_str), char_index);
                char_index += 1;
            } else {
                alpha.insert(in_chars[i], char_index);
                char_index += 1;
            }
            i += 1;
        }
        alpha
    }

    fn hex_to_char(hex: &str) -> char {
        let numeric_code = u8::from_str_radix(&hex, 16).unwrap();
        numeric_code as char
    }
}

#[cfg(test)]
mod test {
    use crate::dfa::*;
    use crate::scanner::*;

    // alphabet builder

    #[test]
    fn one_char_alphabet() {
        let mut b = Alphabet::new();
        b.insert('a', 0);
        assert_eq!(Scanner::alphabet_build("a"), b);
    }

    #[test]
    fn two_char_alphabet() {
        let mut b = Alphabet::new();
        b.insert('a', 0);
        b.insert('b', 1);
        assert_eq!(Scanner::alphabet_build("ab"), b);
    }

    #[test]
    fn new_line_alphabet() {
        let mut b = Alphabet::new();
        b.insert('\n', 0);
        assert_eq!(Scanner::alphabet_build("x0a"), b);
    }

    #[test]
    fn ugly_symbols_line_alphabet() {
        let mut b = Alphabet::new();
        b.insert('\n', 0);
        b.insert(' ', 1);
        b.insert('\\', 2);
        assert_eq!(Scanner::alphabet_build("x0ax20x5C"), b);
    }

    #[test]

    fn alphabet_from_description() {
        let mut b = Alphabet::new();
        // this should be the alphabet from the description
        b.insert('\n', 0); // NEWLINE char
        b.insert(' ', 1); // SPACE char
        b.insert('\\', 2); // WHACK char
        b.insert('o', 3);
        b.insert('p', 4);
        b.insert('q', 5);
        b.insert('r', 6);
        b.insert('s', 7);
        assert_eq!(Scanner::alphabet_build("x0ax20x5C x6fpqrx73"), b);
    }

    // hex to char

    #[test]
    fn a_hex() {
        assert_eq!('a', Scanner::hex_to_char("61"));
    }

    #[test]
    fn space_hex() {
        assert_eq!(' ', Scanner::hex_to_char("20"));
    }

    #[test]
    fn newline_hex() {
        assert_eq!('\n', Scanner::hex_to_char("0a"));
    }

    #[test]
    fn wack_hex() {
        assert_eq!('\\', Scanner::hex_to_char("5C"));
    }

    // transition tables from str
    #[test]
    fn tt_from_str_with_replace() {
        let r = TransitionTables::from_str_custom(
            "wiki/whackamole.tt     whack         x5cooox5cx20x5cooox5c",
        )
        .unwrap();
        assert_eq!(r.id, "whack".to_string());
        assert_eq!(r.replace_with, Some("x5cooox5cx20x5cooox5c".to_string()));
        let dfa = DFA::from_file("wiki/whackamole.tt").unwrap();
        assert_eq!(dfa, r.tt);
    }

    #[test]
    fn tt_from_str_no_replace() {
        let r = TransitionTables::from_str_custom("wiki/nots.tt           opqr").unwrap();
        assert_eq!(r.id, "opqr".to_string());
        assert_eq!(r.replace_with, None);
        assert_eq!(r.tt, DFA::from_file("wiki/nots.tt").unwrap());
    }

    #[test]
    #[should_panic]
    fn tt_from_str_invalid_file() {
        let r = TransitionTables::from_str_custom("this_file_does_not_exist.tt           opqr")
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn tt_from_empty_str() {
        let r = TransitionTables::from_str_custom("").unwrap();
    }

    // currently empty files do not panic... not sure if that is what we want
    #[test]
    fn tt_from_str_empty_file() {
        let r = TransitionTables::from_str_custom("tests/empty_file.tt           opqr").unwrap();
    }
}
