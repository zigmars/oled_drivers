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

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use panic_semihosting as _;
use sh1106::{prelude::*, Builder};
use stm32g4xx_hal::{
    delay::DelayFromCountDownTimer,
    gpio::gpioa::PA5,
    gpio::gpioa::PA6,
    gpio::gpioa::PA7,
    gpio::Alternate,
    gpio::AF5,
    prelude::*,
    rcc::Config,
    //i2c::{BlockingI2c, DutyCycle, Mode},
    spi::Spi,
    stm32::Peripherals,
    timer::Timer,
};

use embedded_hal::spi;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut rcc = rcc.freeze(Config::hsi());
    let timer2 = Timer::new(dp.TIM2, &rcc.clocks);
    let mut delay_tim2 = DelayFromCountDownTimer::new(timer2.start_count_down(100.ms()));

    let gpioa = dp.GPIOA.split(&mut rcc);

    let sclk: PA5<Alternate<AF5>> = gpioa.pa5.into_alternate();
    let miso: PA6<Alternate<AF5>> = gpioa.pa6.into_alternate();
    let mosi: PA7<Alternate<AF5>> = gpioa.pa7.into_alternate();

    let dc = gpioa.pa3.into_push_pull_output();
    let cs = gpioa.pa2.into_push_pull_output();
    let mut reset = gpioa.pa4.into_push_pull_output();

    let spi = Spi::spi1(
        dp.SPI1,
        (sclk, miso, mosi),
        //&mut afio.mapr,
        spi::MODE_0,
        4.mhz(),
        //clocks,
        //&mut rcc.apb2,
        &mut rcc,
    );

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(crate::DisplaySize::Display128x128)
        //.with_size(crate::DisplaySize::Display128x64)
        .with_rotation(crate::DisplayRotation::Rotate180)
        .connect_spi(spi, dc, cs)
        .into();

    match disp.reset(&mut reset, &mut delay_tim2) {
        Ok(_) => {}
        Err(_) => {
            panic!();
        }
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
        disp.flush().unwrap();

        delay_tim2.delay_ms(10_u16);
    }
}
