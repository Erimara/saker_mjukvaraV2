use actix_session::{Session};
use actix_web::{HttpResponse};
use actix_web::cookie::{Cookie, time};

pub async fn logout(session: Session) -> HttpResponse{
    let active_session = session.get::<String>("user_id");
    println!("session that is being logged out: {:?}", active_session);
    session.remove("user_id");
    session.clear();
    let expired_cookie = Cookie::build("user_id", "")
        .path("/")
        .expires(time::OffsetDateTime::now_utc())
        .finish();

    HttpResponse::Ok()
        .cookie(expired_cookie)
        .json("logged out")
 }