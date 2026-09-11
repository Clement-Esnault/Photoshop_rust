use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[wasm_bindgen]
pub fn grayscale(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let gray = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

#[wasm_bindgen]
pub fn sepia(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let new_r = 0.393 * r + 0.769 * g + 0.189 * b;
        let new_g = 0.349 * r + 0.686 * g + 0.168 * b;
        let new_b = 0.272 * r + 0.534 * g + 0.131 * b;

        pixel[0] = new_r.min(255.0) as u8;
        pixel[1] = new_g.min(255.0) as u8;
        pixel[2] = new_b.min(255.0) as u8;
    }
}