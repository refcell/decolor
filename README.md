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

**Asynchronous runtime abstractions for implicit function decoloring.** Decolor is in https://github.com/refcell/decolor/labels/beta

![](https://raw.githubusercontent.com/refcell/decolor/main/etc/banner.png)

**[Install](#usage)**
| [User Docs](#what-is-decolor)
| [Crate Docs][crates.io]
| [Reference][Docs-rs]
| [Contributing](#contributing)
| [License](#license)

## What is decolor?

`decolor` is a [procedural macro][proc-macro] crate that implements
a `#[decolor]` [attribute macro][attribute-macro] used to *"decolor"*
an asynchronous rust function. Concretely, the `#[decolor]` macro
can be placed above an asynchronous function to safely <sup>1</sup> transform it
into a ["purple" function][purple] (a synchronous function that blocks
on asynchronous functionality internally).

**1**: Constructing the `block_on()` call in this way prevents nested runtime
panics, but calling the Handle [block_on][block-on] method itself [panics][handle-panics]
if the provided future panics *or* if the runtime on which a timer future is called upon
is shut down prior to completion. Additionally, the [Runtime][runtime]'s
[`block_on`][runtime-block-on] call will [panic][runtime-panic] if it is called from within
an asynchronous execution context.

[runtime-panic]: https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#panics
[runtime-block-on]: https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
[runtime]: https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#
[block-on]: https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#method.block_on
[handle-panics]: https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#panics-2
[purple]: https://morestina.net/blog/1686/rust-async-is-colored
[attribute-macro]: https://doc.rust-lang.org/beta/reference/procedural-macros.html#attribute-macros
[proc-macro]: https://doc.rust-lang.org/beta/reference/procedural-macros.html

## Usage

Add `decolor` as a dependency with cargo.

```bash,ignore
cargo add decolor
```

A short example for building a purple function using the
[decolor][decolor] decorator.

```rust
use decolor::decolor;
use tokio::time::{sleep, Duration};

#[decolor]
async fn foo() -> anyhow::Result<()> {
    sleep(Duration::from_secs(1)).await;
    println!("Hello, world!");
    Ok(())
}

fn main() {
    assert!(foo().is_ok());
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
