<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Pico — Wickra's O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-pico)
[![CI](https://github.com/wickra-lib/wickra-pico/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-pico/actions/workflows/ci.yml)
[![Firmware](https://github.com/wickra-lib/wickra-pico/actions/workflows/firmware.yml/badge.svg)](https://github.com/wickra-lib/wickra-pico/actions/workflows/firmware.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-pico)
[![Cross-target deterministic](https://img.shields.io/badge/cross--target-deterministic-3b82f6)](docs/DETERMINISM.md)

---

# Wickra Pico

**Wickra's O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same
> streaming indicator core that powers
> [wickra-backtest](https://github.com/wickra-lib/wickra-backtest) and
> [wickra-screener](https://github.com/wickra-lib/wickra-screener) also runs
> `no_std` on a microcontroller — no OS, no allocator, no heap.

<!-- DEMO GIF: LED blinks on the EMA(9)/EMA(21) cross (added in P-PICO-9). -->

Wickra Pico is a hardware showcase, not a library. An embedded replay feed
streams tick-by-tick through the `no_std` Wickra signal kernel (an EMA(9)/EMA(21)
cross); on a signal, a **GPIO LED toggles**. No other technical-analysis stack
runs without an operating system. The on-device signal sequence is verified
**byte-identical** to a `std` host reference — cross-target determinism, the same
guarantee the rest of the ecosystem makes across languages.

The `no_std` indicator kernel comes from
[`embed-core`](https://github.com/wickra-lib/wickra-embed) (allocation-free,
`#![no_std]`, `forbid(unsafe_code)`), consumed as a git dependency.

## How it works

An embedded `const` feed of 128 price ticks streams tick-by-tick through the
`no_std` `SignalEngine` — an EMA(9)/EMA(21) cross. On a **golden cross** the LED
turns on; on a **death cross** it turns off. The engine is O(1) per tick,
allocation-free, and links the same code on the host and on the device. The
firmware is nothing but HAL glue around it.

```rust
let mut engine = SignalEngine::new();
for &price in embedded_data::FEED.iter() {
    match engine.on_tick(f64::from(price)) {
        Some(Signal::GoldenCross) => led.set_high().unwrap(),
        Some(Signal::DeathCross)  => led.set_low().unwrap(),
        None => {}
    }
    delay.delay_ms(60);
}
```

The full pipeline is in [docs/SIGNAL.md](docs/SIGNAL.md).

## Determinism & parity

The signal sequence the RP2040 produces is **byte-identical** to the one the
`std` host reference produces — cross-*target* determinism, the same kind of
guarantee the rest of the ecosystem makes across languages. It holds because the
engine is `f64` end to end, allocation-free, and has no platform-specific math in
the signal path. `wickra-pico-host check` recomputes the sequence and asserts it
matches the committed golden byte-for-byte; a drift guard pins the compiled-in
feed to the formula. See [docs/DETERMINISM.md](docs/DETERMINISM.md).

## The 30-second video

The demo is meant to be filmed: the on-board LED blinking on the EMA cross, no
wiring, no screen. The shot list and captions are in
[docs/VIDEO_SCRIPT.md](docs/VIDEO_SCRIPT.md).

## Hardware

- **Raspberry Pi Pico** (RP2040, `thumbv6m-none-eabi`) — the primary target. The
  demo uses the **on-board LED (GPIO25)**, so it needs no wiring at all.
- **ESP32** (Xtensa) — a roadmap target (needs the `espup` toolchain); see
  [ROADMAP.md](ROADMAP.md).

## Flash it yourself

Grab the `.uf2` from a release (or build it), hold **BOOTSEL**, plug the Pico in,
and drag the file onto the `RPI-RP2` drive. The full instructions — including the
`probe-rs` route — are in [docs/FLASHING.md](docs/FLASHING.md); wiring options
(none needed, plus an external-LED variant) are in
[docs/WIRING.md](docs/WIRING.md).

## Workspace layout

```
crates/wickra-pico-signal   no_std signal kernel over embed-core's EMA cross
crates/wickra-pico-host     std golden-reference generator + parity checker
embedded-data/              the generated const replay feed
firmware/rp-pico            RP2040 firmware (workspace-excluded: own target)
golden/                     the cross-target reference corpus
```

## Building from source

```bash
# Host workspace members (signal kernel, host reference, feed):
cargo build
cargo test
cargo run -p wickra-pico-host -- check   # verify the golden sequence

# RP2040 firmware (its own excluded crate):
cd firmware/rp-pico
cargo build --target thumbv6m-none-eabi --release
```

The firmware is excluded from the host workspace — it builds for a bare-metal
target with its own linker script and `panic = "abort"` profile; see
[ARCHITECTURE.md](ARCHITECTURE.md).

## Requirements

- Rust 1.86+ (MSRV); the `thumbv6m-none-eabi` target for the RP2040 firmware.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The signal
kernel is a fast, non-cryptographic streaming calculator; it handles no keys and
no network.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
