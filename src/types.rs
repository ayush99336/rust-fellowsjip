use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}


#[derive(Serialize)]
pub struct KeypairResponse {
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}


#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}


#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}


#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

// Message verification response
#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub is_valid: bool,
    pub message: String,
    pub public_key: String,
}

// SOL transfer request
#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

// Token transfer request
#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

// Account metadata for instructions
#[derive(Serialize)]
pub struct AccountInfo {
    pub public_key: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

// Instruction data response
#[derive(Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}
