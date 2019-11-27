use std::fmt;
use std::rc::Rc;
use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!(error),
    }
}

struct Vertex {
    name: String,
}

impl Vertex {
    #[allow(dead_code)]
    fn new() -> Vertex {
        Vertex {
            name: String::new(),
        }
    }

    fn from_name(name: &str) -> Vertex {
        Vertex {
            name: String::from(name),
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Vertex {}

struct Graph {
    vertices: Vec<Rc<Vertex>>,
    edges: Vec<(Rc<Vertex>, Rc<Vertex>)>,
}

impl Graph {
    fn new() -> Graph {
        Graph { vertices: Vec::new(), edges: Vec::new() }
    }
    fn add(&mut self, v: &Rc<Vertex>) {
        for w in &self.vertices {
            if w.name == v.name {
                return
            }
        }
        self.vertices.push(Rc::clone(v));
    }
    fn find(&mut self, s: &str) -> Option<Rc<Vertex>> {
        for w in &self.vertices {
            if w.name == s {
                return Some(Rc::clone(&w))
            }
        }
        None
    }
    fn connect(&mut self, v1: &Rc<Vertex>, v2: &Rc<Vertex>) {
        for (w1, w2) in &self.edges {
            // Either "if v1 == *w1 && v2 == *w2 || v1 == *w2 && v2 == *w1 {" or
            if v1 == w1 && v2 == w2 || v1 == w2 && v2 == w1 {
                return
            }
        }
        self.edges.push((Rc::clone(v1), Rc::clone(v2)))
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph[n={}, {}]", self.vertices.len(), self.edges
            .iter().map(|(a, b)| { format!("{}–{}", a.name, b.name) })
            .collect::<Vec<String>>().join(" "))
    }
}

#[derive(PartialEq)]
enum Op {
    Add,
    Connect,
    Print,
}

fn handle_input(g: &mut Graph, input: &String) {
    let input_trimmed = input.trim();
    let mut fields = input_trimmed.split_whitespace();

    let op = match fields.next() {
        Some("add") => Op::Add,
        Some("connect") => Op::Connect,
        Some("print") => Op::Print,
        Some(_) | None => {
            eprintln!("unknown operation");
            return
        },
    };

    match op {
        Op::Add => {
            match fields.next() {
                Some(val) => {
                    let v = Rc::new(Vertex::from_name(val));
                    g.add(&v)
                },
                None => {
                    eprintln!("add whom?");
                    return
                },
            }
        },
        Op::Connect => {
            let two = fields.take(2).collect::<Vec<&str>>();
            if two.len() != 2 {
                eprintln!("expected two arguments");
                return
            }
            let first = g.find(two[0]);
            let second = g.find(two[1]);
            if first.is_none() || second.is_none() {
                eprintln!("unknown vertices");
                return
            }
            g.connect(&first.unwrap(), &second.unwrap());
        },
        Op::Print => {
            println!("{}", g);
        }
    }
}

fn main() {
    let mut g = Graph::new();
    
    // Usage example
    /*let v1 = Rc::new(Vertex { name: "루카스".to_string() });
    let v2 = Rc::new(Vertex { name: "לוּקָס".to_string() });
    g.add(&v1);
    g.add(&v2);
    g.connect(&v1, &v2);*/

    // I/O wrapper
    loop {
        print!("⇐ ");
        io::stdout().flush().unwrap();
        let line = read_line();
        if line == "" || line == "bye\n" {
            break
        }

        handle_input(&mut g, &line);
        println!("⇒ {}", line)
    }

    println!("{}", g);
}

