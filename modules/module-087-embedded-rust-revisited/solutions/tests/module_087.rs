use module_087_solutions::{
    EmbeddedDevice, CONFIG_REG, COUNTER_HIGH, COUNTER_LOW, LED_REG, STATUS_REG, TIMER_IRQ,
};

// --- Basic register I/O ----------------------------------------------------

#[test]
fn write_and_read_register() {
    let mut device = EmbeddedDevice::new();
    device.write_register(0x05, 0xAB);
    assert_eq!(device.read_register(0x05), 0xAB);
}

#[test]
fn write_read_multiple_registers() {
    let mut device = EmbeddedDevice::new();
    device.write_register(0x00, 0xFF);
    device.write_register(0x01, 0x80);
    device.write_register(0xFF, 0x01);
    assert_eq!(device.read_register(0x00), 0xFF);
    assert_eq!(device.read_register(0x01), 0x80);
    assert_eq!(device.read_register(0xFF), 0x01);
}

#[test]
fn led_register_masks_high_nibble() {
    let mut device = EmbeddedDevice::new();
    device.write_register(LED_REG, 0xFA);
    assert_eq!(device.read_register(LED_REG), 0x0A);
    device.write_register(LED_REG, 0x3F);
    assert_eq!(device.read_register(LED_REG), 0x0F);
    device.write_register(LED_REG, 0x00);
    assert_eq!(device.read_register(LED_REG), 0x00);
}

#[test]
fn other_registers_are_not_masked() {
    let mut device = EmbeddedDevice::new();
    device.write_register(CONFIG_REG, 0xF0);
    assert_eq!(device.read_register(CONFIG_REG), 0xF0);
}

// --- Interrupt enable/disable ----------------------------------------------

#[test]
fn enable_and_check_interrupt() {
    let mut device = EmbeddedDevice::new();
    assert!(!device.is_interrupt_enabled(3));
    device.enable_interrupt(3);
    assert!(device.is_interrupt_enabled(3));
}

#[test]
fn disable_clears_enabled_bit() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(7);
    assert!(device.is_interrupt_enabled(7));
    device.disable_interrupt(7);
    assert!(!device.is_interrupt_enabled(7));
}

#[test]
fn multiple_interrupts_independently_enabled() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(0);
    device.enable_interrupt(10);
    device.disable_interrupt(10);
    assert!(device.is_interrupt_enabled(0));
    assert!(!device.is_interrupt_enabled(10));
    assert!(!device.is_interrupt_enabled(5));
}

// --- Interrupt triggering and flags ----------------------------------------

#[test]
fn trigger_sets_flag() {
    let mut device = EmbeddedDevice::new();
    device.trigger_interrupt(2);
    assert_eq!(device.get_pending_interrupts(), 1 << 2);
}

#[test]
fn multiple_triggers_accumulate() {
    let mut device = EmbeddedDevice::new();
    device.trigger_interrupt(0);
    device.trigger_interrupt(5);
    device.trigger_interrupt(15);
    let pending = device.get_pending_interrupts();
    assert_eq!(pending & (1 << 0), 1 << 0);
    assert_eq!(pending & (1 << 5), 1 << 5);
    assert_eq!(pending & (1 << 15), 1 << 15);
}

// --- Interrupt handling ----------------------------------------------------

#[test]
fn handle_interrupt_clears_flag_and_returns_irq() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(4);
    device.trigger_interrupt(4);

    let result = device.handle_interrupt();
    assert_eq!(result, Some(4));
    assert_eq!(device.get_pending_interrupts(), 0, "flag must be cleared");
}

#[test]
fn handle_interrupt_ignores_disabled() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(1);
    device.trigger_interrupt(1);
    device.trigger_interrupt(2); // not enabled

    let result = device.handle_interrupt();
    assert_eq!(result, Some(1));
    assert_eq!(
        device.get_pending_interrupts(),
        1 << 2,
        "disabled IRQ stays pending"
    );
}

#[test]
fn handle_interrupt_returns_none_when_nothing_pending() {
    let mut device = EmbeddedDevice::new();
    assert_eq!(device.handle_interrupt(), None);
}

#[test]
fn handle_interrupt_returns_none_when_only_disabled_pending() {
    let mut device = EmbeddedDevice::new();
    device.trigger_interrupt(3); // not enabled
    assert_eq!(device.handle_interrupt(), None);
}

#[test]
fn highest_priority_handled_first() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(0);
    device.enable_interrupt(1);
    device.trigger_interrupt(0);
    device.trigger_interrupt(1);

    let result = device.handle_interrupt();
    assert_eq!(result, Some(0), "IRQ 0 has higher priority than IRQ 1");
}

#[test]
fn non_timer_interrupt_updates_status_register() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(3);
    device.trigger_interrupt(3);

    device.handle_interrupt();
    assert_eq!(
        device.read_register(STATUS_REG),
        3,
        "STATUS should record the handled IRQ"
    );
}

// --- Timer interrupt --------------------------------------------------------

#[test]
fn timer_tick_sets_flag_and_increment_on_handle() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(TIMER_IRQ);
    assert_eq!(device.timer_counter, 0);

    device.timer_tick();
    assert_ne!(device.get_pending_interrupts() & (1 << TIMER_IRQ), 0);

    let handled = device.handle_interrupt();
    assert_eq!(handled, Some(TIMER_IRQ));
    assert_eq!(device.timer_counter, 1);
    assert_eq!(device.get_pending_interrupts() & (1 << TIMER_IRQ), 0);
}

#[test]
fn timer_counter_accumulates_over_ticks() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(TIMER_IRQ);

    for _ in 0..5 {
        device.timer_tick();
        assert_ne!(device.get_pending_interrupts() & (1 << TIMER_IRQ), 0);
        device.handle_interrupt();
    }
    assert_eq!(device.timer_counter, 5);
}

#[test]
fn timer_tick_without_enable_leaves_flag_but_handler_ignores() {
    let mut device = EmbeddedDevice::new();
    device.timer_tick();
    assert_ne!(device.get_pending_interrupts() & (1 << TIMER_IRQ), 0);
    let result = device.handle_interrupt();
    assert_eq!(
        result, None,
        "timer IRQ is not enabled, so handler skips it"
    );
    assert_eq!(device.timer_counter, 0);
}

// --- Counter registers ------------------------------------------------------

#[test]
fn counter_registers_reflect_timer_counter() {
    let mut device = EmbeddedDevice::new();
    device.enable_interrupt(TIMER_IRQ);
    for _ in 0..0x0101 {
        device.timer_tick();
        device.handle_interrupt();
    }
    assert_eq!(device.timer_counter, 0x0101);
    assert_eq!(device.read_register(COUNTER_LOW), 0x01);
    assert_eq!(device.read_register(COUNTER_HIGH), 0x01);
}

// --- Default implementation -------------------------------------------------

#[test]
fn device_default_is_zeroed() {
    let device = EmbeddedDevice::default();
    for i in 0..=255u8 {
        assert_eq!(device.read_register(i), 0, "register {i} should be 0");
    }
    assert_eq!(device.get_pending_interrupts(), 0);
    assert!(!device.is_interrupt_enabled(0));
}
