/*
 * Copyright 2020-2021 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::dynatrace::dynahist::layout;


const SMALLEST_POSITIVE_NAN: f64 = Double::long_bits_to_double(0x7ff0000000000001);

const GREATEST_POSITIVE_NAN: f64 = Double::long_bits_to_double(0x7fffffffffffffff);
pub struct LayoutTestUtil {
}

impl LayoutTestUtil {

    fn new() -> LayoutTestUtil {
    }

    fn  next_up( value: f64) -> f64  {
        if Double::double_to_long_bits(value) == Double::double_to_long_bits(-0.0) {
            return 0.0;
        }

        return Math::next_up(value);
    }

    fn  next_down( value: f64) -> f64  {
        if Double::double_to_long_bits(value) == Double::double_to_long_bits(0.0) {
            return -0.0;
        }

        return Math::next_down(value);
    }

    fn  valid_na_n_index( layout: &Layout) -> Condition<Integer>  {
         let under_flow_index: i32 = layout.get_underflow_bin_index();
         let over_flow_index: i32 = layout.get_underflow_bin_index();
        return Condition<>::new() {

            pub fn  matches(&self,  value: &Integer) -> bool  {
                return value >= over_flow_index || value <= under_flow_index;
            }
        };
    }

    fn  valid_pos_inf_index( layout: &Layout) -> Condition<Integer>  {
         let over_flow_index: i32 = layout.get_underflow_bin_index();
        return Condition<>::new() {

            pub fn  matches(&self,  value: &Integer) -> bool  {
                return value >= over_flow_index;
            }
        };
    }

    fn  valid_neg_inf_index( layout: &Layout) -> Condition<Integer>  {
         let under_flow_index: i32 = layout.get_underflow_bin_index();
        return Condition<>::new() {

            pub fn  matches(&self,  value: &Integer) -> bool  {
                return value <= under_flow_index;
            }
        };
    }

    pub fn  assert_consistency( layout: &Layout)   {
        assert_equals(&layout.get_bin_lower_bound(&layout.get_underflow_bin_index()), &layout.get_bin_lower_bound(layout.get_underflow_bin_index() - 1), 0.0);
        assert_equals(&layout.get_bin_lower_bound(&layout.get_underflow_bin_index()), &layout.get_bin_lower_bound(Integer::MIN_VALUE), 0.0);
        assert_equals(&layout.get_bin_upper_bound(&layout.get_underflow_bin_index()), &layout.get_bin_upper_bound(layout.get_underflow_bin_index() - 1), 0.0);
        assert_equals(&layout.get_bin_upper_bound(&layout.get_underflow_bin_index()), &layout.get_bin_upper_bound(Integer::MIN_VALUE), 0.0);
        assert_equals(&layout.get_bin_lower_bound(&layout.get_overflow_bin_index()), &layout.get_bin_lower_bound(layout.get_overflow_bin_index() + 1), 0.0);
        assert_equals(&layout.get_bin_lower_bound(&layout.get_overflow_bin_index()), &layout.get_bin_lower_bound(Integer::MAX_VALUE), 0.0);
        assert_equals(&layout.get_bin_upper_bound(&layout.get_overflow_bin_index()), &layout.get_bin_upper_bound(layout.get_overflow_bin_index() + 1), 0.0);
        assert_equals(&layout.get_bin_upper_bound(&layout.get_overflow_bin_index()), &layout.get_bin_upper_bound(Integer::MAX_VALUE), 0.0);
         {
             let mut i: i32 = layout.get_underflow_bin_index();
            while i <= layout.get_overflow_bin_index() {
                {
                     let lower_bound: f64 = layout.get_bin_lower_bound(i);
                    if i == layout.get_underflow_bin_index() {
                        assert_equals(Double::NEGATIVE_INFINITY, lower_bound, 0.0);
                    } else {
                        assert_equals(i, &layout.map_to_bin_index(lower_bound));
                        assert_equals(i - 1, &layout.map_to_bin_index(&::next_down(lower_bound)));
                    }
                     let upper_bound: f64 = layout.get_bin_upper_bound(i);
                    if i == layout.get_overflow_bin_index() {
                        assert_equals(Double::POSITIVE_INFINITY, upper_bound, 0.0);
                    } else {
                        assert_equals(i, &layout.map_to_bin_index(upper_bound));
                        assert_equals(i + 1, &layout.map_to_bin_index(&::next_up(upper_bound)));
                    }
                }
                i += 1;
             }
         }

        assert_equals(&layout.get_bin_lower_bound(layout.get_underflow_bin_index() + 1), &layout.get_normal_range_lower_bound(), 0.0);
        assert_equals(&layout.get_bin_upper_bound(layout.get_overflow_bin_index() - 1), &layout.get_normal_range_upper_bound(), 0.0);
        assert_that(&layout.map_to_bin_index(Double::POSITIVE_INFINITY)).is(&::valid_pos_inf_index(layout));
        assert_that(&layout.map_to_bin_index(SMALLEST_POSITIVE_NAN)).is(&::valid_na_n_index(layout));
        assert_that(&layout.map_to_bin_index(Double::NaN)).is(&::valid_na_n_index(layout));
        assert_that(&layout.map_to_bin_index(GREATEST_POSITIVE_NAN)).is(&::valid_na_n_index(layout));
        assert_that(&layout.map_to_bin_index(Double::NEGATIVE_INFINITY)).is(&::valid_neg_inf_index(layout));
        assert_that(&layout.map_to_bin_index(&Double::long_bits_to_double(0xfff0000000000001))).is(&::valid_na_n_index(layout));
        assert_that(&layout.map_to_bin_index(&Double::long_bits_to_double(0xfff8000000000000))).is(&::valid_na_n_index(layout));
        assert_that(&layout.map_to_bin_index(&Double::long_bits_to_double(0xffffffffffffffff))).is(&::valid_na_n_index(layout));
    }

    fn  calculate_lower_bound_approximation_offset( layout: &AbstractLayout,  bin_idx: i32) -> i64  {
         let approximate_lower_bound: f64 = layout.get_bin_lower_bound_approximation(bin_idx);
         let exact_lower_bound: f64 = layout.get_bin_lower_bound(bin_idx);
         let approximate_lower_bound_long_representation: i64 = Algorithms::map_double_to_long(approximate_lower_bound);
         let exact_lower_bound_long_representation: i64 = Algorithms::map_double_to_long(exact_lower_bound);
        return Math::max(&Math::subtract_exact(approximate_lower_bound_long_representation, exact_lower_bound_long_representation), &Math::subtract_exact(exact_lower_bound_long_representation, approximate_lower_bound_long_representation));
    }

    pub fn  max_lower_bound_approximation_offset( layout: &AbstractLayout) -> i64  {
        return IntStream::range(layout.get_underflow_bin_index() + 1, layout.get_overflow_bin_index() + 1)::map_to_long( bin_idx: & -> ::calculate_lower_bound_approximation_offset(layout, bin_idx))::max()::or_else(0);
    }
}
