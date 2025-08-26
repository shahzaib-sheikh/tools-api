# Developer Tools API (fun project)

A comprehensive HTTP API providing essential developer tools and utilities. Built with Rust and Rocket for high performance and reliability.

## 🚀 Features

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

The project includes comprehensive unit tests for all core functionality including:
- Hash functions (MD5, SHA1, SHA256)
- Base64 encoding/decoding
- URL encoding/decoding
- UUID generation
- Password generation
- Time utilities
- And more...

## 📝 API Examples

### Get your IP address
```bash
curl https://tools.stackfrost.com/ip
```

### Generate a UUID
```bash
curl https://tools.stackfrost.com/uuid
```

### Hash some text
```bash
curl https://tools.stackfrost.com/hash/sha256/hello
```

### Encode to Base64
```bash
curl https://tools.stackfrost.com/base64/hello%20world
```

### Generate a password
```bash
curl https://tools.stackfrost.com/password/16
```

### Get request info
```bash
curl https://tools.stackfrost.com/whoami
```

## 🏗️ Architecture

- **Language**: Rust
- **Framework**: Rocket 0.5
- **Template Engine**: Handlebars
- **Dependencies**: Minimal set focused on core functionality
- **Performance**: Zero-copy where possible, minimal allocations

## 🔧 Dependencies

Core dependencies include:
- `rocket` - Web framework
- `serde` - Serialization/deserialization
- `chrono` - Date and time handling
- `uuid` - UUID generation
- `base64` - Base64 encoding/decoding
- `rand` - Random number generation
- `sha1`, `sha2`, `md5` - Hash functions
- `hex` - Hexadecimal encoding

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🌟 Similar Projects

This API is inspired by services like httpbin.org and provides similar functionality optimized for developer workflows.