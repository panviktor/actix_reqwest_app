use std::path::Display;
use reqwest;
use reqwest::header::{ ACCEPT, CONTENT_TYPE,AUTHORIZATION };
use serde::{Deserialize, Serialize};

use actix_web::{get, web, Responder, Result};
use actix_web::HttpResponse;

use std::fmt;


#[get("/spotify/{search_query}")]
async fn index(search_query: web::Path<String>) -> Result<impl Responder> {
    let str = check_album(&search_query.into_inner()).await;
    Ok(HttpResponse::Ok().body(str))
}

// tokio let's us use "async" on our main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn check_album(search_query: &str) -> String {
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(AUTHORIZATION, "Bearer BQCPmBBAVhZaupS2Cie9-h8AbjanakU98D4jYPYunaN1YX8gpsv541B8ZheCa51Ho_o_Hju5Hm5yRMUHIQDDq2OwZhfruZ0qJmbLJgswlTAK5pBfreH4X2JL_4okT2A_0TVNrbkLhYkR6rdCuJo1r8U4g7P1eGOYcdKlULNPUfQa5cJW4Dg0j-L2jvsjTyoAyRo")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => ("Hm, the response didn't match the shape we expected.").to_string(),
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            ("Need to grab a new token").to_string()
        }
        other => {
            "Uh oh! Something unexpected happened".to_string();
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

fn print_tracks(tracks: Vec<&Track>) -> String {
    let mut string = String::new();
    for track in tracks {
        string.push_str(&track.to_string());
        string.push_str(&"--------- \n".to_string());
    }
    string.to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Track Name: {}, \n Album: {} \n)", self.name, self.album)
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} : {:?}\n)", self.name, self.artists)
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {} and url: {}\n)", self.name, self.external_urls.spotify)
    }
}