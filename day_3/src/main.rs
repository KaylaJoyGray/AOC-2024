use regex::Regex;

fn main() {
    // xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    // Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

    let input = include_str!("../input");

    println!("Sum of mul instructions: {}", parse(input));
    println!("Sum of mul instructions with do/don't: {}", parse_2(input))
}

fn parse(input: &str) -> i64 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .filter_map(|(_, [a, b])| Some(a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()))
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

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

    fn resolve_command(&mut self) -> Command {
        let str = self.chars.iter().collect::<String>();
        let len = str.chars().count();

        self.chars.clear();

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

fn multiply(op1: Option<i64>, op2: Option<i64>) -> i64 {
    if let Some(op1) = op1 {
        if let Some(op2) = op2 {
            return op1 * op2;
        }
    }

    0
}

fn parse_2(input: &str) -> i64 {
    // The do() instruction enables future mul instructions.
    // The don't() instruction disables future mul instructions.
    //
    // Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

    let mut result: i64 = 0;

    let mut enabled: bool = true;
    let mut mul: bool = false;

    let mut cmd_stack = CommandStack::default();
    let mut num_stack: Vec<char> = Vec::new();

    let mut op1: Option<i64> = None;
    let mut op2: Option<i64> = None;

    for c in input.chars() {
        if c == '(' {
            let cmd = cmd_stack.resolve_command();
            match cmd {
                Command::Mul => mul = true,
                Command::Do => enabled = true,
                Command::Dont => enabled = false,
                Command::Invalid => {
                    num_stack.clear();
                    cmd_stack.clear();
                    mul = false;
                }
            }
        } else if c == ')' {
            if enabled && mul {
                op2 = num_stack.iter().collect::<String>().parse::<i64>().ok();
                num_stack.clear();
                result += multiply(op1, op2);
            }
            num_stack.clear();
            cmd_stack.clear();
            mul = false;
        } else if c == ',' {
            if mul {
                op1 = num_stack.iter().collect::<String>().parse::<i64>().ok();
                num_stack.clear();
            }
        } else if c.is_digit(10) {
            num_stack.push(c);
        } else if c.is_alphanumeric() || c == '\'' {
            cmd_stack.push(c);
        } else {
            num_stack.clear();
            cmd_stack.clear();
            mul = false;
        }
    }

    result
}
