use std::fmt;
use std::ops;

const IMAX: usize = isize::MAX as usize;

pub struct World {
    state: WorldState,
    w: usize,
    h: usize,
    a: Vec<Vec<Cell>>,
    b: Vec<Vec<Cell>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Live,
    Dead,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WorldState {
    ACurrent,
    BCurrent,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        if width > IMAX || height > IMAX {
            panic!(
                "dimensions must not be greater than {IMAX}"
            );
        }

        World {
            state: WorldState::ACurrent,
            w: width,
            h: height,
            a: vec![vec![Cell::Dead; height]; width],
            b: vec![vec![Cell::Dead; height]; width],
        }
    }

    pub fn cycle(&mut self) {
        let w = self.w as isize;
        let h = self.h as isize;

        for x in 0..self.w {
            for y in 0..self.h {
                let mut live_cells = 0;

                if self[(w-1, h-1)] == Cell::Live { live_cells += 1; }
                if self[(w+0, h-1)] == Cell::Live { live_cells += 1; }
                if self[(w+1, h-1)] == Cell::Live { live_cells += 1; }
                if self[(w+1, h+0)] == Cell::Live { live_cells += 1; }
                if self[(w+1, h+1)] == Cell::Live { live_cells += 1; }
                if self[(w+0, h+1)] == Cell::Live { live_cells += 1; }
                if self[(w-1, h+1)] == Cell::Live { live_cells += 1; }
                if self[(w-1, h+0)] == Cell::Live { live_cells += 1; }

                if live_cells == 3 {
                    *self.inter(w, h) = Cell::Live;
                    continue;
                }

                if live_cells == 2 {
                    *self.inter(w, h) = self[(w, h)];
                    continue;
                }

                *self.inter(w, h) = Cell::Dead;
            }
        }

        self.state = match self.state {
            WorldState::ACurrent => WorldState::BCurrent,
            WorldState::BCurrent => WorldState::ACurrent,
        };
    }

    fn inter(&mut self, mut x: isize, mut y: isize) -> &mut Cell {
        let inter = match self.state {
            WorldState::ACurrent => &mut self.b,
            WorldState::BCurrent => &mut self.a,
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
        let matrix = match self.state {
            WorldState::ACurrent => &self.a,
            WorldState::BCurrent => &self.b,
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
        let matrix = match self.state {
            WorldState::ACurrent => &mut self.a,
            WorldState::BCurrent => &mut self.b,
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
        let inter = match self.state {
            WorldState::ACurrent => &self.b,
            WorldState::BCurrent => &self.a,
        };

        writeln!(f, "Current: {}x{}", self.w, self.h)?;

        for y in 0..self.h as isize {
            for x in 0..self.w as isize {
                write!(f, "{}",
                    match self[(x, y)] {
                        Cell::Dead => " ",
                        Cell::Live => "█",
                    },
                )?;
            }

            writeln!(f, "")?;
        }

        writeln!(f, "Intermediate: {}x{}", self.w, self.h)?;

        for y in 0..self.h {
            for x in 0..self.w {
                write!(f, "{}",
                    match inter[x][y] {
                        Cell::Dead => " ",
                        Cell::Live => "█",
                    },
                )?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

