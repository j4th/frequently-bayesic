pub mod open_calls {
    use crate::parameter_structs::parameter_structs::*;
    use actix_web::{get, web, HttpResponse, Responder};

    // Get request that returns a JSON file of post titles and their corresponding id's
    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        let post: PostInformation = PostInformation {
            title: "This is the title of a test post".to_string(),
            id: 1,
        };
        HttpResponse::Ok().json(post)
    }

    #[get("/posts/{postid}")]
    async fn get_posts_by_id(info: web::Path<PostId>) -> impl Responder {
        HttpResponse::Ok().body(format!(
            "This will return the content of the post with the id: {}",
            info.postid
        ))
    }
}
