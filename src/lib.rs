const MEMORY_SIZE: usize = 30_000;
pub fn execute_source(source: &str) {
    let mut mem = [0u8; MEMORY_SIZE];
    execute(source, 0, &mut mem);
}
pub fn execute(source: &str, mut mem_ptr: usize, mem: &mut [u8; MEMORY_SIZE]) {
    let mut source_ptr = 0;
    let mut source_chars = source.chars().collect::<Vec<char>>();
    let mut loop_stack: Vec<usize> = Vec::new();
    while source_ptr < source_chars.len() {
        match source_chars[source_ptr] {
            '>' => {
                if mem_ptr == MEMORY_SIZE - 1 {
                    mem_ptr = 0;
                } else {
                    mem_ptr += 1;
                }
            }
            '<' => {
                if mem_ptr == 0 {
                    mem_ptr = MEMORY_SIZE - 1;
                } else {
                    mem_ptr -= 1;
                }
            }
            '+' => {
                if mem[mem_ptr] == 255 {
                    mem[mem_ptr] = 0;
                } else {
                    mem[mem_ptr] += 1;
                }
            }
            '-' => {
                if mem[mem_ptr] == 0 {
                    mem[mem_ptr] = 255;
                } else {
                    mem[mem_ptr] -= 1;
                }
            }
            ',' => {
                mem[mem_ptr] = read_char() as u8;
                // println!("\nread '{}'", mem[mem_ptr] as char);
            }
            '.' => {
                print!("{}", mem[mem_ptr] as char);
            }
            'd' => {
                print!("{}", mem[mem_ptr]);
            }
            '[' => {
                loop_stack.push(source_ptr);
            }
            ']' => {
                if mem[mem_ptr] != 0 {
                    if loop_stack.len() > 0 {
                        source_ptr = *loop_stack.last().unwrap();
                    }
                } else {
                    let _ = loop_stack.pop();
                }
            }
            _ => {}
        }
        source_ptr += 1;
    }
}
pub fn execute_file(path: &str) {
    execute_source(&read_file(path));
}
fn read_file(path: &str) -> String {
    use std::io::Read;
    use std::{env, fs::File, path::Path};
    let mut file = File::open(&Path::new(path)).expect("could not open file");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("could not read file to string");
    s
}
fn read_char() -> char {
    // https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
    extern crate termios;
    use std::io;
    use std::io::Read;
    use std::io::Write;
    use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
    buffer[0] as char
}
