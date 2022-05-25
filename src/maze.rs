// --- use
use crate::v2::{Dir, Pos, D, L, R, U};
use rand::{self, prelude::ThreadRng, Rng};

/// limit
const W_MIN: u32 = 3;
const H_MIN: u32 = 3;
const W_MAX: u32 = i32::MAX as u32;
const H_MAX: u32 = i32::MAX as u32;

/// pass dir at position
pub struct Mark {
    p: Pos,
    crs: u8,
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
            crs: 0,
        }
    }
}

impl Maze {
    pub fn gen(w: u32, h: u32) -> Result<Maze, String> {
        if w < W_MIN || w > W_MAX || h < H_MIN || h > H_MAX {
            return Err("范围不合适".to_string());
        }
        // init map
        let map = init(w, h);
        let mut maze = Maze { w, h, map };
        // dig
        dig(&mut maze);

        return Err("未知异常".to_string());
    }
}

fn init(w: u32, h: u32) -> Vec<Vec<u8>> {
    let mut vv: Vec<Vec<u8>> = Vec::with_capacity(w as usize);
    for _ in 0..w {
        let mut v: Vec<u8> = Vec::with_capacity(h as usize);
        v.fill(0);
        vv.push(v);
    }
    return vv;
}

/// 挖掘通道
fn dig(maze: &mut Maze) {
    let mut p: Pos;
    let map = &mut maze.map;
    // 剩余数量
    let mut more = maze.w * maze.h;
    // rand
    let mut rng = rand::thread_rng();
    // mark when truning
    let mut corners: Vec<Pos> = Vec::with_capacity(more as usize);

    // start at rand pos
    p = Pos::at(rng.gen_range(0..maze.w), rng.gen_range(0..maze.h));
    while more > 0 {
        // 根据四周情况随机取方向
        let rd: u8 = rdir(&mut rng);
        // 如果没有可用方向，回溯至上一个拐点
        // 根据方向获取坐标
        // 连接两点
        more -= 1;
    }
}

/// 获取四周的情况
fn nearby(p: &Pos, map: &Vec<Vec<u8>>, just_unk: bool) -> Vec<Mark> {
    let ds = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    let mut v: Vec<Mark> = Vec::with_capacity(ds.len());
    for d in ds {
        if let Some(n) = p.peek(&d) {
            let m = get_mark(p, map);
            if m < 1 && just_unk { continue }
            v.push(Mark { p: n, crs: d.get() });
        }
    }
    return v;
}

/// 随机方向
fn rdir(rng: &mut ThreadRng) -> u8 {
    let x = rng.gen_range(1u8..5u8);
    match x {
        1 | 2 => x,
        3 => L,
        4 => R,
        _ => 0,
    }
}

/// get mark
fn get_mark(p: &Pos, vv: &Vec<Vec<u8>>) -> u8 {
    let v = &vv[p.0 as usize];
    return *(&v[p.1 as usize]);
}
