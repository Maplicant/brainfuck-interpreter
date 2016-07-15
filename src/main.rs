use std::fs::File;
use std::io::BufReader;
use std::env::args;
use std::collections::BTreeMap;
use std::io::stdout;
use std::io::Write;
use std::io::Read;

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
        self.tape.push(0);
    }

    fn devance(&mut self) {
        self.pointer -= 1; // We assume that the brainfuck code is working
    }

    fn getchar(&self) -> char {
        self.tape[self.pointer] as u8 as char
    }

    fn run(&mut self, str_code: String) {
        let mut code: Vec<char> = Vec::new();
        let mut jump_map: BTreeMap<usize, usize> = BTreeMap::new();
        let mut jump_stack: Vec<usize> = Vec::new();
        let mut pc: usize = 0;
        // Preprocessing, creating the jump map and a vector of the code
        for ch in str_code.chars() {
            match ch {
                '>' | '<' | '+' | '-' | '.' | ',' => (),
                '[' => jump_stack.push(pc),
                ']' => {
                    let open = jump_stack.pop().expect("Brainfuck code is invalid ([ and ] are not equal)");
                    jump_map.insert(open, pc);
                    jump_map.insert(pc, open);
                },
                _ => ()
            }
            code.push(ch);
            pc += 1;
        }
        println!("Created jump_map: {:#?}", jump_map);
        // The actual execution
        let mut pc = 0;
        for ch in code {
            println!("Matching {}", ch);
            match ch {
                '>' => self.advance(),
                '<' => self.devance(),
                '+' => self.inc(),
                '-' => self.dec(),
                '.' => {
                    println!("{}", self.get());
                    // print!("{}", self.getchar());
                    stdout().flush();
                },
                '[' => {
                    if self.get() == 0 {
                        self.pointer = jump_map.get(&self.pointer).unwrap().clone();
                    }
                },
                ']' => {
                    println!("End of loop, got {}", self.get());
                    if self.get() != 0 {
                        self.pointer = jump_map.get(&self.pointer).unwrap().clone();
                    }
                }
                _ => ()
            }
            pc += 1;
            println!("{:#?}", self.tape);
        }
    }
}

fn main() {
    let path = args().nth(1).expect("Didn't get argument.");
    let mut file = File::open(path).expect("File doesn't exist.");
    let mut code: String = String::new();
    file.read_to_string(&mut code);
    Tape::new().run(code);
}
