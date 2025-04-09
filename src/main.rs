// 导入所需的标准库模块
use std::env;  // 用于处理环境变量
use std::io::{self, BufRead, BufReader};  // 用于文件读取操作

fn main() {
    // 尝试从环境变量获取脚本文件路径，或从命令行参数获取
    let script_file = match env::var("SADSCRIPT_FILE") {
        Ok(file) => file,  // 如果环境变量存在，使用它
        Err(_) => {  // 如果环境变量不存在，查找命令行参数
            let args: Vec<String> = env::args().collect();
            if args.len() < 2 {
                // 如果没有提供足够的命令行参数，显示使用说明并退出
                println!("Usage: {} <script.sad> or set SADSCRIPT_FILE environment variable", args[0]);
                return;
            }
            args[1].clone()  // 使用第一个命令行参数作为脚本文件路径
        }
    };
    
    // 使用获取到的脚本文件路径运行解释器
    run_interpreter(&script_file);
}

// 运行解释器的函数，接收脚本文件路径作为参数
fn run_interpreter(file_path: &str) {
    // 打开指定的脚本文件，如果失败则panic
    let file = std::fs::File::open(file_path).expect("Failed to open file");
    // 创建一个缓冲读取器来高效读取文件
    let reader = BufReader::new(file);
    
    // 创建新的解释器实例
    let mut interpreter = Interpreter::new();
    // 逐行读取并解释文件内容
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        interpreter.interpret_line(&line);
    }
    // 解释完成后，输出所有结果
    interpreter.flush_output();
}

// 解释器结构体定义
struct Interpreter {
    output: String,  // 用于存储输出结果的字符串
}

impl Interpreter {
    // 创建新的解释器实例
    fn new() -> Self {
        Interpreter {
            output: String::new(),
        }
    }
    
    // 解释单行代码的方法
    fn interpret_line(&mut self, line: &str) {
        // 将行内容分割成空格分隔的标记
        let mut tokens = line.split_whitespace();
        // 获取第一个标记（命令）
        match tokens.next() {
            // 如果命令是"print"，处理打印操作
            Some("print") => {
                // 将剩余标记合并为一个字符串
                let value = tokens.collect::<Vec<&str>>().join(" ");
                // 移除引号并将结果添加到输出缓冲区
                self.output.push_str(&value.trim_matches('"'));
                // 添加换行符
                self.output.push('\n');
            }
            // 忽略其他命令（当前版本只实现了print命令）
            _ => (),
        }
    }
    
    // 输出并清空所有缓冲的内容
    fn flush_output(&mut self) {
        print!("{}", self.output);
        self.output.clear();
    }
}