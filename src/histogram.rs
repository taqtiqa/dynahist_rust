// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin_iterator::BinIterator;
use crate::bins::bin::Bin;
use crate::layouts::layout::Layout;
use crate::utilities::Algorithms;
//use crate::utilities::data::{ DataInput, DataOutput };
use crate::utilities::Preconditions;

// Use associated types to preserve static dispatch
pub trait Histogram {
    type L: Layout + Preconditions + Algorithms;
    type B: BinIterator + Bin;


   /// Return the underlying [`Layout`] of the histogram.
   ///
    fn get_layout(&self) -> &Self::L {}

   /// Return a type that implements [`BinIterator`], representing the first
   /// non-empty bin.
   ///
   /// Must not be called if the histogram is empty.
   ///
   /// @return a [`BinIterator`] representing the first non-empty bin.
   /// @throws NoSuchElementException if the histogram is empty
   ///
    fn get_first_non_empty_bin(&self) -> &Self::B ;

   /// Return a [`BinIterator`] representing the last non-empty bin.
   ///
   /// Must not be called if the histogram is empty.
   ///
   /// @return a [`BinIterator`] representing last non-empty bin.
   /// @throws NoSuchElementException if the histogram is empty
   ///
    fn get_last_non_empty_bin(&self) -> &Self::B ;


   /// Return a bin iterator, representing the bin containing the value with given rank (0-based)
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param rank, must be greater than or equal to 0 and smaller than {@link #getTotalCount()}
   /// @return bin iterator, representing the bin containing the value with given order (0-based)
   ///
    fn get_bin_by_rank(&self,  rank: i64) -> &Self::B ;


   /// Return the number of added values greater than {@link Layout#getNormalRangeUpperBound()}.
   ///
   /// @return the number of added values greater than {@link Layout#getNormalRangeUpperBound()}
   ///
    fn get_overflow_count(&self) -> i64  {
        if !self.is_empty() {
             let it: &Self::B = self.get_last_non_empty_bin();
            if it.is_overflow_bin() {
                return it.get_bin_count();
            }
        }
        return 0;
    }


   /// Return the number of added values less than {@link Layout#getNormalRangeLowerBound()}.
   ///
   /// @return the number of added values less than {@link Layout#getNormalRangeLowerBound()}
   ///
    fn get_underflow_count(&self) -> i64  {
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
   /// @return the total number of added values
   ///
    fn get_total_count(&self) -> i64 ;


   /// Return the minimum of all added values.
   ///
   /// Return [`f64::NEG_INFINITY`] if the histogram is empty.
   ///
    fn get_min(&self) -> f64 ;


   /// Return the maximum of all added values.
   ///
   /// Return [`f64::NEG_INFINITY`] if the histogram is empty.
   ///
    fn get_max(&self) -> f64 ;


   /// Return the number of values added to histogram bin with given index.
   ///
   /// @param bin_index the histogram bin index
    fn get_count(&self,  bin_index: i32) -> i64 ;


   /// Return {@code true} if this histogram is empty.
   ///
    fn is_empty(&self) -> bool ;


   /// Return an estimation for the value with given (zero-based) rank using the default value
   /// estimator.
   ///
   /// It is guaranteed that the estimated values returned by this function are never less than
   /// {@link #getMin()} or greater than {@link #getMax()}. Furthermore, the estimated values will map
   /// to the same bin again, if the mapping defined by the layout of this histogram is. Therefore,
   /// starting from an empty histogram with the same layout and adding all estimated values once will
   /// result in an equal copy of the histogram.
   ///
   /// Example: If rank is equal to 1, an approximation for the second smallest value will be
   /// returned.
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param rank the zero-based rank, must be nonnegative and less than {@link #getTotalCount()}
   /// @return an approximation for the value with given rank
   ///
    fn get_value(&self,  rank: i64) -> f64 ;


   /// Return an estimate of the value with given (zero-based) rank, using
   /// the given value estimator.
   ///
   /// It is guaranteed that the estimated values returned by this function are never less than
   /// {@link #getMin()} or greater than {@link #getMax()}. Furthermore, the estimated values will map
   /// to the same bin again, if the mapping defined by the layout of this histogram is. Therefore,
   /// starting from an empty histogram with the same layout and adding all estimated values once will
   /// result in an equal copy of the histogram.
   ///
   /// Example: If rank is equal to 1, an approximation for the second smallest value will be
   /// returned.
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param rank the zero-based rank, must be nonnegative and less than {@link #getTotalCount()}
   /// @param valueEstimator the value estimator
   /// @return an approximation for the value with given rank
   ///
    fn get_value_from_estimator(&self,   rank: i64,   value_estimator: &ValueEstimator) -> f64 ;


   /// Return an estimate for the quantile value using the estimated values as given by {@link
   /// #getValue(long)} and the default quantile estimation method. The default behavior might change
   /// in future. Therefore, if well-defined behavior is wanted, use {@link #getQuantile(double,
   /// QuantileEstimator)}.
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param p the p-value in range [0,1]
   /// @return an estimate for the p-quantile
   ///
    fn get_quantile(&self,  p: f64) -> f64 ;


   /// Return an estimate for the quantile value using the estimated values as given by {@link
   /// #getValue(long)} and the given {@link QuantileEstimator}.
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param p the p-value in range [0,1]
   /// @param quantileEstimator the quantile estimator
   /// @return an estimate for the p-quantile
   ///
    fn get_quantile(&self,  p: f64,  quantile_estimator: &QuantileEstimator) -> f64 ;


   /// Return an estimate for the quantile value using the estimated values as given by {@link
   /// #getValue(long)} and the given {@link QuantileEstimator}.
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param p the p-value in range [0,1]
   /// @param valueEstimator the value estimator
   /// @return an estimate for the p-quantile
   ///
    fn get_quantile(&self,  p: f64,  value_estimator: &ValueEstimator) -> f64 ;


   /// Return an estimate for the quantile value using the estimated values as given by {@link
   /// #getValue(long)} and the given {@link QuantileEstimator}.
   ///
   /// The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   /// function is called many times, it is recommended to transform the histogram using {@link
   /// #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   /// operation), whose implementation has a worst case complexity of O(log N).
   ///
   /// @param p the p-value in range [0,1]
   /// @param quantileEstimator the quantile estimator
   /// @param valueEstimator the value estimator
   /// @return an estimate for the p-quantile
   ///
    fn get_quantile(&self,  p: f64,  quantile_estimator: &QuantileEstimator,  value_estimator: &ValueEstimator) -> f64 ;


   /// Return an estimate for the quantile value using the estimated values as given by {@link
   /// #getValue(long)} using the default quantile estimator.
   ///
   /// Preprocessing is recommended, if many calls of {@link #getBinByRank(long)} or {@link
   /// #getValue(long)} are expected.
   ///
   /// @return an immutable pre-processed copy of this histogram
   ///
    fn get_preprocessed_copy(&self) -> Histogram ;


   /// Adds a given value to the histogram.
   ///
   /// Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   /// {@link #isMutable()} returns {@code false}.
   ///
   /// @param value the value to be added to the histogram
   /// @return a reference to this
   /// @throws IllegalArgumentException if value is equal to {@link Double#NaN}
   /// @throws ArithmeticException if the total count of the histogram would overflow
   /// @throws UnsupportedOperationException if modifications are not supported
   ///
    fn add_value(&self,  value: f64) -> Histogram  {
        return self.add_value(value, 1);
    }


   /// Adds a given value to the histogram with a given multiplicity.
   ///
   /// Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   /// {@link #isMutable()} returns {@code false}.
   ///
   /// @param value the value to be added to the histogram
   /// @param count defines the multiplicity
   /// @return a reference to this
   /// @throws IllegalArgumentException if value is equal to {@link Double#NaN} or count is negative
   /// @throws ArithmeticException if the total count of the histogram would overflow
   /// @throws UnsupportedOperationException if modifications are not supported
   ///
    fn add_value(&self,  value: f64,  count: i64) -> Histogram ;


   /// Adds a given histogram to the histogram.
   ///
   /// If the given histogram has a different layout than this histogram, this operation may lead
   /// to unwanted loss of precision. In this case the operation is equivalent to adding all estimated
   /// values as obtained by {@link #getValue(long)}.
   ///
   /// Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   /// {@link #isMutable()} returns {@code false}.
   ///
   /// @param histogram the histogram to be added
   /// @return a reference to this
   /// @throws ArithmeticException if the total count of the histogram would overflow
   /// @throws UnsupportedOperationException if modifications are not supported
   ///
    fn add_histogram(&self,  histogram: &Histogram) -> Histogram ;


   /// Adds a given histogram to the histogram.
   ///
   /// If the given histogram has a different layout than this histogram, this operation may lead
   /// to unwanted loss of precision. In this case the operation is equivalent to adding all estimated
   /// values as obtained by {@link #getValue(long)}.
   ///
   /// Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   /// {@link #isMutable()} returns {@code false}.
   ///
   /// @param histogram the histogram to be added
   /// @param valueEstimator the value estimator
   /// @return a reference to this
   /// @throws ArithmeticException if the total count of the histogram would overflow
   /// @throws UnsupportedOperationException if modifications are not supported
   ///
    fn add_histogram(&self,  histogram: &Histogram,  value_estimator: &ValueEstimator) -> Histogram ;


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
   /// Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   /// {@link #isMutable()} returns {@code false}.
   ///
   /// @param ascendingSequence a {@link LongToDoubleFunction} defining the values of the ascending
   ///     sequence
   /// @param length the sequence length
   /// @return a reference to this
   /// @throws ArithmeticException if the total count of the histogram would overflow
   /// @throws UnsupportedOperationException if modifications are not supported
   ///
    fn add_ascending_sequence(&self,  ascending_sequence: &LongToDoubleFunction,  length: i64) -> Histogram ;


   /// Writes this histogram to a given {@link DataOutput}.
   ///
   /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
   /// the layout when reading using {@link #readAsDynamic(Layout, DataInput)}, {@link
   /// #readAsStatic(Layout, DataInput)} or {@link #readAsPreprocessed(Layout, DataInput)}.
   ///
   /// @param dataOutput the {@link DataOutput}
   /// @throws IOException if an I/O error occurs
   ///
    fn write(&self,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>  ;


   /// Provide an estimate of the histogram's total footprint in bytes
   ///
   /// @return estimate of the histogram's total footprint in bytes
   ///
    fn get_estimated_footprint_in_bytes(&self) -> i64 ;


   /// Return {@code true} if the implementation supports add operations.
   ///
   /// @return {@code true} if add operations are supported
   ///
    fn is_mutable(&self) -> bool ;


   /// Create an empty {@link Histogram} that allocates internal arrays for bin counts dynamically.
   ///
   /// Choose this, if memory efficiency is more important than speed.
   ///
   /// @param layout the [`Layout`] of the histogram
   /// @return an empty {@link Histogram}
   ///
    fn create_dynamic( layout: &Layout) -> Histogram  {
        return DynamicHistogram::new(layout);
    }


   /// Create an empty {@link Histogram} that allocates internal arrays for bin counts statically.
   ///
   /// Choose this, if speed is more efficient than memory efficiency.
   ///
   /// @param layout the [`Layout`] of the histogram
   /// @return an empty {@link Histogram}
   ///
    fn create_static( layout: &Layout) -> Histogram  {
        return StaticHistogram::new(layout);
    }


   /// Reads a histogram from a given {@link DataInput}.
   ///
   /// The returned histogram will allocate internal arrays for bin counts dynamically. The
   /// behavior is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param dataInput the {@link DataInput}
   /// @return the deserialized histogram
   /// @throws IOException if an I/O error occurs
   ///
    fn read_as_dynamic( layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(DynamicHistogram::read(layout, &data_input));
    }


   /// Reads a histogram from a given {@link DataInput}.
   ///
   /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
   /// is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param dataInput the {@link DataInput}
   /// @return the deserialized histogram
   /// @throws IOException if an I/O error occurs
   ///
    fn read_as_static( layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(StaticHistogram::read(layout, &data_input));
    }


   /// Reads a histogram from a given {@link DataInput}.
   ///
   /// The returned histogram will be immutable and preprocessed in order to support fast queries.
   /// The behavior is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param dataInput the {@link DataInput}
   /// @return the deserialized histogram
   /// @throws IOException if an I/O error occurs
   ///
    fn read_as_preprocessed( layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(DynamicHistogram::read(layout, &data_input)::get_preprocessed_copy());
    }


   /// Return an {@link Iterable} over all non-empty bins in ascending order.
   ///
   /// @return the iterable
   ///
    fn non_empty_bins_ascending(&self) -> Iterable<Bin> ;


   /// Return an {@link Iterable} over all non-empty bins in descending order.
   ///
   /// @return the iterable
   ///
    fn non_empty_bins_descending(&self) -> Iterable<Bin> ;
}
