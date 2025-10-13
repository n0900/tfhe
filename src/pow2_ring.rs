use std::{fmt, u64};
use std::iter::Sum;
use std::ops::{Add, Sub, Mul, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign};

use ff::derive::bitvec::array::BitArray;
use num_traits::{Bounded, Zero};

use crate::RingElement;

/// Integer ring Z_{2^M} (M is exponent)
/// Only supports 1 <= M <= 64 so everything fits in `u64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zpow2<const M: u64> {
    value: u64,
}

impl<const M: u64> RingElement for Zpow2<M> {
    const Num_Bits: usize = M as usize;

    fn to_le_bits_re(&self) -> BitArray<[u8; 8]> {
        let mut bit_array = BitArray::<[u8; 8]>::default();

        for i in 0..M {
            let bit = (self.value() >> i) & 1 == 1;  
            bit_array.set(i as usize, bit); 
        }

        bit_array
    }
    fn max_u64() -> u64 {
        Self::max_value().value
    }
}

impl<const M: u64> Zpow2<M> {
    /// Compute mask = 2^M - 1 as u64. Valid only for 1..=64.
    #[inline]
    fn mask() -> u64 {
        // Ensure M in 1..=64
        assert!(M >= 1 && M <= 64, "Exponent M must be in 1..=64");
        if M == 64 {
            u64::MAX
        } else {
            // safe: M < 64 here
            (1u64 << M) - 1
        }
    }

    /// Fast mod by 2^M using bit operations
    #[inline]
    fn fast_mod(x: u64) -> u64 {
        x & Self::mask()
    }

    /// Create a new element
    pub fn new(value: u64) -> Self {
        Self { value: Self::fast_mod(value) }
    }

    /// Return representative in 0 .. 2^M - 1
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn inv(self) -> Option<Self> {
        panic!("This is a ring. We do not implement any inverses")
    }
}

/// Arithmetic using wrapping ops + mask
/// This works because (x mod a) mod b == x mod b <=> b|a
/// since wrapping_add operate mod 2^64
/// b = 2^M always divides it
impl<const M: u64> Add for Zpow2<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let s = self.value.wrapping_add(rhs.value);
        Self::new(s & Self::mask())
    }
}

/// This works because (x mod a) mod b == x mod b <=> b|a
/// since wrapping_sub operate mod 2^64
/// b = 2^M always divides it
impl<const M: u64> Sub for Zpow2<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let s = self.value.wrapping_sub(rhs.value);
        Self::new(s & Self::mask())
    }
}

/// This works because (x mod a) mod b == x mod b <=> b|a
/// since wrapping_mul operate mod 2^64
/// b = 2^M always divides it
impl<const M: u64> Mul for Zpow2<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let p = self.value.wrapping_mul(rhs.value);
        Self::new(p & Self::mask())
    }
}

impl<const M: u64> Neg for Zpow2<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        // two's complement negation then mask
        let neg = (!self.value).wrapping_add(1);
        Self::new(neg & Self::mask())
    }
}

// Assign variants
impl<const M: u64> AddAssign for Zpow2<M> {
    fn add_assign(&mut self, rhs: Self) {
        let s = self.value.wrapping_add(rhs.value);
        self.value = s & Self::mask();
    }
}
impl<const M: u64> SubAssign for Zpow2<M> {
    fn sub_assign(&mut self, rhs: Self) {
        let s = self.value.wrapping_sub(rhs.value);
        self.value = s & Self::mask();
    }
}
impl<const M: u64> MulAssign for Zpow2<M> {
    fn mul_assign(&mut self, rhs: Self) {
        let p = self.value.wrapping_mul(rhs.value);
        self.value = p & Self::mask();
    }
}

// Provide num-traits Zero/One (nalgebra expects num_traits types)
impl<const M: u64> num_traits::Zero for Zpow2<M> {
    fn zero() -> Self { Self::new(0) }
    fn is_zero(&self) -> bool { self.value == 0 }
}
impl<const M: u64> num_traits::One for Zpow2<M> {
    fn one() -> Self { Self::new(1) }
}

// Ergonomics
impl<const M: u64> From<u64> for Zpow2<M> {
    fn from(x: u64) -> Self { Self::new(x) }
}

impl<const M: u64> Into<u64> for Zpow2<M> {
    fn into(self) -> u64 {
        self.value
    }
}

impl<const M: u64> fmt::Display for Zpow2<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Allow summing an iterator of &'a Zpow2<M> into a Zpow2<M>.
// The `'a` is a generic lifetime so this impl applies for any lifetime.
impl<'a, const M: u64> Sum<&'a Zpow2<M>> for Zpow2<M> {
    fn sum<I: Iterator<Item = &'a Zpow2<M>>>(iter: I) -> Self {
        // start from zero and add each referenced element
        iter.fold(Zpow2::zero(), |acc, x| acc + *x)
    }
}

// Also implement Sum for owned values (Iterator<Item = Zpow2<M>>).
impl<const M: u64> Sum<Zpow2<M>> for Zpow2<M> {
    fn sum<I: Iterator<Item = Zpow2<M>>>(iter: I) -> Self {
        iter.fold(Zpow2::zero(), |acc, x| acc + x)
    }
}

impl<const M: u64> Bounded for Zpow2<M> {
    fn min_value() -> Self {
        Self::zero()
    }

    fn max_value() -> Self {
        if M < 64 {
            Self::from((1u64 << M) - 1)
        } else {
            Self::from(u64::MAX)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Matrix3, Vector3};

    // exponent M=3 -> modulus 8
    type Z8 = Zpow2<3>;

    #[test]
    fn test_add() {
        let a = Z8::new(3);
        let b = Z8::new(5);
        assert_eq!((a + b).value(), 0); // 3+5=8 ≡ 0
    }

    #[test]
    fn test_sub() {
        let a = Z8::new(3);
        let b = Z8::new(5);
        assert_eq!((a - b).value(), 6); // 3-5=-2 ≡ 6
    }

    #[test]
    fn test_mul() {
        let a = Z8::new(3);
        let b = Z8::new(15);
        assert_eq!((a * b).value(), 5); // 45 ≡ 5
    }

    #[test]
    fn test_neg() {
        let a = Z8::new(3);
        assert_eq!((-a).value(), 5); // -3 ≡ 5
    }

    #[test]
    fn test_normalization() {
        let a = Z8::new(24); // 24 mod 8 = 0
        assert_eq!(a.value(), 0);
    }

    #[test]
    fn nalgebra_identity_preserves_vector() {
        let i: Matrix3<Z8> = Matrix3::identity();
        let v = Vector3::new(Z8::new(1), Z8::new(2), Z8::new(3));
        let r = i * v;
        assert_eq!(r, v);
    }

    #[test]
    fn nalgebra_diagonal_times_vector() {
        let d = Matrix3::new(
            Z8::new(2), Z8::new(0), Z8::new(0),
            Z8::new(0), Z8::new(2), Z8::new(0),
            Z8::new(0), Z8::new(0), Z8::new(2),
        );
        let v = Vector3::new(Z8::new(3), Z8::new(4), Z8::new(5));
        let r = d * v;
        assert_eq!(r, Vector3::new(Z8::new(6), Z8::new(0), Z8::new(2)));
    }

    #[test]
    fn nalgebra_matrix_mul() {
        let a = Matrix3::new(
            Z8::new(1), Z8::new(2), Z8::new(3),
            Z8::new(4), Z8::new(5), Z8::new(6),
            Z8::new(0), Z8::new(1), Z8::new(2),
        );
        let i: Matrix3<Z8> = Matrix3::identity();
        assert_eq!(a * i, a);
    }
}
