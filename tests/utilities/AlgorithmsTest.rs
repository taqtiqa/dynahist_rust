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
// package com::dynatrace::dynahist::util;

pub struct AlgorithmsTest {
}

impl AlgorithmsTest {

    #[test]
    pub fn  test_interpolate(&self)   {
        assert_equals(4.5, &Algorithms::interpolate(3.5, 3, 4, 4, 5), 0.0);
        assert_equals(4.5, &Algorithms::interpolate(3, 3, 4, 3, 5), 0.0);
        assert_equals(4.5, &Algorithms::interpolate(2, 3, 4, 3, 5), 0.0);
        assert_equals(4, &Algorithms::interpolate(2, 3, 4, 4, 5), 0.0);
        assert_equals(5, &Algorithms::interpolate(6, 3, 4, 4, 5), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(2, 3, Double::POSITIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(3.5, 3, Double::POSITIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(5, 3, Double::POSITIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(2, 3, Double::NEGATIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, 3, Double::NEGATIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(5, 3, Double::NEGATIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(2, 3, Double::NEGATIVE_INFINITY, 4, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(3.5, 3, Double::NEGATIVE_INFINITY, 4, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(5, 3, Double::NEGATIVE_INFINITY, 4, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(2, 3, Double::POSITIVE_INFINITY, 4, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, 3, Double::POSITIVE_INFINITY, 4, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(5, 3, Double::POSITIVE_INFINITY, 4, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(-1, 0, Double::POSITIVE_INFINITY, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(0, 0, Double::POSITIVE_INFINITY, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(1, 0, Double::POSITIVE_INFINITY, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(-1, 0, Double::NEGATIVE_INFINITY, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(0, 0, Double::NEGATIVE_INFINITY, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(1, 0, Double::NEGATIVE_INFINITY, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(-1, 0, Double::POSITIVE_INFINITY, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(0, 0, Double::POSITIVE_INFINITY, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(1, 0, Double::POSITIVE_INFINITY, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(-1, 0, Double::NEGATIVE_INFINITY, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(0, 0, Double::NEGATIVE_INFINITY, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(1, 0, Double::NEGATIVE_INFINITY, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(-1, 0, Double::POSITIVE_INFINITY, 0, 0), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(0, 0, Double::POSITIVE_INFINITY, 0, 0), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(1, 0, Double::POSITIVE_INFINITY, 0, 0), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(-1, 0, Double::NEGATIVE_INFINITY, 0, 0), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(0, 0, Double::NEGATIVE_INFINITY, 0, 0), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(1, 0, Double::NEGATIVE_INFINITY, 0, 0), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(-1, 0, 0, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(0, 0, 0, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(1, 0, 0, 0, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(-1, 0, 0, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(0, 0, 0, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(1, 0, 0, 0, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(-1, -1, Double::POSITIVE_INFINITY, 1, 0), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(0, -1, Double::POSITIVE_INFINITY, 1, 0), 0.0);
        assert_equals(0, &Algorithms::interpolate(1, -1, Double::POSITIVE_INFINITY, 1, 0), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(-1, -1, Double::NEGATIVE_INFINITY, 1, 0), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(0, -1, Double::NEGATIVE_INFINITY, 1, 0), 0.0);
        assert_equals(0, &Algorithms::interpolate(1, -1, Double::NEGATIVE_INFINITY, 1, 0), 0.0);
        assert_equals(0, &Algorithms::interpolate(-1, -1, 0, 1, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(0, -1, 0, 1, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(1, -1, 0, 1, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(0, &Algorithms::interpolate(-1, -1, 0, 1, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(0, -1, 0, 1, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(1, -1, 0, 1, Double::NEGATIVE_INFINITY), 0.0);
        assert_that(&Algorithms::interpolate(&Math::next_up(1e30), 1e30, 1e30, 1e300, 1e300)).is_close_to(&Math::next_up(1e30), &Percentage::with_percentage(1e-6));
        assert_that(&Algorithms::interpolate(&Math::next_up(1e30), 1e300, 1e300, 1e30, 1e30)).is_close_to(&Math::next_up(1e30), &Percentage::with_percentage(1e-6));
        assert_that(&Algorithms::interpolate(&Math::next_down(-1e30), -1e30, -1e30, -1e300, -1e300)).is_close_to(&Math::next_down(-1e30), &Percentage::with_percentage(1e-6));
        assert_that(&Algorithms::interpolate(&Math::next_down(-1e30), -1e300, -1e300, -1e30, -1e30)).is_close_to(&Math::next_down(-1e30), &Percentage::with_percentage(1e-6));
    }

    #[test]
    pub fn  test_interpolate_symmetry(&self)   {
         let random: Random = Random::new(0);
         let num_test_cycles: i32 = 1000;
         {
             let mut i: i32 = 0;
            while i < num_test_cycles {
                {
                     let x_vals: vec![Vec<f64>; 3] = vec![random.next_double(), random.next_double(), random.next_double(), ]
                    ;
                    Arrays::sort(&x_vals);
                     let x1: f64 = x_vals[0];
                     let x: f64 = x_vals[1];
                     let x2: f64 = x_vals[2];
                     let y1: f64 = random.next_double();
                     let y2: f64 = random.next_double();
                     let interpolated_value1: f64 = Algorithms::interpolate(x, x1, y1, x2, y2);
                     let interpolated_value2: f64 = Algorithms::interpolate(x, x2, y2, x1, y1);
                    assert_equals(0, &Double::compare(interpolated_value1, interpolated_value2));
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn  test_interpolate_monotonicity(&self)   {
         let random: Random = Random::new(0);
         let num_test_cycles: i32 = 1000;
         {
             let mut i: i32 = 0;
            while i < num_test_cycles {
                {
                     let x_vals: vec![Vec<f64>; 3] = vec![random.next_double(), random.next_double(), random.next_double(), ]
                    ;
                    Arrays::sort(&x_vals);
                     let x1: f64 = x_vals[0];
                     let x: f64 = x_vals[1];
                     let x2: f64 = x_vals[2];
                     let y1: f64 = random.next_double();
                     let y2: f64 = random.next_double();
                     let interpolated_value_left: f64 = Algorithms::interpolate(&Math::next_down(x), x1, y1, x2, y2);
                     let interpolated_value_mid: f64 = Algorithms::interpolate(x, x1, y1, x2, y2);
                     let interpolated_value_right: f64 = Algorithms::interpolate(&Math::next_up(x), x1, y1, x2, y2);
                    if y1 <= y2 {
                        assert_true(interpolated_value_left <= interpolated_value_mid);
                        assert_true(interpolated_value_mid <= interpolated_value_right);
                    } else {
                        assert_true(interpolated_value_left >= interpolated_value_mid);
                        assert_true(interpolated_value_mid >= interpolated_value_right);
                    }
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn  test_interpolate_na_n(&self)   {
        assert_equals(Double::NaN, &Algorithms::interpolate(Double::NaN, 3, 4, 4, 5), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, Double::NaN, 4, 4, 5), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, 3, 4, Double::NaN, 5), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, Double::NEGATIVE_INFINITY, 4, Double::POSITIVE_INFINITY, 5), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, 2, Double::NEGATIVE_INFINITY, 4, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(3.5, Double::NEGATIVE_INFINITY, 3, Double::POSITIVE_INFINITY, 4), 0.0);
        assert_equals(3, &Algorithms::interpolate(3, Double::NEGATIVE_INFINITY, 3, Double::POSITIVE_INFINITY, 3), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(0, Double::NEGATIVE_INFINITY, 0, Double::POSITIVE_INFINITY, 1), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(0, Double::NEGATIVE_INFINITY, 0, Double::POSITIVE_INFINITY, -1), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(0, Double::NEGATIVE_INFINITY, 1, Double::POSITIVE_INFINITY, 0), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(0, Double::NEGATIVE_INFINITY, -1, Double::POSITIVE_INFINITY, 0), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(0, 1, Double::NEGATIVE_INFINITY, 2, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(1, 1, Double::NEGATIVE_INFINITY, 2, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(1.5, 1, Double::NEGATIVE_INFINITY, 2, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(2, 1, Double::NEGATIVE_INFINITY, 2, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(3, 1, Double::NEGATIVE_INFINITY, 2, Double::POSITIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(0, 1, Double::POSITIVE_INFINITY, 2, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::interpolate(1, 1, Double::POSITIVE_INFINITY, 2, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NaN, &Algorithms::interpolate(1.5, 1, Double::POSITIVE_INFINITY, 2, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(2, 1, Double::POSITIVE_INFINITY, 2, Double::NEGATIVE_INFINITY), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::interpolate(3, 1, Double::POSITIVE_INFINITY, 2, Double::NEGATIVE_INFINITY), 0.0);
    }

    #[test]
    pub fn  test_calculate_midpoint(&self)   {
        assert_equals(-1, &Algorithms::calculate_midpoint(Long::MIN_VALUE, Long::MAX_VALUE));
        assert_equals(-1, &Algorithms::calculate_midpoint(Long::MIN_VALUE, Long::MAX_VALUE - 1));
        assert_equals(-2, &Algorithms::calculate_midpoint(Long::MIN_VALUE, Long::MAX_VALUE - 2));
        assert_equals(-2, &Algorithms::calculate_midpoint(Long::MIN_VALUE, Long::MAX_VALUE - 3));
        assert_equals(0, &Algorithms::calculate_midpoint(Long::MIN_VALUE + 1, Long::MAX_VALUE));
        assert_equals(0, &Algorithms::calculate_midpoint(Long::MIN_VALUE + 2, Long::MAX_VALUE));
        assert_equals(Long::MAX_VALUE, &Algorithms::calculate_midpoint(Long::MAX_VALUE, Long::MAX_VALUE));
        assert_equals(Long::MAX_VALUE - 1, &Algorithms::calculate_midpoint(Long::MAX_VALUE - 1, Long::MAX_VALUE));
        assert_equals(Long::MAX_VALUE - 1, &Algorithms::calculate_midpoint(Long::MAX_VALUE - 2, Long::MAX_VALUE));
        assert_equals(Long::MIN_VALUE, &Algorithms::calculate_midpoint(Long::MIN_VALUE, Long::MIN_VALUE));
        assert_equals(Long::MIN_VALUE, &Algorithms::calculate_midpoint(Long::MIN_VALUE + 1, Long::MIN_VALUE));
        assert_equals(Long::MIN_VALUE + 1, &Algorithms::calculate_midpoint(Long::MIN_VALUE + 2, Long::MIN_VALUE));
        assert_equals(Long::MIN_VALUE + 1, &Algorithms::calculate_midpoint(Long::MIN_VALUE + 3, Long::MIN_VALUE));
        assert_equals(Long::MIN_VALUE / 2, &Algorithms::calculate_midpoint(0, Long::MIN_VALUE));
        assert_equals(Long::MAX_VALUE / 2, &Algorithms::calculate_midpoint(0, Long::MAX_VALUE));
        assert_equals(-5, &Algorithms::calculate_midpoint(-4, -6));
        assert_equals(-6, &Algorithms::calculate_midpoint(-4, -7));
        assert_equals(-6, &Algorithms::calculate_midpoint(-5, -7));
        assert_equals(-6, &Algorithms::calculate_midpoint(-4, -8));
        assert_equals(5, &Algorithms::calculate_midpoint(4, 6));
        assert_equals(5, &Algorithms::calculate_midpoint(4, 7));
        assert_equals(6, &Algorithms::calculate_midpoint(5, 7));
        assert_equals(6, &Algorithms::calculate_midpoint(4, 8));
        assert_equals(0, &Algorithms::calculate_midpoint(-2, 3));
        assert_equals(1, &Algorithms::calculate_midpoint(-1, 4));
        assert_equals(-1, &Algorithms::calculate_midpoint(-3, 2));
    }

    #[test]
    pub fn  test_map_double_to_long(&self)   {
        assert_equals(0, &Algorithms::map_double_to_long(0));
        assert_equals(1, &Algorithms::map_double_to_long(Double::MIN_VALUE));
        assert_equals(0x7fefffffffffffff, &Algorithms::map_double_to_long(Double::MAX_VALUE));
        assert_equals(0x7ff0000000000000, &Algorithms::map_double_to_long(Double::POSITIVE_INFINITY));
        assert_equals(-1, &Algorithms::map_double_to_long(-0.0));
        assert_equals(-2, &Algorithms::map_double_to_long(-Double::MIN_VALUE));
        assert_equals(0x8010000000000000, &Algorithms::map_double_to_long(-Double::MAX_VALUE));
        assert_equals(0x800fffffffffffff, &Algorithms::map_double_to_long(Double::NEGATIVE_INFINITY));
        assert_equals(0x7ff8000000000000, &Algorithms::map_double_to_long(Double::NaN));
    }

    #[test]
    pub fn  test_long_to_double(&self)   {
        assert_equals(0, &Algorithms::map_long_to_double(0), 0.0);
        assert_equals(Double::MIN_VALUE, &Algorithms::map_long_to_double(1), 0.0);
        assert_equals(Double::MAX_VALUE, &Algorithms::map_long_to_double(0x7fefffffffffffff), 0.0);
        assert_equals(Double::POSITIVE_INFINITY, &Algorithms::map_long_to_double(0x7ff0000000000000), 0.0);
        assert_equals(-0.0, &Algorithms::map_long_to_double(-1), 0.0);
        assert_equals(-Double::MIN_VALUE, &Algorithms::map_long_to_double(-2), 0.0);
        assert_equals(-Double::MAX_VALUE, &Algorithms::map_long_to_double(-1 - 0x7fefffffffffffff), 0.0);
        assert_equals(Double::NEGATIVE_INFINITY, &Algorithms::map_long_to_double(-1 - 0x7ff0000000000000), 0.0);
        assert_equals(Double::NaN, &Algorithms::map_long_to_double(0x7ff8000000000000), 0.0);
        assert_equals(Double::NaN, &Algorithms::map_long_to_double(Long::MAX_VALUE), 0.0);
        assert_equals(Double::NaN, &Algorithms::map_long_to_double(0x7ff0000000000000 + 1), 0.0);
        assert_equals(Double::NaN, &Algorithms::map_long_to_double(0x800fffffffffffff - 1), 0.0);
        assert_equals(Double::NaN, &Algorithms::map_long_to_double(-2 - 0x7ff0000000000000), 0.0);
        assert_equals(Double::NaN, &Algorithms::map_long_to_double(Long::MIN_VALUE), 0.0);
    }

    #[test]
    pub fn  test_double_mapping(&self)   {
         let cycles: i32 = 100000;
         let rnd: Random = Random::new(0);
         {
             let mut i: i32 = 0;
            while i < cycles {
                {
                     let d: f64 = rnd.next_double() * 2 - 1;
                    assert_equals(d, &Algorithms::map_long_to_double(&Algorithms::map_double_to_long(d)), 0.0);
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn  test_find_first(&self)   {
         let max_num_evaluations: i32 = 65;
        self.test_find_first(3, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MAX_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MAX_VALUE - 1, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MIN_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MIN_VALUE + 1, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(0, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(0, -1, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(0, Long::MIN_VALUE, 1, max_num_evaluations);
        self.test_find_first(0, -1, 1, max_num_evaluations);
        self.test_find_first(0, -1, 0, max_num_evaluations);
        self.test_find_first(0, 0, 1, max_num_evaluations);
        self.test_find_first(0, 0, 0, max_num_evaluations);
        self.test_find_first(1, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(10, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(100, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MAX_VALUE - 2, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MAX_VALUE - 1, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(Long::MAX_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
        self.test_find_first(0, 0, Long::MAX_VALUE, max_num_evaluations);
        assert_throws(IllegalArgumentException.class, () -> Algorithms::find_first( l: & -> false, 0, Long::MAX_VALUE));
    // assertThrows(IllegalArgumentException.class, () -> Algorithms.findFirst(l -> false, 1, -1));
    // assertThrows(IllegalArgumentException.class, () -> Algorithms.findFirst(l -> false, -1, 1, 2));
    // assertThrows(IllegalArgumentException.class, () -> Algorithms.findFirst(l -> false, -1, 1, -2));
    }

    fn  test_find_first_with_initial_guess( first_true_index: i64,  min: i64,  max: i64,  initial_guess: i64,  max_num_evaluations: i32)   {
         let evaluated_values: Set<Long> = HashSet<>::new();
         let predicate: LongPredicate =  value: & -> {
            assert_true(&evaluated_values.add(value));
            return value >= first_true_index;
        };
        assert_equals(first_true_index, &Algorithms::find_first(&predicate, min, max, initial_guess));
        assert_that(&evaluated_values.size()).is_less_than_or_equal_to(max_num_evaluations);
    }

    fn  test_find_first( first_true_index: i64,  min: i64,  max: i64,  max_num_evaluations: i32)   {
         let evaluated_values: Set<Long> = HashSet<>::new();
         let predicate: LongPredicate =  value: & -> {
            assert_true(&evaluated_values.add(value));
            return value >= first_true_index;
        };
        assert_equals(first_true_index, &Algorithms::find_first(&predicate, min, max));
        assert_that(&evaluated_values.size()).is_less_than_or_equal_to(max_num_evaluations);
    }

    #[test]
    pub fn  test_find_first_with_initial_guess(&self)   {
         let max_num_evaluations: i32 = 128;
        ::test_find_first_with_initial_guess(Long::MAX_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MIN_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MAX_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE + 1, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MIN_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE - 1, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MAX_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MIN_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MAX_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE - 1, max_num_evaluations);
        ::test_find_first_with_initial_guess(Long::MIN_VALUE, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE + 1, max_num_evaluations);
        assert_throws(IllegalArgumentException.class, () -> Algorithms::find_first( l: & -> false, Long::MIN_VALUE, Long::MAX_VALUE, 0));
    }

    #[test]
    pub fn  test_find_first_with_initial_guess2(&self)   {
         let max_num_evaluations_with_initial_guess: i32 = 128;
         let max_num_evaluations: i32 = 65;
         {
             let mut j: i32 = 0;
            while j < 100 {
                {
                     let first_true_index: i64 = Long::MAX_VALUE - j;
                     {
                         let mut i: i32 = 0;
                        while i < 100 {
                            {
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, -1 - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
                }
                j += 1;
             }
         }

         {
             let mut j: i32 = 0;
            while j < 100 {
                {
                     let first_true_index: i64 = Long::MIN_VALUE + j;
                     {
                         let mut i: i32 = 0;
                        while i < 100 {
                            {
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, -1 - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
                }
                j += 1;
             }
         }

         {
             let mut j: i32 = 0;
            while j < 100 {
                {
                     let first_true_index: i64 = j;
                     {
                         let mut i: i32 = 0;
                        while i < 100 {
                            {
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, -1 - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
                }
                j += 1;
             }
         }

         {
             let mut j: i32 = 0;
            while j < 100 {
                {
                     let first_true_index: i64 = -j - 1;
                     {
                         let mut i: i32 = 0;
                        while i < 100 {
                            {
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MAX_VALUE - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, -1 - i, max_num_evaluations_with_initial_guess);
                                ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, Long::MIN_VALUE + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, Long::MIN_VALUE, Long::MAX_VALUE, max_num_evaluations);
                }
                j += 1;
             }
         }

         {
             let mut j: i32 = 0;
            while j < 20 {
                {
                     {
                         let mut i: i32 = 0;
                        while i <= j {
                            {
                                 let first_true_index: i64 = Long::MAX_VALUE - i;
                                 {
                                     let mut k: i32 = 0;
                                    while k <= j {
                                        {
                                            ::test_find_first_with_initial_guess(first_true_index, Long::MAX_VALUE - j, Long::MAX_VALUE, Long::MAX_VALUE - k, max_num_evaluations_with_initial_guess);
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            i += 1;
                         }
                     }

                }
                j += 1;
             }
         }

         {
             let mut j: i32 = 0;
            while j < 20 {
                {
                     {
                         let mut i: i32 = 0;
                        while i <= j {
                            {
                                 let first_true_index: i64 = Long::MIN_VALUE + i;
                                 {
                                     let mut k: i32 = 0;
                                    while k <= j {
                                        {
                                            ::test_find_first_with_initial_guess(first_true_index, Long::MIN_VALUE, Long::MIN_VALUE + j, Long::MIN_VALUE + k, max_num_evaluations_with_initial_guess);
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            i += 1;
                         }
                     }

                }
                j += 1;
             }
         }

    }

    #[test]
    pub fn  test_find_first_with_initial_guess3(&self)   {
        ::test_find_first_with_initial_guess(1, Long::MIN_VALUE, Long::MAX_VALUE, 0, 2);
        ::test_find_first_with_initial_guess(134325, Long::MIN_VALUE, Long::MAX_VALUE, 134324, 2);
        ::test_find_first_with_initial_guess(0, Long::MIN_VALUE, Long::MAX_VALUE, 1, 4);
        ::test_find_first_with_initial_guess(134324, Long::MIN_VALUE, Long::MAX_VALUE, 134325, 4);
        ::test_find_first_with_initial_guess(2, Long::MIN_VALUE, Long::MAX_VALUE, 0, 4);
        ::test_find_first_with_initial_guess(3, Long::MIN_VALUE, Long::MAX_VALUE, 0, 4);
    }

    #[test]
    pub fn  test_interpolate_random(&self)   {
         let random: SplittableRandom = SplittableRandom::new(0);
         let num_iterations: i32 = 1000;
         let a: i64 = 10;
         let maxb: i64 = 100;
         let c: i64 = 10;
         {
             let mut i: i32 = 0;
            while i < num_iterations {
                {
                     let b: i64 = random.next_long(maxb);
                     let l: i64 = random.next_long(Algorithms::NEGATIVE_INFINITY_MAPPED_TO_LONG + 1 + a, Algorithms::POSITIVE_INFINITY_MAPPED_TO_LONG - b - c);
                     let mut x1: f64 = Algorithms::map_long_to_double(l);
                     let mut x2: f64 = Algorithms::map_long_to_double(l + b);
                    if random.next_boolean() {
                         let t: f64 = x1;
                        x1 = x2;
                        x2 = t;
                    }
                     let y1: f64 = random.next_double(-1, 1);
                     let y2: f64 = random.next_double(-1, 1);
                     let previous_y: f64 = Double::NaN;
                     {
                         let mut j: i64 = 0;
                        while j < a + b + c {
                            {
                                 let x: f64 = Algorithms::map_long_to_double(l + j - a);
                                 let y: f64 = Algorithms::interpolate(x, x1, y1, x2, y2);
                                assert_that(y).is_between(&Math::min(y1, y2), &Math::max(y1, y2));
                                if !Double::is_na_n(previous_y) {
                                    if (y1 <= y2 && x1 <= x2) || (y1 >= y2 && x1 >= x2) {
                                        assert_that(y).is_greater_than_or_equal_to(previous_y);
                                    }
                                    if (y1 <= y2 && x1 >= x2) || (y1 >= y2 && x1 <= x2) {
                                        assert_that(y).is_less_than_or_equal_to(previous_y);
                                    }
                                }
                                previous_y = y;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn  test_interpolate_equal_x(&self)   {
         let random: SplittableRandom = SplittableRandom::new(0);
         let num_iterations: i32 = 1000;
         {
             let mut i: i32 = 0;
            while i < num_iterations {
                {
                     let x: f64 = random.next_double(-10, 10);
                     let y1: f64 = random.next_double(-10, 10);
                     let y2: f64 = random.next_double(-10, 10);
                     let expected_y: f64 = 0.5 * y1 + 0.5 * y2;
                    assert_equals(expected_y, &Algorithms::interpolate(&Math::next_down(x), x, y1, x, y2), 0.0);
                    assert_equals(expected_y, &Algorithms::interpolate(x, x, y1, x, y2), 0.0);
                    assert_equals(expected_y, &Algorithms::interpolate(&Math::next_up(x), x, y1, x, y2), 0.0);
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn  test_interpolate_negative_zero(&self)   {
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(-0.0 * 5.0));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-0.0, -1.0, -0.0, -0.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-0.0, -0.0, -0.0, -0.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(Double::POSITIVE_INFINITY, -0.0, -0.0, -0.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(Double::NEGATIVE_INFINITY, -0.0, -0.0, -0.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(Double::NEGATIVE_INFINITY, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-100.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-10.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-1.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(0.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(1.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(10.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(100.0, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(Double::POSITIVE_INFINITY, -10.0, -0.0, 10.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(Double::NEGATIVE_INFINITY, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-100.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-10.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(-1.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(0.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(1.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(10.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(100.0, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(Double::POSITIVE_INFINITY, -10.0, 0.0, 10.0, -0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(5.0, 5.0, -0.0, 5.0, 0.0)));
        assert_equals(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(&Algorithms::interpolate(5.0, 5.0, 0.0, 5.0, -0.0)));
    }

    #[test]
    pub fn  test_clip(&self)   {
        assert_equals(-3, &Algorithms::clip(1, -3, -3));
        assert_equals(3, &Algorithms::clip(1, 3, 7));
        assert_equals(5, &Algorithms::clip(5, 3, 7));
        assert_equals(7, &Algorithms::clip(8, 3, 7));
        assert_throws(IllegalArgumentException.class, () -> Algorithms::clip(10, 5, 4));
    }
}

