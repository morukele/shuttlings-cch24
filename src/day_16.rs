use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{cookie::Cookie, get, http::header::COOKIE, post, web, HttpRequest, HttpResponse};
use jsonwebtoken::{decode_header, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 60);
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,             // Subject of the JWT, can represent the user or session
    exp: usize,              // Expiration timestamp
    data: serde_json::Value, // Arbitrary JSON data
}

#[post("/16/wrap")]
pub async fn wrap(data: web::Json<serde_json::Value>) -> HttpResponse {
    let data = data.into_inner();

    // set expiration to one hour from not
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600;

    // Create and encode JWT from claims
    let cliams = JwtClaims {
        sub: "gift".to_string(),
        exp,
        data,
    };
    let token = jsonwebtoken::encode(&Header::default(), &cliams, &KEYS.encoding).unwrap();

    // Create a Set-Cookie header
    let cookie = Cookie::build("gift", token).finish();

    HttpResponse::Ok().cookie(cookie).finish()
}

#[get("/16/unwrap")]
pub async fn unwrap(req: HttpRequest) -> HttpResponse {
    if req.headers().get(COOKIE).is_none() {
        return HttpResponse::BadRequest().finish();
    }

    if let Some(gift_token) = req.cookie("gift") {
        let token_data = jsonwebtoken::decode::<JwtClaims>(
            gift_token.value(),
            &KEYS.decoding,
            &Validation::default(),
        );

        match token_data {
            Ok(claims) => {
                let msg = claims.claims.data;
                HttpResponse::Ok().json(msg)
            }
            Err(_) => HttpResponse::BadRequest().finish(),
        }
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[post("/16/decode")]
pub async fn decode(jwt: String) -> HttpResponse {
    // decode claim
    let pem = fs::read("./src/certs/day16_santa_public_key.pem").expect("failed to read pem files");
    let key = DecodingKey::from_rsa_pem(&pem);

    let Ok(key) = key else {
        // could not create decoding key from PEM file
        return HttpResponse::InternalServerError().finish();
    };

    // return error for invalid header
    let Ok(_header) = decode_header(&jwt) else {
        println!("bad request line 101");
        return HttpResponse::BadRequest().finish();
    };

    // configure validators
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.algorithms = vec![Algorithm::RS256, Algorithm::RS512];
    validation.set_required_spec_claims(&[""]);

    let claims = jsonwebtoken::decode::<serde_json::Value>(&jwt, &key, &validation).map_err(|e| {
        println!("Claims error: {:?}", e);
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                HttpResponse::Unauthorized().finish()
            }
            _ => HttpResponse::BadRequest().finish(),
        }
    });
    println!(" claims: {:#?}", claims);
    match claims {
        Ok(claims) => HttpResponse::Ok().json(claims.claims),
        Err(err) => err,
    }
}
