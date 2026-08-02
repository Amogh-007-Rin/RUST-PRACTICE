use module_039_solutions::{platform_tag, CRATE_NAME, CRATE_VERSION};

#[test]
fn crate_metadata_comes_from_cargo() {
    assert!(CRATE_NAME.contains("module-039"));
    assert_eq!(CRATE_VERSION, "0.1.0");
}

#[test]
fn platform_tag_matches_the_build_target() {
    assert!(platform_tag() == "unix" || platform_tag() == "non-unix");
    assert_eq!(platform_tag() == "unix", cfg!(unix));
}

#[cfg(feature = "demo")]
use module_039_solutions::{build_tag, demo_square, demo_sum};

#[cfg(feature = "demo")]
mod demo_feature {
    use super::*;

    #[test]
    fn build_tag_reports_the_feature() {
        assert_eq!(build_tag(), "demo-feature-enabled");
    }

    #[test]
    fn demo_math_is_available() {
        assert_eq!(demo_sum(2, 3), 5);
        assert_eq!(demo_sum(0, 0), 0);
        assert_eq!(demo_square(9), 81);
        assert_eq!(demo_square(0), 0);
    }
}
