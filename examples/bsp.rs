//! Abstract different board/chip/hal for examples
//!
//! ```

#[cfg(feature = "embassy-stm32")]
pub mod board {

    use embassy_stm32::gpio::{Level, Output, Speed};
    use embassy_stm32::Config;
    use embassy_time::Delay;

    #[cfg(feature = "i2c")]
    embassy_stm32::bind_interrupts!(struct Irqs {
        I2C2_EV => embassy_stm32::i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C2>;
        I2C2_ER => embassy_stm32::i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C2>;
    });

    pub type OutputPin = Output<'static>;

    #[cfg(feature = "spi")]
    pub mod types {
        pub type OutputPin = embassy_stm32::gpio::Output<'static>;
        type Spi = embassy_stm32::spi::Spi<
            'static,
            embassy_stm32::peripherals::SPI1,
            embassy_stm32::mode::Async,
        >;
        type SPIDevice =
            embedded_hal_bus::spi::ExclusiveDevice<Spi, OutputPin, embassy_time::Delay>;
        pub type DisplayInterface = display_interface_spi::SPIInterface<SPIDevice, OutputPin>;
    }

    #[cfg(feature = "i2c")]
    pub mod types {

        pub type I2c = embassy_stm32::i2c::I2c<
            'static,
            embassy_stm32::peripherals::I2C2,
            embassy_stm32::mode::Async,
        >;
        pub type DisplayInterface = display_interface_i2c::I2CInterface<I2c>;
    }

    pub fn get_board() -> (types::DisplayInterface, OutputPin, Delay) {
        let mut config = Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.sys = Sysclk::HSI;
        }
        let p = embassy_stm32::init(config);

        #[cfg(feature = "spi")]
        let di = {
            use embassy_stm32::time::mhz;

            let cs = Output::new(p.PA2, Level::High, Speed::High);
            let dc = Output::new(p.PA3, Level::High, Speed::High);
            let mut spi_config = embassy_stm32::spi::Config::default();
            spi_config.frequency = mhz(4);
            let spi = embassy_stm32::spi::Spi::new(
                p.SPI1, p.PA5, p.PA7, p.PA6, p.DMA1_CH1, p.DMA1_CH2, spi_config,
            );
            let spi = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, Delay);
            display_interface_spi::SPIInterface::new(spi, dc)
        };

        #[cfg(feature = "i2c")]
        let di = {
            use embassy_stm32::time::Hertz;

            let mut i2c_cfg = embassy_stm32::i2c::Config::default();
            i2c_cfg.sda_pullup = false;
            i2c_cfg.sda_pullup = false;

            let i2c = embassy_stm32::i2c::I2c::new(
                p.I2C2,
                p.PA9, // SCK
                p.PA8, // SDA
                Irqs,
                p.DMA1_CH1,
                p.DMA1_CH2,
                Hertz(100_000),
                i2c_cfg,
            );

            display_interface_i2c::I2CInterface::new(
                i2c,  // I2C
                0x3C, // I2C Address
                0x40, // Databyte
            )
        };

        let reset = Output::new(p.PA4, Level::High, Speed::High);

        (di, reset, Delay {})
    }
}
