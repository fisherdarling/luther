use std::cell::Cell;
use std::collections::BTreeMap;

use crate::dfa::DFA;

type Alphabet = BTreeMap<char, usize>;
type State = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Regex<'d, 'a, 't> {
    pub dfa: &'d DFA,
    pub token: Option<&'t str>,
    alphabet: &'a Alphabet,
    state: Cell<Option<usize>>,
    length: Cell<usize>,
}

impl<'d, 'a, 't> Regex<'d, 'a, 't> {
    pub fn new(dfa: &'d DFA, token: Option<&'t str>, alphabet: &'a Alphabet) -> Self {
        Self {
            dfa,
            token,
            alphabet,
            state: Cell::new(Some(0)),
            length: Cell::new(0),
        }
    }

    // takes in a letter and returns what state we end up at
    fn accept(&self, letter: char) -> Option<State> {
        if let Some(current_state) = self.state.take() {
            let char_index = self.alphabet[&letter];
            let new_state = self.dfa.transition(current_state, char_index);

            if new_state.is_some() {
                self.length.set(self.length.get() + 1);
            }

            self.state.set(new_state);
        }

        self.state.get()
    }

    fn currently_accepting(&self) -> bool {
        if let Some(state) = self.state.get() {
            self.dfa.is_accepting(state)
        } else {
            false
        }
    }

    fn does_accept(&self, state: usize) -> bool {
        self.dfa.is_accepting(state)
    }

    pub fn len(&self) -> usize {
        self.length.get()
    }

    // returns the length of the longest match
    pub fn first_match(&self, input: &str) -> (usize, usize, usize) {
        let mut chars = input.chars();
        let mut length = 0;
        let mut num_newlines = 0;
        let mut position = 1;
        let mut final_pos = 1;
        let mut newlines = 0;

        while let Some(letter) = chars.next() {
            if let Some(next_state) = self.accept(letter) {
                position += 1;
                if letter == '\n' {
                    newlines += 1;
                    position = 1;
                }
                if self.does_accept(next_state) {
                    length = self.len();
                    num_newlines += newlines;
                    final_pos = position;
                    newlines = 0;
                }
            } else {
                break;
            }
        }
        self.reset();

        (length, num_newlines, final_pos)
    }

    pub fn full_match(&self, input: &str) -> bool {
        for letter in input.chars() {
            self.accept(letter);
        }

        let result = self.currently_accepting() && self.length.get() == input.len();
        self.reset();

        result
    }

    pub fn reset(&self) {
        self.length.set(0);
        self.state.set(Some(0));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::iter::FromIterator;

    const ALPHABET: &[(char, usize)] = &[
        ('\n', 0), // NEWLINE char
        (' ', 1),  // SPACE char
        ('\\', 2), // WHACK char
        // The actual alphabet
        ('o', 3),
        ('p', 4),
        ('q', 5),
        ('r', 6),
        ('s', 7),
    ];

    #[test]
    fn noto() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/noto.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("pqrs"));

        assert_eq!(regex.first_match("poo"), (1, 0, 1));
        assert_eq!(regex.first_match("pqo"), (2, 0, 2));
        assert_eq!(regex.first_match("rspqo"), (4, 0, 4));
        assert_eq!(regex.first_match("oprqs"), (0, 0, 0));
        assert_eq!(regex.first_match("owdadfqdasdwa"), (0,0,0));
    }

    #[test]
    fn nots() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/nots.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("pqro"));

        assert_eq!(regex.first_match("pss"), (1, 0, 1));
        assert_eq!(regex.first_match("pqs"), (2, 0, 2));
        assert_eq!(regex.first_match("ropqs"), (4, 0, 4));
        assert_eq!(regex.first_match("sprqo"), (0, 0, 0));
        assert_eq!(regex.first_match("swdadfqdasdwa"), (0, 0, 0));
    }

    #[test]
    fn endsq() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/endsq.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("prsprssprq"));

        assert_eq!(regex.first_match("q"), (1, 0, 1));
        assert_eq!(regex.first_match("pssq"), (4, 0, 4));
        assert_eq!(regex.first_match("prqqqqq"), (7, 0, 7));
        assert_eq!(regex.first_match("roposq"), (6, 0, 6));
        assert_eq!(regex.first_match("oprsq"), (5, 0, 5));

        assert_eq!(regex.first_match("sprqo"), (0,0,0));
        assert_eq!(regex.first_match("p"), (0,0,0));
        assert_eq!(regex.first_match("r"), (0,0,0));
    }

    #[test]
    fn twosmallwords() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/twosmallwords.tt").unwrap();
        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("opqr opqr "));
        assert!(regex.full_match("opqr  opqr "));
        assert!(regex.full_match("opqr opq "));
        assert_eq!(regex.first_match("qpq rrr qpr"), (8, 0, 8));
        assert_eq!(regex.first_match("pppp  rrrp qpr"), (11, 0, 11));

        assert_eq!(regex.first_match("pppp o qpr"), (0,0,0));
        assert_eq!(regex.first_match("p"), (0,0,0));
        assert_eq!(regex.first_match("q p"), (0,0,0));
    }
}
