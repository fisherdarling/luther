pub struct transition_table {
    rows: Vec<Row>,
}

struct Row {
    is_accepting: bool,
    id: usize,
    transitions: Vec<Option<usize>>, // None here represents 'E'
}

impl Row {

    pub fn new (is_accept: bool, id: usize, transitions: Vec<Option<usize>>) -> Self {
        Row{
            is_accepting,
            id,
            transitions,
        }
    }

    //EX: - 0 E 1 E
    //EX: - 1 2 E E
    pub fn from_str_custom(input &str, id: usize) -> Result<Self, ()> {
        match tokens.as_slice() {
            [accept, transitions @ ..] => {
                let is_accept = *accept == "+";
                let transitions: Vec<Option<usize>> =
                    transitions.iter().map(|s| s.parse()).collect();
                Ok(Row::new(is_accept, from_id, to_id, transitions))
            }
            _ => Err(()),
        }
    }
}
