use crate::decimal::common::math::recip::test_impl;

test_impl!(D, 64);
test_impl!(D, 128);
test_impl!(D, 256);
test_impl!(D, 512);

test_impl!(UD, 64);
test_impl!(UD, 128);
test_impl!(UD, 256);
test_impl!(UD, 512);
