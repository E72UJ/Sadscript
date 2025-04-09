use std::env;
use std::io::{self, BufRead, BufReader};

fn main() {
    let script_file = match env::var("SADSCRIPT_FILE") {
        Ok(file) => file,
        Err(_) => {
            let args: Vec<String> = env::args().collect();
            if args.len() < 2 {
                println!("Usage: {} <script.sad> or set SADSCRIPT_FILE environment variable", args[0]);
                return;
            }
            args[1].clone()
        }
    };

    run_interpreter(&script_file);
}

fn run_interpreter(file_path: &str) {
    let file = std::fs::File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut interpreter = Interpreter::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        interpreter.interpret_line(&line);
    }
    interpreter.flush_output();
}

struct Interpreter {
    output: String,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            output: String::new(),
        }
    }

    fn interpret_line(&mut self, line: &str) {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("print") => {
                let value = tokens.collect::<Vec<&str>>().join(" ");
                self.output.push_str(&value.trim_matches('"'));
                self.output.push('\n');
            }
            _ => (),
        }
    }

    fn flush_output(&mut self) {
        print!("{}", self.output);
        self.output.clear();
    }
}