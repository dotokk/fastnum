use crate::bint::UInt;

pub(crate) struct Intrinsics<const N: usize>;

/// _E<sub>limit</sub> = 32'768_
pub(crate) const E_LIMIT: i32 = -(i16::MIN as i32);

/// _E<sub>min</sub> = -32'767_
pub(crate) const E_MIN: i32 = -(i16::MAX as i32);

impl<const N: usize> Intrinsics<N> {
    /// Max length of the _coefficient_ in decimal digits.
    pub(crate) const MAX_CLENGTH: u32 = clength(UInt::<N>::MAX);

    /// _E<sub>max</sub> = E<sub>limit</sub> + (C<sub>length</sub> – 1)_
    pub(crate) const E_MAX: i32 = E_LIMIT + (Self::MAX_CLENGTH as i32 - 1);

    pub(crate) const SERIES_MAX_ITERATIONS: u32 = Self::MAX_CLENGTH * 6;
}

#[inline(always)]
pub(crate) const fn clength<const N: usize>(coeff: UInt<N>) -> u32 {
    if coeff.is_zero() {
        return 1;
    }

    coeff.ilog10() + 1
}
