#![no_main]
//! Fuzz the signal kernel: an arbitrary tick stream -- any f64, including NaN,
//! the infinities and subnormals -- through `on_tick` must never panic, and
//! the same stream through a second engine must yield the same signals: the
//! determinism the firmware and the host golden rely on.

use libfuzzer_sys::fuzz_target;
use wickra_pico_signal::SignalEngine;

fuzz_target!(|ticks: Vec<f64>| {
    let mut a = SignalEngine::new();
    let mut b = SignalEngine::new();
    for &price in &ticks {
        let sa = a.on_tick(price);
        let sb = b.on_tick(price);
        assert_eq!(sa, sb, "two engines fed the same stream disagree");
    }
    // A reset engine replays identically.
    a.reset();
    let mut c = SignalEngine::new();
    for &price in &ticks {
        assert_eq!(a.on_tick(price), c.on_tick(price), "reset does not replay");
    }
});
