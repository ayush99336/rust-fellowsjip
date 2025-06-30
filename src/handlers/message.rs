use axum::{extract::Json as ExtractJson, http::StatusCode, response::Json};
use crate::types::{ApiResponse, SignMessageRequest, SignMessageResponse, VerifyMessageRequest, VerifyMessageResponse};
use crate::utils::{parse_public_key, parse_secret_key, validate_not_empty};
use solana_sdk::signature::{Signature, Signer};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};

/// Sign a message with a private key
pub async fn sign_message(
    ExtractJson(payload): ExtractJson<SignMessageRequest>,
) -> Result<Json<ApiResponse<SignMessageResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    validate_not_empty(&payload.message, "message")
        .map_err(|e| create_error_response(&e))?;
    validate_not_empty(&payload.secret, "secret")
        .map_err(|e| create_error_response(&e))?;

    let keypair = parse_secret_key(&payload.secret)
        .map_err(|e| create_error_response(&e))?;

    let message_bytes = payload.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response_data = SignMessageResponse {
        signature: BASE64_STANDARD.encode(signature.as_ref()),
        public_key: bs58::encode(keypair.pubkey().to_bytes()).into_string(),
        message: payload.message,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response_data),
        message: "Message signed successfully".to_string(),
    }))
}

/// Verify a signed message
pub async fn verify_message(
    ExtractJson(payload): ExtractJson<VerifyMessageRequest>,
) -> Result<Json<ApiResponse<VerifyMessageResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let pubkey = parse_public_key(&payload.pubkey)
        .map_err(|e| create_error_response(&e))?;

    let signature_bytes = BASE64_STANDARD.decode(&payload.signature)
        .map_err(|_| create_error_response("Invalid signature format"))?;

    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| create_error_response("Invalid signature"))?;

    let message_bytes = payload.message.as_bytes();
    let is_valid = signature.verify(pubkey.as_ref(), message_bytes);

    let response_data = VerifyMessageResponse {
        is_valid,
        message: payload.message,
        public_key: payload.pubkey,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response_data),
        message: if is_valid { "Message verification successful" } else { "Message verification failed" }.to_string(),
    }))
}

fn create_error_response(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse {
            success: false,
            data: None,
            message: message.to_string(),
        }),
    )
}
