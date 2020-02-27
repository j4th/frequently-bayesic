pub mod open_calls {
    use crate::parameter_structs::parameter_structs::*;
    use actix_web::{get, web, HttpResponse, Responder};

    // Get request that returns a JSON file of post titles and their corresponding id's
    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        let posts: Vec<PostInformation> = get_posts_information().await;

        HttpResponse::Ok().json(posts)
    }

    // retrieves the post overview information
    async fn get_posts_information() -> Vec<PostInformation> {
        let mut ret_val: Vec<PostInformation> = Vec::new();
        ret_val.push(PostInformation {
            title: "This is the first post test example".to_string(),
            id: 0,
        });
        ret_val.push(PostInformation {
            title: "This is the second post test example".to_string(),
            id: 1,
        });

        return ret_val;
    }

    #[get("/posts/{postid}")]
    async fn get_posts_by_id(info: web::Path<PostId>) -> impl Responder {
        HttpResponse::Ok().body(format!(
            "This will return the content of the post with the id: {}",
            info.postid
        ))
    }
}
