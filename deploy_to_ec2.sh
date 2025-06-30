#!/bin/bash

# EC2 Deployment Script for Rust Solana HTTP Server
# Replace YOUR_EC2_IP with your actual EC2 instance public IP
# Replace YOUR_KEY_PATH with the path to your .pem key file

EC2_IP="YOUR_EC2_IP_HERE"
KEY_PATH="YOUR_KEY_PATH_HERE"
PROJECT_NAME="rust-fellowsjip"

echo "🚀 Deploying Rust Solana HTTP Server to EC2..."

# Step 1: Connect and update system
echo "📦 Updating system packages..."
ssh -i "$KEY_PATH" ubuntu@$EC2_IP << 'EOF'
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential pkg-config libssl-dev curl git
EOF

# Step 2: Install Rust
echo "🦀 Installing Rust..."
ssh -i "$KEY_PATH" ubuntu@$EC2_IP << 'EOF'
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
rustc --version
EOF

# Step 3: Upload project files
echo "📁 Uploading project files..."
scp -i "$KEY_PATH" -r . ubuntu@$EC2_IP:~/$PROJECT_NAME/

# Step 4: Build and run the project
echo "🔨 Building project on EC2..."
ssh -i "$KEY_PATH" ubuntu@$EC2_IP << EOF
cd ~/$PROJECT_NAME
source ~/.cargo/env
cargo build --release
echo "✅ Build complete!"

echo "🌐 Starting server..."
nohup cargo run --release > server.log 2>&1 &
echo "Server started! Check server.log for details."
echo "Your server should be accessible at: http://$EC2_IP:3000"
EOF

echo "🎉 Deployment complete!"
echo "Access your server at: http://$EC2_IP:3000"
echo "SSH into your instance: ssh -i $KEY_PATH ubuntu@$EC2_IP"
