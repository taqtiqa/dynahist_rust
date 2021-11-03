// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::seriate::{deserialization::SerializationReader, serialization::SerializationWriter};
use crate::layouts::layout::Layout;

// Represents the serialization definition for some [`Layout`].
pub struct LayoutSerializationDefinition<L>
where
    L: Layout
{
    serial_version: i64,
    layout: &str,
    writer: SerializationWriter<L>,
    reader: SerializationReader<L>,
}
impl<L> Layout for LayoutSerializationDefinition<L>
where
    L: Layout
{}

impl<L> LayoutSerializationDefinition<L>
where
    L: Layout
{
    fn new( serial_version: i64,  layout: &L,  writer: &SerializationWriter<L>,  reader: &SerializationReader<L>) -> LayoutSerializationDefinition {
        serial_version;
        layout;
        writer as SerializationWriter<L>;
        reader as SerializationReader<L>;
    }
}
