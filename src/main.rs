use actix_web::{web, App, HttpServer};
use log::info;
use utoipa::OpenApi;

use crate::handlers::ApiDoc;

use utoipa_swagger_ui::SwaggerUi;
use crate::config::establish_connection;

mod models;
mod handlers;
mod routes;
mod repository;
mod config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init_logging();
    info!("Starting HTTP server at http://localhost:8000");
    let openapi = ApiDoc::openapi();
    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone()),
            )
            .service(handlers::get_all_user)
            .service(handlers::get_user_by_id)
            .service(handlers::create_user)
            .service(handlers::delete_user_by_id)
            .service(handlers::calculate_possible_combinations)
    })
        .bind(("localhost", 8000))?
        .run()
        .await
}
