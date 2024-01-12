use actix_web::{web, HttpResponse};
use bcrypt::{DEFAULT_COST, hash};
use mongodb::{Database};
use crate::user;

use crate::user_post::Post;
use futures_util::stream::TryStreamExt;
use crate::user::User;


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
        web::resource("/posts")
            .route(web::get().to(get_all_posts))
    );
}

async fn register(data: web::Data<Database>, user: web::Json<user::User>) -> HttpResponse {
    println!("called");
    let db = data.get_ref();
    let collection = db.collection::<User>("users");

    let hashed_password= match hash(&user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            println!("Failed to hash password: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let user_with_hash = User{
        email: user.email.clone(),
        password: hashed_password,
    };

    match collection.insert_one(user_with_hash, None).await{
        Ok(result) => {
            println!("Created user with id: {:?}", result.inserted_id);
            HttpResponse::Ok().json(result);
        }
        Err(e) => {
            println!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}
async fn create_post(data: web::Data<Database>, user_post: web::Json<Post>) -> HttpResponse {

    let db = data.get_ref();

    let collection = db.collection::<Post>("posts");
    let post_a_post = user_post.0;

    match collection.insert_one(post_a_post, None).await{
        Ok(result) => {
            println!("Inserted post with id: {:?}", result.inserted_id);
            ();
        }
        Err(e) => {
            println!("Failed to insert post: {:?}", e);
            ();
        }
    }
    HttpResponse::Ok().finish()
}
async fn get_all_posts(data: web::Data<Database>) -> HttpResponse {
    let db = data.get_ref();
    let collection = db.collection::<Post>("posts");
    match collection.find(None, None).await {
        Ok(cursor) => {
            let posts: Vec<Post> = cursor.try_collect().await.unwrap_or_else(|e| {
                println!("Failed to collect posts: {:?}", e);
                Vec::new()
            });
            HttpResponse::Ok().json(posts)
        }
        Err(e) => {
            println!("Failed to get posts: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}