/*
 * Copyright 2020-2021 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::dynatrace::dynahist::util;


const MSG: &'static str = "msg";

const FORMAT_STRING: &'static str = "value = %s";

const VALUE: i64 = 123;

pub struct PreconditionsTest {
}

impl Preconditions for PreconditionsTest {

    #[test]
    #[should_panic(expected = "msg")]
    fn  test_check_argument_false(&self)   {
        check_argument(false).unwrap();
    }

    #[test]
    fn test_check_argument_true(&self)   {
        assert!(check_argument(true));
    }

    #[test]
    #[should_panic(expected = "msg")]
    fn test_check_argument_with_message_false(&self)   {
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
