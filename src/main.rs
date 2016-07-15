use std::fs::File;
use std::io::BufReader;
use std::env::args;
use std::collections::BTreeMap;

struct Tape {
    pointer: usize,
    tape: Vec<u8>
}

impl Tape {
    fn new() -> Tape {
        Tape {
            pointer: 0,
            tape: vec![0]
        }
    }

    fn get(&self) -> u8 {
        self.tape[self.pointer]
    }

    fn inc(&mut self) {
        self.tape[self.pointer] += 1;
    }

    fn dec(&mut self) {
        self.tape[self.pointer] -= 1; // We assume that the brainfuck code is working
    }

    fn advance(&mut self) {
        self.pointer += 1; // We assume that the brainfuck code is working
    }

    fn devance(&mut self) {
        self.pointer -= 1; // We assume that the brainfuck code is working
    }

    fn getchar(&self) -> char {
        self.tape[self.pointer] as u8 as char
    }

    fn run(&mut self, str_code: &str) {
        let mut code: Vec<char> = Vec::new();
        let mut jump_map: BTreeMap<usize, usize> = BTreeMap::new();
        let mut jump_stack: Vec<usize> = Vec::new();
        let mut pointer: usize = 0;
        // Preprocessing, creating the jump map and a vector of the code
        for ch in str_code.chars() {
            match ch {
                '>' | '<' | '+' | '-' | '.' | ',' => (),
                '[' => jump_stack.put(pointer),
                ']' => {
                    let open = jump_stack.pop();
                    jump_map.insert(open, pointer);
                    jump_map.insert(pointer, open);
                }
            }
            code.put(ch);
            pointer += 1;
        }

        // The actual execution
        for ch in code {
            match ch {
                '>' => self.advance(),
                '<' => self.devance(),
                '+' => self.inc(),
                '-' => self.dec(),
                '.' => {
                    println!("{}", ch);
                    io::stdout().flush();
                },
                _ => ()

            }
        }
    }
}

fn main() {
    let path = args().nth(1).expect("Didn't get argument.");
    println!("{}", path);
    let file = BufReader::new(File::open(path).expect("File doesn't exist."));


}
