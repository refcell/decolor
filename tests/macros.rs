use decolor::decolor;

#[decolor]
async fn foo() -> String {
    let one_second = tokio::time::Duration::from_secs(1);
    tokio::time::sleep(one_second).await;
    "Hello world!".to_string()
}

#[decolor]
async fn bar() {
    let one_second = tokio::time::Duration::from_secs(1);
    tokio::time::sleep(one_second).await;
    println!("Hello world!");
}

#[derive(Debug, PartialEq)]
struct Response {
    status: u16,
    body: String,
}

#[decolor]
async fn res() -> anyhow::Result<Response> {
    let one_second = tokio::time::Duration::from_secs(1);
    tokio::time::sleep(one_second).await;
    Ok(Response {
        status: 200,
        body: "Hello world!".to_string(),
    })
}

#[test]
fn test_decolor_result_with_struct() {
    let expected = Response {
        status: 200,
        body: "Hello world!".to_string(),
    };
    assert_eq!(res().unwrap(), expected);
}

#[test]
fn test_decolor_with_return_type() {
    assert_eq!(foo(), "Hello world!".to_string());
}

#[test]
fn test_decolor_empty_return_type() {
    assert_eq!(bar(), ());
}
