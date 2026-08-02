//! Module 058: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

// ---------------------------------------------------------------------------
// HAL traits
// ---------------------------------------------------------------------------

/// A digital output pin that can drive high or low voltage.
pub trait GpioOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

/// A digital input pin that can be read.
pub trait GpioInput {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

/// A timer that can block for a given number of milliseconds.
pub trait DelayMs {
    fn delay_ms(&mut self, ms: u32);
}

// ---------------------------------------------------------------------------
// VirtualPin — a mock hardware pin
// ---------------------------------------------------------------------------

/// A mock pin that tracks its state as a `bool`.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualPin {
    pub state: bool,
}

impl VirtualPin {
    /// Create a new `VirtualPin` with the given initial state.
    pub fn new(_initial: bool) -> Self {
        panic!("TODO(module-058): implement VirtualPin::new")
    }

    /// Toggle the pin state: high → low, low → high.
    pub fn toggle(&mut self) {
        panic!("TODO(module-058): implement VirtualPin::toggle")
    }
}

impl GpioOutput for VirtualPin {
    fn set_high(&mut self) {
        panic!("TODO(module-058): implement GpioOutput for VirtualPin")
    }
    fn set_low(&mut self) {
        panic!("TODO(module-058): implement GpioOutput for VirtualPin")
    }
}

impl GpioInput for VirtualPin {
    fn is_high(&self) -> bool {
        panic!("TODO(module-058): implement GpioInput for VirtualPin")
    }
    fn is_low(&self) -> bool {
        panic!("TODO(module-058): implement GpioInput for VirtualPin")
    }
}

// ---------------------------------------------------------------------------
// MockTimer — a mock delay provider
// ---------------------------------------------------------------------------

/// A mock timer that accumulates elapsed milliseconds instead of
/// actually sleeping.
#[derive(Debug, Clone, PartialEq)]
pub struct MockTimer {
    pub elapsed_ms: u32,
}

impl MockTimer {
    pub fn new() -> Self {
        panic!("TODO(module-058): implement MockTimer::new")
    }
}

impl Default for MockTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl DelayMs for MockTimer {
    fn delay_ms(&mut self, _ms: u32) {
        panic!("TODO(module-058): implement DelayMs for MockTimer")
    }
}

// ---------------------------------------------------------------------------
// blink_led — generic application logic
// ---------------------------------------------------------------------------

/// Blink an LED by toggling `pin` high and low for `times` cycles.
///
/// Each cycle: set_high, delay for `on_ms`, set_low, delay for `off_ms`.
pub fn blink_led(pin: &mut impl GpioOutput, delay: &mut impl DelayMs, times: u32) {
    let _ = (pin, delay, times);
    panic!("TODO(module-058): implement blink_led")
}
