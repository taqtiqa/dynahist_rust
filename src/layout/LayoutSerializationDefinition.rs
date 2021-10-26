/*
 * Copyright 2020-2021 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::dynatrace::dynahist::layout;

/** Represents the serialization definition for some {@link Layout}. */
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

