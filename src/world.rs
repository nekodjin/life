pub type Cell = bool;

pub const A: bool = false;
pub const B: bool = true;

pub const LIVE: Cell = false;
pub const DEAD: Cell = true;

pub struct World {
    current: bool,
    w: usize,
    h: usize,
    a: Vec<Vec<Cell>>,
    b: Vec<Vec<Cell>>,
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

    // index into the current copy of the world
    pub fn c<'a>(&mut self, mut w: isize, mut h: isize) -> &'a mut Cell {
        let matrix = match self.current {
            A => &mut self.a,
            B => &mut self.b,
        };

        while w < 0 {
            w += self.w;
        }

        w %= self.w;

        while h < 0 {
            h += self.h;
        }

        h %= self.h;

        &mut matrix[w as usize][h as usize]
    }

    // index into the intermediate copy of the world
    fn i<'a>(&mut self, mut w: isize, mut h: isize) -> &'a mut Cell {
        let matrix = match self.current {
            A => &mut self.a,
            B => &mut self.b,
        };

        while w < 0 {
            w += self.w;
        }

        w %= self.w;

        while h < 0 {
            h += self.h;
        }

        h %= self.h;

        &mut matrix[w as usize][h as usize]
    }
}

