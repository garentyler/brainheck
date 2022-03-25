pub const MEMORY_SIZE: usize = 30_000;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Input,
    Output,
    Debug,
    LoopStart,
    LoopEnd,
    Unknown,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Move(isize),
    Add(isize),
    Input,
    Output,
    Debug,
    Loop(Vec<Operation>),
}

#[derive(Debug)]
pub enum InterpreterError {
    /// Error parsing source
    ParseError(String),
    /// Memory overflow
    MemoryOverflow,
    /// Pointer is out of memory bounds
    PointerOverflow,
    /// Error using stdio
    StdioError(std::io::Error),
}

pub struct Program<'io> {
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
    stdin: Box<dyn std::io::Read + 'io>,
    stdout: Box<dyn std::io::Write + 'io>,
}
impl<'io> Program<'io> {
    fn new(stdin: Box<dyn std::io::Read + 'io>, stdout: Box<dyn std::io::Write + 'io>) -> Self {
        Self {
            memory: [0u8; MEMORY_SIZE],
            pointer: 0,
            stdin,
            stdout,
        }
    }
    fn tokenize(source: &str) -> Result<Vec<Token>, InterpreterError> {
        let tokens = source
            .chars()
            .map(|cur| match cur {
                '>' => Token::MoveRight,
                '<' => Token::MoveLeft,
                '+' => Token::Increment,
                '-' => Token::Decrement,
                '.' => Token::Output,
                ',' => Token::Input,
                '[' => Token::LoopStart,
                ']' => Token::LoopEnd,
                '#' => Token::Debug,
                _ => Token::Unknown,
            })
            .filter(|token| token.ne(&Token::Unknown))
            .collect::<Vec<Token>>();
        Ok(tokens)
    }
    fn parse(tokens: &[Token]) -> Result<Vec<Operation>, InterpreterError> {
        use std::collections::LinkedList;
        let mut stack: LinkedList<Vec<Operation>> = LinkedList::new();
        stack.push_back(vec![]);
        for token in tokens {
            let current_operations = stack.back_mut().expect("Stack should not be empty!");
            match token {
                Token::Increment => {
                    if let Some(Operation::Add(x)) = current_operations.last_mut() {
                        *x += 1;
                    } else {
                        current_operations.push(Operation::Add(1))
                    }
                },
                Token::Decrement => {
                    if let Some(Operation::Add(x)) = current_operations.last_mut() {
                        *x += -1;
                    } else {
                        current_operations.push(Operation::Add(-1))
                    }
                },
                Token::MoveLeft => {
                    if let Some(Operation::Move(x)) = current_operations.last_mut() {
                        *x += -1;
                    } else {
                        current_operations.push(Operation::Move(-1))
                    }
                },
                Token::MoveRight => {
                    if let Some(Operation::Move(x)) = current_operations.last_mut() {
                        *x += 1;
                    } else {
                        current_operations.push(Operation::Move(1))
                    }
                },
                Token::Input => current_operations.push(Operation::Input),
                Token::Output => current_operations.push(Operation::Output),
                Token::Debug => current_operations.push(Operation::Debug),
                Token::LoopStart => stack.push_back(vec![]),
                Token::LoopEnd => {
                    let current_operations = stack.pop_back().unwrap();
                    let previous_operations = stack.back_mut().ok_or_else(|| {
                        InterpreterError::ParseError(String::from("Unexpected end of loop"))
                    })?;
        
                    previous_operations.push(Operation::Loop(current_operations))
                },
                Token::Unknown => {
                    return Err(InterpreterError::ParseError(
                        format!("Unexpected token {:?}", token)
                    ));
                },
            }
        }
        let operations = stack.pop_back().unwrap();
        if !stack.is_empty() {
            Err(InterpreterError::ParseError("Expected end of loop".to_owned()))
        } else {
            Ok(operations)
        }
    }
    fn execute(&mut self, operations: &[Operation]) -> Result<(), InterpreterError> {
        for operation in operations {
            match operation {
                Operation::Add(n) => {
                    let value = self.memory[self.pointer];
                    // print!("add({}): {} -> ", n, value);
                    self.memory[self.pointer] = ((value as isize + n) % 256) as u8;
                    // println!("{}", self.memory[self.pointer]);
                },
                Operation::Move(n) => {
                    // print!("move({}): {} -> ", n, self.pointer);
                    self.pointer = (self.memory.len() as isize + self.pointer as isize + n) as usize % self.memory.len();
                    // println!("{}", self.pointer);
                },
                Operation::Input => {
                    let mut buf = [0u8];
                    if let Err(err) = self.stdin.read(&mut buf) {
                        return Err(InterpreterError::StdioError(err));
                    }
                    // println!("input: {}", buf[0]);
                    self.memory[self.pointer] = buf[0];
                },
                Operation::Output => {
                    // println!("output: {}", self.memory[self.pointer] as char);
                    if let Err(err) = self.stdout.write_all(&[self.memory[self.pointer]]) {
                        return Err(InterpreterError::StdioError(err))
                    }
                    if let Err(err) = self.stdout.flush() {
                        return Err(InterpreterError::StdioError(err))
                    }
                },
                Operation::Debug => {
                    // // print the value as a string
                    let value = format!("{}", self.memory[self.pointer]);
                    // println!("debug: {}", value);
                    if let Err(err) = self.stdout.write_all(value.as_bytes()) {
                        return Err(InterpreterError::StdioError(err))
                    }
                    if let Err(err) = self.stdout.flush() {
                        return Err(InterpreterError::StdioError(err))
                    }
                },
                Operation::Loop(operations) => {
                    // println!("loop start");
                    while self.memory[self.pointer] != 0 {
                        self.execute(operations)?;
                        // println!("loop value: {}", self.memory[self.pointer]);
                    }
                    // println!("loop end");
                },
            }
        }
        Ok(())
    }
    pub fn interpret(source: &str, stdin: Box<dyn std::io::Read + 'io>, stdout: Box<dyn std::io::Write + 'io>) -> Result<(), InterpreterError> {
        let tokens = Program::tokenize(&source).unwrap();
        let operations = Program::parse(&tokens).unwrap();
        Program::new(stdin, stdout).execute(&operations)
    }
}