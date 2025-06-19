#[derive(Debug, Clone)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    program_counter: usize,
    program: Vec<char>,
}

impl Computer {
    pub fn new(reg_a: i64, reg_b: i64, reg_c: i64, program: Vec<char>) -> Self {
        Computer {
            register_a: reg_a,
            register_b: reg_b,
            register_c: reg_c,
            program_counter: 0,
            program,
        }
    }

    pub fn from_str(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();

        let reg_a: i64 = lines[0].split_whitespace().last().unwrap().parse().unwrap();
        let reg_b: i64 = lines[1].split_whitespace().last().unwrap().parse().unwrap();
        let reg_c: i64 = lines[2].split_whitespace().last().unwrap().parse().unwrap();

        let program_unparsed = lines[4].split_whitespace().last().unwrap().split(',').collect::<Vec<&str>>();
        let program = program_unparsed.iter().map(|x| x.chars().next().unwrap()).collect::<Vec<char>>();

        Computer::new(reg_a, reg_b, reg_c, program)
    }

    pub fn run(&mut self) -> Vec<u8> {
        let mut total_output = Vec::new();
        while self.program_counter < self.program.len() {
            let instruction = self.parse_instruction(self.program_counter as usize);
            let (output, dont_add) = self.run_instruction(instruction);

            if let Some(output) = output {
                total_output.push(output);
            }

            if !dont_add {
                self.program_counter += 2;
            }
        }

        total_output
    }

    fn parse_combo(combo: char) -> Combo {
        match combo {
            '0' => Combo::Literal(0),
            '1' => Combo::Literal(1),
            '2' => Combo::Literal(2),
            '3' => Combo::Literal(3),
            '4' => Combo::RegA,
            '5' => Combo::RegB,
            '6' => Combo::RegC,
            _ => panic!("Invalid combo: {}", combo),
        }
    }

    fn parse_literal(literal: char) -> Literal {
        let value = literal.to_digit(10).unwrap() as u8;
        Literal { value }
    }

    fn parse_instruction(&self, index: usize) -> Instruction {
        match self.program[index] {
            '0' => {
                // adv
                let combo = Computer::parse_combo(self.program[index + 1]);
                Instruction::Adv(combo)
            }
            '1' => {
                // bxl
                let literal = Computer::parse_literal(self.program[index+ 1]);
                Instruction::Bxl(literal)
            }
            '2' => {
                // bst
                let combo = Computer::parse_combo(self.program[index+ 1]);
                Instruction::Bst(combo)
            }
            '3' => {
                // jnz
                let literal = Computer::parse_literal(self.program[index+ 1]);
                Instruction::Jnz(literal)
            }
            '4' => {
                // bxc
                Instruction::Bxc
            }
            '5' => {
                // out
                let combo = Computer::parse_combo(self.program[index+ 1]);
                Instruction::Out(combo)
            }
            '6' => {
                // bdv
                let combo = Computer::parse_combo(self.program[index+ 1]);
                Instruction::Bdv(combo)
            }
            '7' => {
                // cdv
                let combo = Computer::parse_combo(self.program[index+ 1]);
                Instruction::Cdv(combo)
            }
            _ => panic!("Invalid instruction: {}", self.program[index]),
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) -> (Option<u8>, bool) {
        match instruction {
            Instruction::Adv(combo) => {
                let value = 2_i64.pow(self.get_combo_value(combo) as u32);
                self.register_a /= value;
                (None, false)
            }
            Instruction::Bdv(combo) => {
                let value = 2_i64.pow(self.get_combo_value(combo) as u32);
                self.register_b = self.register_a / value;
                (None, false)
            }
            Instruction::Cdv(combo) => {
                let value = 2_i64.pow(self.get_combo_value(combo) as u32);
                self.register_c = self.register_a / value;
                (None, false)
            }
            Instruction::Bxl(literal) => {
                // bitwise xor of reg b and literal
                self.register_b ^= literal.value as i64;
                (None, false)
            }
            Instruction::Bst(combo) => {
                // bitwise and of reg b and combo
                self.register_b = self.get_combo_value(combo) % 8;
                (None, false)
            }
            Instruction::Jnz(literal) => {
                if self.register_a != 0 {
                    self.program_counter = literal.value as usize;
                }
                (None, self.register_a != 0)
            }
            Instruction::Bxc => {
                self.register_b ^= self.register_c;
                (None, false)
            }
            Instruction::Out(combo) => {
                let value = self.get_combo_value(combo) % 8;
                (Some(value as u8), false)
            }
        }
    }

    fn get_combo_value(&self, combo: Combo) -> i64 {
        match combo {
            Combo::Literal(lit) => lit as i64,
            Combo::RegA => self.register_a,
            Combo::RegB => self.register_b,
            Combo::RegC => self.register_c,
        }
    }



    fn run_in_reverse(&mut self, expected_output:)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[derive(Debug, Clone, Copy)]
enum Combo {
    Literal(u8),
    RegA,
    RegB,
    RegC,
}

#[derive(Debug, Clone, Copy)]
struct Literal {
    value: u8,
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let base_computer = Computer::from_str(&input);

    /*
    while !found.0 {
        // we're trying to find the value of a that makes the computer output its input (program)
        // we're gonna use multi-threading to speed up the process
        // to make it not super stupid, each thread will handle a range of values of a. if none of them find the answer, we'll start again from a higher starting value
        let mut handlers = Vec::new();
        
        println!("Going from {} to {}", lowest_a, lowest_a + 800000);

        for i in 0..8 {
            let (tx, rx) = mpsc::channel();
            let mut computer = base_computer.clone();
            let start = lowest_a + i * 100000;
            let end = start + (i+1) * 100000;
            let finding = looking_for.clone();
            std::thread::spawn(move || {
                let mut found = false;
                for a in start..end {
                    computer.register_a = a;
                    computer.program_counter = 0;
                    computer.register_b = 0;
                    computer.register_c = 0;
                    let output = computer.run();
                    let output_chars = output.split(',').collect::<String>().chars().collect::<Vec<char>>();

                    if output_chars == finding {
                        tx.send(Some((a, output))).unwrap();
                        found = true;
                        break;
                    }
                }
                if !found {
                    tx.send(None).unwrap();
                }
            });
            handlers.push(rx);
        }

        for handler in handlers {
            match handler.recv().unwrap() {
                Some((a, output)) => {
                    found = (true, Some((a, output)));
                    break;
                }
                None => continue,
            }
        }

        lowest_a += 800000;
    }
    */
}
