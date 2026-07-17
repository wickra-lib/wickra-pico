# Flashing

How to get the firmware onto a board and running the demo.

## Raspberry Pi Pico (RP2040)

### BOOTSEL drag-and-drop (no probe)

1. Hold the **BOOTSEL** button while plugging the Pico into USB. It mounts as a
   `RPI-RP2` mass-storage drive.
2. Build and drop the UF2:

   ```bash
   cargo run --release --manifest-path firmware/rp-pico/Cargo.toml
   ```

   The `elf2uf2-rs -d` runner (in `firmware/rp-pico/.cargo/config.toml`) converts
   the ELF to a UF2 and copies it to the drive. The Pico reboots and starts the
   demo — the on-board LED begins toggling at the EMA crosses.

To make the UF2 by hand instead:

```bash
cargo build --release --manifest-path firmware/rp-pico/Cargo.toml
elf2uf2-rs firmware/rp-pico/target/thumbv6m-none-eabi/release/wickra-pico-rp-pico wickra-pico.uf2
# copy wickra-pico.uf2 onto the RPI-RP2 drive
```

### With a debug probe (probe-rs)

Switch the runner in `firmware/rp-pico/.cargo/config.toml` to
`probe-rs run --chip RP2040`, connect a probe (e.g. a second Pico running
`debugprobe`), then:

```bash
cargo run --release --manifest-path firmware/rp-pico/Cargo.toml
```

This flashes over SWD and streams RTT logs (once the `defmt` feature lands).

## ESP32 (optional, roadmap)

The ESP32 target needs the Xtensa toolchain (`espup`); once built, flash with:

```bash
espflash flash --monitor target/xtensa-esp32-none-elf/release/wickra-pico-esp32
```

## Troubleshooting

- **`elf2uf2-rs` not found** — `cargo install elf2uf2-rs`.
- **No `RPI-RP2` drive** — you did not hold BOOTSEL while connecting; unplug and
  retry.
- **LED never changes** — confirm the UF2 flashed (the drive ejects on success)
  and that you flashed the `--release` build.
