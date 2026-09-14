//! The per-update cost of the signal kernel on the host, the number
//! BENCHMARKS.md reports beside the on-device cycle count.
//!
//! `on_tick` past the EMA warmup is the steady-state O(1) path the firmware
//! runs on every sample; the bench feeds a fixed synthetic price walk so the
//! branch the cross takes is the same on every run. `cargo bench` measures it
//! as criterion; under `cargo codspeed` the same source reports instructions.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wickra_pico_signal::{SignalEngine, SLOW};

/// A deterministic price walk: a slow sine over a drift, long enough to cross
/// the fast/slow pair in both directions.
fn prices(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            // 4096 ticks at most: exact in an f64.
            let t = f64::from(u32::try_from(i).expect("walk length fits u32"));
            100.0 + 0.05 * t + 4.0 * (t * 0.11).sin()
        })
        .collect()
}

fn bench_on_tick(c: &mut Criterion) {
    let walk = prices(4096);
    c.bench_function("on_tick steady state", |b| {
        let mut engine = SignalEngine::new();
        // Past the warmup, so the measured call is the O(1) path.
        for p in &walk[..SLOW * 2] {
            engine.on_tick(*p);
        }
        let mut i = SLOW * 2;
        b.iter(|| {
            let out = engine.on_tick(black_box(walk[i]));
            i = (i + 1) % walk.len();
            if i == 0 {
                i = SLOW * 2;
            }
            black_box(out)
        });
    });
    c.bench_function("on_tick from reset over 4096 ticks", |b| {
        b.iter(|| {
            let mut engine = SignalEngine::new();
            let mut crosses = 0usize;
            for p in &walk {
                if engine.on_tick(black_box(*p)).is_some() {
                    crosses += 1;
                }
            }
            black_box(crosses)
        });
    });
}

criterion_group!(benches, bench_on_tick);
criterion_main!(benches);
