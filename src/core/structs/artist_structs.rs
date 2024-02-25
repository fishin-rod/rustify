use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artists {
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub followers: Option<Followers>,
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    pub popularity: Option<i32>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    pub href: Option<String>,
    pub total: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Albums {
    pub href: String,
    pub limit: i32,
    pub next: Option<String>,
    pub offset: i32,
    pub previous: Option<String>,
    pub total: i32,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "album_type")]
    pub album_type: String,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i32,
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub artists: Vec<Artist>,
    #[serde(rename = "album_group")]
    pub album_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopTracks {
    pub tracks: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    #[serde(rename = "available_markets")]
    pub available_markets: Option<Vec<String>>,
    #[serde(rename = "disc_number")]
    pub disc_number: i32,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i32,
    pub explicit: bool,
    #[serde(rename = "external_ids")]
    pub external_ids: ExternalIds,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "is_playable")]
    pub is_playable: bool,
    #[serde(rename = "linked_from")]
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: i32,
    #[serde(rename = "preview_url")]
    pub preview_url: Option<String>,
    #[serde(rename = "track_number")]
    pub track_number: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    #[serde(rename = "is_local")]
    pub is_local: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "album_type")]
    pub album_type: String,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i32,
    #[serde(rename = "available_markets")]
    pub available_markets: Option<Vec<String>>,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restrictions {
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub isrc: String,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedFrom {
}
