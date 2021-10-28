// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// A histogram bin.
pub trait Bin {
    /// Return the number of values belonging to the current bin position.
    ///
    fn get_bin_count(&self) -> i64;

    /// Return a lower bound for all values in this bin.
    ///
    /// The returned value is always greater than or equal to the minimum value of the underlying
    /// histogram.
    ///
    fn get_lower_bound(&self) -> f64;

    /// Return an upper bound for all values in this bin.
    ///
    /// The returned value is always less than or equal to the maximum value of the underlying
    /// histogram.
    ///
    fn get_upper_bound(&self) -> f64;

    /// Return the width of the current bin.
    ///
    fn get_width(&self) -> f64 {
        return self.get_upper_bound() - self.get_lower_bound();
    }

    /// Return the number of values less than the lower bound of the the current bin.
    ///
    fn get_less_count(&self) -> i64;

    /// Return the number of values greater than the upper bound of the the current bin.
    ///
    fn get_greater_count(&self) -> i64;

    /// Return the bin index as defined by the [`Layout`].
    ///
    fn get_bin_index(&self) -> i32;

    /// Return {@code true} if this bin corresponds to the first non-empty bin.
    ///
    fn is_first_non_empty_bin(&self) -> bool {
        return self.get_less_count() == 0;
    }

    /// Return {@code true} if this bin corresponds to the last non-empty bin.
    ///
    fn is_last_non_empty_bin(&self) -> bool {
        return self.get_greater_count() == 0;
    }

    /// Return {@code true} if this bin corresponds to the underflow bin.
    ///
    fn is_underflow_bin(&self) -> bool;

    /// Return {@code true} if this bin corresponds to the overflow bin.
    ///
    fn is_overflow_bin(&self) -> bool;
}
