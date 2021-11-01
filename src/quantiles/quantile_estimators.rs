// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::quantiles::quantile_estimation::QuantileEstimation;
use crate::values::value_estimators::ValueEstimatorUniform;

/// A quantile estimator implementation based on the definition used by the {@code
/// scipy.stats.mstats.mquantiles} method in the SciPy Python library.
///
/// This class is immutable.
///
/// @see <a
///     href="https://docs.scipy.org/doc/scipy-1.5.2/reference/generated/scipy.stats.mstats.mquantiles.html">SciPy
///     reference for scipy.stats.mstats.mquantiles</a>
///

pub struct SciPyQuantileEstimator {
    alphap: f64,
    betap: f64,
}

impl QuantileEstimation for SciPyQuantileEstimator {
    type QEstimator = Self;
    type VEstimator = ValueEstimatorUniform;

    /// Return a [`QuantileEstimation`] instance that uses the SciPy quantile definition with given
    /// plotting positions parameters.
    ///
    /// @param alphap plotting positions parameter
    /// @param betap plotting positions parameter
    /// @return a [`QuantileEstimation`] instance
    ///
    fn create(alphap: f64, betap: f64) -> Self {
        return Self::new(alphap, betap);
    }

    /// Return a [`QuantileEstimation`] instance that uses the SciPy quantile definition with
    /// default parameters.
    ///
    /// @return a [`QuantileEstimation`] instance
    ///
    // fn create() -> Self {
    //     return Self::new(0.4, 0.4);
    // }

    fn new(alphap: f64, betap: f64) -> Self {
        Self::check_argument(alphap >= 0.0);
        Self::check_argument(alphap <= 1.0);
        Self::check_argument(betap >= 0.0);
        Self::check_argument(betap <= 1.0);
        SciPyQuantileEstimator {
            alphap,
            betap,
        }
    }

    fn estimate_quantile_from_fn(&self, p: f64, rank_fn: &fn(u64) -> f64, num_values: i64) -> f64 {
        if num_values == 0 {
            return f64::NAN;
        }
        if num_values == 1 {
            return rank_fn.apply_as_double(0);
        }
        let z: f64 = Self::interpolate(p, 0, self.alphap - 1.0, 1, num_values - self.betap);
        if z <= 0.0 {
            return rank_fn.apply_as_double(0);
        }
        if z >= num_values - 1.0 {
            return rank_fn.apply_as_double(num_values - 1);
        }
        let z_int_part: i64 = z as i64;
        let z_fraction_part: f64 = z - z_int_part;
        if z_fraction_part == 0.0 {
            return rank_fn.apply_as_double(z_int_part);
        }
        let y1: f64 = rank_fn.apply_as_double(z_int_part);
        let y2: f64 = rank_fn.apply_as_double(z_int_part + 1);
        return Self::interpolate(z_fraction_part, 0, y1, 1, y2);
    }

    fn to_string(&self) -> String {
        return format!(
            "{} [alphap={}, betap={}]",
            std::any::type_name::<Self>(),
            self.alphap,
            self.betap
        );
    }
}
