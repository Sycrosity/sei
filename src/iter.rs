use core::{iter::Take, slice::ChunksExact};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::{iterator::raw::RawDataSlice, prelude::PixelColor};

use crate::header::SeiHeader;

use crate::raw_sei::RawSei;
use crate::settings::Padding;

/// When draw is called, the bytes from the image data are written to the target.
/// If StackingMode::Opaque is used, the bytes are written as they are, contiguously.
/// If StackingMode::WhiteTransparent is used, the bytes are written as they are, but the white pixels are skipped.
/// If StackingMode::BlackTransparent is used, the bytes are written as they are, but the black pixels are skipped.
pub struct Pixels<'a, C>
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
