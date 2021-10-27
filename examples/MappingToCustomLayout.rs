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
// package com::dynatrace::dynahist::demo;

pub struct MappingToCustomLayout {}

impl MappingToCustomLayout {
    #[test]
    pub fn record_values_and_map_to_custom_layout1(&self) {
        let layout: Layout = LogLinearLayout::create(1e-5, 1e-2, 0, 1e6);
        let histogram: Histogram = Histogram::create_dynamic(layout);
        let random: Random = Random::new(0);
        {
            let mut i: i32 = 0;
            while i < 1000000 {
                {
                    histogram.add_value(random.next_double() * i);
                }
                i += 20000;
            }
        }

        let result_layout: Layout = CustomLayout::create(0, 1, 10, 100, 1000, 10000, 1000000);
        let result_histogram: Histogram = Histogram::create_dynamic(result_layout);
        result_histogram.add_histogram(histogram);
        assert_eq!(format!(" 0.00000000000000000E+00 -  9.99999999999999900E-01 : *\n 1.00000000000000000E+03 -  9.99999999999999800E+03 : *****\n 1.00000000000000000E+04 -  7.77237591081370300E+05 : **************************************************\n"), &PrintUtil::pretty_print(result_histogram));
    }

    #[test]
    pub fn record_values_and_map_to_custom_layout2(&self) {
        let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, 0, 1e6);
        let histogram: Histogram = Histogram::create_dynamic(layout);
        let random: Random = Random::new(0);
        {
            let mut i: i32 = 0;
            while i < 1000000 {
                {
                    histogram.add_value(random.next_double() * i);
                }
                i += 1;
            }
        }

        let result_layout: Layout = CustomLayout::create(0, 1, 10, 100, 1000, 10000, 1000000);
        let result_histogram: Histogram = Histogram::create_dynamic(result_layout);
        result_histogram.add_histogram(histogram);
        assert_eq!(format!(" 0.00000000000000000E+00 -  9.99999999999999900E-01 :                  14\n 1.00000000000000000E+00 -  9.99999999999999800E+00 :                 114\n 1.00000000000000000E+01 -  9.99999999999999900E+01 :                 925\n 1.00000000000000000E+02 -  9.99999999999999900E+02 :                6971\n 1.00000000000000000E+03 -  9.99999999999999800E+03 :               47863\n 1.00000000000000000E+04 -  9.98000950924521900E+05 :              944113\n"), &PrintUtil::print(result_histogram));
    }
}
