use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod organizer;
mod config;
mod utils;

use organizer::FileOrganizer;
use config::Config;

#[derive(Parser)]
#[command(name = "forg")]
#[command(about = "A CLI tool to organize files by type, date, or custom rules")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Organize files by extension into folders
    ByType {
        /// Source directory to organize
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
        /// Target directory (creates subfolders here)
        #[arg(short, long)]
        target: Option<PathBuf>,
        /// Preview changes without executing
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Organize files by creation/modification date
    ByDate {
        /// Source directory to organize
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
        /// Target directory
        #[arg(short, long)]
        target: Option<PathBuf>,
        /// Date format: year, month, or day
        #[arg(short, long, default_value = "month")]
        format: String,
        /// Preview changes without executing
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Clean up empty directories
    Clean {
        /// Directory to clean
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
        /// Preview changes without executing
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Show file statistics for a directory  
    Stats {
        /// Directory to analyze
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
    },
    /// Generate a default configuration file
    Config {
        /// Output path for config file
        #[arg(short, long, default_value = "forg-config.json")]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::ByType { source, target, dry_run } => {
            let target = target.unwrap_or_else(|| source.clone());
            let organizer = FileOrganizer::new(source, target);
            
            if dry_run {
                println!("{}", "[DRY RUN] No files will be moved".yellow().bold());
            }
            
            organizer.organize_by_type(dry_run)?;
        }
        Commands::ByDate { source, target, format, dry_run } => {
            let target = target.unwrap_or_else(|| source.clone());
            let organizer = FileOrganizer::new(source, target);
            
            if dry_run {
                println!("{}", "[DRY RUN] No files will be moved".yellow().bold());
            }
            
            organizer.organize_by_date(&format, dry_run)?;
        }
        Commands::Clean { directory, dry_run } => {
            let organizer = FileOrganizer::new(directory.clone(), directory);
            
            if dry_run {
                println!("{}", "[DRY RUN] No directories will be removed".yellow().bold());
            }
            
            organizer.clean_empty_dirs(dry_run)?;
        }
        Commands::Stats { directory } => {
            let organizer = FileOrganizer::new(directory.clone(), directory);
            organizer.show_stats()?;
        }
        Commands::Config { output } => {
            Config::generate_default(&output)?;
            println!("{} Generated config file: {}", "[SUCCESS]".green(), output.display());
        }
    }
    
    Ok(())
}