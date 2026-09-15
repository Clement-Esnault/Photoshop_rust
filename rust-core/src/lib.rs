use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

// ---------- GRAYSCALE : version 1, moyenne simple ----------
// Fait la moyenne brute de R, G, B. Rapide mais pas très fidèle à
// la perception humaine (l'œil est plus sensible au vert qu'au bleu).
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
// Coefficients standard (ITU-R BT.601) : plus de poids sur le vert,
// moins sur le bleu, plus fidèle à la perception humaine.
#[wasm_bindgen]
pub fn grayscale_weighted(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        // Les trois coefficients somment à 1.0 — moyenne pondérée.
        let gray = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;

        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

// ---------- SEPIA ----------
// La boucle sur les pixels appelle une fonction utilitaire séparée
// pour le calcul, plus facile à tester isolément.
#[wasm_bindgen]
pub fn sepia(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        let (new_r, new_g, new_b) = sepia_pixel(pixel[0], pixel[1], pixel[2]);
        pixel[0] = new_r;
        pixel[1] = new_g;
        pixel[2] = new_b;
    }
}

// Fonction privée (pas de #[wasm_bindgen]) : transforme 3 valeurs en 3
// valeurs, sans connaître le tableau ni le canal alpha.
fn sepia_pixel(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let new_r = 0.393 * r + 0.769 * g + 0.189 * b;
    let new_g = 0.349 * r + 0.686 * g + 0.168 * b;
    let new_b = 0.272 * r + 0.534 * g + 0.131 * b;

    // .min(255.0) évite le dépassement (ex: blanc pur peut sinon donner >255).
    (new_r.min(255.0) as u8, new_g.min(255.0) as u8, new_b.min(255.0) as u8)
}

// ---------- INVERSION ----------
// Le plus simple : aucun risque de dépassement (255 - x reste 0-255),
// donc pas de conversion f32 ni de .min() nécessaires.
#[wasm_bindgen]
pub fn invert(data: &mut [u8]) {
    for pixel in data.chunks_exact_mut(4) {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
    }
}

// ---------- FLOU : version naïve, O(radius²) par pixel ----------
// Pour chaque pixel, on relit tout le carré de voisins à chaque fois.
#[wasm_bindgen]
pub fn box_blur(data: &mut [u8], width: u32, height: u32, radius: u32) {
    let width = width as i32;
    let height = height as i32;
    let radius = radius as i32;

    // Copie immutable des pixels d'origine, pour ne pas lire des pixels
    // déjà flous pendant qu'on écrit dans `data`.
    let original = data.to_vec();

    // Closure : lit un pixel à (x, y), en collant aux bords si on dépasse.
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
// Un flou carré = un flou horizontal PUIS un flou vertical sur le
// résultat. Même rendu visuel, beaucoup moins de calculs.
#[wasm_bindgen]
pub fn box_blur_fast(data: &mut [u8], width: u32, height: u32, radius: u32) {
    let (width, height, radius) = (width as i32, height as i32, radius as i32);

    let horizontal = blur_pass(data, width, height, radius, true);
    let vertical = blur_pass(&horizontal, width, height, radius, false);

    // `data` doit rester &mut [u8] (format attendu par wasm_bindgen),
    // donc on copie le résultat dedans plutôt que de le remplacer.
    data.copy_from_slice(&vertical);
}

// Fonction privée réutilisée pour les deux passes. `horizontal: bool`
// choisit la direction. Retourne un Vec<u8> possédé.
fn blur_pass(data: &[u8], width: i32, height: i32, radius: i32, horizontal: bool) -> Vec<u8> {
    let mut output = data.to_vec();

    for y in 0..height {
        for x in 0..width {
            for channel in 0..3 {
                let mut sum: u32 = 0;
                let mut count: u32 = 0;

                // On décale x OU y selon la direction, jamais les deux —
                // c'est ce qui réduit le coût de O(r²) à O(r).
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