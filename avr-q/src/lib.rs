// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

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
