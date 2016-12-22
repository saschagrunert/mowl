# mowl
[![Build Status](https://travis-ci.org/saschagrunert/mowl.svg)](https://travis-ci.org/saschagrunert/mowl) [![Build status](https://ci.appveyor.com/api/projects/status/mfmk83rc0vw0wj36?svg=true)](https://ci.appveyor.com/project/saschagrunert/mowl) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/mowl/badge.svg?branch=master)](https://coveralls.io/github/saschagrunert/mowl?branch=master) [![master doc mowl](https://img.shields.io/badge/master_doc-mowl-blue.svg)](https://saschagrunert.github.io/mowl) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/mowl/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/mowl.svg)](https://crates.io/crates/mowl) [![doc.rs](https://docs.rs/mowl/badge.svg)](https://docs.rs/mowl)
## (m)y (ow)n (l)ogger
A simple logger with coloring support

## Example usage
```rust
#[macro_use]
extern crate log;
extern crate mowl;

fn main() {
    mowl::init().unwrap();
    warn!("Warning");
}
```
