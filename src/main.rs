use maze::Maze;


pub mod maze;
pub mod v2;

fn main() {
    run();
}

fn run() {
    let _ = Maze::gen(10, 10);
}

