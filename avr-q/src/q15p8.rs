// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use avr_int24::I24;

/// Q15.8 fixed point number.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Q15p8(I24);

/// Construct a Q15.8 fixed point number.
///
/// The argument to this macro can be a single one.
/// In this case it is the integer part of the Q15.8 value and the fractional part is zero.
///
/// `q15p8!(42)`
///
/// Or the arguments can be two numbers, separated by a slash.
/// In this case the value is a fraction with a numerator and a denominator.
///
/// `q15p8!(42 / 10)`
///
/// The arguments can be prefixed with `const` to enforce `const` evaluation.
///
/// `q15p8!(const 42 / 10)`
///
/// The arguments can be either literals or identifiers.
#[macro_export]
macro_rules! q15p8 {
    (const $numerator:literal / $denominator:literal) => {
        const { $crate::Q15p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:literal / $denominator:literal) => {
        const { $crate::Q15p8::const_from_fraction($numerator, $denominator) }
    };

    (const $numerator:literal / $denominator:ident) => {
        const { $crate::Q15p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:literal / $denominator:ident) => {
        $crate::Q15p8::from_fraction($numerator, $denominator)
    };

    (const $numerator:ident / $denominator:literal) => {
        const { $crate::Q15p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:ident / $denominator:literal) => {
        $crate::Q15p8::from_fraction($numerator, $denominator)
    };

    (const $numerator:ident / $denominator:ident) => {
        const { $crate::Q15p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:ident / $denominator:ident) => {
        $crate::Q15p8::from_fraction($numerator, $denominator)
    };

    (const $numerator:literal) => {
        const { $crate::Q15p8::from_int($numerator) }
    };
    ($numerator:literal) => {
        const { $crate::Q15p8::from_int($numerator) }
    };

    (const $numerator:ident) => {
        const { $crate::Q15p8::from_int($numerator) }
    };
    ($numerator:ident) => {
        $crate::Q15p8::from_int($numerator)
    };
}

#[allow(clippy::should_implement_trait)]
impl Q15p8 {
    /// Length of the fractional part, in bits.
    pub const SHIFT: usize = 8;

    /// Convert a raw Q15.8 value to [Q15p8].
    pub const fn from_q(q: I24) -> Self {
        Self(q)
    }

    /// Convert an integer value to [Q15p8] with fractional part being zero.
    pub const fn from_int(int: i16) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        Self(I24::from_i16(int).shl8())
    }

    /// Convert a numerator/denominator fraction to [Q15p8].
    pub fn from_fraction(numerator: i16, denominator: i16) -> Self {
        Self(I24::from_i16(numerator)) / Self(I24::from_i16(denominator))
    }

    /// Convert a numerator/denominator fraction to [Q15p8].
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q15p8::from_fraction] instead.
    pub const fn const_from_fraction(numerator: i16, denominator: i16) -> Self {
        Self(I24::from_i16(numerator)).const_div(Self(I24::from_i16(denominator)))
    }

    /// Convert this [Q15p8] to a raw Q15.8 value.
    pub const fn to_q(self) -> I24 {
        self.0
    }

    /// Extract the integer part out of this [Q15p8].
    pub const fn to_int(self) -> i16 {
        const {
            assert!(Self::SHIFT == 8);
        }
        self.0.shr8().to_i16()
    }

    /// Convert this [Q15p8] to a [crate::Q7p8].
    pub const fn to_q7p8(&self) -> crate::Q7p8 {
        crate::Q7p8::from_q(self.0.to_i16())
    }

    /// Add and saturate two [Q15p8] values.
    pub fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    /// Add and saturate two [Q15p8] values.
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q15p8::add] instead.
    pub const fn const_add(self, other: Self) -> Self {
        Self(self.0.const_add(other.0))
    }

    /// Subtract and saturate two [Q15p8] values.
    pub fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }

    /// Subtract and saturate two [Q15p8] values.
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q15p8::sub] instead.
    pub const fn const_sub(self, other: Self) -> Self {
        Self(self.0.const_sub(other.0))
    }

    //TODO mul

    /// Divide and saturate two [Q15p8] values.
    pub fn div(self, other: Self) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        Self(self.0.shl8div(other.0))
    }

    /// Divide and saturate two [Q15p8] values.
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q15p8::div] instead.
    pub const fn const_div(self, other: Self) -> Self {
        let a = self.0.to_i32();
        let b = other.0.to_i32();
        let c = if b == 0 {
            if a < 0 { i32::MIN } else { i32::MAX }
        } else {
            (a << Self::SHIFT).saturating_div(b)
        };
        Self(I24::from_i32(c))
    }

    /// Negate and saturate this [Q15p8] value.
    pub fn neg(self) -> Self {
        Self(-self.0)
    }

    /// Negate and saturate this [Q15p8] value.
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q15p8::neg] instead.
    pub const fn const_neg(self) -> Self {
        Self(self.0.const_neg())
    }

    /// Get the absolute and saturated value of this [Q15p8].
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Get the absolute and saturated value of this [Q15p8].
    /// Const variant.
    ///
    /// Only call this function from const context.
    /// From non-const context use the optimized variant [Q15p8::abs] instead.
    pub const fn const_abs(self) -> Self {
        Self(self.0.const_abs())
    }
}

impl From<u8> for Q15p8 {
    fn from(value: u8) -> Self {
        Self::from_int(value.into())
    }
}

impl From<i8> for Q15p8 {
    fn from(value: i8) -> Self {
        Self::from_int(value.into())
    }
}

impl From<i16> for Q15p8 {
    fn from(value: i16) -> Self {
        Self::from_int(value)
    }
}

impl From<Q15p8> for i16 {
    fn from(value: Q15p8) -> Self {
        value.to_int()
    }
}

impl From<crate::Q7p8> for Q15p8 {
    fn from(v: crate::Q7p8) -> Q15p8 {
        v.to_q15p8()
    }
}

impl core::ops::Add for Q15p8 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(self, other)
    }
}

impl core::ops::AddAssign for Q15p8 {
    fn add_assign(&mut self, other: Self) {
        self.0 = (*self + other).0;
    }
}

impl core::ops::Sub for Q15p8 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::sub(self, other)
    }
}

impl core::ops::SubAssign for Q15p8 {
    fn sub_assign(&mut self, other: Self) {
        self.0 = (*self - other).0;
    }
}

impl core::ops::Div for Q15p8 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::div(self, other)
    }
}

impl core::ops::DivAssign for Q15p8 {
    fn div_assign(&mut self, other: Self) {
        self.0 = (*self / other).0;
    }
}

impl core::ops::Neg for Q15p8 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}

// vim: ts=4 sw=4 expandtab
