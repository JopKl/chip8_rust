use std::time::Duration;

use chip8_base;

const SPEED:u64 = 1_000;
pub struct Interpreter{
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],
}

impl Interpreter{
    pub fn new() ->Self{
        Self { 
            memory: [0; 4096],
            registers: [0; 16],
            program_counter: 0x200,
            display: [[chip8_base::Pixel::default(); 64]; 32],
            stack_pointer: 0,
            stack: [0; 16],
        }
    }

    fn fetch(&mut self) -> u16{
        if (self.program_counter >=4096){
            self.program_counter = 0;
        }
        let op = u16::from_be_bytes([
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter+1) as usize]
        ]);
        self.program_counter +=2;
        dbg!(self.program_counter);
        op
    }
}

impl chip8_base::Interpreter for Interpreter {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        self.fetch();
        Some([[chip8_base::Pixel::default(); 64]; 32])
    }

    fn speed(&self) -> std::time::Duration {
        Duration::from_micros(SPEED)
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}
