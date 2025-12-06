use getrandom::getrandom;
use rand::random;
use wasm_bindgen::prelude::*; // 引入 wasm-bindgen

#[derive(Clone, Copy)]
struct Cell {
    id: i32,
    state: bool,          // True for alive, false for dead
    position: (i32, i32), // (x, y) coordinates
}

impl Cell {
    fn new(id: i32, state: bool, position: (i32, i32)) -> Cell {
        Cell {
            id,
            state,
            position,
        }
    }

    fn is_alive(&self) -> bool {
        self.state
    }

    fn update_state(&mut self, live_neighbors: i32) {
        if self.state {
            if live_neighbors < 2 || live_neighbors > 3 {
                self.state = false;
            }
        } else {
            if live_neighbors == 3 {
                self.state = true;
            }
        }
    }
}

struct Grid {
    cells: Vec<Cell>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(width: i32, height: i32) -> Grid {
        let mut cells = Vec::new();
        let mut id = 0;
        for x in 0..width {
            for y in 0..height {
                cells.push(Cell::new(id, false, (x, y)));
                id += 1;
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }

    fn get_neighbors(&self, cell: &Cell) -> Vec<&Cell> {
        let mut neighbors = Vec::new();
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let new_x = cell.position.0 + dx;
            let new_y = cell.position.1 + dy;
            if new_x >= 0 && new_x < self.width && new_y >= 0 && new_y < self.height {
                if let Some(neighbor) = self.cells.iter().find(|&c| c.position == (new_x, new_y)) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }

    fn update(&mut self) {
        let mut next_state = self.cells.clone();
        for cell in self.cells.iter() {
            let live_neighbors = self
                .get_neighbors(cell)
                .iter()
                .filter(|&&neighbor| neighbor.is_alive())
                .count() as i32;

            if let Some(next_cell) = next_state.iter_mut().find(|c| c.position == cell.position) {
                next_cell.update_state(live_neighbors);
            }
        }
        self.cells = next_state;
    }

    fn print(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells.iter().find(|&c| c.position == (x, y)).unwrap();
                if cell.is_alive() {
                    result.push_str("■ ");
                } else {
                    result.push_str("□ ");
                }
            }
            result.push_str("\n");
        }
        result
    }

    fn new_random(width: i32, height: i32, alive_percentage: f32) -> Grid {
        let mut cells = Vec::new();
        let mut id = 0;
        for x in 0..width {
            for y in 0..height {
                let state = random::<f32>() < alive_percentage;
                cells.push(Cell::new(id, state, (x, y)));
                id += 1;
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }
}

// 使用 wasm-bindgen 导出函数
#[wasm_bindgen]
pub fn generate_random() -> Vec<u8> {
    let mut buffer = vec![0u8; 32];
    getrandom(&mut buffer).expect("failed to generate random bytes");
    buffer
}
#[wasm_bindgen]
pub fn run_simulation(width: i32, height: i32, generations: i32) -> String {
    let mut grid = Grid::new_random(width, height, 0.3);

    let mut output = format!("Generation 0:\n{}", grid.print());

    for i in 1..=generations {
        grid.update();
        output.push_str(&format!("\nGeneration {}:\n{}", i, grid.print()));
    }

    output
}
