// SPDX-License-Identifier: MIT
//
// Linux system call names and numbers.
// Copyright (c) 2021  William Findlay
//
// June 14, 2021  William Findlay  Created this.

//! Get system call information for the Linux kernel according to your target
//! architecture.

#![deny(missing_docs)]

mod syscalls;

/// Get system call information for your target architecture.
/// Currently only supports the Linux kernel.
pub struct Syscalls;
impl Syscalls {
    /// Get a system call name by its number, if it exists on your architecture.
    pub fn number(name: &str) -> Option<u64> {
        syscalls::SYSCALL_NUMS
            .get_by_left(name.to_lowercase().as_str())
            .cloned()
    }

    /// Get a system call number by its name, if it exists on your architecture.
    pub fn name(number: &u64) -> Option<&str> {
        syscalls::SYSCALL_NUMS.get_by_right(number).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_number_bijection_test() {
        let number = Syscalls::number("execve").unwrap();
        let name = Syscalls::name(&number).unwrap();
        assert_eq!(name, "execve");
    }
}
