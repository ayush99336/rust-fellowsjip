use axum::response::Json;
use crate::types::ApiResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    uptime: String,
}

#[derive(Serialize)]
pub struct ApiInfoResponse {
    name: String,
    description: String,
    version: String,
    endpoints: Vec<EndpointInfo>,
}

#[derive(Serialize)]
pub struct EndpointInfo {
    method: String,
    path: String,
    description: String,
}

/// Health check endpoint - returns server status
pub async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse {
        success: true,
        data: Some(HealthResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            uptime: "running".to_string(),
        }),
        message: "Solana HTTP Server is running healthy".to_string(),
    })
}

/// Root endpoint - returns API information and available endpoints
pub async fn api_info() -> Json<ApiResponse<ApiInfoResponse>> {
    let endpoints = vec![
        EndpointInfo {
            method: "GET".to_string(),
            path: "/".to_string(),
            description: "API information and available endpoints".to_string(),
        },
        EndpointInfo {
            method: "GET".to_string(),
            path: "/health".to_string(),
            description: "Health check endpoint".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/keypair".to_string(),
            description: "Generate a new Solana keypair".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/token/create".to_string(),
            description: "Create SPL token mint instruction".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/token/mint".to_string(),
            description: "Create mint tokens instruction".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/message/sign".to_string(),
            description: "Sign a message with a private key".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/message/verify".to_string(),
            description: "Verify a signed message".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/send/sol".to_string(),
            description: "Create SOL transfer instruction".to_string(),
        },
        EndpointInfo {
            method: "POST".to_string(),
            path: "/send/token".to_string(),
            description: "Create SPL token transfer instruction".to_string(),
        },
    ];

    Json(ApiResponse {
        success: true,
        data: Some(ApiInfoResponse {
            name: "Solana HTTP Server".to_string(),
            description: "A comprehensive HTTP server for interacting with the Solana blockchain. Deployed on Shuttle.rs for global access.".to_string(),
            version: "1.0.0".to_string(),
            endpoints,
        }),
        message: "Welcome to Solana HTTP Server API - Deployed on Shuttle.rs!".to_string(),
    })
}
