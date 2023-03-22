use std::sync::RwLock;

use actix_web::http::header::ContentType;
use actix_web::web::{Data, Path};
use actix_web::{get, Error, HttpResponse};
use log::debug;

use crate::hex_grid::Cordinate;
use crate::world::{World};

#[get("/health")]
async fn health() -> &'static str {
    "Alive"
}

#[get("/world/{room}")]
async fn get_room(room: Path<String>, world: Data<RwLock<World>>) -> Result<HttpResponse, Error> {

    if let Some(c) = parse_cord(room.as_str()) {

        if let Some(r) = world.read().unwrap().get(&c) {

            let body = serde_json::to_string(r).unwrap();
            debug!("{body:?}");
            return Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body));
        }
    }
    Ok(HttpResponse::BadRequest().finish())
}

fn parse_cord(input: &str) -> Option<Cordinate> {
    let mut iter = input.split(',');
    let q: i32 = iter.next()?.parse().ok()?;
    let r: i32 = iter.next()?.parse().ok()?;
    let s: i32 = iter.next()?.parse().ok()?;
    Cordinate::new(q, r, s).ok()
}
