use std::sync::RwLock;

use actix_web::web::{Data, Path};
use actix_web::{get, Error, HttpResponse};

use crate::world::World;
use hex_grid::Cordinate;
use scrab_public_types::{PublicRoom, PublicRoomManifest, PublicWorld};

#[get("/health")]
async fn health() -> &'static str {
    "Alive"
}

#[get("/world/{room}")]
async fn get_room(room: Path<String>, world: Data<RwLock<World>>) -> HttpResponse {
    if let Some(c) = parse_cord(room.as_str()) {
        if let Some(r) = world.read().unwrap().rooms.get(c) {
            return HttpResponse::Ok().json(PublicRoom::from(r));
        }
    }

    HttpResponse::BadRequest().finish()
}

fn parse_cord(input: &str) -> Option<Cordinate> {
    let mut iter = input
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',');
    let q: i32 = iter.next()?.parse().ok()?;
    let r: i32 = iter.next()?.parse().ok()?;
    let s: i32 = iter.next()?.parse().ok()?;
    Cordinate::new(q, r, s).ok()
}

#[get("/world/")]
async fn get_world_manifest(world: Data<RwLock<World>>) -> HttpResponse {
    if let Ok(world) = world.read() {
        return HttpResponse::Ok().json(PublicWorld::from(world.to_pub()));
    }
    todo!()
}
