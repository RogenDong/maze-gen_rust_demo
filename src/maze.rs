// --- use
use crate::v2::{rev, Dir, Pos, D, L, R, U};
use rand::{self, prelude::ThreadRng, Rng};

// --- 常量
// 限制
const W_MIN: u32 = 3;
const H_MIN: u32 = 3;
const W_MAX: u32 = i32::MAX as u32;
const H_MAX: u32 = i32::MAX as u32;

// 支持 迭代
/// 方向枚举集合
const DIR_LS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

// --- 结构
/// 点位标记
pub struct Mark {
    p: Pos,
    val: u8,
}

/// 迷宫
pub struct Maze {
    w: u32,
    h: u32,
    pub map: Vec<Vec<u8>>,
}

// --- impl

impl Maze {
    pub fn gen(w: u32, h: u32) -> Result<Maze, String> {
        if w < W_MIN || w > W_MAX || h < H_MIN || h > H_MAX {
            return Err("范围不合适".to_string());
        }
        println!("gen start({} * {})", w, h);
        // 初始化
        let mut maze = Maze {
            w,
            h,
            map: vec![vec![0u8; w as usize]; h as usize],
        };
        // 开通
        let more = maze.dig();
        // 查漏
        if more > 0 {
            maze.check(more);
        }
        println!("\n finished");
        // return
        Ok(maze)
    }

    /// 挖掘通道
    fn dig(&mut self) -> u32 {
        let mut pre_d = 0u8;
        let all = self.w * self.h;
        // 剩余数量
        let mut more = all;
        // 随机数生成器
        let mut rng = rand::thread_rng();
        // 拐点
        let mut corners: Vec<Pos> = Vec::with_capacity(more as usize);
        // 坐标 —— 首个随机
        let mut p = Pos {
            x: rng.gen_range(0..self.w),
            y: rng.gen_range(0..self.h),
        };
        println!("start at ({},{})", p.x, p.y);

        loop {
            prgs(all, more);
            if more < 1 {
                break;
            }

            // 步进向四周随机一个方向
            let ops: Vec<Mark> = self.nearby(&p, true);
            if let Some(dest) = ran_one(&ops, &mut rng) {
                // dest.val 表示其相对于 p 的方向
                self.dig_to(&p, dest);

                // 保存拐点
                let next_d = dest.val;
                if pre_d > 0 && next_d != pre_d {
                    corners.push(Pos::cp(&p));
                }

                p.goto(&dest.p);
                pre_d = next_d;
                more -= 1;
            } else {
                // 如果没有可用方向，回溯至上一个拐点
                if let Some(next) = corners.pop() {
                    p = next;
                    continue;
                } else {
                    more -= 1; // 没有拐点，结束
                    prgs(all, more);
                    break;
                }
            }
        }
        return more;
    }

    /// 查漏补缺
    fn check(&mut self, _more: u32) {
        let all = self.w * self.h;
        let mut more = _more;
        // 随机数生成器
        let mut rng = rand::thread_rng();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.map[y as usize][x as usize] > 0 {
                    continue;
                }
                let p = Pos { x, y };
                let ops = self.nearby(&p, false);
                if let Some(dest) = ran_one(&ops, &mut rng) {
                    self.dig_to(&p, dest);
                    more -= 1;
                    prgs(all, more);
                    if more < 1 {
                        break;
                    }
                }
            } // for x
        } // for y
    }

    /// # 连接两点
    /// **dest**中**val**的值是相对于**src**的方向
    fn dig_to(&mut self, src: &Pos, dest: &Mark) {
        let ((sx, sy), (dx, dy)) = (src.tup_usz(), dest.p.tup_usz());
        let cross = U + D + L + R;
        let v = &mut self.map[sy][sx];
        if *v < cross {
            *v += dest.val;
        }
        let v = &mut self.map[dy][dx];
        if *v < cross {
            *v += rev(dest.val);
        }
    }

    /// 获取四周的情况
    fn nearby(&self, p: &Pos, just_unk: bool) -> Vec<Mark> {
        let mut v: Vec<Mark> = Vec::with_capacity(4);
        for d in DIR_LS {
            if let Some(n) = p.peek(&d) {
                if let Some(m) = self.get_mark(&n) {
                    if m > 0 && just_unk {
                        continue;
                    }
                    v.push(Mark { p: n, val: d.val() });
                }
            }
        }
        return v;
    }

    fn get_mark(&self, p: &Pos) -> Option<u8> {
        if p.x >= self.w || p.y >= self.h {
            None
        } else {
            let (x, y) = p.tup_usz();
            Some(self.map[y][x])
        }
    }
}

fn prgs(all: u32, more: u32) {
    let count = all - more;
    print!(
        "\r {}({}%) ",
        count,
        ((count as f32 / all as f32) * 100.0) as u32
    );
}

fn ran_one<'s, T>(src: &'s Vec<T>, ran: &mut ThreadRng) -> Option<&'s T> {
    let len = src.len();
    match len {
        0 => None,
        1 => Some(&src[0]),
        _ => Some(&src[ran.gen_range(0..len)]),
    }
}
