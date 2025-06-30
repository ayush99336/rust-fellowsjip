use axum::response::Json;
use crate::types::{ApiResponse, KeypairResponse};
use solana_sdk::signature::{Keypair, Signer};

/// Generate a new Solana keypair
/// Returns both the public key and secret key in base58 format
pub async fn generate_keypair() -> Json<ApiResponse<KeypairResponse>> {
    let keypair = Keypair::new();
    let public_key = bs58::encode(keypair.pubkey().to_bytes()).into_string();
    let secret_key = bs58::encode(keypair.to_bytes()).into_string();

    Json(ApiResponse {
        success: true,
        data: Some(KeypairResponse {
            public_key,
            secret_key,
        }),
        message: "Keypair generated successfully".to_string(),
    })
}
