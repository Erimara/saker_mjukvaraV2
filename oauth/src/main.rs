#[macro_use]
extern crate serde_derive;

use actix_cors::Cors;
use actix_session::{Session, SessionMiddleware};
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_session::storage::CookieSessionStore;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl, Scope};

use actix_web::cookie::{Cookie, Key, SameSite, time};
use actix_web::cookie::time::Duration;
use actix_web::http::header;
use serde_json::json;


#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        let github_client_id = ClientId::new("[your client id here]".to_string());
        let github_client_secret = ClientSecret::new("[your secret here]".to_string());
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).expect("failed to get github auth");
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).expect("failed to get github token");

        let client = BasicClient::new(
            github_client_id,
            Some(github_client_secret),
            auth_url,
            Some(token_url),
        )
            .set_redirect_uri(
                RedirectUrl::new("http://127.0.0.1:8083/auth".to_string())
                    .expect("Invalid redirect URL"),
            );

        App::new()
            .app_data(web::Data::new(AppState { oauth: client }))
            .wrap(cookie())
            .configure(configure_github_auth_routes)
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5500")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                    .supports_credentials(),
            )

    })
        .bind("127.0.0.1:8083")
        .expect("Can not bind to port 8083")
        .run()
        .await
        .expect("Could not start oauth server");
}

fn cookie() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64])
    )
        .cookie_name(String::from("oauth-secure-cookie"))
        .cookie_secure(false)
        .cookie_http_only(false)
        .cookie_same_site(SameSite::Lax)
        .build()
}
struct AppState {
    oauth: BasicClient,
}

async fn login(data: web::Data<AppState>) -> HttpResponse {
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Ok().json(json!({
        "redirect_url": authorize_url.to_string(),
        "csrf_state": csrf_state.secret(),
    }))
}

async fn logout(session: Session) -> HttpResponse {
    session.remove("login");
    let expired_cookie = Cookie::build("oauth", "")
        .path("/")
        .expires(time::OffsetDateTime::now_utc())
        .finish();
    let html = html_redirect();
    let response = HttpResponse::Ok()
        .cookie(expired_cookie)
        .insert_header(header::ContentType(mime::TEXT_HTML))
        .body(html);
    response
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
    #[serde(default)]
    scope: String,
}
async fn auth(
    session: Session,
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());
    let _scope = params.scope.clone();

    data.oauth.exchange_code(code);
    session.insert("login", true).unwrap();



    let cookie_value = state.secret();
    let cookie = Cookie::build("oauth", cookie_value)
        .http_only(true)
        .max_age(Duration::new(3000, 0))
        .finish();
    let html = html_redirect();

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .cookie(cookie)
        .body(html);
    response
}
pub fn configure_github_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::get().to(login))
    );
    cfg.service(
        web::resource("/logout")
            .route(web::get().to(logout))
    );
    cfg.service(
        web::resource("/auth")
            .route(web::get().to(auth))
    );
}
fn html_redirect() -> String {
    let link = "http://127.0.0.1:5500/client/html/index.html";
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="refresh" content="0;url={}">
            <title>Redirecting...</title>
        </head>
        <body>
            <p>If you are not redirected,<a href="/{}"> Click here{}</a>
        </body>
        </html>
    "#, link, link, link);
    html
}