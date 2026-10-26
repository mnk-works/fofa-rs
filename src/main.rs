use anyhow::Result;
use base64::Engine;
use clap::{Parser, Subcommand};
use colored::*;
use dirs::home_dir;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

/// FOFA.io search results parser
#[derive(Parser, Debug)]
#[command(author, version, about = "FOFA.io search results parser", long_about = None)]
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

    /// Subcommand
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login and save credentials to local config
    Login {
        /// FOFA account email
        #[arg(short, long)]
        email: String,

        /// FOFA API key
        #[arg(short, long)]
        key: String,
    },

    /// Logout and remove saved credentials
    Logout,

    /// Show current authentication status
    Status,

    /// Search FOFA (default action)
    Search {
        /// Search query (e.g., "title=\"Test\"")
        #[arg(short, long, default_value = "title=\"Test\"")]
        query: String,

        /// Maximum number of results to display (default: 50)
        #[arg(short, long, default_value_t = 50)]
        limit: usize,

        /// Page number (default: 1)
        #[arg(short, long, default_value_t = 1)]
        page: usize,
    },
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

fn config_path() -> std::path::PathBuf {
    home_dir().unwrap().join(".config/fofa-rs/credentials.json")
}

fn is_logged_in() -> bool {
    config_path().exists()
}

fn save_credentials(email: &str, key: &str) -> Result<()> {
    let config_dir = home_dir().unwrap().join(".config/fofa-rs");
    fs::create_dir_all(&config_dir)?;
    let credentials = serde_json::json!({
        "email": email,
        "key": key,
    });
    fs::write(config_path(), serde_json::to_string(&credentials)?)?;
    println!("{} Credentials saved to {}", "✓".green().bold(), config_path().display());
    Ok(())
}

fn load_credentials() -> Result<(String, String)> {
    let config_content = fs::read_to_string(config_path())?;
    let credentials: serde_json::Value = serde_json::from_str(&config_content)?;
    let email = credentials["email"].as_str().unwrap_or("you@example.com").to_string();
    let key = credentials["key"].as_str().unwrap_or("your_api_key").to_string();
    Ok((email, key))
}

fn remove_credentials() -> Result<()> {
    if config_path().exists() {
        fs::remove_file(config_path())?;
        println!("{} Credentials removed", "✓".green().bold());
    } else {
        println!("{} No credentials to remove", "ℹ".blue().bold());
    }
    Ok(())
}

fn show_status() -> Result<()> {
    if is_logged_in() {
        let (email, key) = load_credentials()?;
        println!("{} Logged in as: {}", "✓".green().bold(), email.cyan());
        println!("{} API key: {}", "•".dimmed(), format!("{}***{}", &key[..4.min(key.len())], &key[key.len().saturating_sub(4)..]).dimmed());
    } else {
        println!("{} Not logged in", "✗".red().bold());
        println!("Run `fofa-rs login -e <email> -k <key>` to authenticate");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Login { email, key }) => {
            save_credentials(&email, &key)?;
        }
        Some(Commands::Logout) => {
            remove_credentials()?;
        }
        Some(Commands::Status) => {
            show_status()?;
        }
        Some(Commands::Search { query, limit, page }) => {
            do_search(query, limit, page).await?;
        }
        None => {
            // Default action: search with main args
            do_search(args.query, args.limit, args.page).await?;
        }
    }

    Ok(())
}

async fn do_search(query: String, limit: usize, page: usize) -> Result<()> {
    if !is_logged_in() {
        eprintln!("{} Not logged in. Run `fofa-rs login -e <email> -k <key>` first", "✗".red().bold());
        return Ok(());
    }

    let (email, key) = load_credentials()?;

    println!(
        "{}",
        format!("🔍 Querying FOFA: {}", query).bold().green()
    );

    // Encode the query in base64 (as required by FOFA API)
    let query_b64 = base64::engine::general_purpose::STANDARD.encode(query.as_bytes());

    // Build the request URL
    let url = format!(
        "https://fofa.info/api/v1/search/all?email={}&key={}&qbase64={}&page={}&size={}&fields=ip,host,port",
        email, key, query_b64, page, limit.max(50).min(1000)
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
    response.print_results(limit);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_parsing() {
        let args =
            Args::parse_from(["prog", "login", "-e", "test@example.com", "-k", "secret"]);
        match args.command {
            Some(Commands::Login { email, key }) => {
                assert_eq!(email, "test@example.com");
                assert_eq!(key, "secret");
            }
            _ => panic!("Expected Login command"),
        }
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