use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Bus {
    word_1: u128,
    word_2: u128,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            word_1: 0,
            word_2: 0,
        }
    }

    pub fn set(&mut self, index: u8, value: bool) {
        let word = if index < 128 {
            &mut self.word_1
        } else {
            &mut self.word_2
        };
        let index = index % 128;
        if value {
            *word |= 1 << index;
        } else {
            *word &= !(1 << index);
        }
    }

    pub fn get(&self, index: u8) -> bool {
        let word = if index < 128 {
            self.word_1
        } else {
            self.word_2
        };
        let index = index % 128;
        word & (1 << index) != 0
    }

    pub fn last_significant_bit(&self) -> u8 {
        if self.word_2 != 0 {
            255 - self.word_2.leading_zeros() as u8
        } else {
            if self.word_1 != 0 {
                127 - self.word_1.leading_zeros() as u8
            } else {
                0
            }
        }
    }

    pub fn copy(&self, idxs: &[u8]) -> Bus {
        let mut bus = Bus::default();
        let mut res_offset = 0;

        for idx in idxs {
            let val = self.get(*idx);
            bus.set(res_offset, val);
            res_offset += 1;
        }

        bus
    }

    pub fn take(&mut self, idxs: &[u8]) -> Bus {
        let mask = Bus::from(idxs);

        let mut this = Bus::new();
        let mut other = Bus::new();

        let mut this_idx = 0;
        let mut other_idx = 0;

        for idx in 0..=255 {
            if mask.get(idx) {
                other.set(other_idx, self.get(idx));
                other_idx += 1;
            } else {
                this.set(this_idx, self.get(idx));
                this_idx += 1;
            }
        }
        *self = this;
        other
    }
}

impl From<&[u8]> for Bus {
    fn from(indexes: &[u8]) -> Self {
        let mut bus = Bus::new();
        for idx in indexes {
            bus.set(*idx, true);
        }
        bus
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}", self.word_1, self.word_2)
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..=self.last_significant_bit() {
            write!(f, "{}", if self.get(i) { "1" } else { "0" })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_get_set() {
        let mut bits = super::Bus::default();
        let mut expected = [false; 256];

        for _ in 0..1000 {
            let index = rand::random();
            let value = rand::random();
            bits.set(index, value);
            expected[index as usize] = value;
        }

        println!("{}", bits);
        println!("{:?}", bits);
        for i in 0..255 {
            assert_eq!(bits.get(i), expected[i as usize]);
        }
    }

    #[test]
    pub fn test_last_significant_bit() {
        let mut bits = super::Bus::default();
        assert_eq!(bits.last_significant_bit(), 0);
        bits.set(0, true);
        assert_eq!(bits.last_significant_bit(), 0);
        bits.set(1, true);
        assert_eq!(bits.last_significant_bit(), 1);
        bits.set(250, true);
        assert_eq!(bits.last_significant_bit(), 250);
        bits.set(255, true);
        assert_eq!(bits.last_significant_bit(), 255);
    }
}
