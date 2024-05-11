//! Print:
//!
//!  Hello world!
//!
//!     <<=== (R) ===>>  // Moving RUST logo
//!
//!  Hello rust!
//!
//! the text with a 6x8 pixel font.
//!
//! This example is tested with an STM32G431 board connected to a SH1107 based display via SPI or I2C.
//!
//! It should be easy to modify:
//!  - Display type: Choose below
//!  - Chip/board, if STM32:
//!       - Modify chip in Cargo.toml
//!       - Choose different periperals and pins in bsp.rs
//!
//! Run with: `cargo run --example image --features=embassy-stm32 --features=spi --release`.
//! or
//! Run with: `cargo run --example image --features=embassy-stm32 --features=i2c --release`.
//!

#![no_std]
#![no_main]

mod bsp;

use embassy_executor::Spawner;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
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

    let mut display: GraphicsMode<_, _, { 128 * 128 / 8 }> = raw_disp.into();

    display.reset(&mut reset, &mut delay).unwrap();
    display.init().await.unwrap();
    display.clear();
    display.flush().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().await.unwrap();

    loop {
        delay.delay_ms(1000).await;
    }
}
