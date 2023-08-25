use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::bigint::BigUint;
use num::{Integer, One};
use plonky2::field::types::{Field, PrimeField, Sample};
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Ed25519Scalar(pub [u64; 4]);

fn biguint_from_array(arr: [u64; 4]) -> BigUint {
    BigUint::from_slice(&[
        arr[0] as u32,
        (arr[0] >> 32) as u32,
        arr[1] as u32,
        (arr[1] >> 32) as u32,
        arr[2] as u32,
        (arr[2] >> 32) as u32,
        arr[3] as u32,
        (arr[3] >> 32) as u32,
    ])
}

impl Default for Ed25519Scalar {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Ed25519Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.to_canonical_biguint() == other.to_canonical_biguint()
    }
}

impl Eq for Ed25519Scalar {}

impl Hash for Ed25519Scalar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_canonical_biguint().hash(state)
    }
}

impl Display for Ed25519Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.to_canonical_biguint(), f)
    }
}

impl Debug for Ed25519Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.to_canonical_biguint(), f)
    }
}

impl Sample for Ed25519Scalar {
    #[inline]
    fn sample<R>(rng: &mut R) -> Self
        where
            R: rand::RngCore + ?Sized,
    {
        use num::bigint::RandBigInt;
        Self::from_noncanonical_biguint(rng.gen_biguint_below(&Self::order()))
    }
}

impl Field for Ed25519Scalar {
    const ZERO: Self = Self([0; 4]);
    const ONE: Self = Self([1, 0, 0, 0]);
    const TWO: Self = Self([2, 0, 0, 0]);
    const NEG_ONE: Self = Self([
        0x5812631a5cf5d3ee,
        0x14def9dea2f79cd6,
        0x0000000000000000,
        0x1000000000000000,
    ]);

    const TWO_ADICITY: usize = 1;
    const CHARACTERISTIC_TWO_ADICITY: usize = Self::TWO_ADICITY;

    // Sage: `g = GF(p).multiplicative_generator()`
    const MULTIPLICATIVE_GROUP_GENERATOR: Self = Self([2, 0, 0, 0]);

    // Sage: `g_2 = g^((p - 1) / 2)`
    const POWER_OF_TWO_GENERATOR: Self = Self::NEG_ONE;

    const BITS: usize = 256;

    fn order() -> BigUint {
        BigUint::from_slice(&[
            0x5cf5d3ed, 0x5812631a, 0xa2f79cd6, 0x14def9de, 0x00000000, 0x00000000, 0x00000000,
            0x10000000,
        ])
    }
    fn characteristic() -> BigUint {
        Self::order()
    }

    fn try_inverse(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }

        // Fermat's Little Theorem
        Some(self.exp_biguint(&(Self::order() - BigUint::one() - BigUint::one())))
    }

    fn from_noncanonical_biguint(val: BigUint) -> Self {
        // Convert the BigUint value to its u64 digits
        let u64_digits = val.to_u64_digits();

        // Initialize a u64 array of length 4 with zeros
        let mut padded_digits: [u64; 4] = [0; 4];

        // Copy the u64 digits into the padded array
        for (i, &digit) in u64_digits.iter().enumerate() {
            if i >= 4 {
                break;
            }
            padded_digits[i] = digit;
        }

        Self(padded_digits)
    }

    #[inline]
    fn from_canonical_u64(_n: u64) -> Self {
        todo!()
    }

    #[inline]
    fn from_noncanonical_u128(_n: u128) -> Self {
        todo!()
    }

    #[inline]
    fn from_noncanonical_u96(_n: (u64, u32)) -> Self {
        todo!()
    }

    fn from_noncanonical_u64(_n: u64) -> Self {
        todo!()
    }

    fn from_noncanonical_i64(_n: i64) -> Self {
        todo!()
    }
}

impl PrimeField for Ed25519Scalar {
    fn to_canonical_biguint(&self) -> BigUint {
        let result = biguint_from_array(self.0);
        result.mod_floor(&Self::order())
    }
}

impl Neg for Ed25519Scalar {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        if self.is_zero() {
            Self::ZERO
        } else {
            Self::from_noncanonical_biguint(Self::order() - self.to_canonical_biguint())
        }
    }
}

impl Add for Ed25519Scalar {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let result = self.to_canonical_biguint() + rhs.to_canonical_biguint();
        Self::from_noncanonical_biguint(result.mod_floor(&Self::order()))
    }
}

impl AddAssign for Ed25519Scalar {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for Ed25519Scalar {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl Sub for Ed25519Scalar {
    type Output = Self;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl SubAssign for Ed25519Scalar {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Ed25519Scalar {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from_noncanonical_biguint(
            (self.to_canonical_biguint() * rhs.to_canonical_biguint()).mod_floor(&Self::order()),
        )
    }
}

impl MulAssign for Ed25519Scalar {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Product for Ed25519Scalar {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, x| acc * x).unwrap_or(Self::ONE)
    }
}

impl Div for Ed25519Scalar {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl DivAssign for Ed25519Scalar {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
