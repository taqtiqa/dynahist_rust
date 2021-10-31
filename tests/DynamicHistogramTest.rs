// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct DynamicHistogramTest {
    super: AbstractMutableHistogramTest;
}

impl DynamicHistogramTest {

    pub fn create(&self,  layout: impl Layout) -> impl Histogram  {
        return Histogram::create_dynamic(layout);
    }

    pub fn read(&self,  layout: impl Layout,  data_input: impl DataInput) -> Result<Histogram, Rc<DynaHistError>> {
        return Ok(Histogram::read_as_dynamic(layout, &data_input));
    }

    pub fn test_get_estimated_footprint_in_byte(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        assert_eq!(82, &histogram.get_estimated_footprint_in_bytes());
    }

    #[test]
    pub fn test_ensure_count_array_argument_checks(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: DynamicHistogram = DynamicHistogram::new(layout);
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.ensure_count_array(2, -2, 3 as i8));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.ensure_count_array(&layout.get_underflow_bin_index(), 0, 3 as i8));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.ensure_count_array(0, &layout.get_overflow_bin_index(), 3 as i8));
    }
}
