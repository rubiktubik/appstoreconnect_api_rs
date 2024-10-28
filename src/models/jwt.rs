use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("error creating jwt, reason {0}")]
    CreationError(String),
    #[error("error getting jwt")]
    CannotGetJwt,
}

// Claims structure for the JWT payload
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    // The token’s expiration time in Unix epoch time. Tokens that expire more than 20 minutes into the future are not valid except for resources listed in Determine the Appropriate Token Lifetime.
    exp: usize,
    // The token’s creation time, in UNIX epoch time, for example, 1528407600
    iat: usize,
    // Your issuer ID from the API Keys page in App Store Connect, for example, 57246542-96fe-1a63-e053-0824d011072a
    iss: String,
}

pub fn generate_jwt(
    key_id: &str,
    issuer_id: &str,
    p8_file_path: PathBuf,
) -> Result<String, JwtError> {
    // Header with a key ID
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(key_id.to_owned());

    // Current time and expiration time (e.g., 20 minutes from now)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| JwtError::CreationError("Cannot convert time".to_owned()))?
        .as_secs() as usize;
    let exp = now + (20 * 60);

    // Your claims
    let claims = Claims {
        iss: issuer_id.to_owned(),
        exp,
        aud: "appstoreconnect-v1".to_string(),
        iat: now,
    };

    // Load the private key (.p8 file)
    let encoding_key = EncodingKey::from_ec_pem(
        std::fs::read(p8_file_path)
            .map_err(|_| {
                JwtError::CreationError("Cannot load p8 file for creating encoding key".to_owned())
            })?
            .as_ref(),
    )
    .map_err(|_| JwtError::CreationError("Cannot create encoding key from p8 file".to_owned()))?;

    // Encode the JWT
    let token = encode(&header, &claims, &encoding_key).map_err(|_| {
        JwtError::CreationError(
            "error on creating jwt from header,claims and encoding key".to_owned(),
        )
    })?;

    Ok(token)
}
