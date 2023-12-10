use std::collections::VecDeque;

// A Generic Grid of items of type T

pub type CellIndex = (usize, usize);
pub type CellDir = (i32, i32);

#[derive(Debug, Clone)]
pub struct Grid<T: std::fmt::Debug + Clone + Default + PartialEq> {
    pub values: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: std::fmt::Debug + Clone + Default + PartialEq> Grid<T> {
    pub fn new(n: usize, m: usize, val: T) -> Self {
        Grid {
            values: vec![vec![val; m]; n],
            rows: n,
            cols: m,
        }
    }

    pub fn from_str(input: &str, f: fn(char) -> T) -> Self {
        let lines = input
            .split('\n')
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();
        let mut grid = Grid::<T>::new(lines.len(), lines[0].len(), T::default());
        for (i, line) in lines.iter().enumerate() {
            let row = line.chars().map(f).collect::<Vec<_>>();
            grid.set_row(i, row);
        }
        grid
    }

    pub fn count(&self, x: &T) -> usize {
        self.values
            .iter()
            .map(|row| row.iter().filter(|&cell| cell == x).count())
            .sum::<usize>()
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

    pub fn to_flat_idx(&self, i: usize, j: usize) -> usize {
        i * self.cols + j
    }

    pub fn from_flat_idx(&self, idx: usize) -> (usize, usize) {
        (idx / self.cols, idx % self.cols)
    }

    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        if i < self.rows && j < self.cols {
            return Some(self.values[i][j].clone());
        }
        None
    }

    pub fn set_row(&mut self, i: usize, row_vals: Vec<T>) {
        self.values[i] = row_vals;
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) {
        self.values[i][j] = val;
    }

    pub fn cell_in_direction(
        &self,
        i: usize,
        j: usize,
        di: i32,
        dj: i32,
    ) -> Option<(usize, usize)> {
        let mut x = i as i32;
        let mut y = j as i32;
        let mut found = false;
        if ((di < 0 && x + di >= 0) || (di >= 0 && x + di < self.rows as i32))
            && ((dj < 0 && y + dj >= 0) || (dj >= 0 && y + dj < self.cols as i32))
        {
            x += di;
            y += dj;
            found = true;
        }

        if found {
            return Some((x as usize, y as usize));
        }

        None
    }

    pub fn adjacent_in_dir(
        &self,
        i: usize,
        j: usize,
        dirs: &Vec<(i32, i32)>,
    ) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for d in dirs {
            let opt_cell = self.cell_in_direction(i, j, d.0, d.1);
            if let Some(cell) = opt_cell {
                ret.push(cell);
            }
        }
        ret
    }

    pub fn adjacent_2_row(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.adjacent_in_dir(i, j, &vec![(0, 1), (0, -1)])
    }

    pub fn adjacent_4(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.adjacent_in_dir(i, j, &vec![(-1, 0), (0, -1), (1, 0), (0, 1)])
    }

    pub fn adjacent_8(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        self.adjacent_in_dir(
            i,
            j,
            &vec![
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

    pub fn sweep_4(&self, i: usize, j: usize) -> [Vec<(usize, usize)>; 4] {
        const VAL: Vec<(usize, usize)> = vec![];
        let mut ret: [Vec<(usize, usize)>; 4] = [VAL; 4];
        let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (idx, d) in dir.iter().enumerate() {
            let mut x = i;
            let mut y = j;
            loop {
                let cell = self.cell_in_direction(x, y, d.0, d.1);
                if cell.is_none() {
                    break;
                }
                let cell = cell.unwrap();
                x = cell.0;
                y = cell.1;
                ret[idx].push(cell);
            }
        }
        ret
    }

    // Fill all cells where value = replace_id 4_neighboring the ones where value = cluster_id
    pub fn flood_fill(&mut self, cluster_id: T, replace_id: T) {
        let mut q = VecDeque::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j).unwrap() == cluster_id.clone() {
                    q.push_back((i, j));
                }
            }
        }

        let mut visited = Grid::<bool>::new(self.rows, self.cols, false);
        while !q.is_empty() {
            let x = q.pop_front().unwrap();

            if visited.get(x.0, x.1).unwrap() {
                continue;
            }

            visited.set(x.0, x.1, true);

            self.set(x.0, x.1, cluster_id.clone());

            for n in self.adjacent_4(x.0, x.1) {
                if self.get(n.0, n.1).unwrap() == replace_id.clone() {
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

        let c1 = grid.cell_in_direction(0, 0, -1, 0);
        assert!(c1.is_none());

        let c2 = grid.cell_in_direction(0, 0, 1, 0);
        assert!(c2.is_some());
        assert!(c2.unwrap() == (1, 0));

        let nxy = grid.adjacent_4(0, 0);
        assert!(nxy == vec![(1, 0), (0, 1)]);

        let nxy = grid.adjacent_4(2, 4);
        assert!(nxy == vec![(1, 4), (2, 3), (3, 4), (2, 5)]);

        let sxy = grid.sweep_4(2, 4);
        assert!(sxy[0] == vec![(1, 4), (0, 4)]);
        assert!(sxy[1] == vec![(2, 3), (2, 2), (2, 1), (2, 0)]);
        assert!(sxy[2] == vec![(3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4)]);
        assert!(sxy[3] == vec![(2, 5)]);

        let nxy = grid.adjacent_8(0, 0);
        assert!(nxy == vec![(1, 0), (0, 1), (1, 1)]);
    }
}
