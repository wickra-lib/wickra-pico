//! `no_std` signal wrapper over the Wickra `embed-core` indicator kernel.
//!
//! The signal logic (an EMA(9)/EMA(21) cross driving a discrete signal) lands in
//! P-PICO-1; this scaffold pins the crate so the host workspace resolves.
#![no_std]
