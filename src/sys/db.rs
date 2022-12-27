use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
// Document found in mongodb database collection
use bson::document::Document;
use std::env;
use std::error::Error;
use tokio;

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
       env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
 
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
       ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
          .await?;
    let client = Client::with_options(options)?;
 
    // client implements std::sync::Arc, can use clone()
    // https://mongodb.github.io/mongo-rust-driver/manual/connecting.html
    //let client1 = client.clone();
 
    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
       println!("- {}", name);
    }

    //Ok(())
    Ok(client)
}