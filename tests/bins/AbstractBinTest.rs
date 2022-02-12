// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct AbstractBinTest {
}

impl AbstractBinTest {

    #[test]
    fn test_to_string(&self) {
         let less_count: i64 = 2343;
         let greater_count: i64 = 42304;
         let bin_count: i64 = 423489324;
         let bin_index: i32 = 434;
         let layout: Layout = TestLayout::new(-5, 7);
         let histogram: Histogram = Histogram::create_dynamic(layout);
         let bin: BinSketch = AbstractBin::new() {

            fn get_less_count(&self) -> i64 {
                return less_count;
            }

            fn get_greater_count(&self) -> i64 {
                return greater_count;
            }

            fn get_bin_index(&self) -> i32 {
                return bin_index;
            }

            fn get_bin_count(&self) -> i64 {
                return bin_count;
            }

            fn get_histogram(&self) -> impl Histogram {
                return histogram;
            }
        };
        assert_eq!("Bin [bin_index=434, lowerBound=Infinity, upperBound=-Infinity, binCount=423489324, lessCount=2343, greaterCount=42304, isUnderflowBin=false, isOverflowBin=false]", &bin.to_string());
    }
}
