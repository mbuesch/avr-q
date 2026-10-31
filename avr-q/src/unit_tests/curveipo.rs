// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use super::{TestOps, test_assert};
use crate::{Q7p8, q7p8};
use curveipo::CurveIpo;

/// Compare raw Q7.8 values with a tolerance of one LSB.
fn close(a: i16, b: i16) -> bool {
    (a as i32 - b as i32).abs() <= 1
}

fn test_wide_segment(t: &impl TestOps) {
    t.begin("curveipo wide");

    let left = (q7p8!(const -60), q7p8!(const 0));
    let right = (q7p8!(const 60), q7p8!(const 50));
    let point = q7p8!(const 0);
    let expected = q7p8!(const 25).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let left = (q7p8!(const -60), q7p8!(const -60));
    let right = (q7p8!(const 60), q7p8!(const 60));
    let point = q7p8!(const 50);
    let expected = q7p8!(const 50).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let point = q7p8!(const 59);
    let expected = q7p8!(const 59).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let left = (q7p8!(const 0), q7p8!(const -60));
    let right = (q7p8!(const 10), q7p8!(const 60));
    let point = q7p8!(const 5);
    let expected = 0;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));
}

fn test_shallow_slope(t: &impl TestOps) {
    t.begin("curveipo shallow");

    let left = (q7p8!(const 0), q7p8!(const 0));
    let right = (q7p8!(const 100), q7p8!(const 1));
    let point = q7p8!(const 50);
    let expected = q7p8!(const 1 / 2).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let right = (q7p8!(const 100), q7p8!(const 1 / 2));
    let point = q7p8!(const 99);
    let expected = 127;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let right = (q7p8!(const 127), q7p8!(const 1 / 4));
    let point = q7p8!(const 127);
    let expected = q7p8!(const 1 / 4).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let point = q7p8!(const 64);
    let expected = q7p8!(const 1 / 8).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));
}

fn test_slope_and_degenerate(t: &impl TestOps) {
    t.begin("curveipo neg slope");

    let left = (q7p8!(const 0), q7p8!(const 50));
    let right = (q7p8!(const 100), q7p8!(const -50));
    let point = q7p8!(const 25);
    let expected = q7p8!(const 25).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let left = (q7p8!(const 5), q7p8!(const 7));
    let right = (q7p8!(const 5), q7p8!(const 9));
    let point = q7p8!(const 5);
    let expected = q7p8!(const 7).to_q();
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let left = (Q7p8::from_q(0), Q7p8::from_q(0));
    let right = (Q7p8::from_q(2), Q7p8::from_q(1));
    let point = Q7p8::from_q(1);
    let expected = 1;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let right = (Q7p8::from_q(2), Q7p8::from_q(-1));
    let expected = -1;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));
}

fn test_saturation(t: &impl TestOps) {
    t.begin("curveipo sat");

    let left = (q7p8!(const 0), q7p8!(const 0));
    let right = (q7p8!(const 1), q7p8!(const 100));
    let point = q7p8!(const 100);
    let expected = i16::MAX;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let right = (q7p8!(const 1), q7p8!(const -100));
    let expected = i16::MIN;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));

    let left = (Q7p8::from_q(i16::MIN), Q7p8::from_q(i16::MIN));
    let right = (Q7p8::from_q(i16::MAX), Q7p8::from_q(i16::MAX));
    let point = Q7p8::from_q(0);
    let expected = 0;
    let r = point.lin_inter(&left, &right).to_q();
    test_assert!(t, close(r, expected));
}

pub fn test_curveipo(t: &impl TestOps) {
    test_wide_segment(t);
    test_shallow_slope(t);
    test_slope_and_degenerate(t);
    test_saturation(t);
}

// vim: ts=4 sw=4 expandtab
