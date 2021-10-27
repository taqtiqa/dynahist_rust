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

/**
 * An abstract {@link Layout} class that can be used if there is an approximate formula for the
 * reverse mapping (from bin index to bin boundaries).
 */
#[derive(Layout)]
struct AbstractLayout {
}

impl AbstractLayout {

    pub fn  get_bin_lower_bound(&self,  bin_index: i32) -> f64  {
        if bin_index <= get_underflow_bin_index() {
            return f64::NEG_INFINITY;
        }
         let effective_bin_index: i32 = std::cmp::min(&get_overflow_bin_index(), bin_index);
         let approximate_bin_lower_bound: f64 = self.get_bin_lower_bound_approximation(effective_bin_index);
        return map_long_to_double(&find_first_guess( l: & -> map_to_bin_index(&map_long_to_double(l)) >= effective_bin_index, NEGATIVE_INFINITY_MAPPED_TO_LONG, POSITIVE_INFINITY_MAPPED_TO_LONG, &map_double_to_long(approximate_bin_lower_bound)));
    }

    pub fn  get_bin_upper_bound(&self,  bin_index: i32) -> f64  {
        if bin_index >= get_overflow_bin_index() {
            return f64::INFINITY;
        }
         let effective_bin_index: i32 = std::cmp::max(&get_underflow_bin_index(), bin_index);
         let approximate_bin_upper_bound: f64 = self.get_bin_lower_bound_approximation(effective_bin_index + 1);
        return map_long_to_double(~find_first_guess( l: & -> map_to_bin_index(&map_long_to_double(~l)) <= effective_bin_index, ~POSITIVE_INFINITY_MAPPED_TO_LONG, ~NEGATIVE_INFINITY_MAPPED_TO_LONG, ~map_double_to_long(approximate_bin_upper_bound)));
    }

    /**
   * Gives an approximation of the lower bound of bin with given bin index.
   *
   * <p>The method must be defined for all values greater than {@link #get_underflow_bin_index()} and
   * smaller than or equal to {@link #get_overflow_bin_index()}.
   *
   * <p>The return value must not be {@link Double#NaN}.
   *
   * @param bin_index the bin index
   * @return an approximation of the lower bound
   */
    pub fn  get_bin_lower_bound_approximation(&self,  bin_index: i32) -> f64 ;
}
