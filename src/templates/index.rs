use std::borrow::Borrow;
use std::env;
use perseus::prelude::*;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use sycamore::prelude::*;
use crate::models::{Repository, PostRepository, ResponseInfo};
use chrono::*;

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
                                img(class = "topbar-brandimage", src = ".perseus/static/tangram.svg")
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
                        "Git"
                    }
                    div(class = "hero-subtitle"){
                        "Git is a distributed version control system that tracks changes in any set of computer files, usually used for coordinating work among programmers collaboratively developing source code during software development. Its goals include speed, data integrity, and support for distributed, non-linear workflows (thousands of parallel branches running on different systems).

Git was originally authored by Linus Torvalds in 2005 for development of the Linux kernel, with other kernel developers contributing to its initial development. Since 2005, Junio Hamano has been the core maintainer. As with most other distributed version control systems, and unlike most client–server systems, every Git directory on every computer is a full-fledged repository with complete history and full version-tracking abilities, independent of network access or a central server. Git is free and open-source software distributed under the GPL-2.0-only license."
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
                            img(class = "topbar-brandimage", src = ".perseus/static/tangram.svg")
                        }
                    }
                }
            }
        }
    }
}


fn create_feature_card<G: Html>(cx: Scope<'_>, repos: Vec<Repository>) -> View<G> {
    let mut all = Vec::with_capacity(repos.capacity());
    for repo in repos.clone() {
        all.push(view! { cx,
            div(class = "card"){
                div(class = "feature-title"){ "Name: " (repo.name.clone()) ", erstellt am " (repo.created_at.clone()) }
                div(style= "display:flex;"){
                    div(class = "feature-text"){
                        p {"commits " (repo.commit_count.clone()) }
                        p {"forks " (repo.forks_count.clone()) }
                    }
                    div(class = "feature-text"){
                        p {"stars " (repo.stars_count.clone()) }
                        p {"pull requests " (repo.pull_requests.clone()) }
                    }
                    div(class = "feature-text"){
                        p {"watchers " (repo.watchers.clone()) }
                        p {"language " (repo.primary_language.clone()) }
                    }
                }
                p { "languages used: "
                    (create_language_list(cx, repo.languages_used.clone()))
                }
            }
        });
    }
    let markup = View:: new_fragment(all);
    markup
}

fn create_language_list<G: Html>(cx: Scope<'_>, languages: Vec<String>) -> View<G> {
    let mut all = Vec::with_capacity(languages.capacity());
    for language in languages.clone() {
        all.push(view! { cx,
            (slice(language.clone())) ", "
        });
    }
    let markup = View:: new_fragment(all);
    markup
}

fn slice(str: String) -> String{
    let s = String::from(str);
    let s = String::from(&s[5..]);
    s
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .view_with_state(index_page)
        .revalidate_after("5s")
        .should_revalidate_fn(should_revalidate)
        .build_state_fn(get_build_state)
        .head(head)
        .build()
}


#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Git" }
    }
}

#[engine_only_fn]
async fn should_revalidate(
    // This takes the same arguments as request state
    _info: StateGeneratorInfo<()>,
    _req: perseus::Request,
) -> Result<bool, BlamedError<std::convert::Infallible>> {
    // For simplicity's sake, this will always say we should revalidate, but you
    // could make this check any condition
    Ok(true)
}


#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> IndexPageState {

    let base_url = env::var("REST_URL").expect("No url env. variable found");
    let port = env::var("REST_PORT").expect("No port env. variable found");
    let url = format!("http://{}:{}/repositories", base_url, port);
    println!("{}", url);
    let client = reqwest::Client::new();
    let res =match client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .query(&[("timestamp", chrono::Utc::now().timestamp())])
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => {
            return IndexPageState {
                response: ResponseInfo {
                    result: vec![],
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    time: "".to_string(),
                }
            };
        }
    };

    let val = match res.json().await {
        Ok(json) => {
            let body: Value = json;
            body
        },
        Err(e) => { Value::String(String::from("")) }
    };
    let mut test: Vec<ResponseInfo> = serde_json::from_value(val).unwrap_or(vec![]);
    let body = Ok::<ResponseInfo, reqwest::Error>(test[0].clone()).unwrap_or(ResponseInfo {
        result: vec![],
        status: "".to_string(),
        time: "".to_string(),
    });

    IndexPageState {
        response: body,
    }
}