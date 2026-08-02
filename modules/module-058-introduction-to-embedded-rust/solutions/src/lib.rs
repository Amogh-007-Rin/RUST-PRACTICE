//! Module 058: solution — the reference implementation.

pub trait GpioOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

pub trait GpioInput {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

pub trait DelayMs {
    fn delay_ms(&mut self, ms: u32);
}

// ---------------------------------------------------------------------------
// VirtualPin
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualPin {
    pub state: bool,
}

impl VirtualPin {
    pub fn new(initial: bool) -> Self {
        VirtualPin { state: initial }
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
    }
}

impl GpioOutput for VirtualPin {
    fn set_high(&mut self) {
        self.state = true;
    }

    fn set_low(&mut self) {
        self.state = false;
    }
}

impl GpioInput for VirtualPin {
    fn is_high(&self) -> bool {
        self.state
    }

    fn is_low(&self) -> bool {
        !self.state
    }
}

// ---------------------------------------------------------------------------
// MockTimer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct MockTimer {
    pub elapsed_ms: u32,
}

impl MockTimer {
    pub fn new() -> Self {
        MockTimer { elapsed_ms: 0 }
    }
}

impl DelayMs for MockTimer {
    fn delay_ms(&mut self, ms: u32) {
        self.elapsed_ms += ms;
    }
}

impl Default for MockTimer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// blink_led
// ---------------------------------------------------------------------------

pub fn blink_led(pin: &mut impl GpioOutput, delay: &mut impl DelayMs, times: u32) {
    for _ in 0..times {
        pin.set_high();
        delay.delay_ms(500);
        pin.set_low();
        delay.delay_ms(500);
    }
}
