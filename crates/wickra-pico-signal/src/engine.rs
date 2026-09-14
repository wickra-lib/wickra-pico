//! The allocation-free EMA-cross signal engine.
//!
//! This is the whole trading logic of the demo, and it is `no_std` and
//! scalar-only: two EMAs from `wickra-embed-core` plus the sign of their difference.
//! The firmware links this crate and calls [`SignalEngine::on_tick`] once per
//! price; it contains no signal logic of its own.

use wickra_embed_core::{Ema, Indicator};

use crate::signal::Signal;

/// Fast EMA period.
pub const FAST: usize = 9;
/// Slow EMA period.
pub const SLOW: usize = 21;

/// Streaming EMA(9)/EMA(21) cross detector.
///
/// Feed prices in one at a time with [`on_tick`](SignalEngine::on_tick); it
/// returns `Some(Signal)` on the tick where the fast EMA crosses the slow EMA
/// and `None` otherwise (including the whole warmup). The state is three scalars,
/// so it runs identically on a workstation and on a microcontroller.
pub struct SignalEngine {
    ema_fast: Ema,
    ema_slow: Ema,
    /// The last **non-zero** difference `fast - slow`, or `None` until both EMAs
    /// are ready and a non-zero difference has been seen. Keeping the last
    /// non-zero value (rather than the raw difference) makes an exact `0.0` tie
    /// a neutral hold instead of a spurious flip.
    prev_diff: Option<f64>,
}

impl SignalEngine {
    /// Construct a fresh engine with the default `FAST`/`SLOW` periods.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ema_fast: Ema::new(FAST),
            ema_slow: Ema::new(SLOW),
            prev_diff: None,
        }
    }

    /// Clear all state, leaving the engine exactly as freshly constructed.
    pub fn reset(&mut self) {
        self.ema_fast.reset();
        self.ema_slow.reset();
        self.prev_diff = None;
    }

    /// Feed one price and return a cross signal if one fired on this tick.
    ///
    /// Both EMAs are always updated (the side effects must happen every tick).
    /// A signal fires only once both are ready and the sign of `fast - slow`
    /// flips relative to the last non-zero difference.
    pub fn on_tick(&mut self, price: f64) -> Option<Signal> {
        let fast = self.ema_fast.update(price);
        let slow = self.ema_slow.update(price);
        let (Some(fast), Some(slow)) = (fast, slow) else {
            return None;
        };
        let diff = fast - slow;
        let signal = match self.prev_diff {
            Some(prev) => cross(prev, diff),
            None => None,
        };
        // Keep the last *non-zero* difference as the reference sign.
        if diff != 0.0 {
            self.prev_diff = Some(diff);
        }
        signal
    }
}

impl Default for SignalEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Classify a sign flip between the previous and current EMA difference.
///
/// A cross requires a strict sign change: a previously negative difference
/// turning strictly positive is a golden cross, and vice versa. An exact `0.0`
/// on either side yields no cross (the caller keeps the last non-zero reference).
fn cross(prev: f64, cur: f64) -> Option<Signal> {
    if prev < 0.0 && cur > 0.0 {
        Some(Signal::GoldenCross)
    } else if prev > 0.0 && cur < 0.0 {
        Some(Signal::DeathCross)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{cross, Signal};

    #[test]
    fn cross_truth_table() {
        // Golden: negative -> positive.
        assert_eq!(cross(-1.0, 1.0), Some(Signal::GoldenCross));
        // Death: positive -> negative.
        assert_eq!(cross(1.0, -1.0), Some(Signal::DeathCross));
        // No flip: same sign.
        assert_eq!(cross(-1.0, -2.0), None);
        assert_eq!(cross(2.0, 1.0), None);
    }

    #[test]
    fn zero_tie_is_neutral() {
        // Landing exactly on zero is never a cross from either side.
        assert_eq!(cross(-1.0, 0.0), None);
        assert_eq!(cross(1.0, 0.0), None);
        assert_eq!(cross(0.0, 1.0), None);
        assert_eq!(cross(0.0, -1.0), None);
        assert_eq!(cross(0.0, 0.0), None);
    }

    #[test]
    fn cross_is_bit_stable() {
        // The same inputs always classify the same way — no NaN, no drift.
        for _ in 0..3 {
            assert_eq!(cross(-0.000_1, 0.000_1), Some(Signal::GoldenCross));
            assert_eq!(cross(0.000_1, -0.000_1), Some(Signal::DeathCross));
        }
    }
}
