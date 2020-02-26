pub mod open_calls {

    use actix_web::{get, HttpResponse, Responder};

    // Get request that returns a JSON file of post titles and their corresponding id's
    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        HttpResponse::Ok().body("This will return a bunch of post titles and id's!")
    }
}
