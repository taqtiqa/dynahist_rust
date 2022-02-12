// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct StaticHistogramTest {}

impl AbstractMutableHistogramTest for StaticHistogramTest {}

impl StaticHistogramTest {

    fn create(&self,  layout: impl Layout) -> impl Histogram {
        return Histogram::create_static(layout);
    }

    pub fn read(&self,  layout: impl Layout,  data_input: &DataInput) -> Result<Histogram, std::rc::Rc<DynaHistError>> {
        return Ok(Histogram::read_as_static(layout, &data_input));
    }

    #[test]
    fn test_get_estimated_footprint_in_byte(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_static(layout);
        assert_eq!(49772, &histogram.get_estimated_footprint_in_bytes());
    }

    fn test_add_histogram_equal_layout(&self) {
        super.test_add_histogram_equal_layout();
    }

    #[test]
    fn test_static_histogram_constructor(&self) {
        let layout: Layout = Layout::new() {

            fn map_to_bin_index(&self,  value: f64) -> usize {
                return 0;
            }

            fn get_underflow_bin_index(&self) -> usize {
                return 1;
            }

            fn get_overflow_bin_index(&self) -> usize {
                return -1;
            }
        };
        assert_throws(DynaHist::IllegalArgumentError.class, () -> impl Histogram::create_static(layout));
    }
}
