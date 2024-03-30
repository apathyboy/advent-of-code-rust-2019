use std::collections::VecDeque;

pub mod template;

// Use this file to add helper functions and additional modules.

pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

pub type IntcodeProgram = Vec<i128>;

#[derive(Debug, Clone)]
pub struct IntcodeComputer {
    instruction_pointer: usize,
    memory: Vec<i128>,
    input: VecDeque<i128>,
    output: Vec<i128>,
    is_running: bool,
    ticks: usize,
    relative_base: i128,
    default_input: Option<i128>,
    input_requested: bool,
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
            memory: Vec::new(),
            input: VecDeque::new(),
            output: Vec::new(),
            is_running: true,
            ticks: 0,
            relative_base: 0,
            default_input: None,
            input_requested: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }

    pub fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.is_running = true;
    }

    pub fn load_program_from_str(&mut self, input: &str) {
        if let Some(program) = parse_intcode_program(input) {
            self.load_program(&program);
        }
    }

    pub fn load_program(&mut self, program: &IntcodeProgram) {
        self.memory = program.clone();

        self.reset();
    }

    pub fn memory_snapshot(&self) -> &[i128] {
        &self.memory
    }

    pub fn get(&mut self, address: usize) -> Option<i128> {
        if address >= self.memory.len() {
            self.memory.resize(address + 100, 0);
        }

        Some(self.memory[address])
    }

    pub fn set(&mut self, address: usize, val: i128) {
        if address >= self.memory.len() {
            self.memory.resize(address + 100, 0);
        }

        self.memory[address] = val;
    }

    pub fn read_program(&self) -> IntcodeProgram {
        self.memory.clone()
    }

    pub fn has_input(&self) -> bool {
        !self.input.is_empty()
    }

    pub fn set_input(&mut self, val: i128) {
        self.input.push_back(val);
    }

    pub fn add_input_str(&mut self, input: &str) {
        for c in input.chars() {
            self.input.push_back(c as i128);
        }

        self.input.push_back(10);
    }

    pub fn set_default_input(&mut self, val: i128) {
        self.default_input = Some(val);
    }

    fn get_input(&mut self) -> Option<i128> {
        if self.input.is_empty() {
            self.default_input
        } else {
            self.input.pop_front()
        }
    }

    fn set_output(&mut self, val: i128) {
        self.output.push(val);
    }

    pub fn get_output(&self) -> &[i128] {
        &self.output
    }

    pub fn get_next_output(&mut self) -> Option<i128> {
        self.output.pop()
    }

    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    fn parameter_mode(&self, instruction: i128, parameter: i8) -> ParameterMode {
        let mode = instruction as usize / (10_usize.pow(parameter as u32 + 1)) % 10;

        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Invalid parameter mode"),
        }
    }

    fn opcode(&self, instruction: i128) -> i8 {
        (instruction % 100) as i8
    }

    fn read_parameter(&mut self, parameter: i8, instruction: i128) -> Option<i128> {
        let mode = self.parameter_mode(instruction, parameter);

        match mode {
            ParameterMode::Immediate => {
                Some(self.get(self.instruction_pointer + parameter as usize)?)
            }
            _ => {
                let offset = self.read_destination(parameter, instruction)?;
                self.get(offset)
            }
        }
    }

    fn read_destination(&mut self, parameter: i8, instruction: i128) -> Option<usize> {
        let index = self.get(self.instruction_pointer + parameter as usize)?;
        let mode = self.parameter_mode(instruction, parameter);
        match mode {
            ParameterMode::Immediate => panic!("Cannot read destination mode as immediate"),
            ParameterMode::Position => Some(index as usize),
            ParameterMode::Relative => Some((self.relative_base + index) as usize),
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        self.input_requested = false;

        let instruction = self.memory[self.instruction_pointer];

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
            9 => self.op_adj_base(instruction),
            99 => self.op_exit(instruction),
            _ => panic!("Invalid instruction: {instruction}"),
        };

        self.instruction_pointer += steps;
    }

    pub fn run_until_output(&mut self) -> Option<i128> {
        while !self.has_output() {
            self.tick();
        }

        self.input_requested = false;

        self.get_next_output()
    }

    pub fn run_until_io(&mut self) {
        while !self.has_output() && !self.input_requested {
            self.tick();
        }

        self.input_requested = false;
    }

    pub fn run(&mut self) {
        while self.is_running {
            self.tick();
        }

        self.input_requested = false;
    }

    fn op_add(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();
        let input_b = self.read_parameter(2, instruction).unwrap();
        let output = self.read_destination(3, instruction).unwrap();

        self.set(output, input_a + input_b);

        4
    }

    fn op_mul(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();
        let input_b = self.read_parameter(2, instruction).unwrap();
        let output = self.read_destination(3, instruction).unwrap();

        self.set(output, input_a * input_b);

        4
    }

    fn op_in(&mut self, instruction: i128) -> usize {
        let output = self.read_destination(1, instruction).unwrap();

        self.input_requested = true;

        if let Some(input) = self.get_input() {
            self.set(output, input);
        }

        2
    }

    fn op_out(&mut self, instruction: i128) -> usize {
        let output = self.read_parameter(1, instruction).unwrap();

        self.set_output(output);

        2
    }

    fn op_jump_if_true(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();
        let input_b = self.read_parameter(2, instruction).unwrap();

        if input_a != 0 {
            self.instruction_pointer = input_b as usize;

            0
        } else {
            3
        }
    }

    fn op_jump_if_false(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();
        let input_b = self.read_parameter(2, instruction).unwrap();

        if input_a == 0 {
            self.instruction_pointer = input_b as usize;
            0
        } else {
            3
        }
    }

    fn op_lt(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();
        let input_b = self.read_parameter(2, instruction).unwrap();
        let output = self.read_destination(3, instruction).unwrap();

        let result = if input_a < input_b { 1 } else { 0 };

        self.set(output, result);

        4
    }

    fn op_eq(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();
        let input_b = self.read_parameter(2, instruction).unwrap();
        let output = self.read_destination(3, instruction).unwrap();

        let result = if input_a == input_b { 1 } else { 0 };

        self.set(output, result);

        4
    }

    fn op_adj_base(&mut self, instruction: i128) -> usize {
        let input_a = self.read_parameter(1, instruction).unwrap();

        self.relative_base += input_a;

        2
    }

    fn op_exit(&mut self, _instruction: i128) -> usize {
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
