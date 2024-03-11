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
    input: Option<InputSource>,
    output: Vec<i64>,
    is_running: bool,
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
            input: None,
            output: Vec::new(),
            is_running: false,
        }
    }

    pub fn init_input_source(&mut self, input_source: InputSource) {
        self.input = Some(input_source);
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

    fn get_next_input(&mut self) -> Option<i64> {
        if let Some(input_source) = self.input.as_mut() {
            Some(input_source())
        } else {
            None
        }
    }

    fn set_output(&mut self, val: i64) {
        self.output.push(val);
    }

    pub fn get_output(&self) -> &[i64] {
        &self.output
    }

    fn parameter_mode(&self, opcode: i64, parameter: i8) -> ParameterMode {
        let mode = opcode as usize / (10_usize.pow(parameter as u32 + 1)) % 10;

        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode"),
        }
    }

    fn opcode(&self, opcode: i64) -> i8 {
        (opcode % 100) as i8
    }

    fn read_parameter(&self, parameter: i8, opcode: i64) -> i64 {
        let index = (self.instruction_pointer + parameter as usize) % self.program.len();
        let mode = self.parameter_mode(opcode, parameter);

        match mode {
            ParameterMode::Position => self.program[self.program[index] as usize],
            ParameterMode::Immediate => self.program[index],
        }
    }

    fn op_add(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);
        let output_c = self.program[self.instruction_pointer + 3];

        if output_c == 677 {
            panic!("add set to 677");
        }

        self.instruction_pointer += 4;

        self.program[output_c as usize] = input_a + input_b;
    }

    fn op_mul(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);
        let output_c = self.program[self.instruction_pointer + 3];

        if output_c == 677 {
            panic!("add set to 677");
        }
        self.instruction_pointer += 4;

        self.program[output_c as usize] = input_a * input_b;
    }

    fn op_jump_if_true(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);

        if input_a != 0 {
            self.instruction_pointer = input_b as usize;
        } else {
            self.instruction_pointer += 3;
        }
    }

    fn op_jump_if_false(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);

        if input_a == 0 {
            self.instruction_pointer = input_b as usize;
        } else {
            self.instruction_pointer += 3;
        }
    }

    fn op_lt(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);
        let output_c = self.program[self.instruction_pointer + 3];

        if output_c == 677 {
            panic!("add set to 677");
        }
        self.instruction_pointer += 4;

        self.program[output_c as usize] = if input_a < input_b { 1 } else { 0 };
    }

    fn op_eq(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);
        let output_c = self.program[self.instruction_pointer + 3];

        if output_c == 677 {
            panic!("add set to 677");
        }
        self.instruction_pointer += 4;

        self.program[output_c as usize] = if input_a == input_b { 1 } else { 0 };
    }

    fn op_in(&mut self, _opcode: i64) {
        let output = self.program[self.instruction_pointer + 1];

        if output == 677 {
            panic!("add set to 677");
        }
        self.instruction_pointer += 2;

        if let Some(input) = self.get_next_input() {
            self.program[output as usize] = input;
        }
    }

    fn op_out(&mut self, opcode: i64) {
        let output = self.read_parameter(1, opcode);

        self.instruction_pointer += 2;

        self.set_output(output);
    }

    fn op_exit(&mut self, _opcode: i64) {
        self.instruction_pointer += 1;
        self.is_running = false;
    }

    pub fn run(&mut self) {
        self.is_running = true;

        while self.is_running {
            let opcode = self.program[self.instruction_pointer];

            let op = self.opcode(opcode);

            match op {
                1 => {
                    self.op_add(opcode);
                }
                2 => {
                    self.op_mul(opcode);
                }
                3 => {
                    self.op_in(opcode);
                }
                4 => {
                    self.op_out(opcode);
                }
                5 => {
                    self.op_jump_if_true(opcode);
                }
                6 => {
                    self.op_jump_if_false(opcode);
                }
                7 => {
                    self.op_lt(opcode);
                }
                8 => {
                    self.op_eq(opcode);
                }
                99 => {
                    self.op_exit(opcode);
                }
                _ => panic!("Invalid opcode"),
            }
        }
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
