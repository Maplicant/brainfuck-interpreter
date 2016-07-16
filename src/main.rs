use std::fs::File;
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
        self.tape[self.pointer] -= 1;
    }

    fn advance(&mut self) {
        self.pointer += 1;
        self.tape.push(0);
    }

    fn devance(&mut self) {
        self.pointer -= 1;
    }

    fn getchar(&self) -> char {
        self.get() as u8 as char
    }
}

struct VM {
    tape: Tape,
    pc: usize,
    jump_map: BTreeMap<usize, usize>,
    code: Vec<char>
}

impl VM {
    fn new() -> VM {
        VM {
            tape: Tape::new(),
            pc: 0,
            jump_map: BTreeMap::new(),
            code: Vec::new()
        }
    }

    fn run(&mut self, str_code: String) {
        self.preprocess(str_code);

        while self.pc < self.code.len() {
            match self.code[self.pc] {
                '>' => self.tape.advance(),
                '<' => self.tape.devance(),
                '+' => self.tape.inc(),
                '-' => self.tape.dec(),
                '.' => {
                    print!("{}", self.tape.getchar());
                    stdout().flush();
                },
                '[' => {
                    if self.tape.get() == 0 {
                        self.pc = self.jump_map.get(&self.pc).unwrap().clone();
                    }
                },
                ']' => { if self.tape.get() != 0 { self.pc = self.jump_map.get(&self.pc).unwrap().clone(); } },
                _ => ()
            }
            self.pc += 1;
        }
    }

    fn preprocess(&mut self, str_code: String) {
        let mut jump_stack: Vec<usize> = Vec::new();
        let mut ppc: usize = 0; // Program counter, stores current instruction index
        // Preprocessing, creating the jump map and a vector of the code
        for ch in str_code.chars() {
            match ch {
                '[' => jump_stack.push(ppc),
                ']' => {
                    let open = jump_stack.pop().expect("Brainfuck code is invalid ([ and ] are not equal)");
                    self.jump_map.insert(open, ppc);
                    self.jump_map.insert(ppc, open);
                },
                _ => ()
            }
            self.code.push(ch);
            ppc += 1;
        }
    }
}

fn main() {
    let path = args().nth(1).expect("Didn't get argument.");
    let mut file = File::open(path).expect("Couldn't open file.");
    let mut code: String = String::new();
    file.read_to_string(&mut code);
    VM::new().run(code);
}
