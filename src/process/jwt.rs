use crate::JwtAlg;
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub aud: String,
    pub sub: String,
    pub exp: i64,
}

impl JwtClaims {
    pub fn new(aud: String, sub: String, exp: i64) -> Self {
        Self { aud, sub, exp }
    }
}

pub fn jwt_encode(claims: &JwtClaims, alg: JwtAlg) -> Result<String> {
    //b64_url_encode(header) + "." + b64_url_encode(payload) + "." + b64_url_encode(key)
    let (header, key) = match alg {
        JwtAlg::HS256 => (
            Header::new(Algorithm::HS256),
            EncodingKey::from_secret(JWT_SECRET),
        ),
        JwtAlg::HS384 => unimplemented!(),
        JwtAlg::HS512 => unimplemented!(),
        JwtAlg::RS256 => unimplemented!(),
        JwtAlg::RS384 => unimplemented!(),
        JwtAlg::RS512 => unimplemented!(),
        JwtAlg::ES256 => unimplemented!(),
        JwtAlg::ES384 => unimplemented!(),
        JwtAlg::ES512 => unimplemented!(),
    };
    eprintln!("header={header:?}, claims={claims:?}");
    Ok(encode(&header, &claims, &key)?)
}

pub fn jwt_verify(token: &str, aud: &str, sub: &str) -> Result<bool> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[aud.to_string()]);
    validation.sub = Some(sub.to_string());
    validation.set_required_spec_claims(&["exp", "aud", "sub"]);
    let data = decode::<JwtClaims>(token, &DecodingKey::from_secret(JWT_SECRET), &validation)?;
    eprintln!("header={:?}, claims={:?}", data.header, data.claims);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let claims = JwtClaims::new("aud".to_string(), "sub".to_string(), 10000000000);
        let token = jwt_encode(&claims, JwtAlg::HS256).unwrap();
        let valid = jwt_verify(&token, "aud", "sub").unwrap();
        assert!(valid);
    }
}
