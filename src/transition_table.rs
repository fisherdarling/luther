pub struct TransitionTable {
    rows: Vec<Row>,
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

    // Tests for str_parse
    #[test]
    #[should_panic]
    fn test_empty_str_parse() {
        let r = Row::from_str_custom("", 0);
        r.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_some_invalid_str_str_parse() {
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
    fn test_accepting_and_another_id() {
        let r = Row::from_str_custom("+ 1 2 E E", 2).unwrap();
        assert_eq!(r, Row::new(true, 2, vec![Some(1), Some(2), None, None]));
    }
}
