// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

struct StaticHistogram {
    super: AbstractMutableHistogram;

     let mut counts: Vec<i64>;
}

impl StaticHistogram {

    fn new( layout: impl Layout) -> StaticHistogram {
        super(layout);
         let counts_array_size: i32 = layout.get_overflow_bin_index() - layout.get_underflow_bin_index() - 1;
        check_argument(counts_array_size >= 0);
        let .counts = : [i64; counts_array_size] = [0; counts_array_size];
    }

    pub fn add_value(&self,  value: f64,  count: i64) -> impl Histogram {
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
                throw ArithmeticException::new(OVERFLOW_MSG);
            }
        } else if count < 0 {
            return Err(DynaHist::IllegalArgumentError::new(&String::format(null as Locale, NEGATIVE_COUNT_MSG, count)));
        }
        return self;
    }

    pub fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return ((self.counts.len() as i64 * Long::BYTES) + ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES + ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + // counts
        Integer::BYTES) + super.get_estimated_footprint_in_bytes();
    }

    pub fn min_allocated_bin_index_inclusive(&self) -> i32 {
        return get_layout().get_underflow_bin_index() + 1;
    }

    pub fn max_allocated_bin_index_exclusive(&self) -> i32 {
        return get_layout().get_overflow_bin_index();
    }

    pub fn get_allocated_bin_count(&self,  bin_index: i32) -> i64 {
        return self.counts[bin_index - self.min_allocated_bin_index_inclusive()];
    }

    pub fn get_mode(&self) -> i8 {
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

    pub fn ensure_count_array(&self,  min_non_empty_bin_index: i32,  max_non_empty_bin_index: i32,  mode: i8) {
    // not necessary because of static allocation
    }

    pub fn increase_count(&self,  absolute_index: i32,  count: i64) {
        if absolute_index <= get_layout().get_underflow_bin_index() {
            increment_underflow_count(count);
        } else if absolute_index >= get_layout().get_overflow_bin_index() {
            increment_overflow_count(count);
        } else {
            self.counts[absolute_index - get_layout().get_underflow_bin_index() - 1] += count;
        }
    }

    pub fn read( layout: impl Layout,  data_input: impl DataInput) -> Result<StaticHistogram, Rc<DynaHistError>> {
        require_non_null(layout);
        require_non_null(&data_input);
         let histogram: StaticHistogram = StaticHistogram::new(layout);
        deserialize(histogram, &data_input);
        return Ok(histogram);
    }
}
