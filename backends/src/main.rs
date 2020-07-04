//! backends

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use {
    chrono::prelude::*,
    cookie::{Cookie, SameSite},
    http_types::Mime,
    indoc::indoc,
    jsonwebtoken::{encode, Algorithm, EncodingKey, Header},
    serde::{Deserialize, Serialize},
    std::{env, fs, str::FromStr},
    tide::{Body, Request, Response, Result as TideResult, StatusCode},
    time::Duration,
};
// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

static DEFAULT_PORT: &str = "8085";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    input: Option<LoginData>,
    refresh_token: String,
    work_token: Option<String>,
    view_token: Option<String>,
    message: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    femme::with_level(tide::log::Level::Info.to_level_filter());

    let port = env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.into());
    let addr = format!("0.0.0.0:{}", port);

    let mut app = tide::new();

    // for api.~
    app.at("/api").get(json_handler);

    // for id.~
    app.at("/login").post(login_handler);
    // app.at("/auth").post(auth_handler);
    // app.at("/register").post(register_handler);

    // static files anymore?
    // app.at("/src").serve_dir("src/")?;

    // dummy / catch-all
    app.at("/").get(catch_all);
    app.at("*").all(catch_all);

    println!("[backends] listening on: {}", addr);
    app.listen(addr).await?;
    Ok(())
}

async fn catch_all(_req: Request<()>) -> TideResult {
    let body = indoc!(
        r#"
        { "welcome": "this is a JSON only server" }
    "#
    );
    let mut resp = Response::new(StatusCode::Ok);
    let mime = Mime::from_str("application/json; charset=utf-8").unwrap();
    resp.set_content_type(mime);
    resp.set_body(body);
    Ok(resp)
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    body: String,
    token: String,
}

async fn json_handler(_req: Request<()>) -> TideResult {
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("some-key-id-here".to_owned());
    let utc: DateTime<Utc> = Utc::now();
    let iat = utc.timestamp();
    let exp = iat + (60 * 60);
    let my_claims = Claims {
        aud: "id".into(),
        iss: "id".into(),
        sub: "user-id".into(),
        iat,
        exp,
    };

    let secret: String = fs::read_to_string("../data/secrets/refresher.key")?.parse()?;
    let key = secret.trim().as_bytes();
    println!("key = {}", secret.trim());
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(&key))?;
    let message = Message {
        body: "hello, world!".into(),
        token,
    };

    let mut resp = Response::new(StatusCode::Ok);
    let mime = Mime::from_str("application/json; charset=utf-8").unwrap();
    resp.set_content_type(mime);

    resp.set_body(Body::from_json(&message)?);
    Ok(resp)
}

async fn login_handler(mut req: Request<()>) -> TideResult {
    let refresh_token = req.cookie("__Secure-RT");
    // let workToken = req.cookie("__Secure-WT");
    // let viewToken = req.cookie("__Secure-VT");
    match refresh_token {
        Some(cookie) => {
            let lr = LoginResponse {
                input: None,
                refresh_token: cookie.value().into(),
                work_token: None,
                view_token: None,
                message: Some("already logged in".into()),
            };
            let mut resp = Response::new(StatusCode::Ok);
            let mime = Mime::from_str("application/json; charset=utf-8").unwrap();
            resp.set_content_type(mime);
            resp.set_body(Body::from_json(&lr)?);
            Ok(resp)
        }
        None => {
            let login_data: LoginData = req.body_json().await?;
            let max_age = Duration::hours(1);
            let refresh_token = create_token(&login_data.email, max_age.whole_seconds())?;
            let refresh_cookie = Cookie::build("__Secure-RT", refresh_token.clone())
                .domain("id.unicorn.test")
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .max_age(max_age)
                .finish();
            let lr = LoginResponse {
                input: Some(login_data),
                refresh_token: refresh_token,
                work_token: None,
                view_token: None,
                message: Some("already logged in".into()),
            };

            let mut resp = Response::new(StatusCode::Ok);
            let mime = Mime::from_str("application/json; charset=utf-8").unwrap();
            resp.set_content_type(mime);
            resp.insert_cookie(refresh_cookie);
            resp.set_body(Body::from_json(&lr)?);
            Ok(resp)
        }
    }
}

fn create_token(sub: &str, max_age: i64) -> Result<String, tide::Error> {
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("secret:refresher.key".to_owned());
    let utc: DateTime<Utc> = Utc::now();
    let iat = utc.timestamp();
    let exp = iat + max_age;
    let my_claims = Claims {
        aud: "id".into(),
        iss: "id".into(),
        sub: sub.into(),
        iat,
        exp,
    };

    let secret: String = fs::read_to_string("../data/secrets/refresher.key")?.parse()?;
    let key = secret.trim().as_bytes();
    println!("key = {}", secret.trim());
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(&key))?;
    Ok(token)
}
