//! Module 087: Embedded Rust Revisited — exercise scaffold.
//!
//! Building on Module 059's embedded primer, this module implements a
//! simulated embedded device with register-level I/O, interrupt flags,
//! interrupt enable masks, and a timer peripheral — all in pure `std`,
//! testable with `cargo test` on the host.
//!
//! Fill in every `// TODO(module-087)` below.

/// Interrupt request number for the timer peripheral.
pub const TIMER_IRQ: u8 = 0;
/// I/O registers.
pub const STATUS_REG: u8 = 0x00;
pub const CONFIG_REG: u8 = 0x01;
pub const LED_REG: u8 = 0x02;
pub const COUNTER_LOW: u8 = 0x10;
pub const COUNTER_HIGH: u8 = 0x11;

/// A simulated embedded device with 256 registers, an interrupt controller,
/// and a timer.
#[derive(Debug)]
pub struct EmbeddedDevice {
    pub registers: [u8; 256],
    pub interrupt_flags: u16,
    pub enabled_interrupts: u16,
    pub timer_counter: u32,
}

impl EmbeddedDevice {
    /// Creates a new device with all registers, flags, and counters zeroed.
    pub fn new() -> Self {
        // TODO(module-087): initialize all fields to zero/default.
        panic!("TODO(module-087): implement EmbeddedDevice::new");
    }

    /// Writes the 8-bit `value` to the register at `addr`.
    ///
    /// Register 0x02 (LED_REG) is special: only the lowest 4 bits are
    /// writable; bits 4-7 must be ignored (mask the value with 0x0F).
    pub fn write_register(&mut self, addr: u8, value: u8) {
        // TODO(module-087): store `value` into `self.registers[addr as usize]`,
        // with the LED_REG masking rule applied.
        let _ = (addr, value);
        panic!("TODO(module-087): implement EmbeddedDevice::write_register");
    }

    /// Reads the 8-bit value from the register at `addr`.
    pub fn read_register(&self, addr: u8) -> u8 {
        // TODO(module-087): return `self.registers[addr as usize]`.
        let _ = addr;
        panic!("TODO(module-087): implement EmbeddedDevice::read_register");
    }

    /// Enables the interrupt with the given IRQ number (0-15).
    ///
    /// Set the corresponding bit in `self.enabled_interrupts`.
    pub fn enable_interrupt(&mut self, irq: u8) {
        // TODO(module-087): set bit `irq` in `self.enabled_interrupts`.
        let _ = irq;
        panic!("TODO(module-087): implement EmbeddedDevice::enable_interrupt");
    }

    /// Disables the interrupt with the given IRQ number.
    pub fn disable_interrupt(&mut self, irq: u8) {
        // TODO(module-087): clear bit `irq` in `self.enabled_interrupts`.
        let _ = irq;
        panic!("TODO(module-087): implement EmbeddedDevice::disable_interrupt");
    }

    /// Checks whether a specific interrupt is enabled.
    pub fn is_interrupt_enabled(&self, irq: u8) -> bool {
        // TODO(module-087): check bit `irq` in `self.enabled_interrupts`.
        let _ = irq;
        panic!("TODO(module-087): implement EmbeddedDevice::is_interrupt_enabled");
    }

    /// Manually triggers an interrupt — sets the corresponding flag bit.
    ///
    /// A real device would do this in hardware when a peripheral needs
    /// attention; here we simulate that.
    pub fn trigger_interrupt(&mut self, irq: u8) {
        // TODO(module-087): set bit `irq` in `self.interrupt_flags`.
        let _ = irq;
        panic!("TODO(module-087): implement EmbeddedDevice::trigger_interrupt");
    }

    /// Returns the bitmask of currently pending (flagged) interrupts.
    pub fn get_pending_interrupts(&self) -> u16 {
        // TODO(module-087): return `self.interrupt_flags`.
        panic!("TODO(module-087): implement EmbeddedDevice::get_pending_interrupts");
    }

    /// Handles the highest-priority pending-and-enabled interrupt.
    ///
    /// Returns `Some(irq)` if an interrupt was handled, or `None` if no
    /// pending-and-enabled interrupts exist. Checks IRQs from 0 (highest
    /// priority) to 15 (lowest). A handled interrupt must have its flag
    /// cleared *after* processing.
    ///
    /// For TIMER_IRQ: increment `self.timer_counter` before clearing.
    pub fn handle_interrupt(&mut self) -> Option<u8> {
        // TODO(module-087): scan IRQ 0..=15 for a pending-and-enabled bit.
        // If found, process it:
        //   - TIMER_IRQ: increment `self.timer_counter`.
        //   - Other IRQs: update STATUS_REG to record which IRQ fired
        //     (write the IRQ number into STATUS_REG).
        // Clear the flag, then return `Some(irq)`.
        // Return `None` if nothing is pending and enabled.
        panic!("TODO(module-087): implement EmbeddedDevice::handle_interrupt");
    }

    /// Simulates a timer tick: sets the timer interrupt flag.
    ///
    /// Call this once per simulated timer cycle. If the timer interrupt
    /// is enabled, it becomes pending.
    pub fn timer_tick(&mut self) {
        // TODO(module-087): call `self.trigger_interrupt(TIMER_IRQ)`.
        panic!("TODO(module-087): implement EmbeddedDevice::timer_tick");
    }
}

impl Default for EmbeddedDevice {
    fn default() -> Self {
        Self::new()
    }
}
