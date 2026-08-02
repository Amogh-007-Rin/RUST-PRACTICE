//! Module 084: WASM Performance Use Cases — Image Filters
//!
//! You will implement two versions of image filters (grayscale + box blur):
//! a naive version and an optimized version. Both must produce identical
//! output. The optimized version should use chunked iteration and reduce
//! bounds checks for better performance — the kind of optimization that
//! matters when your code runs as WASM in a browser.

/// Convert an RGB image to grayscale using the luminance formula:
/// `gray = 0.299*R + 0.587*G + 0.114*B`
///
/// `pixels` is a flat `Vec<u8>` in RGBRGBRGB... order (length = width*height*3).
/// Returns a new `Vec<u8>` of the same length where each pixel's R, G, B
/// channels are all set to the grayscale value.
pub fn grayscale_naive(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let _ = (pixels, width, height);
    // TODO(module-084): Implement naive grayscale conversion.
    // Iterate pixel by pixel, compute luminance, write R=G=B=luminance.
    // Use (pixels.len() / 3) to determine pixel count.
    panic!("not implemented")
}

/// Optimized grayscale: same result as `grayscale_naive`, but use
/// chunked iteration (`chunks_exact(3)`) and minimize bounds checks.
pub fn grayscale_optimized(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let _ = (pixels, width, height);
    // TODO(module-084): Implement optimized grayscale using chunks_exact(3).
    panic!("not implemented")
}

/// Apply a box blur of the given `radius` to an RGB image.
/// For each pixel, average all pixels within the radius (inclusive) in both
/// x and y directions. Clamp at image edges (don't wrap).
///
/// `pixels` is RGBRGBRGB... flat buffer. Returns a new buffer of the same size.
pub fn box_blur_naive(pixels: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
    let _ = (pixels, width, height, radius);
    // TODO(module-084): Implement naive box blur.
    // For each output pixel (x, y), average all input pixels in
    // [x-radius..=x+radius, y-radius..=y+radius], clamped to image bounds.
    panic!("not implemented")
}

/// Optimized box blur: same result, but precompute a summed-area table (SAT)
/// (also called integral image) for each channel, then compute each output
/// pixel in O(1) from the SAT instead of O(radius^2).
pub fn box_blur_optimized(pixels: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
    let _ = (pixels, width, height, radius);
    // TODO(module-084): Implement SAT-based box blur.
    // Build a u64 SAT for each channel, then for each pixel compute the
    // rectangular sum in O(1) and divide by the actual count of contributing pixels.
    panic!("not implemented")
}

/// Apply grayscale then box blur (radius=1) — the full pipeline.
pub fn pipeline_naive(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let _ = (pixels, width, height);
    // TODO(module-084): Compose grayscale_naive + box_blur_naive(radius=1).
    panic!("not implemented")
}

/// Apply grayscale then box blur (radius=1) using optimized paths.
pub fn pipeline_optimized(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let _ = (pixels, width, height);
    // TODO(module-084): Compose grayscale_optimized + box_blur_optimized(radius=1).
    panic!("not implemented")
}
