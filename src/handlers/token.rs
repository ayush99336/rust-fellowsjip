use axum::{extract::Json as ExtractJson, http::StatusCode, response::Json};
use crate::types::{ApiResponse, CreateTokenRequest, MintTokenRequest, InstructionResponse, AccountInfo};
use crate::utils::{parse_public_key, validate_amount};
use spl_token::instruction as token_instruction;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};

/// Create an SPL token mint instruction
pub async fn create_token(
    ExtractJson(payload): ExtractJson<CreateTokenRequest>,
) -> Result<Json<ApiResponse<InstructionResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mint_authority = parse_public_key(&payload.mint_authority)
        .map_err(|e| create_error_response(&e))?;
    let mint = parse_public_key(&payload.mint)
        .map_err(|e| create_error_response(&e))?;

    let instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        payload.decimals,
    )
    .map_err(|_| create_error_response("Failed to create initialize mint instruction"))?;

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
        message: "Token mint instruction created successfully".to_string(),
    }))
}

/// Create a mint-to instruction for SPL tokens
pub async fn mint_token(
    ExtractJson(payload): ExtractJson<MintTokenRequest>,
) -> Result<Json<ApiResponse<InstructionResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mint = parse_public_key(&payload.mint)
        .map_err(|e| create_error_response(&e))?;
    let destination = parse_public_key(&payload.destination)
        .map_err(|e| create_error_response(&e))?;
    let authority = parse_public_key(&payload.authority)
        .map_err(|e| create_error_response(&e))?;

    validate_amount(payload.amount, "amount")
        .map_err(|e| create_error_response(&e))?;

    let instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    )
    .map_err(|_| create_error_response("Failed to create mint instruction"))?;

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
        message: "Mint instruction created successfully".to_string(),
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
