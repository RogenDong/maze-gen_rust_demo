// --- use
use crate::v2::{rev, Dir, Pos, D, L, R, U};
use rand::{self, Rng};

/// limit
const W_MIN: u32 = 3;
const H_MIN: u32 = 3;
const W_MAX: u32 = i32::MAX as u32;
const H_MAX: u32 = i32::MAX as u32;
const DIR_LS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

/// pass dir at position
pub struct Mark {
    p: Pos,
    val: u8,
}

/// maze
pub struct Maze {
    w: u32,
    h: u32,
    map: Vec<Vec<u8>>,
}

// --- impl

impl Mark {
    pub fn new() -> Mark {
        Mark {
            p: Pos::at(0, 0),
            val: 0,
        }
    }
}

impl Maze {
    pub fn gen(w: u32, h: u32) -> Result<Maze, String> {
        if w < W_MIN || w > W_MAX || h < H_MIN || h > H_MAX {
            return Err("范围不合适".to_string());
        }
        println!("gen start({} * {})", w, h);
        // init map
        let mut maze = Maze {
            w,
            h,
            map: vec![vec![0; h as usize]; w as usize],
        };
        // dig
        dig(&mut maze);

        println!("finish");
        return Ok(maze);
    }
}

/// 挖掘通道
fn dig(maze: &mut Maze) {
    let mut pre_d = 0u8;
    // 剩余数量
    let mut more = maze.w * maze.h;
    // 点位表
    let map = &mut maze.map;
    // random number generator
    let mut rng = rand::thread_rng();
    // 拐点
    let mut corners: Vec<Pos> = Vec::with_capacity(more as usize);
    // start pos
    let mut p = Pos::at(rng.gen_range(0..maze.w), rng.gen_range(0..maze.h));
    println!("start at ({},{})", p.0, p.1);

    let count = maze.w * maze.h;
    while more > 0 {
        let tc = count - more;
        print!("\r{}({}%)", tc, tc / count * 100);
        let next_d: u8;
        let opt_len: usize;
        // 四周情况
        let opt_d = nearby(&p, &map, true);
        opt_len = opt_d.len();
        // 如果没有可用方向，回溯至上一个拐点
        if opt_len < 1 {
            if let Some(bp) = corners.pop() {
                println!("\rback");
                p = bp;
                continue;
            } else {
                println!("\nno corners");
                break;// 没有拐点，结束
            }
        }
        // 取方向 + 坐标
        let tmp = if opt_len < 2 {
            opt_d.get(0)
        } else {
            opt_d.get(rng.gen_range(0..opt_len))
        }
        .unwrap();
        next_d = tmp.val; // val here is dir-val
        p.goto(&tmp.p);
        dig_to(&p, rev(pre_d) + next_d, map);
        // mark turning
        if pre_d > 0 && next_d != pre_d {
            corners.push(Pos::cp(&p));
        }
        pre_d = next_d;
        more -= 1;
    }
    println!("end dig, more={}", more);
}

/// 获取四周的情况
fn nearby(p: &Pos, map: &Vec<Vec<u8>>, just_unk: bool) -> Vec<Mark> {
    let mut v: Vec<Mark> = Vec::with_capacity(4);
    for d in DIR_LS {
        if let Some(n) = p.peek(&d) {
            let m = get_mark(p, map);
            if m > 0 && just_unk {
                // println!("\r({},{})={}", n.0, n.1, m);
                continue;
            }
            v.push(Mark { p: n, val: d.val() });
        }
    }
    return v;
}

fn get_mark(p: &Pos, map: &Vec<Vec<u8>>) -> u8 {
    let v = &map[p.0 as usize];
    return *(&v[p.1 as usize]);
}

fn dig_to(p: &Pos, mark: u8, map: &mut Vec<Vec<u8>>) {
    let v = &mut map[p.0 as usize];
    let y = p.1 as usize;
    if v[y] < U + D + L + R {
        v[y] = mark;
    }
}
