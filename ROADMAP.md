# Roadmap

Wickra Pico is a hardware showcase for Wickra's `no_std` claim. Planned work,
roughly in order:

- **Signal kernel + host reference** — the EMA(9)/EMA(21) cross over `wickra-embed-core`,
  with a `std` golden generator.
- **Embedded feed** — a `const` replay array; later, a UART feed for live input.
- **RP2040 firmware** — stream the feed, toggle the on-board LED on a cross;
  verified in QEMU/Renode and on real hardware.
- **ESP32 firmware** — second target to GA.
- **More indicators** — expose the rest of `wickra-embed-core` (RSI, ATR) on-device.
- **`wickra-embed` facade** — extract a dedicated, documented `no_std` facade
  crate over `wickra-embed-core` for downstream reuse.
- **Demo video + write-up** — the 30-second "LED blinks on the cross" clip.
