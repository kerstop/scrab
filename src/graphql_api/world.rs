use std::fmt::format;

use async_graphql::{ComplexObject, Object};


pub struct GQLWorld {

}

#[Object(name="World")]
impl GQLWorld {
    async fn room(&self, q:i32, r:i32, s:i32) -> String {
        format!("[{q},{r},{s}]")
    }

}