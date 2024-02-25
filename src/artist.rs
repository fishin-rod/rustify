use crate::core::{auth::get_token, cache::Cache, structs::artist_structs::{Albums, Artist, Artists, TopTracks}};

use std::collections::HashMap;

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtistData {
    pub userid: String,
    cache: Cache<ArtistResult>,
    token: String,
    body: HashMap<String, BodyValues>,
    pub artistid: String,
    pub artist: bool,
    pub artists: bool,
    pub albums: bool,
    pub top_tracks: bool,
    pub related_artists: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArtistResult {
    Artist(Artist),
    Artists(Artists),
    Albums(Albums),
    TopTracks(TopTracks),
    RelatedArtists(Artists),
    Null,
    Error(ArtistErrors),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArtistErrors {
    ArtistNotFound,
    InvalidArguments,
    ReqwestError,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BodyValues {
    Str(String),
    Int(i32),
}

macro_rules! GenerateFunction {
    ($doc: expr, $name: ident, $datatype:ty, $($field:tt).+, unwrap) => {
        #[doc=$doc]
        pub fn $name(&self) -> $datatype {
            if let ArtistResult::Artist(artist_data) = self {
                artist_data.$($field).+.clone().unwrap()
            }
            else {
                panic!("Could not get data from: {:?}", self)
            }
        }
    };

    ($doc: expr, $name: ident, $datatype:ty, $($field:tt).+) => {
        #[doc=$doc]
        pub fn $name(&self) -> $datatype {
            if let ArtistResult::Artist(artist_data) = self {
                artist_data.$($field).+.clone()
            }
            else {
                panic!("Could not get data from: {:?}", self)
            }
        }
    };
}

macro_rules! GenerateFunctions {
    ($doc: expr, $name: ident, $datatype:ty, $($field:tt).+) => {
        #[doc=$doc]
        pub fn $name(&self) -> Vec<$datatype> {
            if let ArtistResult::Artists(artist_data) = self {
                let mut vec = Vec::new();
                for artist in &artist_data.artists {
                    vec.push(artist.$($field).+.clone());
                }
                vec
            }
            else if let ArtistResult::RelatedArtists(artist_data) = self {
                let mut vec = Vec::new();
                for artist in &artist_data.artists {
                    vec.push(artist.$($field).+.clone());
                }
                vec
            }
            else {
                panic!("Could not get data from: {:?}", self)
            }
        } 
    };
}

impl Albums {
    // Concatenate two Albums instances
    pub(crate) fn concatenate(mut self, mut other: Albums) -> Self {
        self.items.append(&mut other.items);
        self.total += other.total;
        self.next = other.next.or(self.next); // Take the next from other if present
        self
    }
}

impl ArtistData {
    pub fn new(uid: String, secret: String) -> ArtistData {
        ArtistData {
            userid: uid.to_owned(),
            cache: Cache::new(),
            token: get_token(uid, secret),
            body: HashMap::new(),
            artistid: String::new(),
            artist: false,
            artists: false,
            albums: false,
            top_tracks: false,
            related_artists: false,
        }
    }

    pub fn get_artist(&mut self, artistid: &str) -> &mut Self {
        self.artist = true;
        self.artistid = "/".to_string() + artistid;
        self
    }   

    pub fn get_artists(&mut self, ids: &[&str]) -> &mut Self {
        // Don't take from the cache due to the nature of the function but do add, just in case
        self.artists = true;

        let mut string = "?ids=".to_string();
        for i in 0..ids.len() {
            if i < ids.len()-1 {
                string = string + &format!("{}%2C", ids[i]);
            }
            else {
                string = string + ids[i];
            }
        }
        self.artistid = string;
        self
    }

    // make it optional to add the values at all, don't think its a thing in rust but maybe there is a way
    // fix and follow link example: https://api.spotify.com/v1/artists/0C0XlULifJtAgn6ZNCW2eu/albums?include_groups=album,single,compilation,appears_on&offset=20&limit=20
    pub fn get_albums(&mut self, artistid: &str, groups: Option<Vec<&str>>, market: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> &mut Self {
        self.albums = true;
        self.artistid = "/".to_string() + artistid + "/albums";

        if groups.is_some() {
            self.body.insert("include_groups".to_string(), BodyValues::Str(groups.unwrap().join(",")));
        }
        if market.is_some() {
            self.body.insert("market".to_string(), BodyValues::Str(market.unwrap().to_string()));
        }
        if limit.is_some() {
            self.body.insert("limit".to_string(), BodyValues::Int(limit.unwrap()));
        }
        if offset.is_some() {
            self.body.insert("offset".to_string(), BodyValues::Int(offset.unwrap()));
        }
        self
    }

    // WHY DOES THIS ONE NEED MARKET BUT THE OTHER ONE IS ONLY OPTIONAL?!
    pub fn get_top_tracks(&mut self, artistid: &str, market: &str) -> &mut Self {
        self.top_tracks = true;
        self.artistid = "/".to_string() + artistid + "/top-tracks";

        self.body.insert("market".to_string(), BodyValues::Str(market.to_string()));
        self
    }

    pub fn get_related_artists(&mut self, artistid: &str) -> &mut Self {
        self.related_artists = true;
        self.artistid = "/".to_string() + artistid + "/related-artists";
        self
    }

    #[tokio::main]
    pub async fn run(&mut self) -> ArtistResult {
        if self.cache.contains_key(&self.artistid) {
            let value = self.cache.get(&self.artistid);
            return value.unwrap().to_owned();
        }

        let client = Client::new();
        let url: String = format!("https://api.spotify.com/v1/artists{}", self.artistid);
        let mut auth_string = format!("Bearer {}", self.token);
        auth_string = auth_string.replace('\\', "");
        auth_string = auth_string.replace('\"', "");

        let mut send = client.get(url).header("Authorization", auth_string);
        
        if !self.body.is_empty() {
            for (key, value) in self.body.iter() {
                match value {
                    BodyValues::Str(s) => {
                        send = send.query(&[(key.as_str(), s.as_str())]);
                    }
                    BodyValues::Int(i) => {
                        send = send.query(&[(key.as_str(), *i)]);
                    }
                }
            }
        }

        let response = match send.send().await{
            Ok(response) => response,
            Err(_err) => return ArtistResult::Error(ArtistErrors::ReqwestError),
        };
        let value = self.parse(response).await;
        value
    }

    async fn parse(&mut self, response: Response) -> ArtistResult {
        if response.status() == reqwest::StatusCode::NOT_FOUND{
            return ArtistResult::Error(ArtistErrors::ArtistNotFound);
        }
        else if self.artist {
            self.artist = false;
            let response = match response.json::<Artist>().await{
                Ok(response) => response,
                Err(err) =>  {println!("{:?}", err); return ArtistResult::Error(ArtistErrors::ReqwestError)},
            };
            let value = ArtistResult::Artist(response);
            self.cache.add(&self.artistid, value.clone());
            value
        }
        else if self.artists {
            self.artists = false;
            let response = match response.json::<Artists>().await{
                Ok(response) => response,
                Err(err) => {println!("{:?}", err); return ArtistResult::Error(ArtistErrors::ReqwestError)},
            };
            let value = ArtistResult::Artists(response.clone());
            let ids: Vec<&str> = self.artistid.split(',').collect();
            for i in 0..ids.len() {
                self.cache.add(ids[i], ArtistResult::Artist(response.artists[i].clone()))
            }
            value    
        }
        else if self.albums {
            self.albums = false;
            let response = match response.json::<Albums>().await{
                Ok(response) => response,
                Err(err) => {println!("{:?}", err); return ArtistResult::Error(ArtistErrors::ReqwestError)},
            };
            let mut value = ArtistResult::Albums(response.clone());
            if response.clone().next.is_some() {
                let url = response.clone().next.unwrap();
                value = self.next(&url, response.clone()).await;
            }
            value
        } 
        else if self.top_tracks {
            self.top_tracks = false;
            let response = match response.json::<TopTracks>().await{
                Ok(response) => response,
                Err(err) => {println!("{:?}", err); return ArtistResult::Error(ArtistErrors::ReqwestError)},
            };
            let value = ArtistResult::TopTracks(response.clone());
            value  
        } 
        else if self.related_artists {
            self.related_artists = false;
            let response = match response.json::<Artists>().await{
                Ok(response) => response,
                Err(err) => {println!("{:?}", err); return ArtistResult::Error(ArtistErrors::ReqwestError)},
            };
            let value = ArtistResult::RelatedArtists(response.clone());
            value 
        }
        else {
            ArtistResult::Error(ArtistErrors::ReqwestError)
        }
    }

    async fn next(&self, url: &str, value: Albums) -> ArtistResult {
        let client = Client::new();

        let mut auth_string = format!("Bearer {}", self.token);
        auth_string = auth_string.replace('\\', "");
        auth_string = auth_string.replace('\"', "");

        let send = client.get(url).header("Authorization", auth_string).send().await.unwrap();
        
        let mut response = match send.json::<Albums>().await{
            Ok(response) => response,
            Err(err) => {println!("{:?}", err); return ArtistResult::Error(ArtistErrors::ReqwestError)},
        };

        response = value.concatenate(response.clone());
        let value2 = ArtistResult::Albums(response.clone());
        // ? if there is ever that many, idk value2 needs to be mut then 
        //and #[async_recursion] needs to be added over with the async recursion crate
        //use async_recursion::async_recursion;
        //async-recursion = "1.0.5"
        //if response.clone().next.is_some() {
        //    let url = response.clone().next.unwrap();
        //    value2 = self.next(&url, response.clone()).await;
        //}
        value2
    }
}

impl ArtistResult {
    
    GenerateFunction!("Gets the name of the artist from the ArtistResult struct and returns it as a String", 
    name, String, name);
    
    GenerateFunctions!("Gets the names of the artist from the ArtistResult struct and returns it as a Vec<String>",
    names, String, name);

    GenerateFunction!("Gets the id of the artist from the ArtistResult struct and returns it as a string", 
    id, String, id);
    
    GenerateFunctions!("Gets the ids of the artist from the ArtistResult struct and returns it as a Vec<String>",
    ids, String, id);

    GenerateFunction!("Gets the popularity of the artist from the ArtistResult struct and returns it as a i32.
    More info on popularity and how it works [here](https://developer.spotify.com/documentation/web-api/reference/get-an-artists-top-tracks)", 
    popularity, i32, popularity, unwrap);
    
    pub fn popularitys(&self) -> Vec<i32> {
        if let ArtistResult::Artists(artist_data) = self {
            let mut vec = Vec::new();
            for artist in &artist_data.artists {
                vec.push(artist.popularity.clone().unwrap());
            }
            vec
        }
        else if let ArtistResult::RelatedArtists(artist_data) = self {
            let mut vec = Vec::new();
            for artist in &artist_data.artists{
                vec.push(artist.popularity.clone().unwrap());
            }
            vec
        }
        else {
            panic!("Could not get the artists popularity's")
        }
    }

}