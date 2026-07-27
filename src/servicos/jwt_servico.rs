use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usuário identificado pelo token)
    exp: usize, // Expiry (tempo de expiração do token)
}

pub fn gerar_token_jwt(adm_id: u64) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: adm_id.to_string(),
        exp: expiration_time as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("your_secret_key".as_ref())).unwrap()
}

pub fn verify_token(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret("your_secret_key".as_ref()), &Validation::default()).is_ok()
}