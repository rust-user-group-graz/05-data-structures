use std::fmt;
use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!(error),
    }
}

#[derive(Debug)]
struct Stack {
    content: Vec<u64>
}

impl Stack {
    fn new() -> Stack {
        Stack { content: Vec::<u64>::new() }
    }

    fn push(&mut self, element: u64) -> &mut Self {
        self.content.push(element);
        self
    }

    fn pop(&mut self) -> Option<u64> {
        self.content.pop()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack[{}]", self.content.iter().map(|uint| { uint.to_string() }).collect::<Vec<String>>().join(" "))
    }
}

// First implementation w/o operation distinction
/*fn handle_input(s: &mut Stack, input: &String) {
    let mut provided_integer = 0;
    let mut valid = true;

    let input_trimmed = input.trim();
    match input_trimmed.parse::<u64>() {
        Ok(val) => provided_integer = val,
        Err(_) => {
            println!("… is not an integer");
            valid = false;
        },
    }

    if valid {
        s.push(provided_integer);
    }
}*/

#[derive(PartialEq)]
enum Op {
    Pop,
    Push,
    Print,
}

fn handle_input(s: &mut Stack, input: &String) {
    let input_trimmed = input.trim();
    let mut fields = input_trimmed.split_whitespace();

    let op = match fields.next() {
        Some("push") => Op::Push,
        Some("pop") => Op::Pop,
        Some("print") => Op::Print,
        Some(_) | None => {
            eprintln!("unknown operation");
            return;
        },
    };

    let mut provided_integer = 0;
    if op == Op::Push {
        match fields.next() {
            Some(val) => {
                match val.parse::<u64>() {
                    Ok(val) => provided_integer = val,
                    Err(_) => {
                        eprintln!("… is not an integer");
                        return;
                    },
                }
            },
            None => {
                eprintln!("I find your lack of arguments disturbing!");
                return;
            }
        }
    }

    match op {
        Op::Push => { s.push(provided_integer); },
        Op::Pop => { println!("{:?}", s.pop()); },
        Op::Print => { println!("{}", s); },
    }
}

fn main() {
    let mut s = Stack::new();

    loop {
        print!("⇐ ");
        io::stdout().flush().unwrap();
        let line = read_line();
        if line == "" || line == "bye\n" {
            break
        }

        handle_input(&mut s, &line);

        println!("⇒ {}", line)
    }

    println!("{}", s);
}
