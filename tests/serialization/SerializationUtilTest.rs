// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct SerializationUtilTest {
}

impl SerializationUtilTest {

    #[test]
    pub fn test_read_unsigned_var_int(&self) {
         let array : vec![i8; 9] = vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, ]
        ;
        assert_throws(IOException.class, () -> SerializationUtil::read_unsigned_var_int(DataInputStream::new(ByteArrayInputStream::new(&array))));
    }

    #[test]
    pub fn test_read_unsigned_var_long(&self) {
         let array : vec![i8; 10] = vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, ]
        ;
        assert_throws(IOException.class, () -> SerializationUtil::read_unsigned_var_long(DataInputStream::new(ByteArrayInputStream::new(&array))));
    }

    #[test]
    pub fn test_serialization(&self)  -> Result<(), std::rc::Rc<DynaHistError>> {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram_dynamic: Histogram = Histogram::create_dynamic(layout);
         let histogram_static: Histogram = Histogram::create_dynamic(layout);
         let histogram_preprocessed: Histogram = Histogram::create_dynamic(layout);
        histogram_dynamic.add_ascending_sequence( i: & -> i + 1, 1000000000);
        histogram_preprocessed.add_ascending_sequence(i -> i + 1, 1000000000);
        histogram_static.add_ascending_sequence(i -> i + 1, 1000000000);
       {
             let serialized_histogram: Vec<i8>;
             let compressed_histogram: Vec<i8>;
             let decompressed_histogram: Histogram;
             let deserialized_histogram: Histogram;
            serialized_histogram = SerializationUtil::write(histogram_dynamic);
            compressed_histogram = SerializationUtil::write_compressed(histogram_dynamic);
            deserialized_histogram = SerializationUtil::read_as_dynamic(layout, &serialized_histogram);
            decompressed_histogram = SerializationUtil::read_compressed_as_dynamic(layout, &compressed_histogram);
            assert_eq!(histogram_dynamic, deserialized_histogram);
            assert_eq!(histogram_dynamic, decompressed_histogram);
            assert_eq!(&histogram_dynamic.hash_code(), &deserialized_histogram.hash_code());
            assert_eq!(&histogram_dynamic.hash_code(), &decompressed_histogram.hash_code());
        }
       {
             let serialized_histogram: Vec<i8>;
             let compressed_histogram: Vec<i8>;
             let decompressed_histogram: Histogram;
             let deserialized_histogram: Histogram;
            serialized_histogram = SerializationUtil::write(histogram_dynamic);
            compressed_histogram = SerializationUtil::write_compressed(histogram_dynamic);
            deserialized_histogram = SerializationUtil::read_as_dynamic(layout, &serialized_histogram);
            decompressed_histogram = SerializationUtil::read_compressed_as_dynamic(layout, &compressed_histogram);
            assert_eq!(histogram_dynamic, deserialized_histogram);
            assert_eq!(histogram_dynamic, decompressed_histogram);
            assert_eq!(&histogram_dynamic.hash_code(), &deserialized_histogram.hash_code());
            assert_eq!(&histogram_dynamic.hash_code(), &decompressed_histogram.hash_code());
        }
       {
             let serialized_histogram: Vec<i8>;
             let compressed_histogram: Vec<i8>;
             let decompressed_histogram: Histogram;
             let deserialized_histogram: Histogram;
            serialized_histogram = SerializationUtil::write(histogram_static);
            compressed_histogram = SerializationUtil::write_compressed(histogram_static);
            deserialized_histogram = SerializationUtil::read_as_static(layout, &serialized_histogram);
            decompressed_histogram = SerializationUtil::read_compressed_as_static(layout, &compressed_histogram);
            assert_eq!(histogram_static, deserialized_histogram);
            assert_eq!(histogram_static, decompressed_histogram);
            assert_eq!(&histogram_static.hash_code(), &deserialized_histogram.hash_code());
            assert_eq!(&histogram_static.hash_code(), &decompressed_histogram.hash_code());
        }
       {
             let serialized_histogram: Vec<i8>;
             let compressed_histogram: Vec<i8>;
             let decompressed_histogram: Histogram;
             let deserialized_histogram: Histogram;
            serialized_histogram = SerializationUtil::write(histogram_preprocessed);
            compressed_histogram = SerializationUtil::write_compressed(histogram_preprocessed);
            deserialized_histogram = SerializationUtil::read_as_preprocessed(layout, &serialized_histogram);
            decompressed_histogram = SerializationUtil::read_compressed_as_preprocessed(layout, &compressed_histogram);
            assert_eq!(histogram_preprocessed, deserialized_histogram);
            assert_eq!(histogram_preprocessed, decompressed_histogram);
            assert_eq!(&histogram_preprocessed.hash_code(), &deserialized_histogram.hash_code());
            assert_eq!(&histogram_preprocessed.hash_code(), &decompressed_histogram.hash_code());
        }
    }

    #[test]
    pub fn test_write_and_write_compressed(&self)  -> Result<(), std::rc::Rc<DynaHistError>> {
         let layout: Layout = LogQuadraticLayout::create(1e-5, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
         let expected_serialized_histogram_hex_string: String = "00393FF00000000000004049000000000000BC0EF413800000000008000000080000100000800040010010010040100808082041082108221108912249249494A528";
         let expected_compressed_histogram_hex_string: String = "789C63B0B4FFC000060E9E107A0FDF17E106108303820518181A181C18190418191C04383838141C391439940439262A79AA4C99B25403001DCD097F";
        histogram.add_ascending_sequence( i: & -> i + 1, 50);
         let serialized_histogram: Vec<i8> = SerializationUtil::write(histogram);
         let compressed_histogram: Vec<i8> = SerializationUtil::write_compressed(histogram);
        assert_eq!(&expected_serialized_histogram_hex_string, &SerializationTestUtil::byte_array_to_hex_string(&serialized_histogram));
        assert_eq!(&expected_compressed_histogram_hex_string, &SerializationTestUtil::byte_array_to_hex_string(&compressed_histogram));
    }

    #[test]
    pub fn test_from_byte_array(&self)  -> Result<(), std::rc::Rc<DynaHistError>> {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
         let serialization_reader: SerializationReader<Histogram> =  data_input: & -> impl Histogram::read_as_dynamic(layout, data_input);
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

        assert_eq!(histogram, &SerializationUtil::from_byte_array(serialization_reader, &serialized_histogram));
    }

    #[test]
    pub fn test_to_byte_array(&self)  -> Result<(), std::rc::Rc<DynaHistError>> {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_dynamic(layout);
         let serialization_writer: SerializationWriter<Histogram> = ( data: &,  data_output: &) -> histogram.write(data_output);
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

        assert_array_equals(&serialized_histogram, &SerializationUtil::to_byte_array(serialization_writer, histogram));
    }
}
