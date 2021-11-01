// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/** Simple example of using DynaHist */
pub struct HistogramUsage {
}

impl HistogramUsage {


   /// The [`Layout`] defines the bins for a [`Histogram`] and maps a given value to a
   /// histogram bin index. {@link LogLinearLayout#create(double, double, double, double)} creates a
   /// [`Layout`] Choose [`LogLinearLayout`], if speed is more important than memory
   /// efficiency. {@link LogQuadraticLayout#create(double, double, double, double)} creates a {@link
   /// Layout} Choose [`LogQuadraticLayout`], if memory efficiency is more important than speed.
   /// LogLinearLayout and LogQuadraticLayout guarantee that the bins cover a given interval and that
   /// the bin widths either satisfy an absolute bin width limit or a relative bin width limit.
   ///
   /// {@link Histogram#createDynamic(Layout)} creates a dynamic [`Histogram`] {@link
   /// Histogram#createStatic(Layout)} creates a static [`Histogram`].
   ///
    #[test]
    pub fn create_histogram(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        assert_eq!(format!("{} [layout={}, underFlowCount=0, overFlowCount=0, totalCount=0, min=Infinity, max=-Infinity, counts={}]", histogram.get_class().get_simple_name(), layout), &histogram.to_string());
    }

 Add values using {@link Histogram#addValue(double)} adds a given value to the histogram. */
    #[test]
    pub fn add_single_value(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        histogram.add_value(-5.5);
        assert_eq!(&PrintUtil::print(histogram), "-5.50000000000000000E+00 - -5.50000000000000000E+00 :                   1\n");
    }


   /// Add values with multiplicity using {@link Histogram#addValue(double, long) } adds a given value
   /// to the histogram with a given multiplicity.
   ///
    #[test]
    pub fn add_value_with_multiplicity(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        histogram.add_value(-5.5, 5);
        assert_eq!(&PrintUtil::print(histogram), "-5.50000000000000000E+00 - -5.50000000000000000E+00 :                   5\n");
    }


   /// Get quantile values using {@link Histogram#getQuantile(double)} returns an estimate for the
   /// quantile value. p = 0.5 returns median.
   ///
    #[test]
    pub fn get_median_single_value(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        histogram.add_value(5.5);
         let median: f64 = histogram.get_quantile(0.5);
        assert_eq!(5.5, median, &std::cmp::max(1e-5, 5.5 * 1e-2));
    }

    #[test]
    pub fn get_median_multiple_values(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        {
             let mut i: i32 = 0;
            while i <= 100 {
               {
                    histogram.add_value(i, 5);
                }
                i += 1;
             }
         }

        assert_eq!(50, &histogram.get_quantile(0.5), &std::cmp::max(1e-5, 50.0 * 1e-2));
    }


   /// Merge histograms using {@link Histogram#addHistogram(Histogram)}. If the given histograms have
   /// a different layout, this operation may lead to an unwanted loss of precision.
   ///
    #[test]
    pub fn merge_histogram(&self) {
         let layout1: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let layout2: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram1: Histogram = Histogram::create_dynamic(layout1);
         let histogram2: Histogram = Histogram::create_dynamic(layout2);
         let histogram_total: Histogram = Histogram::create_dynamic(layout1);
        histogram1.add_value(-55.5);
        histogram1.add_value(100);
        histogram2.add_value(5);
        histogram2.add_value(-7.5);
        histogram_total.add_value(-55.5);
        histogram_total.add_value(100);
        histogram_total.add_value(5);
        histogram_total.add_value(-7.5);
        histogram1.add_histogram(histogram2);
        assert_eq!(histogram_total, histogram1);
        assert_eq!(&histogram_total.hash_code(), &histogram1.hash_code());
    }


   /// Write the histograms to to a given [`DataOutput`] using {@link
   /// Histogram#write(DataOutput)}. The [`Layout`] information will not be written. Therefore, it
   /// is necessary to provide the layout when reading using {@link Histogram#readAsDynamic(Layout,
   /// DataInput)} or {@link Histogram#readAsStatic(Layout, DataInput)}.
   ///
    #[test]
    pub fn serialize_and_deserialize_histogram(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
        histogram.add_value(-5.5);
        // serialization
         let serialized_histogram: Vec<i8> = null;
        let tryResult1 = 0;
        'try1: loop {
        ( let byte_array_output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new();
             let data_output_stream: DataOutputStream = DataOutputStream::new(&byte_array_output_stream)) {
            histogram.write(&data_output_stream);
            serialized_histogram = byte_array_output_stream.to_byte_array();
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &IOException) {
                e.print_stack_trace();
            }  0 => break
        }

        // deserialization
         let deserialized_histogram: Histogram = null;
        let tryResult1 = 0;
        'try1: loop {
        ( let byte_array_input_stream: ByteArrayInputStream = ByteArrayInputStream::new(&serialized_histogram);
             let data_input_stream: DataInputStream = DataInputStream::new(&byte_array_input_stream)) {
            deserialized_histogram = Histogram::read_as_dynamic(layout, &data_input_stream);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &IOException) {
                e.print_stack_trace();
            }  0 => break
        }

        assert_eq!(histogram, deserialized_histogram);
        assert_eq!(&histogram.hash_code(), &deserialized_histogram.hash_code());
    }
}
