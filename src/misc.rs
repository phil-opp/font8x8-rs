//! A miscellanous set of characters.

/// A miscellanous set of characters.
///
/// * `U+20A7` (Spanish Pesetas/Pt)
/// * `U+0192` (latin small letter f with hook)
/// * `U+00AA` (feminine ordinal indicator ª)
/// * `U+00BA` (masculine ordinal indicator °)
/// * `U+2310` (reversed not sign)
/// * `U+2264` (less than or equal)
/// * `U+2265` (greater than or equal)
/// * `U+0060` (grave accent)
/// * `U+1EF2` (Y with grave)
/// * `U+1EF3` (y with grave)
pub const MISC: [[u8; 8]; 10] = [
    [0x1F, 0x33, 0x33, 0x5F, 0x63, 0xF3, 0x63, 0xE3],
    [0x70, 0xD8, 0x18, 0x3C, 0x18, 0x18, 0x1B, 0x0E],
    [0x3C, 0x36, 0x36, 0x7C, 0x00, 0x7E, 0x00, 0x00],
    [0x1C, 0x36, 0x36, 0x1C, 0x00, 0x3E, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x3F, 0x03, 0x03, 0x00, 0x00],
    [0x30, 0x18, 0x0C, 0x18, 0x30, 0x00, 0x7E, 0x00],
    [0x0C, 0x18, 0x30, 0x18, 0x0C, 0x00, 0x7E, 0x00],
    [0x0C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x0E, 0x00, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x00],
    [0x00, 0x07, 0x00, 0x33, 0x33, 0x3E, 0x30, 0x1F],
];
