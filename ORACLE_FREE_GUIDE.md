# 🆓 Oracle Cloud Always Free Deployment

## Why Oracle Cloud?
- **FREE FOREVER** (no 12-month limit like AWS)
- **2 AMD VMs**: 1/8 OCPU + 1GB RAM each
- **OR 4 ARM VMs**: 1/4 OCPU + 1GB RAM each  
- **Perfect for**: Small applications that need to run 24/7

## Step 1: Create Oracle Cloud Account
1. Go to **https://cloud.oracle.com**
2. Click **"Start for free"**
3. Sign up with email
4. **No credit card required** for Always Free tier

## Step 2: Create Compute Instance
1. **Go to**: Compute → Instances
2. **Click**: "Create Instance"
3. **Configure**:
   - **Name**: `solana-http-server`
   - **Image**: Ubuntu 22.04
   - **Shape**: VM.Standard.E2.1.Micro (Always Free)
   - **Network**: Create new VCN or use default
   - **SSH Keys**: Upload your public key
4. **Click**: "Create"

## Step 3: Configure Network Security
1. **Go to**: Networking → Virtual Cloud Networks
2. **Click**: Your VCN → Security Lists → Default Security List
3. **Add Ingress Rule**:
   - **Source CIDR**: 0.0.0.0/0
   - **Destination Port Range**: 3000
   - **Protocol**: TCP

## Step 4: Deploy Your App
```bash
# SSH to instance
ssh -i your-private-key ubuntu@your-instance-ip

# Install Rust and dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
sudo apt update && sudo apt install -y git build-essential pkg-config libssl-dev

# Deploy your project (same as AWS guide above)
# ... follow AWS guide steps 3-5
```

## 💰 Cost: $0 Forever! 🎉
