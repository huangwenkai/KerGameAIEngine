//! KerGameAIEngine CLI
//! 
//! Usage: engine run-demo <ID> --headless --seed <N> --report <PATH>

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "engine")]
#[command(about = "KerGameAIEngine CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a demo
    RunDemo {
        /// Demo ID (e.g., M0, M1)
        id: String,

        /// Run in headless mode (no window/rendering)
        #[arg(long)]
        headless: bool,

        /// Random seed for deterministic execution
        #[arg(long, default_value = "0")]
        seed: u64,

        /// Path to save JSON report
        #[arg(long)]
        report: Option<PathBuf>,
    },

    /// List available demos
    ListDemos,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::RunDemo { id, headless, seed, report } => {
            log::info!("Running demo: {} (seed: {}, headless: {})", id, seed, headless);
            
            let demo_report = ker_engine::demo::run_demo(&id, seed, headless)?;
            
            println!("\n=== Demo Report ===");
            println!("Demo:        {}", demo_report.demo_id);
            println!("Status:      {}", if demo_report.success { "✓ SUCCESS" } else { "✗ FAILED" });
            println!("Seed:        {}", demo_report.seed);
            println!("Ticks:       {}", demo_report.tick_count);
            println!("Real time:   {} ms", demo_report.elapsed_real_ms);
            println!("Sim time:    {} ms", demo_report.elapsed_sim_ms);
            println!("Replay hash: {}", demo_report.replay_hash);
            println!("Message:     {}", demo_report.message);
            
            if let Some(report_path) = report {
                demo_report.save(&report_path)?;
                println!("\nReport saved to: {}", report_path.display());
            }
            
            Ok(())
        }

        Commands::ListDemos => {
            let registry = ker_engine::demo::DemoRegistry::new();
            
            println!("Available demos:");
            for demo in registry.list() {
                println!("  {} - {}", demo.id(), demo.description());
            }
            
            Ok(())
        }
    }
}
