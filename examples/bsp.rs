



#[cfg(feature = "embassy-stm32")]
pub mod board {

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embassy_time::Delay;

pub type OutputPin = Output<'static>;
type Spi = embassy_stm32::spi::Spi<'static, embassy_stm32::peripherals::SPI1, embassy_stm32::mode::Blocking>;
type Device = embedded_hal_bus::spi::ExclusiveDevice<Spi, OutputPin, embassy_time::Delay>;
type DisplayInterface = display_interface_spi::SPIInterface<Device, OutputPin>;

pub fn get_board() -> (DisplayInterface, OutputPin, Delay) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.sys = Sysclk::HSI;
    }
    let p = embassy_stm32::init(config);

    let mut spi_config = embassy_stm32::spi::Config::default();
    spi_config.frequency = mhz(4);

    let cs = Output::new(p.PA2, Level::High, Speed::High);
    let dc = Output::new(p.PA3, Level::High, Speed::High);
    let reset = Output::new(p.PA4, Level::High, Speed::High);

    let spi: Spi = embassy_stm32::spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, spi_config);
    let spi: Device = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay);
    let di: DisplayInterface = display_interface_spi::SPIInterface::new(spi, dc);

    (di, reset, Delay{})
}

}