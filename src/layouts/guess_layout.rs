// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::layouts::layout::Layout;

/// The trait used if there is an approximate formula (a guess) for the
/// reverse mapping (from bin index to bin boundaries).
///
///  Such approximations exist for:
///
/// - [`LogOptimalLayout`]
/// - [`LogQuadraticLayout`]
/// - [`LogLinearLayout`]
/// - [`OpenTelemetryLayout`]
///

pub(crate) trait GuessLayout: Layout {

    fn get_bin_lower_bound(&self, bin_index: i32) -> f64 {
        if bin_index <= self.get_underflow_bin_index() {
            return f64::NEG_INFINITY;
        }
        let effective_bin_index: i32 = std::cmp::min(self.get_overflow_bin_index(), bin_index);
        let approximate_bin_lower_bound: f64 =
            self.get_bin_lower_bound_approximation(effective_bin_index);
        let predicate =
            |&x: i32| self.map_to_bin_index(self.map_long_to_double(x)) >= effective_bin_index;
        let first = self.find_first_guess(
            predicate,
            self.NEGATIVE_INFINITY_MAPPED_TO_LONG,
            self.POSITIVE_INFINITY_MAPPED_TO_LONG,
            self.map_double_to_long(approximate_bin_lower_bound),
        );
        return self.map_long_to_double(first);
    }

    fn get_bin_upper_bound(&self, bin_index: i32) -> f64 {
        if bin_index >= self.get_overflow_bin_index() {
            return f64::INFINITY;
        }
        let effective_bin_index: i32 = std::cmp::max(self.get_underflow_bin_index(), bin_index);
        let approximate_bin_upper_bound: f64 =
            self.get_bin_lower_bound_approximation(effective_bin_index + 1);
        let predicate =
            |&x: i32| self.map_to_bin_index(self.map_long_to_double(x)) <= effective_bin_index;
        let first = self.find_first_guess(
            predicate,
            self.POSITIVE_INFINITY_MAPPED_TO_LONG,
            self.NEGATIVE_INFINITY_MAPPED_TO_LONG,
            self.map_double_to_long(approximate_bin_upper_bound),
        );
        return self.map_long_to_double(first);
    }

    /// Return an approximation of the lower bound of bin with given bin index.
    ///
    /// The method must be defined for all values greater than
    /// [`get_underflow_bin_index`] and smaller than or equal to
    /// [`get_overflow_bin_index`].
    ///
    /// The return value must not be [`f64::NAN`].
    ///
    fn get_bin_lower_bound_approximation(&self, bin_index: i32) -> f64;
}
