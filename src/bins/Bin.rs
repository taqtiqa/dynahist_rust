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
// package com::dynatrace::dynahist::bin;

/** A bin of the histogram. */
pub trait Bin {

    /**
   * Returns the number of values belonging to the current bin position.
   *
   * @return the number of values belonging to the current bin position
   */
    fn  get_bin_count(&self) -> i64 ;

    /**
   * Returns a lower bound for all values in this bin.
   *
   * <p>The returned value is always greater than or equal to the minimum value of the underlying
   * histogram.
   *
   * @return a lower bound for all values in this bin
   */
    fn  get_lower_bound(&self) -> f64 ;

    /**
   * Returns an upper bound for all values in this bin.
   *
   * <p>The returned value is always less than or equal to the maximum value of the underlying
   * histogram.
   *
   * @return an upper bound for all values in this bin
   */
    fn  get_upper_bound(&self) -> f64 ;

    /**
   * Returns the width of the current bin.
   *
   * @return the width of the bin
   */
    fn default  get_width(&self) -> f64  {
        return self.get_upper_bound() - self.get_lower_bound();
    }

    /**
   * Returns the number of values less than the lower bound of the the current bin.
   *
   * @return the number of values less than the lower bound of the the current bin
   */
    fn  get_less_count(&self) -> i64 ;

    /**
   * Returns the number of values greater than the upper bound of the the current bin.
   *
   * @return the number of values greater than the upper bound of the the current bin
   */
    fn  get_greater_count(&self) -> i64 ;

    /**
   * Returns the bin index as defined by the {@link Layout}.
   *
   * @return the bin index as defined by the {@link Layout}
   */
    fn  get_bin_index(&self) -> i32 ;

    /**
   * Returns {@code true} if this bin corresponds to the first non-empty bin.
   *
   * @return {@code true} if this bin corresponds to the first non-empty bin
   */
    fn default  is_first_non_empty_bin(&self) -> bool  {
        return self.get_less_count() == 0;
    }

    /**
   * Returns {@code true} if this bin corresponds to the last non-empty bin.
   *
   * @return {@code true} if this bin corresponds to the last non-empty bin
   */
    fn default  is_last_non_empty_bin(&self) -> bool  {
        return self.get_greater_count() == 0;
    }

    /**
   * Returns {@code true} if this bin corresponds to the underflow bin.
   *
   * @return {@code true} if this bin corresponds to the underflow bin
   */
    fn  is_underflow_bin(&self) -> bool ;

    /**
   * Returns {@code true} if this bin corresponds to the overflow bin.
   *
   * @return {@code true} if this bin corresponds to the overflow bin
   */
    fn  is_overflow_bin(&self) -> bool ;
}

