use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};
use crate::models::repository::{Repository, ResponseInfo, ResponseInfoRx};

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[engine_only_fn]
async fn get_build_state(_locale: String) -> AppState {
    AppState {
        // We explicitly tell the first page that no login state has been checked yet
        repos: ResponseInfo {
            status: String::new(),
            time: String::new(),
            result: Vec::new()
        },
    }
}

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    #[rx(nested)]
    pub repos: ResponseInfo,
}

#[cfg(client)]
impl ResponseInfoRx {
    pub async fn login(&self) {
        let response = fetch_repos().await;
        self.result.set(response.result);
    }
}

const API_BASE_URL: &str = "http://localhost:8081/key/repositories";

async fn fetch_repos() -> ResponseInfo {
    let client = reqwest::Client::new();
    let res = client.get(API_BASE_URL)
        .header("Accept","application/json")
        .send()
        .await
        .expect("Error: Database not reachable.");

    let body: ResponseInfo = res.json::<ResponseInfo>().await.unwrap();
    body
}
