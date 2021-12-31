use std::num::ParseIntError;

use std::str::FromStr;

// pls dont change these
// number of cols
const ROW_SIZE: usize = 5;
// number of rows
const NUM_ROWS: usize = 5;

#[derive(Clone)]
pub struct BitMatrix(u32);

impl BitMatrix {
    pub fn set(&mut self, row: usize, col: usize) {
        self.0 |= 1 << row * ROW_SIZE + col
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.0 & (1 << row * ROW_SIZE + col) > 0
    }

    // fn is_row_set(&self, row: usize) -> bool {
    //     let mask = 0x1F << (ROW_SIZE * (NUM_ROWS - row));

    //     self.0 & mask == mask
    // }

    // fn is_col_set(&self, col: usize) -> bool {
    //     let mut mask = 1 << col;
    //     for _ in 0..NUM_ROWS {
    //         mask &= mask << ROW_SIZE;
    //     }
    //     self.0 * mask == mask
    // }

    pub fn any_row_set(&self) -> bool {
        let mut mask = 0x1F;

        for _ in 0..NUM_ROWS {
            if self.0 & mask == mask {
                return true;
            }
            mask <<= ROW_SIZE;
        }

        false
    }

    pub fn any_col_set(&self) -> bool {
        let mut mask = 0x108421;

        for _ in 0..ROW_SIZE {
            if self.0 & mask == mask {
                return true;
            }
            mask = (mask << 1) & 0x3FFFFFFF;
        }

        false
    }
}

// to iterator through all set indices
pub struct BitMatrixUnsetIterator {
    matrix: BitMatrix,
    row: usize,
    col: usize,
}

impl IntoIterator for BitMatrix {
    type Item = (usize, usize);
    type IntoIter = BitMatrixUnsetIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitMatrixUnsetIterator {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}

impl BitMatrixUnsetIterator {
    fn is_valid(&mut self) -> bool {
        self.row < NUM_ROWS && self.col < ROW_SIZE
    }

    fn advance(&mut self) {
        self.col += 1;

        if self.col >= ROW_SIZE {
            self.col = 0;
            self.row += 1;
        }
    }
}

impl Iterator for BitMatrixUnsetIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

        while self.is_valid() && self.matrix.get(self.row, self.col) {
            self.advance();
        }

        let res = (self.row, self.col);
        let is_valid = self.is_valid();

        self.advance();

        if is_valid {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Board {
    rows: [[u16; 5]; 5],
    pub score: BitMatrix,
}

impl Board {
    pub fn mark(&mut self, num: u16) {
        let mut j = 0;
        let i = self.rows.iter().position(|row| {
            if let Some(pos) = row.iter().position(|&val| val == num) {
                j = pos;
                true
            } else {
                false
            }
        });
        if let Some(i) = i {
            self.score.set(i, j);
        }
    }

    pub fn sum_unmarked(&self) -> usize {
        self.score
            .clone()
            .into_iter()
            .map(|(i, j)| self.rows[i][j] as usize)
            .sum()
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: [[u16; 5]; 5] = [[0; 5]; 5];
        s.lines().enumerate().for_each(|(i, line)| {
            line.split_ascii_whitespace()
                .enumerate()
                .for_each(|(j, val)| {
                    rows[i][j] = val.parse().unwrap();
                });
        });
        Ok(Board {
            rows,
            score: BitMatrix(0u32),
        })
    }
}

