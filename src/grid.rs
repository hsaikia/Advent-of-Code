use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use itertools::iproduct;

pub type CellIndex = (usize, usize);
pub type CellDir = (i32, i32);

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    #[default]
    North,
    East,
    West,
    South,
}

impl CardinalDirection {
    #[must_use]
    pub fn to_dir(&self) -> CellDir {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::West => (0, -1),
            Self::East => (0, 1),
        }
    }

    #[must_use]
    pub fn orthogonal(&self) -> Vec<Self> {
        match self {
            Self::North | Self::South => {
                vec![Self::West, Self::East]
            }
            Self::West | Self::East => {
                vec![Self::North, Self::South]
            }
        }
    }

    #[must_use]
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

/// A Generic Grid of items of type T
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Grid<T: std::fmt::Debug + Clone + Default + PartialEq + Hash> {
    pub values: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: std::fmt::Debug + Clone + Default + PartialEq + Hash> Grid<T> {
    pub fn new(n: usize, m: usize, val: T) -> Self {
        Grid {
            values: vec![vec![val; m]; n],
            rows: n,
            cols: m,
        }
    }

    #[must_use]
    pub fn get_hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    #[must_use]
    pub fn from_str(input: &str, f: fn(char) -> T) -> Self {
        let lines = input
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(str::trim)
            .collect::<Vec<_>>();
        let mut grid = Grid::<T>::new(lines.len(), lines[0].len(), T::default());
        for (i, line) in lines.iter().enumerate() {
            let row = line.chars().map(f).collect::<Vec<_>>();
            grid.set_row(i, row);
        }
        grid
    }

    #[must_use]
    pub fn rotate_clockwise(&self) -> Self {
        let mut ret = Grid::new(self.cols, self.rows, T::default());
        for (i, j) in iproduct!(0..self.rows, 0..self.cols) {
            ret.set(&(j, self.rows - 1 - i), self.get(&(i, j)));
        }
        ret
    }

    // flips the column order
    #[must_use]
    pub fn flip_vertical(&self) -> Self {
        let mut ret = Grid::new(self.rows, self.cols, T::default());
        for (i, j) in iproduct!(0..self.rows, 0..self.cols) {
            ret.set(&(i, self.cols - 1 - j), self.get(&(i, j)));
        }
        ret
    }

    #[must_use]
    pub fn positions(&self, x: &T) -> Vec<CellIndex> {
        let mut ret = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.values[r][c] == *x {
                    ret.push((r, c));
                }
            }
        }
        ret
    }

    pub fn set_all(&mut self, positions: &[CellIndex], val: T) {
        for idx in positions.iter() {
            self.set(idx, val.clone());
        }
    }

    #[must_use]
    pub fn count(&self, x: &T) -> usize {
        self.values
            .iter()
            .map(|row| row.iter().filter(|&cell| cell == x).count())
            .sum::<usize>()
    }

    #[must_use]
    pub fn find_in_row(&self, row: usize, x: &T) -> Vec<CellIndex> {
        self.values[row]
            .iter()
            .enumerate()
            .filter_map(|(col, c)| if c == x { Some((row, col)) } else { None })
            .collect::<Vec<_>>()
    }

    #[must_use]
    pub fn find_in_col(&self, col: usize, x: &T) -> Vec<CellIndex> {
        (0..self.rows)
            .filter_map(|row| {
                if self.values[row][col] == *x {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    #[must_use]
    pub fn common_elements_in_rows(&self, r1: usize, r2: usize) -> usize {
        let mut ans = 0;
        for i in 0..self.cols {
            if self.values[r1][i] == self.values[r2][i] {
                ans += 1;
            }
        }
        ans
    }

    #[must_use]
    pub fn common_elements_in_cols(&self, c1: usize, c2: usize) -> usize {
        let mut ans = 0;
        for i in 0..self.rows {
            if self.values[i][c1] == self.values[i][c2] {
                ans += 1;
            }
        }
        ans
    }

    #[must_use]
    pub fn are_rows_equal(&self, r1: usize, r2: usize) -> bool {
        self.common_elements_in_rows(r1, r2) == self.cols
    }

    #[must_use]
    pub fn are_cols_equal(&self, c1: usize, c2: usize) -> bool {
        self.common_elements_in_cols(c1, c2) == self.rows
    }

    pub fn print(&self) {
        for row in &self.values {
            for cell in row {
                print!("{:?}", &cell);
            }
            println!();
        }

        println!();
    }

    /// Works best only if the cell values fit in a 3-character space
    pub fn debug_print(&self)
    where
        T: std::fmt::Display,
    {
        let mut ret: String = String::new();
        for r in 0..=2 * self.rows + 1 {
            for c in 0..=4 * self.cols {
                if r % 2 == 0 {
                    if c % 4 == 0 {
                        ret += "+";
                    } else {
                        ret += "-";
                    }
                } else if c % 4 == 0 {
                    ret += "|";
                } else if c % 4 == 2 {
                    if r == 2 * self.rows + 1 {
                        ret += &format!("{:^3}", (c - 2) / 4);
                    } else {
                        ret += &format!("{:^3}", self.values[(r - 1) / 2][(c - 2) / 4]);
                    }
                }
            }
            if r % 2 == 0 {
                ret += "\n";
            } else {
                ret += &format!("{}\n", (r - 1) / 2);
            }
        }
        println!("{ret}");
    }

    /// # Panics
    ///
    /// Panics if grid index is out of bounds
    #[must_use]
    pub fn to_flat_idx(&self, idx: &CellIndex) -> usize {
        assert!(
            !(idx.0 >= self.rows || idx.1 >= self.cols),
            "Grid index out of bounds"
        );
        idx.0 * self.cols + idx.1
    }

    #[must_use]
    pub fn from_flat_idx(&self, idx: usize) -> CellIndex {
        (idx / self.cols, idx % self.cols)
    }

    /// # Panics
    ///
    /// Panics if grid index is out of bounds
    #[must_use]
    pub fn get(&self, idx: &CellIndex) -> T {
        assert!(
            !(idx.0 >= self.rows || idx.1 >= self.cols),
            "Grid index out of bounds"
        );
        self.values[idx.0][idx.1].clone()
    }

    pub fn set_row(&mut self, i: usize, row_vals: Vec<T>) {
        self.values[i] = row_vals;
    }

    /// # Panics
    ///
    /// Panics if grid index is out of bounds
    pub fn set(&mut self, idx: &CellIndex, val: T) {
        assert!(
            !(idx.0 >= self.rows || idx.1 >= self.cols),
            "Grid index out of bounds"
        );
        self.values[idx.0][idx.1] = val;
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn cell_in_direction(&self, idx: &CellIndex, dir: &CellDir) -> Option<(usize, usize)> {
        let x = idx.0 as i32;
        let y = idx.1 as i32;
        let dx = dir.0;
        let dy = dir.1;
        if ((dx < 0 && x + dx >= 0) || (dx >= 0 && x + dx < self.rows as i32))
            && ((dy < 0 && y + dy >= 0) || (dy >= 0 && y + dy < self.cols as i32))
        {
            return Some(((x + dx) as usize, (y + dy) as usize));
        }

        None
    }

    #[must_use]
    pub fn adjacent_in_dirs(&self, idx: &CellIndex, dirs: &[(i32, i32)]) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for d in dirs {
            let opt_cell = self.cell_in_direction(idx, d);
            if let Some(cell) = opt_cell {
                ret.push(cell);
            }
        }
        ret
    }

    #[must_use]
    pub fn l1_distance(&self, idx1: &CellIndex, idx2: &CellIndex) -> usize {
        let dx = if idx1.0 > idx2.0 {
            idx1.0 - idx2.0
        } else {
            idx2.0 - idx1.0
        };

        let dy = if idx1.1 > idx2.1 {
            idx1.1 - idx2.1
        } else {
            idx2.1 - idx1.1
        };

        dx + dy
    }

    #[must_use]
    pub fn adjacent_2_row(&self, idx: &CellIndex) -> Vec<(usize, usize)> {
        self.adjacent_in_dirs(idx, &[(0, 1), (0, -1)])
    }

    #[must_use]
    pub fn adjacent_4(&self, idx: &CellIndex) -> Vec<(usize, usize)> {
        self.adjacent_in_dirs(idx, &[(-1, 0), (0, -1), (1, 0), (0, 1)])
    }

    #[must_use]
    pub fn adjacent_8(&self, idx: &CellIndex) -> Vec<(usize, usize)> {
        self.adjacent_in_dirs(
            idx,
            &[
                (-1, 0),
                (0, -1),
                (1, 0),
                (0, 1),
                (-1, 1),
                (1, -1),
                (1, 1),
                (-1, -1),
            ],
        )
    }

    #[must_use]
    pub fn sweep_4(&self, idx: &CellIndex) -> [Vec<(usize, usize)>; 4] {
        const VAL: Vec<(usize, usize)> = vec![];
        let mut ret: [Vec<(usize, usize)>; 4] = [VAL; 4];
        let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (i, d) in dir.iter().enumerate() {
            let mut curr_cell = *idx;
            while let Some(cell) = self.cell_in_direction(&curr_cell, d) {
                ret[i].push(cell);
                curr_cell = cell;
            }
        }
        ret
    }

    /// Fill all cells where value = `replace_id`, neighboring the ones where value = `cluster_id`
    pub fn flood_fill(&mut self, cluster_id: &T, replace_id: &T) {
        let mut q = VecDeque::new();

        for (i, j) in iproduct!(0..self.rows, 0..self.cols) {
            if self.get(&(i, j)) == cluster_id.clone() {
                q.push_back((i, j));
            }
        }

        let mut visited = Grid::<bool>::new(self.rows, self.cols, false);
        while let Some(x) = q.pop_front() {
            if visited.get(&x) {
                continue;
            }

            visited.set(&x, true);

            self.set(&x, cluster_id.clone());

            for n in self.adjacent_4(&x) {
                if self.get(&n) == replace_id.clone() {
                    q.push_back(n);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::<i32>::new(10, 6, 0);
        assert!(grid.rows == 10);
        assert!(grid.cols == 6);

        let c1 = grid.cell_in_direction(&(0, 0), &(-1, 0));
        assert!(c1.is_none());

        let c2 = grid.cell_in_direction(&(0, 0), &(1, 0));
        assert!(c2.is_some());
        assert!(c2.unwrap() == (1, 0));

        let nxy = grid.adjacent_4(&(0, 0));
        assert!(nxy == vec![(1, 0), (0, 1)]);

        let nxy = grid.adjacent_4(&(2, 4));
        assert!(nxy == vec![(1, 4), (2, 3), (3, 4), (2, 5)]);

        let sxy = grid.sweep_4(&(2, 4));
        assert!(sxy[0] == vec![(1, 4), (0, 4)]);
        assert!(sxy[1] == vec![(2, 3), (2, 2), (2, 1), (2, 0)]);
        assert!(sxy[2] == vec![(3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4)]);
        assert!(sxy[3] == vec![(2, 5)]);

        let nxy = grid.adjacent_8(&(0, 0));
        assert!(nxy == vec![(1, 0), (0, 1), (1, 1)]);
    }
}
