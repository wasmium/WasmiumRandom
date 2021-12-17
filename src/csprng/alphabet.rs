use crate::{WasmiumRandom, ALPHABET};
use nanorand::{ChaCha8, Rng};

impl WasmiumRandom {
    /// Generate alphabet bytes of size `12`
    pub fn secure_alphabet12() -> [u8; 12] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 12] = [0; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `24`
    pub fn secure_alphabet24() -> [u8; 24] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 24] = [0; 24];

        (0..24).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `32`
    pub fn secure_alphabet32() -> [u8; 32] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 32] = [0; 32];

        (0..32).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `64`
    pub fn secure_alphabet64() -> [u8; 64] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 64] = [0; 64];

        (0..64).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `128`
    pub fn secure_alphabet128() -> [u8; 128] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 128] = [0; 128];

        (0..128).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `256`
    pub fn secure_alphabet256() -> [u8; 256] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 256] = [0; 256];

        (0..256).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `512`
    pub fn secure_alphabet512() -> [u8; 512] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 512] = [0; 512];

        (0..512).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphabet bytes of size `1024`
    pub fn secure_alphabet1024() -> [u8; 1024] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 1024] = [0; 1024];

        (0..1024).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate english alphabet bytes of size `2048`
    pub fn secure_alphabet2048() -> [u8; 2048] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 2048] = [0; 2048];

        (0..2048).for_each(|index| {
            random = rng.generate_range(0_u8..=51);
            random_bytes[index as usize] = ALPHABET[random as usize].as_bytes()[0];
        });

        random_bytes
    }
}

#[cfg(test)]
mod test_length {
    use crate::WasmiumRandom;
    #[test]
    fn test_secure_alphabet() {
        assert_eq!(12_usize, WasmiumRandom::secure_alphabet12().len());
        assert_eq!(24_usize, WasmiumRandom::secure_alphabet24().len());
        assert_eq!(32_usize, WasmiumRandom::secure_alphabet32().len());
        assert_eq!(64_usize, WasmiumRandom::secure_alphabet64().len());
        assert_eq!(128_usize, WasmiumRandom::secure_alphabet128().len());
        assert_eq!(256_usize, WasmiumRandom::secure_alphabet256().len());
        assert_eq!(512_usize, WasmiumRandom::secure_alphabet512().len());
        assert_eq!(1024_usize, WasmiumRandom::secure_alphabet1024().len());
        assert_eq!(2048_usize, WasmiumRandom::secure_alphabet2048().len());
    }
}
