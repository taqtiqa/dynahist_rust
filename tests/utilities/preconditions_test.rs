// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

const MSG: &'static str = "msg";

const FORMAT_STRING: &'static str = "value = %s";

const VALUE: i64 = 123;

pub struct PreconditionsTest {}

impl Preconditions for PreconditionsTest {
    #[test]
    #[should_panic(expected = "msg")]
    fn test_check_argument_false(&self) {
        Self::check_argument(false).unwrap();
    }

    #[test]
    fn test_check_argument_true(&self) {
        assert!(check_argument(true));
    }

    #[test]
    #[should_panic(expected = "msg")]
    fn test_check_argument_with_message_false(&self) {
        check_argument_msg(false, &MSG).unwrap();
    }

    #[test]
    fn test_check_argument_with_message_and_value_true(&self) {
        assert!(check_argument_value(true, &FORMAT_STRING, VALUE));
    }

    #[test]
    #[should_panic(expected = "value = 123")]
    fn test_check_argument_with_message_and_value_false(&self) {
        check_argument_value(false, &FORMAT_STRING, VALUE).unwrap();
    }

    #[test]
    fn test_check_argument_with_message_true(&self) {
        assert!(check_argument_msg(true, &MSG).unwrap());
    }
}
