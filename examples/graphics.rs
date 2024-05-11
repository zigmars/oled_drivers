//! Draw a square, circle and triangle on the screen using the `embedded_graphics` crate.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
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
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
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

    let mut display: GraphicsMode<_, _> = raw_disp.into();

    display.reset(&mut reset, &mut delay).unwrap();
    display.init().await.unwrap();
    display.clear();
    display.flush().await.unwrap();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Rectangle::with_corners(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Circle::new(Point::new(88, 16), 16)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    display.flush().await.unwrap();

    loop {
        delay.delay_ms(1000).await;
    }
}
