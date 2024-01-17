use std::collections::HashMap;
use actix_web::{HttpResponse, web};
use crate::login::github::{do_exchange, get_params};
pub(crate) async fn o_auth_redirect(query_params: web::Query<HashMap<String, String>>) -> HttpResponse {
        println!("triggered");
        let codes = get_params(query_params).await;
        do_exchange(codes.auth.expect("Authorization code not found")).await;
        HttpResponse::Ok().body("Auth check")
}