pub mod open_calls {

    use actix_web::{get, HttpResponse, Responder};

    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        HttpResponse::Ok().body("This will return a bunch of post titles and id's!")
    }
}
