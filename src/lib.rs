mod utils;

use std::mem;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    fn renderme(x: i32, y: i32, w: usize, h: usize, vec_ptr: *const Pixel, vec_len: usize);
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
    let cells: Vec<Pixel> = vec![
        Pixel {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
        Pixel {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        },
        Pixel {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        },
    ];
    renderme(
        100,
        50,
        cells.len(),
        1,
        cells.as_ptr(),
        cells.len() * mem::size_of::<Pixel>(),
    );
}
