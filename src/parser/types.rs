use std::str::FromStr;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub arg1: String,
    pub arg2: u32,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum CommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_IF,
    C_GOTO,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
    #[default]
    NULL,
}

impl Command {
    pub fn new(text: &str) -> Self {
        match text.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["push", arg1, arg2] => Command {
                command_type: CommandType::C_PUSH,
                arg1: arg1.to_string(),
                arg2: arg2.parse::<u32>().unwrap(),
            },
            ["pop", arg1, arg2] => Command {
                command_type: CommandType::C_POP,
                arg1: arg1.to_string(),
                arg2: arg2.parse::<u32>().unwrap(),
            },
            ["label", arg1] => Command {
                command_type: CommandType::C_LABEL,
                arg1: arg1.to_string(),
                arg2: 0,
            },
            ["if-goto", arg1] => Command {
                command_type: CommandType::C_IF,
                arg1: arg1.to_string(),
                arg2: 0,
            },
            [arith] => Command {
                command_type: CommandType::C_ARITHMETIC,
                arg1: arith.to_string(),
                arg2: 0,
            },
            _ => Command {
                command_type: CommandType::NULL,
                arg1: String::new(),
                arg2: 0,
            },
        }
    }
}
