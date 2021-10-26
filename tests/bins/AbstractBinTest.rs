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
// package com::dynatrace::dynahist::bin;

pub struct AbstractBinTest {
}

impl AbstractBinTest {

    #[test]
    pub fn  test_to_string(&self)   {
         let less_count: i64 = 2343;
         let greater_count: i64 = 42304;
         let bin_count: i64 = 423489324;
         let bin_index: i32 = 434;
         let layout: Layout = TestLayout::new(-5, 7);
         let histogram: Histogram = Histogram::create_dynamic(layout);
         let bin: Bin = AbstractBin::new() {

            pub fn  get_less_count(&self) -> i64  {
                return less_count;
            }

            pub fn  get_greater_count(&self) -> i64  {
                return greater_count;
            }

            pub fn  get_bin_index(&self) -> i32  {
                return bin_index;
            }

            pub fn  get_bin_count(&self) -> i64  {
                return bin_count;
            }

            pub fn  get_histogram(&self) -> Histogram  {
                return histogram;
            }
        };
        assert_equals("Bin [binIndex=434, lowerBound=Infinity, upperBound=-Infinity, binCount=423489324, lessCount=2343, greaterCount=42304, isUnderflowBin=false, isOverflowBin=false]", &bin.to_string());
    }
}

