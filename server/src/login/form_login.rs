use actix_session::Session;
use actix_web::{HttpResponse, web};
use bcrypt::verify;
use mongodb::bson::doc;
use mongodb::Database;
use crate::models::user::User;

pub async fn form_login(data: web::Data<Database>, user: web::Json<User>, session: Session) -> HttpResponse{
    let db = data.get_ref();
    let collection = db.collection::<User>("users");

   return if let Some(found_user) = collection.find_one(doc! {"email": &user.email}, None)
       .await.unwrap() {
       if verify(&user.password, &found_user.password).unwrap() {
           println!("success");
           session.insert("user_id", &found_user).unwrap();
           HttpResponse::Ok().json(found_user)
       } else {
           println!("invalid password");
           HttpResponse::InternalServerError().finish()
       }
   } else {
       println!("user not found");
       HttpResponse::Ok().finish()
   };
}
