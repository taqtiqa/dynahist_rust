// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/**
 * A serialization writer for a given type.
 *
 * @param <T> the type to be serialized
 */
pub trait SerializationWriter<T> {
    /// Serializes a given object by writing to a given {@link DataOutput}.
    ///
    /// @param data the object to be serialized
    /// @param dataOutput the data output
    /// @throws IOException if an I/O error occurs.
    ///
    fn write(&self, data: &T, data_output: &DataOutput) -> Result<Void, Rc<Exception>>;
}
