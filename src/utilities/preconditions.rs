// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;

/** Utility trait for preconditions. */
// pub struct Preconditions {
// }

pub trait Preconditions {
    // fn new() -> Preconditions {
    // }

    /// Throws an {@link IllegalArgumentException} if the given expression evaluates to {@code false}.
    ///
    /// @param expression an expression
    /// @throws IllegalArgumentEerror if the given expression evaluates to {@code false}
    ///
    fn check_argument(expression: bool) -> Result<bool, DynaHistError> {
        if !expression {
            let source = "Check argument failed";
            return Err(DynaHistError::IllegalArgumentError { source });
        }
    }

    /// Throws an {@link IllegalArgumentException} if the given expression evaluates to {@code false}.
    ///
    /// @param expression an expression
    /// @param errorMessage an error message
    /// @throws IllegalArgumentException if the given expression evaluates to {@code false}
    ///
    fn check_argument_msg(expression: bool, error_message: &String) -> Result<bool, DynaHistError> {
        if !expression {
            return Err(DynaHistError::IllegalArgumentError { error_message });
        }
    }

    /// Throws an {@link IllegalArgumentException} if the given expression evaluates to {@code false}.
    ///
    /// @param expression an expression
    /// @param errorMessageFormatString an error message format string with a single %s place holder
    /// @param value a long value
    /// @throws IllegalArgumentException if the given expression evaluates to {@code false}
    ///
    fn check_argument_value(
        expression: bool,
        error_message_format_string: &String,
        value: i64,
    ) -> Result<bool, DynaHistError> {
        if !expression {
            let source = error_message_format_string.replace("{}", value.into());
            // "The format functions provided by Rust’s standard library do not
            // have any concept of locale and will produce the same results on
            // all systems regardless of user configuration."
            return Err(DynaHistError::IllegalArgumentError { source });
        }
    }
}
