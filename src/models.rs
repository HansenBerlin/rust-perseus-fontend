pub mod repository {
    use perseus::ReactiveState;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, ReactiveState, Clone)]
    pub struct Repository {
        pub id: String,
        pub commit_count: i32,
        pub created_at: String,
        pub forks_count: i32,
        pub languages_used: Vec<String>,
        pub license: String,
        pub name: String,
        pub primary_language: String,
        pub pull_requests: i32,
        pub stars_count: i32,
        pub watchers: i32,
    }

    #[derive(Serialize, Deserialize, ReactiveState, Clone)]
    pub struct PostRepository {
        pub name: String,
        pub license: String,
    }


    #[rx(alias = "ResponseInfoRx")]
    #[derive(Serialize, Deserialize, ReactiveState, Clone)]
    pub struct ResponseInfo {
        pub time: String,
        pub status: String,
        pub result: Vec<Repository>,
    }



}
