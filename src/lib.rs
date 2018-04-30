#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    size: u8,
    cells: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(size: u8) -> Universe {
        return Universe {
            size,
            cells: vec!(0; (size.pow(2)) as usize),
        }
    }
    pub fn cells_pointer(&self) -> *const u8 {
        self.cells.as_ptr()
    }
    pub fn seed(&mut self) {
        for cell_index in 0..self.size.pow(2) {
            if cell_index % 3 == 0 {
                self.cells[cell_index as usize] = 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inital_generation_no_panic() {
        Universe::new(10);
    }
    #[test]
    #[should_panic]
    fn test_input_bounds() {
        Universe::new(64);
    }
    #[test]
    fn test_seed_no_panic() {
        Universe::new(10).seed();
    }
    #[test]
    fn test_seed_alters_data() {
        let mut universe = Universe::new(10);
        let cells_before = universe.cells.clone();
        universe.seed();
        let cells_after = universe.cells.clone();
        assert_ne!(cells_before, cells_after);
    }
}
