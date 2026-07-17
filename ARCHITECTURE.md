# Architecture

Wickra Pico runs Wickra's O(1) streaming indicator core bare-metal on a $5
microcontroller. It is a hardware showcase built around one guarantee: the
signal sequence produced **on-device** is byte-identical to a `std` host
reference — cross-target determinism.

## The no_std kernel decision (§0)

The indicator kernel must be `no_std` (no OS, no allocator). Two paths were on
the table:

- **Weg A (chosen): git-dep on `embed-core`.** The
  [`wickra-embed`](https://github.com/wickra-lib/wickra-embed) repo already ships
  `embed-core` — an allocation-free, `#![no_std]`, `forbid(unsafe_code)` crate
  with the streaming `Indicator` trait and `Ema`/`Sma`/`Rsi`/`Atr`/`Roc`,
  byte-exact against the main `wickra-core`. Wickra Pico consumes it as a
  version-pinned git dependency (`embed-core = { git = "…/wickra-embed", version
  = "0.1" }`).
- **Weg B (fallback, not taken): make `wickra-core` no_std.** An upstream change
  to the main `wickra` repo giving `wickra-core` a `no_std` feature. Not needed —
  `embed-core` already exists and is verified.

`wickra-core` v0.9 is `std` (uses `thiserror`, optional `rayon`), so it is **not**
usable directly on bare metal; `embed-core` is the no_std path.

## Workspace

| Crate / dir | Target | Role |
|-------------|--------|------|
| `crates/wickra-pico-signal` | host + `no_std` | The signal wrapper: feeds ticks through the EMA(9)/EMA(21) cross from `embed-core` and emits a cross signal. |
| `crates/wickra-pico-host` | host (`std`) | The golden-reference generator — the parity oracle the firmware is checked against. |
| `embedded-data` | host + `no_std` | The embedded replay feed (a `const` array of ticks). |
| `firmware/rp-pico` | `thumbv6m-none-eabi` | RP2040 firmware: streams the feed, toggles a GPIO LED on a cross. |
| `firmware/esp32` | ESP32 | Optional secondary firmware target. |

The firmware crates are **excluded** from the host workspace (`exclude` in the
root `Cargo.toml`): they build for bare-metal targets with their own linker
scripts and panic strategy (`panic = "abort"`), which the host `cargo build`
and host tests (which need unwinding) must not inherit. `cargo build` at the root
builds and tests only the three host members; the firmware is built by its own CI
job for `thumbv6m-none-eabi`.

## Cross-target golden

Where the library repos guarantee cross-*language* byte-equality, Wickra Pico
guarantees cross-*target* byte-equality: the signal sequence emitted on the
device equals the sequence emitted by `wickra-pico-host` on a workstation, for
the same feed. Because the kernel is allocation-free and deterministic, the two
targets cannot diverge.

## See also

- [ROADMAP.md](ROADMAP.md) — extraction of a dedicated `wickra-embed` facade,
  more indicators, a UART feed, ESP32 GA.
- [THREAT_MODEL.md](THREAT_MODEL.md) — narrow: no keys, no network.
