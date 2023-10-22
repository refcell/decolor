/// Purple function demonstratings a blue function
/// that internally hides a blocking call to a red
/// function using a runtime like tokio.
fn purple() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Perform an async file read call
        println!("Inside purple blocking call...");
    });
    Ok(())
}

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
