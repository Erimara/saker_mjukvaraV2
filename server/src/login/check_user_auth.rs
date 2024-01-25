use actix_session::{SessionExt};
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