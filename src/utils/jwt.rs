use axum::http::StatusCode;
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn encode_jwt(id: Uuid) -> Result<String, StatusCode>{
    // Retrieve the JWT secret from environment variables
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Create JWT claims with subject, expiration, and issued at times
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::days(1)).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claims = JWTClaims {
        sub: id.to_string(),
        exp,
        iat,
    };
    // Encode the JWT with the claims and secret key
    return encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<JWTClaims>, StatusCode>{
    // Retrieve the JWT secret from environment variables
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Decode the JWT using the secret key and return the token data
    let token_data: Result<TokenData<JWTClaims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED);
    // If decoding is successful, return the token data
    return token_data;
}