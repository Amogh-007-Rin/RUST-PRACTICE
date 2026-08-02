use module_058_solutions::{blink_led, DelayMs, GpioInput, GpioOutput, MockTimer, VirtualPin};

// ---------------------------------------------------------------------------
// VirtualPin
// ---------------------------------------------------------------------------

#[test]
fn virtual_pin_construction() {
    let pin = VirtualPin::new(false);
    assert!(!pin.state);
    assert!(pin.is_low());

    let pin = VirtualPin::new(true);
    assert!(pin.state);
    assert!(pin.is_high());
}

#[test]
fn virtual_pin_output() {
    let mut pin = VirtualPin::new(false);
    pin.set_high();
    assert!(pin.state);
    pin.set_low();
    assert!(!pin.state);
}

#[test]
fn virtual_pin_toggle() {
    let mut pin = VirtualPin::new(false);
    pin.toggle();
    assert!(pin.state);
    pin.toggle();
    assert!(!pin.state);
}

#[test]
fn virtual_pin_input() {
    let pin = VirtualPin::new(true);
    assert!(pin.is_high());
    assert!(!pin.is_low());

    let pin = VirtualPin::new(false);
    assert!(!pin.is_high());
    assert!(pin.is_low());
}

// ---------------------------------------------------------------------------
// MockTimer
// ---------------------------------------------------------------------------

#[test]
fn mock_timer_starts_at_zero() {
    let timer = MockTimer::new();
    assert_eq!(timer.elapsed_ms, 0);
}

#[test]
fn mock_timer_accumulates() {
    let mut timer = MockTimer::new();
    timer.delay_ms(100);
    timer.delay_ms(200);
    assert_eq!(timer.elapsed_ms, 300);
}

// ---------------------------------------------------------------------------
// blink_led
// ---------------------------------------------------------------------------

#[test]
fn blink_led_three_times() {
    let mut pin = VirtualPin::new(false);
    let mut timer = MockTimer::new();

    blink_led(&mut pin, &mut timer, 3);

    assert!(pin.is_low());
    assert_eq!(timer.elapsed_ms, 3000);
}

#[test]
fn blink_led_zero_times() {
    let mut pin = VirtualPin::new(false);
    let mut timer = MockTimer::new();

    blink_led(&mut pin, &mut timer, 0);

    assert!(pin.is_low());
    assert_eq!(timer.elapsed_ms, 0);
}
