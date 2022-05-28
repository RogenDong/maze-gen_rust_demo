// --- use
use crate::v2::{rev, Dir, Pos, D, L, R, U};
use rand::{self, Rng};

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
    map: Vec<Vec<u8>>,
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
            map: vec![vec![0u8; h as usize]; w as usize],
        };
        // 开通
        let succ = maze.dig();

        // return
        if succ {
            Ok(maze)
        } else {
            Err("生成失败，请检查".to_string())
        }
    }

    /// 挖掘通道
    fn dig(&mut self) -> bool {
        let mut pre_d = 0u8;
        // 剩余数量
        let mut more = self.w * self.h;
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

        let all = self.w * self.h;
        while more > 0 {
            let count = all - more;
            print!(
                "\r {}({}%) ",
                count,
                ((count as f32 / all as f32) * 100.0) as u32
            );
            // 四周情况
            let opt_d: Vec<Mark> = self.nearby(&p, true);
            let opt_len = opt_d.len();

            // 如果没有可用方向，回溯至上一个拐点
            if opt_len < 1 {
                if let Some(next) = corners.pop() {
                    // println!("\rback");
                    p = next;
                    continue;
                }
                // 没有拐点，结束
                println!("... 100%");
                more -= 1;
                break;
            }

            // 取方向 + 坐标
            let dest: &Mark = &opt_d[if opt_len < 2 {
                0
            } else {
                rng.gen_range(0..opt_len)
            }];

            // dest.val 表示其相对于 p 的方向
            self.dig_to(&p, dest);

            // 保存拐点
            let next_d = dest.val;
            if pre_d > 0 && next_d != pre_d {
                corners.push(Pos::cp(&p));
            }

            // 准备下一轮
            p.goto(&dest.p);
            pre_d = next_d;
            more -= 1;
        }
        println!("end dig");
        return more < 1;
    }

    /// # 连接两点
    /// **dest**中**val**的值是相对于**src**的方向
    fn dig_to(&mut self, src: &Pos, dest: &Mark) {
        let (sx, sy, dx, dy) = (
            src.x as usize,
            src.y as usize,
            dest.p.x as usize,
            dest.p.y as usize,
        );
        // update src
        conn(&mut self.map[sx][sy], dest.val);
        // udpate dest
        conn(&mut self.map[dx][dy], rev(dest.val));
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
            Some(self.map[p.x as usize][p.y as usize])
        }
    }
}

/// # 尝试连接
/// 若**src**已经4向通，则不更新
fn conn(src: &mut u8, dest: u8) {
    if *src < U + D + L + R {
        *src += dest;
    }
}
