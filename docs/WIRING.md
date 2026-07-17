# Wiring

## Raspberry Pi Pico — no wiring required

The demo drives the Pico's **on-board LED on GPIO25**. Just plug the board into
USB and flash it — there is nothing to wire.

### Optional: an external LED

If you want a bigger/brighter LED for the video, drive one from any spare GPIO
(the firmware uses GPIO25; change the pin in `firmware/rp-pico/src/main.rs` to
use an external one). Wire it with a current-limiting resistor:

```
   GPIO ──►|── 330Ω ──┐
          LED         │
                      ▼
                     GND
```

- Long leg (anode) toward the GPIO through the resistor, short leg (cathode) to
  a ground pin. 220–470Ω all work.

## ESP32 (optional, roadmap)

Most ESP32 DevKit boards have an on-board LED on **GPIO2**; confirm against your
specific board's schematic before relying on it.
