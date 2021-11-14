// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::layouts::guess_layout::GuessLayout;
use crate::layouts::layout::Layout;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use float_next_after::NextAfter;

const SMALLEST_POSITIVE_NAN: f64 = f64::from_bits(0x7ff0000000000001);

const GREATEST_POSITIVE_NAN: f64 = f64::from_bits(0x7fffffffffffffff);
pub struct LayoutTestUtil {}

impl Preconditions for LayoutTestUtil {}

impl Algorithms for LayoutTestUtil {}

impl LayoutTestUtil {
    fn new() -> LayoutTestUtil {}

    fn next_up(value: f64) -> f64 {
        if Self::to_bits_nan_collapse(value) == Self::to_bits_nan_collapse(-0.0) {
            return 0.0;
        }
        // Java Math::next_up(value) has no counterpart in Rust.
        // closest is float_next_after crate.
        return float_next_after::NextAfter::next_after(value, f64::INFINITY);
    }

    fn next_down(value: f64) -> f64 {
        if Self::to_bits_nan_collapse(value) == Self::to_bits_nan_collapse(0.0) {
            return -0.0;
        }

        return value.next_after(std::f64::NEG_INFINITY);
    }

    // The Java implementation types indices as `int`, or `i32` in Rust.
    // However, we are referring to array indices, and Rust only accepts
    // `usize` when indexing.
    fn valid_nan_index(
        layout: impl Layout,
        idx: usize,
    ) -> std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)> {
        let under_flow_index = layout.get_underflow_bin_index();
        let over_flow_index = layout.get_underflow_bin_index();
        let check = idx >= over_flow_index || idx <= under_flow_index;
        let pair = std::sync::Arc::new((std::sync::Mutex::new(check), std::sync::Condvar::new()));
        return pair;
    }

    fn valid_pos_inf_index(
        layout: impl Layout,
        value: usize,
    ) -> std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)> {
        let over_flow_index: i32 = layout.get_underflow_bin_index();
        let check = value >= over_flow_index;
        let pair = std::sync::Arc::new((std::sync::Mutex::new(check), std::sync::Condvar::new()));
        return pair;
    }

    fn valid_neg_inf_index(
        layout: impl Layout,
        value: usize,
    ) -> std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)> {
        let under_flow_index: usize = layout.get_underflow_bin_index();
        let check = value <= under_flow_index;
        let pair = std::sync::Arc::new((std::sync::Mutex::new(check), std::sync::Condvar::new()));
        return pair;
    }

    fn assert_consistency(layout: impl Layout) {
        approx::relative_eq!(
            &layout.get_bin_lower_bound(&layout.get_underflow_bin_index()),
            &layout.get_bin_lower_bound(layout.get_underflow_bin_index() - 1)
        );
        approx::relative_eq!(
            &layout.get_bin_lower_bound(&layout.get_underflow_bin_index()),
            &layout.get_bin_lower_bound(i32::MIN)
        );
        approx::relative_eq!(
            &layout.get_bin_upper_bound(&layout.get_underflow_bin_index()),
            &layout.get_bin_upper_bound(layout.get_underflow_bin_index() - 1)
        );
        approx::relative_eq!(
            &layout.get_bin_upper_bound(&layout.get_underflow_bin_index()),
            &layout.get_bin_upper_bound(i32::MIN)
        );
        approx::relative_eq!(
            &layout.get_bin_lower_bound(&layout.get_overflow_bin_index()),
            &layout.get_bin_lower_bound(layout.get_overflow_bin_index() + 1)
        );
        approx::relative_eq!(
            &layout.get_bin_lower_bound(&layout.get_overflow_bin_index()),
            &layout.get_bin_lower_bound(i32::MAX)
        );
        approx::relative_eq!(
            &layout.get_bin_upper_bound(&layout.get_overflow_bin_index()),
            &layout.get_bin_upper_bound(layout.get_overflow_bin_index() + 1)
        );
        approx::relative_eq!(
            &layout.get_bin_upper_bound(&layout.get_overflow_bin_index()),
            &layout.get_bin_upper_bound(i32::MAX)
        );
        {
            let mut i: i32 = layout.get_underflow_bin_index();
            while i <= layout.get_overflow_bin_index() {
                {
                    let lower_bound: f64 = layout.get_bin_lower_bound(i);
                    if i == layout.get_underflow_bin_index() {
                        approx::relative_eq!(f64::NEG_INFINITY, lower_bound);
                    } else {
                        approx::relative_eq!(i, &layout.map_to_bin_index(lower_bound));
                        approx::relative_eq!(
                            i - 1,
                            &layout.map_to_bin_index(&float_next_after::NextAfter::next_after(
                                lower_bound,
                                f64::NEG_INFINITY
                            ))
                        );
                    }
                    let upper_bound: f64 = layout.get_bin_upper_bound(i);
                    if i == layout.get_overflow_bin_index() {
                        approx::relative_eq!(f64::INFINITY, upper_bound);
                    } else {
                        assert_eq!(i, &layout.map_to_bin_index(upper_bound));
                        assert_eq!(
                            i + 1,
                            &layout.map_to_bin_index(&float_next_after::NextAfter::next_after(
                                upper_bound,
                                f64::INFINITY
                            ))
                        );
                    }
                }
                i += 1;
            }
        }

        approx::relative_eq!(
            &layout.get_bin_lower_bound(layout.get_underflow_bin_index() + 1),
            &layout.get_normal_range_lower_bound()
        );
        approx::relative_eq!(
            &layout.get_bin_upper_bound(layout.get_overflow_bin_index() - 1),
            &layout.get_normal_range_upper_bound()
        );
        assert!(Self::valid_pos_inf_index(
            layout,
            &layout.map_to_bin_index(f64::INFINITY)
        ));
        // assert_that(&layout.map_to_bin_index(f64::INFINITY)).is(&Self::valid_pos_inf_index(layout));
        assert!(Self::valid_nan_index(
            layout,
            &layout.map_to_bin_index(SMALLEST_POSITIVE_NAN)
        ));
        assert!(Self::valid_nan_index(
            layout,
            &layout.map_to_bin_index(f64::NAN)
        ));
        assert!(Self::valid_nan_index(
            layout,
            &layout.map_to_bin_index(GREATEST_POSITIVE_NAN)
        ));
        assert!(Self::valid_neg_inf_index(
            layout,
            &layout.map_to_bin_index(f64::NEG_INFINITY)
        ));
        assert!(Self::valid_nan_index(
            layout,
            &layout.map_to_bin_index() & f64::from_bits(0xfff0000000000001)
        ));
        assert!(Self::valid_nan_index(
            layout,
            &layout.map_to_bin_index(&f64::from_bits(0xfff8000000000000))
        ));
        assert!(Self::valid_nan_index(
            layout,
            &layout.map_to_bin_index(&f64::from_bits(0xffffffffffffffff))
        ));
    }

    fn calculate_lower_bound_approximation_offset(layout: impl GuessLayout, bin_idx: i32) -> i64 {
        let approximate_lower_bound: f64 = layout.get_bin_lower_bound_approximation(bin_idx);
        let exact_lower_bound: f64 = layout.get_bin_lower_bound(bin_idx);
        let approximate_lower_bound_long_representation: i64 =
            Self::map_double_to_long(approximate_lower_bound);
        let exact_lower_bound_long_representation: i64 =
            Self::map_double_to_long(exact_lower_bound);
        return std::cmp::max(
            num::CheckedSub::checked_sub(
                approximate_lower_bound_long_representation,
                exact_lower_bound_long_representation,
            ),
            num::CheckedSub::checked_sub(
                exact_lower_bound_long_representation,
                approximate_lower_bound_long_representation,
            ),
        );
    }

    fn max_lower_bound_approximation_offset(layout: impl GuessLayout) -> u64 {
        let start = layout.get_underflow_bin_index() + 1;
        let end = layout.get_overflow_bin_index() + 1;
        // This guarantees the NaN will not be present
        let int_stream: std::ops::Range<i32> = start..end;
        // Select the index of the max
        let maxed: Option<u64> = int_stream.iter().map(|index| i64::into(index)).max();
        match maxed {
            Some(i) => i,
            None => 0u64,
        }
    }
}
