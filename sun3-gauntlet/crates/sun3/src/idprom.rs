//! Deterministic Sun IDPROM generation and validation.

/// Sun hardware type code for a Sun 3/60.
pub const SUN_3_60_MACHINE_TYPE: u8 = 0x17;

/// Build a deterministic, valid 32-byte Sun IDPROM.
#[must_use]
pub fn default_idprom(mac: [u8; 6], serial: u32) -> [u8; 32] {
    let mut id = [0_u8; 32];
    id[0] = 1;
    id[1] = SUN_3_60_MACHINE_TYPE;
    id[2..8].copy_from_slice(&mac);

    // A stable manufacturing timestamp keeps deterministic runs byte-for-byte
    // reproducible. 1988-01-01T00:00:00Z is historically plausible.
    id[8..12].copy_from_slice(&567_993_600_u32.to_be_bytes());
    let serial = serial & 0x00ff_ffff;
    id[12] = (serial >> 16) as u8;
    id[13] = (serial >> 8) as u8;
    id[14] = serial as u8;

    id[15] = id[..15].iter().fold(0_u8, |checksum, byte| checksum ^ byte);
    id
}

/// Verify the IDPROM format and longitudinal XOR checksum.
#[must_use]
pub fn is_valid(id: &[u8; 32]) -> bool {
    id[0] == 1 && id[..16].iter().fold(0_u8, |checksum, byte| checksum ^ byte) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_idprom_is_valid_and_deterministic() {
        let id = default_idprom([0x08, 0x00, 0x20, 0x12, 0x34, 0x56], 0x12_34_56);
        assert!(is_valid(&id));
        assert_eq!(id[0], 1);
        assert_eq!(id[1], SUN_3_60_MACHINE_TYPE);
        assert_eq!(&id[2..8], &[0x08, 0x00, 0x20, 0x12, 0x34, 0x56]);
        assert_eq!(&id[12..15], &[0x12, 0x34, 0x56]);
    }
}
