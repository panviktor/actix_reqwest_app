use reqwest;
use reqwest::header::{ ACCEPT, CONTENT_TYPE,AUTHORIZATION };
use serde::{Deserialize, Serialize};

use actix_web::{get, web, Responder, Result};
use actix_web::HttpResponse;

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
        .header(AUTHORIZATION, "Bearer BQC116drDFsNU3zHz9Hs7csqMbJqjTQ_2dRg_vkHsNPQ1hzex6kV9tTB9LstU9bkDuWjWsGhVVTLQL1SFQ9jeCGP6HPeVl4FTCB6iq_fEcWpnON4jvdWKAQcOt0wMSSqKwVzTcIr3738u7XGi77ct2Y2aKHALrMSMVCIcwCmW1v4iyzlObDWjByN2p0nWdal0FY")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                // Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Ok(parsed) => serde_json::to_string(&parsed).unwrap(),
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

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
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
