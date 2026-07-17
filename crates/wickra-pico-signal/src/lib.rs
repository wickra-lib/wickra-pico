//! The shared, `no_std` EMA-cross signal engine for Wickra Pico.
//!
//! [`SignalEngine`] streams prices through an EMA(9)/EMA(21) cross built on
//! `embed-core`'s allocation-free indicator kernel and emits a [`Signal`] on
//! each cross. The crate is `#![no_std]` and scalar-only, so the identical code
//! runs in the microcontroller firmware and in the `std` host reference and unit
//! tests — that is what makes the demo both testable and cross-target
//! deterministic. The firmware contains no signal logic; it only calls
//! [`SignalEngine::on_tick`].
//!
//! ```
//! use wickra_pico_signal::SignalEngine;
//!
//! let mut engine = SignalEngine::new();
//! let mut last = None;
//! for price in [100.0, 101.0, 102.0, 103.0, 104.0] {
//!     if let Some(sig) = engine.on_tick(price) {
//!         last = Some(sig);
//!     }
//! }
//! let _ = last; // a cross may or may not have fired over this short warmup
//! ```
#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

mod engine;
mod signal;

pub use engine::{SignalEngine, FAST, SLOW};
pub use signal::Signal;
