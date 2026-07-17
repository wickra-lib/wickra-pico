//! Host tests for the shared signal engine. The engine is `no_std`, but these
//! run on the host with `std` so we can use `proptest` and `Vec`.

use proptest::prelude::*;
use wickra_pico_signal::{Signal, SignalEngine};

/// Run a price path through a fresh engine and collect the signals it emits.
fn run(prices: &[f64]) -> Vec<Signal> {
    let mut engine = SignalEngine::new();
    let mut out = Vec::new();
    for &p in prices {
        if let Some(sig) = engine.on_tick(p) {
            out.push(sig);
        }
    }
    out
}

#[test]
fn warmup_emits_nothing() {
    // A flat price can never produce a cross: the EMAs converge and never flip.
    let mut engine = SignalEngine::new();
    for _ in 0..100 {
        assert_eq!(engine.on_tick(100.0), None);
    }
}

#[test]
fn golden_then_death_in_order() {
    // Establish the fast EMA *below* the slow with an initial decline, then rise
    // (fast overtakes slow -> golden), then fall again (fast drops below ->
    // death). Each monotonic leg crosses exactly once.
    let mut prices = Vec::new();
    for i in 0..50 {
        prices.push(100.0 - 0.8 * f64::from(i)); // decline 100 -> ~60
    }
    for i in 0..80 {
        prices.push(60.0 + f64::from(i)); // rise 60 -> 139
    }
    for i in 0..80 {
        prices.push(140.0 - f64::from(i)); // decline 140 -> 61
    }
    let signals = run(&prices);
    assert_eq!(
        signals,
        vec![Signal::GoldenCross, Signal::DeathCross],
        "a decline, a rise, then a decline should fire exactly one golden then one death cross"
    );
}

#[test]
fn reset_clears_state() {
    let mut engine = SignalEngine::new();
    // Drive a golden cross.
    for i in 0..30 {
        engine.on_tick(100.0 + f64::from(i));
    }
    engine.reset();
    // After reset the engine is fresh: a flat feed emits nothing again.
    for _ in 0..100 {
        assert_eq!(engine.on_tick(100.0), None);
    }
}

proptest! {
    /// No two identical crosses fire in a row without an opposite cross between
    /// them — a golden must be followed by a death before another golden.
    #[test]
    fn crosses_alternate(prices in prop::collection::vec(50.0f64..150.0, 0..400)) {
        let signals = run(&prices);
        for pair in signals.windows(2) {
            prop_assert_ne!(pair[0], pair[1], "two identical crosses in a row");
        }
    }

    /// The engine is deterministic: the same path always yields the same signals.
    #[test]
    fn deterministic(prices in prop::collection::vec(50.0f64..150.0, 0..400)) {
        prop_assert_eq!(run(&prices), run(&prices));
    }
}
