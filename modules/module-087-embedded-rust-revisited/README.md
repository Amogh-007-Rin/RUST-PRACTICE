# Module 087: Embedded Rust Revisited

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 90–120 min
**Prerequisites:** Module 059 (Embedded Rust Hands-On). Modules 001–080.

## Learning Objectives

- You will be able to implement a simulated embedded device with register-level I/O, interrupt flags, and an interrupt enable mask.
- You will be able to design an interrupt handler that dispatches by priority, processes each IRQ with its own logic, and clears flags after handling.
- You will be able to implement a timer peripheral that triggers on ticks and increments a counter.
- You will be able to explain the real-hardware mapping: registers to MMIO addresses, interrupt flags to NVIC bits, and timer ticks to hardware timer interrupts.

## Why This Matters

Module 059 gave you the `#![no_std]` basics and a simulated LED blinker. Real embedded systems go further: they juggle multiple peripherals (timers, UART, I2C, SPI, GPIO banks) each firing interrupts at unpredictable times, and the firmware must prioritize, handle, and clear those interrupts without missing ticks or corrupting shared state. This module implements the interrupt controller pattern that sits at the heart of every microcontroller firmware — from STM32 HALs to Embassy's async executors. The patterns you build here (register masking, flag polling, priority dispatch) are exactly what you'll write against real hardware registers, just with `unsafe` `*mut u8` instead of a `[u8; 256]` array.

## Concept

### The register model

Every microcontroller peripheral — GPIO, timer, UART, ADC — is exposed to software through a block of memory-mapped registers. To the CPU, these are just bytes at specific addresses. Writing to a "data output" register sets an LED state; reading a "status" register returns a sensor value; writing to an "interrupt enable" register configures which events wake the CPU.

In the simulated device, registers are a `[u8; 256]` array indexed by address:

```rust
let mut device = EmbeddedDevice::new();
device.write_register(LED_REG, 0b0000_0101);  // turn on bits 0 and 2
let status = device.read_register(STATUS_REG); // read the status byte
```

The `LED_REG` (address 0x02) is special: only the low 4 bits are writable — the hardware ignores bits 4-7. This pattern is realistic: many real MCU registers have reserved or read-only bits that writes must mask out or ignore. The exercise enforces this by explicitly masking:

```rust
let masked_value = value & 0x0F; // drop upper nibble
self.registers[LED_REG as usize] = masked_value;
```

### Interrupts: a finite-state machine

An interrupt controller in hardware typically has two per-interrupt bits:

- **Flag (IF)** — set by hardware when the interrupt condition occurs (a timer overflowed, a byte arrived on UART). The MCU reads this to know *what happened*.
- **Enable (IE)** — set by firmware to tell the controller "wake me for this." If the flag is set but the enable is clear, the interrupt is pending but won't fire.

This is modeled as two `u16` bitfields — one for all 16 interrupt flags, one for all 16 enable bits:

```
interrupt_flags:    0b0000_0000_0010_0100   ← IRQs 2 and 5 have fired
enabled_interrupts: 0b0000_0000_0010_0000   ← only IRQ 5 is enabled
effective:          0b0000_0000_0010_0000   ← flags & enable = IRQ 5 only
```

The operation of each is a single bit-manipulation:

```rust
// Setting a flag (hardware side)
self.interrupt_flags |= 1 << irq;

// Enabling an interrupt (firmware side)
self.enabled_interrupts |= 1 << irq;

// Checking if an interrupt should fire
let mask = 1 << irq;
let is_pending_and_enabled = (self.interrupt_flags & mask) != 0
                          && (self.enabled_interrupts & mask) != 0;
```

### Priority-based interrupt handling

When multiple interrupts are pending and enabled, the handler must choose one. The convention used in ARM Cortex-M NVIC and most MCUs is: numerically lower IRQ numbers = higher priority. The handler scans from IRQ 0 upward, handling the first one it finds:

```
for irq in 0..=15 {
    if is_pending_and_enabled(irq) {
        process_irq(irq);   // IRQ-specific logic
        clear_flag(irq);    // prevent re-handling
        return Some(irq);
    }
}
None // nothing to handle
```

Each IRQ triggers device-specific processing. The timer IRQ increments a counter. Other IRQs record which IRQ fired into the STATUS_REG for the firmware to inspect. On real hardware, handlers would be more complex: UART IRQs drain receive FIFOs, GPIO IRQs read pin states, timer IRQs toggle outputs.

### The timer peripheral

A timer is a counter that increments on each tick of a clock. When it overflows or reaches a match value, it sets its interrupt flag. In the exercise:

```rust
fn timer_tick(&mut self) {
    self.trigger_interrupt(TIMER_IRQ);  // set the flag
}
```

The handler increments the timer counter:

```rust
// Inside handle_interrupt, when irq == TIMER_IRQ:
self.timer_counter = self.timer_counter.wrapping_add(1);
self.interrupt_flags &= !(1 << TIMER_IRQ);  // clear flag
```

After 0x0101 ticks have been handled, the timer counter reaches 0x0101, which reads back as two bytes from the COUNTER_LOW (0x10) and COUNTER_HIGH (0x11) registers. This two-register split is standard practice: 8-bit MCUs expose 16-bit or 32-bit counters as adjacent register pairs that must be read in the right order (low byte first to latch the high byte for consistency).

### The full interrupt lifecycle

```
1. timer_tick() → sets interrupt_flags bit 0
2. handle_interrupt() scans 0..=15
3. Finds IRQ 0 pending AND enabled
4. Increments timer_counter
5. Clears interrupt_flags bit 0
6. Returns Some(0)
```

If the timer is not enabled but the flag is set:

```
1. timer_tick() → sets interrupt_flags bit 0
2. handle_interrupt() scans — IRQ 0 is pending but NOT enabled
3. Skips it, returns None
4. timer_counter = 0 (unchanged), flag stays set
```

This models a real scenario: a device interrupt fires between the firmware disabling it and the flag being cleared, leaving a "stale" pending flag that the next enable + handle call will process.

### What this looks like on real hardware

In an STM32 chip, the timer would be something like:

```rust,ignore
// Real STM32 timer register block
const TIM2: *const TimerRegisters = 0x4000_0000 as *const TimerRegisters;

// The "timer tick" is hardware — you configure the timer, start it,
// and it fires interrupts automatically.
// What you write is the interrupt handler:
#[interrupt]
fn TIM2() {
    // Read the status register to confirm the source
    if (*TIM2).sr & (1 << 0) != 0 {
        // Clear the flag (write 0 to clear — common pattern)
        (*TIM2).sr &= !(1 << 0);
        // Do the timer work
        tick_count += 1;
    }
}
```

The simulated model replaces `unsafe` raw pointer access with a safe array. The control flow — check flags, process, clear — is identical.

## Common Pitfalls

- **Forgetting to mask reserved bits in hardware registers.** Writing a 1 to a read-only or reserved bit can lock up a peripheral or put it in an undefined state. The LED_REG in this exercise enforces masking as a habit.
- **Clearing flags after processing, not before.** If you clear the flag too early, a second interrupt arriving between clearing and processing may be lost.
- **Missing the `wrapping_add` on the timer counter.** Embedded counters overflow naturally (a 32-bit counter is fine — it wraps around), and `+` in debug mode panics on overflow. Use `wrapping_add()` for anything that should literally wrap.
- **Handling an interrupt without checking the enable mask.** The flag can be set but the interrupt should not fire — your handler must check both `interrupt_flags & mask` AND `enabled_interrupts & mask`.

## Key Terms

- **Register:** a memory-mapped byte or word that controls or reports the state of a peripheral. In the simulation, a slot in `[u8; 256]`.
- **Interrupt flag (IF):** a bit set by hardware when an event occurs. Software reads it, handles the event, and clears it.
- **Interrupt enable (IE):** a bit set by firmware to allow a specific interrupt to fire.
- **Pending interrupt:** the condition `flag == 1 && enable == 1`. A flagged but disabled interrupt is not pending.
- **Priority:** the order in which interrupts are serviced when multiple are pending. Lower IRQ number = higher priority.
- **Timer counter:** a register that increments on each timer tick, typically used for generating periodic interrupts.
- **NVIC:** Nested Vectored Interrupt Controller — the ARM hardware block that manages interrupt flags, enables, and priorities. Our simulation is a simplified model of one.
- **MMIO:** Memory-Mapped I/O — accessing hardware registers through normal load/store instructions at specific memory addresses.

## Exercise

In `exercises/src/lib.rs` you implement a simulated embedded device. The scaffold provides the `EmbeddedDevice` struct, constants for registers and IRQ numbers. Fill in the `// TODO(module-087)` stubs:

1. **`EmbeddedDevice::new`** — zero-initialize all fields.
2. **`write_register`** / **`read_register`** — basic register I/O; `write_register` must mask the LED_REG value to 0x0F.
3. **`enable_interrupt`** / **`disable_interrupt`** / **`is_interrupt_enabled`** — bit manipulation on the `enabled_interrupts` u16 bitmask.
4. **`trigger_interrupt`** / **`get_pending_interrupts`** — flag management on `interrupt_flags`.
5. **`handle_interrupt`** — scan IRQ 0..=15 for a pending-and-enabled bit. For TIMER_IRQ: increment `timer_counter`. For other IRQs: write the IRQ number to STATUS_REG. Clear the flag after processing. Return `Some(irq)` or `None`.
6. **`timer_tick`** — call `trigger_interrupt(TIMER_IRQ)`.
7. **Counter registers** — the solution adds a special case to `read_register` for COUNTER_LOW/HIGH that reads from the live `timer_counter` field. The exercise scaffold only requires the basic read — the counter register test expects this behavior.

The integration tests in `tests/module_087.rs` cover register writes/reads (including LED nibble masking), interrupt enable/disable, flag setting/clearing, single and multiple interrupt handling, priority ordering, timer ticks and counter accumulation, and the counter register split.

## Further Reading

- [The Embedded Rust Book — "Concurrency" chapter](https://docs.rust-embedded.org/book/concurrency/) — real interrupt handling with the Cortex-M `#[interrupt]` attribute and critical sections.
- [STM32F4xx Reference Manual — "Nested Vectored Interrupt Controller" section](https://www.st.com/resource/en/reference_manual/dm00031020-stm32f405-415-stm32f407-417-stm32f427-437-and-stm32f429-439-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf) — the actual hardware block this module models.
- [`embedded-hal` crate](https://docs.rs/embedded-hal/latest/embedded_hal/) — the trait-based HAL that Rust embedded drivers use for portability across MCUs.
- [Module 059 — Embedded Rust Hands-On](../module-059-embedded-rust-hands-on/README.md) — the `#![no_std]` primer this module builds on.
