// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/** A quantile estimator. */
pub trait QuantileEstimator {


   /// Estimates the quantile from sorted data which can be randomly accessed through the given
   /// function.
   ///
   /// If numValues is equal to 0 the return value will always be {@link Double#isNaN()}.
   ///
   /// The behavior is undefined, if the given function is not monotonic increasing.
   ///
   /// @param p specifies the quantile, must be in [0,1], e.g. 0.5 specifies the median
   /// @param sortedValueFunction a function that gives access the i-th largest (0-based) value if the
   ///     argument is equal to i, the function must accept any nonnegative arguments less than
   ///     numValues
   /// @param numValues the number of values
   /// @return the quantile estimate
   ///
    fn estimate_quantile(&self,  p: f64,  sorted_value_function: &LongToDoubleFunction,  num_values: i64) -> f64 ;


   /// Estimates the quantile from a sorted double array.
   ///
   /// If numValues is equal to 0 the return value will always be {@link Double#isNaN()}.
   ///
   /// The behavior is undefined, if the array is not sorted in ascending order.
   ///
   /// @param p specifies the quantile, must be in [0,1], e.g. 0.5 specifies the median
   /// @param sortedValues a sorted double array
   /// @return the quantile estimate
   ///
    fn estimate_quantile(&self,  p: f64,  sorted_values: &Vec<f64>) -> f64  {
        return self.estimate_quantile(p,  i: & -> sorted_values[i as i32], sorted_values.len());
    }
}
