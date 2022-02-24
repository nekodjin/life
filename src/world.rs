pub type Cell = bool;

pub const LIVE: Cell = false;
pub const DEAD: Cell = true;

const A: bool = false;
const B: bool = true;

const IMAX: usize = isize::MAX as usize;

pub struct World {
    current: bool,
    w: usize,
    h: usize,
    a: Vec<Vec<Cell>>,
    b: Vec<Vec<Cell>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        if width > IMAX || height > IMAX {
            panic!(
                "dimensions must not be greater than {IMAX}"
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
    pub fn c<'a>(&'a mut self, mut w: isize, mut h: isize) -> &'a mut Cell {
        let matrix = match self.current {
            A => &mut self.a,
            B => &mut self.b,
        };

        while w < 0 {
            w += self.w as isize;
        }

        w %= self.w as isize;

        while h < 0 {
            h += self.h as isize;
        }

        h %= self.h as isize;

        &mut matrix[w as usize][h as usize]
    }

    // index into the intermediate copy of the world
    fn i<'a>(&'a mut self, mut w: isize, mut h: isize) -> &'a mut Cell {
        let matrix = match self.current {
            A => &mut self.b,
            B => &mut self.a,
        };

        while w < 0 {
            w += self.w as isize;
        }

        w %= self.w as isize;

        while h < 0 {
            h += self.h as isize;
        }

        h %= self.h as isize;

        &mut matrix[w as usize][h as usize]
    }
}

