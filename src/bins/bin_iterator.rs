// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin::BinSketch;

/// A bin iterator over a histogram.
///
/// Allows iterating over all non-empty bins in ascending or descending order.
///
/// The behavior of the iterator is undefined, if the underlying histogram
/// is modified at the same time.
///
pub trait BinIterator: BinSketch {
    type B: BinSketch;

    /// Advance to the next non-empty bin.
    ///
    /// Must not be called if bin iterator represents the last non-empty bin,
    /// that is if [`is_last_non_empty_bin`] returns `true`.
    ///
    /// # Errors
    ///
    /// Return [`DynaHist::NoSuchElementError`] when called, and the current
    /// bin is the last non-empty bin.  TODO change this behavior to return `None`.
    ///
    fn next(&self);

    /// Retreat to the last non-empty bin.
    ///
    /// Must not be called if bin iterator represents the first non-empty bin,
    /// that is if [`is_first_non_empty_bin`] returns `true`.
    ///
    /// # Errors
    ///
    /// Return [`DynaHist::NoSuchElementError`] when a bin is the first non-empty bin
    ///
    fn previous(&self);

    /// Create a new [`Bin`] instance representing the current position of
    /// this bin iterator
    ///
    /// The returned bin is immutable with respect to this bin iterator.
    /// However, the behavior of the returned bin is undefined when the
    /// underlying histogram gets modified.
    ///
    fn get_bin_copy(&self) -> Self::B;
}
