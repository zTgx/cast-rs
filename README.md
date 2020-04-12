# cast-rs

[![Travis status](https://travis-ci.org/zTgx/cast-rs.svg?branch=master)](https://travis-ci.org/zTgx/cast-rs) [![crate](https://img.shields.io/crates/v/cast-rs.svg)](https://crates.io/crates/cast-rs)

A collection of  types cast for Rust.

`cast-rs` is a meta-crate, re-exporting items from these sub-crates:

| Repository | Crate | Documentation |
| ---------- | ----- | ------------- |
|  [hex](https://github.com/KokaKiwi/rust-hex)  |  [![crate](https://img.shields.io/crates/v/hex.svg)](https://crates.io/crates/hex) |[![documentation](https://docs.rs/hex/badge.svg)](https://docs.rs/hex)
|  [downcast-rs](https://github.com/marcianx/downcast-rs)  |  [![crate](https://img.shields.io/crates/v/downcast_rs.svg)](https://crates.io/crates/downcast_rs) |[![documentation](https://docs.rs/downcast-rs/badge.svg)](https://docs.rs/downcast-rs)
|  [slice-cast](https://github.com/FaultyRAM/slice-cast.git)  |  [![crate](https://img.shields.io/crates/v/slice_cast.svg)](https://crates.io/crates/slice_cast) |[![documentation](https://docs.rs/slice-cast/badge.svg)](https://docs.rs/slice-cast)

Note: `cast-rs` is listed here for reference, but it's not directly included
in `cast-rs`.  This is a `proc-macro` crate for deriving some of `cast-rs`'s traits.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cast-rs = "0.2.1"
```

and those to your crate root:
### hex
```rust
use cast_rs::hexcast;
```

### downcast
```rust
extern crate cast_rs;
use cast_rs::downcast::*;
```

### slicecast
```rust
use cast_rs::slicecast;
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).
