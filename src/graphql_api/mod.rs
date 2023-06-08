use std::sync::Arc;

use actix_web::{web::Data, HttpResponse};
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription,
    Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use hex_grid::Cordinate;
use scrab_types::{Room, Tile, World};

use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub world: Arc<RwLock<World>>,
}

pub type ScrabSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(world: AppState) -> ScrabSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(world)
        .finish()
}

pub async fn graphql_index(schema: Data<ScrabSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/data").finish())
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Information about the game world
    async fn world(&self, ctx: &Context<'_>) -> GQLWorld {
        GQLWorld
    }
}

pub struct GQLWorld;

#[Object(name = "World")]
impl GQLWorld {
    async fn room<'a>(&self, ctx: &Context<'a>, q: i32, r: i32, s: i32) -> Result<GQLRoom, String> {
        let app_data = ctx.data::<AppState>().unwrap();
        if let Ok(cord) = Cordinate::new(q, r, s) {
            if let Some(room) = app_data.world.read().await.rooms.get(cord) {
                Ok(GQLRoom {
                    tiles: room
                        .tiles
                        .iter()
                        .map(|(tile, cordinate)| GQLTile {
                            is_wall: tile.is_wall,
                            cordinate: cordinate.into(),
                        })
                        .collect(),
                    cordinate: GQLCordinate::from(cord),
                })
            } else {
                Err("room does not exist".into())
            }
        } else {
            Err("invalid cordinate".into())
        }
    }
}

#[derive(SimpleObject)]
#[graphql(name = "Room")]
pub struct GQLRoom {
    tiles: Vec<GQLTile>,
    cordinate: GQLCordinate
}

#[derive(SimpleObject)]
#[graphql(name = "Tile")]
pub struct GQLTile {
    is_wall: bool,
    cordinate: GQLCordinate,
}

#[derive(SimpleObject)]
#[graphql(name = "Cordinate")]
pub struct GQLCordinate {
    q: i32,
    r: i32,
    s: i32,
}

impl From<Cordinate> for GQLCordinate {
    fn from(value: Cordinate) -> Self {
        Self {
            q: value.q(),
            r: value.r(),
            s: value.s(),
        }
    }
}
