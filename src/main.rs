mod handlers;
mod types;
mod utils;

#[cfg(test)]
mod tests;

use axum::{routing::post, Router};
use handlers::{generate_keypair, create_token, mint_token, sign_message, verify_message, send_sol, send_token};

/// A Solana HTTP Server built with Rust
/// 
/// This server provides endpoints for interacting with Solana blockchain:
/// - Generate keypairs
/// - Create SPL token instructions
/// - Sign and verify messages
/// - Create transfer instructions
///
/// All endpoints return JSON responses with a consistent format.
#[tokio::main]
async fn main() {
    // Set up the routes
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    // Bind to localhost on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    // Start the server with a friendly message
    println!("🚀 Solana HTTP Server is running!");
    println!("📍 Server address: http://127.0.0.1:3000");
    println!();
    println!("📋 Available endpoints:");
    println!("  POST /keypair        - Generate a new Solana keypair");
    println!("  POST /token/create   - Create SPL token mint instruction");
    println!("  POST /token/mint     - Create mint tokens instruction");
    println!("  POST /message/sign   - Sign a message with a private key");
    println!("  POST /message/verify - Verify a signed message");
    println!("  POST /send/sol       - Create SOL transfer instruction");
    println!("  POST /send/token     - Create SPL token transfer instruction");
    println!();
    println!("✨ Server is ready to handle requests!");

    // Run the server
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
