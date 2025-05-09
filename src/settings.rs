use bon::Builder;

use crate::error::SeiError;

/// Details how to decode the image data or stack it with other files.
#[derive(Debug, Clone, Copy, Builder, PartialEq, Eq, Default)]
pub struct SeiSettings {
    #[builder(default)]
    pub bit_depth: BitDepth,
    #[builder(default)]
    pub white_mode: WhiteMode,
    #[builder(default)]
    pub padding: Padding,
    #[builder(default)]
    pub stacking_mode: StackingMode,
    /// Reserved bits for future use
    #[builder(default)]
    pub unused_bits: u8,
}

impl SeiSettings {
    /// Parse the settings from a byte
    pub fn parse(settings: u8) -> Result<Self, SeiError> {
        let bit_depth = settings & 0b11;
        let white_mode = (settings >> 2) & 0b1;
        let padding = (settings >> 3) & 0b1;
        let stacking_mode = (settings >> 4) & 0b11;
        let unused_bits = (settings >> 6) & 0b11;

        Ok(SeiSettings {
            bit_depth: BitDepth::parse(bit_depth)?,
            white_mode: WhiteMode::parse(white_mode),
            stacking_mode: StackingMode::parse(stacking_mode)?,
            padding: Padding::parse(padding),
            unused_bits,
        })
    }
}

/// How many bits are used to represent each pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BitDepth {
    #[default]
    OneBit = 0b00,
    /// Unused in the current version of the SEI format
    TwoBits = 0b01,
    /// Unused in the current version of the SEI format
    ThreeBits = 0b10,
}

impl BitDepth {
    /// Get the number of bits used to represent each pixel.
    pub(crate) const fn bits(self) -> u32 {
        match self {
            BitDepth::OneBit => 1,
            BitDepth::TwoBits => 2,
            BitDepth::ThreeBits => 3,
        }
    }

    pub(crate) const fn parse(value: u8) -> Result<Self, SeiError> {
        Ok(match value {
            0 => BitDepth::OneBit,
            // 1 => BitDepth::TwoBits,
            // 2 => BitDepth::ThreeBits,
            _ => return Err(SeiError::UnsupportedBitDepth),
        })
    }
}

/// Whether for each pixel if all pixel bits are 0's or 1's should be treated as the white colour.
///
/// Zeros: All bits 0 => white. (E.g. in 2-bit mode, 00 is white and 11 is black)
/// Ones: All bits 1 => white. (E.g. in 3-bit mode, 111 is white and 000 is black)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WhiteMode {
    #[default]
    Zeros = 0,
    Ones = 1,
}

impl WhiteMode {
    pub(crate) const fn parse(value: u8) -> Self {
        match value {
            0 => WhiteMode::Zeros,
            1 => WhiteMode::Ones,
            // Only ever called from the parser with 0 or 1
            _ => unreachable!(),
        }
    }
}

/// Whether or not each row of pixels should be padded to the next byte boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Padding {
    #[default]
    Padding = 0,
    NoPadding = 1,
}

impl Padding {
    pub(crate) const fn parse(value: u8) -> Self {
        match value {
            0 => Padding::Padding,
            1 => Padding::NoPadding,
            // Only ever called from the parser with 0 or 1
            _ => unreachable!(),
        }
    }
}

/// How the image should be stacked relative to the Stack below it
/// Opaque: No transparency, all pixels overwrite the ones below them
/// WhiteTransparent: All white pixels act transparent, not overwriting the pixels below them.
/// BlackTransparent: All black pixels act transparent, not overwriting the pixels below them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StackingMode {
    #[default]
    Opaque = 0b00,
    WhiteTransparent = 0b01,
    BlackTransparent = 0b10,
}

impl StackingMode {
    pub(crate) const fn parse(value: u8) -> Result<Self, SeiError> {
        Ok(match value {
            0 => StackingMode::Opaque,
            1 => StackingMode::WhiteTransparent,
            2 => StackingMode::BlackTransparent,
            _ => return Err(SeiError::UnsupportedTransparency),
        })
    }
}

#[test]
fn test_sei_settings() {
    let settings: u8 = 0b0010_1100;
    let parsed = SeiSettings::parse(settings).unwrap();
    assert_eq!(
        parsed,
        SeiSettings::builder()
            .bit_depth(BitDepth::OneBit)
            .white_mode(WhiteMode::Ones)
            .padding(Padding::NoPadding)
            .stacking_mode(StackingMode::BlackTransparent)
            .build()
    );
}
