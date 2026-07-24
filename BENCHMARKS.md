# Benchmarks

`wickra-pico` is a hardware showcase, not a library: it runs the `no_std` Wickra
signal kernel (an EMA(9)/EMA(21) cross) bare-metal on a Raspberry Pi Pico. The
numbers that matter here are the **per-update cost of the signal kernel** — on a
host in nanoseconds, and on the target MCU in cycles — plus the proof that both
paths produce a **byte-identical** result.

## Methodology

- **Host.** A criterion bench in `crates/wickra-pico-signal` times a single
  `update` call on the steady-state O(1) path (warmed past the EMA warmup), on
  x86-64. This is the same kernel code the firmware runs.
- **MCU.** The RP2040 target is a Cortex-M0+ built for `thumbv6m-none-eabi`. The
  firmware reads the SysTick counter around a fixed batch of `update` calls and
  reports mean cycles per update — the same code path as the host bench, on the
  chip it actually ships to.
- **Determinism.** `wickra-pico-host` runs the identical tick feed through the
  kernel with `std` and asserts the EMA(9)/EMA(21) cross decisions are
  byte-identical to the MCU run. Cross-target determinism is the property the
  showcase demonstrates; see [`docs/DETERMINISM.md`](docs/DETERMINISM.md).
- **Allocation.** Zero, by construction — the signal kernel is `#![no_std]` with
  no allocator linked. There is nothing to measure; the absence is the point, and
  CI enforces it by building for `thumbv6m-none-eabi`.

## Results

Numbers land as the bench (phase P5) and the on-device measurement (phase P6) are
built out. The host column is measured (`cargo bench -p wickra-pico-signal`,
criterion median on x86-64 local dev, rounded — indicative, not a CI-pinned
regression gate; the nightly bench tracks drift). The RP2040 column lands with the
on-device SysTick measurement.

| Signal              | Host (ns/update) | RP2040 Cortex-M0+ (cycles/update) | Allocations |
|---------------------|------------------|-----------------------------------|-------------|
| `Ema<9>`            | ~13              | _pending_                         | 0           |
| `Ema<21>`           | ~13              | _pending_                         | 0           |
| EMA(9)/EMA(21) cross| ~27              | _pending_                         | 0           |

Each EMA update is a single multiply-add on the running value, so the cost is flat
regardless of the period; the cross is just the two updates plus a comparison. The
Cortex-M0+ has no hardware divide and no FPU, so the on-device figure reflects the
soft-float path the firmware links — which is exactly why the kernel is written to
stay on integer/`f32` operations with a bounded per-tick cost.

## Reproducing

```bash
# Host, per-update criterion bench:
cargo bench -p wickra-pico-signal

# On-device cycle counts (requires the Pico toolchain + a board or QEMU):
cargo run -p wickra-pico-host --release
```

All figures are steady-state, post-warmup, single-update costs. Because every
update is O(1) with no allocation, the worst-case per-update latency is bounded —
the property that matters for bare-metal real-time firmware, more than the mean.
