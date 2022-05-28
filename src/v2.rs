
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

/// 连接方向枚举
pub enum Crs {
    // U-1     D-2     UD-3
    Ver,
    // L-4     UL-5    DL-6
    UL,
    DL,
    // UDL-7   R-8     UR-9
    UDL,
    UR,
    // DR-10   UDR-11  LR-12
    DR,
    UDR,
    Hor,
    // ULR-13  DLR-14  UDLR-15
    ULR,
    DLR,
    X,
}

// --- impl

impl Pos {
    pub fn at(x: u32, y: u32) -> Pos {
        Pos { x, y }
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
        let mut x = self.x as i32;
        let mut y = self.y as i32;
        match d {
            Dir::Up => y -= 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
        }
        // return
        if x < 0 || y < 0 {
            false
        } else {
            self.x = x as u32;
            self.y = y as u32;
            true
        }
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
    pub fn by(v: u8) -> Option<Dir> {
        match v {
            U => Some(Dir::Up),
            D => Some(Dir::Down),
            L => Some(Dir::Left),
            R => Some(Dir::Right),
            _ => None,
        }
    }

    pub fn val(&self) -> u8 {
        match self {
            Dir::Up => U,
            Dir::Down => D,
            Dir::Left => R,
            Dir::Right => L,
        }
    }

    /// 取反方向枚举
    pub fn rev(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}
