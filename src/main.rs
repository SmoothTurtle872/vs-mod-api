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

    let data = Endpoint::Mods(Some(ModSearchSettings {
        text: Some("More Classes".to_string()),
        tag_ids: None,
        game_versions: None,
        author: None,
        order_by: None,
        order_direction: None,
    }))
    .get_data(&client)
    .await
    .unwrap();
    println!("{data:?}");
}
