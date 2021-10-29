// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Value estimators.
/// Estimator types for the recorded values from a histogram.
///
/// The default is set in the abstract histogram.
///
/// const DEFAULT_VALUE_ESTIMATOR: impl ValueEstimation = ValueEstimatorUniform.new();
///
pub(crate) struct ValueEstimatorUniform {}
pub(crate) struct ValueEstimatorLowerBound {}
pub(crate) struct ValueEstimatorUpperBound {}
pub(crate) struct ValueEstimatorMidPoint {}
fn new() -> Self {
    Self {}
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorUniform {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorLowerBound {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorUpperBound {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorMidPoint {
    fn new() -> Self {
        Self {}
    }
}

impl ValueEstimatorImpls {

   /// Distributes the values of a bin uniformly over the bin's interval. The distance between two
   /// values is kept constant. Let X be the distance between two points. The distance of the first
   /// value to the bin lower bound will be X/2 unless the bin boundary represents the minimum
   /// recorded value. The distance of the last value to the bin upper bound will be X/2 unless the
   /// bin boundary represents the maximum recorded value.
   ///
   /// If this estimator is used for bins of a [`LogLinearLayout`] or {@link
   /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by the
   /// absolute and relative bin width limits, respectively.
   ///
    // UNIFORM() {

    //     pub fn get_estimate_from_bin(&self,  bin: &Bin,  rank: i64) -> f64  {
    //         let relative_rank: i64 = rank - bin.get_less_count();
    //         return interpolate((relative_rank - (bin.get_bin_count() - relative_rank - 1)), -bin.get_bin_count() + ( if (bin.is_first_non_empty_bin()) { 1 } else { 0 }), &bin.get_lower_bound(), bin.get_bin_count() - ( if (bin.is_last_non_empty_bin()) { 1 } else { 0 }), &bin.get_upper_bound());
    //     }
    // }

   /// Places all values of the bin at its lower bound except for the maximum recorded value.
   ///
   /// It is guaranteed that the estimated value is smaller than or equal to the original value.
   ///
   /// If this estimator is used for bins of a [`LogLinearLayout`] or {@link
   /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by the
   /// absolute and relative bin width limits, respectively.
   ///
    // LOWER_BOUND() {

    //     pub fn get_estimate_from_bin(&self,  bin: &Bin,  rank: i64) -> f64  {
    //         return bin.get_lower_bound();
    //     }
    // }
    // ,
   /// Places all values of the bin at its upper bound except for the minimum recorded value.
   ///
   /// It is guaranteed that the estimated value is greater than or equal to the original value.
   ///
   /// If this estimator is used for bins of a [`LogLinearLayout`] or {@link
   /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by the
   /// absolute and relative bin width limits, respectively.
   ///
    // UPPER_BOUND() {

    //     pub fn get_estimate_from_bin(&self,  bin: &Bin,  rank: i64) -> f64  {
    //         return bin.get_upper_bound();
    //     }
    // }
    // ,
   /// Places all values at the mid point of the bin except for the minimum and maximum recorded
   /// values.
   ///
   /// If this estimator is used for bins of a [`LogLinearLayout`] or {@link
   /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by
   /// half of the absolute and relative bin width limits, respectively.
   ///
    // MID_POINT() {

    //     pub fn get_estimate_from_bin(&self,  bin: &Bin,  rank: i64) -> f64  {
    //         return std::cmp::max(&bin.get_lower_bound(), &std::cmp::min(&bin.get_upper_bound(), (bin.get_lower_bound() + bin.get_upper_bound()) * 0.5));
    //     }
    // }
    // ;

    // pub fn get_value_estimate(&self,  histogram: &Histogram,  rank: i64) -> f64  {
    //     require_non_null(histogram);
    //      let total_count: i64 = histogram.get_total_count();
    //     check_argument(rank >= 0);
    //     check_argument(rank < total_count);
    //     if rank <= 0 {
    //         return histogram.get_min();
    //     }
    //     if rank + 1 == total_count {
    //         return histogram.get_max();
    //     }
    //      let bin: Bin = histogram.get_bin_by_rank(rank);
    //     return self.get_estimate_from_bin(bin, rank);
    // }
}
