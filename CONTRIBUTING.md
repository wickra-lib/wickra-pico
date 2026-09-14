# Contributing to wickra-pico

Thanks for your interest. Issues, bug reports, ideas and pull requests are all
welcome at <https://github.com/wickra-lib/wickra-pico>. For larger changes,
open an issue first so we can agree on the approach.

## Orientation

- The core — the `Indicator` contract, the const-generic no-alloc ring buffer,
  and the verified subset of indicators (`Sma`, `Ema`, `Rsi`, `Atr`, `Roc`, …) —
  lives in `crates/wickra-pico-signal`. It is `#![no_std]` and allocation-free: no
  `Box`, no `Vec`, no heap. `wickra-core` appears only as a **dev-dependency**,
  the parity oracle the no-alloc values are checked byte-for-byte against.
- The C ABI lives in `bindings/c`: a no-alloc, stack-handle surface — the one
  place `unsafe` is re-allowed — with a cbindgen-generated header.
- Examples live under `examples/` (a host CSV runner, a C usage sample, and a
  `#![no_std] #![no_main]` QEMU/Cortex-M demo).

## The dev loop

Every change runs green locally before a commit:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
# The no_std core must build without std on the bare-metal targets:
cargo build -p wickra-pico-signal --no-default-features --target thumbv7em-none-eabihf
cargo build -p wickra-pico-signal --no-default-features --target thumbv6m-none-eabi
# Host tests, including byte-parity against wickra-core:
cargo test -p wickra-pico-signal --all-features
cargo deny check
```

`cargo fmt --all` and the `clippy -D warnings` gate are enforced in CI, and the
no_std core is built on both Cortex-M targets on every PR.

## Conventions

- **Commits are signed** and follow Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`…). One logical change per commit. Open a PR against `main`;
  do not push to `main` directly.
- **All public artifacts are in English** — code, comments, commit messages, PR
  titles and bodies, issues and docs.
- **No secrets, ever.**
- **Production code only** — no mocks outside `#[cfg(test)]`, no TODO stubs, and
  no defensive branches that can never run (they fail coverage). No `panic!`,
  `unwrap` or `expect` in the no_std core path — a panic on an MCU is a reset;
  return `Result`/`Option` instead.

## Changing the signal

No indicator code lives here: the kernel is the published `wickra-embed-core`,
and the whole trading logic of the demo is `SignalEngine` in
`crates/wickra-pico-signal/src/engine.rs` -- `#![no_std]`, allocation-free,
scalar-only, and the same code on the host and on the RP2040. A change to it
(a different period, a new signal) is one change in that file, with a host
test in `crates/wickra-pico-signal/tests/engine_tests.rs`, and then the golden
corpus regenerated with `cargo run -p wickra-pico-host -- bless` and verified
with `cargo run -p wickra-pico-host -- check`, which recomputes the sequence
and asserts it equals `golden/expected/ema_cross.txt` byte-for-byte. Keep the
signal path free of platform math (`f64` end to end, `libm` for the feed), so
the on-device sequence stays identical to the host's -- that parity is the
point of the repository. See [docs/SIGNAL.md](docs/SIGNAL.md) and
[docs/DETERMINISM.md](docs/DETERMINISM.md).

## Developer Certificate of Origin

Contributions are accepted under the [DCO](DCO); sign off your commits with
`git commit -s`. By contributing you agree your work is dual-licensed under
`MIT OR Apache-2.0`.
