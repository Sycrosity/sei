//! `sei` - A .SEI (stacking ePaper image) file parser.
//!
//! no_std-compatible library for parsing SEI files for use with embedded_graphics.

#![cfg_attr(not(test), no_std)]

pub mod error;
pub mod raw_sei;
pub mod settings;

use crate::error::SeiError;
use crate::settings::SeiSettings;

use bon::Builder;
use core::marker::PhantomData;
use raw_sei::RawSei;
// use embedded_graphics_core
use embedded_graphics::{image::ImageRawLE, pixelcolor::BinaryColor, primitives::Rectangle};
use embedded_graphics_core::prelude::*;
use settings::BitDepth;

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

pub struct Sei<'a, C> {
    raw_sei: RawSei<'a>,
    color_type: PhantomData<C>,
}

impl<'a, C> Sei<'a, C>
where
    // C: PixelColor + From<RawU2> + From<Gray2> + From<BinaryColor> + From<RawU4>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    pub fn parse(bytes: &'a [u8]) -> Result<Self, SeiError> {
        let raw_sei = RawSei::parse(bytes)?;

        Ok(Self {
            raw_sei,
            color_type: PhantomData,
        })
    }

    pub fn as_raw(&self) -> &RawSei<'a> {
        &self.raw_sei
    }

    pub fn to_raw(self) -> RawSei<'a> {
        self.raw_sei
    }
}

// impl<C> ImageDrawable for Sei<'_, C>
// where
//     // C: PixelColor + From<RawU2> + From<Gray2> + From<BinaryColor> + From<RawU4>,
//     C: PixelColor + From<<C as PixelColor>::Raw>,
// {
impl ImageDrawable for Sei<'_, BinaryColor> {
    type Color = BinaryColor;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self.raw_sei.header.settings.bit_depth {
            BitDepth::OneBit => {
                ImageRawLE::<BinaryColor>::new(self.raw_sei.data, self.raw_sei.width()).draw(target)
            }

            _ => todo!(),
        }
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self.raw_sei.header.settings.bit_depth {
            BitDepth::OneBit => {
                ImageRawLE::<BinaryColor>::new(self.raw_sei.data, self.raw_sei.width())
                    .draw_sub_image(target, area)
            }

            _ => todo!(),
        }
    }
}

impl<C> OriginDimensions for Sei<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    // C: PixelColor + From<RawU2> + From<Gray2> + From<BinaryColor> + From<RawU4>,
{
    fn size(&self) -> Size {
        Size::new(self.raw_sei.width(), self.raw_sei.height())
    }
}
