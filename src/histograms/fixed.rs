// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// use crate::bins::bin::BinSketch;
// use crate::bins::bin_iterator::BinIterator;
use crate::errors::DynaHistError;
// use crate::histograms::dynamic_histogram::DynamicHistogram;
use crate::histograms::abstract_histogram::AbstractHistogram;
use crate::histograms::abstract_histogram::Probability;
use crate::histograms::abstract_mutable_histogram::AbstractMutableHistogram;
use crate::histograms::histogram::Histogram;
use crate::layouts::layout::Layout;
use crate::quantiles::quantile_estimation::QuantileEstimation;
use crate::sketches::data::DataInput;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;

pub struct StaticHistogram {
    counts: Vec<i64>,
}

impl StaticHistogram {
    fn read(
        layout: impl Layout,
        data_input: &DataInput,
    ) -> Result<Self, std::rc::Rc<DynaHistError>> {
        let histogram: Self = Self::new(layout);
        Self::deserialize(histogram, &data_input);
        return Ok(histogram);
    }
}

impl Algorithms for StaticHistogram {}
impl Preconditions for StaticHistogram {}

impl Histogram for StaticHistogram {
    fn add_values(&self, value: f64, count: i64) -> Self {
        if count > 0 {
            if self.total_count + count >= 0 {
                self.total_count += count;
                Self::update_min_max(value);
                let array_idx: i32 = Self::get_layout().map_to_bin_index(value)
                    - Self::get_layout().get_underflow_bin_index()
                    - 1;
                if array_idx >= 0 && array_idx < self.counts.len() {
                    self.counts[array_idx] += count;
                } else if !value.is_nan() {
                    if array_idx < 0 {
                        Self::increment_underflow_count(count);
                    } else {
                        Self::increment_overflow_count(count);
                    }
                } else {
                    self.total_count -= count;
                    return Err(DynaHistError::IllegalArgumentError {
                        source: Self::NAN_VALUE_MSG,
                    });
                }
            } else {
                return Err(DynaHistError::ArithmeticError(Self::OVERFLOW_MSG));
            }
        } else if count < 0 {
            let source = format!("Count must be non-negative, but was {}!", count,);
            return Err(DynaHistError::IllegalArgumentError { source });
        }
        return self;
    }
}

impl AbstractHistogram for StaticHistogram {}
impl Probability for StaticHistogram {}

impl AbstractMutableHistogram for StaticHistogram {
    fn new(layout: impl Layout) -> Self {
        let counts_array_size: usize =
            layout.get_overflow_bin_index() - layout.get_underflow_bin_index() - 1;
        Self::check_argument(counts_array_size >= 0);
        let counts = vec![0; counts_array_size];
        Self { counts }
    }

    fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return ((self.counts.len() as i64 * i64::BYTES) + Self::ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES + Self::ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + // counts
        i32::BYTES)
            + self.get_estimated_footprint_in_bytes();
    }

    fn min_allocated_bin_index_inclusive(&self) -> i32 {
        return self.get_layout().get_underflow_bin_index() + 1;
    }

    fn max_allocated_bin_index_exclusive(&self) -> i32 {
        return self.get_layout().get_overflow_bin_index();
    }

    fn get_allocated_bin_count(&self, bin_index: i32) -> i64 {
        return self.counts[bin_index - self.min_allocated_bin_index_inclusive()];
    }

    fn get_mode(&self) -> i8 {
        let mut c: i64 = 0;
        {
            let mut i: i32 = 0;
            while i < self.counts.len() {
                {
                    c |= self.counts[i];
                }
                i += 1;
            }
        }

        return self.determine_required_mode(c);
    }

    fn ensure_count_array(
        &self,
        min_non_empty_bin_index: i32,
        max_non_empty_bin_index: i32,
        mode: i8,
    ) {
        // not necessary because of static allocation
    }

    fn increase_count(&self, absolute_index: i32, count: i64) {
        if absolute_index <= self.get_layout().get_underflow_bin_index() {
            self.increment_underflow_count(count);
        } else if absolute_index >= self.get_layout().get_overflow_bin_index() {
            self.increment_overflow_count(count);
        } else {
            self.counts[absolute_index - self.get_layout().get_underflow_bin_index() - 1] += count;
        }
    }
}
