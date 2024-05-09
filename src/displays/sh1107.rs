//! SH1107 display variants and specifics

use crate::display::DisplayVariant;

/// Generic 128x128 with SH1107 controller
#[derive(Debug, Clone, Copy)]
pub struct Sh1107_128_128 {}

impl DisplayVariant for Sh1107_128_128 {
    fn width() -> u8 {
        128
    }
    fn height() -> u8 {
        128
    }
    fn column_offset() -> u8 {
        0
    }
}
