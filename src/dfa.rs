use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DFA {
    rows: Vec<Row>,
}

impl DFA {
    fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let all_rows = reader.lines().flatten();
        let mut rows: Vec<Row> = Vec::new();
        for (_, row) in all_rows.enumerate() {
            match Row::from_str_custom(&row) {
                Ok(row) => rows.push(row),
                _ => break,
            }
            // rows.push(Row::from_str_custom(&row).unwrap());
        }

        Ok(DFA::new(rows))
        // DFA::new(Vec::new())
    }

    pub fn transition(&self, row: usize, letter: usize) -> Option<usize> {
        self.rows[row].transitions()[letter]
    }

    pub fn is_accepting(&self, row: usize) -> bool {
        self.rows[row].is_accepting()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Row {
    is_accepting: bool,
    id: usize,
    transitions: Vec<Option<usize>>, // None here represents 'E'
}

// impl PartialEq for Row {
//     fn eq(&self, other: &Self) -> bool {
//         self.is_accepting == other.is_accepting
//             && self.id == other.id
//             && self.transitions == other.transitions
//     }
// }

impl Row {
    pub fn new(is_accepting: bool, id: usize, transitions: Vec<Option<usize>>) -> Self {
        Self {
            is_accepting,
            id,
            transitions,
        }
    }

    pub fn transitions(&self) -> &[Option<usize>] {
        &self.transitions
    }

    pub fn is_accepting(&self) -> bool {
        self.is_accepting
    }

    //EX: - 0 E 1 E
    //EX: - 1 2 E E
    pub fn from_str_custom(input: &str) -> Result<Self, ()> {
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            [accept, row_id, transitions @ ..] => {
                let is_accept = *accept == "+";
                let transitions: Vec<Option<usize>> = transitions
                    .iter()
                    .map(|n| {
                        if *n == "E" {
                            None
                        } else {
                            Some(n.parse().unwrap())
                        }
                    })
                    .collect();

                Ok(Row::new(is_accept, row_id.parse().unwrap(), transitions))
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // tests for reading in a file
    #[test]
    #[should_panic]
    fn from_file_on_nonexistent_panics_on_unwrap() {
        DFA::from_file("this_file_does_not_exist.tt").unwrap();
    }

    #[test]
    fn two_line_valid_file() {
        let in_file = DFA::from_file("tests/two_liner.tt").unwrap();
        assert_eq!(
            in_file.rows,
            vec![
                Row::new(false, 0, vec![None, Some(1), None]),
                Row::new(false, 1, vec![Some(2), None, None])
            ]
        );
    }

    // do we want empty files to be valid?
    #[test]
    fn empty_file_test() {
        let in_file = DFA::from_file("tests/empty_file.tt").unwrap();
        assert_eq!(in_file.rows, vec![]);
    }

    #[test]
    fn two_line_valid_file_with_extra_lines() {
        let in_file = DFA::from_file("tests/two_liner_extra_lines.tt").unwrap();
        assert_eq!(
            in_file.rows,
            vec![
                Row::new(false, 0, vec![None, Some(1), None]),
                Row::new(false, 1, vec![Some(2), None, None])
            ]
        );
    }

    // Tests for str_parse
    #[test]
    #[should_panic]
    fn empty_str_parse() {
        let r = Row::from_str_custom("");
        r.unwrap();
    }

    #[test]
    #[should_panic]
    fn some_invalid_str_str_parse() {
        let r = Row::from_str_custom("It would not make sense to parse this");
        r.unwrap();
    }

    #[test]
    fn good_str_for_str_parse() {
        let r = Row::from_str_custom("- 0 E 1 E").unwrap();
        assert_eq!(r, Row::new(false, 0, vec![None, Some(1), None]));
    }

    #[test]
    fn another_good_str_for_str_parse() {
        let r = Row::from_str_custom("- 1 2 E E").unwrap();
        assert_eq!(r, Row::new(false, 1, vec![Some(2), None, None]));
    }

    #[test]
    fn accepting_and_another_id() {
        let r = Row::from_str_custom("+ 1 2 E E").unwrap();
        assert_eq!(r, Row::new(true, 1, vec![Some(2), None, None]));
    }
}
