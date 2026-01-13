# Ada Remote Relay Server

The relay server provides signaling and TURN relay functionality for Ada Remote connections.

## Features

- **WebSocket Signaling**: Coordinates WebRTC connection establishment
- **TURN Relay**: Provides relay for connections that can't establish P2P
- **STUN Server**: Helps clients discover their public IP addresses
- **Session Management**: Tracks and manages active remote sessions
- **Scalable**: Designed to handle thousands of concurrent sessions

## Quick Start

### Running Locally

```bash
# Build and run
cargo run --release

# With custom bind address
cargo run --release -- --bind 0.0.0.0:8080
```

### Running with Docker

```bash
# Build and run with docker-compose (from project root)
docker-compose up -d

# Or build manually
docker build -t ada-remote-relay .
docker run -p 8080:8080 ada-remote-relay
```

### Running as a Service (Linux)

```bash
# 1. Build the binary
cargo build --release

# 2. Copy binary to system location
sudo cp target/release/relay-server /opt/ada-remote/relay-server

# 3. Create user
sudo useradd -r -s /bin/false ada-remote

# 4. Copy service file
sudo cp ada-remote-relay.service /etc/systemd/system/

# 5. Create log directory
sudo mkdir -p /var/log/ada-remote
sudo chown ada-remote:ada-remote /var/log/ada-remote

# 6. Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable ada-remote-relay
sudo systemctl start ada-remote-relay

# Check status
sudo systemctl status ada-remote-relay
```

## Configuration

Copy the example configuration file:

```bash
cp config.example.toml config.toml
```

Edit `config.toml` with your settings. Key configuration options:

- **bind**: Address and port to listen on
- **enable_tls**: Enable HTTPS/WSS
- **turn_secret**: Shared secret for TURN authentication
- **external_ip**: Your server's public IP address

## Environment Variables

You can also configure via environment variables:

```bash
export ADA_BIND="0.0.0.0:8080"
export ADA_TURN_SECRET="your-secret-here"
export ADA_EXTERNAL_IP="203.0.113.1"
export RUST_LOG="info"
```

## Deployment

### Recommended Setup

For production deployments:

1. **Use TLS**: Enable HTTPS/WSS with valid certificates
2. **Set External IP**: Configure your public IP for NAT traversal
3. **Configure Firewall**:
   - Port 8080 (or your chosen port) for WebSocket signaling
   - Port 3478 UDP/TCP for TURN relay
   - UDP ports 49152-65535 for media relay (if using TURN)

4. **Enable Rate Limiting**: Protect against abuse
5. **Monitor Metrics**: Use Prometheus endpoint for monitoring
6. **Log Rotation**: Set up log rotation for `/var/log/ada-remote`

### Reverse Proxy (Nginx)

Example Nginx configuration:

```nginx
server {
    listen 443 ssl http2;
    server_name relay.ada-remote.io;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Cloud Deployment

#### AWS

```bash
# Using EC2 with Ubuntu
sudo apt update
sudo apt install -y docker.io docker-compose
git clone https://github.com/AdaWorldAPI/ada-remote.git
cd ada-remote
docker-compose up -d
```

#### DigitalOcean

Use the provided Dockerfile or deploy directly on a Droplet.

#### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ada-remote-relay
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ada-remote-relay
  template:
    metadata:
      labels:
        app: ada-remote-relay
    spec:
      containers:
      - name: relay
        image: ada-remote-relay:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
---
apiVersion: v1
kind: Service
metadata:
  name: ada-remote-relay
spec:
  selector:
    app: ada-remote-relay
  ports:
  - protocol: TCP
    port: 8080
    targetPort: 8080
  type: LoadBalancer
```

## Monitoring

### Health Check

```bash
# Check if server is running
curl http://localhost:8080/health
```

### Prometheus Metrics

Enable metrics in config.toml:

```toml
[metrics]
enable_metrics = true
metrics_bind = "127.0.0.1:9090"
```

Access metrics at `http://localhost:9090/metrics`

### Logs

View logs:

```bash
# Systemd service
sudo journalctl -u ada-remote-relay -f

# Docker
docker logs -f ada-remote-relay

# File
tail -f /var/log/ada-remote/relay.log
```

## Performance Tuning

### System Limits

Increase file descriptor limits:

```bash
# /etc/security/limits.conf
ada-remote soft nofile 65536
ada-remote hard nofile 65536
```

### Kernel Parameters

Optimize network settings:

```bash
# /etc/sysctl.conf
net.core.somaxconn = 4096
net.ipv4.tcp_max_syn_backlog = 4096
net.core.netdev_max_backlog = 5000
```

## Troubleshooting

### Common Issues

**"Address already in use"**
- Another service is using port 8080
- Change the bind port in configuration

**"Permission denied"**
- Use sudo or run as root (not recommended)
- Or bind to port > 1024

**Connections timing out**
- Check firewall rules
- Verify external_ip is set correctly
- Ensure TURN ports are accessible

## Security

- Always use TLS in production
- Keep the TURN secret confidential
- Regularly update dependencies
- Monitor for unusual traffic patterns
- Use fail2ban to prevent brute force attacks

## License

MIT or Apache-2.0
