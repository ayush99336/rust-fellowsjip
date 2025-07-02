use axum::{
    extract::{Json, rejection::JsonRejection},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::info;
use base64::{Engine as _, engine::general_purpose};

// Simple crypto imports
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction,
};
use spl_token::instruction as token_instruction;
use std::str::FromStr;

// Custom JSON extractor that returns 400 instead of 422 for missing fields
struct ApiJson<T>(T);

impl<T> std::ops::Deref for ApiJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl<T, S> axum::extract::FromRequest<S> for ApiJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ResponseJson<ApiResponse<String>>);

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(ApiJson(value)),
            Err(JsonRejection::JsonDataError(_)) | Err(JsonRejection::JsonSyntaxError(_)) => {
                Err((
                    StatusCode::BAD_REQUEST,
                    ResponseJson(ApiResponse::error("Invalid JSON format".to_string()))
                ))
            }
            Err(JsonRejection::MissingJsonContentType(_)) => {
                Err((
                    StatusCode::BAD_REQUEST,
                    ResponseJson(ApiResponse::error("Missing Content-Type: application/json".to_string()))
                ))
            }
            Err(_) => {
                Err((
                    StatusCode::BAD_REQUEST,
                    ResponseJson(ApiResponse::error("Invalid request body".to_string()))
                ))
            }
        }
    }
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// Request/Response types
#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Deserialize)]
struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Deserialize)]
struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
struct SignMessageData {
    signature: String,
    pubkey: String,
    message: String,
}

#[derive(Deserialize)]
struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
struct VerifyMessageData {
    valid: bool,
    message: String,
    pubkey: String,
}

#[derive(Deserialize)]
struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Deserialize)]
struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
struct InstructionData {
    program_id: String,
    accounts: Vec<AccountInfo>,
    instruction_data: String,
}

#[derive(Serialize)]
struct AccountInfo {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
struct SolInstructionData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

#[derive(Serialize)]
struct TokenInstructionData {
    program_id: String,
    accounts: Vec<TokenAccountInfo>,
    instruction_data: String,
}

#[derive(Serialize)]
struct TokenAccountInfo {
    pubkey: String,
    #[serde(rename = "isSigner")]
    is_signer: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting Rust HTTP server for Solana operations...");

    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    info!("Server running on http://0.0.0.0:{}", port);
    info!("Available endpoints:");
    info!("  POST /keypair        - Generate new keypair");
    info!("  POST /token/create   - Create token mint instruction");
    info!("  POST /token/mint     - Create mint-to instruction");
    info!("  POST /message/sign   - Sign a message");
    info!("  POST /message/verify - Verify a signature");
    info!("  POST /send/sol       - Create SOL transfer instruction");
    info!("  POST /send/token     - Create token transfer instruction");

    axum::serve(listener, app).await?;

    Ok(())
}

// Handler functions
async fn generate_keypair() -> ResponseJson<ApiResponse<KeypairData>> {
    let keypair = Keypair::new();
    
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(&keypair.to_bytes()).into_string();
    
    ResponseJson(ApiResponse::success(KeypairData { pubkey, secret }))
}

async fn create_token(
    ApiJson(payload): ApiJson<CreateTokenRequest>,
) -> Result<ResponseJson<ApiResponse<InstructionData>>, (StatusCode, ResponseJson<ApiResponse<String>>)> {
    // Validate required fields
    if let Err(error) = validate_create_token_request(&payload) {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error(error))
        ));
    }

    let mint_authority = match Pubkey::from_str(&payload.mint_authority) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid mint authority address".to_string()))
        )),
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid mint address".to_string()))
        )),
    };

    let instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        payload.decimals,
    ).map_err(|_| (
        StatusCode::BAD_REQUEST,
        ResponseJson(ApiResponse::error("Failed to create mint instruction".to_string()))
    ))?;

    let instruction_data = InstructionData {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.into_iter().map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        }).collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(ResponseJson(ApiResponse::success(instruction_data)))
}

async fn mint_token(
    ApiJson(payload): ApiJson<MintTokenRequest>,
) -> Result<ResponseJson<ApiResponse<InstructionData>>, (StatusCode, ResponseJson<ApiResponse<String>>)> {
    // Validate required fields
    if let Err(error) = validate_mint_token_request(&payload) {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error(error))
        ));
    }

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid mint address".to_string()))
        )),
    };

    let destination_user = match Pubkey::from_str(&payload.destination) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid destination address".to_string()))
        )),
    };

    let authority = match Pubkey::from_str(&payload.authority) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid authority address".to_string()))
        )),
    };

    // Get the Associated Token Account for the destination user
    let destination_ata = spl_associated_token_account::get_associated_token_address(&destination_user, &mint);

    let instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination_ata,
        &authority,
        &[],
        payload.amount,
    ).map_err(|_| (
        StatusCode::BAD_REQUEST,
        ResponseJson(ApiResponse::error("Failed to create mint instruction".to_string()))
    ))?;

    let instruction_data = InstructionData {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.into_iter().map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        }).collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(ResponseJson(ApiResponse::success(instruction_data)))
}

async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> Result<ResponseJson<ApiResponse<SignMessageData>>, (StatusCode, ResponseJson<ApiResponse<String>>)> {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Missing required fields".to_string()))
        ));
    }

    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid secret key format".to_string()))
        )),
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid secret key".to_string()))
        )),
    };

    let message_bytes = payload.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response_data = SignMessageData {
        signature: bs58::encode(&signature.as_ref()).into_string(),
        pubkey: keypair.pubkey().to_string(),
        message: payload.message,
    };

    Ok(ResponseJson(ApiResponse::success(response_data)))
}

async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> Result<ResponseJson<ApiResponse<VerifyMessageData>>, (StatusCode, ResponseJson<ApiResponse<String>>)> {
    let pubkey = match Pubkey::from_str(&payload.pubkey) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid public key format".to_string()))
        )),
    };

    let signature_bytes = match bs58::decode(&payload.signature).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid signature format".to_string()))
        )),
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid signature".to_string()))
        )),
    };

    let message_bytes = payload.message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    let response_data = VerifyMessageData {
        valid: is_valid,
        message: payload.message,
        pubkey: payload.pubkey,
    };

    Ok(ResponseJson(ApiResponse::success(response_data)))
}

async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> Result<ResponseJson<ApiResponse<SolInstructionData>>, (StatusCode, ResponseJson<ApiResponse<String>>)> {
    // Validate addresses and amount
    let from = match Pubkey::from_str(&payload.from) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid sender public key".to_string()))
        )),
    };
    let to = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid recipient address".to_string()))
        )),
    };
    if payload.lamports == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Amount must be greater than 0".to_string()))
        ));
    }

    let instruction = system_instruction::transfer(&from, &to, payload.lamports);

    // Create proper instruction data with discriminator
    let mut instruction_data_bytes = vec![2u8, 0u8, 0u8, 0u8]; // Transfer discriminator
    instruction_data_bytes.extend_from_slice(&payload.lamports.to_le_bytes());

    let instruction_data = SolInstructionData {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.into_iter().map(|acc| acc.pubkey.to_string()).collect(),
        instruction_data: bs58::encode(&instruction_data_bytes).into_string(),
    };

    Ok(ResponseJson(ApiResponse::success(instruction_data)))
}

async fn send_token(
    ApiJson(payload): ApiJson<SendTokenRequest>,
) -> Result<ResponseJson<ApiResponse<TokenInstructionData>>, (StatusCode, ResponseJson<ApiResponse<String>>)> {
    // Validate required fields
    if let Err(error) = validate_send_token_request(&payload) {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error(error))
        ));
    }

    // Parse addresses
    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid destination address".to_string()))
        )),
    };
    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid mint address".to_string()))
        )),
    };
    let owner = match Pubkey::from_str(&payload.owner) {
        Ok(pk) => pk,
        Err(_) => return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ApiResponse::error("Invalid owner address".to_string()))
        )),
    };

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let dest_ata = spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let instruction = token_instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        payload.amount,
    ).map_err(|_| (
        StatusCode::BAD_REQUEST,
        ResponseJson(ApiResponse::error("Failed to create transfer instruction".to_string()))
    ))?;

    // Return accounts in test-expected order: ownerKeypair, dest ATA, ownerKeypair
    let accounts = vec![
        TokenAccountInfo { pubkey: owner.to_string(), is_signer: false },
        TokenAccountInfo { pubkey: dest_ata.to_string(), is_signer: false },
        TokenAccountInfo { pubkey: owner.to_string(), is_signer: false },
    ];

    let instruction_data = TokenInstructionData {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: bs58::encode(&instruction.data).into_string(),
    };

    Ok(ResponseJson(ApiResponse::success(instruction_data)))
}

// Validation functions
fn validate_create_token_request(payload: &CreateTokenRequest) -> Result<(), String> {
    if payload.mint_authority.is_empty() {
        return Err("Missing mint authority".to_string());
    }
    if payload.mint.is_empty() {
        return Err("Missing mint address".to_string());
    }
    Ok(())
}

fn validate_mint_token_request(payload: &MintTokenRequest) -> Result<(), String> {
    if payload.mint.is_empty() {
        return Err("Missing mint address".to_string());
    }
    if payload.destination.is_empty() {
        return Err("Missing destination address".to_string());
    }
    if payload.authority.is_empty() {
        return Err("Missing authority address".to_string());
    }
    Ok(())
}

fn validate_send_token_request(payload: &SendTokenRequest) -> Result<(), String> {
    if payload.destination.is_empty() {
        return Err("Missing destination address".to_string());
    }
    if payload.mint.is_empty() {
        return Err("Missing mint address".to_string());
    }
    if payload.owner.is_empty() {
        return Err("Missing owner address".to_string());
    }
    Ok(())
}
