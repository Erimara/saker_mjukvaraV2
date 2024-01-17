use std::collections::HashMap;
use std::convert::Infallible;
use actix_web::{HttpResponse, web};
use futures_util::TryFutureExt;
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::reqwest::http_client;


#[derive(Debug)]
enum OAuthError {
    UrlParseError(url::ParseError),
}

impl From<url::ParseError> for OAuthError {
    fn from(err: url::ParseError) -> Self {
        OAuthError::UrlParseError(err)
    }
}
pub struct Codes {
    csrf: Option<CsrfToken>,
    pub auth: Option<AuthorizationCode>,
}
pub async fn get_params(query_params: web::Query<HashMap<String, String>>) -> Codes {
    let csrf_token = query_params.get("state").map(|s| CsrfToken::new(s.clone()));
    let auth_code = query_params.get("code").map(|c| AuthorizationCode::new(c.clone()));
    println!("CSRF Token: {:?}", csrf_token);
    println!("Auth Code: {:?}", auth_code);
    Codes { csrf: csrf_token, auth: auth_code }
}

async fn get_client() -> BasicClient{
    let github_client_id = ClientId::new("hidden".to_string());
    let github_client_secret = ClientSecret::new("hidde ".to_string());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).expect("failed to get github auth");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).expect("failed to get github token");
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
        .set_redirect_uri(
            RedirectUrl::new("http://127.0.0.1:8081/auth".to_string())
                .expect("Invalid redirect URL"),
        );

    client
}

pub(crate) async fn setup_oauth() -> Result<HttpResponse, Infallible> {

let client = get_client().await;
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    Ok(HttpResponse::Ok().body("OAuth successful redirect!"))
}
pub async fn do_exchange(authorization_code: AuthorizationCode) -> HttpResponse {
    let client = get_client().await;
    let token_result = client
        .exchange_code(authorization_code)
        .request(http_client)
        .map_err(|err| {
            println!("Token exchange failed: {:?}", err);
            HttpResponse::InternalServerError().body("Token exchange failed.")
        });



    match token_result {
        Ok(token_response) => {
            let access_token = token_response.access_token();
            println!("{:?}", access_token);
            HttpResponse::Ok().body("Token exchange successful!")
        }
        Err(err) => {
            eprintln!("Token exchange failed: {:?}", err);
            HttpResponse::InternalServerError().body("Token exchange failed.")
        }
    }
}
