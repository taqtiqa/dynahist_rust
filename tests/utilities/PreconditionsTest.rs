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

impl PreconditionsTest {

    #[test]
    pub fn  test_check_argument_false(&self)   {
        assert_throws(IllegalArgumentException.class, () -> Preconditions::check_argument(false));
    }

    #[test]
    pub fn  test_check_argument_true(&self)   {
        Preconditions::check_argument(true);
    }

    #[test]
    pub fn  test_check_argument_with_message_false(&self)   {
         let e: IllegalArgumentException = assert_throws(IllegalArgumentException.class, () -> Preconditions::check_argument(false, &MSG));
        assert_equals(&MSG, &e.get_message());
    }

    #[test]
    pub fn  test_check_argument_with_message_and_value_true(&self)   {
        Preconditions::check_argument(true, &FORMAT_STRING, VALUE);
    }

    #[test]
    pub fn  test_check_argument_with_message_and_value_false(&self)   {
         let e: IllegalArgumentException = assert_throws(IllegalArgumentException.class, () -> Preconditions::check_argument(false, &FORMAT_STRING, VALUE));
        assert_equals(&String::format(null as Locale, &FORMAT_STRING, VALUE), &e.get_message());
    }

    #[test]
    pub fn  test_check_argument_with_message_true(&self)   {
        Preconditions::check_argument(true, &MSG);
    }
}

