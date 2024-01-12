use mongodb::{Client, Database};


pub async fn connection(database_url: &str) -> Database{
    let client = Client::with_uri_str(database_url).await.expect("failed to connect");
    client.database("blog_uppgift")
}