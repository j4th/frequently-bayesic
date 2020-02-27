/*
    module to contain all the structs used
    to capture parameters for api calls
*/
pub mod parameter_structs {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct PostId {
        pub postid: u32,
    }
}
