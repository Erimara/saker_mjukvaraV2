use actix_web::{web};
use crate::http_methods::post_methods::{create_post, delete_post};
use crate::http_methods::post_methods::get_all_posts;
use crate::http_methods::user_methods::register;
use crate::login::form_login::form_login;
use crate::login::logout::logout;
pub fn configure_routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::resource("/register")
            .route(web::post().to(register))
    );
    cfg.service(
        web::resource("/create_post")
            .route(web::post().to(create_post))
    );
    cfg.service(
        web::resource("/delete_post/{post_id}")
            .route(web::delete().to(delete_post))
    );
    cfg.service(
        web::resource("/posts")
            .route(web::get().to(get_all_posts))
    );
    cfg.service(
        web::resource("/login")
            .route(web::post().to(form_login))
    );
    cfg.service(
        web::resource("/logout")
            .route(web::delete().to(logout))
    );


}
