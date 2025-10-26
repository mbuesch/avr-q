// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

mod q15p8;
mod q7p8;

pub trait TestOps {
    fn print(&self, text: &str);
    fn print_num(&self, value: u32);
    fn begin(&self, name: &str);
    fn assert(&self, line: u16, ok: bool);
}

macro_rules! test_assert {
    ($test:expr, $ok:expr) => {
        $test.assert(core::line!() as _, $ok);
    };
}
pub(crate) use test_assert;

pub fn run_tests(t: &impl TestOps) {
    t.print("\n\nBegin tests\n");
    q7p8::test_q7p8(t);
    q15p8::test_q15p8(t);
    t.print("Done!\n");
}

// vim: ts=4 sw=4 expandtab
