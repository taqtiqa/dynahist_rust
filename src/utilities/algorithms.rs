// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::utilities::Preconditions;

pub trait Algorithms: Preconditions {
    const INVALID_PREDICATE_MSG_FORMAT_STRING: &'static str =
        "It is expected that the predicate evaluated at the maximum (%s) evaluates to true!";

    //let ininf =  f64::NEG_INFINITY as isize;
    //
    const NEGATIVE_INFINITY_MAPPED_TO_LONG: isize = f64::NEG_INFINITY as isize;

    // assert_eq!(Self::NEGATIVE_INFINITY_MAPPED_TO_LONG, isize::MIN);

    const POSITIVE_INFINITY_MAPPED_TO_LONG: isize = f64::INFINITY as isize;

    // assert_eq!(Self::POSITIVE_INFINITY_MAPPED_TO_LONG, isize::MAX);

    //fn new() -> Box<dyn Algorithms> {}

    /// Interpolates the y-value at given x-value from two given points (x1, y1) and (x2, y2).
    ///
    /// This implementation is strictly symmetric. Meaning that interpolate(x,x1,y1,x2,y2) ==
    /// interpolate(x,x2,y2,x1,y1) always holds. Furthermore, it is guaranteed that the return value is
    /// always in the range [min(y1,y2),max(y1,y2)]. In addition, this interpolation function is
    /// monotonic in x.
    ///
    /// - `x`: the x-value
    /// @param x1 the x-value of point 1
    /// @param y1 the y-value of point 1
    /// @param x2 the x-value of point 2
    /// @param y2 the y-value of point 2
    ///
    /// the interpolated y-value
    ///
    fn interpolate(x: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        // Java implementation uses java.lang.Double.doubleToLongBits().
        // Infinity in long bits: 9218868437227405312
        // -Infinity in long bits: -4503599627370496
        // NaN in long bits: 9221120237041090560
        // Use a home made equivalent, at least until we have a
        // test complete implementation in hand.
        if Self::to_bits_nan_collapse(y1) == Self::to_bits_nan_collapse(y2) {
            return y1;
        }
        if (x <= x1 && x1 < x2) || (x >= x1 && x1 > x2) {
            return y1;
        }
        if (x <= x2 && x2 < x1) || (x >= x2 && x2 > x1) {
            return y2;
        }
        let mut r: f64;
        if x1 != x2 && y1.is_finite() && y2.is_finite() {
            let delta_x: f64 = x2 - x1;
            let delta_y: f64 = y2 - y1;
            let r1: f64 = y1 + delta_y * ((x - x1) / delta_x);
            let r2: f64 = y2 + delta_y * ((x - x2) / delta_x);
            r = r1 * 0.5 + r2 * 0.5;
        } else {
            r = y1 * 0.5 + y2 * 0.5;
        }
        if r >= y1 && r >= y2 {
            return f64::max(y1, y2);
        } else if r <= y1 && r <= y2 {
            return f64::min(y1, y2);
        } else {
            return r;
        }
    }
    // Replicate java.lang.Double.doubleToLongBits().
    // The Rust function `f64::to_bits()` corresponds to
    // java.lang.Double.doubleToRawLongBits().
    // Java's Java.lang.Double.doubleToLongBits "collapses all the bit patterns
    // encoding a NaN to a single 'canonical' NaN value"
    // The "canonical" NAN value is the bits of 0x7ff8000000000000L.
    // Whereas Java's Java.lang.Double.doubleToRawLongBits and Rust's
    // f64::to_bits do not collapse all bit patterns in this way.
    //
    fn to_bits_nan_collapse(x: f64) {
        if x.is_nan() {
            f64::NAN.to_bits()
        } else {
            x.to_bits()
        }
    }

    // Replicate Java self.min_normal_f64():
    // "A constant holding the smallest positive normal value of type float
    // 2-126. It is equal to the hexadecimal floating-point literal
    // 0x1.0p-126f and also equal to Float.intBitsToFloat(0x00800000)."
    //
    fn min_normal_f64() -> f64 {
        f64::from_bits(0x00800000)
    }

    /// Calculates the midpoint of two given [`u64`] values rounded down to the nearest {@code
    /// long} value.
    ///
    /// This implementation works for any values which would lead to over- or underflows when
    /// calculating the midpoint using (a + b) / 2 directly. Furthermore, this implementation is
    /// branch-free.
    ///
    /// - `a`: the first value
    /// - `b`: the second value
    ///
    /// the midpoint
    ///
    fn calculate_midpoint(a: i64, b: i64) -> i64 {
        let a2: i64 = (a ^ 0x8000000000000000) >> /* >>> */ 1;
        let b2: i64 = (b ^ 0x8000000000000000) >> /* >>> */ 1;
        return ((a2 + b2) + (a & b & 1)) ^ 0x8000000000000000;
    }

    /// Bidirectional mapping of a {@code double} value to a [`u64`] value.
    ///
    /// Except for [`Double::NaN`] values, the natural ordering of double values as defined by
    /// [`Double::compare(double, double)`] will be maintained.
    ///
    /// Inverse mapping can be performed using [`#mapLongToDouble(long)`].
    ///
    /// - `x`: the value
    ///
    /// the corresponding [`f64`] value
    ///
    fn map_double_to_long(x: f64) -> i64 {
        let l: i64 = x.to_bits();
        return ((l >> 62) >> /* >>> */ 1) ^ l;
    }

    /// Bidirectional mapping of a [`u64`] value to a {@code double} value.
    ///
    /// Inverse mapping can be performed using [`#mapDoubleToLong(double)`].
    ///
    /// - `l`: [`f64`] value
    ///
    /// the corresponding double value
    ///
    fn map_long_to_double(l: i64) -> f64 {
        // Java implementation uses Double::long_bits_to_double
        return f64::from_bits(((l >> 62) >> /* >>> */ 1) ^ l);
    }

    /// Return the first [`f64`] value in the range `[min, max]`, for which the
    /// given predicate closure returns [`true`].
    ///
    /// The predicate must return [`false`] for all [`f64`] values smaller
    /// than some value `X` from `[min, max]` and must return [`true`] for
    /// all [`f64`] values equal to or greater than `X`.
    /// The return value of this function will be `X`.
    ///
    /// The time complexity is logarithmic in terms of the interval length
    /// `max - min`.
    ///
    /// # Arguments
    ///
    /// - `predicate`: the predicate closure
    /// - `min`: the lower bound, [`i64`], of the search interval
    /// - `max`: the upper bound, [`i64`], of the search interval
    ///
    fn find_first<P>(predicate: &P, min: i64, max: i64) -> i64
    where
        P: Fn(i64) -> bool,
    {
        Self::check_argument(min <= max);
        let mut low: i64 = min;
        let mut high: i64 = max;
        while low + 1 < high {
            let mid: i64 = Self::calculate_midpoint(low, high);
            if predicate.eval(mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        Self::check_argument_value(
            high != max || predicate.eval(high),
            &Self::INVALID_PREDICATE_MSG_FORMAT_STRING,
            max,
        );
        if low == min && low != high && predicate.eval(min) {
            return min;
        }
        return high;
    }

    /// Finds the first [`f64`] value in the range [min, max] for which the given
    ///  predicate returns [`true`].
    ///
    /// The predicate must return [`false`] for all [`f64`] values smaller than some value X from
    /// [min, max] and must return [`true`] for all [`f64`] values equal to or greater than X. The
    /// return value of this function will be X.
    ///
    /// The time complexity is logarithmic in terms of the interval length max - min.
    ///
    /// This function allows to give an initial guess which might speed up finding X, if the initial
    /// guess is already close to X.
    ///
    /// - `predicate`: the predicate
    /// - `min`: the lower bound of the search interval
    /// - `max`: the upper bound of the search interval
    /// - `initialGuess`: an initial guess
    ///
    /// the smallest value for which the predicate evaluates to [`true`]
    ///
    fn find_first_guess<P>(predicate: P, min: i64, max: i64, initial_guess: i64) -> i64
    where
        P: Fn(i64) -> bool,
    {
        Self::check_argument(min <= initial_guess);
        Self::check_argument(initial_guess <= max);
        let mut low: i64;
        let mut high: i64;
        let mut increment: i64 = 1;
        if predicate(initial_guess) {
            low = initial_guess;
            loop {
                {
                    high = low;
                    if high == min {
                        return min;
                    }
                    low = high - increment;
                    if low >= high || low < min {
                        low = min;
                    }
                    increment <<= 1;
                }
                if !(predicate(low)) {
                    break;
                }
            }
        } else {
            high = initial_guess;
            loop {
                {
                    low = high;
                    Self::check_argument_value(
                        low != max,
                        &Self::INVALID_PREDICATE_MSG_FORMAT_STRING,
                        max,
                    );
                    high = low + increment;
                    if high <= low || high > max {
                        high = max;
                    }
                    increment <<= 1;
                }
                if !(!predicate(high)) {
                    break;
                }
            }
        }
        while low + 1 < high {
            let mid: i64 = Self::calculate_midpoint(low, high);
            if predicate.eval(mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        return high;
    }

    /// Clips a given value to a given interval.
    ///
    /// - `value`: the value
    /// - `min`: the minimum value of the interval (inclusive)
    /// - `max`: the maximum value of the interval (inclusive)
    ///
    /// the clipped value
    ///
    fn clip(value: i32, min: i32, max: i32) -> Result<i32, DynaHistError> {
        if value >= min && value <= max {
            return value;
        } else if min > max {
            let source = format!("Illegal argument error - min > max: {} > {}", min, max);
            return Err(DynaHistError::IllegalArgumentError { source });
        } else if value >= min {
            return max;
        } else {
            return min;
        }
    }
}
