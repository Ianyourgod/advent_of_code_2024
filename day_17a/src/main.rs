#![feature(iter_array_chunks)]

#[derive(Debug, Clone)]
pub enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc(Literal),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl Instruction {
    pub fn from_tuple(t: (u8, u8)) -> Self {
        match t.0 {
            0 => Self::Adv(Combo::from_u8(t.1)),
            1 => Self::Bxl(t.1),
            2 => Self::Bst(Combo::from_u8(t.1)),
            3 => Self::Jnz(t.1),
            4 => Self::Bxc(t.1),
            5 => Self::Out(Combo::from_u8(t.1)),
            6 => Self::Bdv(Combo::from_u8(t.1)),
            7 => Self::Cdv(Combo::from_u8(t.1)),
            _ => unreachable!()
        }
    }

    pub fn to_num(&self) -> (u8, u8) {
        match self {
            Instruction::Adv(com) => (0, com.to_num()),
            Instruction::Bxl(lit) => (1, *lit),
            Instruction::Bst(com) => (2, com.to_num()),
            Instruction::Jnz(lit) => (3, *lit),
            Instruction::Bxc(l) => (4, *l), // hardcode ftw
            Instruction::Out(com) => (5, com.to_num()),
            Instruction::Bdv(com) => (6, com.to_num()),
            Instruction::Cdv(com) => (7, com.to_num()),
        }
    }
}

type Literal = u8;

#[derive(Debug, Clone, Copy)]
pub enum Combo {
    Literal(u8),
    RegA,
    RegB,
    RegC
}

impl Combo {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0..=3 => Combo::Literal(n),
            4 => Combo::RegA,
            5 => Combo::RegB,
            6 => Combo::RegC,
            _ => unreachable!()
        }
    }

    pub fn to_num(&self) -> u8 {
        match self {
            Combo::Literal(n) => *n,
            Combo::RegA => 4,
            Combo::RegB => 5,
            Combo::RegC => 6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub ip: usize,
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
}

impl Computer {
    pub fn new(
        reg_a: u64,
        reg_b: u64,
        reg_c: u64,
    ) -> Self {
        Self {
            ip: 0,
            reg_a,
            reg_b,
            reg_c,
        }
    }

    pub fn from_input(input: &str) -> (Self, Vec<Instruction>) {
        let mut lines = input.lines();

        let a_line = lines.next().unwrap();
        let b_line = lines.next().unwrap();
        let c_line = lines.next().unwrap();
        lines.next().unwrap();
        let program_line = lines.next().unwrap().split(": ").nth(1).unwrap();

        let reg_a = a_line.split(": ").nth(1).unwrap().parse().unwrap();
        let reg_b = b_line.split(": ").nth(1).unwrap().parse().unwrap();
        let reg_c = c_line.split(": ").nth(1).unwrap().parse().unwrap();
        let program = program_line.split(',').array_chunks::<2>().map(|[opcode, operand]| {
            let opcode = opcode.parse().unwrap();
            let operand = operand.parse().unwrap();
            Instruction::from_tuple((opcode, operand))
        }).collect();

        (Self::new(reg_a, reg_b, reg_c), program)
    }

    pub fn run(&mut self, a: u64, instructions: &Vec<Instruction>) -> Vec<u8> {
        let mut output = Vec::new();
        self.ip = 0;
        self.reg_a = a;
        self.reg_b = 0;
        self.reg_c = 0;

        while let Some(instruction) = instructions.get(self.ip) {
            match instruction {
                Instruction::Adv(com) => {
                    let denom = 2_u64.pow(self.execute_combo(com) as u32);
                    self.reg_a = self.reg_a / denom;
                }
                Instruction::Bxl(lit) => {
                    self.reg_b = self.reg_b ^ *lit as u64;
                }
                Instruction::Bst(com) => {
                    self.reg_b = self.execute_combo(com) % 8;
                }
                Instruction::Jnz(lit) => {
                    if self.reg_a != 0 {
                        self.ip = *lit as usize;
                        continue;
                    }
                }
                Instruction::Bxc(_) => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                }
                Instruction::Out(com) => {
                    output.push(self.execute_combo(com) as u8 % 8);
                }
                Instruction::Bdv(com) => {
                    let denom = 2_u64.pow(self.execute_combo(com) as u32);
                    self.reg_b = self.reg_a / denom;
                }
                Instruction::Cdv(com) => {
                    let denom = 2_u64.pow(self.execute_combo(com) as u32);
                    self.reg_c = self.reg_a / denom;
                }
            }
            self.ip += 1;
        }

        output
    }

    fn execute_combo(&self, combo: &Combo) -> u64 {
        match combo {
            Combo::Literal(l) => *l as u64,
            Combo::RegA => self.reg_a,
            Combo::RegB => self.reg_b,
            Combo::RegC => self.reg_c
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut computer, instructions) = Computer::from_input(&input);
    let instrs_as_out = instructions.iter().map(|i|i.to_num()).flat_map(|(a, b)| [a, b]).collect::<Vec<_>>();

    //println!("{:?}", computer.run(678628631453, &instructions));
    
    let mut initial_valid = Vec::new();
    for a in 0..1024_u64 {
        if computer.run(a, &instructions)[0] == instrs_as_out[0] {
            initial_valid.push(a);
        }
    }

    let mut valid = vec![initial_valid];

    let mut bit_length = 10;
    for num_correct in 2..=16 {
        valid.push(Vec::new());
        let prev_valid = valid.get(num_correct-2).unwrap().clone();
        for mut a in prev_valid {
            for d_a in 0..8 {
                a = a + (d_a * 2_u64.pow(bit_length));
                let out = computer.run(a, &instructions);

                if out.len()<num_correct {
                    continue;
                }

                let mut is_valid = true;
                for i in 0..num_correct {
                    if out[i]!=instrs_as_out[i] {
                        is_valid = false;
                        break;
                    }
                }

                if is_valid {
                    valid.get_mut(num_correct-1).unwrap().push(a);
                }
            }
        }
        bit_length+=3;
    }
    println!("{:?}", valid.get(14).unwrap().get(0).unwrap());
}