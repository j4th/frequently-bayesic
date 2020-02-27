pub mod open_calls {
    use crate::parameter_structs::parameter_structs;
    use actix_web::{get, web, HttpResponse, Responder};

    // Get request that returns a JSON file of post titles and their corresponding id's
    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        HttpResponse::Ok().body("This will return a bunch of post titles and id's!")
    }

    #[get("/posts/{postid}")]
    async fn get_posts_by_id(info: web::Path<parameter_structs::PostId>) -> impl Responder {
        HttpResponse::Ok().body(format!(
            "This will return the content of the post with the id: {}",
            info.postid
        ))
    }
}
