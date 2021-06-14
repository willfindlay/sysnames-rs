// SPDX-License-Identifier: MIT
//
// Query Linux system call names and numbers at runtime.
// Copyright (c) 2021  William Findlay
//
// June 14, 2021  William Findlay  Created this.

#![allow(missing_docs)]

use std::env;
use std::include;

include!(concat!(env!("OUT_DIR"), "/syscalls.rs"));
