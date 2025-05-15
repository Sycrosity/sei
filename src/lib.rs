//! `sei` - A .SEI (stacking ePaper image) file parser.
//!
//! no_std-compatible library for parsing SEI files for use with embedded_graphics.

#![cfg_attr(not(test), no_std)]

pub mod error;
pub mod header;
pub mod iter;
pub mod raw_sei;
pub mod settings;
// pub mod stack;

use crate::error::SeiError;

use iter::Pixels;
use raw_sei::RawSei;

use core::marker::PhantomData;

use embedded_graphics::{
    draw_target::DrawTarget, draw_target::DrawTargetExt, pixelcolor::BinaryColor,
    prelude::ContiguousIteratorExt, prelude::*, primitives::Rectangle,
};
use header::SeiHeader;
use settings::*;

#[derive(Clone, Copy)]
pub struct Sei<'a, C> {
    raw_sei: RawSei<'a>,
    color_type: PhantomData<C>,
}

impl<C> core::fmt::Debug for Sei<'_, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Sei")
            .field("raw_sei", &self.raw_sei)
            .finish()
    }
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

    pub const fn as_raw(&self) -> &RawSei<'a> {
        &self.raw_sei
    }

    pub const fn to_raw(self) -> RawSei<'a> {
        self.raw_sei
    }

    fn data_width(&self) -> u32 {
        self.raw_sei.data_width()
    }

    pub const fn set_stacking_mode(&mut self, stacking_mode: StackingMode) {
        self.raw_sei.header.settings.stacking_mode = stacking_mode;
    }

    pub const fn stacking_mode(&self) -> StackingMode {
        self.raw_sei.header.settings.stacking_mode
    }

    pub const fn set_invert(&mut self, invert: bool) {
        self.raw_sei.header.settings.invert = invert;
    }

    pub const fn invert(&mut self) {
        self.raw_sei.header.settings.invert = !self.raw_sei.header.settings.invert;
    }

    pub const fn inverted(&self) -> bool {
        self.raw_sei.header.settings.invert
    }

    pub fn bit_depth(&self) -> BitDepth {
        self.raw_sei.header.settings.bit_depth
    }

    pub fn width(&self) -> u32 {
        self.raw_sei.width()
    }

    pub fn height(&self) -> u32 {
        self.raw_sei.height()
    }
}

impl ImageDrawable for Sei<'_, BinaryColor> {
    type Color = BinaryColor;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let raw_data = Pixels::<BinaryColor>::new(self.raw_sei);

        target.draw_iter(
            raw_data
                .into_pixels(&self.bounding_box())
                .filter(|Pixel(_, colour)| {
                    // if self.inverted() {
                    //     BinaryColor::Off
                    // }

                    let colour = if self.inverted() {
                        &colour.invert()
                    } else {
                        colour
                    };
                    match self.stacking_mode() {
                        StackingMode::Opaque => true,
                        StackingMode::WhiteTransparent => colour.is_on(),
                        StackingMode::BlackTransparent => colour.is_off(),
                    }
                })
                .map(|Pixel(pos, colour)| {
                    let colour = if self.inverted() {
                        colour.invert()
                    } else {
                        colour
                    };
                    // let colour = match self.stacking_mode() {
                    //     StackingMode::Opaque => colour,
                    //     StackingMode::WhiteTransparent | StackingMode::BlackTransparent => {
                    //         if self.inverted() {
                    //             BinaryColor::On
                    //         } else {
                    //             BinaryColor::Off
                    //         }
                    //     }
                    // };

                    Pixel(pos, colour)
                }),
        )
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // Validate the area is within bounds
        if area.is_zero_sized()
            || area.top_left.x < 0
            || area.top_left.y < 0
            || area.top_left.x as u32 + area.size.width > self.size().width
            || area.top_left.y as u32 + area.size.height > self.size().height
        {
            return Ok(());
        }

        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

impl<C> OriginDimensions for Sei<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn size(&self) -> Size {
        Size::new(self.raw_sei.width(), self.raw_sei.height())
    }
}
