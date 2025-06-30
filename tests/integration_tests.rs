use rust_http_server::*;
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
}
