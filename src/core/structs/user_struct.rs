use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub display_name: Option<String>,
    pub external_urls: Urls,
    pub followers: Follow,
    pub href: String,
    pub id: String,
    pub images: Image,
    pub product: String,
    #[serde(rename="type")]
    pub utype: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Follow {
    // Null
    pub href: String,
    pub total: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>
}