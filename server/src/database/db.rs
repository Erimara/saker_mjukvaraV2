use mongodb::{Client, Database};


pub async fn connection() -> Database{
    let client = Client::with_uri_str("x").await.expect("failed to connect");
    client.database("blog_uppgift")
}