use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DecoderError {
    #[error("checksum mismatch: expected '{0}', actual '{1}'")]
    ChecksumMismatch(u8, u8),
    #[error("the sequence is empty")]
    EmptySequence,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("timings error: {0}")]
    TimingsError(#[from] super::timings::TimingsError),
    #[error("wrong epilogue")]
    WrongEpilogue,
    #[error("wrong preamble")]
    WrongPreamble,
    #[error("wrong timings number: expected '{0}', actual '{1}")]
    WrongTimingsNumber(usize, usize),
}
