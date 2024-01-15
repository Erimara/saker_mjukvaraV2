use actix_cors::Cors;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::{Key};
use actix_web::http::header;
mod routes;
mod db;
mod models;
mod http_methods;
mod login;

#[actix_web::main]
async fn main() {
    let address = "127.0.0.1:8080";
    let database_url = "hidden";
    let database = db::connection(database_url).await;
    let secret = Key::generate();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))

            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:63342")
                    .allowed_methods(vec!["GET","POST"])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .wrap(
              SessionMiddleware::new(CookieSessionStore::default(), secret.clone())
            )
            .configure(routes::configure_routes)
    })
        .bind(address).expect("Failed to bind")
        .run()
        .await
        .expect("Failed to run server");
}
