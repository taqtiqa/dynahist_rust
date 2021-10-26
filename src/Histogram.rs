/*
 * Copyright 2020-2021 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::dynatrace::dynahist;

pub trait Histogram {

    /**
   * Returns the underlying {@link Layout} of the histogram.
   *
   * @return the underlying {@link Layout} of the histogram
   */
    fn  get_layout(&self) -> Layout ;

    /**
   * Returns a {@link BinIterator} representing the first non-empty bin.
   *
   * <p>Must not be called if the histogram is empty.
   *
   * @return a {@link BinIterator} representing the first non-empty bin.
   * @throws NoSuchElementException if the histogram is empty
   */
    fn  get_first_non_empty_bin(&self) -> BinIterator ;

    /**
   * Returns a {@link BinIterator} representing the last non-empty bin.
   *
   * <p>Must not be called if the histogram is empty.
   *
   * @return a {@link BinIterator} representing last non-empty bin.
   * @throws NoSuchElementException if the histogram is empty
   */
    fn  get_last_non_empty_bin(&self) -> BinIterator ;

    /**
   * Returns a bin iterator, representing the bin containing the value with given rank (0-based)
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param rank, must be greater than or equal to 0 and smaller than {@link #getTotalCount()}
   * @return bin iterator, representing the bin containing the value with given order (0-based)
   */
    fn  get_bin_by_rank(&self,  rank: i64) -> BinIterator ;

    /**
   * Returns the number of added values greater than {@link Layout#getNormalRangeUpperBound()}.
   *
   * @return the number of added values greater than {@link Layout#getNormalRangeUpperBound()}
   */
    fn default  get_overflow_count(&self) -> i64  {
        if !self.is_empty() {
             let it: BinIterator = self.get_last_non_empty_bin();
            if it.is_overflow_bin() {
                return it.get_bin_count();
            }
        }
        return 0;
    }

    /**
   * Returns the number of added values less than {@link Layout#getNormalRangeLowerBound()}.
   *
   * @return the number of added values less than {@link Layout#getNormalRangeLowerBound()}
   */
    fn default  get_underflow_count(&self) -> i64  {
        if !self.is_empty() {
             let it: BinIterator = self.get_first_non_empty_bin();
            if it.is_underflow_bin() {
                return it.get_bin_count();
            }
        }
        return 0;
    }

    /**
   * Returns the total number of added values.
   *
   * @return the total number of added values
   */
    fn  get_total_count(&self) -> i64 ;

    /**
   * Returns the minimum of all added values.
   *
   * <p>Returns {@link Double#POSITIVE_INFINITY} if the histogram is empty.
   *
   * @return the minimum of all added values
   */
    fn  get_min(&self) -> f64 ;

    /**
   * Returns the maximum of all added values.
   *
   * <p>Returns {@link Double#NEGATIVE_INFINITY} if the histogram is empty.
   *
   * @return the maximum of all added values
   */
    fn  get_max(&self) -> f64 ;

    /**
   * Returns the number of values added to histogram bin with given index.
   *
   * @param binIndex the histogram bin index
   * @return the number of values added to histogram bin with given index
   */
    fn  get_count(&self,  bin_index: i32) -> i64 ;

    /**
   * Returns {@code true} if this histogram is empty.
   *
   * @return {@code true} if this histogram is empty
   */
    fn  is_empty(&self) -> bool ;

    /**
   * Returns an estimation for the value with given (zero-based) rank using the default value
   * estimator.
   *
   * <p>It is guaranteed that the estimated values returned by this function are never less than
   * {@link #getMin()} or greater than {@link #getMax()}. Furthermore, the estimated values will map
   * to the same bin again, if the mapping defined by the layout of this histogram is. Therefore,
   * starting from an empty histogram with the same layout and adding all estimated values once will
   * result in an equal copy of the histogram.
   *
   * <p>Example: If rank is equal to 1, an approximation for the second smallest value will be
   * returned.
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param rank the zero-based rank, must be nonnegative and less than {@link #getTotalCount()}
   * @return an approximation for the value with given rank
   */
    fn  get_value(&self,  rank: i64) -> f64 ;

    /**
   * Returns an estimation for the value with given (zero-based) rank using the given value
   * estimator.
   *
   * <p>It is guaranteed that the estimated values returned by this function are never less than
   * {@link #getMin()} or greater than {@link #getMax()}. Furthermore, the estimated values will map
   * to the same bin again, if the mapping defined by the layout of this histogram is. Therefore,
   * starting from an empty histogram with the same layout and adding all estimated values once will
   * result in an equal copy of the histogram.
   *
   * <p>Example: If rank is equal to 1, an approximation for the second smallest value will be
   * returned.
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param rank the zero-based rank, must be nonnegative and less than {@link #getTotalCount()}
   * @param valueEstimator the value estimator
   * @return an approximation for the value with given rank
   */
    fn  get_value(&self,  rank: i64,  value_estimator: &ValueEstimator) -> f64 ;

    /**
   * Returns an estimate for the quantile value using the estimated values as given by {@link
   * #getValue(long)} and the default quantile estimation method. The default behavior might change
   * in future. Therefore, if well-defined behavior is wanted, use {@link #getQuantile(double,
   * QuantileEstimator)}.
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param p the p-value in range [0,1]
   * @return an estimate for the p-quantile
   */
    fn  get_quantile(&self,  p: f64) -> f64 ;

    /**
   * Returns an estimate for the quantile value using the estimated values as given by {@link
   * #getValue(long)} and the given {@link QuantileEstimator}.
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param p the p-value in range [0,1]
   * @param quantileEstimator the quantile estimator
   * @return an estimate for the p-quantile
   */
    fn  get_quantile(&self,  p: f64,  quantile_estimator: &QuantileEstimator) -> f64 ;

    /**
   * Returns an estimate for the quantile value using the estimated values as given by {@link
   * #getValue(long)} and the given {@link QuantileEstimator}.
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param p the p-value in range [0,1]
   * @param valueEstimator the value estimator
   * @return an estimate for the p-quantile
   */
    fn  get_quantile(&self,  p: f64,  value_estimator: &ValueEstimator) -> f64 ;

    /**
   * Returns an estimate for the quantile value using the estimated values as given by {@link
   * #getValue(long)} and the given {@link QuantileEstimator}.
   *
   * <p>The runtime of this method may be O(N) where N is the number of bins. Therefore, if this
   * function is called many times, it is recommended to transform the histogram using {@link
   * #getPreprocessedCopy()} into a @link {@link PreprocessedHistogram} first (which is an O(N)
   * operation), whose implementation has a worst case complexity of O(log N).
   *
   * @param p the p-value in range [0,1]
   * @param quantileEstimator the quantile estimator
   * @param valueEstimator the value estimator
   * @return an estimate for the p-quantile
   */
    fn  get_quantile(&self,  p: f64,  quantile_estimator: &QuantileEstimator,  value_estimator: &ValueEstimator) -> f64 ;

    /**
   * Returns an estimate for the quantile value using the estimated values as given by {@link
   * #getValue(long)} using the default quantile estimator.
   *
   * <p>Preprocessing is recommended, if many calls of {@link #getBinByRank(long)} or {@link
   * #getValue(long)} are expected.
   *
   * @return an immutable pre-processed copy of this histogram
   */
    fn  get_preprocessed_copy(&self) -> Histogram ;

    /**
   * Adds a given value to the histogram.
   *
   * <p>Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   * {@link #isMutable()} returns {@code false}.
   *
   * @param value the value to be added to the histogram
   * @return a reference to this
   * @throws IllegalArgumentException if value is equal to {@link Double#NaN}
   * @throws ArithmeticException if the total count of the histogram would overflow
   * @throws UnsupportedOperationException if modifications are not supported
   */
    fn default  add_value(&self,  value: f64) -> Histogram  {
        return self.add_value(value, 1);
    }

    /**
   * Adds a given value to the histogram with a given multiplicity.
   *
   * <p>Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   * {@link #isMutable()} returns {@code false}.
   *
   * @param value the value to be added to the histogram
   * @param count defines the multiplicity
   * @return a reference to this
   * @throws IllegalArgumentException if value is equal to {@link Double#NaN} or count is negative
   * @throws ArithmeticException if the total count of the histogram would overflow
   * @throws UnsupportedOperationException if modifications are not supported
   */
    fn  add_value(&self,  value: f64,  count: i64) -> Histogram ;

    /**
   * Adds a given histogram to the histogram.
   *
   * <p>If the given histogram has a different layout than this histogram, this operation may lead
   * to unwanted loss of precision. In this case the operation is equivalent to adding all estimated
   * values as obtained by {@link #getValue(long)}.
   *
   * <p>Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   * {@link #isMutable()} returns {@code false}.
   *
   * @param histogram the histogram to be added
   * @return a reference to this
   * @throws ArithmeticException if the total count of the histogram would overflow
   * @throws UnsupportedOperationException if modifications are not supported
   */
    fn  add_histogram(&self,  histogram: &Histogram) -> Histogram ;

    /**
   * Adds a given histogram to the histogram.
   *
   * <p>If the given histogram has a different layout than this histogram, this operation may lead
   * to unwanted loss of precision. In this case the operation is equivalent to adding all estimated
   * values as obtained by {@link #getValue(long)}.
   *
   * <p>Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   * {@link #isMutable()} returns {@code false}.
   *
   * @param histogram the histogram to be added
   * @param valueEstimator the value estimator
   * @return a reference to this
   * @throws ArithmeticException if the total count of the histogram would overflow
   * @throws UnsupportedOperationException if modifications are not supported
   */
    fn  add_histogram(&self,  histogram: &Histogram,  value_estimator: &ValueEstimator) -> Histogram ;

    /**
   * Adds an ascending sequence to the histogram.
   *
   * <p>The function {@code ascendingSequence} must be defined for all arguments greater than or
   * equal to 0 and smaller than {@code length} and must be monotonic increasing. The behavior is
   * undefined otherwise.
   *
   * <p>By relying on the monotony of the provided sequence, histogram implementations can insert
   * the entire sequence with a time complexity that increases with the number of bins rather than
   * with the sequence length.
   *
   * <p>Throws an {@link UnsupportedOperationException}, if the implementation is not mutable and
   * {@link #isMutable()} returns {@code false}.
   *
   * @param ascendingSequence a {@link LongToDoubleFunction} defining the values of the ascending
   *     sequence
   * @param length the sequence length
   * @return a reference to this
   * @throws ArithmeticException if the total count of the histogram would overflow
   * @throws UnsupportedOperationException if modifications are not supported
   */
    fn  add_ascending_sequence(&self,  ascending_sequence: &LongToDoubleFunction,  length: i64) -> Histogram ;

    /**
   * Writes this histogram to a given {@link DataOutput}.
   *
   * <p>The {@link Layout} information will not be written. Therefore, it is necessary to provide
   * the layout when reading using {@link #readAsDynamic(Layout, DataInput)}, {@link
   * #readAsStatic(Layout, DataInput)} or {@link #readAsPreprocessed(Layout, DataInput)}.
   *
   * @param dataOutput the {@link DataOutput}
   * @throws IOException if an I/O error occurs
   */
    fn  write(&self,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>  ;

    /**
   * Provide an estimate of the histogram's total footprint in bytes
   *
   * @return estimate of the histogram's total footprint in bytes
   */
    fn  get_estimated_footprint_in_bytes(&self) -> i64 ;

    /**
   * Returns {@code true} if the implementation supports add operations.
   *
   * @return {@code true} if add operations are supported
   */
    fn  is_mutable(&self) -> bool ;

    /**
   * Creates an empty {@link Histogram} that allocates internal arrays for bin counts dynamically.
   *
   * <p>Choose this, if memory efficiency is more important than speed.
   *
   * @param layout the {@link Layout} of the histogram
   * @return an empty {@link Histogram}
   */
    fn  create_dynamic( layout: &Layout) -> Histogram  {
        return DynamicHistogram::new(layout);
    }

    /**
   * Creates an empty {@link Histogram} that allocates internal arrays for bin counts statically.
   *
   * <p>Choose this, if speed is more efficient than memory efficiency.
   *
   * @param layout the {@link Layout} of the histogram
   * @return an empty {@link Histogram}
   */
    fn  create_static( layout: &Layout) -> Histogram  {
        return StaticHistogram::new(layout);
    }

    /**
   * Reads a histogram from a given {@link DataInput}.
   *
   * <p>The returned histogram will allocate internal arrays for bin counts dynamically. The
   * behavior is undefined if the given layout does not match the layout before serialization.
   *
   * @param layout the {@link Layout}
   * @param dataInput the {@link DataInput}
   * @return the deserialized histogram
   * @throws IOException if an I/O error occurs
   */
    fn  read_as_dynamic( layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(DynamicHistogram::read(layout, &data_input));
    }

    /**
   * Reads a histogram from a given {@link DataInput}.
   *
   * <p>The returned histogram will allocate internal arrays for bin counts statically. The behavior
   * is undefined if the given layout does not match the layout before serialization.
   *
   * @param layout the {@link Layout}
   * @param dataInput the {@link DataInput}
   * @return the deserialized histogram
   * @throws IOException if an I/O error occurs
   */
    fn  read_as_static( layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(StaticHistogram::read(layout, &data_input));
    }

    /**
   * Reads a histogram from a given {@link DataInput}.
   *
   * <p>The returned histogram will be immutable and preprocessed in order to support fast queries.
   * The behavior is undefined if the given layout does not match the layout before serialization.
   *
   * @param layout the {@link Layout}
   * @param dataInput the {@link DataInput}
   * @return the deserialized histogram
   * @throws IOException if an I/O error occurs
   */
    fn  read_as_preprocessed( layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(DynamicHistogram::read(layout, &data_input)::get_preprocessed_copy());
    }

    /**
   * Returns an {@link Iterable} over all non-empty bins in ascending order.
   *
   * @return the iterable
   */
    fn  non_empty_bins_ascending(&self) -> Iterable<Bin> ;

    /**
   * Returns an {@link Iterable} over all non-empty bins in descending order.
   *
   * @return the iterable
   */
    fn  non_empty_bins_descending(&self) -> Iterable<Bin> ;
}

