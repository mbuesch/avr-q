// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use super::{TestOps, test_assert};
use crate::{Q7p8, q7p8};

const MIN: Q7p8 = Q7p8::from_q(i16::MIN);
const MAX: Q7p8 = Q7p8::from_q(i16::MAX);
const ZERO: Q7p8 = Q7p8::from_q(0);
const ONE_LSB: Q7p8 = Q7p8::from_q(1);

fn test_base(t: &impl TestOps) {
    t.begin("base");

    let a = q7p8!(const 10 / 2).to_q();
    let b = q7p8!(10 / 2).to_q();
    let c = q7p8!(const 5).to_q();
    let d = q7p8!(5).to_q();
    test_assert!(t, a == 0x0500 && a == b && b == c && c == d);

    let a = q7p8!(const 3 / 2).to_q();
    test_assert!(t, a == 0x0180);

    let a: Q7p8 = 9_i8.into();
    let b: i8 = a.to_int();
    test_assert!(t, a.to_q() == 0x0900 && b == 9);

    let num = 7_i16;
    let den = 3_i16;
    test_assert!(t, q7p8!(num / den) == q7p8!(const 7 / 3));
    test_assert!(t, Q7p8::from_fraction(num, den) == q7p8!(const 7 / 3));

    let min16: Q7p8 = i8::MIN.into();
    let max16: Q7p8 = i8::MAX.into();
    test_assert!(t, min16.to_q() == i16::MIN && min16.to_int() == i8::MIN);
    test_assert!(t, max16.to_q() == 0x7f00 && max16.to_int() == i8::MAX);
}

fn test_add(t: &impl TestOps) {
    t.begin("add");

    let a = q7p8!(const 7 / 3);
    let b = q7p8!(const 1 / 3);
    let c = q7p8!(const 8 / 3);
    test_assert!(t, a + b == c);
}

fn test_sub(t: &impl TestOps) {
    t.begin("sub");

    let a = q7p8!(const 7 / 3);
    let b = q7p8!(const 1 / 3);
    let c = q7p8!(const 6 / 3);
    test_assert!(t, a - b == c);
}

fn test_mul(t: &impl TestOps) {
    t.begin("mul");

    let a = q7p8!(const 1 / 2);
    let b = q7p8!(const 1 / 4);
    let c = q7p8!(const 1 / 8);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);
}

fn test_div(t: &impl TestOps) {
    t.begin("div");

    let a = q7p8!(const 1 / 2);
    let b = q7p8!(const 4 / 1);
    let c = q7p8!(const 1 / 8);
    test_assert!(t, a / b == c);
    test_assert!(t, a.const_div(b) == c);
}

fn test_neg(t: &impl TestOps) {
    t.begin("neg");

    let a = q7p8!(const 18 / 5);
    let b = q7p8!(const -18 / 5);
    test_assert!(t, -a == b);
}

fn test_abs(t: &impl TestOps) {
    t.begin("abs");

    let a = q7p8!(const 18 / 5);
    let b = q7p8!(const 18 / 5);
    test_assert!(t, a.abs() == b);

    let a = q7p8!(const -18 / 5);
    let b = q7p8!(const 18 / 5);
    test_assert!(t, a.abs() == b);
}

fn test_boundaries(t: &impl TestOps) {
    t.begin("boundaries");

    test_assert!(t, MAX + ONE_LSB == MAX);
    test_assert!(t, MAX.add(ONE_LSB) == MAX);
    test_assert!(t, MIN - ONE_LSB == MIN);
    test_assert!(t, MIN.sub(ONE_LSB) == MIN);

    test_assert!(t, MAX * MAX == MAX);
    test_assert!(t, MAX.const_mul(MAX) == MAX);
    test_assert!(t, MIN * MAX == MIN);
    test_assert!(t, MIN.const_mul(MAX) == MIN);

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

    test_assert!(t, -MIN == MAX);
    test_assert!(t, MIN.abs() == MAX);
}

pub fn test_q7p8(t: &impl TestOps) {
    t.print("q7p8\n");
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
