// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/** Estimates recorded values from a histogram. */
pub trait ValueEstimator {
    /// Distributes the values of a bin uniformly over the bin's interval. The distance between two
    /// values is kept constant. Let X be the distance between two points. The distance of the first
    /// value to the bin lower bound will be X/2 unless the bin boundary represents the minimum
    /// recorded value. The distance of the last value to the bin upper bound will be X/2 unless the
    /// bin boundary represents the maximum recorded value.
    ///
    /// If this estimator is used for bins of a {@link LogLinearLayout} or {@link
    /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by the
    /// absolute and relative bin width limits, respectively.
    ///
    const UNIFORM: ValueEstimator = ValueEstimatorImpls::UNIFORM;

    /// Places all values of the bin at its lower bound except for the maximum recorded value.
    ///
    /// It is guaranteed that the estimated value is smaller than or equal to the original value.
    ///
    /// If this estimator is used for bins of a {@link LogLinearLayout} or {@link
    /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by the
    /// absolute and relative bin width limits, respectively.
    ///
    const LOWER_BOUND: ValueEstimator = ValueEstimatorImpls::LOWER_BOUND;

    /// Places all values of the bin at its upper bound except for the minimum recorded value.
    ///
    /// It is guaranteed that the estimated value is greater than or equal to the original value.
    ///
    /// If this estimator is used for bins of a {@link LogLinearLayout} or {@link
    /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by the
    /// absolute and relative bin width limits, respectively.
    ///
    const UPPER_BOUND: ValueEstimator = ValueEstimatorImpls::UPPER_BOUND;

    /// Places all values at the mid point of the bin except for the minimum and maximum recorded
    /// values.
    ///
    /// If this estimator is used for bins of a {@link LogLinearLayout} or {@link
    /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by
    /// half of the absolute and relative bin width limits, respectively.
    ///
    const MID_POINT: ValueEstimator = ValueEstimatorImpls::MID_POINT;

    /// Estimates a recorded value with given zero-based rank from the given histogram.
    ///
    /// The estimated value must always be in the value range of the bin it belongs to.
    ///
    /// If rank == 0, {@link Histogram#getMin()} is returned. If rank == {@link
    /// Histogram#getTotalCount()} - 1, {@link Histogram#getMax()} is returned.
    ///
    /// @param histogram the histogram
    /// @param rank the zero-based rank
    /// @return the estimated value
    /// @throws IllegalArgumentException if 0 &le; rank &lt; {@link Histogram#getTotalCount()} does not
    ///     hold
    ///
    fn get_value_estimate(&self, histogram: &Histogram, rank: i64) -> f64;
}
