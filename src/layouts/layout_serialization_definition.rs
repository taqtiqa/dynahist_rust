// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Represents the serialization definition for some [`Layout`].
pub struct LayoutSerializationDefinition {
    serial_version: i64,
    clazz: T,
    writer: SerializationWriter<Layout>,
    reader: SerializationReader<Layout>,
}

impl Layout for LayoutSerializationDefinition {
    fn new( serial_version: i64,  clazz: &Class<T>,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>) -> LayoutSerializationDefinition {
        serial_version;
        clazz = require_non_null(&clazz);
        writer = require_non_null(writer) as SerializationWriter<Layout>;
        reader = require_non_null(reader) as SerializationReader<Layout>;
    }
}
