// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::layouts::layout::Layout;
use crate::utilities::data::DataInput;

/// A deserializer for a given histogram layout.
///
/// # Arguments
///
/// - [`L`]: The histogram layout type to be deserialized. Available layouts are
///
///     - [`CustomLayout`]
///     - [`LogLinearLayout`]
///     - [`LogOptimalLayout`]
///     - [`LogQuadraticLayout`]
///     - [`OpenTelemetryLayout`]
///
pub struct SerializationReader {}

impl SeriateRead for SerializationReader {}

pub trait SeriateRead {
    type L: Layout;

    /// Return a deserialized histogram by reading from a given [`DataInput`].
    ///
    /// We return a [`DynaHistError::IOError`] instead, which must be handled
    /// by the caller, and effectively prevents callers from continuing with
    /// deserialization.
    ///
    /// # Port
    ///
    /// Upstream (Java) suggests:
    ///
    /// > "Implementations should never return `null` except for the case
    /// `null` was really the serialized value."
    ///
    /// However, Rust does not contain a `null` type. Rather the Rust port
    /// never serializes `None`, `()` nor any `&str` or `String`.
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOError`] if an I/O error occurs.
    ///
    /// # Arguments
    ///
    /// - [`data_input`] the data input
    ///
    fn read(&self, data_input: &DataInput) -> Result<Self::L, std::rc::Rc<DynaHistError>>;
}
