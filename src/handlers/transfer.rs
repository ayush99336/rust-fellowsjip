use axum::{extract::Json as ExtractJson, http::StatusCode, response::Json};
use crate::types::{ApiResponse, SendSolRequest, SendTokenRequest, InstructionResponse, AccountInfo};
use crate::utils::{parse_public_key, validate_amount};
use solana_sdk::system_instruction;
use spl_token::instruction as token_instruction;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};

/// Create a SOL transfer instruction
pub async fn send_sol(
    ExtractJson(payload): ExtractJson<SendSolRequest>,
) -> Result<Json<ApiResponse<InstructionResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let from = parse_public_key(&payload.from)
        .map_err(|e| create_error_response(&e))?;
    let to = parse_public_key(&payload.to)
        .map_err(|e| create_error_response(&e))?;

    validate_amount(payload.lamports, "lamports")
        .map_err(|e| create_error_response(&e))?;

    let instruction = system_instruction::transfer(&from, &to, payload.lamports);

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            public_key: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let instruction_data = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: BASE64_STANDARD.encode(&instruction.data),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(instruction_data),
        message: "SOL transfer instruction created successfully".to_string(),
    }))
}

/// Create an SPL token transfer instruction
pub async fn send_token(
    ExtractJson(payload): ExtractJson<SendTokenRequest>,
) -> Result<Json<ApiResponse<InstructionResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let destination = parse_public_key(&payload.destination)
        .map_err(|e| create_error_response(&e))?;
    let mint = parse_public_key(&payload.mint)
        .map_err(|e| create_error_response(&e))?;
    let owner = parse_public_key(&payload.owner)
        .map_err(|e| create_error_response(&e))?;

    validate_amount(payload.amount, "amount")
        .map_err(|e| create_error_response(&e))?;

    // Get associated token accounts
    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let dest_ata = spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let instruction = token_instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        payload.amount,
    )
    .map_err(|_| create_error_response("Failed to create transfer instruction"))?;

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            public_key: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let instruction_data = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: BASE64_STANDARD.encode(&instruction.data),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(instruction_data),
        message: "Token transfer instruction created successfully".to_string(),
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
