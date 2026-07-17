//! The discrete cross signal emitted by the engine.

/// A moving-average cross signal.
///
/// Emitted by [`SignalEngine`](crate::SignalEngine) when the fast EMA crosses
/// the slow EMA. On the device a `GoldenCross` turns the LED on and a
/// `DeathCross` turns it off; on the host the [`token`](Signal::token) string is
/// written to the golden reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    /// The fast EMA crossed **above** the slow EMA (bullish).
    GoldenCross,
    /// The fast EMA crossed **below** the slow EMA (bearish).
    DeathCross,
}

impl Signal {
    /// A stable, machine-readable token for the signal.
    ///
    /// This is the exact string written to the host golden reference and logged
    /// over `defmt` on-device, so it is part of the cross-target contract.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Signal::GoldenCross => "golden_cross",
            Signal::DeathCross => "death_cross",
        }
    }
}
