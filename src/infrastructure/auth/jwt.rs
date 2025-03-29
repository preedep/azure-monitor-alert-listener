use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: usize,
    pub iss: String,
    pub iat: usize,
    pub nbf: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtVerifier {
    #[serde(skip)]
    cache: Arc<RwLock<HashMap<String, (u64, Vec<Jwk>)>>>, // Use timestamp instead of Instant, and skip serialization
    #[serde(skip)] // Exclude the client from serialization/deserialization
    client: Client,
    #[serde(with = "humantime_serde")] // Serialize/deserialize Duration using human-readable format
    ttl: Duration,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
    pub kty: String,
    pub alg: String,
    #[serde(rename = "use")]
    pub use_: String,
}

impl JwtVerifier {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            client: Client::new(),
            ttl: Duration::from_secs(3600),
        }
    }

    pub async fn verify(
        &self,
        token: &str,
        tenant_id: &str,
        expected_aud: &str,
    ) -> Result<Claims, String> {
        let jwks = self.get_jwks(tenant_id).await?;

        let header = decode_header(token).map_err(|e| e.to_string())?;
        let kid = header.kid.ok_or("Missing kid")?;

        let jwk = jwks
            .iter()
            .find(|j| j.kid == kid)
            .ok_or("Matching JWK not found")?;

        let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| format!("Bad RSA: {}", e))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[expected_aud]);

        let decoded = decode::<Claims>(token, &key, &validation)
            .map_err(|e| format!("JWT decode error: {}", e))?;

        Ok(decoded.claims)
    }

    async fn get_jwks(&self, tenant_id: &str) -> Result<Vec<Jwk>, String> {
        let mut map = self.cache.write().await;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(); // Use a Unix timestamp

        // Use the Unix timestamp to compare
        if let Some((ts, jwks)) = map.get(tenant_id) {
            if now - *ts < self.ttl.as_secs() {
                return Ok(jwks.to_vec());
            }
        }

        let url = format!("https://login.microsoftonline.com/{tenant_id}/discovery/v2.0/keys");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let jwks: Jwks = resp.json().await.map_err(|e| e.to_string())?;
        map.insert(tenant_id.to_string(), (now, jwks.keys.clone()));

        Ok(jwks.keys)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Jwks {
    keys: Vec<Jwk>,
}
