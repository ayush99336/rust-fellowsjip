use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;

/// Parse a base58 encoded public key string into a Pubkey
/// Returns a user-friendly error message if parsing fails
pub fn parse_public_key(pubkey_str: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(pubkey_str).map_err(|_| {
        format!("Invalid public key format: '{}'. Expected a base58 encoded string.", pubkey_str)
    })
}

/// Parse a base58 encoded secret key string into a Keypair
/// The secret key should be 64 bytes when decoded
pub fn parse_secret_key(secret_str: &str) -> Result<Keypair, String> {
    // First, try to decode the base58 string
    let secret_bytes = bs58::decode(secret_str)
        .into_vec()
        .map_err(|_| {
            "Invalid secret key format. Expected a base58 encoded string.".to_string()
        })?;
    
    // Check if the decoded bytes have the correct length
    if secret_bytes.len() != 64 {
        return Err(format!(
            "Invalid secret key length: expected 64 bytes, got {}",
            secret_bytes.len()
        ));
    }
    
    // Try to create a keypair from the bytes
    Keypair::from_bytes(&secret_bytes).map_err(|err| {
        format!("Failed to create keypair from secret key: {}", err)
    })
}

/// Validate that an amount is greater than zero
pub fn validate_amount(amount: u64, field_name: &str) -> Result<(), String> {
    if amount == 0 {
        return Err(format!("{} must be greater than 0", field_name));
    }
    Ok(())
}

/// Check if a string is empty or only whitespace
pub fn validate_not_empty(value: &str, field_name: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{} cannot be empty", field_name));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_amount() {
        assert!(validate_amount(100, "amount").is_ok());
        assert!(validate_amount(0, "amount").is_err());
    }

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("hello", "message").is_ok());
        assert!(validate_not_empty("", "message").is_err());
        assert!(validate_not_empty("   ", "message").is_err());
    }
}
