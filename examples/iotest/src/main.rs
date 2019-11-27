use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            eprintln!("{} bytes, {:?}, {:?}", n, String::from(&input).into_bytes(), input);
            input
        }
        Err(error) => panic!(error),
    }
}

fn main() {
    loop {
        print!("⇐ ");
        io::stdout().flush().unwrap();
        let line = read_line();
        if line == "" || line == "bye\n" {
            break
        }
        println!("⇒ {}", line)
    }
}
