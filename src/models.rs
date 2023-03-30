pub mod repository {
    use perseus::ReactiveState;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, ReactiveState, Clone)]
    pub struct Repository {
        pub commit_count: i32,
        pub created_at: String,
        pub forks_count: i32,
        pub id: String,
        pub languages_used: Vec<String>,
        pub licence: String,
        pub name: String,
        pub primary_language: String,
        pub pull_requests: i32,
        pub stars_count: i32,
        pub watchers: i32,
    }

    #[derive(Serialize, Deserialize, ReactiveState, Clone)]
    pub struct PostRepository {
        pub name: String,
        pub licence: String,
    }




    #[rx(alias = "ResponseInfoRx")]
    #[derive(Serialize, Deserialize, ReactiveState, Clone)]
    pub struct ResponseInfo {
        pub result: Vec<Repository>,
        pub status: String,
        pub time: String,
    }
}
