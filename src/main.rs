use maze::Maze;
use std::{env, time};

pub mod maze;
pub mod v2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: maze <width> <height> [print? default:true]");
        return;
    }

    let mut prt = true;
    if let Ok(_w) = args[1].parse::<u32>() {
        let w = _w;
        if let Ok(_h) = args[2].parse::<u32>() {
            let h = _h;
            if args.len() > 3 {
                if let Ok(_p) = args[3].parse::<bool>() {
                    prt = _p;
                }
            }
            run(w, h, prt);
        }
    }
}

fn run(w: u32, h: u32, prt: bool) {
    let timer = time::Instant::now();
    let res = Maze::gen(w, h);
    println!("time: {}ms", timer.elapsed().as_millis());

    let maze = match res {
        Ok(tmp) => tmp,
        Err(msg) => {
            println!("gen maze failure! msg:\n{}", msg);
            return;
        }
    };

    if prt {
        // let conn_char: Vec<char> = " ↓↑│→┘┐┤←└┌├─┴┬┼".chars().collect();
        let conn_char: Vec<char> = " ··│·┘┐┤·└┌├─┴┬┼".chars().collect();
        println!();
        for row in maze.map {
            let rs: String = row.iter().map(|&m| conn_char[m as usize] as char).collect();
            println!("{}", rs);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn ts() {
        let prt = false;
        let w = 512;
        let h = 512;
        run(w, h, prt);
    }
}
