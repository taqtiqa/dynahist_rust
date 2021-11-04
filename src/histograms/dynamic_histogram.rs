// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// #[derive(Debug)]
pub struct DynamicHistogram {
    counts: Vec<i64>,
    // use 2^mode bits for counting, mode is in the range {0, 1, 2, 3, 4, 5, 6}
    mode: i8,
    number_of_unused_counts: i8,
    index_offset: i32,
    histogram_type: &str,
}

impl AbstractMutableHistogram for DynamicHistogram {

    fn get_bit_offset( idx: i32,  mode: i8) -> i32 {
        return idx << mode;
    }

    fn get_count_mask( mode: i32) -> i64 {
        return 0xFFFFFFFFFFFFFFFF >> /* >>> */ (0xFFFFFFFF << mode);
    }

    fn get_array_index( idx: i32,  mode: i8) -> i32 {
        return idx >> (6 - mode);
    }

    fn get_count( counts: &Vec<i64>,  relative_idx: i32,  mode: i8) -> i64 {
         let array_idx: i32 = Self::get_array_index(relative_idx, mode);
         let value: i64 = counts[array_idx];
        return (value >> /* >>> */ Self::get_bit_offset(relative_idx, mode)) & Self::get_count_mask(mode);
    }

    fn set_count( counts: &Vec<i64>,  relative_idx: i32,  mode: i8,  new_value: i64) {
        // here newValue must be smaller than (1 << (mode+1))
         let bit_offset: i32 = Self::get_bit_offset(relative_idx, mode);
         let mask: i64 = Self::get_count_mask(mode) << bit_offset;
         let delete_mask: i64 = ~mask;
         let set_mask: i64 = new_value << bit_offset;
         let array_idx: i32 = Self::get_array_index(relative_idx, mode);
        counts[array_idx] = (counts[array_idx] & delete_mask) | set_mask;
    }

    fn get_long_array_size( num_counters: i32,  mode: i8) -> i32 {
        return ((num_counters - 1) >> /* >>> */ (6 - mode)) + 1;
    }

    fn get_num_counters( counts: &Vec<i64>,  number_of_unused_counts: i8,  mode: i8) -> i32 {
        return (counts.len() << (6 - mode)) - number_of_unused_counts;
    }

    fn get_mode(&self) -> i8 {
        return self.mode;
    }

    fn new( layout: impl Layout) -> DynamicHistogram {
        let mode = 0;
        let indexOffset = layout.get_underflow_bin_index() + 1;
        let numberOfUnusedCounts = 0;
        let counts = EMPTY_COUNTS;
    }

    fn read( layout: impl Layout,  data_input: &DataInput) -> Result<DynamicHistogram, std::rc::Rc<DynaHistError>> {


         let histogram: DynamicHistogram = DynamicHistogram::new(layout);
        deserialize(histogram, &data_input);
        return Ok(histogram);
    }

    fn add_values(&self,  value: f64,   count: i64) -> DynamicHistogram {
         let absolute_index: i32 = get_layout().map_to_bin_index(value);
         let relative_index: i32 = absolute_index - self.index_offset;
         let array_idx: i32 = Self::get_array_index(relative_index, self.mode);
         let bit_offset: i32 = Self::get_bit_offset(relative_index, self.mode);
         let mask: i64 = Self::get_count_mask(self.mode);
        if count > 0 {
            if total_count + count >= 0 {
                total_count += count;
                update_min_max(value);
                if array_idx >= 0 && array_idx < self.counts.len() {
                     let old_value: i64 = self.counts[array_idx];
                     let new_count: i64 = ((old_value >> /* >>> */ bit_offset) & mask) + count;
                    self.counts[array_idx] += (count << bit_offset);
                    if (new_count & (~mask)) != 0 {
                        self.counts[array_idx] = old_value;
                        self.try_to_extend_and_increase_count(absolute_index, count, value);
                    }
                } else {
                    self.try_to_extend_and_increase_count(absolute_index, count, value);
                }
            } else {
                return Err(DynaHistError::ArithmeticError(OVERFLOW_MSG));;
            }
        } else if count < 0 {
            return Err(DynaHist::IllegalArgumentError::new(&String::format(null as Locale, NEGATIVE_COUNT_MSG, count)));
        }
        return self;
    }

    fn try_to_extend_and_increase_count(&self,  absolute_index: i32,  count: i64,  value: f64) {
        if !value.is_nan() {
            self.increase_count(absolute_index, count);
        } else {
            total_count -= count;
            return Err(DynaHist::IllegalArgumentError::new(NAN_VALUE_MSG));
        }
    }

    fn increase_count(&self,  absolute_index: i32,  count: i64) {
        if absolute_index <= get_layout().get_underflow_bin_index() {
            increment_underflow_count(count);
        } else if absolute_index >= get_layout().get_overflow_bin_index() {
            increment_overflow_count(count);
        } else {
             let relative_index: i32 = absolute_index - self.index_offset;
             let new_count: i64;
            if relative_index >= 0 && relative_index < Self::get_num_counters(&self.counts, self.number_of_unused_counts, self.mode) {
                new_count = Self::get_count(&self.counts, relative_index, self.mode) + count;
            } else {
                new_count = count;
            }
            self.ensure_count_array(absolute_index, absolute_index, &determine_required_mode(new_count));
            Self::set_count(&self.counts, absolute_index - self.index_offset, self.mode, new_count);
        }
    }

    fn ensure_count_array(&self,  min_absolute_index: i32,  max_absolute_index: i32,  required_mode: i8) {
        Self::check_argument(min_absolute_index <= max_absolute_index);
        Self::check_argument(min_absolute_index > get_layout().get_underflow_bin_index());
        Self::check_argument(max_absolute_index < get_layout().get_overflow_bin_index());
         let new_min_absolute_index: i32;
         let new_max_absolute_index: i32;
         let current_num_counters: i32 = Self::get_num_counters(&self.counts, self.number_of_unused_counts, self.mode);
         let current_min_absolute_index: i32 = self.index_offset;
         let current_max_absolute_index: i32 = self.index_offset + current_num_counters - 1;
         let is_expansion_necessary: bool = false;
        if self.counts.len() > 0 {
            if min_absolute_index < current_min_absolute_index {
                new_min_absolute_index = std::cmp::max(get_layout().get_underflow_bin_index() + 1, &std::cmp::min(min_absolute_index, Math::ceil(current_min_absolute_index - current_num_counters * GROW_FACTOR) as i32));
                is_expansion_necessary = true;
            } else {
                new_min_absolute_index = current_min_absolute_index;
            }
            if max_absolute_index > current_max_absolute_index {
                new_max_absolute_index = std::cmp::min(get_layout().get_overflow_bin_index() - 1, &std::cmp::max(max_absolute_index, Math::ceil(current_max_absolute_index + current_num_counters * GROW_FACTOR) as i32));
                is_expansion_necessary = true;
            } else {
                new_max_absolute_index = current_max_absolute_index;
            }
        } else {
            new_min_absolute_index = min_absolute_index;
            new_max_absolute_index = max_absolute_index;
            is_expansion_necessary = true;
        }
         let new_mode: i8;
        if required_mode > self.mode {
            is_expansion_necessary = true;
            new_mode = required_mode;
        } else {
            new_mode = self.mode;
        }
        if is_expansion_necessary {
             let new_num_counters: i32 = new_max_absolute_index - new_min_absolute_index + 1;
             let new_from: i32 = current_min_absolute_index - new_min_absolute_index;
             let old_counts: Vec<i64> = self.counts;
            self.counts = : [i64; Self::get_long_array_size(new_num_counters, new_mode)] = [0; Self::get_long_array_size(new_num_counters, new_mode)];
            {
                 let mut i: i32 = 0;
                while i < current_num_counters {
                   {
                        Self::set_count(&self.counts, i + new_from, new_mode, &Self::get_count(&old_counts, i, self.mode));
                    }
                    i += 1;
                 }
             }

             let number_of_unused_bits: i32 = (self.counts.len() << 6) - (new_num_counters << new_mode);
            self.counts[self.counts.len() - 1] |= ~(0xffffffffffffffff >> /* >>> */ number_of_unused_bits);
            self.mode = new_mode;
            self.index_offset = new_min_absolute_index;
            self.number_of_unused_counts = (number_of_unused_bits >> self.mode) as i8;
        }
    }

    fn add_histogram_from_estimator(&self,  histogram: impl Histogram,  value_estimator: &ValueEstimator) -> impl Histogram {


        if histogram.is_empty() {
            return self;
        }
        if get_layout().equals(&histogram.get_layout()) {
            total_count += histogram.get_total_count();
            if total_count < 0 {
                total_count -= histogram.get_total_count();
                return Err(DynaHistError::ArithmeticError(OVERFLOW_MSG));;
            }
            updates_min_max(&histogram.get_min(), &histogram.get_max());
            increment_underflow_count(&histogram.get_underflow_count());
            increment_overflow_count(&histogram.get_overflow_count());
            if histogram.get_underflow_count() + histogram.get_overflow_count() < histogram.get_total_count() {
                 let first_bin: BinIterator = histogram.get_first_non_empty_bin();
                 let last_bin: BinIterator = histogram.get_last_non_empty_bin();
                if first_bin.is_underflow_bin() {
                    first_bin.next();
                }
                if last_bin.is_overflow_bin() {
                    last_bin.previous();
                }
               {
                     let desired_mode: i8;
                    if histogram.type == "DynamicHistogram" {
                        desired_mode = std::cmp::max(self.mode, (histogram as DynamicHistogram)::mode) as i8;
                    } else {
                        desired_mode = self.mode;
                    }
                    self.ensure_count_array(&first_bin.get_bin_index(), &last_bin.get_bin_index(), desired_mode);
                }
                 let mut limit: i64 = Self::get_count_mask(self.mode);
                while true {
                     let relative_index: i32 = first_bin.get_bin_index() - self.index_offset;
                     let merged_count: i64 = Self::get_count(&self.counts, relative_index, self.mode) + first_bin.get_bin_count();
                    if merged_count > limit {
                        self.ensure_count_array(&first_bin.get_bin_index(), &first_bin.get_bin_index(), &determine_required_mode(merged_count));
                        limit = Self::get_count_mask(self.mode);
                    }
                    Self::set_count(&self.counts, relative_index, self.mode, merged_count);
                    if first_bin.get_bin_index() == last_bin.get_bin_index() {
                        break;
                    }
                    first_bin.next();
                }
            }
        } else {
            super.add_histogram_from_estimator(histogram, value_estimator);
        }
        return self;
    }

    fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return (ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + (self.counts.len() as i64) * Long::BYTES + ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES + // counts
        Integer::BYTES) + // mode
        Byte::BYTES + // numberOfUnusedCounts
        Byte::BYTES + // indexOffset
        Integer::BYTES + super.get_estimated_footprint_in_bytes();
    }

    fn min_allocated_bin_index_inclusive(&self) -> i32 {
        return self.index_offset;
    }

    fn max_allocated_bin_index_exclusive(&self) -> i32 {
        return self.index_offset + Self::get_num_counters(&self.counts, self.number_of_unused_counts, self.mode);
    }

    fn get_allocated_bin_count(&self,  bin_index: i32) -> i64 {
        return Self::get_count(&self.counts, bin_index - self.index_offset, self.mode);
    }
}
