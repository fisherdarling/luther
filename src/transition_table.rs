use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct TransitionTable {
    rows: Vec<Row>,
}

impl TransitionTable {
    fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut all_rows = reader.lines().flatten();
        let mut rows: Vec<Row> = Vec::new();
        for (index, row) in all_rows.enumerate() {
            match Row::from_str_custom(&row, index) {
                Ok(row) => rows.push(row),
                _ => break,
            }
            // rows.push(Row::from_str_custom(&row).unwrap());
        }

        Ok(TransitionTable::new(rows))
        // TransitionTable::new(Vec::new())
    }
}

#[derive(Debug, Default)]
struct Row {
    is_accepting: bool,
    id: usize,
    transitions: Vec<Option<usize>>, // None here represents 'E'
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.is_accepting == other.is_accepting
            && self.id == other.id
            && self.transitions == other.transitions
    }
}

impl Row {
    pub fn new(is_accepting: bool, id: usize, transitions: Vec<Option<usize>>) -> Self {
        Self {
            is_accepting,
            id,
            transitions,
        }
    }

    //EX: - 0 E 1 E
    //EX: - 1 2 E E
    pub fn from_str_custom(input: &str, id: usize) -> Result<Self, ()> {
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            [accept, transitions @ ..] => {
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

                Ok(Row::new(is_accept, id, transitions))
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::transition_table::Row;
    use crate::transition_table::TransitionTable;

    // tests for reading in a file
    #[test]
    #[should_panic]
    fn from_file_on_nonexistent_panics_on_unwrap() {
        TransitionTable::from_file("this_file_does_not_exist.tt").unwrap();
    }

    #[test]
    fn two_line_valid_file() {
        let in_file = TransitionTable::from_file("tests/two_liner.tt").unwrap();
        assert_eq!(
            in_file.rows,
            vec![
                Row::new(false, 0, vec![Some(0), None, Some(1), None]),
                Row::new(false, 1, vec![Some(1), Some(2), None, None])
            ]
        );
    }

    #[test]
    fn empty_file_test() {
        let in_file = TransitionTable::from_file("tests/empty_file.tt").unwrap();
        assert_eq!(in_file.rows, vec![]);
    }

    #[test]
    fn two_line_valid_file_with_extra_lines() {
        let in_file = TransitionTable::from_file("tests/two_liner_extra_lines.tt").unwrap();
        assert_eq!(
            in_file.rows,
            vec![
                Row::new(false, 0, vec![Some(0), None, Some(1), None]),
                Row::new(false, 1, vec![Some(1), Some(2), None, None])
            ]
        );
    }

    // Tests for str_parse
    #[test]
    #[should_panic]
    fn empty_str_parse() {
        let r = Row::from_str_custom("", 0);
        r.unwrap();
    }

    #[test]
    #[should_panic]
    fn some_invalid_str_str_parse() {
        let r = Row::from_str_custom("It would not make sense to parse this", 0);
        r.unwrap();
    }

    #[test]
    fn good_str_for_str_parse() {
        let r = Row::from_str_custom("- 0 E 1 E", 0).unwrap();
        assert_eq!(r, Row::new(false, 0, vec![Some(0), None, Some(1), None]));
    }

    #[test]
    fn another_good_str_for_str_parse() {
        let r = Row::from_str_custom("- 1 2 E E", 0).unwrap();
        assert_eq!(r, Row::new(false, 0, vec![Some(1), Some(2), None, None]));
    }

    #[test]
    fn accepting_and_another_id() {
        let r = Row::from_str_custom("+ 1 2 E E", 2).unwrap();
        assert_eq!(r, Row::new(true, 2, vec![Some(1), Some(2), None, None]));
    }
}
