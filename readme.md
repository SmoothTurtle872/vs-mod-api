This is a basic rust interface for the [Vintage Story Mod Database JSON API](https://github.com/anegostudios/vsmoddb#vs-mod-db-api-docs).
To use this you will need to add `reqwest` and `tokio` to your project.

A basic example to get all of the mods in the database.
```rust
use reqwest::Client;
use rusty_vs_mod_api::v1::Endpoint;

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .user_agent(format!(
            "VintageStoryRustModApiExample/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();

    let data = Endpoint::Mods(None).get_data(&client).await.unwrap();
    println!("{data:?}");
}
```

Do I plan to support v2 of the API? Yes. As you can see by the v1 module, I do intend on supporting v2 in the future, however that is under devlopment and much more complex. So for now it is only v1
