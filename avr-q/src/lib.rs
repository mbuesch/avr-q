// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

//! # Fixed point arithmetic for AVR
//!
//! This library provides
//! [fixed-point arithmetic types](https://en.wikipedia.org/wiki/Q_(number_format))
//! for AVR microcontrollers.
//!
//! ## The fixed point formats supported are
//!
//! - **Q7.8 format**: A 16-bit fixed-point number with 7 integer bits and 8 fractional bits [Q7p8].
//! - **Q15.8 format**: A 24-bit fixed-point number with 15 integer bits and 8 fractional bits [Q15p8].
//!
//! ## The supported operations are
//!
//! - Basic arithmetic operations: addition, subtraction, multiplication, and division.
//! - Macros for easy construction of fixed-point numbers from integers or fractions.
//! - Conversions between fixed-point types and integer types.
//! - Optional `curveipo` feature for curve interpolation.
//!
//! ## Creating fixed-point numbers
//!
//! ```
//! use avr_q::{q7p8, Q7p8};
//!
//! // From an integer
//! let a = q7p8!(const 5);
//! assert_eq!(a.to_int(), 5);
//!
//! // From a fraction
//! let b = q7p8!(const 1 / 2); // 0.5
//! let c = q7p8!(const 10 / 3); // 3.33...
//!
//! // From variables
//! let numerator = 10_i16;
//! let denominator = 3_i16;
//! let d = q7p8!(numerator / denominator);
//! ```
//!
//! # Arithmetic operations
//!
//! ```
//! use avr_q::{q7p8, Q7p8};
//!
//! let a = q7p8!(const 1 / 2); // 0.5
//! let b = q7p8!(const 1 / 4); // 0.25
//!
//! let sum = a + b;
//! assert_eq!(sum, q7p8!(const 3 / 4)); // 0.75
//!
//! let diff = a - b;
//! assert_eq!(diff, q7p8!(const 1 / 4)); // 0.25
//!
//! let prod = a * b;
//! assert_eq!(prod, q7p8!(const 1 / 8)); // 0.125
//!
//! let quot = a / b;
//! assert_eq!(quot, q7p8!(const 2)); // 2.0
//!
//! let neg = -a;
//! assert_eq!(neg, q7p8!(const -1 / 2));
//!
//! let abs = neg.abs();
//! assert_eq!(abs, q7p8!(const 1 / 2));
//! ```
//!
//! ## Crate features
//!
//! - `curveipo` (enabled by default):
//!   The `curveipo` feature enables the implementations of all traits from the
//!   [curveipo crate](https://crates.io/crates/curveipo)
//!   for all fixed-point types.
//!   The `curveipo` crate provides 2D curve interpolation support.

#![cfg_attr(not(test), no_std)]

mod q15p8;
mod q7p8;

#[cfg(feature = "curveipo")]
mod curveipo;

#[cfg(any(feature = "__internal_test__", test))]
pub mod unit_tests;

pub use crate::{q7p8::Q7p8, q15p8::Q15p8};

#[cfg(test)]
mod test {
    use crate::unit_tests;

    struct TestRunner {}

    impl unit_tests::TestOps for TestRunner {
        fn print(&self, text: &str) {
            print!("{text}");
        }

        fn print_num(&self, value: u32) {
            print!("{value}");
        }

        fn begin(&self, name: &str) {
            println!("Begin: {name}");
        }

        fn assert(&self, line: u16, ok: bool) {
            if ok {
                println!("line {line}: Ok");
            } else {
                panic!("line {line}: FAILED");
            }
        }
    }

    #[test]
    fn test_q() {
        let t = TestRunner {};
        unit_tests::run_tests(&t);
    }
}

// vim: ts=4 sw=4 expandtab
