// 方向真值
pub const U: u8 = 1;
pub const D: u8 = 2;
pub const L: u8 = 4;
pub const R: u8 = 8;

/// 反向真值
pub fn rev(d: u8) -> u8 {
    match d {
        U | L => d * 2,
        D | R => d / 2,
        _ => d,
    }
}

/// 无符号坐标
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

/// 方向枚举
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

// --- impl

impl Pos {
    pub fn tup_usz(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    /// 复制坐标
    pub fn cp(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y,
        }
    }

    /// 按方向单步移动坐标。
    /// 返回成功与否
    /// 不成功不移动
    pub fn mv(&mut self, d: &Dir) -> bool {
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
            Dir::Up => y -= 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
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
    pub fn peek(&self, d: &Dir) -> Option<Pos> {
        let mut tmp = self.cp();
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

impl Dir {
    pub fn val(&self) -> u8 {
        match self {
            Dir::Up => U,
            Dir::Down => D,
            Dir::Left => L,
            Dir::Right => R,
        }
    }
}
