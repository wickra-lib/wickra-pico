# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- CI/CD: the repository's workflow suite — `ci.yml` (host jobs: fmt, clippy,
  3-OS test, `no_std` thumbv6m build, host-golden parity check, MSRV 1.86,
  cargo-deny), `firmware.yml` (the RP2040 `thumbv6m-none-eabi` cross-compile,
  clippy and size report — the hardware gate), plus `codeql`, `scorecard`,
  `zizmor`, link-check and metadata-audit workflows, and a demo `release.yml`
  that builds the flashable `.uf2` with a SLSA build-provenance attestation and
  attaches it to a GitHub Release (no package registries — this is a demo).
- `docs/SIGNAL.md` and `docs/DETERMINISM.md`: the exact EMA-cross pipeline and
  the cross-target byte-parity guarantee (`f64` end to end, no platform math in
  the signal path, `libm::sinf` for a reproducible feed).
- `libm` is now used for the feed formula's `sin`, so the committed golden feed
  is bit-identical regardless of which OS runs `bless`.
- `docs/`: the demo reproducibility guides — `FLASHING.md` (BOOTSEL UF2 and
  probe-rs), `WIRING.md` (the on-board GPIO25 LED needs no wiring, plus an
  external-LED variant) and `VIDEO_SCRIPT.md` (the 30-second "LED blinks on the
  cross" clip).
- `golden/`: the cross-target reference corpus — `data/ema_cross.csv`, the
  byte-exact `expected/ema_cross.txt`, and `README.md` documenting the feed
  formula, the `bless` regeneration command, and why the on-device sequence
  matches the golden by construction (identical engine + feed) plus the host
  `check` and drift-guard verifications.
- `firmware/rp-pico`: the Raspberry Pi Pico (RP2040) firmware — the showcase.
  `#![no_std]`/`#![no_main]` on `thumbv6m-none-eabi` (rp-pico BSP + cortex-m-rt,
  blocking, `panic-halt`, no allocator). It streams the embedded `FEED` through
  the shared `SignalEngine` and toggles the on-board LED (GPIO25) on each cross —
  the same engine as the host reference, so the on-device sequence matches the
  golden. Workspace-excluded (own target/linker/`panic = "abort"`); a 10.5 KB
  release ELF, far under the 2 MB flash.
- `wickra-pico-host`: the host golden generator and byte-exact parity checker.
  `bless` regenerates the CSV feed, the embedded `FEED` const and the expected
  signal sequence from the single feed formula (`100 + 15·sin(i/8) + 0.05·i`,
  128 f32 ticks); `check` recomputes the sequence and asserts it matches the
  committed golden byte-for-byte — the host side of the cross-target guarantee.
  A drift-guard test pins the committed `embedded-data` `FEED` to the formula.
- `embedded-data`: the generated `FEED`/`FEED_LEN` const replay feed.
- `wickra-pico-signal`: the shared, `#![no_std]`, allocation-free EMA(9)/EMA(21)
  cross engine over `embed-core`. `SignalEngine::on_tick` streams prices and
  emits a `Signal` (golden/death cross) on each crossing, with an exact-`0.0` tie
  treated as a neutral hold. Host tests (warmup, ordered golden-then-death,
  reset, and proptest alternation/determinism) plus the `thumbv6m-none-eabi`
  build verify it runs identically on the host and bare metal.
- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the host workspace
  (`wickra-pico-signal`, `wickra-pico-host`, `embedded-data`) with the
  `firmware/*` crates excluded, and the `no_std`-kernel decision (Weg A:
  `embed-core` git dependency — see `ARCHITECTURE.md`).

[Unreleased]: https://github.com/wickra-lib/wickra-pico/commits/main
