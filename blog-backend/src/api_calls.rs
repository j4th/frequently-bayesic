pub mod open_calls {

    use actix_web::{get, web, HttpResponse, Responder};
    use serde::Deserialize;

    // Get request that returns a JSON file of post titles and their corresponding id's
    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        HttpResponse::Ok().body("This will return a bunch of post titles and id's!")
    }

    #[derive(Deserialize)]
    struct Info {
        postid: u32,
    }

    #[get("/posts/{postid}")]
    async fn get_posts_by_id(info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body(format!(
            "This will return the content of the post with the id: {}",
            info.postid
        ))
    }
}
