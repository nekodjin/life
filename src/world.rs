use std::fmt;
use std::ops;

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

    fn inter(&self, mut x: isize, mut y: isize) -> Cell {
        let inter = match self.current {
            A => &self.b,
            B => &self.a,
        };

        let w = self.w as isize;
        let h = self.h as isize;

        while x < 0 {
            x += w;
        }

        x %= w;

        while y < 0 {
            y += h;
        }

        y %= h;

        let x = x as usize;
        let y = y as usize;

        inter[x][y]
    }

    fn inter_mut(&mut self, mut x: isize, mut y: isize) -> &mut Cell {
        let inter = match self.current {
            A => &mut self.b,
            B => &mut self.a,
        };

        let w = self.w as isize;
        let h = self.h as isize;

        while x < 0 {
            x += w;
        }

        x %= w;

        while y < 0 {
            y += h;
        }

        y %= h;

        let x = x as usize;
        let y = y as usize;

        &mut inter[x][y]
    }
}

impl ops::Index<(isize, isize)> for World {
    type Output = Cell;

    fn index(&self, idx: (isize, isize)) -> &Cell {
        let matrix = match self.current {
            A => &self.a,
            B => &self.b,
        };

        let w = self.w as isize;
        let h = self.h as isize;

        let mut x = idx.0;
        let mut y = idx.1;

        while x < 0 {
            x += w;
        }

        x %= w;

        while y < 0 {
            y += h;
        }

        y %= h;

        let x = x as usize;
        let y = y as usize;

        &matrix[x][y]
    }
}

impl ops::IndexMut<(isize, isize)> for World {
    fn index_mut(&mut self, idx: (isize, isize)) -> &mut Cell {
        let matrix = match self.current {
            A => &mut self.a,
            B => &mut self.b,
        };

        let w = self.w as isize;
        let h = self.h as isize;

        let mut x = idx.0;
        let mut y = idx.1;

        while x < 0 {
            x += w;
        }

        x %= w;

        while y < 0 {
            y += h;
        }

        y %= h;

        let x = x as usize;
        let y = y as usize;

        &mut matrix[x][y]
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inter = match self.current {
            A => &self.b,
            B => &self.a,
        };

        writeln!(f, "Current: {}x{}", self.w, self.h)?;

        for y in 0..self.h as isize {
            for x in 0..self.w as isize {
                write!(f, "{}",
                    match self[(x, y)] {
                        DEAD => " ",
                        LIVE => "█",
                    },
                )?;
            }

            writeln!(f, "")?;
        }

        writeln!(f, "Intermediate: {}x{}", self.w, self.h)?;

        for y in 0..self.h as isize {
            for x in 0..self.w as isize {
                write!(f, "{}",
                    match self.inter(x, y) {
                        DEAD => " ",
                        LIVE => "█",
                    },
                )?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

