<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Pico — Wickra's O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-pico)
[![CI](https://github.com/wickra-lib/wickra-pico/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-pico/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-pico)

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

## Status

Early development (0.1.0, unreleased). Built out in phases; this scaffold pins
the repository, governance and supply-chain configuration ahead of the signal
kernel, the embedded feed and the firmware crates.

## Hardware

- **Raspberry Pi Pico** (RP2040, `thumbv6m-none-eabi`) — the primary target.
- **ESP32** — optional secondary target.

## Workspace layout

```
crates/wickra-pico-signal   no_std signal wrapper over embed-core's EMA cross
crates/wickra-pico-host     std golden-reference generator (the parity oracle)
embedded-data/              the embedded replay feed (const array)
firmware/rp-pico            RP2040 firmware (workspace-excluded: own target)
firmware/esp32              ESP32 firmware  (workspace-excluded: own target)
```

## Building from source

```bash
# Host workspace members (signal kernel, host reference, feed):
cargo build
cargo test
```

Firmware crates build for their own bare-metal targets and are excluded from the
host workspace; see [ARCHITECTURE.md](ARCHITECTURE.md).

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
