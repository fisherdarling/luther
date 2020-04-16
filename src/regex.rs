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
    pub replace_with: Option<String>,
    state: Cell<Option<usize>>,
    length: Cell<usize>,
}

impl<'d, 'a, 't> Regex<'d, 'a, 't> {
    pub fn new(
        dfa: &'d DFA,
        token: Option<&'t str>,
        alphabet: &'a Alphabet,
        replace_with: Option<String>,
    ) -> Self {
        Self {
            dfa,
            token,
            alphabet,
            replace_with,
            state: Cell::new(Some(0)),
            length: Cell::new(0),
        }
    }

    // takes in a letter and returns what state we end up at
    fn accept(&self, letter: char) -> Option<State> {
        if let Some(current_state) = self.state.take() {
            // Getting a character not in the alphabet is a hard error
            let char_index = *self
                .alphabet
                .get(&letter)
                .unwrap_or_else(|| std::process::exit(42));
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
    pub fn first_match(&self, input: &str, newline: char) -> (usize, usize, usize) {
        let mut chars = input.chars();
        let mut length = 0;
        let mut num_newlines = 0;
        let mut position = 1;
        let mut final_pos = 1;
        let mut newlines = 0;

        while let Some(letter) = chars.next() {
            if let Some(next_state) = self.accept(letter) {
                position += 1;
                if letter == newline {
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

        let regex = Regex::new(&dfa, None, &alpha, None);

        assert!(regex.full_match("pqrs"));

        assert_eq!(regex.first_match("poo", '\n'), (1, 0, 1));
        assert_eq!(regex.first_match("pqo", '\n'), (2, 0, 2));
        assert_eq!(regex.first_match("rspqo", '\n'), (4, 0, 4));
        assert_eq!(regex.first_match("oprqs", '\n'), (0, 0, 0));
        assert_eq!(regex.first_match("owdadfqdasdwa", '\n'), (0, 0, 0));
    }

    #[test]
    fn nots() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/nots.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha, None);

        assert!(regex.full_match("pqro"));

        assert_eq!(regex.first_match("pss", '\n'), (1, 0, 1));
        assert_eq!(regex.first_match("pqs", '\n'), (2, 0, 2));
        assert_eq!(regex.first_match("ropqs", '\n'), (4, 0, 4));
        assert_eq!(regex.first_match("sprqo", '\n'), (0, 0, 0));
        assert_eq!(regex.first_match("swdadfqdasdwa", '\n'), (0, 0, 0));
    }

    #[test]
    fn endsq() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/endsq.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha, None);

        assert!(regex.full_match("prsprssprq"));

        assert_eq!(regex.first_match("q", '\n'), (1, 0, 1));
        assert_eq!(regex.first_match("pssq", '\n'), (4, 0, 4));
        assert_eq!(regex.first_match("prqqqqq", '\n'), (7, 0, 7));
        assert_eq!(regex.first_match("roposq", '\n'), (6, 0, 6));
        assert_eq!(regex.first_match("oprsq", '\n'), (5, 0, 5));

        assert_eq!(regex.first_match("sprqo", '\n'), (0, 0, 0));
        assert_eq!(regex.first_match("p", '\n'), (0, 0, 0));
        assert_eq!(regex.first_match("r", '\n'), (0, 0, 0));
    }

    #[test]
    fn twosmallwords() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/twosmallwords.tt").unwrap();
        let regex = Regex::new(&dfa, None, &alpha, None);

        assert!(regex.full_match("opqr opqr "));
        assert!(regex.full_match("opqr  opqr "));
        assert!(regex.full_match("opqr opq "));
        assert_eq!(regex.first_match("qpq rrr qpr", '\n'), (8, 0, 8));
        assert_eq!(regex.first_match("pppp  rrrp qpr", '\n'), (11, 0, 11));

        assert_eq!(regex.first_match("pppp o qpr", '\n'), (0, 0, 0));
        assert_eq!(regex.first_match("p", '\n'), (0, 0, 0));
        assert_eq!(regex.first_match("q p", '\n'), (0, 0, 0));
    }
}
