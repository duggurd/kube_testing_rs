use axum::{
    extract::State, 
    response::Html
};

use std::sync::Arc;

use tera::{Tera, Context};

#[axum::debug_handler]
pub async fn home(State(state): State<Arc<Tera>> ) -> Html<String> {

    let mut dummy_context = Context::new();
    dummy_context.insert("dummy", "dummy_value");

    let rendered = state.render("views/home.html", &dummy_context).unwrap();

    Html(rendered)
}