use crate::dfa::DFA;
// use crate::driver::Driver;
use crate::alphabet_translator::alphabet_translator::char_to_hex_a_string;
use crate::regex::Regex;
use crate::scanner::Scanner;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
pub struct Driver {
    // regexes: Vec<Regex>,
}

impl Driver {
    pub fn run(scanner: Scanner, src_file: PathBuf, out_path: PathBuf) {
        let mut src_lines_as_str = crate::driver::Driver::build_source_vecs(src_file).unwrap(); // TODO fix the crate path

        let mut output_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(out_path)
            .unwrap();
        let mut regxs: Vec<Regex> = Vec::new();
        let alpha = scanner.get_alpha();
        let trans = scanner.get_trans();
        //println!("{}", src_lines_as_str);
        // Create all regexs
        for t in trans {
            let replace_value = match &t.replace_with {
                None => None,
                Some(t) => Some(t.to_string()),
            };
            regxs.push(Regex::new(&t.tt, Some(&t.id), &alpha, replace_value));
        }

        let mut line_number = 1;
        let mut position = 1;
        let mut previos_pos = 1;
        let mut prev_line = 1;
        loop {
            let mut longest = 0;
            let mut num_newlines = 0;
            let mut regex_id = "";
            let mut regex_replace_value: Option<&String> = None;
            for r in regxs.iter() {
                let (length, newlines, char_number) = r.first_match(src_lines_as_str.as_str());
                if length > longest {
                    num_newlines = newlines;
                    longest = length;
                    position = char_number;
                    regex_id = r.token.unwrap();
                    regex_replace_value = r.replace_with.as_ref();
                }
            }
            line_number += num_newlines;
            let hex_encoded_output = char_to_hex_a_string(&src_lines_as_str[..longest].to_string());
            let line = format!(
                "{} {} {} {}",
                regex_id,
                regex_replace_value.unwrap_or(&hex_encoded_output),
                prev_line,
                previos_pos
            );
            println!("{}", line);
            writeln!(output_file, "{}", line);
            prev_line = line_number;
            if num_newlines == 0 {
                previos_pos += longest;
            } else {
                previos_pos = position;
            }
            src_lines_as_str = src_lines_as_str[longest..].to_string(); // chop off what we tokenized
            if src_lines_as_str.len() == 0 {
                break;
            }
        }

        //let crate::driver::Driver::make_output(&regxs, &mut src_lines_as_str);
        //println!("{}", str_ohea);
    }

    /*pub fn make_output(regexs: &Vec<Regex>, src_lines_as_str: &mut String) -> String {
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
    }*/

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
