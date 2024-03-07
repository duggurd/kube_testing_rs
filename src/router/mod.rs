use axum::{
    routing::get, 
    Router
};

use tera::Tera;
use std::sync::Arc;

mod home;
mod services;
mod index;

use index::index;
use home::home;
use services::services;

pub fn app(shared_state: Arc<Tera>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/home", get(home))
        .route("/services", get(services))
        .with_state(shared_state.clone())
}