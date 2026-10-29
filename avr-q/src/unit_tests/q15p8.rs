// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use super::{TestOps, test_assert};
use crate::{Q15p8, q15p8};
use avr_int24::is24;

const MIN: Q15p8 = Q15p8::from_q(is24::from_i32(-0x800000));
const MAX: Q15p8 = Q15p8::from_q(is24::from_i32(0x7fffff));
const ZERO: Q15p8 = Q15p8::from_q(is24::from_i32(0));
const ONE_LSB: Q15p8 = Q15p8::from_q(is24::from_i32(1));

fn test_base(t: &impl TestOps) {
    t.begin("base");

    let a = q15p8!(const 10 / 2).to_q().to_i32();
    let b = q15p8!(10 / 2).to_q().to_i32();
    let c = q15p8!(const 5).to_q().to_i32();
    let d = q15p8!(5).to_q().to_i32();
    test_assert!(t, a == 0x000500 && a == b && b == c && c == d);

    let a = q15p8!(const 3 / 2).to_q().to_i32();
    test_assert!(t, a == 0x000180);

    let a: Q15p8 = 9000_i16.into();
    let b: i16 = a.to_int();
    test_assert!(t, a.to_q().to_i32() == 0x232800 && b == 9000);

    let num = 7_i16;
    let den = 3_i16;
    test_assert!(t, q15p8!(num / den) == q15p8!(const 7 / 3));
    test_assert!(t, Q15p8::from_fraction(num, den) == q15p8!(const 7 / 3));

    let min16: Q15p8 = i16::MIN.into();
    let max16: Q15p8 = i16::MAX.into();
    test_assert!(
        t,
        min16.to_q().to_i32() == -0x800000 && min16.to_int() == i16::MIN
    );
    test_assert!(
        t,
        max16.to_q().to_i32() == 0x7fff00 && max16.to_int() == i16::MAX
    );
}

fn test_add(t: &impl TestOps) {
    t.begin("add");

    let a = q15p8!(const 7000 / 3);
    let b = q15p8!(const 1000 / 3);
    let c = q15p8!(const 8000 / 3);
    test_assert!(t, a + b == c);
    test_assert!(t, a.const_add(b) == c);
}

fn test_sub(t: &impl TestOps) {
    t.begin("sub");

    let a = q15p8!(const 7000 / 3);
    let b = q15p8!(const 1000 / 3);
    let c = q15p8!(const 6000 / 3);
    test_assert!(t, a - b == c);
    test_assert!(t, a.const_sub(b) == c);
}

fn test_mul(t: &impl TestOps) {
    t.begin("mul");

    /*
    let a = q15p8!(const 1000 / 2);
    let b = q15p8!(const 1 / 4);
    let c = q15p8!(const 1000 / 8);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);
    */
}

fn test_div(t: &impl TestOps) {
    t.begin("div");

    let a = q15p8!(const 1000 / 2);
    let b = q15p8!(const 4 / 1);
    let c = q15p8!(const 1000 / 8);
    test_assert!(t, a / b == c);
    test_assert!(t, a.const_div(b) == c);
}

fn test_neg(t: &impl TestOps) {
    t.begin("neg");

    let a = q15p8!(const 18000 / 5);
    let b = q15p8!(const -18000 / 5);
    test_assert!(t, -a == b);
    test_assert!(t, a.const_neg() == b);
}

fn test_abs(t: &impl TestOps) {
    t.begin("abs");

    let a = q15p8!(const 18000 / 5);
    let b = q15p8!(const 18000 / 5);
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);

    let a = q15p8!(const -18000 / 5);
    let b = q15p8!(const 18000 / 5);
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);
}

fn test_boundaries(t: &impl TestOps) {
    t.begin("boundaries");

    test_assert!(t, MAX + ONE_LSB == MAX);
    test_assert!(t, MAX.const_add(ONE_LSB) == MAX);
    test_assert!(t, MIN - ONE_LSB == MIN);
    test_assert!(t, MIN.const_sub(ONE_LSB) == MIN);

    test_assert!(t, -MIN == MAX);
    test_assert!(t, MIN.const_neg() == MAX);
    test_assert!(t, MIN.abs() == MAX);
    test_assert!(t, MIN.const_abs() == MAX);

    test_assert!(t, MAX / ONE_LSB == MAX);
    test_assert!(t, MAX.const_div(ONE_LSB) == MAX);
    test_assert!(t, MIN / ONE_LSB == MIN);
    test_assert!(t, MIN.const_div(ONE_LSB) == MIN);
    test_assert!(t, MAX / ZERO == MAX);
    test_assert!(t, MAX.const_div(ZERO) == MAX);
    test_assert!(t, MIN / ZERO == MIN);
    test_assert!(t, MIN.const_div(ZERO) == MIN);
    test_assert!(t, ZERO / ZERO == MAX);
    test_assert!(t, ZERO.const_div(ZERO) == MAX);
}

pub fn test_q15p8(t: &impl TestOps) {
    t.print("q15p8\n");
    test_base(t);
    test_add(t);
    test_sub(t);
    test_mul(t);
    test_div(t);
    test_neg(t);
    test_abs(t);
    test_boundaries(t);
}

// vim: ts=4 sw=4 expandtab
