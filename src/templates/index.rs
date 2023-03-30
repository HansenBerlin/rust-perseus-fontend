use perseus::prelude::*;
use sycamore::prelude::*;
use models::repository;
use crate::models;
use crate::models::repository::{PostRepository, Repository, ResponseInfo, ResponseInfoRx};
use reqwest;



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

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}