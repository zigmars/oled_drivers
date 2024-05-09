//! Interface factory
//!
//! This is the easiest way to create a driver instance. You can set various parameters of the
//! driver and give it an interface to use. The builder will return a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which you should coerce to a richer
//! display mode, like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation (0 deg) and size (128x64):
//!
//! ```rust,no_run
//! use sh1106::{mode::GraphicsMode, Builder};
//! let spi = /* SPI interface from your HAL of choice */
//! # sh1106::test_helpers::SpiStub;
//! let dc = /* GPIO data/command select pin */
//! # sh1106::test_helpers::PinStub;
//!
//! // This example does not use a Chip Select pin
//! let cs = sh1106::builder::NoOutputPin::new();
//!
//! Builder::new().connect_spi(spi, dc, cs);
//! ```
//!
//! Connect over I2C, changing lots of options
//!
//! ```rust,no_run
//! use sh1106::{displayrotation::DisplayRotation, displaysize::DisplaySize, Builder};
//!
//! let i2c = /* I2C interface from your HAL of choice */
//! # sh1106::test_helpers::I2cStub;
//!
//! Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .with_i2c_addr(0x3D)
//!     .with_size(DisplaySize::Display128x32)
//!     .connect_i2c(i2c);
//! ```
//!
//! The above examples will produce a [RawMode](../mode/raw/struct.RawMode.html) instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`GraphicsMode` mode](../mode/graphics/struct.GraphicsMode.html):
//!
//! ```rust,no_run
//! use sh1106::{mode::GraphicsMode, Builder};
//! let spi = /* SPI interface from your HAL of choice */
//! # sh1106::test_helpers::SpiStub;
//! let dc = /* GPIO data/command select pin */
//! # sh1106::test_helpers::PinStub;
//!
//! // This example does not use a Chip Select pin
//! let cs = sh1106::builder::NoOutputPin::new();
//!
//! let display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc, cs).into();
//! ```

use display_interface::AsyncWriteOnlyDataCommand;
use hal::digital::OutputPin;

use crate::{
    displayrotation::DisplayRotation,
    mode::{displaymode::DisplayMode, raw::RawMode},
    properties::DisplayProperties,
};

/// Builder struct. Driver options and interface are set using its methods.
///
/// See the [module level documentation](crate::builder) for more details.
#[derive(Clone, Copy)]
pub struct Builder<DV> {
    variant: DV,
    rotation: DisplayRotation,
}

impl<DV> Builder<DV> {
    /// Create new builder with a default size of 128 x 64 pixels and no rotation.
    pub fn new(variant: DV) -> Builder<DV> {
        Builder::<DV> {
            variant,
            rotation: DisplayRotation::Rotate0,
        }
    }
}

impl<DV> Builder<DV> {
    /// Set the rotation of the display to one of four values. Defaults to no rotation.
    pub fn with_rotation(self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..self }
    }

    /// Finish the builder and use the given interface to communicate with the display.
    pub fn connect<DI>(self, interface: DI) -> DisplayMode<RawMode<DV, DI>>
    where
        DI: AsyncWriteOnlyDataCommand,
        DV: crate::display::DisplayVariant,
    {
        let properties = DisplayProperties::new(self.variant, interface, self.rotation);
        DisplayMode::<RawMode<DV, DI>>::new(properties)
    }
}

/// Marker type for no reset pin.
#[derive(Clone, Copy)]
pub enum NoOutputPin {}

impl OutputPin for NoOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl hal::digital::ErrorType for NoOutputPin {
    type Error = core::convert::Infallible;
}

#[cfg(test)]
mod tests {
    use super::NoOutputPin;
    use embedded_hal::digital::v2::OutputPin;

    enum SomeError {}

    struct SomeDriver<P: OutputPin<Error = SomeError>> {
        #[allow(dead_code)]
        p: P,
    }

    #[test]
    fn test_output_pin() {
        let p = NoOutputPin::new();
        let _d = SomeDriver { p };

        assert!(true);
    }
}
