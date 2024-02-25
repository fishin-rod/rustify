//  Find a way to store the key to be used multiple times
// key resets only every hour

use std::collections::HashMap;

use reqwest::{Client, header};
use serde_json::Value;

// Makes a web app token to be used by an APP!
#[tokio::main]
pub async fn get_token(id: String, secret: String) -> String {
    const URL: &str = "https://accounts.spotify.com/api/token";
    let mut request_body = HashMap::new();
    request_body.insert("grant_type", "client_credentials");
    request_body.insert("client_id", &id);
    request_body.insert("client_secret", &secret);

    let client = Client::new();

    let response = client
        .post(URL)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded").form(&request_body)
        .send()
        .await.unwrap();

    let value: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    return value.get("access_token").expect("Error: Was not able to parse JSON data check that your inputs are correct and that spotify's servers are not down ").to_string();
}

// do one for the user auth that involves redirects?