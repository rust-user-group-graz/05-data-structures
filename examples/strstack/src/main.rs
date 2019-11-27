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
    content: Vec<String>
}

impl Stack {
    fn new() -> Stack {
        Stack { content: Vec::<String>::new() }
    }

    fn push(&mut self, element: String) -> &mut Self {
        self.content.push(element);
        self
    }

    fn pop(&mut self) -> Option<String> {
        self.content.pop()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack[{}]", self.content.iter().map(|uint| { uint.to_string() }).collect::<Vec<String>>().join(" "))
    }
}

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

    let mut provided_string = String::new();
    if op == Op::Push {
        match fields.next() {
            Some(val) => {
                provided_string = String::from(val)
            },
            None => {
                eprintln!("I find your lack of arguments disturbing!");
                return;
            }
        }
    }

    match op {
        Op::Push => { s.push(provided_string); },
        Op::Pop => { println!("{:?}", s.pop()); },
        Op::Print => { println!("{}", s); },
    }
}

fn main() {
    let a = 4;
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
    println!("{:p} {:p}", &a, &s);
}
