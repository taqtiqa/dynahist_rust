// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/** Represents the serialization definition for some [`Layout`]. */
pub struct LayoutSerializationDefinition {

     let serial_version: i64;

     let clazz: Class<?>;

     let writer: SerializationWriter<Layout>;

     let reader: SerializationReader<Layout>;
}

impl LayoutSerializationDefinition {

    <T extends Layout> fn new( serial_version: i64,  clazz: &Class<T>,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>) -> LayoutSerializationDefinition {
        let .serialVersion = serial_version;
        let .clazz = require_non_null(&clazz);
        let .writer = require_non_null(writer) as SerializationWriter<Layout>;
        let .reader = require_non_null(reader) as SerializationReader<Layout>;
    }
}
