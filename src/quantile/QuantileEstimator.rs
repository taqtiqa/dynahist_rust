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
// package com::dynatrace::dynahist::quantile;

/** A quantile estimator. */
pub trait QuantileEstimator {

    /**
   * Estimates the quantile from sorted data which can be randomly accessed through the given
   * function.
   *
   * <p>If numValues is equal to 0 the return value will always be {@link Double#isNaN()}.
   *
   * <p>The behavior is undefined, if the given function is not monotonic increasing.
   *
   * @param p specifies the quantile, must be in [0,1], e.g. 0.5 specifies the median
   * @param sortedValueFunction a function that gives access the i-th largest (0-based) value if the
   *     argument is equal to i, the function must accept any nonnegative arguments less than
   *     numValues
   * @param numValues the number of values
   * @return the quantile estimate
   */
    fn  estimate_quantile(&self,  p: f64,  sorted_value_function: &LongToDoubleFunction,  num_values: i64) -> f64 ;

    /**
   * Estimates the quantile from a sorted double array.
   *
   * <p>If numValues is equal to 0 the return value will always be {@link Double#isNaN()}.
   *
   * <p>The behavior is undefined, if the array is not sorted in ascending order.
   *
   * @param p specifies the quantile, must be in [0,1], e.g. 0.5 specifies the median
   * @param sortedValues a sorted double array
   * @return the quantile estimate
   */
    fn default  estimate_quantile(&self,  p: f64,  sorted_values: &Vec<f64>) -> f64  {
        return self.estimate_quantile(p,  i: & -> sorted_values[i as i32], sorted_values.len());
    }
}

