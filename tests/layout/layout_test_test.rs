// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate dynahist;

// Declare the helper module
mod test;

use test;

pub struct TestLayoutTest {}

impl TestLayoutTest {
    #[test]
    pub fn test_consistency(&self) {
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
                                let layout: Layout =
                                    TestLayout::new(under_flow_bin_index, over_flow_bin_index);
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
