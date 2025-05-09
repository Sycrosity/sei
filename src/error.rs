#[derive(Debug, Clone, Copy)]
pub enum SeiError {
    IncorrectLength(u32, u32),
    IncorrectMagicBytes,
    InvalidVersion,
    InvalidDataOffset,
    UnsupportedBitDepth,
    UnsupportedTransparency,
    UnreadableFile,
}
