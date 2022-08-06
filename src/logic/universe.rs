use rand::Rng;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<CellState>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn next_step(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (CellState::Alive, x) if x < 2 => CellState::Dead,
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    (CellState::Alive, x) if x > 3 => CellState::Dead,
                    (CellState::Dead, 3) => CellState::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|_| {
                let rand_num = rand::thread_rng().gen_range(0, 4);
                if rand_num == 0 {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn get_grid(&self) -> Vec<Vec<CellState>> {
        let mut rows: Vec<Vec<CellState>> = Vec::new();
        let width = usize::try_from(self.width).unwrap();
        let height = usize::try_from(self.height).unwrap();
        for y in 0..height {
            let mut row: Vec<CellState> = Vec::new();
            for x in 0..width {
                let cell = self.cells[y * width + x];
                row.push(cell);
            }
            rows.push(row);
        }
        return rows;
    }
}
