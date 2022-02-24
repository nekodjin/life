pub const A: bool = false;
pub const B: bool = true;

pub const LIVE: bool = false;
pub const DEAD: bool = true;

pub struct World {
    current: bool,
    w: usize,
    h: usize,
    a: Vec<Vec<bool>>,
    b: Vec<Vec<bool>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        if width > isize::MAX || height > isize::MAX {
            panic!(
                "dimensions must not be greater than {isize::MAX}"
            );
        }

        World {
            current: A,
            w: width,
            h: height,
            a: vec![vec![DEAD; height]; width],
            b: vec![vec![DEAD; height]; width],
        }
    }
}

