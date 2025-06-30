# Solana HTTP Server

A comprehensive HTTP server built with Rust for interacting with the Solana blockchain. This server provides REST API endpoints for common Solana operations including keypair generation, SPL token management, message signing/verification, and transaction instruction creation.

## 🚀 Features

- **Keypair Management**: Generate new Solana keypairs
- **SPL Token Operations**: Create mint instructions and mint tokens
- **Message Signing**: Sign and verify messages with Ed25519 cryptography
- **Transfer Instructions**: Create SOL and SPL token transfer instructions
- **Modern Architecture**: Built with Axum, Tokio, and the Solana SDK
- **Consistent API**: All endpoints return JSON with a standard response format
- **Type Safety**: Comprehensive request/response validation
- **Modular Design**: Clean, maintainable code structure

## 🛠 Technology Stack

- **Rust** - Systems programming language
- **Axum** - Modern async web framework
- **Tokio** - Async runtime
- **Solana SDK** - Solana blockchain integration
- **SPL Token** - Solana Program Library for tokens
- **Serde** - Serialization/deserialization

## 📋 API Endpoints

All endpoints accept and return JSON. The standard response format is:

```json
{
  "success": boolean,
  "data": object | null,
  "message": string
}
```

### Generate Keypair
- **POST** `/keypair`
- **Description**: Generate a new Solana keypair
- **Response**:
```json
{
  "success": true,
  "data": {
    "public_key": "base58_encoded_public_key",
    "secret_key": "base58_encoded_secret_key"
  },
  "message": "Keypair generated successfully"
}
```

### Create SPL Token Mint
- **POST** `/token/create`
- **Description**: Create an SPL token mint instruction
- **Request Body**:
```json
{
  "mint_authority": "base58_public_key",
  "mint": "base58_public_key",
  "decimals": 9
}
```

### Mint SPL Tokens
- **POST** `/token/mint`
- **Description**: Create a mint-to instruction
- **Request Body**:
```json
{
  "mint": "base58_public_key",
  "destination": "base58_public_key",
  "authority": "base58_public_key",
  "amount": 1000000
}
```

### Sign Message
- **POST** `/message/sign`
- **Description**: Sign a message with a private key
- **Request Body**:
```json
{
  "message": "Your message here",
  "secret": "base58_encoded_secret_key"
}
```

### Verify Message
- **POST** `/message/verify`
- **Description**: Verify a signed message
- **Request Body**:
```json
{
  "message": "Your message here",
  "signature": "base64_encoded_signature",
  "pubkey": "base58_public_key"
}
```

### Send SOL
- **POST** `/send/sol`
- **Description**: Create a SOL transfer instruction
- **Request Body**:
```json
{
  "from": "base58_public_key",
  "to": "base58_public_key",
  "lamports": 1000000
}
```

### Send SPL Tokens
- **POST** `/send/token`
- **Description**: Create an SPL token transfer instruction
- **Request Body**:
```json
{
  "destination": "base58_public_key",
  "mint": "base58_public_key",
  "owner": "base58_public_key",
  "amount": 1000000
}
```

## 🛠 Installation & Setup

### Prerequisites

- Rust 1.70.0 or later
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd rust-fellowsjip
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## 📖 Usage Examples

### Generate a New Keypair
```bash
curl -X POST http://127.0.0.1:3000/keypair
```

### Sign a Message
```bash
curl -X POST http://127.0.0.1:3000/message/sign \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Hello, Solana!",
    "secret": "your_base58_secret_key_here"
  }'
```

### Create SOL Transfer Instruction
```bash
curl -X POST http://127.0.0.1:3000/send/sol \
  -H "Content-Type: application/json" \
  -d '{
    "from": "source_public_key",
    "to": "destination_public_key",
    "lamports": 1000000
  }'
```

## 🏗 Project Structure

```
src/
├── main.rs           # Server setup and routing
├── handlers/         # Request handlers by feature
│   ├── mod.rs        # Handler module exports
│   ├── keypair.rs    # Keypair generation
│   ├── token.rs      # SPL token operations
│   ├── message.rs    # Message signing/verification
│   └── transfer.rs   # Transfer instructions
├── types.rs          # Request/response type definitions
├── utils.rs          # Utility functions and validation
└── tests.rs          # Test suite
```

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

## 🐳 Docker

Build and run with Docker:
```bash
docker build -t solana-http-server .
docker run -p 3000:3000 solana-http-server
```

## 🔒 Security Considerations

- This server is designed for development and testing purposes
- Private keys are handled in memory but should be managed securely in production
- Consider implementing rate limiting and authentication for production use
- All cryptographic operations use the Solana SDK's secure implementations

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🔗 Related Resources

- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Program](https://spl.solana.com/token)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Tokio Documentation](https://docs.rs/tokio/latest/tokio/)

### Testing the Server

```bash
# Test with curl
curl http://localhost:3000

# Or visit in browser
open http://localhost:3000
```

## 📦 Dependencies

- **hyper**: HTTP implementation for Rust
- **tokio**: Asynchronous runtime for Rust

## 🏗 Project Structure

```
src/
└── main.rs          # Main server entry point
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Create a pull request

## 📝 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- Built with the amazing Rust ecosystem
- Thanks to the Hyper and Tokio communities
