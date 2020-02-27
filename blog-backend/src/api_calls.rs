pub mod open_calls {
    use crate::parameter_structs::parameter_structs::*;
    use actix_web::{get, web, HttpResponse, Responder};

    // Get request that returns a JSON file of post titles and their corresponding id's
    #[get("/posts")]
    async fn get_posts() -> impl Responder {
        let posts: Vec<PostInformation> = get_posts_information().await;

        return HttpResponse::Ok().json(posts);
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
        let post: Post = get_post_information(info.postid).await;

        return HttpResponse::Ok().json(post);
    }

    async fn get_post_information(post_id: u32) -> Post {
        let ret_val: Post = Post {
            title: "This is the first post test example".to_string(),
            article: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                .to_string(),
            author: "Testy McTesterson".to_string(),
            date: "2020-02-27".to_string(),
            id: post_id,
        };

        return ret_val;
    }
}
