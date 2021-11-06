// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin::BinSketch;
use crate::bins::bin_iterator::BinIterator;
use crate::errors::DynaHistError;
// use crate::histograms::dynamic_histogram::DynamicHistogram;
use crate::histograms::abstract_histogram::AbstractHistogram;
use crate::histograms::histogram::Histogram;
use crate::histograms::abstract_mutable_histogram::AbstractMutableHistogram;
use crate::layouts::layout::Layout;
use crate::quantiles::quantile_estimation::QuantileEstimation;
use crate::sketches::data::DataInput;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;

/// A preprocessed and immutable histogram that allows fast order statistic queries.

pub struct PreprocessedHistogram {
    accumulated_counts: Vec<i64>,
    histogram_type: String,
    max: f64,
    min: f64,
    non_empty_bin_indices: Vec<i32>,
}

impl PreprocessedHistogram {

    const EMPTY_BIN_INDICES: Vec<usize> = vec![0];

    const EMPTY_ACCUMULATED_COUNTS: usize = 0;

    fn of( histogram: impl Histogram) -> impl Histogram {
        if histogram.histogram_type == "PreprocessedHistogram".to_string() {
            return histogram;
        } else {
            return Self::new(histogram);
        }
    }

    fn new( histogram: impl Histogram) -> Self {
        let layout = &histogram.get_layout();
        layout.min = histogram.get_min();
        layout.max = histogram.get_max();
        if histogram.is_empty() {
            layout.non_empty_bin_indices = Self::EMPTY_BIN_INDICES;
            layout.accumulated_counts = Self::EMPTY_ACCUMULATED_COUNTS;
        } else {
             let first_non_empty_bin: BinIterator = histogram.get_first_non_empty_bin();
             let last_non_empty_bin: BinIterator = histogram.get_last_non_empty_bin();
             let first_non_empty_bin_index: i32 = first_non_empty_bin.get_bin_index();
             let last_non_empty_bin_index: i32 = last_non_empty_bin.get_bin_index();
             let bin_index_range: i32 = last_non_empty_bin_index - first_non_empty_bin_index + 1;
             let non_empty_bin_indices_tmp: [i32; bin_index_range] = [0; bin_index_range];
             let accumulated_counts_tmp: [i64; bin_index_range] = [0; bin_index_range];
             let bin_iterator: BinIterator = first_non_empty_bin;
            non_empty_bin_indices_tmp[0] = bin_iterator.get_bin_index();
            accumulated_counts_tmp[0] = bin_iterator.get_bin_count();
             let non_empty_bin_counter: i32 = 1;
            while !bin_iterator.is_last_non_empty_bin() {
                bin_iterator.next();
                non_empty_bin_indices_tmp[non_empty_bin_counter] = bin_iterator.get_bin_index();
                accumulated_counts_tmp[non_empty_bin_counter] = accumulated_counts_tmp[non_empty_bin_counter - 1] + bin_iterator.get_bin_count();
                non_empty_bin_counter += 1;
            }
            non_empty_bin_indices = Arrays::copy_of(&non_empty_bin_indices_tmp, non_empty_bin_counter);
            accumulated_counts = Arrays::copy_of(&accumulated_counts_tmp, non_empty_bin_counter);
        }
    }

    fn get_bin_by_rank(&self,  rank: i64) -> BinIterator {
         let total_count: i64 = self.get_total_count();
        Self::check_argument(rank >= 0);
        Self::check_argument(rank < total_count);
         let i: i32 = Arrays::binary_search(&self.accumulated_counts, rank + 1);
        return BinIteratorImpl::new( if (i >= 0) { i } else { -(i + 1) });
    }

    fn get_count_of_non_empty_bin(&self,  non_empty_bin_index: i32) -> i64 {
        if non_empty_bin_index > 0 {
            return self.accumulated_counts[non_empty_bin_index] - self.accumulated_counts[non_empty_bin_index - 1];
        } else {
            return self.accumulated_counts[non_empty_bin_index];
        }
    }

    fn check_if_element_exists(&self) {
        if self.is_empty() {
            return Err(DynaHistError::NoSuchElementError);
        }
    }

    fn get_first_non_empty_bin(&self) -> BinIterator {
        self.check_if_element_exists();
        return BinIteratorImpl::new(0);
    }

    fn get_last_non_empty_bin(&self) -> BinIterator {
        self.check_if_element_exists();
        return BinIteratorImpl::new(self.non_empty_bin_indices.len() - 1);
    }

    fn get_total_count(&self) -> i64 {
        return  if (self.accumulated_counts.len() > 0) { self.accumulated_counts[self.accumulated_counts.len() - 1] } else { 0 };
    }

    fn get_min(&self) -> f64 {
        return self.min;
    }

    fn get_max(&self) -> f64 {
        return self.max;
    }

    fn get_count(&self,  bin_index: i32) -> i64 {
         let non_empty_bin_index: i32 = Arrays::binary_search(&self.non_empty_bin_indices, bin_index);
        if non_empty_bin_index >= 0 {
            return self.get_count_of_non_empty_bin(non_empty_bin_index);
        } else {
            return 0;
        }
    }

    struct AbstractBinExtended {
        super: AbstractBin;
    }

    impl AbstractBinExtended {

        fn get_non_empty_index(&self) -> i32 ;

        fn get_bin_count(&self) -> i64 {
            return self.get_count_of_non_empty_bin(&self.get_non_empty_index());
        }

        fn get_less_count(&self) -> i64 {
            return  if (self.get_non_empty_index() > 0) { self.accumulated_counts[self.get_non_empty_index() - 1] } else { 0 };
        }

        fn get_greater_count(&self) -> i64 {
            return self.get_total_count() - self.accumulated_counts[self.get_non_empty_index()];
        }

        fn get_bin_index(&self) -> i32 {
            return self.non_empty_bin_indices[self.get_non_empty_index()];
        }

        fn get_histogram(&self) -> impl Histogram {
            return PreprocessedHistogram;
        }
    }


    struct BinCopyImpl {
        super: AbstractBinExtended;

         let non_empty_bin_index: i32;
    }

    impl BinCopyImpl {

        fn new( non_empty_bin_index: i32) -> BinCopyImpl {
            let .nonEmptyBinIndex = non_empty_bin_index;
        }

        fn get_non_empty_index(&self) -> i32 {
            return self.non_empty_bin_index;
        }
    }


    #[derive(BinIterator)]
    struct BinIteratorImpl {
        super: AbstractBinExtended;

         let non_empty_bin_index: i32;
    }

    impl BinIteratorImpl {

        fn new( non_empty_bin_index: i32) -> BinIteratorImpl {
            let .nonEmptyBinIndex = non_empty_bin_index;
        }

        fn get_non_empty_index(&self) -> i32 {
            return self.non_empty_bin_index;
        }

        fn next(&self) {
            if self.non_empty_bin_index + 1 >= self.accumulated_counts.len() {
                return Err(DynaHistError::NoSuchElementError);
            }
            self.non_empty_bin_index += 1;
        }

        fn previous(&self) {
            if self.non_empty_bin_index <= 0 {
                return Err(DynaHistError::NoSuchElementError);
            }
            self.non_empty_bin_index -= 1;
        }

        fn get_bin_copy(&self) -> BinSketch {
            return BinCopyImpl::new(self.non_empty_bin_index);
        }
    }


    fn add_values(&self,  value: f64,   count: i64) -> impl Histogram {
        return Err(DynaHistError::UnsupportedOperationError::new());
    }

    fn add_value(&self,  value: f64) -> impl Histogram {
        return Err(DynaHistError::UnsupportedOperationError::new());
    }

    fn add_histogram_from_estimator(&self,  histogram: impl Histogram,  value_estimator: &ValueEstimator) -> impl Histogram {
        return Err(DynaHistError::UnsupportedOperationError::new());
    }

    fn add_ascending_sequence<F: Fn(i64) -> f64>(&self,  ascending_sequence: &F,  length: i64) -> impl Histogram {
        return Err(DynaHistError::UnsupportedOperationError::new());
    }

    fn write(&self,  data_output: &DataOutput)  -> Result<(), std::rc::Rc<DynaHistError>> {
        Histogram::create_dynamic(&get_layout()).add_histogram().write(&data_output);
    }

    fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return // min, max
        2 * Double::BYTES + (ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES + ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + Integer::BYTES + // accumulatedCounts
        (self.accumulated_counts.len() as i64) * Long::BYTES) + (ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES + ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + Integer::BYTES + // nonEmptyBinIndices
        (self.non_empty_bin_indices.len() as i64) * Integer::BYTES) + super.get_estimated_footprint_in_bytes();
    }

    fn is_mutable(&self) -> bool {
        return false;
    }
}

impl AbstractHistogram for PreprocessedHistogram {
}