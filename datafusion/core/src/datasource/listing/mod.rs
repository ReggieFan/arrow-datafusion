// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! A table that uses the `ObjectStore` listing capability
//! to get the list of files to process.

mod helpers;
mod table;

use datafusion_common::ScalarValue;
use datafusion_data_access::{object_store::local, FileMeta, Result, SizedFile};
use futures::Stream;
use std::pin::Pin;

pub use table::{ListingOptions, ListingTable, ListingTableConfig};

/// Stream of files get listed from object store
pub type PartitionedFileStream =
    Pin<Box<dyn Stream<Item = Result<PartitionedFile>> + Send + Sync + 'static>>;

#[derive(Debug, Clone)]
/// A single file that should be read, along with its schema, statistics
/// and partition column values that need to be appended to each row.
pub struct PartitionedFile {
    /// Path for the file (e.g. URL, filesystem path, etc)
    pub file_meta: FileMeta,
    /// Values of partition columns to be appended to each row
    pub partition_values: Vec<ScalarValue>,
    // We may include row group range here for a more fine-grained parallel execution
}

impl PartitionedFile {
    /// Create a simple file without metadata or partition
    pub fn new(path: String, size: u64) -> Self {
        Self {
            file_meta: FileMeta {
                sized_file: SizedFile { path, size },
                last_modified: None,
            },
            partition_values: vec![],
        }
    }
}

impl std::fmt::Display for PartitionedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.file_meta)
    }
}

/// Helper method to fetch the file size and date at given path and create a `FileMeta`
pub fn local_unpartitioned_file(file: String) -> PartitionedFile {
    PartitionedFile {
        file_meta: local::local_unpartitioned_file(file),
        partition_values: vec![],
    }
}
