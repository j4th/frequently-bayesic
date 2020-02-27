mod api_calls;
#[cfg(test)]
mod api_test;
mod parameter_structs;

use crate::api_calls::open_calls;
use actix_web::{App, HttpServer};

// Binding function of the API, initializes the actix (https://actix.rs/) server and binds the endpoints
// found in the api_calls.rs file to that server
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(open_calls::get_posts)
            .service(open_calls::get_posts_by_id)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
