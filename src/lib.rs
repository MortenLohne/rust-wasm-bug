use std::mem;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let cursed_vec = get_cursed_vec();
    console::log_1(
        &format!(
            "Attempting to drop cursed vector at {:p}",
            cursed_vec.as_ptr()
        )
        .into(),
    );
    mem::drop(cursed_vec);
    console::log_1(&JsValue::from_str("Dropped cursed vector"));
    Ok(())
}

fn get_cursed_vec() -> Vec<u8> {
    // Reserve a 1.5 GiB outer vector, to OOM faster
    let mut test_vector: Vec<Vec<u8>> = Vec::with_capacity(2usize.pow(27));

    // Allocate 1KiB vectors until we run out of memory
    loop {
        let mut inner_vector = vec![];
        if inner_vector.try_reserve_exact(1024).is_err() {
            // Remove the final inner vector. It is cursed, and cannot be dropped.
            return mem::take(test_vector.last_mut().unwrap());
        };
        test_vector.push(inner_vector);
    }
}
