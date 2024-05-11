//! Print "Hello world!". Uses the `embedded_graphics` crate to draw
//! the text with a 6x8 pixel font.
//!
//! This example is for the STM32G431 board connected to a SH1107 based display via SPI.
//!
//! Example Wiring:
//!
//! ```
//!      Display -> Board
//!         SCK     PA5
//!         MOSI    PA7
//!         MISO    PA6
//!         CS      PA2
//!         DC      PA3
//!         RESET   PA4
//!         GND     GND
//!         VCC     3v3
//!
//! Tested using a WeAct studios STM32G431CBU6 board.
//!
//! ```
//! Run with: `cargo run --example spi --features=embassy-stm32 --release`.

#![no_std]
#![no_main]
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Use internal clock source
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.sys = Sysclk::HSI;
    }
    let p = embassy_stm32::init(config);

    // Select 4MHz SPI clock
    let mut spi_config = embassy_stm32::spi::Config::default();
    spi_config.frequency = mhz(4);

    // Init cs, dc and reset I/O pins
    let cs = Output::new(p.PA2, Level::High, Speed::High);
    let dc = Output::new(p.PA3, Level::High, Speed::High);
    let mut reset = Output::new(p.PA4, Level::High, Speed::High);

    // Initialise the SPI peripheral
    let spi = embassy_stm32::spi::Spi::new(
        p.SPI1, p.PA5, p.PA7, p.PA6, p.DMA1_CH1, p.DMA1_CH2, spi_config,
    );

    // Wrap SPI peripheral with CS pin to represent a single device on SPI bus
    let spi = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay);

    // Wrap SPI device in display-interfce along with Data/Command pin
    let di = display_interface_spi::SPIInterface::new(spi, dc);

    let mut delay = Delay {};

    let raw_disp = Builder::new(oled_async::displays::sh1107::Sh1107_128_128 {})
        .with_rotation(crate::DisplayRotation::Rotate180)
        .connect(di);

    let mut disp: GraphicsMode<_, _> = raw_disp.into();

    disp.reset(&mut reset, &mut delay).unwrap();
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
