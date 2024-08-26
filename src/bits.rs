use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct BitBoard(pub u64, pub u64);

#[pymethods]
impl BitBoard {
    #[new]
    pub fn default() -> Self {
        Self(0x0000_0008_1000_0000, 0x0000_0010_0800_0000)
    }

    /// Returns bitboards of `self`.
    #[must_use]
    pub const fn get(&self) -> [u64; 2] {
        [self.0, self.1]
    }

    /// Passes without playing.
    pub fn pass_move(&self) -> Self {
        Self(self.1, self.0)
    }

    /// Returns the `Game` state after playing the given move.
    /// Panics when `place` is larger than 63.
    #[must_use]
    pub fn make_move(&self, place: usize) -> Option<Self> {
        let diff = (0..4)
            .map(|i| unsafe {
                use crate::consts::*;
                use bitintr::*;
                //maybe we should change INDEX to LUT of raw pointers
                //and save some instructions
                //not sure if that saves some CPU cycles
                //wait before bench
                u64::from(*RESULT.get_unchecked(
                    //https://github.com/rust-lang/rust/issues/51713
                    INDEX[place][i] as usize * 32
                        + self.0.pext(MASK[place][i][0]) as usize * 64
                        + self.1.pext(MASK[place][i][1]) as usize,
                ))
                .pdep(MASK[place][i][1])
            })
            .fold(0, core::ops::BitOr::bitor);
        //for arm processors, we should use brev and hyperbola quintessence, as arm has rbit instruction
        //https://www.chessprogramming.org/Hyperbola_Quintessence
        //or maybe magic bitboards
        //even RISC-V has bdep bext
        if diff == 0 || ((self.0 | self.1) & 1 << place != 0) {
            None
        } else {
            Some(Self(self.1 ^ diff, self.0 ^ diff ^ 1 << place))
        }
    }

    /// Returns the bitboard representation of available moves.
    #[must_use]
    pub fn available_moves(&self) -> u64 {
        //the below should compile to AVX2 instructions(256bit)
        [-9, -8, -7, -1, 1, 7, 8, 9]
            //we use iter_mut because of the noalias bug
            .iter_mut()
            .map(|i| self.gen(*i))
            .fold(0, core::ops::BitOr::bitor)
            & !self.0
            & !self.1
    }

    pub fn available_moves_list(&self) -> Vec<usize> {
        let mask = self.available_moves();
        (0..64).filter(|i| mask >> i & 1 == 1).collect()
    }

    #[must_use]
    fn gen(&self, dir: isize) -> u64 {
        //rotate might be faster on AVX-512
        fn shift(x: u64, y: isize) -> u64 {
            if y > 0 {
                x >> y
            } else {
                x << -y
            }
        }
        let x = self.0;
        //if we change above to rotate, we should also modify the following
        let y = self.1
            & match dir.rem_euclid(8) {
                0 => !0,
                1 | 7 => 0x7E7E_7E7E_7E7E_7E7E,
                _ => unreachable!(),
            };
        let d = dir;
        let x = x | y & shift(x, d);
        let y = y & shift(y, d);
        let d = d * 2;
        let x = x | y & shift(x, d);
        let y = y & shift(y, d);
        let d = d * 2;
        let x = x | y & shift(x, d);
        shift(x ^ self.0, dir)
    }

    pub fn count(&self) -> (i32, i32) {
        (self.0.count_ones() as i32, self.1.count_ones() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn default_test() {
        let x = BitBoard::default();
        assert_eq!(x.available_moves(), 0x0000_1020_0408_0000);

        let x = x.make_move(44).unwrap();
        assert_eq!(x.get(), [0x0000_0000_0800_0000, 0x0000_1018_1000_0000]);
        assert_eq!(x.count(), (1, 4));
    }
}
