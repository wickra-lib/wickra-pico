//! Wickra Pico firmware for the Raspberry Pi Pico (RP2040).
//!
//! Streams the embedded replay feed tick-by-tick through the shared
//! [`SignalEngine`] and drives the on-board LED (GPIO25): a golden cross turns
//! it on, a death cross turns it off. The firmware contains no signal logic —
//! it is HAL glue around the same `no_std` engine the host reference runs, so
//! the on-device signal sequence is identical to `golden/expected/ema_cross.txt`.

#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
};

use wickra_pico_signal::{Signal, SignalEngine};

/// Second-stage bootloader for the Pico's on-board W25Q080 flash.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp_pico::BOOT2_FIRMWARE;

/// Milliseconds between ticks — slow enough that the LED transitions are visible.
const TICK_MS: u32 = 60;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led = pins.led.into_push_pull_output();

    // The whole demo: fold the const feed through the engine, toggle the LED on
    // each cross. `f64::from(price)` matches the host reference exactly.
    let mut engine = SignalEngine::new();
    for &price in embedded_data::FEED.iter() {
        match engine.on_tick(f64::from(price)) {
            Some(Signal::GoldenCross) => led.set_high().unwrap(),
            Some(Signal::DeathCross) => led.set_low().unwrap(),
            None => {}
        }
        delay.delay_ms(TICK_MS);
    }

    // Feed exhausted: hold the final LED state and idle.
    loop {
        cortex_m::asm::wfi();
    }
}
