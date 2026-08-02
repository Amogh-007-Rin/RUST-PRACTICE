# Module 058: Introduction to Embedded Rust

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 45–75 min
**Prerequisites:** Module 016 (Traits I), Module 017 (Traits II), Module 025 (Advanced Traits)

## Learning Objectives

- You will be able to explain what `#![no_std]` means and when embedded Rust requires it.
- You will be able to define and implement Hardware Abstraction Layer (HAL) traits: `GpioOutput`, `GpioInput`, and `DelayMs`.
- You will be able to implement a mock hardware layer for testing on a host machine.
- You will be able to write generic functions over HAL traits that compile for both real hardware and test mocks.

## Why This Matters

Rust's embedded ecosystem is one of the language's fastest-growing domains. Microcontrollers (MCUs) like ARM Cortex-M, RISC-V, and AVR run Rust without an operating system — no heap, no `std`, no `println!`. Instead, Rust's trait system provides the Hardware Abstraction Layer (HAL) pattern: a set of traits (`GpioOutput`, `SpiBus`, `DelayMs`) that chip vendors implement for their hardware. Your application code targets the **trait**, not the chip. This means you can test your blinking-LED logic on your laptop by swapping in a mock implementation, then deploy the exact same code to a Cortex-M board. This module introduces the HAL pattern with a simplified mock setup — the mental model you'll need for real `embedded-hal` and `embassy` work.

## Concept

### `#![no_std]` — the crate attribute

A standard Rust binary links against `std`, which assumes an operating system with files, threads, networking, and a heap allocator. Embedded devices (and kernel modules, and some WASM targets) have none of these. Adding `#![no_std]` at the top of `lib.rs` or `main.rs` tells the compiler:

- Don't link `std`.
- Use `core` instead (a minimal subset: basic types, `Option`, `Result`, iterators, `Future`, etc.).
- You lose `println!`, `Vec`, `Box`, `HashMap`, `fs`, `net`, and threads.

You can bring back `Vec` and `Box` by providing an allocator (using the `alloc` crate), but many embedded programs use only stack-allocated data.

For this module, we stay in a `std` environment to keep testing simple. But the trait pattern we build mirrors `embedded-hal` exactly — the same code would compile under `#![no_std]` if we removed `std`-only features.

### The HAL pattern

"Hardware Abstraction Layer" means: define a trait that describes what the hardware can do, and let implementations provide the how.

```rust
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
```

`GpioOutput` represents a pin that can drive a high or low voltage. `GpioInput` represents a pin that can be read. `DelayMs` represents a timer that can block for milliseconds.

A real embedded chip (e.g., STM32 or nRF52) has a Peripheral Access Crate (PAC) that gives raw register access. The HAL crate implements these traits for the PAC types. Your application never touches raw registers directly.

### Mock hardware for testing

To test on a host machine, implement the traits with plain Rust structs:

```rust
pub struct VirtualPin {
    pub state: bool,
}

impl GpioOutput for VirtualPin {
    fn set_high(&mut self) { self.state = true; }
    fn set_low(&mut self)  { self.state = false; }
}

impl GpioInput for VirtualPin {
    fn is_high(&self) -> bool { self.state }
    fn is_low(&self)  -> bool { !self.state }
}
```

`VirtualPin` tracks a `bool` in memory. `set_high()` sets it; `is_high()` reads it. No hardware involved — but the trait interface is identical to a real GPIO pin.

### Generic functions over HAL traits

The magic of the HAL pattern is that application logic becomes generic:

```rust
pub fn blink_led(
    pin: &mut impl GpioOutput,
    delay: &mut impl DelayMs,
    times: u32,
) {
    for _ in 0..times {
        pin.set_high();
        delay.delay_ms(500);
        pin.set_low();
        delay.delay_ms(500);
    }
}
```

This function works with `VirtualPin` (for testing) or a real `stm32f4xx_hal::gpio::OutputPin` (on hardware). The compiler monomorphizes it for each concrete type, producing separate machine code — zero runtime overhead from the trait abstraction.

### A mock timer

```rust
pub struct MockTimer {
    pub elapsed_ms: u32,
}

impl DelayMs for MockTimer {
    fn delay_ms(&mut self, ms: u32) {
        self.elapsed_ms += ms;
        // In real hardware this would busy-wait or set a timer interrupt.
    }
}
```

In tests, we can assert that `elapsed_ms` matches expectations (e.g., "blink 3 times should accumulate 3 * 1000 ms").

### The full embedded stack

A real embedded Rust project uses this layered architecture:

```
┌──────────────────────────────────────┐
│  Application code                    │  ← your business logic
│  (generic over HAL traits)           │
├──────────────────────────────────────┤
│  HAL crate (implements traits for    │  ← e.g., stm32f4xx-hal
│  the chip's peripherals)             │
├──────────────────────────────────────┤
│  PAC crate (unsafe register access)  │  ← e.g., stm32f4
├──────────────────────────────────────┤
│  Microcontroller hardware            │  ← silicon
└──────────────────────────────────────┘
```

The HAL layer is the key abstraction boundary. It's what lets `embassy` (an async embedded runtime) and `embedded-hal` crates work across dozens of different chip families.

### The trait-or-enum choice

Embedded Rust has two schools of thought:

1. **Trait-based HAL** (`embedded-hal`): traits like `GpioOutput` + concrete impls. Zero-cost, fully generic, compile-time dispatch. The ecosystem standard.
2. **Enum-based pin abstraction**: pins as `enum PinState { High, Low }`. Simpler but involves runtime checks and doesn't scale well across chip families.

This module follows the trait approach because it's what `embedded-hal` uses and what the industry has converged on.

### ASCII diagram: blink_led flow

```
blink_led(pin, delay, times=2)
│
├─ iteration 0:
│   pin.set_high()      ──►  VirtualPin { state: true }
│   delay.delay_ms(500) ──►  MockTimer { elapsed_ms += 500 }
│   pin.set_low()       ──►  VirtualPin { state: false }
│   delay.delay_ms(500) ──►  MockTimer { elapsed_ms += 500 }
│
├─ iteration 1:
│   pin.set_high()
│   delay.delay_ms(500)
│   pin.set_low()
│   delay.delay_ms(500)
│
└─ done: MockTimer.elapsed_ms == 2000
```

### Toggle pattern

A common GPIO operation is toggling — flipping the pin's state without knowing its current value:

```rust
fn toggle(pin: &mut impl GpioOutput) {
    // This is why GpioInput + GpioOutput are separate traits:
    // to toggle, we need to read current state.
}
```

The exercise includes a `toggle()` method on `VirtualPin` that reads its internal state and flips it. On real hardware, some chips have dedicated toggle registers; on others, you read-modify-write. The trait pattern hides this detail.

## Common Pitfalls

- **Forgetting that `no_std` removes `println!` and `Vec`.** Use `defmt` or `rtt-target` for logging, and stack arrays or `heapless` for collections.
- **Confusing `GpioInput` and `GpioOutput`.** A pin configured for output cannot read its state (on most hardware) without reconfiguration.
- **Busy-waiting with `delay_ms`.** In real embedded code, busy-waits burn power. Use timer interrupts or `embassy`'s async delay. Our mock version is fine for testing.
- **Mock drift.** If your mock behaves differently from real hardware (e.g., a pin that reads back the output value vs one that doesn't), tests pass but hardware fails. Keep mocks faithful to the datasheet.

## Key Terms

- **`#![no_std]`:** a crate attribute that removes the standard library, leaving only `core`.
- **HAL (Hardware Abstraction Layer):** a set of traits that abstract hardware peripherals so application code is chip-agnostic.
- **PAC (Peripheral Access Crate):** unsafe, auto-generated code providing raw register access for a specific chip.
- **GPIO (General Purpose Input/Output):** a digital pin that can be configured as input or output.
- **`embedded-hal`:** the community-standard crate defining HAL traits for Rust embedded development.
- **Mock:** a software-only implementation of a HAL trait used for testing on a host machine.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-058)` comments.
2. Define the `GpioOutput` trait with `set_high()` and `set_low()`.
3. Define the `GpioInput` trait with `is_high()` and `is_low()`.
4. Define the `DelayMs` trait with `delay_ms()`.
5. Implement `VirtualPin` with `new()`, `toggle()`, `set_high()`, `set_low()`.
6. Implement `GpioOutput` and `GpioInput` for `VirtualPin`.
7. Implement `MockTimer` with `new()` and `DelayMs`.
8. Implement `blink_led()` — a generic function that blinks a pin `times` times with equal on/off delays.
9. Run `cargo test -p module-058-exercises` until all tests pass.
10. Compare with `solutions/` afterwards.

## Further Reading

- [The Embedded Rust Book](https://docs.rust-embedded.org/book/) — the official guide to embedded Rust.
- [`embedded-hal` crate](https://crates.io/crates/embedded-hal) — the community-standard HAL traits.
- [The Discovery Book](https://docs.rust-embedded.org/discovery/) — hands-on embedded Rust with a real microcontroller board.
- [`defmt` logging framework](https://defmt.ferrous-systems.com/) — `println!`-like logging for `no_std` environments.
