#![allow(dead_code)]
use super::types::{Command, CommandType};
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Parser {
    pub lines: Vec<String>,

    // Current command
    pub command: Command,

    // Current command type
    pub command_type: CommandType,

    // First argument of the current line
    pub arg1: String,

    // Second argument of the current line
    pub arg2: u32,
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_file(&mut self, file_name: &str) {
        let file = std::fs::File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        self.lines = reader.lines().collect::<Result<_, _>>().unwrap();
        // call Sys.init when processing Sys.vm file
        if file_name.contains("Sys.vm") {
            self.lines.insert(0, "call Sys.init 0".to_string());
        }
    }

    pub fn advance(&mut self) {
        let mut line = self.lines.remove(0);
        line = line.split("//").next().unwrap_or("").trim().to_string();

        let command = Command::new(&line);
        if command.command_type == CommandType::NULL && self.has_more_commands() {
            return self.advance();
        }

        self.command = command.clone();
        self.command_type = command.command_type.clone();
        self.arg1 = command.arg1.clone(); // target stack
        self.arg2 = command.arg2.clone(); // index
    }

    pub fn has_more_commands(&self) -> bool {
        self.lines.len() > 0
    }
}
