// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::layouts::layout::Layout;
use crate::utilities::data::DataOutput;

/// A serializer for a given histogram layout.
///
/// # Arguments
///
/// - [`L`]: The histogram layout type to be serialized. Available layouts are
///
///     - [`CustomLayout`]
///     - [`LogLinearLayout`]
///     - [`LogOptimalLayout`]
///     - [`LogQuadraticLayout`]
///     - [`OpenTelemetryLayout`]
///
pub struct SerializationWriter {}

impl SeriateWrite for SerializationWriter {}

pub trait SeriateWrite {
    type L: Layout;

    /// Serializes a given object by writing to a given [`DataOutput`].
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOError`] if an I/O error occurs.
    ///
    /// # Arguments
    ///
    /// - `data`: The object to be serialized
    /// - `data_output`: The data output
    ///
    fn write(
        &self,
        data: &Self::L,
        data_output: &DataOutput,
    ) -> Result<(), std::rc::Rc<DynaHistError::IOError>>;
}
