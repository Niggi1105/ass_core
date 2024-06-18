use core::panic;
use std::{
    fs::File,
    io::{BufRead, Read},
};

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
}

pub struct Program {
    lines: Vec<Vec<Token>>,
}

impl Program {
    pub fn new(lines: Vec<Vec<Token>>) -> Self {
        Self { lines }
    }

    pub fn from_file(path: String) -> Self {
        let mut f = File::open(path).expect("can't open source file");
        let mut source_text = String::new();
        f.read_to_string(&mut source_text)
            .expect("can't read source file");
        let lines = source_text
            .lines()
            .map(|line| {
                let raw_tokens: Vec<&str> = line.split_ascii_whitespace().collect();
                let mut tokens = Vec::new();
                if raw_tokens.len() == 0 {
                    tokens
                } else if raw_tokens[0][0..1] == *"//" {
                    tokens
                } else {
                    tokens.push(Token::Mnemonic(raw_tokens[0].to_string()));
                    tokens.append(&mut Self::tokenize(raw_tokens[1..].to_vec()));
                    tokens
                }
            })
            .filter(|line| line.len() >= 1)
            .collect();
        Self { lines }
    }

    fn tokenize(raw_tk: Vec<&str>) -> Vec<Token> {
        raw_tk
            .iter()
            .map(|tk| {
                if tk[0..0] == *"%" {
                    Token::Register(tk[1..].to_string())
                } else if let Ok(n) = tk.parse::<i64>() {
                    Token::Number(n)
                } else {
                    panic!("unkown token")
                }
            })
            .collect()
    }
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
