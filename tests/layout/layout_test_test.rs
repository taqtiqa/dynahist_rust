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
// package com::dynatrace::dynahist::layout;

extern crate dynahist;

// Declare the helper module
mod test;

use test;

pub struct TestLayoutTest {
}

impl TestLayoutTest {

    #[test]
    pub fn  test_consistency(&self)   {
         let min: i32 = -10;
         let max: i32 = 10;
         {
             let under_flow_bin_index: i32 = min;
            while under_flow_bin_index <= max {
                {
                     {
                         let over_flow_bin_index: i32 = under_flow_bin_index + 1;
                        while over_flow_bin_index <= max {
                            {
                                 let layout: Layout = TestLayout::new(under_flow_bin_index, over_flow_bin_index);
                                LayoutTestUtil::assert_consistency(layout);
                            }
                            over_flow_bin_index += 1;
                         }
                     }

                }
                under_flow_bin_index += 1;
             }
         }

    }
}
