mod minesweeper;
mod random;

use std::cell::RefCell;

use minesweeper::*;
use wasm_bindgen::prelude::*;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper > = RefCell::new(Minesweeper::new(10, 10, 5));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_fields(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().open((x, y));
    });
}
