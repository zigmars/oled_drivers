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

mod bsp;

use embassy_executor::Spawner;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use panic_semihosting as _;
use sh1106::{prelude::*, Builder};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (di, mut reset, mut delay) = bsp::board::get_board();

    //type Display = sh1106::displays::sh1107::Sh1107_128_128;
    type Display = sh1106::displays::ssd1309::Ssd1309_128_64;
    let mut disp: GraphicsMode<_, _> = Builder::new(Display {})
        .with_rotation(crate::DisplayRotation::Rotate180)
        .connect(di)
        .into();

    disp.reset(&mut reset, &mut delay).unwrap();
    disp.init().await.unwrap();
    disp.clear();
    disp.flush().await.unwrap();

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

        Text::with_baseline(
            "Hello Rust!",
            Point::new(0, disp.get_dimensions().1 as i32),
            text_style,
            Baseline::Bottom,
        )
        .draw(&mut disp)
        .unwrap();

        Image::new(&im, Point::new(x, y_diff / 2))
            .draw(&mut disp)
            .unwrap();
        x += dir;
        if dir > 0 && x >= x_diff {
            dir = -1;
        } else if dir < 0 && x <= 0 {
            dir = 1;
        }
        disp.flush().await.unwrap();
    }
}
