use actix_web::{web::Data, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type ScrabSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> ScrabSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
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
    async fn test(&self) -> &'static str {
        "alive"
    }
}
