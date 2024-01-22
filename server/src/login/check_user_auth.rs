use std::fs::OpenOptions;
use std::io::Write;
use actix_session::{SessionExt};
use actix_web::cookie::time;
use actix_web::{HttpRequest};

pub async fn is_logged_in(req: HttpRequest) -> bool {
    let session = req.get_session();
    let auth_admin = session.get::<bool>("is_admin");
    let auth_user = session.get::<String>("user_id");

    if let Some(cookie) = req.cookie("oauth") {
        let oauth_value = cookie.value().to_string();
        println!("oauth: {}", oauth_value);
        return true
    }
    if let (Ok(Some(is_user)), Ok(Some(is_admin))) = (auth_user, auth_admin) {
        println!("is user: {:?}", is_user);
        println!("is admin: {:?}", is_admin);
        true
    } else {
        println!("is not logged in");
        false
    }
}

pub async fn is_admin(req: HttpRequest) -> bool {
    let session = req.get_session();
    let auth_admin = session.get::<bool>("is_admin");
    println!("admin?: {:?}", auth_admin);
    if let Some(is_admin) = auth_admin.expect("No admin found") {
        is_admin
    } else {
        false
    }
}

pub async fn collect_data(req: HttpRequest){
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("logs.txt");

    let timestamp = time::OffsetDateTime::now_utc();
    let user_agent = req.headers().get("User-Agent");
    let ip = req.peer_addr();
    let method = req.method();
    let path = req.uri().path();
    let cookies = req.cookies().expect("No cookies found");

    let log = format!(
        "Timestamp: {:?}. \
        user-agent: {:?}. \
        ip-address: {:?}. \
        method: {:?}. \
        path: {:?},\
        cookies: {:?}. ",
        timestamp,
        user_agent,
        ip,
        method,
        path,
        cookies
    );
    file.unwrap().write_all(log.as_str().as_ref()).expect("Could not log data");
}