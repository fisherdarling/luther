use crate::dfa::DFA;
// use crate::driver::Driver;
use crate::regex::Regex;
use crate::scanner::Scanner;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
pub struct Driver {
    // regexes: Vec<Regex>,
}

impl Driver {
    pub fn run(scanner: Scanner, src_file: PathBuf, _out: Option<PathBuf>) {
        let mut src_lines_as_str = crate::driver::Driver::build_source_vecs(src_file).unwrap(); // TODO fix the crate path

        let mut regxs: Vec<Regex> = Vec::new();
        let alpha = scanner.get_alpha();
        let trans = scanner.get_trans();
        let test_token = Some("This is a input stream");
        for t in trans {
            // println!("{:#?}", t);
            regxs.push(Regex::new(&t.tt, Some(&t.id), &alpha));
        }
        crate::driver::Driver::find_longest_match(&mut regxs, &src_lines_as_str);

        crate::driver::Driver::make_output(&regxs, &mut src_lines_as_str);
    }

    pub fn make_output(regexs: &Vec<Regex>, src_lines_as_str: &mut String) -> String {
        let mut output_line = String::new();
        let best_match = &regexs[0];
        let best_match_len = best_match.first_match(src_lines_as_str).unwrap(); // TODO What do we do in the case that nothing matches? I guess panic? IGNORE should deal with this
        let remaining_str = src_lines_as_str.split_off(best_match_len);

        output_line.push_str(&best_match.token.unwrap());
        // output_line.push_str(&best_match.) // TODO

        remaining_str // returns how much to 'move forward on the string we should go'
    }

    pub fn find_longest_match(regexs: &mut Vec<Regex>, src_lines_as_str: &String) {
        regexs.sort_by(|r1, r2| {
            r2.first_match(&src_lines_as_str)
                .unwrap_or(0)
                .cmp(&r1.first_match(&src_lines_as_str).unwrap_or(0))
        });
    }

    pub fn build_source_vecs(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        // TODO replace this with std::fs::read_to_string()
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut source_string = String::new();

        let all_rows = reader.lines();

        for r in all_rows {
            // r.unwrap();
            source_string.push_str(&r.unwrap());
            source_string.push('\n');
        }
        Ok(source_string)
    }
}
