# leveldb-sys

Lowlevel bindings to the leveldb C library.

## Dependencies

* Your platforms C++ compiler (usually `gcc` or `clang` on Linux and Unix, Visual Studio Build environment on Windows)
* `cmake`

## Usage

If your project is using Cargo, drop the following lines in your Cargo.toml:

```
[dependencies]

leveldb-sys = "*"
```

## Features

`levelbd-sys` offers a `snappy` feature to build the snappy library.

## LICENSE

MIT

## BSD support

To build leveldb-sys you need to install `gmake` (GNU Make)
