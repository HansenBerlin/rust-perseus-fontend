mod templates;

use perseus::prelude::*;
use sycamore::prelude::view;


#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .index_view(|cx| {
            view! { cx,
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        // Perseus automatically resolves `/.perseus/static/` URLs to the contents of the `static/` directory at the project root
                        link(rel = "stylesheet", href = ".perseus/static/style.css")
                    }
                    body {
                        // Quirk: this creates a wrapper `<div>` around the root `<div>` by necessity
                        PerseusRoot()
                    }
                }
            }
        })
}