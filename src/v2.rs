/// 方向真值
type DV = u8;

// --- 常量
// 方向真值
const U: DV = 1 << 0;
const D: DV = 1 << 1;
const L: DV = 1 << 2;
const R: DV = 1 << 3;

/// 无符号坐标
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

/// 方向枚举
#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

// --- impl

impl Dir {
    /// 方向枚举真值
    pub fn val(&self) -> DV {
        use Dir::*;
        match self {
            Up => U,
            Down => D,
            Left => L,
            Right => R,
        }
    }

    /// 取反方向
    pub fn rev(&self) -> Dir {
        use Dir::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl std::ops::BitAnd<u8> for Dir {
    type Output = bool;
    /// 使用枚举比较方向值
    fn bitand(self, d: DV) -> Self::Output {
        (self.val() & d) > 0
    }
}

impl Pos {
    /// 按方向单步移动坐标。
    /// - 返回成功与否
    /// - 不成功不移动
    pub fn mv(&mut self, d: Dir) -> bool {
        use Dir::*;

        let max = i32::MAX as u32;
        if self.x > max {
            self.x = max;
            return false;
        }
        if self.y > max {
            self.y = max;
            return false;
        }
        let (mut x, mut y) = (self.x as i32, self.y as i32);
        match d {
            Up => y -= 1,
            Down => y += 1,
            Left => x -= 1,
            Right => x += 1,
        }
        // return
        if x < 0 || y < 0 {
            return false;
        }
        self.x = x as u32;
        self.y = y as u32;
        true
    }

    /// 尝试根据方向获取新坐标
    pub fn peek(&self, d: Dir) -> Option<Pos> {
        let mut tmp = self.clone();
        // return
        if tmp.mv(d) {
            Some(tmp)
        } else {
            None
        }
    }

    /// 跳转坐标
    pub fn goto(&mut self, p: &Pos) {
        self.x = p.x;
        self.y = p.y;
    }
}
