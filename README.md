# decolor 

[![Build Status]][actions]
[![License]][mit-license]
[![Docs]][Docs-rs]
[![Latest Version]][crates.io]
[![rustc 1.31+]][Rust 1.31]

[Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/decolor/ci.yml?branch=main
[actions]: https://github.com/refcell/decolor/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/decolor.svg
[crates.io]: https://crates.io/crates/decolor
[rustc 1.31+]: https://img.shields.io/badge/rustc_1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/badge/license-MIT-7795AF.svg
[mit-license]: https://github.com/refcell/decolor/blob/main/LICENSE.md
[Docs-rs]: https://docs.rs/decolor/
[Docs]: https://img.shields.io/docsrs/decolor.svg?color=319e8c&label=docs.rs


**...** Decolor is in https://github.com/refcell/decolor/labels/beta.

![](./etc/banner.png)

**[Install](#usage)**
| [User Docs](#what-is-decolor)
| [Crate Docs][crates.io]
| [Reference][Docs-rs]
| [Contributing](#contributing)
| [License](#license)

## What is decolor?

`decolor` is a


## Usage

Add `decolor` as a dependency with cargo.

```ignore,sh,no_run
cargo add decolor
```

A short example for building a purple function using the
[decolor][decolor] decorator.

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("no implemented!");
    Ok(())    
}
```

## Contributing

All contributions are welcome! Experimentation is highly encouraged
and new issues are welcome.

## Troubleshooting & Bug Reports

Please check existing issues for similar bugs or
[open an issue](https://github.com/refcell/decolor/issues/new)
if no relevant issue already exists.

## License

This project is licensed under the [MIT License](LICENSE.md).
Free and open-source, forever.
*All our rust are belong to you.*
