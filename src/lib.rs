pub struct Instructionset {
    cpu: CPUSpecs,
    instructions: Vec<Box<dyn Instruction>>,
}

pub struct CPUSpecs {
    registers: u8,
    ram: u64,
    rom: u64,
}

pub trait Instruction {
    fn assemble(&self) -> u64;
    fn mnemonic(&self) -> String;
    fn description(&self) -> String;
}

pub enum Token {
    Mnemonic(String),
    Number(i64),
    Register(String),
    String(String),
}

impl CPUSpecs {
    pub fn new(registers: u8, ram: u64, rom: u64) -> Self {
        Self {
            registers,
            ram,
            rom,
        }
    }
}

impl Instructionset {
    pub fn new(instructions: Vec<Box<dyn Instruction>>, cpu: CPUSpecs) -> Self {
        Self { instructions, cpu }
    }

    pub fn assemble_instruction(&self, tokens: &Vec<Token>) -> u64 {
        match &tokens[0] {
            Token::Mnemonic(mnem) => self
                .instructions
                .iter()
                .find(|instr| instr.mnemonic() == *mnem)
                .expect("Unknown mnemonic")
                .assemble(),
            _ => panic!("invalid first Token, Token needs to be mnemonic"),
        }
    }
}

