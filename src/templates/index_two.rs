use std::borrow::Borrow;
use perseus::prelude::*;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use sycamore::prelude::*;
use crate::models::repository::ResponseInfo;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    response: ResponseInfo,
}

fn index_page<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    IndexPageStateRx {
        response,
    }: &'a IndexPageStateRx,
) -> View<G> {



    view! { cx,
        p { (format!("IP address of the server was: {}", response.get().time)) }
        p { (format!("IP address of the server was: {}", response.get().status)) }
        p { (format!("IP address of the server was: {}", response.get().result[0].clone().name)) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .build()
}

fn unescape(s: &str) -> serde_json::Result<String> {
    serde_json::from_str(&format!("\"{}\"", s))
}

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<reqwest::Error>> {

    let body = perseus::utils::cache_fallible_res(
        "ipify",
        || async {
            // This just gets the IP address of the machine that built the app
            //let res = reqwest::get("http://localhost:8088/repositories").await.unwrap().json().await?;
            let client = reqwest::Client::new();
            let res = client
                .get("http://localhost:8088/repositories")
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .send()
                .await
                .expect("error");
            let val = match res.json().await {
                Ok(json) => {
                    let body: Value = json;
                    body
                },
                Err(e) => panic!("error")
            };
            let mut test: Vec<ResponseInfo> = serde_json::from_value(val).unwrap();

            //println!("{:?}", response);
            Ok::<ResponseInfo, reqwest::Error>(test[0].clone())
        },
        true,
    )
        .await?;
    // Note that `?` is able to convert from `reqwest::Error` -> `BlamedError<reqwest::Error>`


    Ok(IndexPageState {
        response: body,
    })
}