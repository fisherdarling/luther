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

    pub fn len(&self) -> Option<usize> {
        Some(self.length.get())
    }

    pub fn first_match(&self, input: &str) -> Option<usize> {
        let mut chars = input.chars();
        let mut prev_state = None;

        while let Some(letter) = chars.next() {
            if let Some(next_state) = self.accept(letter) {
                prev_state = Some(next_state);
            } else {
                break;
            }
        }

        let result = if let Some(prev_state) = prev_state {
            if self.does_accept(prev_state) {
                self.len()
            } else {
                None
            }
        } else {
            None
        };
        self.reset();

        result
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

        assert_eq!(regex.first_match("poo"), Some(1));
        assert_eq!(regex.first_match("pqo"), Some(2));
        assert_eq!(regex.first_match("rspqo"), Some(4));
        assert_eq!(regex.first_match("oprqs"), None);
        assert_eq!(regex.first_match("owdadfqdasdwa"), None);
    }

    #[test]
    fn nots() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/nots.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("pqro"));

        assert_eq!(regex.first_match("pss"), Some(1));
        assert_eq!(regex.first_match("pqs"), Some(2));
        assert_eq!(regex.first_match("ropqs"), Some(4));
        assert_eq!(regex.first_match("sprqo"), None);
        assert_eq!(regex.first_match("swdadfqdasdwa"), None);
    }

    #[test]
    fn endsq() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/endsq.tt").unwrap();

        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("prsprssprq"));

        assert_eq!(regex.first_match("q"), Some(1));
        assert_eq!(regex.first_match("pssq"), Some(4));
        assert_eq!(regex.first_match("prqqqqq"), Some(7));
        assert_eq!(regex.first_match("roposq"), Some(6));
        assert_eq!(regex.first_match("oprsq"), Some(5));

        assert_eq!(regex.first_match("sprqo"), None);
        assert_eq!(regex.first_match("p"), None);
        assert_eq!(regex.first_match("r"), None);
    }

    #[test]
    fn twosmallwords() {
        let alpha = Alphabet::from_iter(ALPHABET.iter().copied());
        let dfa = DFA::from_file("./wiki/twosmallwords.tt").unwrap();
        let regex = Regex::new(&dfa, None, &alpha);

        assert!(regex.full_match("opqr opqr "));
        assert!(regex.full_match("opqr  opqr "));
        assert!(regex.full_match("opqr opq "));
        assert_eq!(regex.first_match("qpq rrr qpr"), Some(8));
        assert_eq!(regex.first_match("pppp  rrrp qpr"), Some(11));

        assert_eq!(regex.first_match("pppp o qpr"), None);
        assert_eq!(regex.first_match("p"), None);
        assert_eq!(regex.first_match("q p"), None);
    }
}
