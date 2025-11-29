//! FILE: main.rs
//!
//! DESCRIPTION:
//! RP2350 Embedded Rust Embassy Blink Application.
//! 
//! BRIEF:
//! Main application entry point for RP2350 GPIO blink driver using Embassy.
//! Implements async LED blinking on GPIO 16.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: November 28, 2025
//! UPDATE DATE: November 29, 2025

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use panic_halt as _;

/// Main application entry point.
///
/// # Details
/// Implements the infinite async blink loop on GPIO 16.
/// LED toggles every 500ms using Embassy async runtime.
///
/// # Arguments
/// * `_spawner` - Embassy task spawner (unused)
///
/// # Returns
/// Never returns (infinite loop)
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_16, Level::Low);

    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
