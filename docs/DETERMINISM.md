# Determinism & cross-target parity

Where the Wickra library repos guarantee cross-*language* byte-equality, Wickra
Pico guarantees cross-*target* byte-equality: the signal sequence produced
**on the RP2040** equals the sequence produced by the `std` host reference on a
workstation, byte-for-byte. This document is why that holds.

## One numeric type, everywhere: `f64`

The Wickra indicator core (`wickra-core`, and the `no_std` `wickra-embed-core` it
mirrors) is `f64`-native — `Ema::update` takes and returns `f64`. The handoff
spec left the numeric type open (an f32 placeholder), with an explicit
instruction: **if the core is `f64`, use `f64` everywhere, including the
firmware.** So Wickra Pico is `f64` end to end — the engine, the difference, and
the comparison. There is no `f32`↔`f64` conversion inside the signal path, which
is the single most important property for determinism: one type, one rounding,
one set of bits.

The replay feed is stored as `f32` (it is a compact price series, and `f32`
round-trips cleanly through the CSV and the `const`), but every value widens
**losslessly** to `f64` via `f64::from(price)` before it reaches the engine —
the same widening on the host and on the device. An `f32`→`f64` widening is
exact by IEEE-754, so this introduces no divergence.

The RP2040's Cortex-M0+ has no hardware FPU, so `f64` runs as soft-float via
`compiler-builtins`. It is slower than a hardware FPU would be — irrelevant at
one tick every 60 ms — and, crucially, **rounding-correct**: soft-float `f64`
produces the same rounded result as any other IEEE-754 `f64` implementation.

## Why the bits match

IEEE-754 `f64` arithmetic with a fixed operation order is deterministic across
targets, provided none of the divergence sources are present:

- **No FMA / no contraction.** No `mul_add` or fused multiply-add in the signal
  path. Rust does not contract floating-point operations by default, and we do
  not opt in via intrinsics. A fused multiply-add rounds once instead of twice
  and would differ from the two-step host result.
- **No fast-math.** No `-ffast-math`-style relaxation anywhere.
- **No 80-bit intermediates.** The host builds for SSE2 `f64` (not x87's 80-bit
  registers), so host intermediates are true `f64`, matching the device.

The engine links the **identical** `wickra-pico-signal` code on all three sides
(host, RP2040, and any future target); only the HAL shell around it differs.
There is no per-target reformatting and no re-implementation, so there is nothing
to drift.

## The one place platform math leaked in — and the fix

The feed itself is generated from a formula:

```
price(i) = 100.0 + 15.0 * sin(i / 8.0) + 0.05 * i     # 128 f32 ticks
```

The first version computed `sin` with the standard library's `f32::sin`, which
dispatches to the host platform's libm. Those libms are **not** bit-identical:
Windows and Linux/glibc disagreed by ~1 ULP on exactly one sample (index 101).
A `const` feed blessed on one OS then tripped the drift-guard test on another.

The fix is [`libm::sinf`](https://docs.rs/libm) — a pure-Rust softfloat
implementation that produces the same bits on every platform. The feed formula
now calls `libm::sinf`, so the committed feed is reproducible regardless of which
OS runs `bless`. (The firmware never computes `sin` at all — it reads the
pre-computed `const` feed from flash — but making the *generator* deterministic
is what keeps the golden reproducible in CI.)

## The three checks that enforce it

1. **Host parity** — `cargo run -p wickra-pico-host -- check` recomputes the
   signal sequence with the host engine and asserts it equals
   [`golden/expected/ema_cross.txt`](../golden/expected/ema_cross.txt)
   byte-for-byte. Runs as the `host-golden` CI job.
2. **Drift guard** — the `embedded_feed_matches_formula` test in
   [`crates/wickra-pico-host/src/gen.rs`](../crates/wickra-pico-host/src/gen.rs)
   pins the committed `embedded-data` `FEED` to the formula output, comparing
   raw `f32::to_bits`, so the feed the firmware compiles in cannot drift from the
   one the golden was blessed from.
3. **Engine bit-stability** — `cross_is_bit_stable` and `zero_tie_is_neutral` in
   [`crates/wickra-pico-signal/src/engine.rs`](../crates/wickra-pico-signal/src/engine.rs),
   plus the `deterministic` proptest in the integration tests, pin the cross
   detection to exact bits with the neutral-`0.0` tie rule (no epsilon).

## On-device confirmation (deferred)

An empirical on-device check — running the firmware ELF under a Renode/QEMU
RP2040 simulation and capturing the emitted sequence over RTT — is tracked in
[../ROADMAP.md](../ROADMAP.md). It needs a `defmt` output feature in the
firmware. The parity above already pins the sequence by construction: identical
engine, identical feed, lossless widening, no platform math in the path.

## See also

- [SIGNAL.md](SIGNAL.md) — the EMA-cross pipeline the determinism applies to.
- [../golden/README.md](../golden/README.md) — the golden corpus and `bless`.
- [../ARCHITECTURE.md](../ARCHITECTURE.md) — the cross-target design.
