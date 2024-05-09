//! Display variant

use display_interface::{AsyncWriteOnlyDataCommand, DisplayError};

/// Trait to represent a speciffic display
pub trait DisplayVariant {
    /// Get integral dimensions from DisplaySize
    fn dimensions() -> (u8, u8) {
        (Self::width(), Self::height())
    }

    /// Width of display
    fn width() -> u8;

    /// Height of display
    fn height() -> u8;

    /// Coumn offset
    fn column_offset() -> u8;

    /// Initialise the display for column mode
    #[allow(async_fn_in_trait)]
    async fn init_column_mode<DI>(iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand;
}
