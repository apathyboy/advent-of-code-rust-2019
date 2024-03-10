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
    output: Option<OutputSink>,
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
            output: None,
            is_running: false,
        }
    }

    pub fn init_input_source(&mut self, input_source: InputSource) {
        self.input = Some(input_source);
    }

    pub fn init_output_sink(&mut self, output_sink: OutputSink) {
        self.output = Some(output_sink);
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

    fn set_output(&mut self, output: i64) {
        if let Some(output_sink) = self.output.as_mut() {
            output_sink(output);
        }
    }

    fn parameter_mode(&self, opcode: i64, parameter: i8) -> ParameterMode {
        let mode = opcode / 10i64.pow(parameter as u32 + 1) % 10;

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
        let mode = self.parameter_mode(opcode, parameter);

        match mode {
            ParameterMode::Position => {
                self.program[self.program[self.instruction_pointer + parameter as usize] as usize]
            }
            ParameterMode::Immediate => self.program[self.instruction_pointer + parameter as usize],
        }
    }

    fn op_add(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);
        let output_c = self.program[self.instruction_pointer + 3];

        self.instruction_pointer += 4;

        self.program[output_c as usize] = input_a + input_b;
    }

    fn op_mul(&mut self, opcode: i64) {
        let input_a = self.read_parameter(1, opcode);
        let input_b = self.read_parameter(2, opcode);
        let output_c = self.program[self.instruction_pointer + 3];

        self.instruction_pointer += 4;

        self.program[output_c as usize] = input_a * input_b;
    }

    fn op_out(&mut self, _opcode: i64) {
        let output = self.program[self.instruction_pointer + 1];

        self.instruction_pointer += 2;

        if let Some(input) = self.get_next_input() {
            self.program[output as usize] = input;
        }
    }

    fn op_in(&mut self, opcode: i64) {
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

            match self.opcode(opcode) {
                1 => {
                    self.op_add(opcode);
                }
                2 => {
                    self.op_mul(opcode);
                }
                3 => {
                    self.op_out(opcode);
                }
                4 => {
                    self.op_in(opcode);
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
    Some(input.split(',').filter_map(|n| n.parse().ok()).collect())
}
