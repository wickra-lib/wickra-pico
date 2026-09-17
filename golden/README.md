# Golden fixtures

The cross-target reference for the Wickra Pico demo. The signal sequence the
firmware produces on the device must equal the sequence here, byte-for-byte —
that is the cross-target-parity guarantee.

## Layout

- **`data/ema_cross.csv`** — the replay feed (`index,price`, 128 rows).
- **`expected/ema_cross.txt`** — the expected signal sequence, one
  `"<index> <token>"` line per cross.

Both are **generated — never hand-edit**; see [Blessing](#blessing).

## The feed formula

```
price(i) = 100.0 + 15.0 * sin(i / 8.0) + 0.05 * i     # computed in f32, i = 0..128
```

A drifting sine wave that produces a handful of clean EMA(9)/EMA(21) crosses.
The `f32` values widen losslessly to the `f64` the engine consumes, so the host
and the device see the same feed.

## Expected sequence

```
23 death_cross
49 golden_cross
75 death_cross
99 golden_cross
125 death_cross
```

On the device the on-board LED goes **off** at each `death_cross` and **on** at
each `golden_cross` — the five transitions are the visible demo.

## Why the parity holds

The firmware and the host reference run the **identical** `SignalEngine` (from
`wickra-pico-signal`) over the **identical** `FEED` (from `embedded-data`), with
the identical `f64::from(price)` widening. The engine is scalar-only and
allocation-free with no platform-specific floating-point paths, so the two
targets cannot diverge. This is verified two ways today:

1. **Host parity** — `cargo run -p wickra-pico-host -- check` recomputes the
   sequence and asserts it equals `expected/ema_cross.txt` byte-for-byte.
2. **Drift guard** — a `wickra-pico-host` test pins the committed `embedded-data`
   `FEED` (bit-for-bit) to the formula output, so the feed the firmware compiles
   in cannot drift from the one the golden was blessed from.

An additional *empirical* on-device check (running the firmware ELF under a
Renode/QEMU RP2040 simulation and capturing the sequence over RTT) is tracked in
[`../ROADMAP.md`](../ROADMAP.md); the guarantee above already pins the sequence.

## Blessing

Regenerate both files (and the embedded `FEED` const) from the single feed
formula; the host-side test and the firmware image then agree on the same bytes:

```bash
cargo run -p wickra-pico-host -- bless
```
