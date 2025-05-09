//! `sei` - A .SEI (stacking ePaper image) file parser.
//!
//! no_std-compatible library for parsing SEI files for use with embedded_graphics.

#![cfg_attr(not(test), no_std)]

pub mod error;
pub mod header;
pub mod raw_sei;
pub mod settings;

use crate::error::SeiError;

use core::{iter::Take, marker::PhantomData, slice::ChunksExact};
use raw_sei::RawSei;

use embedded_graphics::{
    draw_target::DrawTarget,
    draw_target::DrawTargetExt,
    iterator::raw::RawDataSlice,
    pixelcolor::{BinaryColor, raw::LittleEndian},
    prelude::ContiguousIteratorExt,
    prelude::*,
    primitives::Rectangle,
};
use header::SeiHeader;
use settings::*;

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

    pub const fn as_raw(&self) -> &RawSei<'a> {
        &self.raw_sei
    }

    pub const fn to_raw(self) -> RawSei<'a> {
        self.raw_sei
    }

    pub const fn set_stacking_mode(&mut self, stacking_mode: StackingMode) {
        self.raw_sei.header.settings.stacking_mode = stacking_mode;
    }

    pub const fn stacking_mode(&self) -> StackingMode {
        self.raw_sei.header.settings.stacking_mode
    }

    pub const fn set_white_mode(&mut self, white_mode: WhiteMode) {
        self.raw_sei.header.settings.white_mode = white_mode;
    }

    pub const fn white_mode(&self) -> WhiteMode {
        self.raw_sei.header.settings.white_mode
    }

    pub fn bit_depth(&self) -> BitDepth {
        self.raw_sei.header.settings.bit_depth
    }
}

/// When draw is called, the bytes from the image data are written to the target.
/// If StackingMode::Opaque is used, the bytes are written as they are, contiguously.
/// If StackingMode::WhiteTransparent is used, the bytes are written as they are, but the white pixels are skipped.
/// If StackingMode::BlackTransparent is used, the bytes are written as they are, but the black pixels are skipped.
struct Pixels<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    RawDataSlice<'a, C::Raw, LittleEndian>: IntoIterator<Item = C::Raw>,
{
    rows: ChunksExact<'a, u8>,
    current_row: Take<<RawDataSlice<'a, C::Raw, LittleEndian> as IntoIterator>::IntoIter>,
    header: SeiHeader,
}

impl<'a, C> Pixels<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    RawDataSlice<'a, C::Raw, LittleEndian>: IntoIterator<Item = C::Raw>,
{
    pub fn new(sei: RawSei<'a>) -> Self {
        Self {
            rows: sei.img_data.chunks_exact((sei.data_width() as usize) / 8),
            current_row: RawDataSlice::new(&[]).into_iter().take(0),
            header: sei.header,
        }
    }
}

impl<'a, C> Iterator for Pixels<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    RawDataSlice<'a, C::Raw, LittleEndian>: IntoIterator<Item = C::Raw>,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {

        self.current_row.next().map(C::from).or_else(|| {

            //TODO - deal with this later
            let width = if self.header.settings.padding == Padding::NoPadding {
                self.header.width as usize
            } else {
                // Calculate the number of bits to skip at the end of each row
                self.header.data_width() as usize
            };

            self.current_row = RawDataSlice::new(self.rows.next()?).into_iter().take(width);

            self.current_row.next().map(C::from)
        })
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
                    // Apply stacking mode logic
                    match self.stacking_mode() {
                        StackingMode::Opaque => true,
                        StackingMode::WhiteTransparent => colour.is_on(),
                        StackingMode::BlackTransparent => colour.is_off(),
                    }
                })
                .map(|Pixel(pos, colour)| {
                    let colour = match self.stacking_mode() {
                        StackingMode::Opaque => colour,
                        StackingMode::WhiteTransparent | StackingMode::BlackTransparent => {
                            if self.white_mode() == WhiteMode::Zeros {
                                BinaryColor::On
                            } else {
                                BinaryColor::Off
                            }
                        }
                    };

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
