use super::parser::CommandType;

pub struct CodeWriter {
    counter: u64,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn write_arithmetic(&mut self, operator: String) -> Vec<String> {
        match operator.as_str() {
            "add" => Self::operate_two("+"),
            "sub" => Self::operate_two("-"),
            "eq" => self.compare("JEQ"),
            "gt" => self.compare("JGT"),
            "lt" => self.compare("JLT"),
            "and" => Self::operate_two("&"),
            "or" => Self::operate_two("|"),
            "neg" => Self::operate_one("-"),
            "not" => Self::operate_one("!"),
            _ => todo!(),
        }
    }

    pub fn advance(&mut self, command: CommandType, arg1: String, arg2: u32) -> Vec<String> {
        let mut seg_name = String::new();
        let mut arg2 = arg2;
        let mut da = false;
        match arg1.as_str() {
            "local" => seg_name = "LCL".to_string(),
            "argument" => seg_name = "ARG".to_string(),
            "this" => seg_name = "THIS".to_string(),
            "that" => seg_name = "THAT".to_string(),
            "constant" => seg_name = "constant".to_string(),
            "pointer" => {
                if arg2 == 0 {
                    seg_name = "THIS".to_string()
                } else {
                    seg_name = "THAT".to_string()
                }
                da = true;
                arg2 = 0;
            }
            "static" => seg_name = format!("StaticTest.{}", arg2),
            "temp" => seg_name = "5".to_string(),
            _ => panic!(""),
        };

        match command {
            CommandType::C_PUSH => Self::push_segment(seg_name, arg2, da),
            CommandType::C_POP => Self::pop_segment(seg_name, arg2, da),
            CommandType::C_ARITHMETIC { command } => self.write_arithmetic(command),
            _ => panic!("Invalid command"),
        }
    }

    fn compare(&mut self, cond: &str) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];

        instructions.push("@SP".to_string());
        instructions.push("AM=M-1".to_string());
        instructions.push("D=M".to_string());
        instructions.push("A=A-1".to_string());
        instructions.push("D=M-D".to_string());
        instructions.push(format!("@JUMP{}", self.counter).to_string());
        instructions.push(format!("D;{}", cond).to_string());
        instructions.push("D=0".to_string());
        instructions.push(format!("@WRITER{}", self.counter).to_string());
        instructions.push("0;JMP".to_string());
        instructions.push(format!("(JUMP{})", self.counter).to_string());
        instructions.push("D=-1".to_string());
        instructions.push(format!("@WRITER{}", self.counter).to_string());
        instructions.push("0;JMP".to_string());
        instructions.push(format!("(WRITER{})", self.counter).to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M-1".to_string());
        instructions.push("M=D".to_string());
        // Self::decrease_sp(&mut instructions);
        self.counter += 1;

        instructions
    }

    fn operate_one(op: &str) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("A=A-1".to_string());
        instructions.push(format!("M={op}M").to_string());
        instructions
    }

    fn operate_two(op: &str) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("A=A-1".to_string());
        instructions.push("D=M".to_string());
        instructions.push("A=A-1".to_string());
        instructions.push(format!("M=M{}D", op).to_string());
        Self::decrease_sp(&mut instructions);

        instructions
    }

    fn pop_segment(seg: String, index: u32, da: bool) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];
        Self::calculate_seg(seg, index, da, &mut instructions);
        Self::decrease_sp(&mut instructions);
        Self::set_seg(&mut instructions);

        instructions
    }

    pub fn push_segment(seg: String, index: u32, da: bool) -> Vec<String> {
        let mut instructions: Vec<String> = vec![];
        match seg.as_str() {
            "constant" => {
                instructions.push(format!("@{}", index));
                instructions.push("D=A".to_string());
                instructions.push("@SP".to_string());
                instructions.push("A=M".to_string());
                instructions.push("M=D".to_string());
            }
            _ => Self::push_seg(seg, index, da, &mut instructions),
        };
        Self::increase_sp(&mut instructions);
        instructions
    }

    fn calculate_seg(seg: String, index: u32, da: bool, instructions: &mut Vec<String>) {
        println!("calculatoop, {}, {}", seg, index);
        instructions.push(format!("@{}", seg).to_string());
        if seg.as_str() == "5" || da {
            instructions.push("D=A".to_string());
        } else {
            instructions.push("D=M".to_string());
        }
        instructions.push(format!("@{}", index).to_string());
        instructions.push("D=D+A".to_string());
        instructions.push("@R13".to_string());
        instructions.push("M=D".to_string());
    }

    fn set_seg(instructions: &mut Vec<String>) {
        instructions.push("@R13".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
    }

    fn push_seg(seg: String, index: u32, da: bool, instructions: &mut Vec<String>) {
        instructions.push(format!("@{}", seg).to_string());
        if seg.as_str() == "5" || da {
            instructions.push("D=A".to_string());
        } else {
            instructions.push("D=M".to_string());
        }
        instructions.push(format!("@{}", index).to_string());
        instructions.push("A=D+A".to_string());
        instructions.push("D=M".to_string());
        instructions.push("@SP".to_string());
        instructions.push("A=M".to_string());
        instructions.push("M=D".to_string());
    }

    fn decrease_sp(instructions: &mut Vec<String>) {
        instructions.push("@SP".to_string());
        instructions.push("AM=M-1".to_string());
        instructions.push("D=M".to_string());
    }

    fn increase_sp(instructions: &mut Vec<String>) {
        instructions.push("@SP".to_string());
        instructions.push("M=M+1".to_string());
    }
}
