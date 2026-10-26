use anyhow::Result;
use base64::Engine;
use clap::Parser;
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// FOFA.io search results parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query (e.g., "title=\"Test\"")
    #[arg(short, long, default_value = "title=\"Test\"")]
    query: String,

    /// Maximum number of results to display (default: 50)
    #[arg(short, long, default_value_t = 50)]
    limit: usize,

    /// Page number (default: 1)
    #[arg(short, long, default_value_t = 1)]
    page: usize,

    /// FOFA account email (optional, defaults to you@example.com)
    #[arg(short, long, env = "FOFA_EMAIL")]
    email: Option<String>,

    /// FOFA API key (optional, defaults to your_api_key)
    #[arg(short, long, env = "FOFA_KEY")]
    key: Option<String>,
}

/// FOFA API search response
#[derive(Debug, Deserialize, Serialize)]
struct FofaSearchResponse {
    error: bool,
    #[serde(default)]
    errmsg: String,
    size: usize,
    page: usize,
    mode: String,
    query: String,
    results: Vec<Vec<String>>,
}

impl FofaSearchResponse {
    fn print_results(&self, limit: usize) {
        let count = self.results.len().min(limit);
        println!(
            "\n{} {}",
            format!("{count} results retrieved").bold().cyan(),
            format!("(showing {} of {})", count, self.size).dimmed()
        );
        println!("{}", "─".repeat(80));

        for (i, result) in self.results.iter().take(limit).enumerate() {
            let idx = format!("{}.", i + 1).dimmed();

            let ip = result.first().map(|s| s.as_str()).unwrap_or("");
            let domain = result.get(1).map(|s| s.as_str()).unwrap_or("");
            let port = result.get(2).map(|s| s.as_str()).unwrap_or("");

            println!(
                "{} {} {} {}",
                idx,
                ip.blue().bold(),
                ":",
                port.yellow()
            );

            if !domain.is_empty() {
                println!("    {}", format!("({domain})").green());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let email = args.email.unwrap_or_else(|| "you@example.com".to_string());
    let key = args.key.unwrap_or_else(|| "your_api_key".to_string());

    // Check if using default credentials (demo mode)
    let is_demo = email == "you@example.com" && key == "your_api_key";

    if is_demo {
        println!(
            "{}",
            "🔍 Demo mode: Using sample data (no FOFA credentials provided)".bold().yellow()
        );
        println!("{}", "=".repeat(80));

        // Sample FOFA results - demonstrates the parser with real data format
        let sample_results: Vec<Vec<String>> = vec![
            vec!["142.250.190.46".to_string(), "www.google.com".to_string(), "443".to_string(), "https".to_string(), "HTTP".to_string()],
            vec!["104.16.106.248".to_string(), "cdnjs.cloudflare.com".to_string(), "443".to_string(), "https".to_string(), "cloudflare".to_string()],
            vec!["172.217.168.238".to_string(), "www.google.com".to_string(), "443".to_string(), "https".to_string(), "Google".to_string()],
            vec!["151.101.1.133".to_string(), "github.com".to_string(), "443".to_string(), "https".to_string(), "GitHub".to_string()],
            vec!["13.224.164.55".to_string(), "windows.com".to_string(), "443".to_string(), "https".to_string(), "Microsoft".to_string()],
        ];

        let total = 8555; // Simulated total
        let displayed = sample_results.len();

        println!(
            "{} {}",
            format!("{displayed} results retrieved").bold().cyan(),
            format!("(showing {} of {})", displayed, total).dimmed()
        );
        println!("{}", "─".repeat(80));

        for (i, result) in sample_results.iter().enumerate() {
            let idx = format!("{}.", i + 1).dimmed();
            let ip = result.first().map(|s| s.as_str()).unwrap_or("");
            let port = result.get(2).map(|s| s.as_str()).unwrap_or("");
            let host = result.get(1).map(|s| s.as_str()).unwrap_or("");

            println!(
                "{} {} {} {}",
                idx,
                ip.blue().bold(),
                ":",
                port.yellow()
            );

            if !host.is_empty() {
                println!("    {}", format!("({host})").green());
            }
        }

        println!("\n{}", "💡 Tip: Get your own FOFA API key at https://fofa.info".dimmed().italic());
        println!("{}", "=".repeat(80));
    } else {
        // Real API mode
        println!(
            "{}",
            format!("🔍 Querying FOFA: {}", args.query).bold().green()
        );

        // Encode the query in base64 (as required by FOFA API)
        let query_b64 = base64::engine::general_purpose::STANDARD.encode(args.query.as_bytes());

        // Build the request URL
        let url = format!(
            "https://fofa.info/api/v1/search/all?email={}&key={}&qbase64={}&page={}&size={}&fields=ip,host,port",
            email, key, query_b64, args.page, args.limit.max(50).min(1000)
        );

        // Send the HTTP GET request
        let client = Client::builder()
            .user_agent("fofa-rs/0.1")
            .build()?;

        let resp = client.get(&url).send().await?.error_for_status()?;

        let json_str = resp.text().await?;

        // Parse the JSON response
        let response: FofaSearchResponse = match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}: {e}", "Failed to parse response".red().bold());
                eprintln!("Raw response: {}", json_str.dimmed());
                return Err(anyhow::anyhow!("JSON parse error"));
            }
        };

        // Check for API errors
        if response.error {
            eprintln!("{}: {}", "FOFA API Error".red().bold(), response.errmsg);
            eprintln!("Please verify your email and API key are correct.");
            return Err(anyhow::anyhow!("API error: {}", response.errmsg));
        }

        // Display results
        response.print_results(args.limit);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parsing() {
        let args = Args::parse_from(["prog", "--query", "title=\"Test\""]);
        assert_eq!(args.query, "title=\"Test\"");
    }

    #[test]
    fn test_base64_encoding() {
        let query = "title=\"bing\"";
        let encoded = base64::engine::general_purpose::STANDARD.encode(query.as_bytes());
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), query);
    }

    #[test]
    fn test_response_parsing() {
        let json_str = r#"{
            "error": false,
            "size": 2,
            "page": 1,
            "mode": "extended",
            "query": "port=80",
            "results": [["1.2.3.4", "example.com", "80"], ["5.6.7.8", "", "443"]]
        }"#;

        let response: FofaSearchResponse = serde_json::from_str(json_str).unwrap();
        assert!(!response.error);
        assert_eq!(response.size, 2);
        assert_eq!(response.results[0][0], "1.2.3.4");
        assert_eq!(response.results[1][2], "443");
    }
}
