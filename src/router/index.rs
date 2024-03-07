use axum::{
    extract::State, 
    response::Html
};

use std::sync::Arc;

use tera::{Tera, Context};

#[axum::debug_handler]
pub async fn index(State(state): State<Arc<Tera>> ) -> Html<String> {

    let mut dummy_context = Context::new();
    dummy_context.insert("dummy", "dummy_value");

    let rendered = state.render("views/index.html", &dummy_context).unwrap();

    Html(rendered)
}