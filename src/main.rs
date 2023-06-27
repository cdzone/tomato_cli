// const CLEAR: &str = "\x1B[2J\x1B[1;1H";
// const  CLEAR_TO_END: &str = "\033[K";
const BAR_WIDTH:usize = 35;
use std::{io::{self, Stdin, Write}, thread::sleep, process::Command};
use std::time;

fn print_menu(stdin:&Stdin) -> String {
    println!("\n\n--- 命令行番茄钟 ---\n菜单时间 {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("1. Pomodoro（25分钟）");
    println!("2. Short break（5分钟）");
    println!("3. Long break（10分钟）");
    println!("4. 退出");
    print!(">");
    let mut input_string = String::new();
    loop{
        io::stdout().flush().unwrap();
        input_string.clear();
        stdin.read_line(&mut input_string).unwrap();
        if ["1", "2", "3", "4"].contains(&input_string.trim()){
            return input_string.trim().into();
        } else {
            println!("输入有误，请重新输入");
            print!(">");
        }
}
}

fn convert_to_mstext(time:i32) -> String {
    let minutes = time / 60;
    let seconds = time % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

fn osx_terminal_notifier(title:&str, content:&str) {
    Command::new("terminal-notifier").args(["-message", content, "-title", title, "-sound", "default"]).spawn().unwrap();
}

fn progress_bar(progress_text:String, progress:i32, total:i32) {
    let progress_percent:f32 = 1.0 - progress as f32 /total as f32;
    let passed =  "*".repeat((progress_percent*BAR_WIDTH as f32 ) as usize);
    let left = " ".repeat(BAR_WIDTH-passed.len());
    print!("\r{}/{} [{}{}] {}%", progress_text, convert_to_mstext(total), passed, left, (progress_percent*100.0) as i32 );
    io::stdout().flush().unwrap();
}

fn countdown_tomato(time:i32) {
    let mut time_remain = time;
    let total_time = time;
    while time_remain > 0 {
        time_remain = time_remain - 1 ;
        let remain_text = convert_to_mstext(time_remain); 
        // println!("Time remaining: {}", time_remain);
        progress_bar(remain_text, time_remain, total_time);
        sleep(time::Duration::from_secs(1));
    }
    osx_terminal_notifier("倒计时结束", "请选择下一项任务");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        let input_cmd = print_menu(&stdin);
        // println!("input cmd: {}", input_cmd);
        let time_length = match input_cmd.as_str() {
            "1" => 1500,
            "2" => 300,
            "3" => 600,
            &_ => break,
        };
        let count_handler = std::thread::spawn(move || countdown_tomato(time_length));
        count_handler.join().unwrap();
    }
    Ok(())
}
