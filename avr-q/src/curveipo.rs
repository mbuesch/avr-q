// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

macro_rules! impl_curveipo {
    ($type:ty, $zero:expr) => {
        impl curveipo::CurvePoint<$type> for ($type, $type) {
            #[inline(always)]
            fn x(&self) -> $type {
                self.0
            }

            #[inline(always)]
            fn y(&self) -> $type {
                self.1
            }
        }

        impl curveipo::CurveIpo for $type {
            #[inline(never)]
            fn lin_inter(
                &self,
                left: &impl curveipo::CurvePoint<Self>,
                right: &impl curveipo::CurvePoint<Self>,
            ) -> Self {
                let dx = right.x() - left.x();
                let dy = right.y() - left.y();
                if dx == $zero {
                    left.y()
                } else {
                    ((*self - left.x()) * (dy / dx)) + left.y()
                }
            }
        }
    };
}

impl_curveipo!(crate::Q7p8, crate::q7p8!(const 0));
//TODO impl_curveipo!(crate::Q15p8, crate::q15p8!(const 0));

// vim: ts=4 sw=4 expandtab
