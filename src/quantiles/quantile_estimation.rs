// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;
// use crate::values::value_estimators::ValueEstimatorUniform;

// Sealing stop crates other than DynaHist from implementing any functions
// provided by this trait.
// mod private {
//     pub trait Sealed {}
//     impl Sealed for dyn super::QuantileEstimation { }
// }

/// A quantile estimator.
// pub trait QuantileEstimation<QEstimator = Self>:
pub trait QuantileEstimation:
    Preconditions + Algorithms + ValueEstimation
{
    // type QEstimator: QuantileEstimation;
    // type VEstimator: ValueEstimation;

    // This default is used in SciPy 1.5.2
    //const DEFAULT: Self = Self::create(0.4, 0.4);

    // Trait private methods, not allowed for user to call.
    #[doc(hidden)]

    /// Return a implementation of the [`QuantileEstimation`] trait quantile
    /// definition with given plotting positions parameters.
    ///
    /// - `alphap`: plotting positions parameter
    /// - `betap`: plotting positions parameter
    ///
    /// a [`QuantileEstimation`] instance
    ///
    fn create(alphap: f64, betap: f64) -> Self;

    fn new(alphap: f64, betap: f64) -> Self {
        Self::create(alphap, betap)
    }

    fn to_string(&self) -> String;

    /// Return an estimate for the quantile value using the estimated values as given by
    /// [`get_value(i64)`] and the default quantile estimation method. The default behavior might change
    /// in future. Therefore, if well-defined behavior is wanted, use {@link #getQuantile(double,
    /// QuantileEstimation)}.
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - `p`: the p-value in range [0,1]
    ///
    /// an estimate for the p-quantile
    ///
    fn get_quantile(&self, p: f64) -> f64;

    /// TODO: Deprecate or merge this documentation
    /// Return an estimate for the p-quantile value using the estimated values as given by
    /// [`get_value(i64)`] and the given [`QuantileEstimation`].
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - `p`: the p-value in range [0,1]
    /// - `quantile_estimator`: the quantile estimator
    ///
    /// an estimate for the p-quantile
    ///
    //fn get_quantile(&self, p: f64, quantile_estimator: &Self::Q) -> f64;

    /// TODO: Deprecate or merge this documentation
    /// Return an estimate for the quantile value using the estimated values as given by
    /// and the given [`QuantileEstimation`].
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - `p`: the p-value in range [0,1]
    /// - `value_estimator`: the value estimator
    ///
    /// an estimate for the p-quantile
    ///
    //fn get_quantile(&self, p: f64, value_estimator: &Self::V) -> f64;

    /// Return an estimate for the quantile value using the estimated values as given by
    /// [`get_value(i64)`] and the given [`QuantileEstimation`] implementation.
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - `p`: the p-value in range [0,1]
    /// - `quantile_estimator`: the quantile estimator
    /// - `value_estimator`: the value estimator
    ///
    /// an estimate for the p-quantile
    ///
    fn get_quantile_from_estimator(&self, quantile_estimator: impl QuantileEstimation) -> f64;

    /// Return the p-quantile estimate from sorted data which can be randomly
    ///  accessed through the given function.
    ///
    /// If `num_values` is equal to `0`, the return value will always be
    /// [`Double::isNaN()`].
    ///
    /// The behavior is undefined, if the given function is not monotonic increasing.
    ///
    /// - `p`: specifies the quantile, must be in [0,1], e.g. 0.5
    /// specifies the median
    /// - `rank_fn` a function that gives access the `i`-th
    /// largest (`0`-based) value if the argument is equal to `i`, the function
    /// must accept any nonnegative arguments less than `num_values`
    /// - `num_values`: the number of values
    ///
    fn estimate_quantile_from_fn(&self, p: f64, rank_fn: &fn(u64) -> f64, num_values: i64) -> f64;

    /// Return the p-quantile estimate from a sorted double array.
    ///
    /// If `num_values` is equal to 0 the return value will always be [`Double::isNaN()`].
    ///
    /// The behavior is undefined, if the array is not sorted in ascending order.
    ///
    /// - `p`: specifies the quantile, must be in `[0,1]`, e.g. `0.5` specifies the median
    /// - `sorted_values`: a sorted double array
    ///
    fn estimate_quantile_from_values(&self, p: f64, sorted_values: &Vec<f64>) -> f64 {
        let rank_fn = |&rank| sorted_values[rank as usize];
        return self.estimate_quantile_from_fn(p, rank_fn, sorted_values.len());
    }
}
