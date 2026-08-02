use module_059_solutions::{
    process_timer_interrupt, read_sensor, ButtonDebouncer, DebouncerState, SimulatedMcu,
};

// ---------------------------------------------------------------------------
// SimulatedMcu
// ---------------------------------------------------------------------------

#[test]
fn mcu_new_defaults() {
    let mcu = SimulatedMcu::new();
    assert_eq!(mcu.timer_counter, 0);
    for v in &mcu.analog_pins {
        assert_eq!(*v, 0);
    }
    for v in &mcu.digital_pins {
        assert!(!v);
    }
}

#[test]
fn read_sensor_valid() {
    let mut mcu = SimulatedMcu::new();
    mcu.analog_pins[2] = 2048;
    assert_eq!(read_sensor(&mcu, 2), Some(2048));
    assert_eq!(read_sensor(&mcu, 0), Some(0));
    assert_eq!(read_sensor(&mcu, 3), Some(0));
}

#[test]
fn read_sensor_out_of_range() {
    let mcu = SimulatedMcu::new();
    assert_eq!(read_sensor(&mcu, 4), None);
    assert_eq!(read_sensor(&mcu, 255), None);
}

#[test]
fn timer_interrupt_increments() {
    let mut mcu = SimulatedMcu::new();
    assert_eq!(mcu.timer_counter, 0);
    for i in 1..=5 {
        process_timer_interrupt(&mut mcu);
        assert_eq!(mcu.timer_counter, i);
    }
}

// ---------------------------------------------------------------------------
// ButtonDebouncer
// ---------------------------------------------------------------------------

#[test]
fn debouncer_initial_state() {
    let db = ButtonDebouncer::new(5);
    assert_eq!(db.state, DebouncerState::Idle);
}

#[test]
fn debouncer_no_activity() {
    let mut db = ButtonDebouncer::new(5);
    for _ in 0..100 {
        assert_eq!(db.update(false), None);
    }
    assert_eq!(db.state, DebouncerState::Idle);
}

#[test]
fn debouncer_bounce_during_press() {
    let mut db = ButtonDebouncer::new(5);
    db.update(true);
    db.update(false);
    assert_eq!(db.state, DebouncerState::Idle);

    db.update(true);
    db.update(true);
    db.update(false);
    assert_eq!(db.state, DebouncerState::Idle);
}

#[test]
fn debouncer_confirmed_press() {
    let mut db = ButtonDebouncer::new(3);

    assert_eq!(db.update(true), None);
    assert_eq!(db.update(true), None);
    assert_eq!(db.update(true), Some(true));
    assert_eq!(db.state, DebouncerState::Pressed);
}

#[test]
fn debouncer_held_and_released() {
    let mut db = ButtonDebouncer::new(3);

    db.update(true);
    db.update(true);
    assert_eq!(db.update(true), Some(true));

    for _ in 0..10 {
        assert_eq!(db.update(true), None);
    }

    db.update(false);
    db.update(true);
    assert_eq!(db.state, DebouncerState::Pressed);

    assert_eq!(db.update(false), None);
    assert_eq!(db.update(false), None);
    assert_eq!(db.update(false), Some(false));
    assert_eq!(db.state, DebouncerState::Idle);
}

#[test]
fn debouncer_reset() {
    let mut db = ButtonDebouncer::new(3);
    db.update(true);
    db.update(true);
    assert_eq!(db.state, DebouncerState::Pressing);

    db.reset();
    assert_eq!(db.state, DebouncerState::Idle);
}

#[test]
fn debouncer_full_cycle_two_presses() {
    let mut db = ButtonDebouncer::new(3);

    db.update(true);
    db.update(true);
    assert_eq!(db.update(true), Some(true));
    db.update(false);
    db.update(false);
    assert_eq!(db.update(false), Some(false));

    db.update(true);
    db.update(true);
    assert_eq!(db.update(true), Some(true));
    db.update(false);
    db.update(false);
    assert_eq!(db.update(false), Some(false));
}
