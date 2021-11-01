// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct PreprocessedHistogramTest {
    super: AbstractHistogramTest;
}

impl PreprocessedHistogramTest {

    pub fn create(&self,  layout: impl Layout) -> impl Histogram {
        return Histogram::create_dynamic(layout)::get_preprocessed_copy();
    }

    pub fn read(&self,  layout: impl Layout,  data_input: impl DataInput) -> Result<Histogram, Rc<DynaHistError>> {
        return Ok(Histogram::read_as_preprocessed(layout, &data_input));
    }

    pub fn add_values(&self,  histogram: impl Histogram,  values: f64) -> impl Histogram {
        if values == null {
            return histogram;
        }

         let mutable_histogram: Histogram = Histogram::create_static(&histogram.get_layout());
        mutable_histogram.add_histogram(histogram);
        for  let x: f64 in values {
            mutable_histogram.add_value(x);
        }
        return mutable_histogram.get_preprocessed_copy();
    }

    #[test]
    pub fn test_get_estimated_footprint_in_byte(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let preprocessed_histogram: Histogram = Histogram::create_dynamic(layout)::get_preprocessed_copy();
        assert_eq!(72, &preprocessed_histogram.get_estimated_footprint_in_bytes());
    }

    #[test]
    pub fn test_exceptions(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        histogram.add_value(-5.5);
         let preprocessed_histogram: Histogram = histogram.get_preprocessed_copy();
        histogram.add_value(-4.4);
         let iterator: BinIterator = preprocessed_histogram.get_first_non_empty_bin();
        assert_throws(UnsupportedOperationException.class, () -> preprocessed_histogram.add_value(-5.5));
        assert_throws(UnsupportedOperationException.class, () -> preprocessed_histogram.add_value(-5.5, 5));
        assert_throws(UnsupportedOperationException.class, () -> preprocessed_histogram.add_histogram(histogram));
        assert_throws(UnsupportedOperationException.class, () -> preprocessed_histogram.add_ascending_sequence( j: & -> 100, 10));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> preprocessed_histogram.get_bin_by_rank(-1));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> preprocessed_histogram.get_bin_by_rank(1));
    }

    #[test]
    pub fn test_read_as_preprocessed(&self)  -> Result<Void, Rc<DynaHistError>> {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        histogram.add_value(-5.5);
         let byte_array_output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new();
         let data_output_stream: DataOutputStream = DataOutputStream::new(&byte_array_output_stream);
        histogram.write(&data_output_stream);
         let serialized_histogram: Vec<i8> = byte_array_output_stream.to_byte_array();
         let data_input_stream: DataInputStream = DataInputStream::new(ByteArrayInputStream::new(&serialized_histogram));
         let deserialized_histogram: Histogram = Histogram::read_as_preprocessed(layout, &data_input_stream);
        assert_eq!(histogram, deserialized_histogram);
        assert_eq!(&histogram.hash_code(), &deserialized_histogram.hash_code());
    }

    #[test]
    pub fn test_is_mutable(&self) {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout)::get_preprocessed_copy();
        assert_false(&histogram.is_mutable());
    }
}
