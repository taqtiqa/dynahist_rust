// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin::BinSketch;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimators::*;
use crate::Histogram;

// Sealing a trait stops crates other than DynaHist from implementing any traits
// that use it.
mod private {
    pub trait Sealed {}
    impl Sealed for super::ValueEstimation {}
}

/// This trait is sealed and cannot be implemented for callers to avoid
/// breaking backwards compatibility when adding new derived traits.
///
pub trait ValueEstimation: Preconditions + Algorithms + private::Sealed {
    fn new() -> Self;

    // Trait private methods, not allowed for user to call.
    #[doc(hidden)]
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
    //const UNIFORM: ValueEstimator = ValueEstimatorImpls::UNIFORM;
    // const DEFAULT_VALUE_ESTIMATOR: ValueEstimator = ValueEstimator::UNIFORM;
    fn get_uniform_estimate_from_bin(&self, bin: &Bin, rank: i64) -> f64 {
        let relative_rank: i64 = rank - bin.get_less_count();
        return Self::interpolate(
            (relative_rank - (bin.get_bin_count() - relative_rank - 1)),
            -bin.get_bin_count() + (if (bin.is_first_non_empty_bin()) { 1 } else { 0 }),
            &bin.get_lower_bound(),
            bin.get_bin_count() - (if (bin.is_last_non_empty_bin()) { 1 } else { 0 }),
            &bin.get_upper_bound(),
        );
    }

    /// Places all values of the bin at its lower bound except for the maximum recorded value.
    ///
    /// It is guaranteed that the estimated value is smaller than or equal to the original value.
    ///
    /// If this estimator is used for bins of a [`LogLinearLayout`] or
    /// [`LogQuadraticLayout`], the maximum absolute and relative estimation errors will be bounded by the
    /// absolute and relative bin width limits, respectively.
    ///
    //const LOWER_BOUND: ValueEstimator = ValueEstimatorImpls::LOWER_BOUND;
    fn get_lower_bound_estimate_from_bin(&self, bin: &Bin, rank: i64) -> f64 {
        return bin.get_lower_bound();
    }

    /// Places all values of the bin at its upper bound except for the minimum
    /// recorded value.
    ///
    /// It is guaranteed that the estimated value is greater than or equal to
    /// the original value.
    ///
    /// If this estimator is used for bins of a [`LogLinearLayout`] or {@link
    /// LogQuadraticLayout}, the maximum absolute and relative estimation
    /// errors will be bounded by the absolute and relative bin width limits,
    /// respectively.
    ///
    //const UPPER_BOUND: ValueEstimator = ValueEstimatorImpls::UPPER_BOUND;
    fn get_upper_bound_estimate_from_bin(&self, bin: &Bin, rank: i64) -> f64 {
        return bin.get_upper_bound();
    }

    /// Places all values at the mid point of the bin except for the minimum and maximum recorded
    /// values.
    ///
    /// If this estimator is used for bins of a [`LogLinearLayout`] or {@link
    /// LogQuadraticLayout}, the maximum absolute and relative estimation errors will be bounded by
    /// half of the absolute and relative bin width limits, respectively.
    ///
    //const MID_POINT: ValueEstimator = ValueEstimatorImpls::MID_POINT;
    fn get_mid_point_estimate_from_bin(&self, bin: &Bin, rank: i64) -> f64 {
        return std::cmp::max(
            &bin.get_lower_bound(),
            &std::cmp::min(
                &bin.get_upper_bound(),
                (bin.get_lower_bound() + bin.get_upper_bound()) * 0.5,
            ),
        );
    }

    /// Estimate a recorded value with given zero-based rank from the given histogram.
    ///
    /// The estimated value must always be in the value range of the bin it belongs to.
    ///
    /// If rank == 0, [`Histogram::getMin()`] is returned. If rank == {@link
    /// Histogram#getTotalCount()} - 1, [`Histogram::getMax()`] is returned.
    ///
    /// - `histogram`: the histogram
    /// - `rank`: the zero-based rank
    ///
    /// the estimated value
    ///
    /// # Errors
    ///
    /// DynaHist::IllegalArgumentError if 0 &le; rank &lt; [`Histogram::getTotalCount()`] does not
    ///     hold
    ///
    fn get_value_estimate(&self, histogram: impl Histogram, rank: i64) -> f64 {
        let total_count: i64 = histogram.get_total_count();
        Self::check_argument(rank >= 0);
        Self::check_argument(rank < total_count);
        if rank <= 0 {
            return histogram.get_min();
        }
        if rank + 1 == total_count {
            return histogram.get_max();
        }
        let bin: dyn BinSketch = histogram.get_bin_by_rank(rank);
        return self.get_estimate_from_bin(bin, rank);
    }

    // The upstream Java implements `getEstimateFromBin` by overriding the
    // function according to how it is called. Example:
    //
    // ```java
    // ValueEstimatorImpls.LOWER_BOUND.getEstimateFromBin(BIN1, 14)
    // ```
    //
    // Currently, specialization is not possible in stable Rust.
    // To align the initial implementation with upstream we use this workaround
    // to implement specialization.
    //
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=46300603da13abb4d751f4cd7d2545f8
    //
    // use std::any::Any;
    //
    // struct MyS {}
    // impl MyS {
    //     fn new() -> Self {
    //         Self {}
    //     }
    // }
    //
    // fn as_type<'a, V: 'static, U: 'static>(x: &'a V) -> Option<&'a U> {
    //     <dyn Any>::downcast_ref(x)
    // }
    //
    // fn describe<V: 'static , T, S>(input: &V,_input2: &T, _input3: &S) -> String {
    //     if let Some(_f) = as_type::<_, f64>(input) {
    //         format!("it's a float")
    //     } else if let Some(_i) = as_type::<_, i32>(input) {
    //         format!("it's a integer")
    //     } else if let Some(_m) = as_type::<_, MyS>(input) {
    //         format!("it's mine")
    //     } else {
    //         format!("it's a thing")
    //     }
    // }
    //
    // fn main() {
    //     let m=MyS::new();
    //     println!("{}, {}, {}", describe(&1.,&1.,&1.),
    //                            describe(&1i32,&1,&1),
    //                            describe(&m,&1.,&1.));
    // }
    //
    // The `dyn Any` should not use an object trait, and should compile to
    // same thing as static (e.g. when using different names) at opt-level=2
    // and above.
    // A trait object isn't used because both of the following are true:
    //
    // 1. the concrete type is known, and
    // 2. the method being called is inline.
    //
    //
    /// Return the estimated value with given zero-based rank and bin.
    ///
    /// It can be assumed that the value of given rank was mapped into the
    /// given bin.
    /// Furthermore, this function will never be called for ranks corresponding
    /// to the minimum and maximum recorded value, because they are both stored
    /// explicitly by the histogram.
    ///
    fn as_type<'a, V: 'static, U: 'static>(x: &'a V) -> Option<&'a U> {
        <dyn std::any::Any>::downcast_ref(x)
    }

    // The original port implementtaion
    // fn get_estimate_from_bin(&self, bin: &Bin, rank: i64) -> f64;
    fn get_estimate_from_bin<'a, B: BinSketch, i64>(&self, bin: &B, rank: i64) -> f64 {
        if let Some(_u) = Self::as_type::<_, ValueEstimatorUniform>(self) {
            tracing::info!("it's a ValueEstimatorUniform");
            Self::get_uniform_estimate_from_bin(&self, bin, rank)
        } else if let Some(_l) = Self::as_type::<_, ValueEstimatorLowerBound>(self) {
            tracing::info!("it's a ValueEstimatorLowerBound");
            Self::get_lower_bound_estimate_from_bin(&self, bin, rank)
        } else if let Some(_p) = Self::as_type::<_, ValueEstimatorUpperBound>(self) {
            tracing::info!("it's a ValueEstimatorUpperBound");
            Self::get_upper_bound_estimate_from_bin(&self, bin, rank)
        } else if let Some(_m) = Self::as_type::<_, ValueEstimatorMidPoint>(self) {
            tracing::info!("it's a ValueEstimatorMidPoint");
            Self::get_mid_point_estimate_from_bin(&self, bin, rank)
        } else {
            tracing::info!("it's something we don't know")
        }
    }
}
