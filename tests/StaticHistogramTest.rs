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

pub struct StaticHistogramTest {
    super: AbstractMutableHistogramTest;
}

impl StaticHistogramTest {

    pub fn  create(&self,  layout: &Layout) -> Histogram  {
        return Histogram::create_static(layout);
    }

    pub fn  read(&self,  layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(Histogram::read_as_static(layout, &data_input));
    }

    #[test]
    pub fn  test_get_estimated_footprint_in_byte(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_static(layout);
        assert_eq!(49772, &histogram.get_estimated_footprint_in_bytes());
    }

    pub fn  test_add_histogram_equal_layout(&self)   {
        super.test_add_histogram_equal_layout();
    }

    #[test]
    pub fn  test_static_histogram_constructor(&self)   {
         let layout: Layout = Layout::new() {

            pub fn  map_to_bin_index(&self,  value: f64) -> usize  {
                return 0;
            }

            pub fn  get_underflow_bin_index(&self) -> usize  {
                return 1;
            }

            pub fn  get_overflow_bin_index(&self) -> usize  {
                return -1;
            }
        };
        assert_throws(IllegalArgumentException.class, () -> Histogram::create_static(layout));
    }
}
