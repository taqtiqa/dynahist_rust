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

use crate::errors::DynaHistError;

/** Utility trait for preconditions. */
// pub struct Preconditions {
// }

pub trait Preconditions {

    // fn new() -> Preconditions {
    // }

   /**
   * Throws an {@link IllegalArgumentException} if the given expression evaluates to {@code false}.
   *
   * @param expression an expression
   * @throws IllegalArgumentEerror if the given expression evaluates to {@code false}
   */
    fn check_argument( expression: bool) -> Result<bool, DynaHistError> {
        if !expression {
            let source = "Check argument failed";
            return Err(DynaHistError::IllegalArgumentError { source });
        }
    }

   /**
   * Throws an {@link IllegalArgumentException} if the given expression evaluates to {@code false}.
   *
   * @param expression an expression
   * @param errorMessage an error message
   * @throws IllegalArgumentException if the given expression evaluates to {@code false}
   */
    fn check_argument_msg(expression: bool,  error_message: &String) -> Result<bool, DynaHistError> {
        if !expression {
            return Err( DynaHistError::IllegalArgumentError { error_message } );
        }
    }

    /**
   * Throws an {@link IllegalArgumentException} if the given expression evaluates to {@code false}.
   *
   * @param expression an expression
   * @param errorMessageFormatString an error message format string with a single %s place holder
   * @param value a long value
   * @throws IllegalArgumentException if the given expression evaluates to {@code false}
   */
    fn  check_argument_value( expression: bool,  error_message_format_string: &String,  value: i64) -> Result<bool, DynaHistError> {
        if !expression {
            let source = error_message_format_string.replace("{}", value.into());
            // "The format functions provided by Rust’s standard library do not
            // have any concept of locale and will produce the same results on
            // all systems regardless of user configuration."
            return Err(DynaHistError::IllegalArgumentError { source } );
        }
    }
}
