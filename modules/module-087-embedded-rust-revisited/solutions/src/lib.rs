//! Module 087: Embedded Rust Revisited — reference solution.

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
        Self {
            registers: [0; 256],
            interrupt_flags: 0,
            enabled_interrupts: 0,
            timer_counter: 0,
        }
    }

    /// Writes the 8-bit `value` to the register at `addr`.
    ///
    /// Register 0x02 (LED_REG) is special: only the lowest 4 bits are
    /// writable; bits 4-7 must be ignored.
    pub fn write_register(&mut self, addr: u8, value: u8) {
        let masked = if addr == LED_REG { value & 0x0F } else { value };
        self.registers[addr as usize] = masked;
    }

    /// Reads the 8-bit value from the register at `addr`.
    ///
    /// Counter registers (COUNTER_LOW and COUNTER_HIGH) read from the live
    /// `timer_counter` field instead of the register array.
    pub fn read_register(&self, addr: u8) -> u8 {
        match addr {
            COUNTER_LOW => (self.timer_counter & 0xFF) as u8,
            COUNTER_HIGH => ((self.timer_counter >> 8) & 0xFF) as u8,
            _ => self.registers[addr as usize],
        }
    }

    /// Enables the interrupt with the given IRQ number (0-15).
    pub fn enable_interrupt(&mut self, irq: u8) {
        self.enabled_interrupts |= 1 << irq;
    }

    /// Disables the interrupt with the given IRQ number.
    pub fn disable_interrupt(&mut self, irq: u8) {
        self.enabled_interrupts &= !(1 << irq);
    }

    /// Checks whether a specific interrupt is enabled.
    pub fn is_interrupt_enabled(&self, irq: u8) -> bool {
        (self.enabled_interrupts & (1 << irq)) != 0
    }

    /// Manually triggers an interrupt — sets the corresponding flag bit.
    pub fn trigger_interrupt(&mut self, irq: u8) {
        self.interrupt_flags |= 1 << irq;
    }

    /// Returns the bitmask of currently pending (flagged) interrupts.
    pub fn get_pending_interrupts(&self) -> u16 {
        self.interrupt_flags
    }

    /// Handles the highest-priority pending-and-enabled interrupt.
    ///
    /// Returns `Some(irq)` if an interrupt was handled, or `None`. Checks
    /// IRQs from 0 (highest priority) to 15 (lowest). For TIMER_IRQ,
    /// increments `timer_counter`. For other IRQs, records the IRQ number
    /// in STATUS_REG.
    pub fn handle_interrupt(&mut self) -> Option<u8> {
        for irq in 0u8..=15 {
            let mask = 1u16 << irq;
            if (self.interrupt_flags & mask) != 0 && (self.enabled_interrupts & mask) != 0 {
                if irq == TIMER_IRQ {
                    self.timer_counter = self.timer_counter.wrapping_add(1);
                } else {
                    self.registers[STATUS_REG as usize] = irq;
                }
                self.interrupt_flags &= !mask;
                return Some(irq);
            }
        }
        None
    }

    /// Simulates a timer tick: sets the timer interrupt flag.
    pub fn timer_tick(&mut self) {
        self.trigger_interrupt(TIMER_IRQ);
    }
}

impl Default for EmbeddedDevice {
    fn default() -> Self {
        Self::new()
    }
}
