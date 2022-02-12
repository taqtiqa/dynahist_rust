// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;

/// Utility trait for preconditions.

pub trait Preconditions {
    /// Return [`true`] if the given expression evaluates to [`true`].
    ///
    /// # Errors
    ///
    /// Return [`DynaHist::IllegalArgumentError`] if the given expression
    /// evaluates to [`false`].
    ///
    fn check_argument(expression: bool) -> Result<bool, DynaHistError> {
        if !expression {
            let source = "Check argument failed";
            return Err(DynaHistError::IllegalArgumentError { source });
        }
        Ok(expression)
    }

    /// Return [`true`] if the given expression evaluates to [`true`].
    ///
    /// # Errors
    ///
    /// Return [`DynaHist::IllegalArgumentError`], with the given error message
    /// [`String`], when the given expression evaluates to [`false`].
    /// The error message string is not evaluated by [`format!`], or
    /// altered in any other way.
    ///
    /// # Arguments
    ///
    /// - [`expression`]: An expression
    /// - [`error_message`]: An error message [`String`]
    ///
    fn check_argument_msg(expression: bool, error_message: &String) -> Result<bool, DynaHistError> {
        if !expression {
            return Err(DynaHistError::IllegalArgumentError { error_message });
        }
    }

    /// Return [`true`] if the given expression evaluates to [`true`].
    ///
    /// # Errors
    ///
    /// Return [`DynaHist::IllegalArgumentError`] if the given expression evaluates to [`false`]
    ///
    /// # Port
    ///
    /// Upstream (Java) returns a string that may in future use the system locale.
    /// Currently, Rust [`stdlib`] does not provide this functionality.
    /// If Upstream requires this we welcome any PR implementing
    /// this via existing crates, or any update to the [`stdlib`].
    ///
    /// # Arguments
    ///
    /// - `expression`: An expression
    /// - `error_message_format_string`: An error message format string with a single `{}`%s place holder
    /// - `value`: A [`f64`] value
    ///
    fn check_argument_value(
        expression: bool,
        error_message_format_string: &String,
        value: i64,
    ) -> Result<bool, DynaHistError> {
        if !expression {
            let source = error_message_format_string.replace("{}", value.into());
            // From `stdlib`:
            // "The format functions provided by Rust’s standard library do not
            // have any concept of locale and will produce the same results on
            // all systems regardless of user configuration."
            return Err(DynaHistError::IllegalArgumentError { source });
        }
    }

    /// Prevent registration of layouts with serial versions listed below.
    /// In this way the library can be extended by layouts without potentially
    /// breaking custom layout serializations.
    ///
    const RESERVED_SERIAL_VERSIONS: std::collections::HashSet<u64> = [
        0xee3300c2e8f5499a,
        0x1bfa94ee047b874b,
        0xbe18431aec9dc75a,
        0x223891975b23ed2c,
        0x2526788dcaa5cbc3,
        0xfb14800cbdeab19f,
        0x3da1f941232f78d5,
        0x798f673f19cad268,
        0x7ebd8e2ca5d5e8f8,
        0xe90f4adf283fd8bd,
        0x49315914ea256847,
        0x67a0fdcfcac8adae,
        0xe8c9051d7fec4813,
        0x2966822dd3059653,
        0x771ffd77267e093b,
        0x3ac21d977fd66983,
        0xe072aaeb8a78cd56,
        0x45e4b9a63372e5a0,
        0x45b786df0c91df63,
        0x04717a29a3ae8880,
        0x42de22571ad7c1ab,
        0x76beb97c8c76b6cb,
        0x36bd063c44dd1c47,
        0x1e3413545e2e9aa4,
        0xb2f4232b093c1344,
        0x3ad516c404bb339f,
        0x0d5fdd5ccdb87d2e,
        0x355ef46e1b222b84,
        0x55bcebfcdecdbd6b,
        0x2cf39955769a240f,
        0x344b93535d597e5a,
        0x539b205ae48839b9,
        0x60aa5f359c34974d,
        0x23a8ad0a51093596,
        0x022d7b6a057e7426,
        0x29b10e374caf5524,
        0xd89bb40750f0b639,
        0xcc612183ab823bb0,
        0xe36bb5d2393522ac,
        0x220b749e65ce3926,
        0xf94932966a8ef201,
        0x47554432b5b68636,
        0x971d37ae9e554d95,
        0xa92bce46971262b3,
        0xf50c445c4f5bc615,
        0xe2485f42cef3685a,
        0x3d207553b1b9e7a7,
        0x6a9f5dd769615e93,
        0x3e148a4afd4a0c36,
        0xbad9df1ac314a9c5,
        0x1c1c6feeb0c75786,
        0xccdabd212aed7e33,
        0x139b7acbebfd55ba,
        0xb2480b85e1963a07,
        0x92ef3b1f4fd81c96,
        0x456ddb8c438a95e2,
        0x03c7f1eb167d2577,
        0x03171ab995b64e8b,
        0x1ad67f4b6a032331,
        0x4c0c4551819b67ba,
        0xdc3e981d2b123e5c,
        0x6bcf0e0aa34c2440,
        0xa39cd039cc6aa0db,
        0x6a4d1f5a7ecb9a1e,
        0x6134942188864fe8,
        0xb0762cc0006514b8,
        0x66eaf227554aa3cc,
        0x9c8fec4796a43400,
        0x5ca59771320292fb,
        0x0a14771bfb087051,
        0x938e4e374cc45f38,
        0xcea5420ccc9440d7,
        0xf6c9d75166100f38,
        0x7c74b81edddea9f8,
        0x13840cf38ad88cc7,
        0xa2704e8ff2e1391c,
        0x5ea7252040e5d5db,
        0xb37a491fe6012576,
        0xa26ae9a10288adb8,
        0xf85dd78c4404cc3e,
        0xa5e57bcf4dea2f81,
        0xb0545edeecc62d21,
        0xbc7ff83b839f1468,
        0x37ca3455f1646aaa,
        0x7749ac894823244b,
        0x800b649f63a23cdb,
        0xba73b7a4362c7e17,
        0x24026d1c4bf092be,
        0x9c57278d440abb4c,
        0xdfb7f6f10330d130,
        0x49a507976a675dca,
        0x34d085dbbc54fa43,
        0x981b98182d9c8820,
        0xd2f3247856570daa,
        0xc01e528457e0b09a,
        0xaa8583c5bd37ff70,
        0xac460f81599bfbec,
        0xa1ddc1de18d263bf,
    ]
    .into();
}
