#![allow(non_snake_case)]

use luther::dfa::*;
use structopt::StructOpt;

use std::path::PathBuf;

/// LUTHER Reads a scanner definition file, a source file, and outputs matched
/// tokens from DFA files described in the definition input.
#[derive(Debug, Default, StructOpt)]
#[structopt(name = "LUTHER")]
pub struct Args {
    /// Path to the scanning definition file
    #[structopt(parse(from_os_str))]
    definition: PathBuf,

    /// Path to a file of source to be tokenized by LUTHER
    #[structopt(parse(from_os_str))]
    source: PathBuf,

    /// Path to an output file for storing the tokenized source. Stdout if not present.
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}
fn main() {
    let args = Args::from_args();
    println!("{:?}", args);

    let temp_input_file_name = "some_tt.tt";

    let dfa = DFA::from_file(&temp_input_file_name).unwrap();
}
