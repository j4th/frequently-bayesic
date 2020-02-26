pub mod api_tests {
    use crate::api_calls::open_calls;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_get_posts_responds() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts)).await;
        let req = test::TestRequest::get().uri("/posts").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_posts_fails() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts)).await;
        let req = test::TestRequest::get().uri("/wrongEndpoint").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(!resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_posts_id_responds() {
        let mut app = test::init_service(App::new().service(open_calls::get_posts)).await;
        let req = test::TestRequest::get().uri("/posts/1").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
