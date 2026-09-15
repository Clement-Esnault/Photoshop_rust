use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

// ---------- GRAYSCALE : version 1, moyenne simple ----------
#[wasm_bindgen]
pub fn grayscale(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let gray = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

// ---------- GRAYSCALE : version 2, luminance pondérée ----------
#[wasm_bindgen]
pub fn grayscale_weighted(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let gray = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

// ---------- SEPIA ----------
#[wasm_bindgen]
pub fn sepia(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let (new_r, new_g, new_b) = sepia_pixel(pixel[0], pixel[1], pixel[2]);
        pixel[0] = new_r;
        pixel[1] = new_g;
        pixel[2] = new_b;
    }
}

fn sepia_pixel(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let new_r = 0.393 * r + 0.769 * g + 0.189 * b;
    let new_g = 0.349 * r + 0.686 * g + 0.168 * b;
    let new_b = 0.272 * r + 0.534 * g + 0.131 * b;

    (new_r.min(255.0) as u8, new_g.min(255.0) as u8, new_b.min(255.0) as u8)
}

// ---------- INVERSION ----------
#[wasm_bindgen]
pub fn invert(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
    }
}

// ---------- FLOU : version naïve, O(radius²) par pixel ----------
#[wasm_bindgen]
pub fn box_blur(data: &mut [u8], width: u32, height: u32, radius: u32) {
    let width = width as i32;
    let height = height as i32;
    let radius = radius as i32;

    let original = data.to_vec();

    let get_pixel = |x: i32, y: i32, channel: usize| -> u32 {
        let x = x.clamp(0, width - 1);
        let y = y.clamp(0, height - 1);
        let index = ((y * width + x) * 4 + channel as i32) as usize;
        original[index] as u32
    };

    for y in 0..height {
        for x in 0..width {
            for channel in 0..3 {
                let mut sum: u32 = 0;
                let mut count: u32 = 0;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        sum += get_pixel(x + dx, y + dy, channel);
                        count += 1;
                    }
                }
                let index = ((y * width + x) * 4 + channel as i32) as usize;
                data[index] = (sum / count) as u8;
            }
        }
    }
}

// ---------- FLOU : version séparable, O(radius) par pixel ----------
#[wasm_bindgen]
pub fn box_blur_fast(data: &mut [u8], width: u32, height: u32, radius: u32) {
    let (width, height, radius) = (width as i32, height as i32, radius as i32);

    let horizontal = blur_pass(data, width, height, radius, true);
    let vertical = blur_pass(&horizontal, width, height, radius, false);

    data.copy_from_slice(&vertical);
}

fn blur_pass(data: &[u8], width: i32, height: i32, radius: i32, horizontal: bool) -> Vec<u8> {
    let mut output = data.to_vec();

    for y in 0..height {
        for x in 0..width {
            for channel in 0..3 {
                let mut sum: u32 = 0;
                let mut count: u32 = 0;

                for offset in -radius..=radius {
                    let (sx, sy) = if horizontal { (x + offset, y) } else { (x, y + offset) };
                    let sx = sx.clamp(0, width - 1);
                    let sy = sy.clamp(0, height - 1);
                    let index = ((sy * width + sx) * 4 + channel) as usize;
                    sum += data[index] as u32;
                    count += 1;
                }

                let index = ((y * width + x) * 4 + channel) as usize;
                output[index] = (sum / count) as u8;
            }
        }
    }
    output
}


#[wasm_bindgen]
pub fn brightness(data: &mut [u8], amount: i32) {
    for pixel in data.chunks_exact_mut(4) {
        for i in 0..3 {
            // clamp() garantit qu'on reste dans 0-255 même avec un amount négatif.
            let value = pixel[i] as i32 + amount;
            pixel[i] = value.clamp(0, 255) as u8;
        }
    }
}


#[wasm_bindgen]
pub fn contrast(data: &mut [u8], amount: f32) {
    // `amount` est un facteur multiplicateur : 1.0 = aucun changement,
    // >1.0 = plus de contraste, <1.0 = moins de contraste.
    for pixel in data.chunks_exact_mut(4) {
        for i in 0..3 {
            let value = pixel[i] as f32;
            // On centre autour de 128, on amplifie l'écart, on recentre.
            let new_value = (value - 128.0) * amount + 128.0;
            pixel[i] = new_value.clamp(0.0, 255.0) as u8;
        }
    }
}

// ---------- TESTS ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sums_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn grayscale_averages_rgb() {
        // Rouge pur : R=255, G=0, B=0, A=255.
        let mut pixel = [255u8, 0, 0, 255];
        grayscale(&mut pixel);

        // Moyenne attendue : (255+0+0)/3 = 85.
        assert_eq!(pixel[0], 85);
        assert_eq!(pixel[1], 85);
        assert_eq!(pixel[2], 85);
        assert_eq!(pixel[3], 255); // alpha inchangé
    }

    #[test]
    fn grayscale_weighted_favors_green() {
        // Deux pixels de même luminosité brute (255) mais de canaux différents :
        // le vert doit peser plus lourd que le bleu dans le résultat.
        let mut green_pixel = [0u8, 255, 0, 255];
        let mut blue_pixel = [0u8, 0, 255, 255];

        grayscale_weighted(&mut green_pixel);
        grayscale_weighted(&mut blue_pixel);

        // 0.587 * 255 ≈ 150 pour le vert, 0.114 * 255 ≈ 29 pour le bleu.
        assert!(green_pixel[0] > blue_pixel[0]);
    }

    #[test]
    fn sepia_turns_black_into_black() {
        // Un pixel noir doit rester noir (tous les coefficients * 0 = 0).
        let mut pixel = [0u8, 0, 0, 255];
        sepia(&mut pixel);

        assert_eq!(pixel[0], 0);
        assert_eq!(pixel[1], 0);
        assert_eq!(pixel[2], 0);
    }

#[test]
fn sepia_clamps_white_to_255() {
    // Blanc pur : R et G dépassent 255 sans le .min() (1.351 et 1.203
    // respectivement), donc plafonnent à 255. B reste sous 255 naturellement
    // (facteur 0.937), donc pas de plafonnement nécessaire pour ce canal.
    let mut pixel = [255u8, 255, 255, 255];
    sepia(&mut pixel);

    assert_eq!(pixel[0], 255);
    assert_eq!(pixel[1], 255);
    assert_eq!(pixel[2], 238);
}
    #[test]
    fn invert_flips_each_channel() {
        let mut pixel = [0u8, 100, 255, 255];
        invert(&mut pixel);

        assert_eq!(pixel[0], 255); // 255 - 0
        assert_eq!(pixel[1], 155); // 255 - 100
        assert_eq!(pixel[2], 0);   // 255 - 255
        assert_eq!(pixel[3], 255); // alpha inchangé
    }

    #[test]
    fn invert_twice_returns_original() {
        // Propriété mathématique : inverser deux fois = ne rien changer.
        let mut pixel = [10u8, 20, 30, 255];
        let original = pixel;

        invert(&mut pixel);
        invert(&mut pixel);

        assert_eq!(pixel, original);
    }

    #[test]
    fn box_blur_on_uniform_image_stays_unchanged() {
        // Une image 3x3 entièrement grise (128,128,128) : le flou d'une
        // zone uniforme ne doit rien changer, la moyenne reste 128 partout.
        let mut data = vec![128u8; 3 * 3 * 4];
        box_blur(&mut data, 3, 3, 1);

        for value in &data {
            assert_eq!(*value, 128);
        }
    }

    #[test]
    fn box_blur_naive_and_fast_give_same_result() {
        // Les deux implémentations doivent produire un résultat identique,
        // seule leur performance diffère.
        let width = 5;
        let height = 5;
        let mut data_naive: Vec<u8> = (0..width * height * 4).map(|i| (i % 256) as u8).collect();
        let mut data_fast = data_naive.clone();

        box_blur(&mut data_naive, width, height, 2);
        box_blur_fast(&mut data_fast, width, height, 2);

        assert_eq!(data_naive, data_fast);
    }
    #[test]
fn brightness_increases_value() {
    let mut pixel = [100u8, 100, 100, 255];
    brightness(&mut pixel, 50);
    assert_eq!(pixel[0], 150);
}

#[test]
fn brightness_clamps_at_255() {
    let mut pixel = [200u8, 200, 200, 255];
    brightness(&mut pixel, 100); // 200+100 = 300, doit plafonner à 255
    assert_eq!(pixel[0], 255);
}

#[test]
fn contrast_at_one_changes_nothing() {
    let mut pixel = [150u8, 150, 150, 255];
    contrast(&mut pixel, 1.0);
    assert_eq!(pixel[0], 150);
}

#[test]
fn contrast_pushes_away_from_middle_gray() {
    // 180 est au-dessus de 128 (gris moyen) : avec un contraste > 1,
    // il doit s'éloigner encore plus vers 255.
    let mut pixel = [180u8, 180, 180, 255];
    contrast(&mut pixel, 2.0);
    assert!(pixel[0] > 180);
}
}