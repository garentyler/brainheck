extern crate clap;
use clap::{App, Arg, SubCommand};
fn main() {
    let matches = App::new("brainheck")
        .version("1.0")
        .author("Garen Tyler <garentyler@gmail.com>")
        .about("A brainfuck interpreter written in Rust.")
        .arg(
            Arg::with_name("filename")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    brainheck::execute_file(matches.value_of("filename").unwrap());
}
