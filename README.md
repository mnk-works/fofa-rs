# fofa-rs

A fast, lightweight Rust CLI tool for parsing FOFA.io search results.

![Build Status](https://github.com/mnk-works/fofa-rs/workflows/CI/badge.svg)
![Crates.io](https://img.shields.io/crates/v/fofa-rs.svg)
![License](https://img.shields.io/crates/l/fofa-rs.svg)

## Features

- 🔍 **Query FOFA API** - Search for hosts, services, and vulnerabilities
- 🎨 **Colored output** - Easy-to-read formatted results
- 📄 **Multiple fields** - IP, port, host/domain, and more
- ⚡ **Fast & lightweight** - Built with Rust, zero runtime overhead
- 🔧 **Flexible** - Configurable query, limit, page, and output

## Installation

### From crates.io (recommended)

```bash
cargo install fofa-rs
```

### From source

```bash
git clone https://github.com/mnk-works/fofa-rs
cd fofa-rs
cargo install --path .
```

### Pre-built binaries

Download from [Releases](https://github.com/mnk-works/fofa-rs/releases).

## Usage

### Basic usage

```bash
fofa-rs -e your@email.com -k your_api_key -q "title=\"nginx\""
```

### Options

```
Usage: fofa-rs [OPTIONS] --email <EMAIL> --key <KEY>

Options:
  -q, --query <QUERY>      Search query (e.g., "title=\"Test\"") [default: title="Test"]
  -l, --limit <LIMIT>      Maximum number of results to display (default: 50)
  -p, --page <PAGE>        Page number (default: 1)
  -e, --email <EMAIL>      FOFA account email [env: FOFA_EMAIL]
  -k, --key <KEY>          FOFA API key [env: FOFA_KEY]
  -h, --help               Print help
  -V, --version            Print version
```

### Using environment variables

```bash
export FOFA_EMAIL="your@email.com"
export FOFA_KEY="your_api_key"
fofa-rs -q "port=80"
```

### Example queries

```bash
# Search by title
fofa-rs -q "title=\"nginx\""

# Search by port
fofa-rs -q "port=8080"

# Search by country
fofa-rs -q "country=US"

# Combined queries
fofa-rs -q "port=443 && country=CN && protocol=https"

# Search by banner
fofa-rs -q "banner=\"Apache\""
```

## Output Format

```
🔍 Querying FOFA: port=80
────────────────────────────────────────────────────────────────────────
1. 192.168.1.1  : 80
2. example.com  : 443  (example.com)
3. 10.0.0.1     : 8080 (internal.example.com)
```

## FOFA Query Syntax

Refer to [FOFA Query Syntax](https://fofa.info/help) for full documentation. Common operators:

| Operator | Example | Description |
|----------|---------|-------------|
| `&&` | `port=80 && country=US` | AND |
| `||` | `port=80 || port=443` | OR |
| `=` | `title="nginx"` | Exact match |
| `!=` | `country!="CN"` | Not equal |
| `>` | `port>1024` | Greater than |

## Building from source

```bash
git clone https://github.com/mnk-works/fofa-rs
cd fofa-rs
cargo build --release
./target/release/fofa-rs --help
```

## API Reference

This tool uses the FOFA API v1 `/api/v1/search/all` endpoint.

### Default fields requested
- `ip` - IP address
- `host` - Host/domain name
- `port` - Port number

### Rate limits

Free accounts: ~50 results per query (first page). For more results, upgrade your FOFA account.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Disclaimer

This tool is for authorized security research and asset discovery only. Users are responsible for complying with FOFA's Terms of Service and applicable laws.