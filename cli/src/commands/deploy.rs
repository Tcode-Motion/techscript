//! # tsc deploy Command
//!
//! Provides deployment automation and guidance for TechScript WebForge applications.
//! Supports static CDN deployments, containerized Docker builds, and self-hosted server daemons.

use crate::exit_code::ExitCode;
use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn execute(target_opt: Option<&str>) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let target = target_opt.unwrap_or("guide").to_lowercase();

    println!(
        "{}",
        "================================================================================"
            .cyan()
            .bold()
    );
    println!("                   TECHSCRIPT WEBFORGE DEPLOYMENT ADAPTER                       ");
    println!(
        "{}",
        "================================================================================"
            .cyan()
            .bold()
    );

    match target.as_str() {
        "docker" => {
            println!("🐳 Generating production Docker configuration for TechScript WebForge...");
            let entry_file = if current_dir.join("src/main.txs").exists() {
                "src/main.txs"
            } else if current_dir.join("main.txs").exists() {
                "main.txs"
            } else if current_dir.join("src/main.tech").exists() {
                "src/main.tech"
            } else {
                "src/main.txs"
            };
            let dockerfile = format!(
                r#"# Production Dockerfile for TechScript WebForge
FROM rust:1.80-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p techscript_cli

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/tsc /usr/local/bin/tsc
COPY . .
EXPOSE 3000
CMD ["tsc", "run", "{}"]
"#,
                entry_file
            );
            let docker_compose = r#"version: '3.8'
services:
  webforge:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./webforge.db:/app/webforge.db
    restart: unless-stopped
"#;
            if let Err(e) = std::fs::write(current_dir.join("Dockerfile"), dockerfile) {
                eprintln!("Failed to write Dockerfile: {}", e);
                return ExitCode::IoError;
            }
            if let Err(e) = std::fs::write(current_dir.join("docker-compose.yml"), docker_compose) {
                eprintln!("Failed to write docker-compose.yml: {}", e);
                return ExitCode::IoError;
            }
            println!("  Created: Dockerfile");
            println!("  Created: docker-compose.yml");
            println!(
                "\n{}",
                "✓ Docker deployment files generated successfully."
                    .green()
                    .bold()
            );
            println!(
                "Run '{}' to launch in containerized mode.",
                "docker compose up --build -d".cyan()
            );
        }
        "static" | "vercel" | "netlify" => {
            println!(
                "⚡ Preparing static assets for CDN deployment (Vercel, Netlify, Cloudflare)..."
            );
            let res = crate::commands::build::execute(
                Some("web"),
                Some("release"),
                false,
                false,
                "web",
                false,
            );
            if res != ExitCode::Success {
                return res;
            }
            println!(
                "\n{}",
                "✓ Static assets ready in 'dist/' directory.".green().bold()
            );
            println!("Deployment instructions:");
            println!("  • Vercel:           Run 'vercel --prod dist'");
            println!("  • Netlify:          Run 'netlify deploy --prod --dir=dist'");
            println!("  • Cloudflare Pages: Run 'wrangler pages deploy dist'");
        }
        _ => {
            println!("Available Deployment Targets for TechScript WebForge:\n");
            println!(
                "  1. {}   Deploy production bundle to Vercel, Netlify, or Cloudflare Pages",
                "tsc deploy -t static".cyan().bold()
            );
            println!(
                "  2. {}   Generate Dockerfile and docker-compose.yml for container deployment",
                "tsc deploy -t docker".cyan().bold()
            );
            println!(
                "  3. {}   Directly compile production bundle into 'dist/'",
                "tsc build web --release".cyan().bold()
            );
            println!("\nTo execute any target, run 'tsc deploy -t <target>'.");
        }
    }

    ExitCode::Success
}
