// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::layouts::layout::Layout;
use crate::layouts::Sketch;
use crate::seriate::deserialization::SeriateRead;
use crate::seriate::serialization::SeriateWrite;
use crate::seriate::{deserialization::SerializationReader, serialization::SerializationWriter};
use crate::utilities::{Algorithms, Preconditions};

// Represents the serialization definition for some [`Layout`].
pub struct LayoutSerializationDefinition
// where
//     L: Layout
{
    pub serial_version: i64,
    pub layout: String,
    pub writer: SerializationWriter,
    pub reader: SerializationReader,
}
impl Algorithms for LayoutSerializationDefinition {}
impl Preconditions for LayoutSerializationDefinition {}
// impl Layout for LayoutSerializationDefinition
//     where
//        L: Layout
// {}

impl LayoutSerializationDefinition {
    pub fn new(serial_version: i64, layout: Sketch) -> Self {
        let desc = match layout {
            Sketch::Custom => "Custom".to_string(),
            Sketch::LogOptimal => "LogOptimal".to_string(),
            Sketch::LogLinear => "LogLinear".to_string(),
            Sketch::LogQuadratic => "LogQuadratic".to_string(),
            Sketch::OpenTelemetryExponentialBuckets => {
                "OpenTelemetryExponentialBuckets".to_string()
            }
        };
        Self {
            serial_version,
            layout: desc,
            writer: SerializationWriter::new(layout),
            reader: SerializationReader::new(layout),
        }
    }
}
