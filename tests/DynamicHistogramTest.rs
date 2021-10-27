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

pub struct DynamicHistogramTest {
    super: AbstractMutableHistogramTest;
}

impl DynamicHistogramTest {

    pub fn  create(&self,  layout: &Layout) -> Histogram  {
        return Histogram::create_dynamic(layout);
    }

    pub fn  read(&self,  layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(Histogram::read_as_dynamic(layout, &data_input));
    }

    pub fn  test_get_estimated_footprint_in_byte(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        assert_eq!(82, &histogram.get_estimated_footprint_in_bytes());
    }

    #[test]
    pub fn  test_ensure_count_array_argument_checks(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: DynamicHistogram = DynamicHistogram::new(layout);
        assert_throws(IllegalArgumentException.class, () -> histogram.ensure_count_array(2, -2, 3 as i8));
        assert_throws(IllegalArgumentException.class, () -> histogram.ensure_count_array(&layout.get_underflow_bin_index(), 0, 3 as i8));
        assert_throws(IllegalArgumentException.class, () -> histogram.ensure_count_array(0, &layout.get_overflow_bin_index(), 3 as i8));
    }
}
