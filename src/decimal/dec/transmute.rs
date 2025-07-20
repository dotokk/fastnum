use crate::{
    bint::UInt,
    decimal::{dec::scale::rescale, Decimal},
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn transmute<const N: usize, const M: usize>(mut d: D<N>) -> D<M> {
    let mut digits = [0; M];
    let mut i = 0;

    if M >= N {
        while i < N {
            digits[i] = d.digits.digits()[i];
            i += 1;
        }
    } else {
        // FIXME
        if UInt::<N>::BITS - d.digits.leading_zeros() > UInt::<M>::BITS {
            while UInt::<N>::BITS - d.digits.leading_zeros() > UInt::<M>::BITS {
                let scale = d.cb.get_scale() - 1;
                rescale(&mut d, scale);
            }
        }

        while i < M {
            digits[i] = d.digits.digits()[i];
            i += 1;
        }
    }

    D::new(UInt::from_digits(digits), d.cb)
}
