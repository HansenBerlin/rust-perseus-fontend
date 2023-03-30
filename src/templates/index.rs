use perseus::prelude::*;
use sycamore::prelude::*;
use models::repository;
use crate::models;
use crate::models::repository::{PostRepository, Repository, ResponseInfo};
use sycamore::suspense::Suspense;
use reqwasm::http::Request;

fn index_page<G: Html>(cx: Scope) -> View<G> {
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
                        button(class = "button-cst"){
                            "Test Button"
                        }
                    }
                }
                div(class = "features"){

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

const API_BASE_URL: &str = "http://localhost:8081/key/repositories";

async fn fetch_repos() -> Result<Vec<Repository>, reqwasm::Error> {
    let resp = reqwasm::http::Request::get(API_BASE_URL)
        .send()
        .await
        .unwrap();

    //let resp = Request::get(API_BASE_URL).send().await?;
    //let body = resp.json::<Visits>().await?;

    let body: ResponseInfo = resp.json::<ResponseInfo>().await.unwrap();
    Ok(body.result)
}

fn create_feature_card<G: Html>(cx: Scope) -> View<G> {
    let repos = fetch_repos().await.unwrap();
    let mut all = Vec::with_capacity(repos.capacity());
    for repo in repos {
        all.push(view! { cx,
            div(class = "card"){
                div(class = "feature-title"){ "Status: " (repo.name) }
                div(class = "feature-text"){ "Time " (repo.primary_language) }
            }
        });
    }
    let markup = View:: new_fragment(all);
    (markup)
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}