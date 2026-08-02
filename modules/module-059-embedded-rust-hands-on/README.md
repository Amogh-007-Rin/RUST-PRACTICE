# Module 059: Embedded Rust Hands-On

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 60–90 min
**Prerequisites:** Module 058 (Introduction to Embedded Rust)

## Learning Objectives

- You will be able to design a simulated microcontroller with analog and digital I/O pins.
- You will be able to implement a timer interrupt handler that updates a clock counter.
- You will be able to read simulated sensor values through a HAL-like API.
- You will be able to implement a button debouncing state machine using timer ticks.

## Why This Matters

Real embedded programming involves subtle hardware behaviors that are expensive to debug on physical boards: interrupt timing, peripheral state machines, and signal debouncing. Building a simulated microcontroller lets you develop and test logic on your laptop before flashing to hardware. This module extends the HAL pattern from Module 058 with a simulated MCU that has analog sensors (for reading temperature, light, etc.), timer interrupts, and a button debouncer — the three most common embedded patterns you'll encounter in real firmware. The debouncer state machine in particular is a rite of passage for embedded developers: mechanical switches "bounce" (make/break contact rapidly) for milliseconds after a press, and your firmware must filter this noise.

## Concept

### The simulated MCU

We model a simple microcontroller with:

- **Analog pins** (ADC): an array of 16-bit values representing sensor readings (e.g., temperature = 0–4095 on a 12-bit ADC).
- **Digital pins**: an array of booleans representing GPIO states (on/off).
- **Timer counter**: a monotonically incrementing 64-bit counter that advances on each timer interrupt.

```rust
pub struct SimulatedMcu {
    pub analog_pins: [u16; 4],
    pub digital_pins: [bool; 4],
    pub timer_counter: u64,
}
```

On real hardware, the timer peripheral fires an interrupt at a fixed frequency (e.g., 1 kHz). Each interrupt calls an interrupt service routine (ISR) that increments the counter and checks for pending work. In our simulation, `process_timer_interrupt()` models this ISR:

```rust
pub fn process_timer_interrupt(mcu: &mut SimulatedMcu) {
    mcu.timer_counter += 1;
}
```

### Reading a sensor

Analog sensors return a voltage the ADC converts to a digital value. Our simulated `read_sensor` simply reads from the pin array:

```rust
pub fn read_sensor(mcu: &SimulatedMcu, pin_id: u8) -> Option<u16> {
    mcu.analog_pins.get(pin_id as usize).copied()
}
```

Returns `None` if the pin ID is out of range. The `Option` pattern mirrors real HAL APIs where you get a `Result` if the pin isn't configured for analog input.

### Button debouncing

When you press a mechanical button, the metal contacts don't make a clean connection immediately — they bounce, producing a noisy signal:

```
Ideal press:        ────┐       ┌────
                        └───────┘

Actual signal:      ────┐┌┐┌┐┌┐┌─┐───
                        └┘└┘└┘└┘ └─
                        ◄─ bounce ─►
                           period
```

If your code reacts to every edge, one physical press registers as multiple presses. A debouncer filters this by requiring the signal to be **stable** for a minimum duration before accepting the state change.

### Debouncer state machine

The debouncer has four states:

```
                 ┌─────────┐
   button down   │  Idle   │  button up
    ┌───────────►│         │◄───────────┐
    │            └────┬────┘            │
    │                 │ button down     │
    │            ┌────▼────┐            │
    │  bounce    │Pressing │  stable    │
    │  detected  │         │  for N     │
    │ ◄──────────│         ├──────────► │
    │            └─────────┘            │
    │                                  │
    │            ┌─────────┐            │
    │   stable   │ Pressed │  bounce    │
    │   for N    │         │  detected  │
    │ ◄──────────┤         ├──────────► │
    │            └────┬────┘            │
    │                 │ button up       │
    │            ┌────▼────┐            │
    │            │Releasing│  stable    │
    │            │         ├────────────┘
    │            └─────────┘
    │
    └── (transition ignored if counter < N)
```

- **Idle**: no button activity. On `true` input → `Pressing`, reset stability counter.
- **Pressing**: button is down but not yet confirmed. On `false` input → back to `Idle` (bounce). On `true` for N consecutive ticks → `Pressed`, **emit a press event** (`Some(true)`).
- **Pressed**: button confirmed pressed. On `false` input → `Releasing`, reset counter.
- **Releasing**: button is up but not yet confirmed. On `true` input → back to `Pressed` (bounce). On `false` for N consecutive ticks → `Idle`, **emit a release event** (`Some(false)`).

The debouncer emits `None` on every tick where no state transition is confirmed. It emits `Some(true)` for a confirmed press and `Some(false)` for a confirmed release.

```rust
pub struct ButtonDebouncer {
    state: DebouncerState,
    stable_counter: u32,
    debounce_ticks: u32,
}

impl ButtonDebouncer {
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
                        return Some(true); // confirmed press
                    }
                } else {
                    self.state = DebouncerState::Idle; // bounce
                }
                None
            }
            // ... similar for Pressed and Releasing
        }
    }
}
```

### Tying it together

A real embedded loop for reading a debounced button looks like:

```rust
let mut mcu = SimulatedMcu::new();
let mut debouncer = ButtonDebouncer::new(5); // 5-tick debounce

loop {
    process_timer_interrupt(&mut mcu);

    let raw = mcu.digital_pins[0]; // read raw button state
    if let Some(pressed) = debouncer.update(raw) {
        if pressed {
            // toggle an LED
            mcu.digital_pins[1] = !mcu.digital_pins[1];
        }
    }
}
```

Each timer tick calls `process_timer_interrupt`, which advances the clock. The debouncer uses the tick count (implicitly, via `update` being called once per tick) to measure stability duration.

### ASCII diagram: debouncer timeline

```
Tick:     0    1    2    3    4    5    6    7    8    9   10   11   12   13
Raw:     ────┐ ┌──┐ ┌──┐ ┌──┐                ┌──┐ ┌──┐ ┌──┐ ┌──┐
             └─┘  └─┘  └─┘  └────────────────┘  └─┘  └─┘  └─┘  └─────────
             ◄──── bounce ────►                ◄──── bounce ────►
                                                
State:  Idle →Pressing(1)→(2)→(3)→(4)→Pressed   →Releasing(1)→(2)→(3)→(4)→Idle
                                                        │                  │
Emit:                                             Some(true)         Some(false)
```

At tick 5, the counter reaches the debounce threshold (5), so `Pressed` is entered and `Some(true)` is emitted. At tick 12, the release is confirmed and `Some(false)` is emitted.

## Common Pitfalls

- **Debounce threshold too short.** If the threshold is shorter than the physical bounce duration, you'll get spurious presses. 5–20 ms is typical.
- **Debounce threshold too long.** The button feels laggy. Users perceive delays above 50 ms.
- **Not resetting the counter on state change.** If the counter isn't zeroed when entering `Pressing` or `Releasing`, the debouncer may skip the stable check and fire immediately.
- **Reading a digital pin as analog or vice versa.** On real hardware, pins are multiplexed — you configure them before reading. Our simulation keeps separate arrays to mirror this.
- **Assuming timer interrupts are instant.** Real ISRs have latency and can be preempted. Keep ISRs short (read a register, set a flag) and do processing in the main loop.

## Key Terms

- **ADC (Analog-to-Digital Converter):** converts a continuous voltage to a discrete digital value (e.g., 0–4095 for a 12-bit ADC).
- **ISR (Interrupt Service Routine):** a function executed by the CPU in response to a hardware interrupt.
- **Debouncing:** filtering mechanical switch noise to produce a single clean transition per press/release.
- **Timer tick:** a single increment of a hardware timer, typically driven by a crystal oscillator.
- **State machine:** a model of behavior composed of a finite set of states and transition rules between them.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-059)` comments.
2. Implement `SimulatedMcu::new()` — initialize analog pins to 0, digital pins to false, timer_counter to 0.
3. Implement `read_sensor()` — return `Some(value)` for valid `pin_id`, `None` for invalid.
4. Implement `process_timer_interrupt()` — increment the timer counter.
5. Implement the `ButtonDebouncer` state machine: `new()`, `update()`, `reset()`.
6. Run `cargo test -p module-059-exercises` until all tests pass.
7. Compare with `solutions/` afterwards.

## Further Reading

- [The Embedded Rust Book — Concurrency](https://docs.rust-embedded.org/book/concurrency/) — interrupts, critical sections, and sharing data with ISRs.
- [Jack Ganssle's Debouncing Guide](http://www.ganssle.com/debouncing.htm) — the definitive guide to switch debouncing.
- [State Machine pattern in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/) — implementing finite state machines idiomatically.
- [`embedded-hal` digital traits](https://docs.rs/embedded-hal/latest/embedded_hal/digital/index.html) — the real `InputPin` and `OutputPin` traits.
