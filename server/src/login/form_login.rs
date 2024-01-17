use actix_session::Session;
use actix_web::{HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::http::header;
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
           let user_id = found_user.id.map(|id| id.to_string());
           println!("successful login");

           session.insert("user_id", user_id.clone()).expect("Could not insert session");
           let user_session = session.get::<String>("user_id").unwrap_or_default();

           let cookie_value = user_session.unwrap_or_default();
           let cookie = Cookie::build("user_id", cookie_value)
               .http_only(true)
               .max_age(Duration::new(3000, 0))
               .finish();
           println!("Session started with userID{:?}",&user_id);
           HttpResponse::Ok()
               .insert_header(header::ContentType(mime::APPLICATION_JSON))
               .cookie(cookie)
               .json(found_user.id)

       } else {
           println!("invalid password");
           HttpResponse::InternalServerError().finish()
       }
   } else {
       println!("user not found");
       HttpResponse::Ok().finish()
   };
}
