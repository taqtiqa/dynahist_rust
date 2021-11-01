// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// A deserializer for a given histogram layout.
///
/// # Arguments
///
/// - [`T`]: The histogram layout type to be deserialized. Available layouts are
///
///     - [`CustomLayout`]
///     - [`LogLinearLayout`]
///     - [`LogOptimalLayout`]
///     - [`LogQuadraticLayout`]
///     - [`OpenTelemetryLayout`]
///
pub trait SerializationReader<T> {
    
    /// Deserializes an object by reading from a given [`DataInput`].
    ///
    /// Implementations should never return {@code null} except for the case {@code null} was really
    /// the serialized value. Returning {@code null} in all other cases like for deprecated
    /// serialization formats is very dangerous, especially if not all serialized bytes are consumed
    /// within the {@link #read} method. If the caller chooses to continue with deserialization from
    /// the same [`DataInput`], wrong data may be deserialized due to the resulting misalignment.
    /// This may lead to severe problems like huge unwanted allocations, if for example the wrong array
    /// length was read first. Consider throwing an {@link IOException} instead, which must be handled
    /// by the caller anyway, and which effectively prevents callers from continuing with
    /// deserialization.
    ///
    /// @param dataInput the data input
    /// @return the deserialized object
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs.
    ///
    fn read(&self, data_input: impl DataInput) -> Result<T, std::rc::Rc<DynaHistError>>;
}
