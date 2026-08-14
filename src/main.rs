use reqwest::Client;
use rusty_vs_mod_api::api::v1::{Endpoint, ModSearchSettings};

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .user_agent(format!(
            "VintageStoryRustModApiExamples/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();

    let data = Endpoint::Mods(None).get_data(&client).await.unwrap();
    println!("{data:?}");
}
