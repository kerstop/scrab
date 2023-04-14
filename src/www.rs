use std::ops::Deref;
use std::sync::{Arc, RwLock};

use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};

use hex_grid::Cordinate;
use scrab_public_types::{PubRoom, WorldManifest};
use scrab_types::World;

#[derive(Clone)]
pub struct AppState {
    pub world: Arc<RwLock<World>>,
}

#[get("/health")]
async fn health() -> &'static str {
    "Alive"
}

#[get("/world/{room}")]
async fn get_room(room: Path<String>, app: Data<AppState>) -> HttpResponse {
    if let Some(c) = parse_cord(room.as_str()) {
        if let Some(r) = app.world.read().unwrap().rooms.get(c) {
            return HttpResponse::Ok().json(PubRoom::from(r));
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
async fn get_world_manifest(app: Data<AppState>) -> HttpResponse {
    let world = app.world.read().unwrap();
    return HttpResponse::Ok().json(WorldManifest::from(world.deref()));
}
