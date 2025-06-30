# 🚀 Solana HTTP Server - Cloud Deployment Guide

This guide will help you deploy your Rust-based Solana HTTP server to various cloud platforms.

## 📋 Prerequisites

- Docker installed locally
- Cloud platform account (AWS, Digital Ocean, etc.)
- Your project builds successfully with `cargo build --release`

## 🐳 Docker Deployment (Recommended)

### 1. Create Dockerfile

Create a `Dockerfile` in your project root:

```dockerfile
# Use the official Rust image as the build environment
FROM rust:1.75 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/solana-http-server /app/solana-http-server

# Change ownership to the non-root user
RUN chown appuser:appuser /app/solana-http-server

# Switch to the non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info

# Run the application
CMD ["./solana-http-server"]
```

### 2. Create .dockerignore

```
target/
.git/
.gitignore
README.md
Dockerfile
.dockerignore
*.md
```

### 3. Build and Test Docker Image

```bash
# Build the Docker image
docker build -t solana-http-server .

# Test the image locally
docker run -p 3000:3000 solana-http-server

# Test the endpoints
curl http://localhost:3000/health
curl http://localhost:3000/
```

## ☁️ AWS Deployment Options

### Option 1: AWS App Runner (Easiest)

1. **Push to Container Registry:**
   ```bash
   # Tag and push to Amazon ECR
   aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account-id>.dkr.ecr.us-east-1.amazonaws.com
   docker tag solana-http-server:latest <account-id>.dkr.ecr.us-east-1.amazonaws.com/solana-http-server:latest
   docker push <account-id>.dkr.ecr.us-east-1.amazonaws.com/solana-http-server:latest
   ```

2. **Create App Runner Service:**
   - Go to AWS App Runner console
   - Create new service
   - Choose "Container registry" as source
   - Select your ECR image
   - Configure port: 3000
   - Deploy!

### Option 2: AWS ECS Fargate

1. **Create Task Definition:**
   ```json
   {
     "family": "solana-http-server",
     "networkMode": "awsvpc",
     "requiresCompatibilities": ["FARGATE"],
     "cpu": "256",
     "memory": "512",
     "executionRoleArn": "arn:aws:iam::<account-id>:role/ecsTaskExecutionRole",
     "containerDefinitions": [
       {
         "name": "solana-http-server",
         "image": "<account-id>.dkr.ecr.us-east-1.amazonaws.com/solana-http-server:latest",
         "portMappings": [
           {
             "containerPort": 3000,
             "protocol": "tcp"
           }
         ],
         "essential": true,
         "logConfiguration": {
           "logDriver": "awslogs",
           "options": {
             "awslogs-group": "/ecs/solana-http-server",
             "awslogs-region": "us-east-1",
             "awslogs-stream-prefix": "ecs"
           }
         }
       }
     ]
   }
   ```

2. **Create ECS Service with Application Load Balancer**

### Option 3: AWS EC2 (More Control)

1. **Launch EC2 Instance:**
   - Choose Ubuntu 22.04 LTS
   - Instance type: t3.micro (free tier)
   - Configure security group: Allow port 3000

2. **Setup on EC2:**
   ```bash
   # SSH to your instance
   ssh -i your-key.pem ubuntu@your-ec2-ip

   # Install Docker
   sudo apt update
   sudo apt install docker.io -y
   sudo systemctl start docker
   sudo systemctl enable docker
   sudo usermod -aG docker ubuntu

   # Pull and run your image
   docker pull <account-id>.dkr.ecr.us-east-1.amazonaws.com/solana-http-server:latest
   docker run -d -p 3000:3000 --name solana-server --restart unless-stopped <account-id>.dkr.ecr.us-east-1.amazonaws.com/solana-http-server:latest
   ```

## 🌊 Digital Ocean Deployment

### Option 1: Digital Ocean App Platform (Easiest)

1. **Create GitHub/GitLab Repository** with your code

2. **Create App Platform App:**
   - Go to Digital Ocean App Platform
   - Connect your repository
   - Configure build settings:
     ```yaml
     name: solana-http-server
     services:
     - name: web
       source_dir: /
       github:
         repo: your-username/your-repo
         branch: main
       run_command: cargo run --release
       environment_slug: rust
       instance_count: 1
       instance_size_slug: basic-xxs
       routes:
       - path: /
       http_port: 3000
     ```

### Option 2: Digital Ocean Container Registry + Droplet

1. **Push to DO Container Registry:**
   ```bash
   # Install doctl and authenticate
   doctl auth init

   # Create container registry
   doctl registry create solana-registry

   # Tag and push
   docker tag solana-http-server registry.digitalocean.com/solana-registry/solana-http-server:latest
   docker push registry.digitalocean.com/solana-registry/solana-http-server:latest
   ```

2. **Deploy on Droplet:**
   ```bash
   # Create and SSH to droplet
   doctl compute droplet create solana-server --image ubuntu-22-04-x64 --size s-1vcpu-1gb --region nyc1 --ssh-keys <your-ssh-key-id>

   # SSH and setup
   ssh root@your-droplet-ip

   # Install Docker
   apt update && apt install docker.io -y

   # Login to registry
   doctl registry login

   # Pull and run
   docker pull registry.digitalocean.com/solana-registry/solana-http-server:latest
   docker run -d -p 3000:3000 --name solana-server --restart unless-stopped registry.digitalocean.com/solana-registry/solana-http-server:latest
   ```

## 🔧 Environment Configuration

### Production Environment Variables

Create a `.env` file for production:
```bash
RUST_LOG=info
PORT=3000
# Add any other configuration needed
```

### Health Checks

All platforms should use these endpoints for health checks:
- **Health Check URL:** `http://your-domain:3000/health`
- **Expected Response:** `{"success":true,"data":{"status":"healthy"}}`

## 🚀 Quick Start Commands

### Local Development
```bash
cargo run                          # Run locally
curl http://localhost:3000/health  # Test health
curl http://localhost:3000/        # Test API info
```

### Docker
```bash
docker build -t solana-http-server .
docker run -p 3000:3000 solana-http-server
```

### Production
```bash
cargo build --release             # Build optimized binary
./target/release/solana-http-server # Run production build
```

## 📊 Monitoring & Logging

### Health Check Endpoint
- **URL:** `/health`
- **Response:** JSON with server status
- **Use for:** Load balancer health checks, monitoring

### API Information
- **URL:** `/`
- **Response:** Complete API documentation
- **Use for:** API discovery, documentation

## 🔒 Security Considerations

1. **Use HTTPS** in production (configure load balancer/reverse proxy)
2. **Rate limiting** (consider adding middleware)
3. **Environment secrets** (use cloud platform secret management)
4. **Network security** (configure security groups/firewalls appropriately)

## 💰 Cost Optimization

### AWS
- **App Runner:** ~$25-50/month for basic usage
- **ECS Fargate:** ~$15-30/month for small instances
- **EC2 t3.micro:** Free tier available, ~$8.50/month after

### Digital Ocean
- **App Platform:** ~$12-25/month
- **Droplet:** ~$6-12/month for basic droplets

## 🎯 Next Steps

1. Choose your preferred deployment platform
2. Follow the specific deployment steps above
3. Set up monitoring and health checks
4. Configure custom domain (optional)
5. Set up CI/CD pipeline for automated deployments

Your Solana HTTP Server is now ready for cloud deployment! 🚀
