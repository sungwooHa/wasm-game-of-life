extern crate wasm_bindgen;
use crate::cell::Cell;
use std::fmt;
use wasm_bindgen::prelude::*;
extern crate js_sys;

#[wasm_bindgen]
pub struct Universe {
    width: u32,       //map size
    height: u32,      //map size
    cells: Vec<Cell>, //height * width 만큼 cell을 가짐.
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_|{
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                }else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,

       }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    //1 tick.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neghbors = self.live_nighbor_count(row, column);

                next[idx] = match (cell, live_neghbors) {
                    //주변에 살아 있는 cell에 2개보다 적을경우 Dead
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    //주변에 살아 있는 cell이 2 or 3일 경우 Alive
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //주변에 살아 있는 cell이 4이상 일 경우 Dead
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //주변에 죽은 cell이 3개일 경우 Alive
                    (Cell::Dead, 3) => Cell::Alive,
                    //나머지는 현재 상태를 유지한다.
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

   pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_width(&mut self, width : u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height : u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }
    
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}


impl Universe{
    
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize //vector 하나로 관리 되게 때문에
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self,  cells : &[(u32,u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive
        }
    }

    fn live_nighbor_count(&self, row: u32, column: u32) -> u8 {
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
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(f, "{}", if cell == Cell::Dead { '◻' } else { '◼' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}