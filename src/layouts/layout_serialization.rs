// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::layouts::custom_layout::CustomLayout;
use crate::layouts::layout::Layout;
use crate::layouts::layout_serialization_definition::LayoutSerializationDefinition;
use crate::layouts::log_linear_layout::LogLinearLayout;
use crate::layouts::log_optimal_layout::LogOptimalLayout;
use crate::layouts::log_quadratic_layout::LogQuadraticLayout;
use crate::layouts::open_telemetry_exponential_buckets_layout::OpenTelemetryExponentialBucketsLayout;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;

struct LayoutSerialization {
    serial_to_definitions: std::collections::BTreeMap<u64, LayoutSerializationDefinition>,
    class_to_definitions: std::collections::BTreeMap<String, LayoutSerializationDefinition>,
}

impl Preconditions for LayoutSerialization {}

impl Algorithms for LayoutSerialization {}

impl LayoutSerialization {
    fn new() -> LayoutSerialization {
        let serial_to_definitions: std::collections::BTreeMap<u64, LayoutSerializationDefinition> =
            std::collections::btree_map::BTreeMap::new();

        let class_to_definitions: std::collections::BTreeMap<
            String,
            LayoutSerializationDefinition,
        > = std::collections::btree_map::BTreeMap::new();

        // Return `LayoutSerialization` with populated fields
        Self::register(vec![
            LayoutSerializationDefinition::new(
                0x7f862c3808df6fcd,
                std::any::type_name::<CustomLayout>().to_string(),
                CustomLayout::write,
                CustomLayout::read,
            ),
            LayoutSerializationDefinition::new(
                0x05d0c7e2dc0316e8,
                std::any::type_name::<LogLinearLayout>().to_string(),
                LogLinearLayout::write,
                LogLinearLayout::read,
            ),
            LayoutSerializationDefinition::new(
                0x9d36115de11d38d6,
                std::any::type_name::<LogQuadraticLayout>().to_string(),
                LogQuadraticLayout::write,
                LogQuadraticLayout::read,
            ),
            LayoutSerializationDefinition::new(
                0x70c0ef16c3809948,
                std::any::type_name::<LogOptimalLayout>().to_string(),
                LogOptimalLayout::write,
                LogOptimalLayout::read,
            ),
            LayoutSerializationDefinition::new(
                0xf6e717a16f0a6a4a,
                std::any::type_name::<OpenTelemetryExponentialBucketsLayout>().to_string(),
                OpenTelemetryExponentialBucketsLayout::write,
                OpenTelemetryExponentialBucketsLayout::read,
            ),
        ])
    }

    // new layout implementations must be registered before
    // serialization/deserialization
    fn register(definitions: &Vec<LayoutSerializationDefinition>) {
        let new_serial_to_definitions = std::collections::HashMap::with_capacity(
            Self.serial_to_definitions.size() + definitions.len(),
        );

        let new_class_to_definitions = std::collections::HashMap::with_capacity(
            Self.class_to_definitions.size() + definitions.len(),
        );

        new_serial_to_definitions.put_all(&Self.serial_to_definitions);
        new_class_to_definitions.put_all(&Self.class_to_definitions);

        for definition in definitions {
            Self::check_argument(!Self::RESERVED_SERIAL_VERSIONS::contains(
                definition.serialVersion,
            ));
            let old_def1: LayoutSerializationDefinition =
                new_serial_to_definitions.put(definition.serialVersion, definition);
            Self::check_argument(&old_def1.layout == definition.layout);

            let old_def2: LayoutSerializationDefinition =
                new_class_to_definitions.put(definition.layout, definition);
            Self::check_argument(old_def2.serial_version == definition.serial_version);
        }
        LayoutSerialization {
            serial_to_definitions: std::collections::HashMap::new(&new_serial_to_definitions),
            class_to_definitions: std::collections::HashMap::new(&new_class_to_definitions),
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
        if Self.class_to_definitions.contains(&layout.histogram_type) {
            definition = Self.class_to_definitions.get(&layout.histogram_type);
        } else {
            return Err(DynaHistError::IOError.context(format!(
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
        let serialization_version: i64 = data_input.read_i64();
        let definition: LayoutSerializationDefinition;
        if Self.class_to_definitions.contains(&serialization_version) {
            definition = Self.serial_to_definitions.get(&serialization_version);
        } else {
            return Err(DynaHistError::IOError.context(format!(
                "{} is an unknown layout serialization version!",
                serialization_version
            )));
        }
        return Ok(definition.reader.read(&data_input));
    }
}
