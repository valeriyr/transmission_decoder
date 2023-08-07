mod checksum;
mod error;
mod timings;

pub use error::DecoderError;

use self::timings::*;

pub type Result<T> = std::result::Result<T, DecoderError>;

pub fn decode(sequence: &str) -> Result<String> {
    // Initial checking
    if sequence.is_empty() {
        return Err(DecoderError::EmptySequence);
    }

    // Timings checking
    let timings: Result<Vec<_>> = sequence
        .split_whitespace()
        .map(|timing| timing.parse::<u32>().map_err(|e| e.into()))
        .collect();

    let timings = timings?;
    let timings_number = timings.len();

    const EXPECTED_TIMINGS_NUMBER: usize = 43;

    if timings_number != EXPECTED_TIMINGS_NUMBER {
        return Err(DecoderError::WrongTimingsNumber(
            EXPECTED_TIMINGS_NUMBER,
            timings_number,
        ));
    }

    // Preamble checking
    let preamble: &[u32] = &timings[0..2];

    if !is_timing_valid(preamble[0], 18) || !is_timing_valid(preamble[1], 8) {
        return Err(DecoderError::WrongPreamble);
    }

    // Epilogue checking
    let epilogue: &[u32] = &timings[42..EXPECTED_TIMINGS_NUMBER];

    if !is_timing_valid(epilogue[0], 1) {
        return Err(DecoderError::WrongEpilogue);
    }

    // Decode checksum
    let expected_checksum: u8 = decode_value(&timings[34..42], 4)?;

    // Decode data
    let data: u16 = decode_value(&timings[2..34], 16)?;
    let actual_checksum = checksum::calculate(data);

    if actual_checksum != expected_checksum {
        return Err(DecoderError::ChecksumMismatch(
            expected_checksum,
            actual_checksum,
        ));
    }

    Ok(format!("{:04x}", data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_1() {
        let sequence = String::from("8532 3624 402 550 467 524 489 491 580 392 454 1700 440 1607 475 534 501 591 498 443 364 574 477 591 394 1412 452 511 382 423 448 494 539 507 453 1321 572 1132 452 415 539 1165 583");

        assert_eq!(decode(&sequence).unwrap(), "0c10");
    }

    #[test]
    fn valid_2() {
        let sequence = String::from("8332 4021 498 1732 537 1454 361 546 513 580 441 1378 445 361 471 1722 511 598 547 1794 440 1754 376 1126 491 1736 475 1213 512 1373 594 1441 360 558 383 454 369 531 580 1393 419 1631 520");

        assert_eq!(decode(&sequence).unwrap(), "cafe");
    }

    #[test]
    fn valid_3() {
        let sequence = String::from("8651 4412 442 553 443 1452 585 563 392 420 507 360 491 491 381 1209 544 399 599 1326 403 587 560 1188 493 1294 457 1342 572 1430 553 589 543 598 596 1206 581 1670 556 468 468 1454 442");

        assert_eq!(decode(&sequence).unwrap(), "42bc");
    }

    #[test]
    fn invalid_1() {
        let sequence = String::from("6273 3021 498 1231 496 565 496 449 547 562 587 1497 434 1637 387 589 596 1180 388 1635 551 1198 401 551 375 1465 519 414 72 1617 394 585 564 1244 529 1650 513 580 513 580 513 580 513 580 486");

        assert_eq!(
            decode(&sequence).unwrap_err(),
            DecoderError::WrongTimingsNumber(43, 45)
        );
    }

    #[test]
    fn invalid_2() {
        let sequence = String::from("7682 4701 403 1671 456 1368 535 559 518 1535 473 459 439 1249 490 1288 574 1787 578 399 540 413 382 494 428 1793 452 1099 439 1730 588 1161 484 1610 521 363 532 563 442 553 450 1548 393");

        assert_eq!(
            decode(&sequence).unwrap_err(),
            DecoderError::ChecksumMismatch(1, 4)
        );
    }
}
