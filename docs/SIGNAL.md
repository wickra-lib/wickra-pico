# The signal

Wickra Pico's whole behaviour is one rule: stream a price feed through two
exponential moving averages and fire a signal when the fast one crosses the
slow one. This document is the exact, allocation-free pipeline — the same code
runs on the host reference and on the microcontroller.

## The engine

[`wickra-pico-signal`](../crates/wickra-pico-signal) is `#![no_std]` and
allocation-free. Its state is three scalars:

```rust
pub struct SignalEngine {
    ema_fast: Ema,          // EMA(9),  from wickra-embed-core
    ema_slow: Ema,          // EMA(21), from wickra-embed-core
    prev_diff: Option<f64>, // last (fast - slow) with both EMAs defined
}
```

`FAST = 9`, `SLOW = 21`. The two `Ema`s come from
[`wickra-embed-core`](https://github.com/wickra-lib/wickra-embed) — the `no_std`,
`forbid(unsafe_code)` Wickra indicator kernel — so the arithmetic is the same
one the rest of the ecosystem ships, not a re-implementation.

There is no `Vec`, no `Box`, no allocator, and no I/O in the engine: it is pure
logic, which is exactly why it is 100 % host-testable and why the on-device
result cannot diverge from the host reference.

## `on_tick` — the cross detection

`fn on_tick(&mut self, price: f64) -> Option<Signal>` is O(1) and returns a
signal on the exact bar where a cross happens, `None` otherwise:

1. `f = ema_fast.update(price)`, `s = ema_slow.update(price)`.
2. If either EMA is still warming up (`None`), leave `prev_diff` untouched and
   return `None`.
3. Otherwise `diff = f - s`.
4. Compare against the previous difference:
   - **golden cross** — `prev < 0.0 && diff > 0.0` → the fast EMA crossed the
     slow one from below → `Signal::GoldenCross`.
   - **death cross** — `prev > 0.0 && diff < 0.0` → crossed from above →
     `Signal::DeathCross`.
   - anything else (including an exact `0.0` touch) → no signal.
5. Store the last **non-zero** `diff` as the reference and return the signal.

The tie rule is deliberate: an exact `diff == 0.0` is a neutral hold, not a
cross. Only the reference difference's sign matters, and a zero is never stored
as that reference, so a signal fires only on a true sign change. There is **no
epsilon** — the comparison is on exact IEEE-754 bits, because the goal is
byte-exact cross-target parity, not "approximately equal". The feed is chosen so
no pathological floating-point edges occur (see [DETERMINISM.md](DETERMINISM.md)).

## `Signal` — the event

```rust
pub enum Signal {
    GoldenCross, // fast crossed slow upward   → LED on
    DeathCross,  // fast crossed slow downward  → LED off
}
```

`Signal` is `Copy`, with no `String` and no `serde` on-device. `token()` renders
a stable ASCII token (`"golden_cross"` / `"death_cross"`) — used only by the
host golden report, never on the microcontroller.

## The on-device pipeline (no JSON, no FFI)

This is a microcontroller demo, so there is no `command_json`, no FFI boundary,
and no serialization on the device. The only "interface" is the streaming
`Indicator` contract plus the fixed feed loop. The firmware is HAL glue around
the engine:

```rust
let mut engine = SignalEngine::new();
for &price in embedded_data::FEED.iter() {
    match engine.on_tick(f64::from(price)) {
        Some(Signal::GoldenCross) => led.set_high().unwrap(),
        Some(Signal::DeathCross)  => led.set_low().unwrap(),
        None => {}
    }
    delay.delay_ms(TICK_MS);
}
```

The feed is [`embedded-data`](../embedded-data)'s `FEED` — a `const [f32; 128]`
that lives in flash (no RAM copy, no allocation). Each `f32` price widens
losslessly to the `f64` the engine consumes.

## Warmup

`EMA(21)` is the slower to become defined, so no signal can fire until the slow
EMA has a value and there is a previous difference to compare against. The test
`warmup_emits_nothing` in
[`crates/wickra-pico-signal/tests/engine_tests.rs`](../crates/wickra-pico-signal/tests/engine_tests.rs)
pins this: the engine returns `None` throughout warmup and only starts emitting
once both EMAs are defined.

## The golden sequence

Running the engine over the committed feed produces exactly five crosses:

```
23 death_cross
49 golden_cross
75 death_cross
99 golden_cross
125 death_cross
```

On the device the on-board LED goes **off** at each `death_cross` and **on** at
each `golden_cross` — the five visible transitions are the demo. The host
`wickra-pico-host check` asserts this sequence byte-for-byte against
[`golden/expected/ema_cross.txt`](../golden/expected/ema_cross.txt); see
[DETERMINISM.md](DETERMINISM.md) for why the device produces the same sequence.

## See also

- [DETERMINISM.md](DETERMINISM.md) — the cross-target parity guarantee.
- [../golden/README.md](../golden/README.md) — the feed formula and the `bless`
  regeneration command.
- [../ARCHITECTURE.md](../ARCHITECTURE.md) — how the pieces fit together.
