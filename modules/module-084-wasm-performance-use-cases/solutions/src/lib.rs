//! Module 084: WASM Performance Use Cases — Image Filters (solution)

pub fn grayscale_naive(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let _ = (width, height);
    let pixel_count = pixels.len() / 3;
    let mut out = vec![0u8; pixels.len()];
    for i in 0..pixel_count {
        let r = pixels[i * 3] as f64;
        let g = pixels[i * 3 + 1] as f64;
        let b = pixels[i * 3 + 2] as f64;
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
        out[i * 3] = gray;
        out[i * 3 + 1] = gray;
        out[i * 3 + 2] = gray;
    }
    out
}

pub fn grayscale_optimized(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let _ = (width, height);
    let mut out = Vec::with_capacity(pixels.len());
    for chunk in pixels.chunks_exact(3) {
        let r = chunk[0] as f64;
        let g = chunk[1] as f64;
        let b = chunk[2] as f64;
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
        out.push(gray);
        out.push(gray);
        out.push(gray);
    }
    out
}

pub fn box_blur_naive(pixels: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
    let w = width as i64;
    let h = height as i64;
    let r = radius as i64;
    let mut out = vec![0u8; pixels.len()];

    for y in 0..h {
        for x in 0..w {
            let mut sum_r: u64 = 0;
            let mut sum_g: u64 = 0;
            let mut sum_b: u64 = 0;
            let mut count: u64 = 0;

            let y_min = (y - r).max(0);
            let y_max = (y + r).min(h - 1);
            let x_min = (x - r).max(0);
            let x_max = (x + r).min(w - 1);

            for ky in y_min..=y_max {
                for kx in x_min..=x_max {
                    let idx = ((ky * w + kx) * 3) as usize;
                    sum_r += pixels[idx] as u64;
                    sum_g += pixels[idx + 1] as u64;
                    sum_b += pixels[idx + 2] as u64;
                    count += 1;
                }
            }

            let idx = ((y * w + x) * 3) as usize;
            out[idx] = (sum_r / count) as u8;
            out[idx + 1] = (sum_g / count) as u8;
            out[idx + 2] = (sum_b / count) as u8;
        }
    }
    out
}

pub fn box_blur_optimized(pixels: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let r = radius as usize;

    // Build summed-area tables for each channel
    // sat[y+1][x+1] = sum of all pixels in [0..=y][0..=x]
    let sat_w = w + 1;
    let sat_h = h + 1;
    let mut sat_r = vec![0u64; sat_w * sat_h];
    let mut sat_g = vec![0u64; sat_w * sat_h];
    let mut sat_b = vec![0u64; sat_w * sat_h];

    for y in 0..h {
        let mut row_r = 0u64;
        let mut row_g = 0u64;
        let mut row_b = 0u64;
        for x in 0..w {
            let idx = (y * w + x) * 3;
            row_r += pixels[idx] as u64;
            row_g += pixels[idx + 1] as u64;
            row_b += pixels[idx + 2] as u64;
            let sat_idx = (y + 1) * sat_w + (x + 1);
            let prev_row = y * sat_w + (x + 1);
            sat_r[sat_idx] = row_r + sat_r[prev_row];
            sat_g[sat_idx] = row_g + sat_g[prev_row];
            sat_b[sat_idx] = row_b + sat_b[prev_row];
        }
    }

    let query = |sat: &[u64], x1: usize, y1: usize, x2: usize, y2: usize| -> u64 {
        let a = sat[(y2 + 1) * sat_w + (x2 + 1)];
        let b = sat[y1 * sat_w + (x2 + 1)];
        let c = sat[(y2 + 1) * sat_w + x1];
        let d = sat[y1 * sat_w + x1];
        (a + d).saturating_sub(b).saturating_sub(c)
    };

    let mut out = vec![0u8; pixels.len()];
    for y in 0..h {
        for x in 0..w {
            let y_min = y.saturating_sub(r);
            let x_min = x.saturating_sub(r);
            let y_max = (y + r).min(h - 1);
            let x_max = (x + r).min(w - 1);
            let count = ((y_max - y_min + 1) * (x_max - x_min + 1)) as u64;

            let sr = query(&sat_r, x_min, y_min, x_max, y_max);
            let sg = query(&sat_g, x_min, y_min, x_max, y_max);
            let sb = query(&sat_b, x_min, y_min, x_max, y_max);

            let idx = (y * w + x) * 3;
            out[idx] = (sr / count) as u8;
            out[idx + 1] = (sg / count) as u8;
            out[idx + 2] = (sb / count) as u8;
        }
    }
    out
}

pub fn pipeline_naive(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let gray = grayscale_naive(pixels, width, height);
    box_blur_naive(&gray, width, height, 1)
}

pub fn pipeline_optimized(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let gray = grayscale_optimized(pixels, width, height);
    box_blur_optimized(&gray, width, height, 1)
}
