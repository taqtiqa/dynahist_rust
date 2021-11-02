// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

struct StaticHistogram {
    counts: Vec<i64>,
}

impl AbstractMutableHistogram for StaticHistogram {

    fn new( layout: impl Layout) -> StaticHistogram {
        let counts_array_size: i32 = layout.get_overflow_bin_index() - layout.get_underflow_bin_index() - 1;
        Self::check_argument(counts_array_size >= 0);
        let counts: [i64; counts_array_size] = [0; counts_array_size];
        self.counts = counts;
    }

    fn add_value(&self,  value: f64,  count: i64) -> impl Histogram {
        if count > 0 {
            if total_count + count >= 0 {
                total_count += count;
                update_min_max(value);
                 let array_idx: i32 = get_layout().map_to_bin_index(value) - get_layout().get_underflow_bin_index() - 1;
                if array_idx >= 0 && array_idx < self.counts.len() {
                    self.counts[array_idx] += count;
                } else {
                    if !value.is_nan() {
                        if array_idx < 0 {
                            increment_underflow_count(count);
                        } else {
                            increment_overflow_count(count);
                        }
                    } else {
                        total_count -= count;
                        return Err(DynaHist::IllegalArgumentError::new(NAN_VALUE_MSG));
                    }
                }
            } else {
                ArithmeticError(OVERFLOW_MSG);
            }
        } else if count < 0 {
            return Err(DynaHist::IllegalArgumentError::new(&String::format(null as Locale, NEGATIVE_COUNT_MSG, count)));
        }
        return self;
    }

    fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return ((self.counts.len() as i64 * Long::BYTES) + ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES + ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + // counts
        Integer::BYTES) + super.get_estimated_footprint_in_bytes();
    }

    fn min_allocated_bin_index_inclusive(&self) -> i32 {
        return get_layout().get_underflow_bin_index() + 1;
    }

    fn max_allocated_bin_index_exclusive(&self) -> i32 {
        return get_layout().get_overflow_bin_index();
    }

    fn get_allocated_bin_count(&self,  bin_index: i32) -> i64 {
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

        return determine_required_mode(c);
    }

    fn ensure_count_array(&self,  min_non_empty_bin_index: i32,  max_non_empty_bin_index: i32,  mode: i8) {
    // not necessary because of static allocation
    }

    fn increase_count(&self,  absolute_index: i32,  count: i64) {
        if absolute_index <= get_layout().get_underflow_bin_index() {
            increment_underflow_count(count);
        } else if absolute_index >= get_layout().get_overflow_bin_index() {
            increment_overflow_count(count);
        } else {
            self.counts[absolute_index - get_layout().get_underflow_bin_index() - 1] += count;
        }
    }

    fn read( layout: impl Layout,  data_input: &DataInput) -> Result<StaticHistogram, std::rc::Rc<DynaHistError>> {
        require_non_null(layout);
        require_non_null(&data_input);
         let histogram: StaticHistogram = StaticHistogram::new(layout);
        deserialize(histogram, &data_input);
        return Ok(histogram);
    }
}
