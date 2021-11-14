// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::seriate::deserialization::SeriateRead;
use crate::seriate::serialization::SeriateWrite;
use crate::seriate::{deserialization::SerializationReader, serialization::SerializationWriter};
use crate::layouts::layout::Layout;
use crate::utilities::{Algorithms, Preconditions};

// Represents the serialization definition for some [`Layout`].
pub struct LayoutSerializationDefinition<L: Layout>
// where
//     L: Layout
{
    pub serial_version: i64,
    pub layout: String,
    pub writer: SerializationWriter<L>,
    pub reader: SerializationReader<L>,
}
impl<L: Layout> Algorithms for LayoutSerializationDefinition<L> {}
impl<L: Layout> Preconditions for LayoutSerializationDefinition<L> {}
// impl Layout for LayoutSerializationDefinition
//     where
//        L: Layout
// {}

impl<L> LayoutSerializationDefinition<L>
where
    L: Layout
{
    pub fn new( serial_version: i64,  layout: L) -> Self {
        Self {
            serial_version,
            layout: layout.into().to_string(),
            writer: SerializationWriter::new(layout),
            reader: SerializationReader::new(layout),
        }
    }
}
