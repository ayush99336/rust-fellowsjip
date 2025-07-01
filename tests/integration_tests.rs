use solana_http_server::*;
use axum::extract::Json;

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_keypair_generation() {
        let response = generate_keypair().await;
        let api_response = response.0;
        
        assert!(api_response.success);
        assert!(api_response.data.is_some());
        
        let keypair_data = api_response.data.unwrap();
        assert!(!keypair_data.public_key.is_empty());
        assert!(!keypair_data.secret_key.is_empty());
        assert_eq!(api_response.message, "Keypair generated successfully");
    }

    #[tokio::test]
    async fn test_message_signing_and_verification() {
        // Generate a keypair
        let keypair_response = generate_keypair().await;
        let keypair_data = keypair_response.0.data.unwrap();

        // Sign a message
        let message = "Hello, Solana World!";
        let sign_request = SignMessageRequest {
            message: message.to_string(),
            secret: keypair_data.secret_key,
        };

        let sign_result = sign_message(Json(sign_request)).await;
        assert!(sign_result.is_ok());
        
        let sign_response = sign_result.unwrap().0;
        assert!(sign_response.success);
        assert!(sign_response.data.is_some());
        
        let sign_data = sign_response.data.unwrap();
        assert!(!sign_data.signature.is_empty());
        assert_eq!(sign_data.public_key, keypair_data.public_key);
        assert_eq!(sign_data.message, message);

        // Verify the message
        let verify_request = VerifyMessageRequest {
            message: message.to_string(),
            signature: sign_data.signature,
            pubkey: keypair_data.public_key,
        };

        let verify_result = verify_message(Json(verify_request)).await;
        assert!(verify_result.is_ok());
        
        let verify_response = verify_result.unwrap().0;
        assert!(verify_response.success);
        assert!(verify_response.data.is_some());
        
        let verify_data = verify_response.data.unwrap();
        assert!(verify_data.is_valid);
        assert_eq!(verify_data.message, message);
    }

    #[tokio::test]
    async fn test_sol_transfer_instruction_creation() {
        // Generate two keypairs for from/to addresses
        let from_keypair = generate_keypair().await.0.data.unwrap();
        let to_keypair = generate_keypair().await.0.data.unwrap();

        // Test valid SOL transfer
        let transfer_request = SendSolRequest {
            from: from_keypair.public_key.clone(),
            to: to_keypair.public_key.clone(),
            lamports: 1000000, // 0.001 SOL
        };

        let result = send_sol(Json(transfer_request)).await;
        assert!(result.is_ok());
        
        let response = result.unwrap().0;
        assert!(response.success);
        assert!(response.data.is_some());
        
        let instruction_data = response.data.unwrap();
        assert_eq!(instruction_data.program_id, "11111111111111111111111111111111"); // System Program
        assert_eq!(instruction_data.accounts.len(), 2); // from and to accounts
        assert!(instruction_data.accounts[0].is_signer); // from account should be signer
        assert!(instruction_data.accounts[0].is_writable); // from account should be writable
        assert!(instruction_data.accounts[1].is_writable); // to account should be writable
        assert!(!instruction_data.instruction_data.is_empty());
    }

    #[tokio::test]
    async fn test_sol_transfer_invalid_addresses() {
        // Test with invalid 'from' address
        let invalid_request = SendSolRequest {
            from: "invalid_address".to_string(),
            to: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
            lamports: 1000000,
        };

        let result = send_sol(Json(invalid_request)).await;
        assert!(result.is_err());
        
        let (status, error_response) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert!(!error_response.0.success);

        // Test with invalid 'to' address
        let invalid_request2 = SendSolRequest {
            from: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
            to: "invalid_address".to_string(),
            lamports: 1000000,
        };

        let result2 = send_sol(Json(invalid_request2)).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_sol_transfer_zero_amount() {
        let from_keypair = generate_keypair().await.0.data.unwrap();
        let to_keypair = generate_keypair().await.0.data.unwrap();

        // Test with zero lamports (should fail)
        let zero_request = SendSolRequest {
            from: from_keypair.public_key,
            to: to_keypair.public_key,
            lamports: 0,
        };

        let result = send_sol(Json(zero_request)).await;
        assert!(result.is_err());
        
        let (status, error_response) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert!(!error_response.0.success);
        assert!(error_response.0.message.contains("lamports"));
    }

    #[tokio::test]
    async fn test_token_creation_instruction() {
        let authority_keypair = generate_keypair().await.0.data.unwrap();
        let mint_keypair = generate_keypair().await.0.data.unwrap();

        let create_request = CreateTokenRequest {
            mint: mint_keypair.public_key.clone(),
            mint_authority: authority_keypair.public_key.clone(),
            decimals: 9,
        };

        let result = create_token(Json(create_request)).await;
        assert!(result.is_ok());
        
        let response = result.unwrap().0;
        assert!(response.success);
        assert!(response.data.is_some());
        
        let instruction_data = response.data.unwrap();
        assert_eq!(instruction_data.program_id, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"); // SPL Token Program
        assert!(!instruction_data.accounts.is_empty());
        assert!(!instruction_data.instruction_data.is_empty());
    }

    #[tokio::test]
    async fn test_token_creation_invalid_addresses() {
        // Test with invalid mint address
        let invalid_request = CreateTokenRequest {
            mint: "invalid_mint".to_string(),
            mint_authority: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
            decimals: 9,
        };

        let result = create_token(Json(invalid_request)).await;
        assert!(result.is_err());
        
        let (status, error_response) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert!(!error_response.0.success);
    }

    #[tokio::test]
    async fn test_token_mint_instruction() {
        let authority_keypair = generate_keypair().await.0.data.unwrap();
        let mint_keypair = generate_keypair().await.0.data.unwrap();
        let destination_keypair = generate_keypair().await.0.data.unwrap();

        let mint_request = MintTokenRequest {
            mint: mint_keypair.public_key.clone(),
            destination: destination_keypair.public_key.clone(),
            authority: authority_keypair.public_key.clone(),
            amount: 1000000000, // 1 token with 9 decimals
        };

        let result = mint_token(Json(mint_request)).await;
        assert!(result.is_ok());
        
        let response = result.unwrap().0;
        assert!(response.success);
        assert!(response.data.is_some());
        
        let instruction_data = response.data.unwrap();
        assert_eq!(instruction_data.program_id, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"); // SPL Token Program
        assert!(!instruction_data.accounts.is_empty());
        assert!(!instruction_data.instruction_data.is_empty());
    }

    #[tokio::test]
    async fn test_token_transfer_instruction() {
        let owner_keypair = generate_keypair().await.0.data.unwrap();
        let destination_keypair = generate_keypair().await.0.data.unwrap();
        let mint_keypair = generate_keypair().await.0.data.unwrap();

        let transfer_request = SendTokenRequest {
            mint: mint_keypair.public_key.clone(),
            destination: destination_keypair.public_key.clone(),
            owner: owner_keypair.public_key.clone(),
            amount: 500000000, // 0.5 tokens with 9 decimals
        };

        let result = send_token(Json(transfer_request)).await;
        assert!(result.is_ok());
        
        let response = result.unwrap().0;
        assert!(response.success);
        assert!(response.data.is_some());
        
        let instruction_data = response.data.unwrap();
        assert_eq!(instruction_data.program_id, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"); // SPL Token Program
        assert!(!instruction_data.accounts.is_empty());
        assert!(!instruction_data.instruction_data.is_empty());

        // Should have 3 accounts: source ATA, dest ATA, and owner
        assert_eq!(instruction_data.accounts.len(), 3);
    }

    #[tokio::test]
    async fn test_token_transfer_zero_amount() {
        let owner_keypair = generate_keypair().await.0.data.unwrap();
        let destination_keypair = generate_keypair().await.0.data.unwrap();
        let mint_keypair = generate_keypair().await.0.data.unwrap();

        // Test with zero amount (should fail)
        let zero_request = SendTokenRequest {
            mint: mint_keypair.public_key,
            destination: destination_keypair.public_key,
            owner: owner_keypair.public_key,
            amount: 0,
        };

        let result = send_token(Json(zero_request)).await;
        assert!(result.is_err());
        
        let (status, error_response) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert!(!error_response.0.success);
        assert!(error_response.0.message.contains("amount"));
    }

    #[tokio::test]
    async fn test_message_signing_invalid_secret_key() {
        // Test with invalid secret key format
        let invalid_request = SignMessageRequest {
            message: "Hello World".to_string(),
            secret: "invalid_secret_key".to_string(),
        };

        let result = sign_message(Json(invalid_request)).await;
        assert!(result.is_err());
        
        let (status, error_response) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert!(!error_response.0.success);
        assert!(error_response.0.message.contains("Invalid secret key"));
    }

    #[tokio::test]
    async fn test_message_verification_invalid_signature() {
        let keypair = generate_keypair().await.0.data.unwrap();
        
        // Test with invalid signature
        let invalid_verify_request = VerifyMessageRequest {
            message: "Hello World".to_string(),
            signature: "invalid_signature".to_string(),
            pubkey: keypair.public_key,
        };

        let result = verify_message(Json(invalid_verify_request)).await;
        assert!(result.is_err());
        
        let (status, error_response) = result.unwrap_err();
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert!(!error_response.0.success);
    }

    #[test]
    fn test_utility_functions() {
        // Test amount validation
        assert!(validate_amount(100, "amount").is_ok());
        assert!(validate_amount(0, "amount").is_err());
        
        // Test empty string validation
        assert!(validate_not_empty("hello", "message").is_ok());
        assert!(validate_not_empty("", "message").is_err());
        assert!(validate_not_empty("   ", "message").is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Test very large amounts
        assert!(validate_amount(u64::MAX, "amount").is_ok());
        
        // Test boundary values
        assert!(validate_amount(1, "amount").is_ok());
        assert!(validate_amount(0, "amount").is_err());
        
        // Test string validation with unicode
        assert!(validate_not_empty("Hello 🚀", "message").is_ok());
        assert!(validate_not_empty("   ", "trimmed").is_err());
    }
}
