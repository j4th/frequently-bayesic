/*
    module to contain all the structs used
    to capture parameters for api calls
*/
pub mod parameter_structs {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct PostId {
        pub postid: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PostInformation {
        pub title: String,
        pub id: i32,
    }
}
