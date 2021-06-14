// SPDX-License-Identifier: MIT
//
// Linux system call names and numbers.
// Copyright (c) 2021  William Findlay
//
// June 14, 2021  William Findlay  Created this.

use std::collections::HashMap;

use csv::ReaderBuilder;
use glob::glob;

pub fn main() {
    let syscalls = ingest();
    eprintln!("{:#?}", syscalls);
}

fn ingest() -> HashMap<String, HashMap<String, u64>> {
    let mut map: HashMap<String, HashMap<String, u64>> = HashMap::new();

    let mut csv_reader = ReaderBuilder::new();
    csv_reader
        .delimiter(b'\t')
        .trim(csv::Trim::Fields)
        .flexible(true);

    for table in glob("syscalls-table/tables/*")
        .expect("Failed to glob?!")
        .filter_map(Result::ok)
    {
        let syscalls = map
            .entry(
                table
                    .file_name()
                    .expect("No filename?!")
                    .to_str()
                    .expect("Invalid string?!")
                    .replace("syscalls-", ""),
            )
            .or_default();

        let mut reader = csv_reader
            .from_path(table)
            .expect("Failed to open table for reading");

        for record in reader.records().filter_map(Result::ok) {
            let name = record.get(0).expect("No syscall name?!");
            if let Some(number) = record.get(1).and_then(|n| n.parse::<u64>().ok()) {
                syscalls.insert(name.to_string(), number);
            }
        }
    }

    map
}
