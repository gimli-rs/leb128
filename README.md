# `leb128`

[![](http://meritbadge.herokuapp.com/leb128) ![](https://img.shields.io/crates/d/leb128.png)](https://crates.io/crates/leb128) [![Build Status](https://travis-ci.org/fitzgen/leb128.png?branch=master)](https://travis-ci.org/fitzgen/leb128) [![Coverage Status](https://coveralls.io/repos/github/fitzgen/leb128/badge.svg?branch=master)](https://coveralls.io/github/fitzgen/leb128?branch=master)

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length integer
encoding.

The implementation is a direct translation of the psuedocode in the DWARF 4
standard's appendix C.

## Install

Either

    $ cargo add leb128

or add this to your `Cargo.toml`:

    [dependencies]
    leb128 = "0.2.1"

## Example

```rust
use leb128;

let mut buf = [0; 1024];

// Write to anything that implements `std::io::Write`.
{
    let mut writable = &mut buf[..];
    leb128::write::signed(&mut writable, -12345).expect("Should write number");
}

// Read from anything that implements `std::io::Read`.
let mut readable = &buf[..];
let val = leb128::read::signed(&mut readable).expect("Should read number");
assert_eq!(val, -12345);
```

## Documentation

[Documentation](http://fitzgen.github.io/leb128/leb128/index.html)

## License

Licensed under either of

  * Apache License, Version 2.0 ([`LICENSE-APACHE`](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([`LICENSE-MIT`](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
