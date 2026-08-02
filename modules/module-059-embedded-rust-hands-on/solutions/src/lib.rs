//! Module 059: solution — the reference implementation.

#[derive(Debug, Clone)]
pub struct SimulatedMcu {
    pub analog_pins: [u16; 4],
    pub digital_pins: [bool; 4],
    pub timer_counter: u64,
}

impl SimulatedMcu {
    pub fn new() -> Self {
        SimulatedMcu {
            analog_pins: [0; 4],
            digital_pins: [false; 4],
            timer_counter: 0,
        }
    }
}

impl Default for SimulatedMcu {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_sensor(mcu: &SimulatedMcu, pin_id: u8) -> Option<u16> {
    mcu.analog_pins.get(pin_id as usize).copied()
}

pub fn process_timer_interrupt(mcu: &mut SimulatedMcu) {
    mcu.timer_counter += 1;
}

// ---------------------------------------------------------------------------
// Button debouncer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebouncerState {
    Idle,
    Pressing,
    Pressed,
    Releasing,
}

#[derive(Debug, Clone)]
pub struct ButtonDebouncer {
    pub state: DebouncerState,
    stable_counter: u32,
    debounce_ticks: u32,
}

impl ButtonDebouncer {
    pub fn new(debounce_ticks: u32) -> Self {
        ButtonDebouncer {
            state: DebouncerState::Idle,
            stable_counter: 0,
            debounce_ticks,
        }
    }

    pub fn update(&mut self, button_is_pressed: bool) -> Option<bool> {
        match self.state {
            DebouncerState::Idle => {
                if button_is_pressed {
                    self.state = DebouncerState::Pressing;
                    self.stable_counter = 1;
                }
                None
            }
            DebouncerState::Pressing => {
                if button_is_pressed {
                    self.stable_counter += 1;
                    if self.stable_counter >= self.debounce_ticks {
                        self.state = DebouncerState::Pressed;
                        return Some(true);
                    }
                } else {
                    self.state = DebouncerState::Idle;
                }
                None
            }
            DebouncerState::Pressed => {
                if !button_is_pressed {
                    self.state = DebouncerState::Releasing;
                    self.stable_counter = 1;
                }
                None
            }
            DebouncerState::Releasing => {
                if !button_is_pressed {
                    self.stable_counter += 1;
                    if self.stable_counter >= self.debounce_ticks {
                        self.state = DebouncerState::Idle;
                        return Some(false);
                    }
                } else {
                    self.state = DebouncerState::Pressed;
                }
                None
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = DebouncerState::Idle;
        self.stable_counter = 0;
    }
}
