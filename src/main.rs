const BAR_WIDTH:usize = 35;
use std::{io::{self, Stdin, Write}, thread::sleep, process::{Command, Stdio}, path::Path};
use std::time;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="cdzone<cdzone@yeah.net>", version="0.2", about="A simple cli tomato clock.", long_about = None)]
struct Args {
    #[clap(short='s', long="shout_loud", value_parser, default_value_t = 1)]
    shout_times: i32,
    #[clap(short='n', long="notify_sound", value_parser)]
    notify_sound:Option<String>,
}


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

fn osx_terminal_notifier(title:&str, content:&str, sound:Option<String>) {
    if let Some(sound_path) = sound {
        if check_path_exist(&sound_path) {
            Command::new("terminal-notifier").args(["-message", content, "-title", title]).spawn().unwrap();
            Command::new("ffplay").args(["-i", &sound_path, "-autoexit", "-nodisp"]).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
            return ;
        } 
    }
    Command::new("terminal-notifier").args(["-message", content, "-title", title, "-sound", "default"]).spawn().unwrap();
}

fn check_path_exist(path:&str) -> bool {
    let path_obj = Path::new(path);
    if path_obj.exists() {
        true
    } else {
        println!("{path} not exist!");
        false
    }
}

fn progress_bar(progress_text:String, progress:i32, total:i32) {
    let progress_percent:f32 = 1.0 - progress as f32 /total as f32;
    let passed =  "*".repeat((progress_percent*BAR_WIDTH as f32 ) as usize);
    let left = " ".repeat(BAR_WIDTH-passed.len());
    print!("\r{}/{} [{}{}] {}%", progress_text, convert_to_mstext(total), passed, left, (progress_percent*100.0) as i32 );
    io::stdout().flush().unwrap();
}

fn countdown_tomato(time:i32, notify_times:i32, shout_sound:Option<String>) {
    let mut time_remain = time;
    let total_time = time;
    while time_remain > 0 {
        time_remain = time_remain - 1 ;
        let remain_text = convert_to_mstext(time_remain); 
        // println!("Time remaining: {}", time_remain);
        progress_bar(remain_text, time_remain, total_time);
        sleep(time::Duration::from_secs(1));
    }
    let mut notified_times = 0 ;
    while notified_times < notify_times{
        let shout_sound_dup = shout_sound.clone();
        osx_terminal_notifier("倒计时结束", "请选择下一项任务", shout_sound_dup);
        notified_times = notified_times + 1; 
        sleep(time::Duration::from_secs(10));
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let shout_times = args.shout_times;
    let stdin = io::stdin();
    let shout_sound = args.notify_sound;
    loop {
        let input_cmd = print_menu(&stdin);
        // println!("input cmd: {}", input_cmd);
        let time_length = match input_cmd.as_str() {
            "1" => 1500,
            "2" => 300,
            "3" => 600,
            &_ => break,
        };
        let shout_sound_arg = shout_sound.clone();
        let count_handler = std::thread::spawn(move || countdown_tomato(time_length, shout_times, shout_sound_arg));
        count_handler.join().unwrap();
    }
    Ok(())
}
