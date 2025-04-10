use clap::{Parser, Subcommand};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process;

/// 简单的脚本语言解释器
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 要执行的脚本文件
    #[arg(value_name = "FILE")]
    script_file: Option<PathBuf>,

    /// 设置日志级别（保留参数）
    #[arg(short, long, value_name = "LEVEL", default_value = "info")]
    log_level: String,

    /// 启用严格模式
    #[arg(short, long)]
    strict: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 交互式运行解释器
    Repl {},
    
    /// 执行代码片段
    Exec {
        /// 要执行的代码
        #[arg(value_name = "CODE")]
        code: String,
    },
}

struct Interpreter {
    output: String,
    strict_mode: bool,
}

impl Interpreter {
    fn new(strict_mode: bool) -> Self {
        Interpreter {
            output: String::new(),
            strict_mode,
        }
    }

    fn interpret_line(&mut self, line: &str) {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("print") => {
                let value = tokens.collect::<Vec<&str>>().join(" ");
                let cleaned = value.trim_matches('"');
                
                if self.strict_mode && cleaned.contains('\\') {
                    eprintln!("严格模式错误：禁止使用转义字符");
                    return;
                }
                
                self.output.push_str(cleaned);
                self.output.push('\n');
            }
            Some(cmd) if self.strict_mode => {
                eprintln!("严格模式错误：未知命令 '{}'", cmd);
            }
            Some(_) => {} // 非严格模式忽略未知命令
            None => {}
        }
    }

    fn flush_output(&mut self) {
        print!("{}", self.output);
        self.output.clear();
    }
}

fn main() {
    let cli = Cli::parse();

    // 处理环境变量配置
    let config_path = env::var("MY_SCRIPT_CONFIG")
        .unwrap_or_else(|_| "~/.myscript/config".to_string());
    let debug_mode = env::var("MY_SCRIPT_DEBUG")
        .unwrap_or_else(|_| "false".to_string()) == "true";

    println!("配置文件路径: {}", config_path);
    println!("调试模式: {}", debug_mode);

    match &cli.command {
        Some(Commands::Repl {}) => {
            println!("启动交互式解释器...");
            run_repl(cli.strict);
        }
        Some(Commands::Exec { code }) => {
            println!("执行代码: {}", code);
            execute_code(code.as_str(), cli.strict); // 修正此处
        }
        None => {
            if let Some(path) = cli.script_file {
                execute_script_file(&path, cli.strict);
            } else {
                println!("没有提供命令或脚本文件。使用 --help 查看帮助信息。");
                process::exit(1);
            }
        }
    }
}

fn run_repl(strict_mode: bool) {
    println!("REPL 模式 (严格模式: {})", strict_mode);
    let mut interpreter = Interpreter::new(strict_mode);
    
    let stdin = io::stdin();
    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,  // Ctrl+D
            Ok(_) => {
                let line = line.trim();
                if line.eq_ignore_ascii_case("exit") {
                    break;
                }
                interpreter.interpret_line(line);
                interpreter.flush_output();
            }
            Err(e) => {
                eprintln!("读取输入错误: {}", e);
                break;
            }
        }
    }
}

fn execute_code(code: &str, strict_mode: bool) {
    let mut interpreter = Interpreter::new(strict_mode);
    interpreter.interpret_line(code);
    interpreter.flush_output();
}

fn execute_script_file(path: &PathBuf, strict_mode: bool) {
    println!("执行脚本文件 (严格模式: {}): {:?}", strict_mode, path);
    
    match File::open(path) {
        Ok(file) => {
            let mut interpreter = Interpreter::new(strict_mode);
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                match line {
                    Ok(line) => interpreter.interpret_line(&line),
                    Err(e) => {
                        eprintln!("读取行错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            
            interpreter.flush_output();
        }
        Err(e) => {
            eprintln!("无法打开文件: {}", e);
            process::exit(1);
        }
    }
}