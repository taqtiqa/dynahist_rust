// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct LayoutSerializationTest {
}

impl LayoutSerializationTest {

    #[test]
    pub fn test(&self)  -> Result<Void, Rc<DynaHistError>> {
         let absolute_bin_width_limit: f64 = 0.01;
         let relative_bin_width_limit: f64 = 0.05;
         let value_range_lower_bound: f64 = 10;
         let value_range_upper_bound: f64 = 1000;
         let precision: i32 = 6;
         let log_linear_layout: Layout = LogLinearLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
         let log_quadratic_layout: Layout = LogQuadraticLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
         let log_optimal_layout: Layout = LogOptimalLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
         let otel_exp_bucket_layout: Layout = OpenTelemetryExponentialBucketsLayout::create(precision);
         let custom_layout: Layout = CustomLayout::create(-1, 1, 2, 3);
        // check if layouts are pairwise different
        assert_not_equals(custom_layout, log_linear_layout);
        assert_not_equals(custom_layout, log_quadratic_layout);
        assert_not_equals(custom_layout, log_optimal_layout);
        assert_not_equals(custom_layout, otel_exp_bucket_layout);
        assert_not_equals(log_linear_layout, log_quadratic_layout);
        assert_not_equals(log_linear_layout, log_optimal_layout);
        assert_not_equals(log_linear_layout, otel_exp_bucket_layout);
        assert_not_equals(log_quadratic_layout, log_optimal_layout);
        assert_not_equals(log_quadratic_layout, otel_exp_bucket_layout);
        assert_not_equals(log_optimal_layout, otel_exp_bucket_layout);
         let deserialized_log_linear_layout: Layout = SerializationTestUtil::test_serialization(log_linear_layout, Layout::writeWithTypeInfo, Layout::readWithTypeInfo, "05D0C7E2DC0316E8003F847AE147AE147B3FA999999999999A8C02A404");
         let deserialized_log_quadratic_layout: Layout = SerializationTestUtil::test_serialization(log_quadratic_layout, Layout::writeWithTypeInfo, Layout::readWithTypeInfo, "9D36115DE11D38D6003F847AE147AE147B3FA999999999999AD201A203");
         let deserialized_log_optimal_layout: Layout = SerializationTestUtil::test_serialization(log_optimal_layout, Layout::writeWithTypeInfo, Layout::readWithTypeInfo, "70C0EF16C3809948003F847AE147AE147B3FA999999999999AC6018603");
         let deserialized_otel_exp_bucket_layout: Layout = SerializationTestUtil::test_serialization(otel_exp_bucket_layout, Layout::writeWithTypeInfo, Layout::readWithTypeInfo, "F6E717A16F0A6A4A0006");
         let deserialized_custom_layout: Layout = SerializationTestUtil::test_serialization(custom_layout, Layout::writeWithTypeInfo, Layout::readWithTypeInfo, "7F862C3808DF6FCD0004BFF00000000000003FF000000000000040000000000000004008000000000000");
        assert_eq!(log_linear_layout, deserialized_log_linear_layout);
        assert_eq!(log_quadratic_layout, deserialized_log_quadratic_layout);
        assert_eq!(log_optimal_layout, deserialized_log_optimal_layout);
        assert_eq!(otel_exp_bucket_layout, deserialized_otel_exp_bucket_layout);
        assert_eq!(custom_layout, deserialized_custom_layout);
    }

    #[derive(Layout)]
    struct BaseTestLayout {
    }

    impl BaseTestLayout {

        pub fn map_to_bin_index(&self,  value: f64) -> usize {
            throw UnsupportedOperationException::new();
        }

        pub fn get_underflow_bin_index(&self) -> usize {
            throw UnsupportedOperationException::new();
        }

        pub fn get_overflow_bin_index(&self) -> usize {
            throw UnsupportedOperationException::new();
        }
    }


    #[test]
    pub fn test_layout_registration(&self) {
        struct TestLayout1 {
            super: BaseTestLayout;
        }

        impl TestLayout1 {
        }

        struct TestLayout2 {
            super: BaseTestLayout;
        }

        impl TestLayout2 {
        }

         let serial_version: i64 = 0xfd6be2444812868e;
         let def1: LayoutSerializationDefinition = Layout::define_serialization(serial_version, TestLayout1.class, ( data: &,  data_output: &) -> {
        },  data_input: & -> TestLayout1::new());
        Layout::register(def1);
        // LayoutSerializationDefinition def2 =
        // Layout.defineSerialization(
        //     serialVersion,
        //     TestLayout2.class,
        //     (data, dataOutput) -> {},
        //     dataInput -> new TestLayout2());
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Layout::register(// registration of another serialization using the same serial version must
        def2));
    // fail
    }

    #[test]
    pub fn test_layout_registration_with_reserved_serial_version(&self) {
        struct TestLayout {
            super: BaseTestLayout;
        }

        impl TestLayout {
        }

        // this serial version is among the reserved list
         let serial_version: i64 = 0x3e148a4afd4a0c36;
         let def: LayoutSerializationDefinition = Layout::define_serialization(serial_version, TestLayout.class, ( data: &,  data_output: &) -> {
        },  data_input: & -> TestLayout::new());
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Layout::register(def));
    }

    #[test]
    pub fn test_layout_registration_with_null_argument(&self) {
        LayoutSerialization::register(null);
    }

    #[test]
    pub fn test_layout_registration_with_same_serials_and_different_types(&self) {
        struct TestLayout1 {
            super: BaseTestLayout;
        }

        impl TestLayout1 {
        }

        struct TestLayout2 {
            super: BaseTestLayout;
        }

        impl TestLayout2 {
        }

         let serial_version: i64 = 0xd7937e3e7c687bcd;
         let def1: LayoutSerializationDefinition = Layout::define_serialization(serial_version, TestLayout1.class, ( data: &,  data_output: &) -> {
        },  data_input: & -> TestLayout1::new());
        // LayoutSerializationDefinition def2 =
        // Layout.defineSerialization(
        //     serialVersion,
        //     TestLayout2.class,
        //     (data, dataOutput) -> {},
        //     dataInput -> new TestLayout2());
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Layout::register(def1, def2));
    }

    #[test]
    pub fn test_layout_registration_with_same_serials_and_same_types(&self) {
        struct TestLayout {
            super: BaseTestLayout;
        }

        impl TestLayout {
        }

         let serial_version: i64 = 0x7c5456827a2e71ca;
         let def1: LayoutSerializationDefinition = Layout::define_serialization(serial_version, TestLayout.class, ( data: &,  data_output: &) -> {
        },  data_input: & -> TestLayout::new());
        // LayoutSerializationDefinition def2 =
        // Layout.defineSerialization(
        //     serialVersion,
        //     TestLayout.class,
        //     (data, dataOutput) -> {},
        //     dataInput -> new TestLayout());
        Layout::register(def1, def2);
    }

    #[test]
    pub fn test_layout_registration_with_different_serials_and_same_types(&self) {
        struct TestLayout {
            super: BaseTestLayout;
        }

        impl TestLayout {
        }

         let serial_version1: i64 = 0x794bdbf8691c97ae;
         let serial_version2: i64 = 0x9e1069916e5fd9c9;
         let def1: LayoutSerializationDefinition = Layout::define_serialization(serial_version1, TestLayout.class, ( data: &,  data_output: &) -> {
        },  data_input: & -> TestLayout::new());
        // LayoutSerializationDefinition def2 =
        //     Layout.defineSerialization(
        //         serialVersion2,
        //         TestLayout.class,
        //         (data, dataOutput) -> {},
        //         dataInput -> new TestLayout());
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Layout::register(def1, def2));
    }

    #[test]
    pub fn test_layout_registration_with_different_serials_and_differen_types(&self) {
        struct TestLayout1 {
            super: BaseTestLayout;
        }

        impl TestLayout1 {
        }

        struct TestLayout2 {
            super: BaseTestLayout;
        }

        impl TestLayout2 {
        }

         let serial_version1: i64 = 0x84bc993bcfbfc331;
         let serial_version2: i64 = 0x467ce723a53f6415;
         let def1: LayoutSerializationDefinition = Layout::define_serialization(serial_version1, TestLayout1.class, ( data: &,  data_output: &) -> {
        },  data_input: & -> TestLayout1::new());
        // LayoutSerializationDefinition def2 =
        //     Layout.defineSerialization(
        //         serialVersion2,
        //         TestLayout2.class,
        //         (data, dataOutput) -> {},
        //         dataInput -> new TestLayout2());
        Layout::register(def1, def2);
    }

    #[test]
    pub fn test_write_with_type_info_for_unregistered_layout(&self) {
        struct TestLayout {
            super: BaseTestLayout;
        }

        impl TestLayout {
        }

         let layout: Layout = TestLayout::new();
        assert_throws(IOException.class, () -> SerializationTestUtil::to_byte_array(Layout::writeWithTypeInfo, layout));
    }

    #[test]
    pub fn test_read_with_type_info_for_unregistered_layout(&self) {
         let data: Vec<i8> = SerializationTestUtil::hex_string_to_byte_array("3ECCC0D9B0D7A08B");
        assert_throws(IOException.class, () -> SerializationTestUtil::from_byte_array(Layout::readWithTypeInfo, &data));
    }
}
