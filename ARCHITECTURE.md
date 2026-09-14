# Architecture

Wickra Pico runs Wickra's O(1) streaming indicator core bare-metal on a $5
microcontroller. It is a hardware showcase built around one guarantee: the
signal sequence produced **on-device** is byte-identical to a `std` host
reference — cross-target determinism.

## The no_std kernel decision (§0)

The indicator kernel must be `no_std` (no OS, no allocator). Two paths were on
the table:

- **Weg A (chosen): depend on `wickra-embed-core`.** The
  [`wickra-embed`](https://github.com/wickra-lib/wickra-embed) repo ships
  `wickra-embed-core` — an allocation-free, `#![no_std]`, `forbid(unsafe_code)`
  crate with the streaming `Indicator` trait and `Ema`/`Sma`/`Rsi`/`Atr`/`Roc`,
  byte-exact against the main `wickra-core`. Wickra Pico consumes it from
  crates.io, pinned exactly (`wickra-embed-core = "=0.1.0"`).
- **Weg B (fallback, not taken): make `wickra-core` no_std.** An upstream change
  to the main `wickra` repo giving `wickra-core` a `no_std` feature. Not needed —
  `wickra-embed-core` already exists and is verified.

`wickra-core` v0.9 is `std` (uses `thiserror`, optional `rayon`), so it is **not**
usable directly on bare metal; `wickra-embed-core` is the no_std path.

## Workspace

| Crate / dir | Target | Role |
|-------------|--------|------|
| `crates/wickra-pico-signal` | host + `no_std` | The signal wrapper: feeds ticks through the EMA(9)/EMA(21) cross from `wickra-embed-core` and emits a cross signal. |
| `crates/wickra-pico-host` | host (`std`) | The golden-reference generator and byte-exact parity checker — the oracle the firmware is checked against. |
| `embedded-data` | host + `no_std` | The embedded replay feed (a generated `const [f32; 128]`). |
| `firmware/rp-pico` | `thumbv6m-none-eabi` | RP2040 firmware: streams the feed, toggles the on-board LED on a cross. The showcase. |

The three host members build and test with a plain `cargo build` / `cargo test`
at the root. The firmware crate is **excluded** from the host workspace
(`exclude` in the root `Cargo.toml`): it builds for a bare-metal target with its
own linker script, its own `Cargo.lock`, and a `panic = "abort"` profile, which
the host build and the host tests (which need unwinding) must not inherit. The
firmware is built by its own `firmware.yml` CI job for `thumbv6m-none-eabi` — the
hardware gate.

An **ESP32** (Xtensa) firmware is a roadmap item, not a present crate: it needs
the `espup` toolchain, which is not in the current toolchain matrix. See
[ROADMAP.md](ROADMAP.md).

## The firmware

[`firmware/rp-pico`](firmware/rp-pico) is `#![no_std]`/`#![no_main]` on
`thumbv6m-none-eabi`, built on the `rp-pico` BSP + `cortex-m-rt` (a plain
blocking `#[entry]`, not an async executor — the demo is a single loop, so the
simpler model wins). It contains **no signal logic**: it initialises the clocks
and GPIO, then folds the `const` feed through the shared `SignalEngine` and
drives the on-board LED — high on a golden cross, low on a death cross. It is a
~10.5 KB release ELF, far under the Pico's 2 MB flash.

The on-board LED is **GPIO25**, so the default demo needs no wiring at all; an
external-LED variant is documented in [docs/WIRING.md](docs/WIRING.md), and
flashing (BOOTSEL `.uf2` or `probe-rs`) in [docs/FLASHING.md](docs/FLASHING.md).

## Numeric type: `f64` everywhere

The `wickra-embed-core` `Ema` is `f64`-native, so — following the handoff's "one type,
everywhere" rule — Wickra Pico is `f64` end to end: the engine, the difference,
and the comparison. The feed is stored as `f32` and widens **losslessly** to
`f64` before the engine, identically on the host and the device. Keeping a single
type through the whole signal path is what makes the bits identical across
targets. See [docs/DETERMINISM.md](docs/DETERMINISM.md).

## Cross-target golden

Where the library repos guarantee cross-*language* byte-equality, Wickra Pico
guarantees cross-*target* byte-equality: the signal sequence emitted on the
device equals the sequence emitted by `wickra-pico-host` on a workstation, for
the same feed. Because the engine is allocation-free, deterministic, and links
identically on both sides — with no platform math in the signal path — the two
targets cannot diverge. The full argument, and the three checks that enforce it,
are in [docs/DETERMINISM.md](docs/DETERMINISM.md); the pipeline itself is in
[docs/SIGNAL.md](docs/SIGNAL.md).

## See also

- [docs/SIGNAL.md](docs/SIGNAL.md) — the EMA-cross pipeline, exact.
- [docs/DETERMINISM.md](docs/DETERMINISM.md) — the cross-target parity guarantee.
- [ROADMAP.md](ROADMAP.md) — ESP32 GA, an on-device Renode/QEMU sim, a UART feed.
- [THREAT_MODEL.md](THREAT_MODEL.md) — narrow: no keys, no network.
