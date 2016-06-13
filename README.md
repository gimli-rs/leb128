# `leb128`

[![](http://meritbadge.herokuapp.com/leb128)![](https://img.shields.io/crates/d/leb128.png)](https://crates.io/crates/leb128)

[![Build Status](https://travis-ci.org/fitzgen/leb128.png?branch=master)](https://travis-ci.org/fitzgen/leb128)

[![Coverage Status](https://coveralls.io/repos/github/fitzgen/leb128/badge.svg?branch=master)](https://coveralls.io/github/fitzgen/leb128?branch=master)

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length integer
encoding.

The implementation is a direct translation of the psuedocode in the DWARF 4
standard's appendix C.

## Install

Either

    $ cargo add leb128

or add this to your `Cargo.toml`:

    [dependencies]
    leb128 = "0.1.0"

## Documentation

[Documentation](http://fitzgen.github.io/leb128/leb128/index.html)
