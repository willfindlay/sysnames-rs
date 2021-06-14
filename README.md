# Sysnames

[![crates.io badge](https://img.shields.io/crates/v/sysnames.svg)](https://crates.io/crates/sysnames)

A Rust library for Linux system call names and numbers based on [Marcin Juszkiewicz's excellent repository].
The goal is to help you work with system call information in Rust.
For example, this library might be particularly useful when working with [libbpf-rs].

To use this in your project:
```toml
[dependencies]
sysnames = "0.1"
```

## Disclaimer

This library is currently a work in progress. Expect updates regularly. Semantic versioning will be followed after the 1.0.0 release.

[Marcin Juszkiewicz's excellent repository]: https://github.com/hrw/syscalls-table
[libbpf-rs]: https://github.com/libbpf/libbpf-rs
