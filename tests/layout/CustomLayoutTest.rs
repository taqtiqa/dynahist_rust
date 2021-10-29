// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct CustomLayoutTest {
}

impl CustomLayoutTest {

    #[test]
    pub fn test_consistency(&self)   {
        LayoutTestUtil::assert_consistency(&CustomLayout::create(f64::INFINITY));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(-1, 1));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(-1, 0, 1));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(-0.0, 0.0));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(&Math::next_up(f64::NEG_INFINITY)));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(&Math::next_up(f64::NEG_INFINITY)));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(&Math::next_up(f64::NEG_INFINITY), f64::INFINITY));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(-3, -1.5, 234, 4324234));
        LayoutTestUtil::assert_consistency(&CustomLayout::create(-34234, -3, -1.5, 234, 4324234));
    }

    #[test]
    pub fn test_serialization(&self)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
         let layout: CustomLayout = CustomLayout::create(-3, -1.5, 234, 4324234);
         let deserialized_layout: CustomLayout = SerializationTestUtil::test_serialization(layout, CustomLayout::write, CustomLayout::read, "0004C008000000000000BFF8000000000000406D40000000000041507EE280000000");
        assert_eq!(deserialized_layout, layout);
    }

    #[test]
    pub fn test_hash_code(&self)   {
         let layout: CustomLayout = CustomLayout::create(-3, -1.5, 234, 4324234);
        assert_eq!(327767682, &layout.hash_code());
    }

    #[test]
    pub fn test_to_string(&self)   {
         let layout: Layout = CustomLayout::create(-3, -1.5, 234, 4324234);
        assert_eq!("CustomLayout [sortedBinBoundaries=[-3.0, -1.5, 234.0, 4324234.0]]", &layout.to_string());
    }

    #[test]
    pub fn test_equals(&self)   {
         let layout: Layout = CustomLayout::create(-3, -1.5, 234, 4324234);
        assert_false(&layout.equals(null));
        assert_eq!(layout, layout);
        assert_not_equals(layout, &LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6));
        assert_not_equals(layout, &CustomLayout::create(-3, -1.5, 234, 353, 4324234));
    }

    #[test]
    pub fn test_create(&self)   {
        assert_throws(IllegalArgumentException.class, CustomLayout::create);
        assert_throws(IllegalArgumentException.class, () -> CustomLayout::create(f64::NEG_INFINITY));
        assert_throws(IllegalArgumentException.class, () -> CustomLayout::create(1, 0));
    }
}
