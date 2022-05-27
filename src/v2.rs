
/// directory codes
pub const U: u8 = 1;
pub const D: u8 = 2;
pub const L: u8 = 4;
pub const R: u8 = 8;

/// 反向
pub fn rev(d: u8) -> u8 {
    match d {
        U | L => d * 2,
        D | R => d / 2,
        _ => d,
    }
}

/// position
pub struct Pos(pub u32, pub u32);

/// directory
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/// link(cross) case
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
        Pos(x, y)
    }

    pub fn cp(p: &Pos) -> Pos {
        Pos(p.0, p.1)
    }

    /// 按方向单步移动坐标。
    /// 返回成功与否
    /// 不成功不移动
    pub fn mv(&mut self, d: &Dir) -> bool {
        let mut x = self.0;
        let mut y = self.1;
        let mut r = true;
        match d {
            Dir::Up => {
                if y < 1 {
                    r = false;
                } else {
                    y -= 1;
                }
            }
            Dir::Down => {
                if y > i32::MAX as u32 {
                    r = false;
                } else {
                    y += 1;
                }
            }
            Dir::Left => {
                if x < 1 {
                    r = false;
                } else {
                    x -= 1;
                }
            }
            Dir::Right => {
                if x > i32::MAX as u32 {
                    r = false;
                } else {
                    x += 1;
                }
            }
        }
        if r {
            self.0 = x;
            self.1 = y;
        }
        return r;
    }

    /// 尝试根据方向获取新坐标
    pub fn peek(&self, d: &Dir) -> Option<Pos> {
        let mut tmp = Pos::cp(self);
        let f = tmp.mv(d);
        return if f { Some(tmp) } else { None };
    }

    /// 跳转坐标
    pub fn goto(&mut self, p: &Pos) {
        self.0 = p.0;
        self.1 = p.1;
    }
}

impl Dir {
    /// by code
    pub fn by(v: u8) -> Option<Dir> {
        match v {
            U => Some(Dir::Up),
            D => Some(Dir::Down),
            L => Some(Dir::Left),
            R => Some(Dir::Right),
            _ => None,
        }
    }

    /// get code
    pub fn val(&self) -> u8 {
        match self {
            Dir::Up => U,
            Dir::Down => D,
            Dir::Left => R,
            Dir::Right => L,
        }
    }

    /// get reverse dir
    pub fn rev(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}
