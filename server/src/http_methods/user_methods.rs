use actix_web::{HttpResponse, web};
use bcrypt::{DEFAULT_COST, hash};
use mongodb::bson::doc;
use mongodb::Database;
use crate::models::user::User;
use crate::utils::validate::email_validation;
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
    let is_valid_email = match email_validation(user.email.clone()){
        Ok(email) => email,
        Err(e) => {
            println!("Non valid email: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let does_email_exist = doc! { "email": &is_valid_email };
    if let Ok(Some(_email_exists)) = collection.find_one(does_email_exist.clone(), None).await {
        return HttpResponse::InternalServerError().finish();
    } else if let Err(e) = collection.find_one(does_email_exist, None).await {
        return HttpResponse::InternalServerError().finish();
    }

    let hashed_user = User{
        id: None,
        email:is_valid_email,
        password: hash_password,
        admin:false
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
