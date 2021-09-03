use perseus::Template;
use std::sync::Arc;
use sycamore::prelude::{component, template, GenericNode, Template as SycamoreTemplate};

#[component(IndexPage<G>)]
pub fn index_page() -> SycamoreTemplate<G> {
    template! {
        p { "Welcome to the app!" }
    }
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Arc::new(|_| {
        template! {
            IndexPage()
        }
    })
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index").template(template_fn())
}