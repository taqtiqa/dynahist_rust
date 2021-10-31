// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/**
 * A quantile estimator implementation based on the definition used by the {@code
 * scipy.stats.mstats.mquantiles} method in the SciPy Python library.
 *
 * This class is immutable.
 *
 * @see <a
 *     href="https://docs.scipy.org/doc/scipy-1.5.2/reference/generated/scipy.stats.mstats.mquantiles.html">SciPy
 *     reference for scipy.stats.mstats.mquantiles</a>
 */

 const DEFAULT: QuantileEstimator = // this is also the default in SciPy version 1.5.2
::create(0.4, 0.4);
#[derive(QuantileEstimator)]
pub struct SciPyQuantileEstimator {

     let alphap: f64;

     let betap: f64;
}

impl SciPyQuantileEstimator {


   /// Return a [`QuantileEstimation`] instance that uses the SciPy quantile definition with given
   /// plotting positions parameters.
   ///
   /// @param alphap plotting positions parameter
   /// @param betap plotting positions parameter
   /// @return a [`QuantileEstimation`] instance
   ///
    pub fn create( alphap: f64,  betap: f64) -> QuantileEstimator  {
        return SciPyQuantileEstimator::new(alphap, betap);
    }


   /// Return a [`QuantileEstimation`] instance that uses the SciPy quantile definition with
   /// default parameters.
   ///
   /// @return a [`QuantileEstimation`] instance
   ///
    pub fn create() -> QuantileEstimator  {
        return DEFAULT;
    }

    fn new( alphap: f64,  betap: f64) -> SciPyQuantileEstimator {
        check_argument(alphap >= 0.0);
        check_argument(alphap <= 1.0);
        check_argument(betap >= 0.0);
        check_argument(betap <= 1.0);
        let .alphap = alphap;
        let .betap = betap;
    }

    pub fn estimate_quantile(&self,  p: f64,  sorted_value_function: &LongToDoubleFunction,  num_values: i64) -> f64  {
        if num_values == 0 {
            return f64::NAN;
        }
        if num_values == 1 {
            return sorted_value_function.apply_as_double(0);
        }
         let z: f64 = Algorithms::interpolate(p, 0, self.alphap - 1.0, 1, num_values - self.betap);
        if z <= 0.0 {
            return sorted_value_function.apply_as_double(0);
        }
        if z >= num_values - 1.0 {
            return sorted_value_function.apply_as_double(num_values - 1);
        }
         let z_int_part: i64 = z as i64;
         let z_fraction_part: f64 = z - z_int_part;
        if z_fraction_part == 0.0 {
            return sorted_value_function.apply_as_double(z_int_part);
        }
         let y1: f64 = sorted_value_function.apply_as_double(z_int_part);
         let y2: f64 = sorted_value_function.apply_as_double(z_int_part + 1);
        return Algorithms::interpolate(z_fraction_part, 0, y1, 1, y2);
    }

    pub fn to_string(&self) -> String  {
        return format!("{} [alphap={}, betap={}]", get_class().get_simple_name(), self.alphap, self.betap);
    }
}
