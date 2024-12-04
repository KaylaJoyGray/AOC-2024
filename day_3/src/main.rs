// use regex::Regex;

fn main() {
    let input = include_str!("../input");
    let mut parser = Parser::new();

    println!("Sum of mul instructions: {}", parser.parse(input, false));

    println!(
        "Sum of mul instructions with do/don't: {}",
        parser.parse(input, true)
    )
}

// fn parse_regex(input: &str) -> i32 {
//     let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
//
//     regex
//         .captures_iter(input)
//         .map(|c| c.extract())
//         .filter_map(|(_, [a, b])| Some(a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()))
//         .collect::<Vec<i32>>()
//         .iter()
//         .sum()
// }

#[derive(Debug)]
enum Command {
    Mul,
    Do,
    Dont,
    Invalid,
}

#[derive(Default)]
struct CommandStack {
    chars: Vec<char>,
}

impl CommandStack {
    fn push(&mut self, c: char) {
        self.chars.push(c);
    }

    fn resolve_command(&self) -> Command {
        let str = self.chars.iter().collect::<String>();
        let len = str.chars().count();

        if len > 4 {
            if &str[len - 5..] == "don't" {
                return Command::Dont;
            }
        }

        if len > 2 {
            if &str[len - 3..] == "mul" {
                return Command::Mul;
            }
        }

        if len > 1 {
            if &str[len - 2..] == "do" {
                return Command::Do;
            }
        }

        Command::Invalid
    }

    fn clear(&mut self) {
        self.chars.clear()
    }
}

#[derive(Default)]
struct NumStack {
    chars: Vec<char>,
}

impl NumStack {
    fn push(&mut self, c: char) {
        self.chars.push(c);
    }

    fn resolve_number(&self) -> Option<i32> {
        self.chars.iter().collect::<String>().parse::<i32>().ok()
    }

    fn clear(&mut self) {
        self.chars.clear()
    }
}

struct Parser {
    enabled: bool,
    mul: bool,

    cmd_stack: CommandStack,
    num_stack: NumStack,

    op1: Option<i32>,
    op2: Option<i32>,
}

impl Parser {
    fn new() -> Self {
        Self {
            enabled: true,
            mul: false,
            cmd_stack: CommandStack::default(),
            num_stack: NumStack::default(),
            op1: None,
            op2: None,
        }
    }

    fn parse(&mut self, input: &str, allow_non_mul: bool) -> i32 {
        let mut result = 0;

        for c in input.chars() {
            if c == '(' {
                if allow_non_mul {
                    match self.cmd_stack.resolve_command() {
                        Command::Mul => self.mul = true,
                        Command::Do => self.enabled = true,
                        Command::Dont => self.enabled = false,
                        Command::Invalid => self.reset(),
                    }
                } else {
                    match self.cmd_stack.resolve_command() {
                        Command::Mul => self.mul = true,
                        _ => self.reset(),
                    }
                }
                self.cmd_stack.clear();
            } else if c == ')' {
                if self.enabled && self.mul {
                    self.op2 = self.num_stack.resolve_number();
                    self.num_stack.clear();
                    result += self.multiply();
                }
                self.reset()
            } else if c == ',' && self.mul {
                self.op1 = self.num_stack.resolve_number();
                self.num_stack.clear();
            } else if c.is_digit(10) {
                self.num_stack.push(c);
            } else if c.is_alphabetic() || c == '\'' {
                self.cmd_stack.push(c);
            } else {
                self.reset()
            }
        }

        self.enabled = true;
        self.reset();
        result
    }

    fn multiply(&self) -> i32 {
        if let Some(op1) = self.op1 {
            if let Some(op2) = self.op2 {
                return op1 * op2;
            }
        }

        0
    }

    fn reset(&mut self) {
        self.num_stack.clear();
        self.cmd_stack.clear();
        self.mul = false;
        self.op1 = None;
        self.op2 = None;
    }
}
