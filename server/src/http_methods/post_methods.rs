use std::str::FromStr;
use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::TryStreamExt;
use mongodb::bson::{Bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;

use crate::models::user_post::Post;
use crate::login::check_user_auth::{is_admin, is_logged_in};

pub(crate) async fn create_post(data: web::Data<Database>, user_post: web::Json<Post>, req: HttpRequest) -> HttpResponse {
    if is_logged_in(req.clone()).await{
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
    } else {
        println!("Failed to insert post");
        HttpResponse::InternalServerError().finish()
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
pub(crate) async fn get_post_by_id(data: web::Data<Database>, post_id:web::Path<String>) -> HttpResponse {
    let db = data.get_ref();
    let collection = db.collection::<Post>("posts");
    let post_id_bson = doc!{"_id":Bson::ObjectId(ObjectId::from_str(&post_id).unwrap())};
    match collection.find_one(post_id_bson, None).await {
        Ok(post) => {
            HttpResponse::Ok().json(post)
        }
        Err(e) => {
            println!("Failed to get posts: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_post(data: web::Data<Database>, post_id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    if is_admin(req.clone()).await {
        let db = data.get_ref();
        let collection = db.collection::<Post>("posts");

        let post_id_bson = Bson::ObjectId(ObjectId::from_str(&post_id).unwrap());
        let filter = doc! {"_id": post_id_bson};
        match collection.find_one_and_delete(filter, None).await {
            Ok(Some(deleted_post)) => {
                println!("Deleted post with id: {:?}", deleted_post.id);
                HttpResponse::Ok().json(deleted_post)
            }
            Ok(None) => {
                println!("Document not found for deletion");
                HttpResponse::NotFound().finish()
            }
            Err(e) => {
                println!("Error deleting post: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        println!("Failed to delete post");
        HttpResponse::InternalServerError().finish()
    }
}
