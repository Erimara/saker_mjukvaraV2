use actix_session::{SessionExt};
use actix_web::guard::GuardContext;

pub fn verify_admin(ctx: &GuardContext) -> bool {
    let auth_admin = ctx.get_session().get::<bool>("is_admin");
    println!("{:?}", auth_admin);
    if let Ok(Some(is_admin)) = auth_admin {
        if is_admin {
            println!("is admin");
            true
        } else {
            println!("is not admin");
            false
        }
    } else {
        println!("is not logged in");
        false
    }
}

pub fn verify_user(ctx: &GuardContext) -> bool {
    let auth_user = ctx.get_session().get::<bool>("user_id");
    println!("{:?}", auth_user);
    if let Ok(Some(is_user)) = auth_user {
        if is_user {
            println!("is user");
            true
        } else {
            println!("is not user");
            false
        }
    } else {
        println!("user is not logged in");
        false
    }
}
