# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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
