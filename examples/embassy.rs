//! Print "Hello world!" with "Hello rust!" underneath. Uses the `embedded_graphics` crate to draw
//! the text with a 6x8 pixel font.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB9
//! (green)  SCL -> PB8
//! ```
//!
//! Run on a Blue Pill with `cargo run --example text`.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use panic_semihosting as _;
use sh1106::{prelude::*, Builder};

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embassy_time::Delay;



#[entry]
fn main() -> ! {

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.sys = Sysclk::HSI;
    }
    let p = embassy_stm32::init(config);

    let mut spi_config = embassy_stm32::spi::Config::default();
    spi_config.frequency = mhz(1);

    let cs = Output::new(p.PA2, Level::High, Speed::High);
    let dc = Output::new(p.PA3, Level::High, Speed::High);
    let mut reset = Output::new(p.PA4, Level::High, Speed::High);

    let spi = embassy_stm32::spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, spi_config);

    let spi = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay);

    let di = display_interface_spi::SPIInterface::new(spi, dc);

    let mut disp: GraphicsMode<_> = Builder::new()
            .with_size(crate::DisplaySize::Display128x128)
            //.with_size(crate::DisplaySize::Display128x64)
            .with_rotation(crate::DisplayRotation::Rotate180)
            .connect(di).into();

    let mut delay = Delay {};

    match disp.reset(&mut reset, &mut delay) {
        Ok(_) => {}
        Err(_) => {panic!();}
    };

    disp.init().unwrap();

    disp.clear();

    disp.flush().unwrap();

    let im: ImageRawLE<BinaryColor> = ImageRawLE::new(include_bytes!("./rust.raw"), 64);

    let (x_diff, y_diff) = {
        let dwidth = disp.get_dimensions().0 as i32;
        let dheight = disp.get_dimensions().1 as i32;
        let iwidth = im.size().width as i32;
        let iheight = im.size().height as i32;
        (dwidth - iwidth, dheight - iheight)
    };


    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();


    let mut dir = 1;
    let mut x = 0;
    loop {
        
        disp.clear();

        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut disp)
            .unwrap();

        Text::with_baseline("Hello Rust!", Point::new(0, disp.get_dimensions().1 as i32), text_style, Baseline::Bottom)
            .draw(&mut disp)
            .unwrap();

        Image::new(&im, Point::new(x, y_diff/2)).draw(&mut disp).unwrap();
        x += dir;
        if dir > 0 && x >= x_diff {
            dir = -1;
        } else if dir < 0 && x <= 0 {
            dir = 1;
        }
        disp.flush().unwrap();
    }
}

