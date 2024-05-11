//! Draw a 1 bit per pixel black and white image.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! This example is tested with an STM32G431 board connected to a SH1107 based display via SPI or I2C.
//!
//! It should be easy to modify:
//!  - Display type: Choose below
//!  - Chip/board, if STM32:
//!       - Modify chip in Cargo.toml
//!       - Choose different periperals and pins in bsp.rs
//!
//! Run with: `cargo run --example embassy --features=embassy-stm32 --features=spi --release`.
//! or
//! Run with: `cargo run --example embassy --features=embassy-stm32 --features=i2c --release`.
//!

#![no_std]
#![no_main]

mod bsp;

use embassy_executor::Spawner;
use embedded_hal_async::delay::DelayNs;
use oled_async::{prelude::*, Builder};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (di, mut reset, mut delay) = bsp::board::get_board();

    type Display = oled_async::displays::sh1107::Sh1107_128_128;
    //type Display = oled_async::displays::sh1108::Sh1108_64_160;
    //type Display = oled_async::displays::ssd1309::Ssd1309_128_64;

    let raw_disp = Builder::new(Display {})
        .with_rotation(crate::DisplayRotation::Rotate180)
        .connect(di);

    let mut display: GraphicsMode<_, _> = raw_disp.into();

    display.reset(&mut reset, &mut delay).unwrap();
    display.init().await.unwrap();
    display.clear();
    display.flush().await.unwrap();

    // Top side
    display.set_pixel(0, 0, 1);
    display.set_pixel(1, 0, 1);
    display.set_pixel(2, 0, 1);
    display.set_pixel(3, 0, 1);

    // Right side
    display.set_pixel(3, 0, 1);
    display.set_pixel(3, 1, 1);
    display.set_pixel(3, 2, 1);
    display.set_pixel(3, 3, 1);

    // Bottom side
    display.set_pixel(0, 3, 1);
    display.set_pixel(1, 3, 1);
    display.set_pixel(2, 3, 1);
    display.set_pixel(3, 3, 1);

    // Left side
    display.set_pixel(0, 0, 1);
    display.set_pixel(0, 1, 1);
    display.set_pixel(0, 2, 1);
    display.set_pixel(0, 3, 1);

    display.flush().await.unwrap();

    loop {
        delay.delay_ms(1000).await;
    }
}
