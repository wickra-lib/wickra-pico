<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Pico — Wickra's O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico" width="100%"></a>
</p>

[![Built on Wickra](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/built-on.svg)](https://github.com/wickra-lib/wickra)
[![Status](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/status.svg)](https://github.com/wickra-lib/wickra-pico)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/ci.svg)](https://github.com/wickra-lib/wickra-pico/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/codeql.svg)](https://github.com/wickra-lib/wickra-pico/actions/workflows/codeql.yml)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/release.svg)](https://github.com/wickra-lib/wickra-pico/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-pico)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/provenance.svg)](https://github.com/wickra-lib/wickra-pico/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/docs.svg)](https://pico.wickra.org)
[![Firmware](https://github.com/wickra-lib/wickra-pico/actions/workflows/firmware.yml/badge.svg)](https://github.com/wickra-lib/wickra-pico/actions/workflows/firmware.yml)
[![Cross-target deterministic](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/cross-target.svg)](docs/DETERMINISM.md)

---

**Wickra's O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross.**

> **▶ Live demos:** the backtester compiled to WebAssembly, an equity curve building bar by bar — **[backtest-live.wickra.org](https://backtest-live.wickra.org)**;
> one StrategySpec side by side in Python, Rust, JS and Go — **[playground.wickra.org](https://playground.wickra.org)**;
> all 514 indicators of the core over a real Binance feed — **[live.wickra.org](https://live.wickra.org)**. Zero backend, all of them.

**Part of the [Wickra ecosystem](#ecosystem):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal) and 20 more — see [the full list](https://github.com/wickra-lib).
streaming indicator core that powers
[wickra-backtest](https://github.com/wickra-lib/wickra-backtest) and
[wickra-screener](https://github.com/wickra-lib/wickra-screener) also runs
`no_std` on a microcontroller — no OS, no allocator, no heap.

<!-- DEMO GIF: LED blinks on the EMA(9)/EMA(21) cross (added in P-PICO-9). -->

Wickra Pico is a hardware showcase, not a library. An embedded replay feed
streams tick-by-tick through the `no_std` Wickra signal kernel (an EMA(9)/EMA(21)
cross); on a signal, a **GPIO LED toggles**. No other technical-analysis stack
runs without an operating system. The on-device signal sequence is verified
**byte-identical** to a `std` host reference — cross-target determinism, the same
guarantee the rest of the ecosystem makes across languages.

The `no_std` indicator kernel comes from
[`wickra-embed-core`](https://github.com/wickra-lib/wickra-embed) (allocation-free,
`#![no_std]`, `forbid(unsafe_code)`), from crates.io.

```bash
# Verify the golden signal sequence on the host, then build the firmware:
cargo run -p wickra-pico-host -- check
( cd firmware/rp-pico && cargo build --target thumbv6m-none-eabi --release )
```

## Status

**0.1.2 — the current release.** The firmware image, the host-side tests and the
golden corpus are in place and green in CI. [ROADMAP.md](ROADMAP.md) has what is
done, what is open and what is not planned.

## Documentation

The reference documentation lives at **[pico.wickra.org](https://pico.wickra.org)**.
What stays beside the code is in [`docs/`](docs/README.md): the signal, the
determinism argument, wiring and flashing.

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

## Project layout

```
crates/wickra-pico-signal   no_std signal kernel over wickra-embed-core's EMA cross
crates/wickra-pico-host     std golden-reference generator + parity checker
embedded-data/              the generated const replay feed
firmware/rp-pico            RP2040 firmware (workspace-excluded: own target)
golden/                     the cross-target reference corpus
```

## Building everything from source

```bash
# Host workspace members (signal kernel, host reference, feed):
cargo build --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p wickra-pico-host -- check   # verify the golden sequence

# The kernel for the Cortex-M0+, allocation-free, std off:
cargo build -p wickra-pico-signal --no-default-features --target thumbv6m-none-eabi

# RP2040 firmware (its own excluded crate):
( cd firmware/rp-pico && cargo build --target thumbv6m-none-eabi --release )
```

The firmware is excluded from the host workspace — it builds for a bare-metal
target with its own linker script and `panic = "abort"` profile; see
[ARCHITECTURE.md](ARCHITECTURE.md).

## Testing

Run the suites with the commands in
[Building everything from source](#building-everything-from-source).

- **`wickra-pico-signal`** — unit and property tests over the EMA cross, and
  the byte-parity tests against the std `wickra-core`: the same ticks through
  the no_std kernel and the reference produce the same signals.
- **`wickra-pico-host`** — recomputes the signal sequence over the golden
  series and asserts it equals `golden/expected/ema_cross.txt` byte-for-byte,
  the host half of the cross-target guarantee.
- **The firmware** — `firmware.yml` cross-compiles it for `thumbv6m-none-eabi`
  on every push; the on-device run reproduces the same sequence.
- **Bench** — `cargo bench -p wickra-pico-signal` times `on_tick` on the host;
  see [BENCHMARKS.md](BENCHMARKS.md).

## Requirements

- **Rust 1.86+** — the workspace MSRV.
- The **`thumbv6m-none-eabi`** target for the RP2040 firmware
  (`rustup target add thumbv6m-none-eabi`), and `elf2uf2-rs` to turn the ELF
  into a drag-and-drop `.uf2`.
- A **Raspberry Pi Pico** (RP2040) to run it; the host suite needs no hardware.

## Benchmarks

The per-update cost of the signal kernel on the host and, once measured, in
cycles on the RP2040 — see [BENCHMARKS.md](BENCHMARKS.md); reproduce with
`cargo bench -p wickra-pico-signal`.

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at millions of backtests per second, mutating and crossing JSON specs across the 514-indicator space
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 514 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a 514-dim live vector, for similarity search, clustering and anomaly detection
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

Docs at [docs.wickra.org](https://docs.wickra.org); the marketing site and
in-browser demo at [wickra.org](https://wickra.org).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The signal
kernel is a fast, non-cryptographic streaming calculator; it handles no keys and
no network.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option. Use it, fork it, modify it, redistribute it — commercially or
not — file issues, send pull requests; all welcome.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Disclaimer

Wickra Pico is a hardware demonstration, provided "as is" without warranty of
any kind. The signal it blinks is an EMA cross over an embedded replay feed;
nothing here is financial advice, and trading carries risk of loss.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-pico">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-pico/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-pico/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-pico star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-pico/star-history.svg">
</p>
