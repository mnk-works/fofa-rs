## Usage

### Basic usage (requires FOFA credentials)
```bash
export FOFA_EMAIL="your@email.com"
export FOFA_KEY="your_api_key"
fofa-rs -q "title=\"nginx\""
```

### Demo mode (no credentials needed)
```bash
# Uses sample data without FOFA credentials
fofa-rs
```

### Options
```
Usage: fofa-rs [OPTIONS] --email <EMAIL> --key <KEY>

Options:
  -q, --query <QUERY>      Search query (e.g., "title=\"Test\"") [default: title="Test"]
  -l, --limit <LIMIT>      Maximum number of results to display (default: 50)
  -p, --page <PAGE>        Page number (default: 1)
  -e, --email <EMAIL>      FOFA account email [env: FOFA_EMAIL=]
  -k, --key <KEY>          FOFA API key [env: FOFA_KEY=]
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

### Output Format

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

## Disclaimer

This tool is for authorized security research and asset discovery only. Users are responsible for complying with FOFA's Terms of Service and applicable laws. **FOFA API access requires valid email and API key credentials.**