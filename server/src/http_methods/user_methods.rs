use actix_web::{HttpResponse, web};
use bcrypt::{DEFAULT_COST, hash};
use mongodb::Database;
use crate::models::user::User;

pub(crate) async fn register(data: web::Data<Database>, user: web::Json<User>) -> HttpResponse {
    let db = data.get_ref();
    let collection = db.collection::<User>("users");

    let hash_password= match hash(&user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            println!("Failed to hash password: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let hashed_user = User{
        id: None,
        email: user.email.clone(),
        password: hash_password,
    };

    return match collection.insert_one(hashed_user, None).await {
        Ok(result) => {
            println!("Created user with id: {:?}", result.inserted_id);
            HttpResponse::Ok()
                .json(result)
        }
        Err(e) => {
            println!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
