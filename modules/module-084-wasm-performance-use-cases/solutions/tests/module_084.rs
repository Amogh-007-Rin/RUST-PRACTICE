use module_084_solutions::*;

fn test_image() -> (Vec<u8>, u32, u32) {
    let pixels: Vec<u8> = vec![
        255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128, 64, 64, 64, 192, 192, 192, 10, 20, 30, 40,
        50, 60, 100, 150, 200, 200, 100, 50, 0, 0, 0, 255, 255, 255,
    ];
    (pixels, 4, 3)
}

#[test]
fn grayscale_naive_produces_correct_luminance() {
    let (pixels, w, h) = test_image();
    let gray = grayscale_naive(&pixels, w, h);
    assert_eq!(gray.len(), pixels.len());
    assert_eq!(gray[0], 76);
    assert_eq!(gray[1], 76);
    assert_eq!(gray[2], 76);
    assert_eq!(gray[3], 149);
    assert_eq!(gray[4], 149);
    assert_eq!(gray[5], 149);
}

#[test]
fn grayscale_optimized_matches_naive() {
    let (pixels, w, h) = test_image();
    let naive = grayscale_naive(&pixels, w, h);
    let optimized = grayscale_optimized(&pixels, w, h);
    assert_eq!(naive, optimized);
}

#[test]
fn box_blur_radius_zero_is_identity() {
    let (pixels, w, h) = test_image();
    let blurred = box_blur_naive(&pixels, w, h, 0);
    assert_eq!(blurred, pixels);
}

#[test]
fn box_blur_optimized_matches_naive() {
    let (pixels, w, h) = test_image();
    for radius in 0..=3 {
        let naive = box_blur_naive(&pixels, w, h, radius);
        let optimized = box_blur_optimized(&pixels, w, h, radius);
        assert_eq!(naive, optimized, "mismatch at radius {radius}");
    }
}

#[test]
fn pipeline_naive_and_optimized_match() {
    let (pixels, w, h) = test_image();
    let naive = pipeline_naive(&pixels, w, h);
    let optimized = pipeline_optimized(&pixels, w, h);
    assert_eq!(naive, optimized);
}

#[test]
fn pipeline_output_is_grayscale_blurred() {
    let (pixels, w, h) = test_image();
    let result = pipeline_naive(&pixels, w, h);
    for chunk in result.chunks(3) {
        assert_eq!(chunk[0], chunk[1]);
        assert_eq!(chunk[1], chunk[2]);
    }
}

#[test]
fn single_pixel_image() {
    let pixels = vec![100, 150, 200];
    let gray = grayscale_naive(&pixels, 1, 1);
    assert_eq!(gray[0], 140);
    assert_eq!(gray[1], 140);
    assert_eq!(gray[2], 140);
    let blurred = box_blur_naive(&pixels, 1, 1, 1);
    assert_eq!(blurred, pixels);
}
