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
        println!("usage: maze <width> <height> [print? default:true]");
        return;
    }

    let mut prt = true;
    let mut res: Result<Maze, String> = Err("invalid width or height!".to_string());
    if let Ok(w) = args[1].parse::<u32>() {
        if let Ok(h) = args[2].parse::<u32>() {
            res = Maze::gen(w, h);
            if args.len() > 3 {
                if let Ok(pt) = args[3].parse::<bool>() {
                    prt = pt;
                }
            }
        }
    }

    let maze: Maze;
    match res {
        Ok(tmp) => maze = tmp,
        Err(msg) => {
            println!("gen maze failure! msg:\n{}", msg);
            return;
        }
    }

    if prt {
        // let conn_char: Vec<char> = " ↓↑│→┘┐┤←└┌├─┴┬┼".chars().collect();
        let conn_char: Vec<char> = " ··│·┘┐┤·└┌├─┴┬┼".chars().collect();
        println!();
        for row in maze.map {
            for val in row {
                print!("{}", conn_char[val as usize]);
                // print!("{}", val as usize);
            }
            println!();
        }
        println!();
    }
}
