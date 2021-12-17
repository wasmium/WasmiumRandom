use crate::{WasmiumRandom, BIP39_WORDLIST, EFF_LARGE, EFF_SHORT};
use nanorand::{ChaCha8, Rng, WyRand};

impl WasmiumRandom {
    /// Generate BIP39 phrase `6` words long`
    pub fn secure_bip39() -> [&'static str; 6] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 6] = ["TRY_GENERATING_SOMETHING_RANDOM"; 6];

        (0..6).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `8` words long`
    pub fn secure_bip39_8words() -> [&'static str; 8] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 8] = ["TRY_GENERATING_SOMETHING_RANDOM"; 8];

        (0..8).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `12` words long`
    pub fn secure_bip39_12words() -> [&'static str; 12] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 12] = ["TRY_GENERATING_SOMETHING_RANDOM"; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `16` words long`
    pub fn secure_bip39_16words() -> [&'static str; 16] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 16] = ["TRY_GENERATING_SOMETHING_RANDOM"; 16];

        (0..16).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate EFF Short word list phrase `6` words long`
    pub fn secure_eff_shortlist() -> [&'static str; 6] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 6] = ["TRY_GENERATING_SOMETHING_RANDOM"; 6];

        (0..6).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF short word list phrase `8` words long`
    pub fn secure_eff_shortlist_8words() -> [&'static str; 8] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 8] = ["TRY_GENERATING_SOMETHING_RANDOM"; 8];

        (0..8).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF short word list phrase `12` words long`
    pub fn secure_eff_shortlist_12words() -> [&'static str; 12] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 12] = ["TRY_GENERATING_SOMETHING_RANDOM"; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF short word list phrase `16` words long`
    pub fn secure_eff_shortlist_16words() -> [&'static str; 16] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 16] = ["TRY_GENERATING_SOMETHING_RANDOM"; 16];

        (0..16).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `4` words long`
    pub fn secure_eff_largelist() -> [&'static str; 4] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 4] = ["TRY_GENERATING_SOMETHING_RANDOM"; 4];

        (0..4).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `6` words long`
    pub fn secure_eff_largelist_6words() -> [&'static str; 6] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 6] = ["TRY_GENERATING_SOMETHING_RANDOM"; 6];

        (0..6).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `8` words long`
    pub fn secure_eff_largelist_8words() -> [&'static str; 8] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 8] = ["TRY_GENERATING_SOMETHING_RANDOM"; 8];

        (0..8).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `12` words long`
    pub fn secure_eff_largelist_12words() -> [&'static str; 12] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 12] = ["TRY_GENERATING_SOMETHING_RANDOM"; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `16` words long`
    pub fn secure_eff_largelist_16words() -> [&'static str; 16] {
        let mut rng = ChaCha8::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 16] = ["TRY_GENERATING_SOMETHING_RANDOM"; 16];

        (0..16).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `6` words long`
    pub fn wyrand_bip39() -> [&'static str; 6] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 6] = ["TRY_GENERATING_SOMETHING_RANDOM"; 6];

        (0..6).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `8` words long`
    pub fn wyrand_bip39_8words() -> [&'static str; 8] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 8] = ["TRY_GENERATING_SOMETHING_RANDOM"; 8];

        (0..8).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `12` words long`
    pub fn wyrand_bip39_12words() -> [&'static str; 12] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 12] = ["TRY_GENERATING_SOMETHING_RANDOM"; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate BIP39 phrase `16` words long`
    pub fn wyrand_bip39_16words() -> [&'static str; 16] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 16] = ["TRY_GENERATING_SOMETHING_RANDOM"; 16];

        (0..16).for_each(|index| {
            random = rng.generate_range(0_u16..BIP39_WORDLIST.len() as u16);
            random_str[index as usize] = BIP39_WORDLIST[random as usize];
        });

        random_str
    }

    /// Generate EFF Short word list phrase `6` words long`
    pub fn wyrand_eff_shortlist() -> [&'static str; 6] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 6] = ["TRY_GENERATING_SOMETHING_RANDOM"; 6];

        (0..6).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF short word list phrase `8` words long`
    pub fn wyrand_eff_shortlist_8words() -> [&'static str; 8] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 8] = ["TRY_GENERATING_SOMETHING_RANDOM"; 8];

        (0..8).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF short word list phrase `12` words long`
    pub fn wyrand_eff_shortlist_12words() -> [&'static str; 12] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 12] = ["TRY_GENERATING_SOMETHING_RANDOM"; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF short word list phrase `16` words long`
    pub fn wyrand_eff_shortlist_16words() -> [&'static str; 16] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 16] = ["TRY_GENERATING_SOMETHING_RANDOM"; 16];

        (0..16).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_SHORT.len() as u16);
            random_str[index as usize] = EFF_SHORT[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `4` words long`
    pub fn wyrand_eff_largelist() -> [&'static str; 4] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 4] = ["TRY_GENERATING_SOMETHING_RANDOM"; 4];

        (0..4).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `6` words long`
    pub fn wyrand_eff_largelist_6words() -> [&'static str; 6] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 6] = ["TRY_GENERATING_SOMETHING_RANDOM"; 6];

        (0..6).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `8` words long`
    pub fn wyrand_eff_largelist_8words() -> [&'static str; 8] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 8] = ["TRY_GENERATING_SOMETHING_RANDOM"; 8];

        (0..8).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `12` words long`
    pub fn wyrand_eff_largelist_12words() -> [&'static str; 12] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 12] = ["TRY_GENERATING_SOMETHING_RANDOM"; 12];

        (0..12).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }

    /// Generate EFF large word list phrase `16` words long`
    pub fn wyrand_eff_largelist_16words() -> [&'static str; 16] {
        let mut rng = WyRand::new();
        let mut random = 0_u16;
        let mut random_str: [&'static str; 16] = ["TRY_GENERATING_SOMETHING_RANDOM"; 16];

        (0..16).for_each(|index| {
            random = rng.generate_range(0_u16..EFF_LARGE.len() as u16);
            random_str[index as usize] = EFF_LARGE[random as usize];
        });

        random_str
    }
}

#[cfg(test)]
mod test_length {
    use crate::WasmiumRandom;
    #[test]
    fn test_bip39_secure() {
        assert_eq!(6_usize, WasmiumRandom::secure_bip39().len());
        assert_eq!(8_usize, WasmiumRandom::secure_bip39_8words().len());
        assert_eq!(12_usize, WasmiumRandom::secure_bip39_12words().len());
        assert_eq!(16_usize, WasmiumRandom::secure_bip39_16words().len());
    }

    #[test]
    fn test_eff_shortlist_secure() {
        assert_eq!(6_usize, WasmiumRandom::secure_eff_shortlist().len());
        assert_eq!(8_usize, WasmiumRandom::secure_eff_shortlist_8words().len());
        assert_eq!(
            12_usize,
            WasmiumRandom::secure_eff_shortlist_12words().len()
        );
        assert_eq!(
            16_usize,
            WasmiumRandom::secure_eff_shortlist_16words().len()
        );
    }

    #[test]
    fn test_eff_largelist_secure() {
        assert_eq!(4_usize, WasmiumRandom::secure_eff_largelist().len());
        assert_eq!(6_usize, WasmiumRandom::secure_eff_largelist_6words().len());
        assert_eq!(8_usize, WasmiumRandom::secure_eff_largelist_8words().len());
        assert_eq!(
            12_usize,
            WasmiumRandom::secure_eff_largelist_12words().len()
        );
        assert_eq!(
            16_usize,
            WasmiumRandom::secure_eff_largelist_16words().len()
        );
    }

    /************************* TEsts here */

    #[test]
    fn test_bip39_wyrand() {
        assert_eq!(6_usize, WasmiumRandom::wyrand_bip39().len());
        assert_eq!(8_usize, WasmiumRandom::wyrand_bip39_8words().len());
        assert_eq!(12_usize, WasmiumRandom::wyrand_bip39_12words().len());
        assert_eq!(16_usize, WasmiumRandom::wyrand_bip39_16words().len());
    }

    #[test]
    fn test_eff_shortlist_wyrand() {
        assert_eq!(6_usize, WasmiumRandom::wyrand_eff_shortlist().len());
        assert_eq!(8_usize, WasmiumRandom::wyrand_eff_shortlist_8words().len());
        assert_eq!(
            12_usize,
            WasmiumRandom::wyrand_eff_shortlist_12words().len()
        );
        assert_eq!(
            16_usize,
            WasmiumRandom::wyrand_eff_shortlist_16words().len()
        );
    }

    #[test]
    fn test_eff_largelist_wyrand() {
        assert_eq!(4_usize, WasmiumRandom::wyrand_eff_largelist().len());
        assert_eq!(6_usize, WasmiumRandom::wyrand_eff_largelist_6words().len());
        assert_eq!(8_usize, WasmiumRandom::wyrand_eff_largelist_8words().len());
        assert_eq!(
            12_usize,
            WasmiumRandom::wyrand_eff_shortlist_12words().len()
        );
        assert_eq!(
            16_usize,
            WasmiumRandom::wyrand_eff_shortlist_16words().len()
        );
    }
}
