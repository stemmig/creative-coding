mod window;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn main() {
    window::run();
    println!("Hello, world!");
}
