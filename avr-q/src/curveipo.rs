// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

macro_rules! impl_curveipo {
    ($type:ty) => {
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
                if dx == <$type>::ZERO {
                    return left.y();
                }
                let dxs = *self - left.x();

                // The quotient (dy / dx) is truncated to 1/256 steps.
                // This error is scaled up by dxs in the plain formula
                // (dxs * (dy / dx)), which hurts if the slope is small.
                // Pick the arrangement with the smallest error instead.

                if dy.abs() <= dx.abs() && dx.abs() >= <$type>::ONE {
                    // Shallow slope and not a tiny dx:
                    // Multiply first and divide afterwards.
                    // Not possible, if the product saturates.
                    let num = dxs * dy;
                    if num != <$type>::MAX && num != <$type>::MIN {
                        return (num / dx) + left.y();
                    }
                }

                if dy.abs() <= dxs.abs() {
                    // The ratio (dxs / dx) has an error of dy / 256 only.
                    ((dxs / dx) * dy) + left.y()
                } else {
                    // The slope (dy / dx) has an error of dxs / 256 only.
                    (dxs * (dy / dx)) + left.y()
                }
            }
        }
    };
}

impl_curveipo!(crate::Q7p8);
//TODO impl_curveipo!(crate::Q15p8);

// vim: ts=4 sw=4 expandtab
