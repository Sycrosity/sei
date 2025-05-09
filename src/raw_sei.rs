use crate::{SeiHeader, error::SeiError, settings::SeiSettings};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RawSei<'a> {
    pub header: SeiHeader,
    pub img_data: &'a [u8],
}

impl<'a> RawSei<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, SeiError> {
        if bytes.len() < 10 {
            return Err(SeiError::UnreadableFile);
        }

        if &bytes[0..2] != b"SE" {
            return Err(SeiError::IncorrectMagicBytes);
        }

        let version = bytes[2];
        let data_offset = bytes[3] as usize;

        let width = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let height = u16::from_le_bytes(bytes[6..8].try_into().unwrap());
        let settings = bytes[8];
        let settings = SeiSettings::parse(settings)?;
        let z_index = bytes[9];

        if data_offset > bytes.len() {
            return Err(SeiError::InvalidDataOffset);
        }

        if (settings.bit_depth.bits() * width as u32).div_ceil(8) * height as u32
            != (bytes.len() - data_offset) as u32
        {
            return Err(SeiError::IncorrectLength(
                bytes.len() as u32 - data_offset as u32,
                (settings.bit_depth.bits() * width as u32).div_ceil(8) * height as u32,
            ));
        }

        let header = SeiHeader {
            version,
            data_offset,
            width,
            height,
            settings,
            z_index,
        };

        let (_, data) = bytes.split_at(data_offset);

        Ok(Self {
            header,
            img_data: data,
        })
    }

    pub const fn width(&self) -> u32 {
        self.header.width as u32
    }

    pub const fn height(&self) -> u32 {
        self.header.height as u32
    }

    pub const fn pixels_per_byte(&self) -> u32 {
        8 / self.header.settings.bit_depth.bits()
    }

    pub const fn bit_depth(&self) -> u32 {
        self.header.bit_depth()
    }

    /// Get the number of pixels used including padding.
    pub const fn data_width(&self) -> u32 {
        self.header.data_width()
    }
}

#[test]
fn test_1bit_sei() {
    use crate::settings::*;

    let bytes = &[
        b'S',
        b'E', // Magic bytes
        1,    // Version
        0x0A, // Data offset
        8,    // Width
        0,
        2, // Height
        0,
        0b0010_0100, // Settings
        5,           // Z index
        0b10101010,  // Img data
        0b01010101,  // Img data
    ];
    let sei = RawSei::parse(bytes).unwrap();

    assert_eq!(
        sei.header,
        SeiHeader::builder()
            .version(1)
            .data_offset(0x0A)
            .settings(
                SeiSettings::builder()
                    .bit_depth(BitDepth::OneBit)
                    .white_mode(WhiteMode::Ones)
                    .padding(Padding::Padding)
                    .stacking_mode(StackingMode::BlackTransparent)
                    .build(),
            )
            .width(8)
            .height(2)
            .z_index(5)
            .build()
    );

    assert_eq!(sei.img_data, &[0b10101010, 0b01010101]);
}

#[test]
fn data_width() {
    use crate::settings::*;

    let raw = RawSei {
        header: SeiHeader::builder()
            .settings(SeiSettings::builder().bit_depth(BitDepth::OneBit).build())
            .width(11)
            .height(2)
            .build(),
        img_data: &[0b10101010, 0b0101010],
    };

    assert_eq!(raw.data_width(), 16);

    let raw = RawSei {
        header: SeiHeader::builder()
            .settings(SeiSettings::builder().bit_depth(BitDepth::TwoBits).build())
            .width(11)
            .height(2)
            .build(),
        img_data: &[
            0b10101010, 0b01010101, 0b01010100, 0b01010101, 0b01010101, 0b01010100,
        ],
    };

    assert_eq!(raw.data_width(), 12);
}

// #[test]
// fn test_2bit_sei() {
//     use crate::settings::*;

//     let bytes = &[
//         b'S', b'E', // Magic bytes
//         1,    // Version
//         0x0A, // Data offset
//         8,    // Width (LE)
//         0, 2, // Height (LE)
//         0, 0b00100101, // Settings
//         2,          // Z index
//         0b10101010, // Img data
//         0b10101010, 0b01010101, 0b01010101,
//     ];
//     let sei = RawSei::parse(bytes).unwrap();

//     assert_eq!(
//         sei.header,
//         SeiHeader::builder()
//             .version(1)
//             .data_offset(0x0A)
//             .settings(
//                 SeiSettings::builder()
//                     .bit_depth(BitDepth::TwoBits)
//                     .white_mode(WhiteMode::Ones)
//                     .padding(Padding::Padding)
//                     .stacking_mode(StackingMode::BlackTransparent)
//                     .build(),
//             )
//             .width(8)
//             .height(2)
//             .z_index(2)
//             .build()
//     );

//     assert_eq!(sei.data, &[0b10101010, 0b10101010, 0b01010101, 0b01010101]);
// }
