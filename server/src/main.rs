use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod routes;
mod db;
mod user;
mod user_post;

#[actix_web::main]
async fn main() {
    let bind_address = "127.0.0.1:8080";
    let database_url = "hidden";


    let database = db::connection(database_url).await;
    let app = move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(Cors::default().allowed_origin("http://localhost:63342"))
            /*   .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600))*/
            .configure(routes::configure_routes)
    };

    // Start the server
    HttpServer::new(app)
        .bind(bind_address).expect("Failed to bind")
        .run()
        .await
        .expect("Failed to run server");
}
