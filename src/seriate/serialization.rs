// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;

/// A serialization writer for a given type `T`, the type to be serialized.
///
pub trait SerializationWriter<T> {

    /// Serializes a given object by writing to a given [`DataOutput`].
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOException`] if an I/O error occurs.
    ///
    /// # Arguments
    ///
    /// - `data`: The object to be serialized
    /// - `data_output`: The data output
    ///
    fn write(&self, data: &T, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError::IOException>>;
}
