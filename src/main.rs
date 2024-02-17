mod check_graphics_server;

use std::env;
use std::process::exit;
use std::string::String;
use check_graphics_server::{GraphicsSystem, get_graphics_system};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage:\nwayland-checker [Options]...\nOptions:\n  -p, --pid\n  -h, --help");
        exit(1);
    }

    let pid_index = args
        .iter()
        .position(|arg| arg == "-p" || arg == "--pid")
        .unwrap_or_else(|| {
            eprintln!("Missing --pid option");
            exit(1);
        });

    let pid = match args.get(pid_index + 1) {
        Some(pid_str) => match pid_str.parse::<u32>() {
            Ok(pid) => pid,
            Err(_) => {
                eprintln!("Invalid PID: {}", pid_str);
                exit(1);
            }
        },
        None => {
            eprintln!("Missing value for --pid option");
            exit(1);
        }
    };

    match get_graphics_system(pid) {
        GraphicsSystem::Wayland  => println!("PID: \x1b[32m{}\x1b[0m used Wayland",  pid),
        GraphicsSystem::XWayland => println!("PID: \x1b[32m{}\x1b[0m used XWayland", pid),
        GraphicsSystem::CLI      => println!("PID: \x1b[32m{}\x1b[0m used CLI",      pid),
        GraphicsSystem::Unknown  => println!("PID: \x1b[32m{}\x1b[0m used Unknown",  pid),
    }
}
