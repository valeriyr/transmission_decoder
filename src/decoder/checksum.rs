pub fn calculate(data: u16) -> u8 {
    let mut checksum: u8 = 0;
    let mut index: usize = 0;

    while index < 4 {
        let shift = index * 4;

        // Should never panic
        let group: u8 = ((data & 0x000f << shift) >> shift).try_into().unwrap();

        checksum = checksum.overflowing_add(group).0;

        index += 1;
    }

    checksum & 0x0f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_checksum() {
        assert_eq!(calculate(0x86F7), 0x4);
    }
}
