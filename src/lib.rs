pub mod template;

// Use this file to add helper functions and additional modules.

pub type IntcodeProgram = Vec<i64>;

pub struct IntcodeComputer {
    instruction_pointer: usize,
    program: IntcodeProgram,
}

impl Default for IntcodeComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl IntcodeComputer {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0,
            program: IntcodeProgram::new(),
        }
    }

    pub fn reset(&mut self) {
        self.instruction_pointer = 0;
    }

    pub fn load_program(&mut self, program: &IntcodeProgram) {
        self.program = program.clone();

        self.reset();
    }

    pub fn memory_snapshot(&self) -> &[i64] {
        &self.program
    }

    pub fn register(&self, register: usize) -> Option<i64> {
        if register < self.program.len() {
            Some(self.program[register])
        } else {
            None
        }
    }

    pub fn read_program(&self) -> IntcodeProgram {
        self.program.clone()
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.program[self.instruction_pointer];

            match opcode {
                1 => {
                    let input_a = self.program[self.instruction_pointer + 1];
                    let input_b = self.program[self.instruction_pointer + 2];
                    let output_c = self.program[self.instruction_pointer + 3];

                    self.program[output_c as usize] =
                        self.program[input_a as usize] + self.program[input_b as usize];

                    self.instruction_pointer += 4;
                }
                2 => {
                    let input_a = self.program[self.instruction_pointer + 1];
                    let input_b = self.program[self.instruction_pointer + 2];
                    let output_c = self.program[self.instruction_pointer + 3];

                    self.program[output_c as usize] =
                        self.program[input_a as usize] * self.program[input_b as usize];

                    self.instruction_pointer += 4;
                }
                99 => {
                    self.instruction_pointer += 1;

                    break;
                }
                _ => panic!("Invalid opcode"),
            }
        }
    }
}

pub fn parse_intcode_program(input: &str) -> Option<IntcodeProgram> {
    Some(input.split(',').filter_map(|n| n.parse().ok()).collect())
}
