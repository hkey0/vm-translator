#![allow(dead_code)]
use super::types::CommandType;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Parser {
    pub lines: Vec<String>,

    // Current command
    pub command: CommandType,

    // First argument of the current line
    pub arg1: String,

    // Second argument of the current line
    pub arg2: u32,
}

impl Parser {
    pub fn new(file_name: &str) -> Self {
        let file = std::fs::File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

        Self {
            lines,
            ..Default::default()
        }
    }

    pub fn advance(&mut self) {
        let mut line = self.lines.remove(0);
        line = line.split("//").next().unwrap_or("").trim().to_string();

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 1 {
            let command = parts[0].parse::<CommandType>().unwrap();
            self.command = command;
            return;
        } else if parts.len() == 2 {
            match parts[0] {
                "if-goto" => {
                    self.command = CommandType::C_IF {
                        name: parts[1].to_string(),
                    }
                }
                "label" => {
                    self.command = CommandType::C_LABEL {
                        name: parts[1].to_string(),
                    }
                }
                _ => panic!(),
            }
            return;
        } else if parts.len() != 3 && self.has_more_commands() {
            return self.advance();
        }

        self.command = parts[0].parse::<CommandType>().unwrap();
        self.arg1 = parts[1].to_string(); // target stack
        self.arg2 = parts[2].parse::<u32>().unwrap(); // index
    }

    pub fn has_more_commands(&self) -> bool {
        self.lines.len() > 0
    }
}
