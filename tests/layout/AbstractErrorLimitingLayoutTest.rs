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

pub struct AbstractErrorLimitingLayoutTest {
}

impl AbstractErrorLimitingLayoutTest {

    pub fn  create_layout(&self,  absolute_bin_width_limit: f64,  relative_bin_width_limit: f64,  value_range_lower_bound: f64,  value_range_upper_bound: f64) -> AbstractLayout ;

    pub fn  assert_index_symmetry(&self,  idx: i32,  negative_idx: i32)   {
        assert_equals(-idx - 1, negative_idx);
    }

    #[test]
    pub fn  test1(&self)   {
         let layout: Layout = self.create_layout(1e-6, 0.001, 0, 1);
        assert_true(layout.get_underflow_bin_index() >= layout.map_to_bin_index(Double::NEGATIVE_INFINITY));
        assert_true(layout.get_overflow_bin_index() <= layout.map_to_bin_index(Double::POSITIVE_INFINITY));
        assert_equals(Double::NEGATIVE_INFINITY, &layout.get_bin_lower_bound(&layout.get_underflow_bin_index()), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &layout.get_bin_upper_bound(&layout.get_overflow_bin_index()), 0.0);
    }

    #[test]
    pub fn  test_general(&self)   {
         let absolute_bin_width_limits: vec![Vec<f64>; 4] = vec![1e0, 1e1, 1e2, 1e3, ]
        ;
         let relative_bin_width_limits: vec![Vec<f64>; 8] = vec![0.0, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
         let value_range_lower_bound: f64 = -1e6;
         let value_range_upper_bound: f64 = 1e6;
         let num_values: i32 = 1_000;
         let eps: f64 = 1e-6;
         let mut values: [f64; num_values] = [0.0; num_values];
         let random: Random = Random::new(0);
         {
             let mut i: i32 = 0;
            while i < num_values {
                {
                    values[i] = value_range_lower_bound + random.next_double() * (value_range_upper_bound - value_range_lower_bound);
                }
                i += 1;
             }
         }

        for  let absolute_bin_width_limit: f64 in absolute_bin_width_limits {
            for  let relative_bin_width_limit: f64 in relative_bin_width_limits {
                 let layout: Layout = self.create_layout(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
                LayoutTestUtil::assert_consistency(layout);
                for  let value: f64 in values {
                     let idx: i32 = layout.map_to_bin_index(value);
                     let negative_idx: i32 = layout.map_to_bin_index(-value);
                    self.assert_index_symmetry(idx, negative_idx);
                     let lower_bound: f64 = layout.get_bin_lower_bound(idx);
                     let upper_bound: f64 = layout.get_bin_upper_bound(idx);
                    assert_that(lower_bound).is_less_than_or_equal_to(value);
                    assert_that(upper_bound).is_greater_than_or_equal_to(value);
                     let is_relative_bin_width_limit_fulfilled: bool = Math::abs(upper_bound - lower_bound) / Math::max(&Math::abs(lower_bound), &Math::abs(upper_bound)) <= relative_bin_width_limit * (1.0 + eps);
                     let is_absolute_bin_width_limit_fulfilled: bool = Math::abs(upper_bound - lower_bound) <= absolute_bin_width_limit * (1.0 + eps);
                    assert_true(is_absolute_bin_width_limit_fulfilled || is_relative_bin_width_limit_fulfilled);
                }
                 {
                     let mut i: i32 = layout.get_underflow_bin_index() + 1;
                    while i <= layout.get_overflow_bin_index() - 1 {
                        {
                             let lower_bound: f64 = layout.get_bin_lower_bound(i);
                             let upper_bound: f64 = layout.get_bin_upper_bound(i);
                             let is_relative_bin_width_limit_fulfilled: bool = Math::abs(upper_bound - lower_bound) / Math::max(&Math::abs(lower_bound), &Math::abs(upper_bound)) <= relative_bin_width_limit * (1.0 + eps);
                             let is_absolute_bin_width_limit_fulfilled: bool = Math::abs(upper_bound - lower_bound) <= absolute_bin_width_limit * (1.0 + eps);
                            assert_true(is_absolute_bin_width_limit_fulfilled || is_relative_bin_width_limit_fulfilled);
                        }
                        i += 1;
                     }
                 }

            }
        }
    }

    #[test]
    pub fn  test_large_layout(&self)   {
         let layout: Layout = self.create_layout(1e-6, 1e-3, -1e12, 1e12);
         {
             let bin_index: i32 = layout.get_underflow_bin_index() + 1;
            while bin_index < layout.get_overflow_bin_index() {
                {
                    assert_equals(bin_index, &layout.map_to_bin_index(&layout.get_bin_lower_bound(bin_index)));
                    assert_equals(bin_index, &layout.map_to_bin_index(&layout.get_bin_upper_bound(bin_index)));
                }
                bin_index += 1;
             }
         }

    }

    #[test]
    pub fn  test_get_bin_lower_bound_approximation(&self)   {
         let absolute_bin_width_limit: f64 = 1;
         let relative_bin_width_limit: f64 = 0.01;
         let eps: f64 = 1e-4;
         let layout: AbstractLayout = self.create_layout(absolute_bin_width_limit, relative_bin_width_limit, 0, 2000);
         {
             let transition_idx: i32 = 0;
            while transition_idx <= layout.get_overflow_bin_index() {
                {
                     let transition: f64 = layout.get_bin_lower_bound_approximation(transition_idx);
                     let transition_low: f64 = Math::min(transition * (1.0 - eps * relative_bin_width_limit), transition - eps * absolute_bin_width_limit);
                     let transition_high: f64 = Math::max(transition * (1.0 + eps * relative_bin_width_limit), transition + eps * absolute_bin_width_limit);
                     let bin_index_low: i32 = transition_idx - 1;
                     let bin_index_high: i32 = transition_idx;
                    assert_equals(bin_index_low, &layout.map_to_bin_index(transition_low));
                    assert_equals(bin_index_high, &layout.map_to_bin_index(transition_high));
                }
                transition_idx += 1;
             }
         }

    }

    #[test]
    pub fn  test_create_equidistant_layout(&self)   {
         let absolute_error_limits: vec![Vec<f64>; 4] = vec![Double::MIN_NORMAL, 1.0, 100.0, Double::MAX_VALUE / Integer::MAX_VALUE, ]
        ;
        for  let absolute_error_limit: f64 in absolute_error_limits {
            self.create_layout(absolute_error_limit, 0, 0, absolute_error_limit * (Integer::MAX_VALUE - 1.0));
            assert_throws(IllegalArgumentException.class, () -> self.create_layout(absolute_error_limit, 0, 0, absolute_error_limit * Integer::MAX_VALUE));
        }
    }

    #[test]
    pub fn  test_create(&self)   {
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-2, -1e6, Double::POSITIVE_INFINITY));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-2, Double::NEGATIVE_INFINITY, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-2, 1e6, 1e-6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-2, 1, Double::NaN));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-2, Double::NaN, 1));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-10, 1e-6, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-10, Long::MIN_VALUE, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, 1e-9, 1e-6, 1e6));
        self.create_layout(Double::MIN_NORMAL, 0, 0, 1000 * Double::MIN_NORMAL);
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(Double::MIN_NORMAL, 0, 0, Double::MAX_VALUE));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(-1, 1e-2, -1e6, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1e-8, -1, -1e6, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(Double::POSITIVE_INFINITY, 1, -1e6, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1, Double::POSITIVE_INFINITY, -1e6, 1e6));
        assert_throws(IllegalArgumentException.class, () -> self.create_layout(1, 0, -2, Integer::MAX_VALUE - 3));
        // no exception should be thrown in this case
        self.create_layout(1, 0, -2, Integer::MAX_VALUE - 4);
    }

    #[test]
    pub fn  test_same_equals(&self)   {
         let layout: Layout = self.create_layout(1e-8, 1e-2, -1e6, 1e6);
        assert_equals(layout, layout);
    }
}

