#[derive(Clone, Copy, Debug)]
pub enum SeiError {
    IncorrectLength,
    IncorrectMagicBytes,
    InvalidVersion,
    InvalidDataOffset,
    UnsupportedBitDepth,
    UnsupportedTransparency,
    UnreadableFile,
    // UnsupportedSetting,
    NoPadding,
}

impl core::fmt::Display for SeiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IncorrectLength => write!(f, "IncorrectLength"),
            Self::IncorrectMagicBytes => write!(f, "IncorrectMagicBytes"),
            Self::InvalidVersion => write!(f, "InvalidVersion"),
            Self::InvalidDataOffset => write!(f, "InvalidDataOffset"),
            Self::UnsupportedBitDepth => write!(f, "UnsupportedBitDepth"),
            Self::UnsupportedTransparency => write!(f, "UnsupportedTransparency"),
            Self::UnreadableFile => write!(f, "UnreadableFile"),
            Self::NoPadding => write!(
                f,
                "`Padding::NoPadding` settings option is supported in this version."
            ),
        }
    }
}

impl core::error::Error for SeiError {}
