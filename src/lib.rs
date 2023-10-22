#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use anyhow::Result;

/// Purple function demonstratings a blue function
/// that internally hides a blocking call to a red
/// function using a runtime like tokio.
pub fn purple() -> Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Perform an async file read call
        println!("Inside purple blocking call...");
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_purple_inside_runtime() {
        let result = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // This function call will panic because internally
                // it tries to construct a tokio runtime, which would
                // create a nested runtime (not allowed).
                let _ = purple();
            });
        });
        assert!(result.is_err());
    }
}
