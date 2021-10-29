// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::Layout;

const SMALLEST_POSITIVE_NAN: f64 = f64::from_bits(0x7ff0000000000001);

const GREATEST_POSITIVE_NAN: f64 = f64::from_bits(0x7fffffffffffffff);
pub struct LayoutTestUtil {}

impl Preconditions for LayoutTestUtil {}

impl Algorithms for LayoutTestUtil {}

impl LayoutTestUtil {

    fn new() -> LayoutTestUtil {
    }

    fn next_up(value: f64) -> f64  {
        if Self::to_bits_nan_collapse(value) == Self::to_bits_nan_collapse(-0.0) {
            return 0.0;
        }
        // Java Math::next_up(value) has no counterpart in Rust.
        // closest is float_next_after crate.
        return value.next_after(std::f64::INFINITY);
    }

    fn next_down( value: f64) -> f64  {
        if Self::to_bits_nan_collapse(value) == Self::to_bits_nan_collapse(0.0) {
            return -0.0;
        }

        return value.next_after(std::f64::NEG_INFINITY);
    }

    // The Java implementation types indices as `int`, or `i32` in Rust.
    // However, since we are referring to array indices, and Rust only accepts
    // `usize` when indexing .
    fn valid_nan_index( layout: &Layout, idx: usize) -> bool  {
         let under_flow_index = layout.get_underflow_bin_index();
         let over_flow_index = layout.get_underflow_bin_index();
         let check = idx >= over_flow_index || idx <= under_flow_index;
         return check;
        // return Condition<>::new() {

        //     pub fn matches(&self,  value: &Integer) -> bool  {
        //         return value >= over_flow_index || value <= under_flow_index;
        //     }
        // };
    }

    fn valid_pos_inf_index( layout: &Layout, value: f64) -> Condition<Integer>  {
         let over_flow_index: i32 = layout.get_underflow_bin_index();
         let check = value >= over_flow_index;
         return check;
        // return Condition<>::new() {

        //     pub fn matches(&self,  value: &Integer) -> bool  {
        //         return value >= over_flow_index;
        //     }
        // };
    }

    fn valid_neg_inf_index( layout: &Layout) -> Condition<Integer>  {
         let under_flow_index: i32 = layout.get_underflow_bin_index();
         let check = value <= under_flow_index;
         return check;
        // return Condition<>::new() {

        //     pub fn matches(&self,  value: &Integer) -> bool  {
        //         return value <= under_flow_index;
        //     }
        // };
    }

    pub fn assert_consistency( layout: &Layout)   {
        assert_eq!(&layout.get_bin_lower_bound(&layout.get_underflow_bin_index()), &layout.get_bin_lower_bound(layout.get_underflow_bin_index() - 1), 0.0);
        assert_eq!(&layout.get_bin_lower_bound(&layout.get_underflow_bin_index()), &layout.get_bin_lower_bound(Integer::MIN_VALUE), 0.0);
        assert_eq!(&layout.get_bin_upper_bound(&layout.get_underflow_bin_index()), &layout.get_bin_upper_bound(layout.get_underflow_bin_index() - 1), 0.0);
        assert_eq!(&layout.get_bin_upper_bound(&layout.get_underflow_bin_index()), &layout.get_bin_upper_bound(Integer::MIN_VALUE), 0.0);
        assert_eq!(&layout.get_bin_lower_bound(&layout.get_overflow_bin_index()), &layout.get_bin_lower_bound(layout.get_overflow_bin_index() + 1), 0.0);
        assert_eq!(&layout.get_bin_lower_bound(&layout.get_overflow_bin_index()), &layout.get_bin_lower_bound(Integer::MAX_VALUE), 0.0);
        assert_eq!(&layout.get_bin_upper_bound(&layout.get_overflow_bin_index()), &layout.get_bin_upper_bound(layout.get_overflow_bin_index() + 1), 0.0);
        assert_eq!(&layout.get_bin_upper_bound(&layout.get_overflow_bin_index()), &layout.get_bin_upper_bound(Integer::MAX_VALUE), 0.0);
         {
             let mut i: i32 = layout.get_underflow_bin_index();
            while i <= layout.get_overflow_bin_index() {
                {
                     let lower_bound: f64 = layout.get_bin_lower_bound(i);
                    if i == layout.get_underflow_bin_index() {
                        assert_eq!(f64::NEG_INFINITY, lower_bound, 0.0);
                    } else {
                        assert_eq!(i, &layout.map_to_bin_index(lower_bound));
                        assert_eq!(i - 1, &layout.map_to_bin_index(&::next_down(lower_bound)));
                    }
                     let upper_bound: f64 = layout.get_bin_upper_bound(i);
                    if i == layout.get_overflow_bin_index() {
                        assert_eq!(f64::INFINITY, upper_bound, 0.0);
                    } else {
                        assert_eq!(i, &layout.map_to_bin_index(upper_bound));
                        assert_eq!(i + 1, &layout.map_to_bin_index(&::next_up(upper_bound)));
                    }
                }
                i += 1;
             }
         }

        assert_eq!(&layout.get_bin_lower_bound(layout.get_underflow_bin_index() + 1), &layout.get_normal_range_lower_bound(), 0.0);
        assert_eq!(&layout.get_bin_upper_bound(layout.get_overflow_bin_index() - 1), &layout.get_normal_range_upper_bound(), 0.0);
        assert!(valid_pos_inf_index(layout, &layout.map_to_bin_index(f64::INFINITY)));
        // assert_that(&layout.map_to_bin_index(f64::INFINITY)).is(&::valid_pos_inf_index(layout));
        assert_that(&layout.map_to_bin_index(SMALLEST_POSITIVE_NAN)).is(&::valid_nan_index(layout));
        assert_that(&layout.map_to_bin_index(f64::NAN)).is(&::valid_nan_index(layout));
        assert_that(&layout.map_to_bin_index(GREATEST_POSITIVE_NAN)).is(&::valid_nan_index(layout));
        assert_that(&layout.map_to_bin_index(f64::NEG_INFINITY)).is(&::valid_neg_inf_index(layout));
        assert_that(&layout.map_to_bin_index(&f64::from_bits(0xfff0000000000001))).is(&::valid_nan_index(layout));
        assert_that(&layout.map_to_bin_index(&f64::from_bits(0xfff8000000000000))).is(&::valid_nan_index(layout));
        assert_that(&layout.map_to_bin_index(&f64::from_bits(0xffffffffffffffff))).is(&::valid_nan_index(layout));
    }

    fn calculate_lower_bound_approximation_offset( layout: &GuessLayout,  bin_idx: i32) -> i64  {
         let approximate_lower_bound: f64 = layout.get_bin_lower_bound_approximation(bin_idx);
         let exact_lower_bound: f64 = layout.get_bin_lower_bound(bin_idx);
         let approximate_lower_bound_long_representation: i64 = Algorithms::map_double_to_long(approximate_lower_bound);
         let exact_lower_bound_long_representation: i64 = Algorithms::map_double_to_long(exact_lower_bound);
        return std::cmp::max(&Math::subtract_exact(approximate_lower_bound_long_representation, exact_lower_bound_long_representation), &Math::subtract_exact(exact_lower_bound_long_representation, approximate_lower_bound_long_representation));
    }

    pub fn max_lower_bound_approximation_offset( layout: &GuessLayout) -> i64  {
        return IntStream::range(layout.get_underflow_bin_index() + 1, layout.get_overflow_bin_index() + 1)::map_to_long( bin_idx: & -> ::calculate_lower_bound_approximation_offset(layout, bin_idx))::max()::or_else(0);
    }
}
