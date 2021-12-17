use crate::{WasmiumRandom, ALPHANUMERIC};
use nanorand::{ChaCha8, Rng};

impl WasmiumRandom {
    /// Generate alphanumeric bytes of size `12`
    pub fn secure_alphanumeric12() -> [u8; 12] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 12] = [0; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `24`
    pub fn secure_alphanumeric24() -> [u8; 24] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 24] = [0; 24];

        (0..24).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `32`
    pub fn secure_alphanumeric32() -> [u8; 32] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 32] = [0; 32];

        (0..32).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `64`
    pub fn secure_alphanumeric64() -> [u8; 64] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 64] = [0; 64];

        (0..64).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `128`
    pub fn secure_alphanumeric128() -> [u8; 128] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 128] = [0; 128];

        (0..128).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `256`
    pub fn secure_alphanumeric256() -> [u8; 256] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 256] = [0; 256];

        (0..256).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `512`
    pub fn secure_alphanumeric512() -> [u8; 512] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 512] = [0; 512];

        (0..512).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `1024`
    pub fn secure_alphanumeric1024() -> [u8; 1024] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 1024] = [0; 1024];

        (0..1024).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }

    /// Generate alphanumeric bytes of size `2048`
    pub fn secure_alphanumeric2048() -> [u8; 2048] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u8;
        let mut random_bytes: [u8; 2048] = [0; 2048];

        (0..2048).for_each(|index| {
            random = rng.generate_range(0_u8..=61);
            random_bytes[index as usize] = ALPHANUMERIC[random as usize].as_bytes()[0];
        });

        random_bytes
    }
}

#[cfg(test)]
mod test_length {
    use crate::WasmiumRandom;
    #[test]
    fn test_secure_alphanumeric() {
        assert_eq!(12_usize, WasmiumRandom::secure_alphanumeric12().len());
        assert_eq!(24_usize, WasmiumRandom::secure_alphanumeric24().len());
        assert_eq!(32_usize, WasmiumRandom::secure_alphanumeric32().len());
        assert_eq!(64_usize, WasmiumRandom::secure_alphanumeric64().len());
        assert_eq!(128_usize, WasmiumRandom::secure_alphanumeric128().len());
        assert_eq!(256_usize, WasmiumRandom::secure_alphanumeric256().len());
        assert_eq!(512_usize, WasmiumRandom::secure_alphanumeric512().len());
        assert_eq!(1024_usize, WasmiumRandom::secure_alphanumeric1024().len());
        assert_eq!(2048_usize, WasmiumRandom::secure_alphanumeric2048().len());
    }
}
