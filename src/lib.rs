mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
#[repr(u8)] // #[repr(u8)], so that each cell is represented as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// https://rustwasm.github.io/docs/book/game-of-life/implementing.html
impl Cell {}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    /// To access the cell at a given row and column, we translate the row and column into an index into the cells vector, as described earlier:
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// In order to calculate the next state of a cell, we need to get a count of how many of its neighbors are alive. Let's write a live_neighbor_count method to do just that!
    /// The live_neighbor_count method uses deltas and modulo to avoid special casing the edges of the universe with ifs.
    ///
    /// # Explanation
    ///
    /// ## % operator
    /// - When applying a delta of -1, we add self.height - 1
    ///     and let the modulo do its thing,
    ///     rather than attempting to subtract 1.
    /// - row and column can be 0, and if we attempted to subtract 1 from them,
    ///     there would be an unsigned integer underflow.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count: u8 = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                // Create an infinite cylindrical overlapped universe
                // Bypasses the need for infinite storage
                // % helps to return a zero value at the edge of the next cell
                let neighbour_row: u32 = (row + delta_row) % self.height;
                let neighbour_column: u32 = (column + delta_row) % self.width;

                let idx: usize = self.get_index(neighbour_row, neighbour_column);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    /// Public methods, exported to JavaScript.
    // compute the next generation from the current on
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, column);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
    }
}

/*
  Rule 1: Any live cell with fewer than two live neighbours
  // dies, as if caused by underpopulation.
  (Cell::Alive, x) if x < 2 => Cell::Dead,

  // Rule 2: Any live cell with two or three live neighbours
  // lives on to the next generation.
  (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
  
  // Rule 3: Any live cell with more than three live
  // neighbours dies, as if by overpopulation.
  (Cell::Alive, x) if x > 3 => Cell::Dead,

  // Rule 4: Any dead cell with exactly three live neighbours
  // becomes a live cell, as if by reproduction.
  (Cell::Dead, 3) => Cell::Alive,

  // All other cells remain in the same state.
  (otherwise, _) => otherwise,
*/
