use clap::{App, Arg};
fn main() {
    let matches = App::new("brainheck")
        .version("1.0")
        .author("Garen Tyler <garentyler@garen.dev>")
        .about("A brainfuck interpreter written in Rust.")
        .arg(
            Arg::with_name("filename")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    let source = std::fs::read_to_string(matches.value_of("filename").unwrap()).unwrap();
    brainheck::Program::interpret(&source, Box::new(std::io::stdin()), Box::new(std::io::stdout())).expect("Error interpreting program");
}
