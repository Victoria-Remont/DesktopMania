use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use chrono::{Duration,Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    role:String,
    exp: usize,
}

pub struct Role{
    user:String,
    admin:String,
}

impl Role{
    pub fn to_string(&self) -> String{
        format!("{}",&self.user)
    }
}

const JWT_SECRET: &[u8] = b"secret";

pub fn create_jwt(user_id: &str, role: &Role) -> Result<String,ServiceError> {
    let expiration = Utc::now().checked_add_signed(Duration::seconds(60)).expect("valid timestamp").timestamp();

    let claims= Claims{
        sub: user_id.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).map_err(|_| ServiceError::JWKSFetchError)
}


pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"))
        .expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}