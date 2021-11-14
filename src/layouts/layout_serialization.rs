// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::layouts::custom_layout::CustomLayout;
use crate::layouts::layout::Layout;
use crate::layouts::Sketch;
use crate::layouts::layout_serialization_definition::LayoutSerializationDefinition;
use crate::layouts::log_linear_layout::LogLinearLayout;
use crate::layouts::log_optimal_layout::LogOptimalLayout;
use crate::layouts::log_quadratic_layout::LogQuadraticLayout;
use crate::layouts::open_telemetry_exponential_buckets_layout::OpenTelemetryExponentialBucketsLayout;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;

pub struct LayoutSerialization<L: Layout> {
    count: usize,
    serial_to_definitions: std::collections::HashMap<i64, LayoutSerializationDefinition<L>>,
    layout_to_definitions: std::collections::HashMap<String, LayoutSerializationDefinition<L>>,
}

impl<L: Layout> Algorithms for LayoutSerialization<L> {}
impl<L: Layout> Preconditions for LayoutSerialization<L> {}

impl<L> LayoutSerialization<L>
where
    L: Layout
{

    fn new(length: usize) -> Self {
        let count = 5;
        let serial_to_definitions: std::collections::HashMap<i64, LayoutSerializationDefinition> =
            std::collections::HashMap::with_capacity(length);
        let layout_to_definitions: std::collections::HashMap<
            String,
            LayoutSerializationDefinition,
        > = std::collections::HashMap::with_capacity(length);

        Self {
            count,
            serial_to_definitions,
            layout_to_definitions,
        }
    }

    fn register_current() -> Self {
        Self::register(&vec![
            &LayoutSerializationDefinition::new(
                0x7f862c3808df6fcd,
                Sketch::Custom
            ),
            &LayoutSerializationDefinition::new(
                0x05d0c7e2dc0316e8,
                Sketch::LogLinear,
            ),
            &LayoutSerializationDefinition::new(
                0x9d36115de11d38d6,
                Sketch::LogQuadratic,
            ),
            &LayoutSerializationDefinition::new(
                0x70c0ef16c3809948,
                Sketch::LogOptimal,
            ),
            &LayoutSerializationDefinition::new(
                0xf6e717a16f0a6a4a,
                Sketch::OpenTelemetryExponentialBuckets,
            ),
        ])
    }

    // Register layout implementations before serialization/deserialization
    fn register(definitions: &Vec<&LayoutSerializationDefinition<L>>) -> Self {
        let seriate = Self::new(definitions.len());

        for definition in definitions {
            Self::check_argument(!Self::RESERVED_SERIAL_VERSIONS::contains(
                definition.serial_version,
            ));
            let old_def1: LayoutSerializationDefinition = seriate
                .serial_to_definitions
                .put(definition.serial_version, definition);
            Self::check_argument(&old_def1.layout == definition.layout);

            let old_def2: LayoutSerializationDefinition = seriate
                .layout_to_definitions
                .put(definition.layout, definition);
            Self::check_argument(old_def2.serial_version == definition.serial_version);
        }
        Self {
            count: definitions.len(),
            serial_to_definitions: seriate.serial_to_definitions.clone(),
            layout_to_definitions: seriate.layout_to_definitions.clone(),
        }
    }

    /// Infallible write to a buffer store (memory).
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOError`] if the serialization layout
    /// definition cannot be found.
    ///
    /// # Panics
    ///
    /// Write operations are infallible.  Hence, if a write fails, something has
    /// gone seriously wrong in memory and the write will panic at this point.
    ///
    fn write(
        layout: impl Layout,
        data_output: impl bytes::BufMut,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        let definition: LayoutSerializationDefinition;
        if layout.layout_to_definitions.contains_key(&layout.histogram_type) {
            definition = layout.layout_to_definitions.get(&layout.histogram_type);
        } else {
            return Err(DynaHistError::IOError( format!(
                "{} has not been registered for serialization!",
                layout.histogram_type
            )));
        }
        data_output.put_i64(definition.serial_version);
        definition.writer.write(layout, &data_output);
    }

    /// Infallible read from a buffer store (memory).
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOError`] if the serialization layout
    /// definition cannot be found.
    ///
    /// # Panics
    ///
    /// Read operations are infallible.  Hence, if a read fails, something has
    /// gone seriously wrong in memory and the read will panic at this point.
    ///
    fn read(data_input: impl bytes::Buf) -> Result<impl Layout, std::rc::Rc<DynaHistError>> {
        let serialization_version: i64 = data_input.get_i64();
        let definition: LayoutSerializationDefinition;
        let layout_seriate = Self::new(5);
        let definition = layout_seriate.serial_to_definitions
            .get(&serialization_version)
            .ok_or(DynaHistError::IOError( anyhow::anyhow!(format!(
                "{} is an unknown layout serialization version!",
                serialization_version
            ))));
        return Ok(definition.unwrap().reader.read(&data_input));
    }
}
