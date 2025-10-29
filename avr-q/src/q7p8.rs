// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use avr_int24::Int24;

/// Q7.8 fixed point number.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Q7p8(i16);

/// Construct a Q7.8 fixed point number.
///
/// The argument to this macro can be a single one.
/// In this case it is the integer part of the Q7.8 value and the fractional part is zero.
///
/// `q7p8!(42)`
///
/// Or the arguments can be two numbers, separated by a slash.
/// In this case the value is a fraction with a numerator and a denominator.
///
/// `q7p8!(42 / 10)`
///
/// The arguments can be prefixed with `const` to enforce `const` evaluation.
///
/// `q7p8!(const 42 / 10)`
///
/// The arguments can be either literals or identifiers.
#[macro_export]
macro_rules! q7p8 {
    (const $numerator:literal / $denominator:literal) => {
        const { $crate::Q7p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:literal / $denominator:literal) => {
        const { $crate::Q7p8::const_from_fraction($numerator, $denominator) }
    };

    (const $numerator:literal / $denominator:ident) => {
        const { $crate::Q7p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:literal / $denominator:ident) => {
        $crate::Q7p8::from_fraction($numerator, $denominator)
    };

    (const $numerator:ident / $denominator:literal) => {
        const { $crate::Q7p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:ident / $denominator:literal) => {
        $crate::Q7p8::from_fraction($numerator, $denominator)
    };

    (const $numerator:ident / $denominator:ident) => {
        const { $crate::Q7p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:ident / $denominator:ident) => {
        $crate::Q7p8::from_fraction($numerator, $denominator)
    };

    (const $numerator:literal) => {
        const { $crate::Q7p8::from_int($numerator) }
    };
    ($numerator:literal) => {
        const { $crate::Q7p8::from_int($numerator) }
    };

    (const $numerator:ident) => {
        const { $crate::Q7p8::from_int($numerator) }
    };
    ($numerator:ident) => {
        $crate::Q7p8::from_int($numerator)
    };
}

#[allow(clippy::should_implement_trait)]
impl Q7p8 {
    /// Length of the fractional part, in bits.
    pub const SHIFT: usize = 8;

    /// Convert a raw Q7.8 value to [Q7p8].
    pub const fn from_q(q: i16) -> Self {
        Self(q)
    }

    /// Convert an integer value to [Q7p8] with fractional part being zero.
    pub const fn from_int(int: i8) -> Self {
        Self::from_q((int as i16) << Self::SHIFT)
    }

    /// Convert a numerator/denominator fraction to [Q7p8].
    pub fn from_fraction(numerator: i16, denominator: i16) -> Self {
        Self(numerator) / Self(denominator)
    }

    /// Convert a numerator/denominator fraction to [Q7p8].
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q7p8::from_fraction] instead.
    pub const fn const_from_fraction(numerator: i16, denominator: i16) -> Self {
        Self(numerator).const_div(Self(denominator))
    }

    /// Convert this [Q7p8] to a raw Q7.8 value.
    pub const fn to_q(self) -> i16 {
        self.0
    }

    /// Extract the integer part out of this [Q7p8].
    pub const fn to_int(self) -> i8 {
        (self.to_q() >> Self::SHIFT) as i8
    }

    /// Convert this [Q7p8] to a [crate::Q15p8].
    pub const fn to_q15p8(&self) -> crate::Q15p8 {
        crate::Q15p8::from_q(Int24::from_i16(self.to_q()))
    }

    /// Add and saturate two [Q7p8] values.
    #[inline(never)]
    pub const fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Subtract and saturate two [Q7p8] values.
    #[inline(never)]
    pub const fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    /// Multiply and saturate two [Q7p8] values.
    #[inline(never)]
    pub fn mul(self, other: Self) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        let a = Int24::from_i16(self.0);
        let b = Int24::from_i16(other.0);
        let c = (a * b).shr8();
        Self(c.to_i16())
    }

    /// Multiply and saturate two [Q7p8] values.
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q7p8::mul] instead.
    pub const fn const_mul(self, other: Self) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        let a = Int24::from_i16(self.0);
        let b = Int24::from_i16(other.0);
        let c = (a.const_mul(b)).shr8();
        Self(c.to_i16())
    }

    /// Divide and saturate two [Q7p8] values.
    #[inline(never)]
    pub fn div(self, other: Self) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        let a = Int24::from_i16(self.0);
        let b = Int24::from_i16(other.0);
        let c = a.shl8() / b;
        Self(c.to_i16())
    }

    /// Divide and saturate two [Q7p8] values.
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q7p8::div] instead.
    pub const fn const_div(self, other: Self) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        let a = Int24::from_i16(self.0);
        let b = Int24::from_i16(other.0);
        let c = a.shl8().const_div(b);
        Self(c.to_i16())
    }

    /// Negate and saturate this [Q7p8] value.
    #[inline(never)]
    pub const fn neg(self) -> Self {
        Self(self.0.saturating_neg())
    }

    /// Get the absolute and saturated value of this [Q7p8].
    #[inline(never)]
    pub const fn abs(self) -> Self {
        Self(self.0.saturating_abs())
    }
}

impl From<i8> for Q7p8 {
    fn from(value: i8) -> Self {
        Self::from_int(value)
    }
}

impl From<Q7p8> for i8 {
    fn from(value: Q7p8) -> Self {
        value.to_int()
    }
}

impl From<crate::Q15p8> for Q7p8 {
    fn from(v: crate::Q15p8) -> Q7p8 {
        v.to_q7p8()
    }
}

impl core::ops::Add for Q7p8 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(self, other)
    }
}

impl core::ops::AddAssign for Q7p8 {
    fn add_assign(&mut self, other: Self) {
        self.0 = (*self + other).0;
    }
}

impl core::ops::Sub for Q7p8 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::sub(self, other)
    }
}

impl core::ops::SubAssign for Q7p8 {
    fn sub_assign(&mut self, other: Self) {
        self.0 = (*self - other).0;
    }
}

impl core::ops::Mul for Q7p8 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::mul(self, other)
    }
}

impl core::ops::MulAssign for Q7p8 {
    fn mul_assign(&mut self, other: Self) {
        self.0 = (*self * other).0;
    }
}

impl core::ops::Div for Q7p8 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::div(self, other)
    }
}

impl core::ops::DivAssign for Q7p8 {
    fn div_assign(&mut self, other: Self) {
        self.0 = (*self / other).0;
    }
}

impl core::ops::Neg for Q7p8 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}

// vim: ts=4 sw=4 expandtab
