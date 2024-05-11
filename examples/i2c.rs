//! Print "Hello world!". Uses the `embedded_graphics` crate to draw
//! the text with a 6x8 pixel font.
//!
//! This example is for the STM32G431 board connected to a SH1107 based display via I2C.
//!
//! Example Wiring:
//!
//! ```
//!      Display -> Board
//!         SDA     PA8
//!         SCL     PA9
//!         GND     GND
//!         VCC     3v3
//!
//! Tested using a WeAct studios STM32G431CBU6 board.
//!
//! ```
//! Run with: `cargo run --example i2c --features=embassy-stm32 --release`.

#![no_std]
#![no_main]
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, Config};
use embassy_time::Delay;

use embassy_executor::Spawner;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use embedded_hal::delay::DelayNs;
use oled_async::{prelude::*, Builder};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C2_EV => embassy_stm32::i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C2>;
    I2C2_ER => embassy_stm32::i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Use internal clock source
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.sys = Sysclk::HSI;
    }
    let p = embassy_stm32::init(config);

    let mut i2c_cfg = embassy_stm32::i2c::Config::default();
    i2c_cfg.sda_pullup = false;
    i2c_cfg.sda_pullup = false;

    type I2c = embassy_stm32::i2c::I2c<
        'static,
        embassy_stm32::peripherals::I2C2,
        embassy_stm32::mode::Async,
    >;
    type I2cInterface = display_interface_i2c::I2CInterface<I2c>;
    let i2c: I2c = embassy_stm32::i2c::I2c::new(
        p.I2C2,
        p.PA9, // SCK
        p.PA8, // SDA
        Irqs,
        p.DMA1_CH1,
        p.DMA1_CH2,
        Hertz(100_000),
        i2c_cfg,
    );

    let di: I2cInterface = display_interface_i2c::I2CInterface::new(
        i2c,  // I2C
        0x3C, // I2C Address
        0x40, // Databyte
    );

    let mut delay = Delay {};

    let raw_disp = Builder::new(oled_async::displays::sh1107::Sh1107_128_128 {})
        .with_rotation(crate::DisplayRotation::Rotate180)
        .connect(di);

    let mut disp: GraphicsMode<_, _> = raw_disp.into();

    disp.init().await.unwrap();
    disp.clear();
    disp.flush().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    disp.flush().await.unwrap();

    loop {
        delay.delay_ms(1000);
    }
}
