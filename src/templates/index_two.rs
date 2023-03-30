use std::borrow::Borrow;
use perseus::prelude::*;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use sycamore::prelude::*;
use crate::models::repository::{Repository, ResponseInfo};

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
        div {
            header(style = "position: fixed; width: 100%;")
            {
                div(style = "background-color: #222;", class = "navbar"){
                    div(class = "container nav-container"){
                        a(class = "topbar-link"){
                            div(class = "topbar-brandwrapper"){
                                img(class = "topbar-brandimage", src = ".perseus/static/tangram.svg"){
                                    div(class = "topbar-brandtitle"){
                                        "Title"
                                    }
                                }
                            }
                        }
                        div(style = "right: 7vw; position: fixed; top: 25px;"){
                            input(class="checkbox", type="checkbox", style="right:0;", name="", id=""){}
                            div(class="hamburger-lines"){
                                span (class="line line1"){}
                                span (class="line line2"){}
                                span (class="line line3"){}
                            }
                            div(class="menu-items"){
                                li{a (href="#"){"Home"}}
                                li{a (href="#"){"Home2"}}
                                li{a (href="#"){"Home3"}}
                                li{a (href="#"){"Home4"}}
                                li{a (href="#"){"Home5"}}
                            }
                        }
                    }
                }
            }
            main(style = "padding-top: 16vh;"){
                div(class = "her-wrapper"){
                    div(class = "hero-title"){
                        "Hello"
                    }
                    div(class = "hero-subtitle"){
                        "Tangram is a programmable build system and package manager in which all dependencies are specified explicitly and pinned with a lockfile. You get the exact same versions of every package on every machine, so your builds are simple, reproducible, cacheable, and distributable"
                    }
                    div(class = "hero-buttons"){


                    }
                }
                div(class = "features"){
                    (create_feature_card(cx, response.get().result.clone()))
                }
            }
            footer{
                div(style = "background: #222;"){
                    div(class = "footer-inner"){
                        div(class = "topbar-brandwrapper"){
                            img(class = "topbar-brandimage", src = ".perseus/static/tangram.svg"){
                                div(class = "topbar-brandtitle"){
                                    "Title"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


fn create_feature_card<G: Html>(cx: Scope<'_>, repos: Vec<Repository>) -> View<G> {
    //let repos = fetch_repos().await;
    let mut all = Vec::with_capacity(repos.capacity());
    for repo in repos.clone() {
        all.push(view! { cx,
            div(class = "card"){
                div(class = "feature-title"){ "Status: " (repo.name.clone()) }
                div(class = "feature-text"){ "Time " (repo.primary_language.clone()) }
            }
        });
    }
    let markup = View:: new_fragment(all);
    markup
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head(head)
        .build()
}


#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<reqwest::Error>> {

    let body = perseus::utils::cache_fallible_res(
        "ipify",
        || async {
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
            Ok::<ResponseInfo, reqwest::Error>(test[0].clone())
        },
        true,
    )
        .await?;


    Ok(IndexPageState {
        response: body,
    })
}