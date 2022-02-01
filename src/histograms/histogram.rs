// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin::BinSketch;
use crate::bins::bin_iterator::BinIterator;
use crate::errors::DynaHistError;
use crate::histograms::dynamic_histogram::DynamicHistogram;
use crate::histograms::fixed::StaticHistogram;
use crate::layouts::layout::Layout;
use crate::quantiles::quantile_estimation::QuantileEstimation;
use crate::seriate::SeriateHistogram;
use crate::sketches::data::{DataInput, DataOutput};
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;

// Use associated types to preserve static dispatch
pub trait Histogram {
    type L: Layout + Preconditions + Algorithms;
    type B: BinIterator + BinSketch + Iterator;
    type Q: QuantileEstimation;
    type V: ValueEstimation;

    /// Return the underlying [`Layout`] of the histogram.
    ///
    fn get_layout(&self) -> &Self::L {}

    /// Return a type that implements [`BinIterator`], representing the first
    /// non-empty bin.
    ///
    /// Must not be called if the histogram is empty.
    ///
    ///
    /// a [`BinIterator`] representing the first non-empty bin.
    ///
    /// # Errors
    ///
    /// NoSuchElementError if the histogram is empty
    ///
    fn get_first_non_empty_bin(&self) -> &Self::B;

    /// Return a [`BinIterator`] representing the last non-empty bin.
    ///
    /// Must not be called if the histogram is empty.
    ///
    ///
    /// a [`BinIterator`] representing last non-empty bin.
    ///
    /// # Errors
    ///
    /// NoSuchElementError if the histogram is empty
    ///
    fn get_last_non_empty_bin(&self) -> &Self::B;

    /// Return a bin iterator, representing the bin containing the value with given rank (0-based)
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - rank, must be greater than or equal to 0 and smaller than [`#getTotalCount()`]
    ///
    /// bin iterator, representing the bin containing the value with given order (0-based)
    ///
    fn get_bin_by_rank(&self, rank: i64) -> &Self::B;

    /// Return the number of added values greater than [`Layout::getNormalRangeUpperBound()`].
    ///
    ///
    /// the number of added values greater than [`Layout::getNormalRangeUpperBound()`]
    ///
    fn get_overflow_count(&self) -> i64 {
        if !self.is_empty() {
            let it: &Self::B = self.get_last_non_empty_bin();
            if it.is_overflow_bin() {
                return it.get_bin_count();
            }
        }
        return 0;
    }

    /// Return the number of added values less than [`Layout::getNormalRangeLowerBound()`].
    ///
    ///
    /// the number of added values less than [`Layout::getNormalRangeLowerBound()`]
    ///
    fn get_underflow_count(&self) -> i64 {
        if !self.is_empty() {
            let it: &Self::B = self.get_first_non_empty_bin();
            if it.is_underflow_bin() {
                return it.get_bin_count();
            }
        }
        return 0;
    }

    /// Return the total number of added values.
    ///
    ///
    /// the total number of added values
    ///
    fn get_total_count(&self) -> i64;

    /// Return the minimum of all added values.
    ///
    /// Return [`f64::NEG_INFINITY`] if the histogram is empty.
    ///
    fn get_min(&self) -> f64;

    /// Return the maximum of all added values.
    ///
    /// Return [`f64::NEG_INFINITY`] if the histogram is empty.
    ///
    fn get_max(&self) -> f64;

    /// Return the number of values added to histogram bin with given index.
    ///
    /// - `bin_index`: the histogram bin index
    fn get_count(&self, bin_index: i32) -> i64;

    /// Return [`true`] if this histogram is empty.
    ///
    fn is_empty(&self) -> bool;

    /// Return an estimation for the value with given (zero-based) rank using the default value
    /// estimator.
    ///
    /// It is guaranteed that the estimated values returned by this function are never less than
    /// [`#getMin()} or greater than {@link #getMax()`]. Furthermore, the estimated values will map
    /// to the same bin again, if the mapping defined by the layout of this histogram is. Therefore,
    /// starting from an empty histogram with the same layout and adding all estimated values once will
    /// result in an equal copy of the histogram.
    ///
    /// Example: If rank is equal to 1, an approximation for the second smallest value will be
    /// returned.
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - `rank`: the zero-based rank, must be nonnegative and less than [`#getTotalCount()`]
    ///
    /// an approximation for the value with given rank
    ///
    fn get_value(&self, rank: i64) -> f64;

    /// Return an estimate of the value with given (zero-based) rank, using
    /// the given value estimator.
    ///
    /// It is guaranteed that the estimated values returned by this function are never less than
    /// [`#getMin()} or greater than {@link #getMax()`]. Furthermore, the estimated values will map
    /// to the same bin again, if the mapping defined by the layout of this histogram is. Therefore,
    /// starting from an empty histogram with the same layout and adding all estimated values once will
    /// result in an equal copy of the histogram.
    ///
    /// Example: If rank is equal to 1, an approximation for the second smallest value will be
    /// returned.
    ///
    /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
    /// function is called many times, it is recommended to transform the histogram using
    /// [`get_preprocessed_copy()`] into a @link [`PreprocessedHistogram`] first (which is an O(N)
    /// operation), whose implementation has a worst case complexity of O(log N).
    ///
    /// - `rank`: the zero-based rank, must be nonnegative and less than [`#getTotalCount()`]
    /// - `value_estimator`: the value estimator
    ///
    /// an approximation for the value with given rank
    ///
    fn get_value_from_estimator(&self, rank: i64, value_estimator: &Self::V) -> f64;

    /// Return an estimate for the quantile value using the estimated values as given by
    /// [`get_value(i64)`] using the default quantile estimator.
    ///
    /// Preprocessing is recommended, if many calls of [`#getBinByRank(long)`] or
    /// [`get_value(i64)`] are expected.
    ///
    ///
    /// an immutable pre-processed copy of this histogram
    ///
    fn get_preprocessed_copy(&self) -> Self;

    /// Adds a given value to the histogram.
    ///
    /// Return an [`UnsupportedOperationError`], if the implementation is not mutable and
    /// [`#isMutable()} returns {@code false`].
    ///
    /// - `value`: the value to be added to the histogram
    ///
    /// a reference to this
    ///
    /// # Errors
    ///
    /// DynaHist::IllegalArgumentError if value is equal to [`Double::NaN`]
    ///
    /// # Errors
    ///
    /// ArithmeticError if the total count of the histogram would overflow
    ///
    /// # Errors
    ///
    /// UnsupportedOperationError if modifications are not supported
    ///
    fn add_value(&self, value: f64) -> Self {
        return self.add_values(value, 1);
    }

    /// Adds a given value to the histogram with a given multiplicity.
    ///
    /// Return an [`UnsupportedOperationError`], if the implementation is not mutable and
    /// [`#isMutable()} returns {@code false`].
    ///
    /// - `value`: the value to be added to the histogram
    /// - `count`: defines the multiplicity
    ///
    /// a reference to this
    ///
    /// # Errors
    ///
    /// DynaHist::IllegalArgumentError if value is equal to [`Double::NaN`] or count is negative
    ///
    /// # Errors
    ///
    /// ArithmeticError if the total count of the histogram would overflow
    ///
    /// # Errors
    ///
    /// UnsupportedOperationError if modifications are not supported
    ///
    fn add_values(&self, value: f64, count: i64) -> Self;

    /// Adds a given histogram to the histogram.
    ///
    /// If the given histogram has a different layout than this histogram, this operation may lead
    /// to unwanted loss of precision. In this case the operation is equivalent to adding all estimated
    /// values as obtained by [`get_value(i64)`].
    ///
    /// Return an [`UnsupportedOperationError`], if the implementation is not mutable and
    /// [`#isMutable()} returns {@code false`].
    ///
    /// - `histogram`: the histogram to be added
    ///
    /// a reference to this
    ///
    /// # Errors
    ///
    /// ArithmeticError if the total count of the histogram would overflow
    ///
    /// # Errors
    ///
    /// UnsupportedOperationError if modifications are not supported
    ///
    fn add_histogram(&self, histogram: &Self) -> Self;

    /// Add a given histogram to another, and return a reference to the
    /// resulting histogram.
    ///
    /// If the given histogram has a different layout than this histogram, this operation may lead
    /// to unwanted loss of precision. In this case the operation is equivalent to adding all estimated
    /// values, as obtained by [`get_value(i64)`].
    ///
    /// # Errors
    ///
    /// - [`DynaHistError::UnsupportedOperationError`], if the implementation is not mutable and
    /// [`is_mutable()`] returns [`false`].
    ///
    /// - [`DynaHistError::ArithmeticError`] if the total count of the histogram would overflow    ///
    /// - [`DynaHistError::UnsupportedOperationError`] if modifications are not supported
    ///
    /// # Arguments
    ///
    /// - `histogram`: the histogram to be added
    /// - `value_estimator`: the value estimator
    ///

    fn add_histogram_from_estimator(&self, histogram: &Self, value_estimator: &Self::V) -> Self;

    /// Adds an ascending sequence to the histogram.
    ///
    /// The function {@code ascendingSequence} must be defined for all arguments greater than or
    /// equal to 0 and smaller than {@code length} and must be monotonic increasing. The behavior is
    /// undefined otherwise.
    ///
    /// By relying on the monotony of the provided sequence, histogram implementations can insert
    /// the entire sequence with a time complexity that increases with the number of bins rather than
    /// with the sequence length.
    ///
    /// Return an [`UnsupportedOperationError`], if the implementation is not mutable and
    /// [`#isMutable()} returns {@code false`].
    ///
    /// - `ascendingSequence`: a [`Closure`] (`|x| {2*x}`) defining the values of the ascending
    ///     sequence
    /// - `length`: the sequence length
    ///
    /// a reference to this
    ///
    /// # Errors
    ///
    /// ArithmeticError if the total count of the histogram would overflow
    ///
    /// # Errors
    ///
    /// UnsupportedOperationError if modifications are not supported
    ///
    fn add_ascending_sequence<F: Fn(i64) -> f64>(
        &self,
        ascending_sequence: &F,
        length: i64,
    ) -> Self;

    /// Write this histogram to a given [`DataOutput`].
    ///
    /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
    /// the layout when reading using [`#read_as_dynamic(Layout, DataInput)`], {@link
    /// #readAsStatic(Layout, DataInput)} or [`#read_as_preprocessed(Layout, DataInput)`].
    ///
    /// - `dataOutput`: the [`DataOutput`]
    ///
    /// # Errors
    ///
    /// [`DynaHistError::IOError`] if an I/O error occurs
    ///
    fn write(&self, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError>>;

    /// Provide an estimate of the histogram's total footprint in bytes
    ///
    ///
    /// estimate of the histogram's total footprint in bytes
    ///
    fn get_estimated_footprint_in_bytes(&self) -> i64;

    /// Return [`true`] if the implementation supports add operations.
    ///
    ///
    /// [`true`] if add operations are supported
    ///
    fn is_mutable(&self) -> bool;

    /// Create an empty [`Histogram`] that allocates internal arrays for bin counts dynamically.
    ///
    /// Choose this, if memory efficiency is more important than speed.
    ///
    /// - `layout`: the [`Layout`] of the histogram
    ///
    /// an empty [`Histogram`]
    ///
    fn create_dynamic(layout: impl Layout) -> Self {
        return DynamicHistogram::new(layout);
    }

    /// Create an empty [`Histogram`] that allocates internal arrays for bin counts statically.
    ///
    /// Choose this, if speed is more efficient than memory efficiency.
    ///
    /// - `layout`: the [`Layout`] of the histogram
    ///
    /// an empty [`Histogram`]
    ///
    fn create_static(layout: impl Layout) -> Self {
        return StaticHistogram::new(layout);
    }

    /// Read a histogram from a given [`DataInput`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts dynamically. The
    /// behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// - [`layout`: the [`Layout`]
    /// - [`data_input`] the [`DataInput`]
    ///
    /// the deserialized histogram
    ///
    /// # Errors
    ///
    /// [`DynaHistError::IOError`] if an I/O error occurs
    ///
    fn read_as_dynamic(
        layout: impl Layout,
        data_input: &DataInput,
    ) -> Result<Self, std::rc::Rc<DynaHistError>>
    where
        Self: Sized,
    {
        return Ok(DynamicHistogram::read(layout, &data_input));
    }

    /// Read a histogram from a given [`DataInput`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
    /// is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - [`data_input`] the [`DataInput`]
    ///
    /// the deserialized histogram
    ///
    /// # Errors
    ///
    /// [`DynaHistError::IOError`] if an I/O error occurs
    ///
    fn read_as_static(
        layout: impl Layout,
        data_input: &DataInput,
    ) -> Result<Self, std::rc::Rc<DynaHistError>>
    where
        Self: Sized,
    {
        return Ok(StaticHistogram::read(layout, &data_input));
    }

    /// Read a histogram from a given [`DataInput`].
    ///
    /// The returned histogram will be immutable and preprocessed in order to support fast queries.
    /// The behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - [`data_input`] the [`DataInput`]
    ///
    /// the deserialized histogram
    ///
    /// # Errors
    ///
    /// [`DynaHistError::IOError`] if an I/O error occurs
    ///
    fn read_as_preprocessed(
        layout: impl Layout,
        data_input: &DataInput,
    ) -> Result<Self, std::rc::Rc<DynaHistError>>
    where
        Self: Sized,
    {
        let histogram = DynamicHistogram::read(layout, &data_input);
        return Ok(histogram.get_preprocessed_copy());
    }

    /// Return an [`Iterator`] over all non-empty bins in ascending order.
    ///
    fn non_empty_bins_ascending(&self) -> &Self::B;

    /// Return an [`Iterator`] over all non-empty bins in descending order.
    ///
    fn non_empty_bins_descending(&self) -> &Self::B;
}
