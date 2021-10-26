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
// package com::dynatrace::dynahist::demo;

pub struct BinIteration {
}

impl BinIteration {

    #[test]
    pub fn  test_bin_iteration_ascending_order(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-3, 1e-2, 0, 1E6);
         let histogram: Histogram = Histogram::create_dynamic(layout)::add_value(7.5, 3)::add_value(3.5, 6)::add_value(8.5, 11)::add_value(9.5, 2);
         let expected_bin_counts : vec![i64; 4] = vec![6, 3, 11, 2, ]
        ;
         let bin_iterator: BinIterator = histogram.get_first_non_empty_bin();
         let mut counter: i32 = 0;
        assert_equals(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        while !bin_iterator.is_last_non_empty_bin() {
            counter += 1;
            bin_iterator.next();
            bin_iterator.get_bin_count();
            assert_equals(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        }
    }

    #[test]
    pub fn  test_bin_iteration_descending_order(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-3, 1e-2, 0, 1E6);
         let histogram: Histogram = Histogram::create_dynamic(layout)::add_value(7.5, 3)::add_value(3.5, 6)::add_value(8.5, 11)::add_value(9.5, 2);
         let expected_bin_counts : vec![i64; 4] = vec![2, 11, 3, 6, ]
        ;
         let bin_iterator: BinIterator = histogram.get_last_non_empty_bin();
         let mut counter: i32 = 0;
        assert_equals(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        while !bin_iterator.is_first_non_empty_bin() {
            counter += 1;
            bin_iterator.previous();
            bin_iterator.get_bin_count();
            assert_equals(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        }
    }
}

