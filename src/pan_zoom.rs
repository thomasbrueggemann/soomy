use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = panzoom)]
    pub fn pan_zoom(element: HtmlElement);
}
