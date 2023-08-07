use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TimingsError {
    #[error("abnormal led on bit value: {0}")]
    AbnormalLedOnBitValue(u32),
    #[error("abnormal led off bit value: {0}")]
    AbnormalLedOffBitValue(u32),
    #[error("wrong timings number: expected '{0}', actual '{1}'")]
    WrongTimingsNumber(usize, usize),
}
