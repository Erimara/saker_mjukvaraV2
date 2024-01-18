use actix_cors::Cors;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::{Key, SameSite};
use actix_web::http::header;
use crate::database::db::connection;
use crate::http_methods::routes;
mod models;
mod http_methods;
mod login;
mod database;
#[actix_web::main]
async fn main() {
    let address = "127.0.0.1:8081";
    let oauth_address = "127.0.0.1:8082";
    let database = connection().await;

    let main_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5500")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .supports_credentials(),
            )
            .wrap(session_middleware())
            .configure(routes::configure_routes)
    })
        .bind(address)
        .expect("Failed to bind main server")
        .run();

    let github_auth_server = HttpServer::new(move || {
        App::new()
            .wrap(session_middleware())
            .configure(routes::configure_github_auth_routes)
    })
        .bind(oauth_address)
        .expect("Failed to bind oauth")
        .run();

    tokio::try_join!(main_server, github_auth_server).expect("Failed to run servers");
}
fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64])
    )
        .cookie_name(String::from("very-secure-cookie"))
        .cookie_secure(false)
        .cookie_http_only(false)
        .cookie_same_site(SameSite::Lax)
        .build()
}

