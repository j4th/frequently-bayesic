pub mod api_tests {
    use crate::api_calls::open_calls;
    use crate::parameter_structs::parameter_structs::PostInformation;
    use actix_web::{test, App};

    // Test to check that the /posts endpoint responds
    #[actix_rt::test]
    async fn test_get_posts_responds() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts)).await;
        let req = test::TestRequest::get().uri("/posts").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_posts_responds_with_json() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts)).await;
        let req = test::TestRequest::get().uri("/posts").to_request();
        let _resp: Vec<PostInformation> = test::read_response_json(&mut app, req).await;

        // if the PostInformation objects serialize succesfully then we can assume the JSON is as expected
        assert!(true);
    }

    // Test to check that an invalid endpoint /wrongEndpoint does not respond
    #[actix_rt::test]
    async fn test_get_posts_fails() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts)).await;
        let req = test::TestRequest::get().uri("/wrongEndpoint").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(!resp.status().is_success());
    }

    // Test to check that the /posts/id endpoint responds
    #[actix_rt::test]
    async fn test_get_posts_id_responds() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts_by_id)).await;
        let req = test::TestRequest::get().uri("/posts/1").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
