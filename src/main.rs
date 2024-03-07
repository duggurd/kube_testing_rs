use std::{error::Error, sync::Arc};
use tera::Tera;
use axum::Router;
use tower_http::services::ServeDir;

mod router;
use router::app;


pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
   
    let tera = match Tera::new("./templates/**/*")  {
        Ok(t) => {
            println!("Succesfully templated tera"); 
            t
        },
        Err(e) => {
            println!("failed to create tera: {}", e);
            ::std::process::exit(1);
        }
    };

    let shared_state = Arc::new(tera);

    let static_files = ServeDir::new("assets");

    let app = Router::new()
        .nest_service("/assets", static_files)
        .with_state(shared_state.clone())
        .nest("/", app(shared_state.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())


}