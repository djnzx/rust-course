use reqwest;

#[tokio::test]
async fn f1() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://api64.ipify.org?format=text")
        .await?
        .text()
        .await?;
    println!("Ваша IP-адреса: {}", response);
    Ok(())
}

#[test]
fn test_f1() {
    let future = f1();
    future.unwrap()
}
