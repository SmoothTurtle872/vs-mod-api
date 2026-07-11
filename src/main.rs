use reqwest::{Client, get};
use vs_mod_api::api::v1::Endpoint;

// Examples
#[tokio::main]
async fn main() {
    let url = Endpoint::Authors.get_url().unwrap();
    println!("{:?}", url);
    let client = Client::builder()
        .user_agent("VsModAPIRustCrate/1.0")
        .build()
        .unwrap();
    let request = client.get(url).send().await.unwrap();

    println!("{:?}", request)
}
