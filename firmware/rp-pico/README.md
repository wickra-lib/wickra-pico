# Wickra Pico — RP2040 firmware

Raspberry Pi Pico (RP2040) firmware for the Wickra Pico demo. It streams the
embedded replay feed through the shared `no_std` `SignalEngine` and toggles the
on-board LED (GPIO25): a golden cross turns it on, a death cross turns it off.
The firmware is HAL glue only — the signal logic lives in `wickra-pico-signal`,
so the on-device sequence matches the host golden byte-for-byte.

## Approach

`rp-pico` (the board-support crate) + `cortex-m-rt` (`#[entry]`), blocking — no
async executor. Chosen over `embassy-rp` for a minimal dependency surface; the
demo does not need concurrency. `panic-halt` on panic. The signal engine folds
the const `FEED` from `embedded-data`; there is no heap and no allocator.

This crate is **excluded** from the host workspace (it builds for
`thumbv6m-none-eabi` with its own linker script and `panic = "abort"`), so it has
its own `Cargo.lock` and is built by its own CI job.

## Build

```bash
cargo build --release           # target thumbv6m-none-eabi (set in .cargo/config.toml)
```

## Flash

Hold **BOOTSEL** while plugging the Pico in, then:

```bash
cargo run --release             # runs elf2uf2-rs -d to drop the UF2 onto the drive
```

Or with a debug probe, switch the runner in `.cargo/config.toml` to
`probe-rs run --chip RP2040`. Once flashed, the LED changes state at each EMA
cross — the visible cross-target-parity proof.
