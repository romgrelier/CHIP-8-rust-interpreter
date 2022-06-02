use std::ops::Index;
use std::ops::IndexMut;
use std::fmt;

pub struct Screen {
    matrix: [bool; 64 * 32],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            matrix: [false; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        self.matrix = [false; 64 * 32];
    }
}

impl Index<(usize, usize)> for Screen {
    type Output = bool;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.matrix[idx.0 * 64 + idx.1]
    }
}

impl IndexMut<(usize, usize)> for Screen {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[idx.0 * 64 + idx.1]
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..32 {
            for x in 0..64 {
                if self[(y, x)] {
                    write!(f, "X")?
                } else {
                    write!(f, " ")?
                }
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}
