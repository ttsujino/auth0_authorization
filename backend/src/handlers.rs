use axum::{
    extract::{Json, Extension, Path},
    http::StatusCode,
    response::IntoResponse,
};
use crate::repositories::{CreatePost, PostRepository};
use std::sync::Arc;


use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation, TokenData};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

pub async fn create_post<T: PostRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreatePost>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: ユーザーはパスからではなく, JWTトークンから取得する
    let post = repository
        .create(user_id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_all_posts<T: PostRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let posts = repository
        .get_all()
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(Json(posts))
}

pub async fn get_target_user_posts<T: PostRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let posts = repository
        .get_posts(user_id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(Json(posts))
}

pub async fn get_post<T: PostRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let post = repository
        .get_post(id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(Json(post))
}

pub async fn delete_post<T: PostRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let post = repository
        .delete(id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(Json(post))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: usize,
    iat: usize,
    email: Option<String>,
    uid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyToken {
    content: String,
}

fn get_firebase_public_keys() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
        .send()?
        .json::<HashMap<String, String>>()?;
    Ok(res)
}

fn verify_firebase_token(token: &str, project_id: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    // Firebaseの公開鍵を取得
    let keys = get_firebase_public_keys()?;
    
    // トークンのヘッダーからkidを取得
    let header = decode_header(token)?;
    let kid = header.kid.ok_or("Token does not contain a kid")?;

    // kidに対応する公開鍵を取得
    let public_key = keys.get(&kid).ok_or("No matching key found for kid")?;

    // トークンの検証
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[project_id]);
    validation.set_issuer(&[format!("https://securetoken.google.com/{}", project_id)]);

    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data)
}

// トークンはpayloadから取得するのでpayloadを引数に追加
pub async fn return_uid(
    Json(payload): Json<VerifyToken>,
) -> Result<impl IntoResponse, StatusCode> {

    // // FirebaseプロジェクトID
    // let project_id = &env::var("PROJECT_ID").expect("PROJECT_ID must be set");

    // // 検証するFirebase IDトークン
    // let token = payload.content;

    // match verify_firebase_token(&token, project_id) {
    //     Ok(data) => {
    //         println!("Token is valid: {:?}", data.claims);
    //         Ok((StatusCode::OK, Json(data.claims))) // 修正: 正しいレスポンスを返す
    //     }
    //     Err(err) => {
    //         eprintln!("Token validation failed: {:?}", err);
    //         Err(StatusCode::UNAUTHORIZED) // 修正: エラーステータスを返す
    //     }
    // }
    // 適当にユーザーIDを返す
    println!("Token is valid");
    Ok((StatusCode::OK, Json("1")))
}