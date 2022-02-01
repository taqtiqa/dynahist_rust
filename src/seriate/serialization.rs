// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::layouts::layout::Layout;
use crate::layouts::Sketch;
use crate::sketches::data::DataOutput;

/// A serializer for a given histogram layout.
///
/// # Arguments
///
/// - [`L`]: The histogram sketch layout type to be serialized. Available
///   layouts are
///
///     - [`Custom`]
///     - [`LogLinear`]
///     - [`LogOptimal`]
///     - [`LogQuadratic`]
///     - [`OpenTelemetry`]
///
pub struct SerializationWriter {
    layout: Sketch,
}

impl SerializationWriter {

    fn new(layout: Sketch) -> Self {
       Self{layout}
    }
}

pub trait SeriateWrite {
    type L: Layout;

    // fn new(layout: Sketch) -> Self;

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
    ) -> Result<(), std::rc::Rc<DynaHistError>>;
}
