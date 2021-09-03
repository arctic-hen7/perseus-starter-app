mod templates;
mod error_pages;

use perseus::define_app;

#[derive(perseus::Route)]
pub enum Route {
    #[to("/")]
    Index,
    #[not_found]
    NotFound,
}

define_app! {
    root: "#root",
    route: Route,
    router: {
        Route::Index => [
            "index".to_string(),
            templates::index::template_fn()
        ]
    },
    error_pages: crate::error_pages::get_error_pages(),
    templates: [
        crate::templates::index::get_template::<G>()
    ]
}