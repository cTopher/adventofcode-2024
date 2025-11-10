use std::mem;
use std::str::FromStr;

pub struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    pub program: Vec<u8>,
    output: Vec<u8>,
    instruction_pointer: usize,
}

impl Computer {
    pub fn run(&mut self) -> Vec<u8> {
        while self.instruction_pointer < self.program.len() {
            self.run_instruction();
        }
        mem::take(&mut self.output)
    }

    pub const fn reset(&mut self, register_a: usize) {
        self.register_a = register_a;
        self.register_b = 0;
        self.register_c = 0;
        self.instruction_pointer = 0;
    }

    fn run_instruction(&mut self) {
        let opcode = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => {
                self.jnz(operand);
                return;
            }
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid opcode: {opcode}"),
        }
        self.instruction_pointer += 2;
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand: {operand}"),
        }
    }

    const fn literal_operand(operand: u8) -> usize {
        operand as usize
    }

    fn adv(&mut self, operand: u8) {
        self.register_a = self.division(operand);
    }

    const fn bxl(&mut self, operand: u8) {
        self.register_b ^= Self::literal_operand(operand);
    }

    fn bst(&mut self, operand: u8) {
        self.register_b = self.combo_operand(operand) % 8;
    }

    const fn jnz(&mut self, operand: u8) {
        if self.register_a == 0 {
            self.instruction_pointer += 2;
        } else {
            self.instruction_pointer = Self::literal_operand(operand);
        }
    }

    const fn bxc(&mut self, _operand: u8) {
        self.register_b ^= self.register_c;
    }

    fn out(&mut self, operand: u8) {
        let value = self.combo_operand(operand) % 8;
        self.output.push(u8::try_from(value).unwrap());
    }

    fn bdv(&mut self, operand: u8) {
        self.register_b = self.division(operand);
    }

    fn cdv(&mut self, operand: u8) {
        self.register_c = self.division(operand);
    }

    fn division(&self, operand: u8) -> usize {
        self.register_a >> self.combo_operand(operand)
    }
}

impl FromStr for Computer {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let mut lines = s.lines();
        macro_rules! line {
            ($prefix: expr) => {
                lines
                    .next()
                    .unwrap()
                    .strip_prefix(concat!($prefix, ": "))
                    .unwrap()
            };
        }
        let register_a = line!("Register A").parse().unwrap();
        let register_b = line!("Register B").parse().unwrap();
        let register_c = line!("Register C").parse().unwrap();
        assert_eq!(lines.next(), Some(""));
        let program = line!("Program")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self {
            register_a,
            register_b,
            register_c,
            program,
            instruction_pointer: 0,
            output: Vec::new(),
        })
    }
}
