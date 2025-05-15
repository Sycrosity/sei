use bon::Builder;

use crate::settings::SeiSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Builder)]
pub struct SeiHeader {
    #[builder(default = 1)]
    pub version: u8,
    /// Offset to the start of the image data
    /// uses usize to be used as an index for the byte array
    #[builder(default = 0x0A)]
    pub data_offset: usize,
    pub width: u16,
    pub height: u16,
    #[builder(default)]
    pub settings: SeiSettings,
    #[builder(default)]
    pub z_index: u8,
}

impl SeiHeader {
    pub const fn bit_depth(&self) -> u32 {
        self.settings.bit_depth.bits()
    }

    pub const fn pixels_per_byte(&self) -> u32 {
        8 / self.bit_depth()
    }

    // Width of data based on the padding
    pub const fn data_width(&self) -> u32 {
        if self.settings.padding.padding() {
            (self.bit_depth() * self.width as u32).div_ceil(8) * self.pixels_per_byte()
        } else {
            self.width as u32
        }
    }
}
