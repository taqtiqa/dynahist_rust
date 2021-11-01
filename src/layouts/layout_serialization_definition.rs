// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::seriate::{deserialization::SerializationReader, serialization::SerializationWriter};
use crate::layouts::layout::Layout;

// Represents the serialization definition for some [`Layout`].
pub struct LayoutSerializationDefinition {
    serial_version: i64,
    clazz: T: Layout,
    writer: SerializationWriter<T>,
    reader: SerializationReader<T>,
}

impl Layout for LayoutSerializationDefinition {
    fn new( serial_version: i64,  clazz: &T,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>) -> LayoutSerializationDefinition {
        serial_version;
        clazz;
        writer as SerializationWriter<T>;
        reader as SerializationReader<T>;
    }
}
