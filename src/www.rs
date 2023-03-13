use actix_web::get;


#[get("/health")]
async fn health() -> &'static str {
    "Alive"
}