// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use avr_int24::Int24;

/// Q7.8 fixed point number.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Q7p8(i16);

/// Construct a  Q7.8 fixed point number.
#[macro_export]
macro_rules! q7p8 {
    (const $numerator:literal / $denominator:literal) => {
        const { $crate::Q7p8::const_from_fraction($numerator, $denominator) }
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::Q7p8::from_fraction($numerator, $denominator)
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
        $crate::Q7p8::from_int($numerator)
    };

    (const $numerator:ident) => {
        const { $crate::Q7p8::from_int($numerator) }
    };
    ($numerator:ident) => {
        $crate::Q7p8::from_int($numerator)
    };
}

impl Q7p8 {
    pub const SHIFT: usize = 8;

    pub const fn from_q(q: i16) -> Self {
        Self(q)
    }

    pub const fn from_int(int: i16) -> Self {
        Self::from_q(int << Self::SHIFT)
    }

    pub const fn const_from_fraction(numerator: i16, denominator: i16) -> Self {
        Self(numerator).const_div(Self(denominator))
    }

    pub fn from_fraction(numerator: i16, denominator: i16) -> Self {
        Self(numerator) / Self(denominator)
    }

    pub const fn to_q(self) -> i16 {
        self.0
    }

    pub const fn to_q15p8(&self) -> crate::Q15p8 {
        crate::Q15p8::from_q(Int24::from_i16(self.to_q()))
    }

    pub const fn to_int(self) -> i16 {
        self.to_q() >> Self::SHIFT
    }

    #[inline(never)]
    pub const fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    #[inline(never)]
    pub const fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

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

    pub const fn const_div(self, other: Self) -> Self {
        const {
            assert!(Self::SHIFT == 8);
        }
        let a = Int24::from_i16(self.0);
        let b = Int24::from_i16(other.0);
        let c = a.shl8().const_div(b);
        Self(c.to_i16())
    }

    #[inline(never)]
    pub const fn neg(self) -> Self {
        if self.0 == i16::MIN {
            Self(i16::MAX)
        } else {
            Self(-self.0)
        }
    }

    #[inline(never)]
    pub const fn abs(self) -> Self {
        if self.0 < 0 { self.neg() } else { self }
    }
}

impl From<u8> for Q7p8 {
    fn from(value: u8) -> Self {
        Self::from_int(value.into())
    }
}

impl From<i8> for Q7p8 {
    fn from(value: i8) -> Self {
        Self::from_int(value.into())
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
        Q7p8::add(self, other)
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
        Q7p8::sub(self, other)
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
        Q7p8::mul(self, other)
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
        Q7p8::div(self, other)
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
        Q7p8::neg(self)
    }
}

// vim: ts=4 sw=4 expandtab
