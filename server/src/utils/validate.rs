use std::fs::OpenOptions;
use std::io::Write;
use actix_web::cookie::time;
use actix_web::HttpRequest;
use validator::{validate_email};

pub fn email_validation(email: String) -> Result<String, &'static str> {
    if validate_email(&email) {
        return Ok(email)
    } else {
        Err("Invalid email")
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