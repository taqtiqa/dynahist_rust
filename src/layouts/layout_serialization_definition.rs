// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::seriate::{deserialization::SerializationReader, serialization::SerializationWriter};
use crate::layouts::layout::Layout;
use crate::utilities::{Algorithms, Preconditions};

// Represents the serialization definition for some [`Layout`].
pub struct LayoutSerializationDefinition
// where
//     L: Layout
{
    serial_version: i64,
    layout: String,
    writer: SerializationWriter,
    reader: SerializationReader,
}
impl Algorithms for LayoutSerializationDefinition {}
impl Preconditions for LayoutSerializationDefinition {}
// impl Layout for LayoutSerializationDefinition
//     where
//        L: Layout
// {}

impl LayoutSerializationDefinition
// where
//     L: Layout
{
    pub fn new( serial_version: i64,  layout: String,  writer: &SerializationWriter,  reader: &SerializationReader) -> Self {
        Self {
            serial_version,
            layout,
            writer: *writer,
            reader: *reader,
        }
    }
}
