use std::str::FromStr;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum CommandType {
    C_ARITHMETIC {
        command: String,
    },
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
    #[default]
    NULL,
}

impl FromStr for CommandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lows = s.to_lowercase();
        match lows.as_str() {
            "push" => Ok(Self::C_PUSH),
            "pop" => Ok(Self::C_POP),
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                Ok(Self::C_ARITHMETIC { command: lows })
            }
            _ => Err("Slap".to_string()),
        }
    }
}
