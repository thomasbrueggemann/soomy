use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Zoomist;

    #[wasm_bindgen(constructor)]
    pub fn new(id: &str) -> Zoomist;
}
