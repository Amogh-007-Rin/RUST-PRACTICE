//! Module 059: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

// ---------------------------------------------------------------------------
// SimulatedMcu
// ---------------------------------------------------------------------------

/// A simulated microcontroller with analog pins, digital pins, and a timer.
#[derive(Debug, Clone)]
pub struct SimulatedMcu {
    pub analog_pins: [u16; 4],
    pub digital_pins: [bool; 4],
    pub timer_counter: u64,
}

impl SimulatedMcu {
    /// Create a new MCU with all pins at 0 / false and timer at 0.
    pub fn new() -> Self {
        panic!("TODO(module-059): implement SimulatedMcu::new")
    }
}

impl Default for SimulatedMcu {
    fn default() -> Self {
        Self::new()
    }
}

/// Read the analog value from `pin_id`. Returns `None` if `pin_id >= 4`.
pub fn read_sensor(mcu: &SimulatedMcu, pin_id: u8) -> Option<u16> {
    let _ = (mcu, pin_id);
    panic!("TODO(module-059): implement read_sensor")
}

/// Simulate a timer interrupt: increment `timer_counter` by 1.
pub fn process_timer_interrupt(mcu: &mut SimulatedMcu) {
    let _ = mcu;
    panic!("TODO(module-059): implement process_timer_interrupt")
}

// ---------------------------------------------------------------------------
// Button debouncer state machine
// ---------------------------------------------------------------------------

/// The debouncer's current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebouncerState {
    /// No button activity.
    Idle,
    /// Button is down but hasn't been stable long enough.
    Pressing,
    /// Button is confirmed pressed.
    Pressed,
    /// Button is up but hasn't been stable long enough.
    Releasing,
}

/// A button debouncer that filters mechanical switch bounce.
///
/// Call `update()` on every timer tick with the raw button state.
/// It returns:
/// - `Some(true)` when a press is confirmed.
/// - `Some(false)` when a release is confirmed.
/// - `None` when no transition is confirmed.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ButtonDebouncer {
    pub state: DebouncerState,
    stable_counter: u32,
    debounce_ticks: u32,
}

impl ButtonDebouncer {
    /// Create a new debouncer in `Idle` state.
    ///
    /// `debounce_ticks` is how many consecutive identical samples are
    /// required before a transition is confirmed (typical: 5–20).
    pub fn new(debounce_ticks: u32) -> Self {
        let _ = debounce_ticks;
        panic!("TODO(module-059): implement ButtonDebouncer::new")
    }

    /// Process one timer tick with the raw button reading.
    pub fn update(&mut self, button_is_pressed: bool) -> Option<bool> {
        let _ = button_is_pressed;
        panic!("TODO(module-059): implement ButtonDebouncer::update")
    }

    /// Reset the debouncer to Idle state.
    pub fn reset(&mut self) {
        panic!("TODO(module-059): implement ButtonDebouncer::reset")
    }
}
