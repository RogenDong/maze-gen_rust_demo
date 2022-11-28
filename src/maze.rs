// --- use
use crate::v2::{Dir, Pos};
use rand::{self, prelude::ThreadRng, Rng};
use std::ops::{Index, IndexMut};

// --- 类型别称
/// 向标（Direction Mark）；连接值
/// - 利用二进制位（bit-flag）标记多个方向
type DM = u8;

// --- 常量
// 限制
const W_MIN: u32 = 3;
const H_MIN: u32 = 3;
const W_MAX: u32 = i32::MAX as u32;
const H_MAX: u32 = i32::MAX as u32;

/// 方向枚举支持迭代
const DIR_LS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

// --- 结构
/// 点位标记
#[derive(Clone, Copy)]
struct Mark {
    p: Pos,
    d: Dir,
}

/// 迷宫
pub struct Maze {
    w: u32,
    h: u32,
    /// 点位连接值 `map[y][x]`
    pub map: Vec<Vec<DM>>,
}

// --- func

/// 进度
/// ### 参数
/// - `all` 全部
/// - `more` 剩余
fn prgs(all: u32, more: u32) {
    let count = all - more;
    print!(
        "\r {}({}%) ",
        count,
        ((count as f32 / all as f32) * 100.0) as u32
    );
}

/// 随机获取集合中一个元素
/// ### 约束
/// - `s` 集合的生命周期
/// - `T` 集合内元素的类型
/// ### 参数
/// - `src` 源集合
/// - `ran` 随机数生成器
/// ### 返回
/// 是否得到有效结果
fn ran_one<'s, T>(src: &'s [T], ran: &mut ThreadRng) -> Option<&'s T> {
    let len = src.len();
    match len {
        0 => None,
        1 => Some(&src[0]),
        _ => Some(&src[ran.gen_range(0..len)]),
    }
}

// --- impl

impl std::ops::BitOrAssign<Dir> for DM {
    /// 方便枚举参与位运算
    fn bitor_assign(&mut self, d: Dir) {
        *self |= d.val()
    }
}

impl Index<(u32, u32)> for Maze {
    type Output = DM;
    /// 通过 (x, y) 元组获取点的连接方向（不可变借用）
    fn index(&self, p: (u32, u32)) -> &Self::Output {
        let (x, y) = p;
        &self.map[y as usize][x as usize]
    }
}

impl Index<&Pos> for Maze {
    type Output = DM;
    /// 通过[坐标 Pos](Pos)获取点的连接方向（不可变借用）
    fn index(&self, p: &Pos) -> &Self::Output {
        &self.map[p.y as usize][p.x as usize]
    }
}

impl IndexMut<&Pos> for Maze {
    /// 通过[坐标 Pos](Pos)获取点的连接方向（可变借用）
    fn index_mut(&mut self, p: &Pos) -> &mut Self::Output {
        &mut self.map[p.y as usize][p.x as usize]
    }
}

impl Maze {
    /// 执行迷宫生成
    /// ### 参数
    /// - `w` 宽
    /// - `h` 高
    pub fn gen(w: u32, h: u32) -> Result<Maze, String> {
        if w < W_MIN || w > W_MAX || h < H_MIN || h > H_MAX {
            return Err(format!(
                "宽高区间：{}<=宽<={}, {}<=高<={}",
                W_MAX, W_MIN, H_MAX, H_MIN
            ));
        }
        println!("宽高: ({} * {})", w, h);
        // 初始化
        let mut maze = Maze {
            w,
            h,
            map: vec![vec![0u8; w as usize]; h as usize],
        };
        // 开通
        maze.dig();
        println!("完成迷宫生成");
        // return
        Ok(maze)
    }

    /// 挖掘通道
    fn dig(&mut self) {
        // 总格子数
        let all = self.w * self.h;
        // 剩余数量
        let mut more = all;
        // 随机数生成器
        let mut rng = rand::thread_rng();
        // 拐点
        let mut corners: Vec<Pos> = Vec::with_capacity(more as usize);
        // 上一个方向（方向值）
        let mut pre_dv = 0u8;
        // 坐标 —— 首个随机
        let mut p = Pos {
            x: rng.gen_range(0..self.w),
            y: rng.gen_range(0..self.h),
        };
        println!("start at ({},{})", p.x, p.y);
        // TODO 从起点周围4方向并行挖掘
        loop {
            prgs(all, more);
            if more < 1 {
                break;
            }
            // 步进向四周随机一个方向
            let ops = self.nearby(&p, true);
            if let Some(dest) = ran_one(&ops, &mut rng) {
                self.conn(&p, dest);
                let next_dv = dest.d.val();
                // 非起始 && 不同向 && 四周仍有空地
                if pre_dv > 0 && pre_dv != next_dv && ops.len() > 1 {
                    // 保存拐点
                    corners.push(p);
                }
                p.goto(&dest.p);
                pre_dv = next_dv;
                more -= 1;
                continue;
            }
            // 如果没有可用方向，回溯至上一个拐点
            if let Some(next) = corners.pop() {
                p = next;
                continue;
            }
            // 没有拐点，结束
            more -= 1;
            prgs(all, more);
            break;
        }
        println!()
    }

    /// 连接两点
    /// ### 参数
    /// - `src` 出发点
    /// - `dest`: [Mark]
    ///   - `Mark::p` 目标点
    ///   - `Mark::d` 目标点相对于出发点的方向
    fn conn(&mut self, src: &Pos, dest: &Mark) {
        let dm = &mut self[src];
        *dm |= dest.d;
        let dm = &mut self[&dest.p];
        *dm |= dest.d.rev();
    }

    /// 获取四周的情况
    /// ### 参数
    /// - `p` 中心点
    /// - `just_unk` 是否只探索未知区域
    /// ### 返回
    /// 四周有效点位集合
    /// - `Mark::p` 目标点
    /// - `Mark::d` 目标点相对于中心点的方向
    fn nearby(&self, p: &Pos, just_unk: bool) -> Vec<Mark> {
        let mut result = Vec::with_capacity(4);
        for d in DIR_LS {
            if let Some(near) = p.peek(d) {
                if near.x < self.w && near.y < self.h {
                    // 获取点位向标（已打通的方向）
                    let dm = self[&near];
                    // 跳过：点位已通 &&（只取未通 || 与当前点连通）
                    if dm > 0 && (just_unk || (d & dm)) {
                        continue;
                    }
                    result.push(Mark { p: near, d });
                }
            }
        }
        // return
        result
    }
}
