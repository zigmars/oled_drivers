//! Display variant

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
}
