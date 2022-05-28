use maze::Maze;
use std::env;

pub mod maze;
pub mod v2;

fn main() {
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: maze <width> <height>");
        return;
    }
    if let Ok(w) = args[1].parse::<u32>() {
        if let Ok(h) = args[2].parse::<u32>() {
            if let Err(msg) = Maze::gen(w, h) {
                println!("gen maze failure! msg:\n{}", msg)
            }
        }
    }
}

