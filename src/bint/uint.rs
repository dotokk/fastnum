mod bits;
mod carrying;
mod checked;
mod cmp;
mod convert;
mod endian;
mod impls;
mod intrinsics;
mod math;
mod num;
mod overflowing;
mod powers;
mod saturating;
mod strict;
mod unchecked;
mod widening;
mod wrapping;

use bnum::BUint;

use crate::bint::{consts::consts_impl, doc};

/// Generic Unsigned integer type composed of 64-bit
/// digits, of arbitrary fixed size which must be known at compile time.
///
/// Digits are stored in little endian (the least significant digit first).
/// This integer type aims to exactly replicate the behaviours of Rust's
/// built-in unsigned integer types: `u8`, `u16`, `u32`, `u64`, `u128` and
/// `usize`.
/// The const generic parameter `N` is the number of 64-bit digits that are
/// stored.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize, borsh::BorshSchema)
)]
#[repr(transparent)]
pub struct UInt<const N: usize>(pub(super) BUint<N>);

consts_impl!(UInt, U, BUint);
