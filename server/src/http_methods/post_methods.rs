use actix_web::{HttpResponse, web};
use futures_util::TryStreamExt;
use mongodb::Database;
use crate::models::user_post::Post;

pub(crate) async fn create_post(data: web::Data<Database>, user_post: web::Json<Post>) -> HttpResponse {

    let db = data.get_ref();

    let collection = db.collection::<Post>("posts");
    let post_a_post = user_post.0;

    return match collection.insert_one(post_a_post, None).await{
        Ok(result) => {
            println!("Inserted post with id: {:?}", result.inserted_id);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            println!("Failed to insert post: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
pub(crate) async fn get_all_posts(data: web::Data<Database>) -> HttpResponse {
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