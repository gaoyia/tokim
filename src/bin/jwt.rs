use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_token() -> Result<String, jsonwebtoken::errors::Error> {
    // 设置过期时间为一小时
    let expiration_time = Utc::now() + Duration::hours(1);

    // 设置payload claim
    let claims = Claims {
        sub: "user123".to_owned(),
        exp: expiration_time.timestamp() as usize,
    };

    // 使用HS256算法，使用你的自定义密钥，生成token
    let key = b"your-secret-key";
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key))?;
    Ok(token)
}

fn validate_token(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let key = b"your-secret-key";
    let validation = Validation::default();
    decode::<Claims>(token, &DecodingKey::from_secret(key), &validation)?;
    Ok(())
}

fn main() {
    let token = generate_token().expect("Failed to generate token.");
    println!("Token: {}", token);

    match validate_token(&token) {
        Ok(_) => println!("Token is valid."),
        Err(err) => eprintln!("Token is invalid: {}", err),
    }
}