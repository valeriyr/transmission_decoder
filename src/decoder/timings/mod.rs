mod error;

pub use error::TimingsError;

pub type Result<T> = std::result::Result<T, TimingsError>;

pub fn is_timing_valid(timing: u32, multiplier: u32) -> bool {
    if multiplier == 0 {
        return false;
    }

    const TIME_UNIT: u32 = 480;

    let etalon = TIME_UNIT * multiplier;
    let precision = etalon * 25 / 100;

    let min = etalon - precision;
    let max = etalon + precision;

    min <= timing && timing <= max
}

pub fn decode_value<
    T: From<u8>
        + std::cmp::Eq
        + std::ops::Shl<usize>
        + std::ops::BitOr<<T as std::ops::Shl<usize>>::Output, Output = T>,
>(
    timings: &[u32],
    bits: usize,
) -> Result<T> {
    let expected_timings_number = bits * 2;
    let actual_timings_number = timings.len();

    if expected_timings_number != actual_timings_number {
        return Err(TimingsError::WrongTimingsNumber(
            expected_timings_number,
            actual_timings_number,
        ));
    }

    let mut value: T = 0.into();
    let mut index: usize = 0;

    while index < bits {
        let pos = index * 2;

        let on = timings[pos];
        let off = timings[pos + 1];

        let bit: T = decode_bit(on, off)?;

        if bit == 1.into() {
            value = value | bit << (bits - 1 - index);
        }

        index += 1;
    }

    Ok(value)
}

pub fn decode_bit<T: From<u8>>(on: u32, off: u32) -> Result<T> {
    if !is_timing_valid(on, 1) {
        return Err(TimingsError::AbnormalLedOnBitValue(on));
    }

    if is_timing_valid(off, 1) {
        Ok(0.into())
    } else if is_timing_valid(off, 3) {
        Ok(1.into())
    } else {
        Err(TimingsError::AbnormalLedOffBitValue(off))
    }
}

#[cfg(test)]
mod tests {
    mod timing_validation {
        use crate::decoder::timings::is_timing_valid;

        #[test]
        fn valid_timing_with_multiplier_one() {
            assert_eq!(is_timing_valid(480, 1), true);
        }

        #[test]
        fn valid_timing_with_multiplier_three() {
            assert_eq!(is_timing_valid(1440, 3), true);
        }

        #[test]
        fn minimum_accepteble_timing() {
            assert_eq!(is_timing_valid(1080, 3), true);
        }

        #[test]
        fn maximum_accepteble_timing() {
            assert_eq!(is_timing_valid(1800, 3), true);
        }

        #[test]
        fn zero_multiplier() {
            assert_eq!(is_timing_valid(480, 0), false);
        }

        #[test]
        fn timing_less_then_minimum() {
            assert_eq!(is_timing_valid(1079, 3), false);
        }

        #[test]
        fn timing_more_then_maximum() {
            assert_eq!(is_timing_valid(1801, 3), false);
        }
    }

    mod bit_decoding {
        use crate::decoder::timings::{decode_bit, TimingsError};

        #[test]
        fn valid_bit_0() {
            assert_eq!(decode_bit::<u8>(480, 480).unwrap(), 0);
        }

        #[test]
        fn valid_bit_1() {
            assert_eq!(decode_bit::<u8>(480, 1440).unwrap(), 1);
        }

        #[test]
        fn invalid_timing_on() {
            assert_eq!(
                decode_bit::<u8>(359, 480).unwrap_err(),
                TimingsError::AbnormalLedOnBitValue(359)
            );
        }

        #[test]
        fn invalid_timing_off() {
            assert_eq!(
                decode_bit::<u8>(480, 700).unwrap_err(),
                TimingsError::AbnormalLedOffBitValue(700)
            );
        }
    }

    mod value_decoding {
        use crate::decoder::timings::{decode_value, TimingsError};

        #[test]
        fn valid_8bit_value() {
            let timings: [u32; 16] = [
                402, 550, 467, 524, 489, 491, 580, 392, 454, 1700, 440, 1607, 475, 534, 501, 591,
            ];

            assert_eq!(decode_value::<u8>(&timings, 8).unwrap(), 12);
        }

        #[test]
        fn valid_16bit_value() {
            let timings: [u32; 32] = [
                402, 550, 467, 524, 489, 491, 580, 392, 454, 1700, 440, 1607, 475, 534, 501, 591,
                498, 443, 364, 574, 477, 591, 394, 1412, 452, 511, 382, 423, 448, 494, 539, 507,
            ];

            assert_eq!(decode_value::<u16>(&timings, 16).unwrap(), 3088);
        }

        #[test]
        fn invalid_timings_len() {
            let timings: [u32; 15] = [
                402, 550, 467, 524, 489, 491, 580, 392, 454, 1700, 440, 1607, 475, 534, 501,
            ];

            assert_eq!(
                decode_value::<u8>(&timings, 8).unwrap_err(),
                TimingsError::WrongTimingsNumber(16, 15)
            );
        }

        #[test]
        fn invalid_on_timing() {
            let timings: [u32; 16] = [
                402, 550, 467, 524, 489, 491, 580, 392, 454, 1700, 440, 1607, 475, 534, 601, 591,
            ];

            assert_eq!(
                decode_value::<u8>(&timings, 8).unwrap_err(),
                TimingsError::AbnormalLedOnBitValue(601)
            );
        }

        #[test]
        fn invalid_off_timing() {
            let timings: [u32; 16] = [
                402, 550, 467, 524, 489, 491, 580, 392, 454, 1700, 440, 1607, 475, 634, 501, 591,
            ];

            assert_eq!(
                decode_value::<u8>(&timings, 8).unwrap_err(),
                TimingsError::AbnormalLedOffBitValue(634)
            );
        }
    }
}
