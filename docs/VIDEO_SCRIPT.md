# Video script — the 30-second demo

The whole pitch is a short clip of an LED blinking on a $5 board with no OS.

## Shot list (~30s)

1. **0–4s — the hook.** Close-up of the bare Pico on a desk. Caption:
   *"Wickra's O(1) indicator core — running bare-metal on a $5 Raspberry Pi Pico."*
2. **4–10s — flash it.** Screen recording: `cargo run --release` → the UF2 drops
   onto `RPI-RP2` → the board reboots. Caption: *"no OS, no allocator, no heap."*
3. **10–24s — the payoff.** Board close-up: the on-board LED toggles. Split with
   a terminal showing the signal log (`23 death_cross`, `49 golden_cross`, …) so
   the viewer sees each LED change line up with an EMA cross. Caption:
   *"EMA(9)/EMA(21) cross → LED. The exact same signals the desktop library
   produces."*
4. **24–30s — the claim.** Text card: *"The same indicator core runs on your
   server, in your browser, and on this microcontroller — byte-for-byte."* End on
   the repo URL and `wickra.org`.

## Captions / copy

- One-liner: *"Wickra's technical-analysis core running `no_std` on a $5
  microcontroller — the LED blinks on the EMA cross."*
- Byte-for-byte: the on-device sequence equals `golden/expected/ema_cross.txt`
  (see [`../golden/README.md`](../golden/README.md)).

## Where to post

- Hacker News — *Show HN: Wickra's TA core running no_std on a $5 Pico*.
- Reddit — r/rust, r/embedded.
- Always link back to the repository and to https://wickra.org.

## Notes

- Film the LED at a shutter speed that captures the ~60 ms ticks cleanly.
- Keep the clip under 30s; the LED transitions at the five cross points are the
  entire story.
