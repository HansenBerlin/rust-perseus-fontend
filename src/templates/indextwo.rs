use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            header
            {
                div(style = "background-color: #222;", class = "navbar"){
                    div(class = "container nav-container")
                    {
                        a(class = "topbar-link"){
                            div(class = "topbar-brandwrapper"){
                                img(class = "topbar-brandimage", src = ".perseus/static/tangram.svg"){
                                    div(class = "topbar-brandtitle"){
                                        "Title"
                                    }
                                }
                            }
                        }
                    input(class="checkbox", type="checkbox", name="", id=""){}
                        div(class="hamburger-lines"){
                            span (class="line line1"){}
                            span (class="line line2"){}
                            span (class="line line3"){}
                        }
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
            main{

                div(class = "hero-title"){
                    "Hello"
                }
                div(class = "hero-subtitle"){
                    "Tangram is a programmable build system and package manager in which all dependencies are specified explicitly and pinned with a lockfile. You get the exact same versions of every package on every machine, so your builds are simple, reproducible, cacheable, and distributable"
                }
                button(class = "button-cst"){
                    "Test Button"
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus2!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("indextwo").view(index_page).head(head).build()
}