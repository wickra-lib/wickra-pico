---
name: Bug report (Detailed)
about: Long-form bug report with environment matrix, minimal reproducer, and expected-vs-actual sections.
title: "[Bug] <short description>"
labels: ["bug", "triage"]
assignees: []
---

## Summary

<!-- One or two sentences. What did you expect, what happened instead? -->

## Affected part

- [ ] The signal kernel (`crates/wickra-pico-signal`)
- [ ] The host golden generator / parity checker (`crates/wickra-pico-host`)
- [ ] The RP2040 firmware (`firmware/rp-pico`)
- [ ] The embedded data feed (`embedded-data`)
- [ ] Docs only

## Environment

| Field                | Value                                  |
| -------------------- | -------------------------------------- |
| Wickra Pico version  | `e.g. 0.1.0`                           |
| Board                | `e.g. Raspberry Pi Pico (RP2040), Pico W` |
| Host OS / arch       | `e.g. Windows 11 x86_64, Linux glibc`  |
| Rust toolchain       | `rustc --version`, `rustup target list --installed` |
| Flashing tool        | `e.g. UF2 drag-and-drop, probe-rs, elf2uf2-rs version` |

## Minimal reproducer

<!--
Paste the smallest possible code snippet that triggers the bug.
If the input data matters, attach a CSV/JSON or paste a few rows inline.
-->

```rust
use wickra_pico_signal::SignalEngine;
let mut engine = SignalEngine::new();
// the ticks that trigger it
```

## Actual output

```
<paste stack trace, panic, wrong values, etc.>
```

## Expected output

<!-- What should the kernel have signalled? The golden series in golden/ and the std wickra-core are the references. -->

## Additional context

<!-- Logs, screenshots, links to related issues, anything else useful. -->
