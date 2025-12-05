<img src="https://github.com/mytechnotalent/RP2350_Rust_Blink_Driver/blob/main/RP2350_Rust_Blink_Driver.png?raw=true">

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)
### VIDEO PROMO [HERE](https://www.youtube.com/watch?v=aD7X9sXirF8)

<br>

# RP2350 Rust Blink Driver
An RP2350 blink driver written in Rust w/ Embassy.

<br>

# Install ARM Toolchain
## NOTE: Be SURE to select `Add path to environment variable` on setup.
[HERE](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)

<br>

# Hardware
## Raspberry Pi Pico 2 w/ Header [BUY](https://www.pishop.us/product/raspberry-pi-pico-2-with-header)
## USB A-Male to USB Micro-B Cable [BUY](https://www.pishop.us/product/usb-a-male-to-usb-micro-b-cable-6-inches)
## Raspberry Pi Pico Debug Probe [BUY](https://www.pishop.us/product/raspberry-pi-debug-probe)
## Complete Component Kit for Raspberry Pi [BUY](https://www.pishop.us/product/complete-component-kit-for-raspberry-pi)
## 10pc 25v 1000uF Capacitor [BUY](https://www.amazon.com/Cionyce-Capacitor-Electrolytic-CapacitorsMicrowave/dp/B0B63CCQ2N?th=1)
### 10% PiShop DISCOUNT CODE - KVPE_HS320548_10PC

<br>

# Build
```
cargo run
```

<br>

# Clean
```
cargo clean
```

<br>

# main.rs Code
```rust
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
```

<br>

# License
[Apache License 2.0](https://github.com/mytechnotalent/RP2350_Rust_Blink_Driver/blob/main/LICENSE)
