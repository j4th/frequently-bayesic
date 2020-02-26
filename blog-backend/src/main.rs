mod api_calls;
#[cfg(test)]
mod api_test;

use crate::api_calls::open_calls;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(open_calls::get_posts))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
