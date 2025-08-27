# Developer Tools API (fun project)

A comprehensive HTTP API providing essential developer tools and utilities. Built with Rust and Rocket for high performance and reliability. (not that it was needed)


### Network & IP Utilities
- **`/whoami`** - Show IP address, headers, and cookies
- **`/ip`** - Return public IP in plain text
- **`/ip-info`** - Return IP with geolocation info (JSON)
- **`/headers`** - Return all HTTP request headers
- **`/user-agent`** - Return the User-Agent string

### Data Processing & Encoding
- **`/base64/{text}`** - Encode text to Base64
- **`/base64-decode/{b64}`** - Decode Base64 string
- **`/urlencode/{text}`** - URL-encode given text
- **`/urldecode/{encoded}`** - URL-decode given string
- **`/hash/{algo}/{text}`** - Hash text using algorithm (md5, sha1, sha256)

### Cryptography & Security
- **`/jwt-decode/{token}`** - Decode JWT header & payload (no verification)

### Generators & Utilities
- **`/uuid`** - Generate random UUID v4
- **`/lorem/{words}`** - Generate lorem ipsum text (max 1000 words)
- **`/color`** - Return random hex color code
- **`/password/{length}`** - Generate random secure password (max 128 characters)
- **`/number/{min}/{max}`** - Generate random integer between min and max

### Time & Date
- **`/timestamp`** - Return current Unix timestamp (seconds & milliseconds)
- **`/time/utc`** - Return current UTC date/time
- **`/time/{tz}`** - Return current time in given timezone

### Testing & Development
- **`/echo`** - Echo back request method, query, body, and headers
- **`/delay/{seconds}`** - Delay response by given seconds (max 30 seconds)
- **`/status/{code}`** - Return given HTTP status code
- **`/ping`** - Return "pong"

### Fun Utilities
- **`/cat-fact`** - Return random cat fact
- **`/quote`** - Return random inspirational quote

## 🛠️ Installation & Running

### Prerequisites
- Rust (1.80.0 or higher)
- Cargo

### Build & Run
```bash
git clone <repository-url>
cd tools-api
cargo build --release
cargo run
```

The server will start on `http://0.0.0.0:8000` by default.

### Configuration
Edit `Rocket.toml` to customize server settings:
```toml
[debug]
address = "0.0.0.0"
port = 8000
```

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
