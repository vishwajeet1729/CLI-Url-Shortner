use mongodb::{bson::doc, options::ClientOptions, Client};
use rand::Rng;
use std::io::{self, Write};

const MONGO_URI: &str = "mongodb+srv://vishwajeetwalse9767:qSWh7SXSyWyGI4Fo@cluster0.l5a39x1.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
const DB_NAME: &str = "url_shortener";
const COLLECTION_NAME: &str = "urls";

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Connect to MongoDB
    let client = Client::with_uri_str(MONGO_URI).await?;
    let db = client.database(DB_NAME);
    let collection = db.collection::<mongodb::bson::Document>(COLLECTION_NAME);

    // Ask user for input
    print!("Enter a URL to shorten: ");
    io::stdout().flush().unwrap();
    let mut input_url = String::new();
    io::stdin().read_line(&mut input_url).unwrap();
    let input_url = input_url.trim(); // Remove extra spaces/newlines

    // Generate a random short code
    let short_code = generate_short_code();

    // Save to MongoDB
    let doc = doc! { "short_code": &short_code, "original_url": &input_url };
    collection.insert_one(doc, None).await?;

    // Print the shortened URL
    println!("Shortened URL: https://short.ly/{}", short_code);

    Ok(())
}

// Function to generate a random short code
fn generate_short_code() -> String {
    let charset: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..6).map(|_| charset[rng.gen_range(0..charset.len())]).collect()
}
