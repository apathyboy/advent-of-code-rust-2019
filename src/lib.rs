use std::collections::VecDeque;

pub mod template;

// Use this file to add helper functions and additional modules.

pub enum ParameterMode {
    Position,
    Immediate,
}

pub type IntcodeProgram = Vec<i64>;
pub type InputSource = Box<dyn Fn() -> i64>;
pub type OutputSink = Box<dyn FnMut(i64)>;

pub struct IntcodeComputer {
    instruction_pointer: usize,
    program: IntcodeProgram,
    input: VecDeque<i64>,
    output: Vec<i64>,
    is_running: bool,
    ticks: usize,
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
            input: VecDeque::new(),
            output: Vec::new(),
            is_running: true,
            ticks: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.is_running = true;
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

    pub fn set_input(&mut self, val: i64) {
        self.input.push_back(val);
    }

    fn get_input(&mut self) -> Option<i64> {
        self.input.pop_front()
    }

    fn set_output(&mut self, val: i64) {
        self.output.push(val);
    }

    pub fn get_output(&self) -> &[i64] {
        &self.output
    }

    pub fn get_next_output(&mut self) -> Option<i64> {
        self.output.pop()
    }

    fn parameter_mode(&self, instruction: i64, parameter: i8) -> ParameterMode {
        let mode = instruction as usize / (10_usize.pow(parameter as u32 + 1)) % 10;

        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode"),
        }
    }

    fn opcode(&self, instruction: i64) -> i8 {
        (instruction % 100) as i8
    }

    fn read_parameter(&self, parameter: i8, instruction: i64) -> i64 {
        let index = (self.instruction_pointer + parameter as usize) % self.program.len();
        let mode = self.parameter_mode(instruction, parameter);

        match mode {
            ParameterMode::Position => self.program[self.program[index] as usize],
            ParameterMode::Immediate => self.program[index],
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;

        let instruction = self.program[self.instruction_pointer];

        let op = self.opcode(instruction);

        let steps = match op {
            1 => self.op_add(instruction),
            2 => self.op_mul(instruction),
            3 => self.op_in(instruction),
            4 => self.op_out(instruction),
            5 => self.op_jump_if_true(instruction),
            6 => self.op_jump_if_false(instruction),
            7 => self.op_lt(instruction),
            8 => self.op_eq(instruction),
            99 => self.op_exit(instruction),
            _ => panic!("Invalid instruction: {instruction}"),
        };

        self.instruction_pointer += steps;
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.tick();
        }
    }

    fn op_add(&mut self, instruction: i64) -> usize {
        let input_a = self.read_parameter(1, instruction);
        let input_b = self.read_parameter(2, instruction);
        let output_c = self.program[self.instruction_pointer + 3];

        self.program[output_c as usize] = input_a + input_b;

        4
    }

    fn op_mul(&mut self, instruction: i64) -> usize {
        let input_a = self.read_parameter(1, instruction);
        let input_b = self.read_parameter(2, instruction);
        let output_c = self.program[self.instruction_pointer + 3];

        self.program[output_c as usize] = input_a * input_b;

        4
    }

    fn op_jump_if_true(&mut self, instruction: i64) -> usize {
        let input_a = self.read_parameter(1, instruction);
        let input_b = self.read_parameter(2, instruction);

        if input_a != 0 {
            self.instruction_pointer = input_b as usize;

            0
        } else {
            3
        }
    }

    fn op_jump_if_false(&mut self, instruction: i64) -> usize {
        let input_a = self.read_parameter(1, instruction);
        let input_b = self.read_parameter(2, instruction);

        if input_a == 0 {
            self.instruction_pointer = input_b as usize;
            0
        } else {
            3
        }
    }

    fn op_lt(&mut self, instruction: i64) -> usize {
        let input_a = self.read_parameter(1, instruction);
        let input_b = self.read_parameter(2, instruction);
        let output_c = self.program[self.instruction_pointer + 3];

        self.program[output_c as usize] = if input_a < input_b { 1 } else { 0 };

        4
    }

    fn op_eq(&mut self, instruction: i64) -> usize {
        let input_a = self.read_parameter(1, instruction);
        let input_b = self.read_parameter(2, instruction);
        let output_c = self.program[self.instruction_pointer + 3];

        self.program[output_c as usize] = if input_a == input_b { 1 } else { 0 };

        4
    }

    fn op_in(&mut self, _instruction: i64) -> usize {
        let output = self.program[self.instruction_pointer + 1];

        if let Some(input) = self.get_input() {
            self.program[output as usize] = input;
        }

        2
    }

    fn op_out(&mut self, instruction: i64) -> usize {
        let output = self.read_parameter(1, instruction);

        self.set_output(output);

        2
    }

    fn op_exit(&mut self, _instruction: i64) -> usize {
        self.is_running = false;

        1
    }
}

pub fn parse_intcode_program(input: &str) -> Option<IntcodeProgram> {
    Some(
        input
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect(),
    )
}
